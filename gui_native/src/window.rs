pub mod integration;

use {
    iced::Application,
    iced_winit::Color,
    std::fmt::Debug,
    winit::event_loop::{EventLoop, EventLoopProxy},
};

/// An event to modify the behavior of the window wrapping the application.
///
/// This is produced via the function provided to `Window::run_with_events`.
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Whether or not the application is hidden.
    ///
    /// When false, the window is activated and focused.
    HiddenOrFocused(bool),
    Title(String),
    BackgroundColor(Color),
}

/// A helper to abstract Iced & Winit integration.
pub struct Window<A, M: 'static> {
    application: A,
    event_loop: EventLoop<M>,
}

impl<A, M> Window<A, M> {
    pub fn new(application: A) -> Self {
        Self {
            application,
            event_loop: EventLoop::with_user_event(),
        }
    }
    pub fn event_loop_proxy(&self) -> EventLoopProxy<M> {
        self.event_loop.create_proxy()
    }
}
impl<A> Window<A, A::Message>
where
    A: Application + 'static,
{
    pub fn run(self) -> ! {
        self.run_with_events(|_| None)
    }
    pub fn run_with_events<F>(self, message_map: F) -> !
    where
        F: Fn(&A::Message) -> Option<WindowEvent> + 'static,
    {
        let Window {
            application,
            event_loop,
        } = self;
        integration::run(application, event_loop, message_map)
    }
}
