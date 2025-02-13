use std::{io::{stdout, Result}, sync::{mpsc::{self, Receiver, SendError, Sender}, Arc, Mutex}, thread::{self, JoinHandle}, u16};

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture, Event}, style::available_color_count, terminal::{disable_raw_mode, enable_raw_mode, window_size}, ExecutableCommand};

use crate::{geometry::size::Size, style::{color::ColorSystem, theme::Theme}, terminal::get_windows_terminal_supports};

use super::{screen::Screen, widget::Widget};

pub struct App {
    running: Arc<Mutex<bool>>,
    framerate: FrameRate,
    color_system: ColorSystem,
    theme: Theme,
    event_channel: (Sender<Event>, Receiver<Event>),
    event_thread: Option<JoinHandle<()>>,
    screens: Vec<Screen>
}

pub enum FrameRate {
    Max(u16),
    Unlimited
}

impl App {
    pub fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            framerate: FrameRate::Max(60),
            color_system: App::detect_color_system(),
            theme: Theme::TOKYO_NIGHT,
            event_channel: mpsc::channel(),
            event_thread: None,
            screens: Vec::new()
        }
    }

    pub fn terminal_size() -> Size {
        let size = window_size().unwrap();
        Size::new(size.columns, size.rows)
    }

    #[allow(unreachable_code)]
    pub fn detect_color_system() -> ColorSystem {
        #[cfg(windows)] {
            if get_windows_terminal_supports().virtual_terminal_processing {
                return ColorSystem::TrueColor;
            } else {
                return ColorSystem::LegacyWindows;
            }
        }
        match available_color_count() {
            u16::MAX => ColorSystem::TrueColor,
            256 => ColorSystem::EightBit,
            8 => ColorSystem::Standard,
            _ => panic!("Bad color count, expected u16::Max, 256 or 8")
        }
    }

    pub fn set_color_system(&mut self, system: ColorSystem) -> &mut Self {
        self.color_system = system;
        self
    }

    pub fn set_theme(&mut self, theme: Theme) -> &mut Self {
        self.theme = theme;
        self
    }

    pub fn push_screen(&mut self, screen: Screen) -> &mut Self {
        self.screens.push(screen);
        self
    }

    pub fn pop_screen(&mut self) -> &mut Self {
        self.screens.pop();
        self
    }

    /// Get the top screen.
    pub fn screen(&self) -> Option<&Screen> {
        self.screens.last()
    }

    /// Get the top screen as mutable.
    pub fn screen_mut(&mut self) -> Option<&mut Screen> {
        self.screens.last_mut()
    }

    fn init_fullscreen() -> Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnableMouseCapture)?;
        Ok(())
    }

    fn uninit_fullscreen() -> Result<()> {
        stdout().execute(DisableMouseCapture)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn run(&mut self) {
        // Check if the app has been already run.
        if *self.running.lock().unwrap() || self.event_thread.is_some() {
            return;
        }

        Self::init_fullscreen().unwrap();
        *self.running.lock().unwrap() = true;
        let event_thread_running = self.running.clone();
        let sender = self.event_channel.0.clone();
        self.event_thread = Some(thread::spawn(move || {
            while *event_thread_running.lock().unwrap() {
                let event = crossterm::event::read().unwrap();
                sender.send(event).unwrap();
            }
        }));
        while *self.running.lock().unwrap() {
            if let Some(screen) = self.screen()  {
                let view = screen.view(self.color_system, &self.theme);
                print!("{}", view);
            }
        }

        self.stop().unwrap();
    }

    pub fn stop(&mut self) -> Result<()> {
        *self.running.lock().unwrap() = false;

        // Stop event thread.
        if let Some(thread) = self.event_thread.take() {
            thread.join().unwrap();
        }

        Self::uninit_fullscreen()?;
        Ok(())
    }
}

impl Widget for App {
    fn process_event(&mut self, event: Event) -> bool {
        for screen in self.screens.iter_mut() {
            screen.process_event(event.clone());
        }
        false // App is the top widget.
    }
}


#[cfg(test)]
mod tests {
    use crate::widget::screen::Screen;

    use super::App;

    #[test]
    fn detect_color_system() {
        let color_system = super::App::detect_color_system();
        println!("Color system: {:?}", color_system);
    }

    #[test]
    fn run() {
        let mut app = App::new();
        let screen = Screen::new()
        app.run();
    }
}