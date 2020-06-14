use {
    mnemosyne_desktop::{
        application::{Application as NemApplication, Instance},
        window::{Window, WindowEvent},
    },
    std::thread,
};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().filter_or("NEM_LOG", "info"));

    log::error!("woo");
    // <Instance<Counter> as NemApplication>::run(());
    Counter::run();

    // let window = Window::new(Counter::new(()));
    // let event_loop = window.event_loop_proxy();
    // thread::spawn(move || {
    //     use std::{
    //         io::Read,
    //         process::{Command, Stdio},
    //     };
    //     let listener = Command::new("nem-desktop-deviceinputs")
    //         .stdout(Stdio::piped())
    //         .stderr(Stdio::inherit())
    //         .spawn()
    //         .unwrap();
    //     let mut listener_stdout = listener.stdout.unwrap();

    //     let mut stdout_buf = vec![0u8; 1];
    //     let mut visible = true;
    //     loop {
    //         listener_stdout.read_exact(&mut stdout_buf).unwrap();
    //         visible = !visible;
    //         log::debug!("toggling visibility, {}", visible);
    //         event_loop
    //             .send_event(Message::WindowEvent(WindowEvent::HiddenOrFocused(visible)))
    //             .expect("event loop closed, background thread ending");
    //     }
    // });
    // window.run_with_events(|message| match message {
    //     Message::WindowEvent(event) => Some(event.clone()),
    //     _ => None,
    // })
}

use iced::{
    button, Align, Application, Button, Column, Command, Element, Sandbox, Subscription, Text,
};

#[derive(Default)]
pub struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

use mnemosyne_desktop::subscriptions::global_hotkey::{Event as HotkeyEvent, GlobalHotkey};

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    WindowEvent(WindowEvent),
    Hotkey(HotkeyEvent),
}

impl Application for Counter {
    type Executor = iced_futures::executor::AsyncStd;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            _ => {}
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        GlobalHotkey::subscription().map(Message::Hotkey)
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
