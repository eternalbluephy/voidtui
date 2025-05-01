use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::{
    any::Any, collections::HashMap, io::{stdout, Result, Write}, marker::PhantomData, sync::{
        atomic::{AtomicU16, Ordering},
        mpsc, Arc, Mutex,
    }, thread::{self, JoinHandle}, time::{Duration, Instant}, u16
};

use crate::{
    buffer::buffer::Buffer, event::Event, geometry::{area::Area, length::Length, size::Size}, shell::Shell, style::{
        color::{Color, ColorSystem},
        theme::Theme,
    }, terminal, widget::{element::Element, widget::Widget}
};

static MOUSE_X: AtomicU16 = AtomicU16::new(0);
static MOUSE_Y: AtomicU16 = AtomicU16::new(0);

#[allow(unused_variables)]
pub trait Program<Message: Clone> {
    /// Update the program state with a message.
    fn update(&mut self, message: Message) {}

    /// Returns the element to be rendered.
    fn view(&mut self) -> Element<'_, Message>;

    /// Returns the theme of the program.
    fn theme(&self) -> Theme {
        Theme::TOKYO_NIGHT
    }

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {}

    fn should_stop(&self) -> bool {
        false
    }
}

pub struct App<Message>
where
    Message: Clone,
{
    /// Caution: This is not implemented yet.
    /// 
    /// The hashmap of states where widgets defaultly store.
    /// You can otherwise store your states in your program struct.
    /// This depends on how the widget supported.
    _states: HashMap<usize, Box<dyn Any>>,
    running: Arc<Mutex<bool>>,
    framerate: FrameRate,
    color_system: ColorSystem,
    background: Option<Color>,
    event_thread: Option<JoinHandle<()>>,
    _message: PhantomData<Message>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    Max(u16),
    Unlimited,
}

impl<Message> App<Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            _states: HashMap::new(),
            running: Arc::new(Mutex::new(false)),
            framerate: FrameRate::Max(60),
            color_system: terminal::detect_color_system(),
            background: None,
            event_thread: None,
            _message: PhantomData,
        }
    }

    pub fn background(mut self, background: Option<impl Into<Color>>) -> Self {
        self.background = background.map(|bg| bg.into());
        self
    }

    pub fn color_system(mut self, system: ColorSystem) -> Self {
        self.color_system = system;
        self
    }

    pub fn framerate(mut self, framerate: FrameRate) -> Self {
        self.framerate = framerate;
        self
    }

    /// Run the app and enter the main loop.
    /// This function will change the terminal environment until [`Self::stop`] is called.
    pub fn run<P: Program<Message>>(&mut self, mut program: P) -> Result<()> {
        // Check if the app has been already run.
        if *self.running.lock().unwrap() || self.event_thread.is_some() {
            return Ok(());
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
            let theme = program.theme();
            // First, we process with the elements.
            let mut element = program.view();
            let area = Self::layout(element.widget_mut());

            let mut shell = Shell::new();
            // Events should be storaged for further usage.
            let mut events: Vec<Event> = Vec::new();
            while let Ok(event) = receiver.try_recv() {
                events.push(event.clone().into());
                element
                    .widget_mut()
                    .process_event(
                        event.into(), &mut shell, area
                    );
            }
            element.widget_mut().update();
            

            self.draw(area, element.widget(), &theme)?;
            // We need to update the program state after dropping the widget due to borrow check.
            drop(element);
            // Element's part has ended, we can process program itself now.
            for event in events {
                self.process_event(event.clone(), &mut shell);
                program.process_event(event, &mut shell);
            }
            for message in shell.messages() {
                program.update(message.clone());
            }
            if program.should_stop() {
                break;
            }

            // Finally, sleep to limit the frame rate.
            if let FrameRate::Max(fps) = self.framerate {
                let elapsed = timepoint.elapsed();
                let target = Duration::from_millis(1000 / fps as u64);
                if elapsed < target {
                    thread::sleep(target - elapsed);
                }
                timepoint = Instant::now();
            }
        }

        self.stop()?;
        Ok(())
    }

    /// Stop the app and exit the main loop.
    /// This function will restore the terminal environment to the original state.
    pub fn stop(&mut self) -> Result<()> {
        *self.running.lock().unwrap() = false;

        // Stop event thread.
        if let Some(thread) = self.event_thread.take() {
            thread.join().unwrap();
        }

        Self::uninit_fullscreen()?;
        Ok(())
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

    fn layout(widget: &mut dyn Widget<Message>) -> Area {
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
        let area = Area::from_size(Size::new(width, height)).min(terminal::size());
        widget.layout(area);
        area
    }

    fn draw(&self, area: Area, widget: &dyn Widget<Message>, theme: &Theme) -> Result<()> {
        let terminal_area = Area::from_size(terminal::size());
        let mut background = Buffer::new(terminal_area.width, terminal_area.height);
        background.render_background(terminal_area, self.background);
        widget.render(area, &mut background, theme);
        stdout().execute(crossterm::cursor::MoveTo(0, 0))?;
        print!("{}", background.view(self.color_system, theme));
        Ok(())
    }

    fn process_event(&mut self, event: Event, _shell: &mut Shell<Message>) {
        // Update mouse position
        if let Event::Mouse(event) = event.clone() {
            MOUSE_X.store(event.absolute_position.x, Ordering::Relaxed);
            MOUSE_Y.store(event.absolute_position.y, Ordering::Relaxed);
        }
    }
}

pub struct Mouse;

impl Mouse {
    pub fn x() -> u16 {
        MOUSE_X.load(Ordering::Relaxed)
    }

    pub fn y() -> u16 {
        MOUSE_Y.load(Ordering::Relaxed)
    }
}
