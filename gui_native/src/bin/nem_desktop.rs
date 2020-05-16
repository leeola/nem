use {
    mnemosyne_desktop::window::{Window, WindowEvent},
    std::thread,
};

fn main() {
    let window = Window::new(Counter::new());
    let event_loop = window.event_loop_proxy();
    thread::spawn(move || {
        #[derive(Debug, Copy, Clone)]
        enum Color {
            Red,
            Green,
            Blue,
        }
        let mut color = Color::Red;
        loop {
            thread::sleep(std::time::Duration::from_secs(2));
            event_loop
                .send_event(Message::WindowEvent(WindowEvent::BackgroundColor(
                    match color {
                        Color::Red => iced_winit::Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        },
                        Color::Green => iced_winit::Color {
                            r: 0.0,
                            g: 1.0,
                            b: 0.0,
                            a: 1.0,
                        },
                        Color::Blue => iced_winit::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 1.0,
                            a: 1.0,
                        },
                    },
                )))
                .expect("event loop closed, background thread ending");
            color = match color {
                Color::Red => Color::Green,
                Color::Green => Color::Blue,
                Color::Blue => Color::Red,
            };
        }
    });
    window.run_with_events(|message| match message {
        Message::WindowEvent(event) => Some(event.clone()),
        _ => None,
    })
}

use iced::{button, Align, Button, Column, Element, Sandbox, Text};

#[derive(Default)]
pub struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    WindowEvent(WindowEvent),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                Text::new(self.value.to_string())
                    .size(50)
                    .color([0.0, 0.0, 0.0]),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
