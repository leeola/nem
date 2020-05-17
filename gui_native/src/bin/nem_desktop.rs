use {
    mnemosyne_desktop::window::{Window, WindowEvent},
    std::thread,
};

fn main() {
    let window = Window::new(Counter::new());
    let event_loop = window.event_loop_proxy();
    thread::spawn(move || {
        use std::{
            io::Read,
            process::{Command, Stdio},
        };
        let listener = Command::new("nem-desktop-deviceinputs")
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let mut listener_stdout = listener.stdout.unwrap();

        let mut stdout_buf = vec![0u8; 1];
        let mut visible = true;
        loop {
            listener_stdout.read_exact(&mut stdout_buf).unwrap();
            visible = !visible;
            log::debug!("toggling visibility, {}", visible);
            event_loop
                .send_event(Message::WindowEvent(WindowEvent::Visible(visible)))
                .expect("event loop closed, background thread ending");
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
