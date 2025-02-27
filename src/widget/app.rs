use std::{
    io::{stdout, Result, Write},
    marker::PhantomData,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
    u16,
};

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size},
    shell::Shell,
    style::{
        color::{Color, ColorSystem},
        theme::Theme,
    },
    terminal,
};

use super::{element::Element, widget::Widget};

pub trait Program<'a, Message: Clone> {
    /// Update the program state with a message.
    fn update(&mut self, message: Message);

    /// Returns the element to be rendered.
    fn view(&self) -> Element<'a, Message>;

    /// Returns the theme of the program.
    fn theme(&self) -> Theme {
        Theme::TOKYO_NIGHT
    }
}

pub struct App<Message, Program>
where
    Message: Clone,
    Program: for<'a> self::Program<'a, Message>,
{
    running: Arc<Mutex<bool>>,
    framerate: FrameRate,
    color_system: ColorSystem,
    background: Option<Color>,
    event_thread: Option<JoinHandle<()>>,
    quit_key: KeyCode,
    program: Program,
    _message: PhantomData<Message>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    Max(u16),
    Unlimited,
}

impl<Message, Program> App<Message, Program>
where
    Message: Clone,
    Program: for<'a> self::Program<'a, Message>,
{
    pub fn new(instance: Program) -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            framerate: FrameRate::Max(60),
            color_system: terminal::detect_color_system(),
            background: None,
            event_thread: None,
            quit_key: KeyCode::Char('q'),
            program: instance,
            _message: PhantomData,
        }
    }

    pub fn color_system(mut self, system: ColorSystem) -> Self {
        self.color_system = system;
        self
    }

    pub fn framerate(mut self, framerate: FrameRate) -> Self {
        self.framerate = framerate;
        self
    }

    pub fn quit_key(&mut self, key: KeyCode) -> &mut Self {
        self.quit_key = key;
        self
    }

    /// Run the app and enter the main loop.
    /// This function will change the terminal environment until [`Self::stop`] is called.
    pub fn run(&mut self) {
        // Check if the app has been already run.
        if *self.running.lock().unwrap() || self.event_thread.is_some() {
            return;
        }

        Self::init_fullscreen().unwrap();
        *self.running.lock().unwrap() = true;

        // Start event thread.
        let event_thread_running = self.running.clone();
        let (sender, receiver) = mpsc::channel();
        self.event_thread = Some(thread::spawn(move || {
            while *event_thread_running.lock().unwrap() {
                let event = crossterm::event::read().unwrap();
                sender.send(event).unwrap();
            }
        }));

        let mut timepoint = Instant::now();
        // Main output and event processing loop.
        while *self.running.lock().unwrap() {
            // First, draw the widget.
            let mut element = self.program.view();
            let widget = element.widget_mut();
            self.draw(widget, &self.program.theme());

            // Second, process events.
            while let Ok(event) = receiver.try_recv() {
                let mut shell = Shell::new();
                widget.process_event(event.clone(), &mut shell);
                self.process_event(event, &mut shell);
                for message in shell.messages() {
                    self.program.update(message.clone());
                }
            }

            // Third, sleep to limit the frame rate.
            if let FrameRate::Max(fps) = self.framerate {
                let elapsed = timepoint.elapsed();
                let target = Duration::from_millis(1000 / fps as u64);
                if elapsed < target {
                    thread::sleep(target - elapsed);
                }
                timepoint = Instant::now();
            }
        }

        self.stop();
    }

    /// Stop the app and exit the main loop.
    /// This function will restore the terminal environment to the original state.
    pub fn stop(&mut self) {
        *self.running.lock().unwrap() = false;

        // Stop event thread.
        if let Some(thread) = self.event_thread.take() {
            thread.join().unwrap();
        }

        Self::uninit_fullscreen().unwrap();
    }

    fn init_fullscreen() -> Result<()> {
        enable_raw_mode()?;
        stdout().queue(cursor::Hide)?;
        stdout().queue(EnableMouseCapture)?;
        stdout().queue(EnterAlternateScreen)?;
        stdout().flush()?;
        Ok(())
    }

    fn uninit_fullscreen() -> Result<()> {
        stdout().queue(LeaveAlternateScreen)?;
        stdout().queue(DisableMouseCapture)?;
        stdout().queue(cursor::Show)?;
        stdout().flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    fn draw(&self, widget: &mut dyn Widget<Message>, theme: &Theme) {
        let width = match widget.size_hint().width {
            Length::Preferred => widget.size().width,
            Length::Fixed(width) => width,
            _ => terminal::size().width,
        };
        let height = match widget.size_hint().height {
            Length::Preferred => widget.size().height,
            Length::Fixed(height) => height,
            _ => terminal::size().height,
        };
        let area = Area::from_size(Size::new(width, height));
        widget.layout(area);
        let mut background = Buffer::new(area.width, area.height);
        background.render_background(area, self.background);
        widget.render(area, &mut background, theme);
        print!("\x1b[H{}", background.view(self.color_system, theme));
    }

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {
        if shell.is_event_captured() {
            return;
        }
        match event {
            Event::Key(key_event) => {
                if key_event.code == self.quit_key {
                    self.stop();
                }
            }
            _ => {}
        }
    }
}

impl<Message, Program> Widget<Message> for App<Message, Program>
where
    Message: Clone,
    Program: for<'a> self::Program<'a, Message>,
{
    #[allow(unused_variables)]
    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme) {}

    fn size(&self) -> Size<u16> {
        Size::new(0, 0)
    }

    #[allow(unused_variables)]
    fn layout(&mut self, viewport: Area) {}
}
