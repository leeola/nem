use {
    rdev::EventType,
    std::{cell::RefCell, collections::HashMap, io::Write},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct ModifierState {
    alt_left: bool,
    alt_right: bool,
    control_left: bool,
    control_right: bool,
    meta_left: bool,
    meta_right: bool,
    shift_left: bool,
    shift_right: bool,
}

/// A copy of rdev::Key, with Hash implemented.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Key {
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown(u32),
}

thread_local! {
    static MODIFIER_STATE: RefCell<ModifierState> = RefCell::new(
        ModifierState::default(),
    );
    static BINDINGS: RefCell<HashMap<(ModifierState, Key), u8>> = RefCell::new(HashMap::new());
}

fn main() {
    BINDINGS.with(|ref_cell| {
        // TODO: make the reporting keybinds configurable. Likely via CLI input,
        // binding a key combination to a single u8 output.
        let mut bindings = ref_cell.borrow_mut();
        bindings.insert(
            (
                ModifierState {
                    alt_left: false,
                    alt_right: false,
                    control_left: false,
                    control_right: false,
                    meta_left: true,
                    meta_right: false,
                    shift_left: false,
                    shift_right: false,
                },
                Key::BackSlash,
            ),
            0,
        );
    });

    rdev::listen(|event| {
        MODIFIER_STATE.with(|modifier_state| {
            BINDINGS.with(|bindings| {
                let key = match event.event_type {
                    EventType::KeyPress(rdev::Key::Alt) => {
                        modifier_state.borrow_mut().alt_left = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::Alt) => {
                        modifier_state.borrow_mut().alt_left = false;
                        return;
                    }
                    EventType::KeyPress(rdev::Key::AltGr) => {
                        modifier_state.borrow_mut().alt_right = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::AltGr) => {
                        modifier_state.borrow_mut().alt_right = false;
                        return;
                    }
                    EventType::KeyPress(rdev::Key::ShiftLeft) => {
                        modifier_state.borrow_mut().shift_left = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::ShiftLeft) => {
                        modifier_state.borrow_mut().shift_left = false;
                        return;
                    }
                    EventType::KeyPress(rdev::Key::ShiftRight) => {
                        modifier_state.borrow_mut().shift_right = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::ShiftRight) => {
                        modifier_state.borrow_mut().shift_right = false;
                        return;
                    }
                    EventType::KeyPress(rdev::Key::ControlLeft) => {
                        modifier_state.borrow_mut().control_left = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::ControlLeft) => {
                        modifier_state.borrow_mut().control_left = false;
                        return;
                    }
                    EventType::KeyPress(rdev::Key::ControlRight) => {
                        modifier_state.borrow_mut().control_right = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::ControlRight) => {
                        modifier_state.borrow_mut().control_right = false;
                        return;
                    }
                    EventType::KeyPress(rdev::Key::MetaLeft) => {
                        modifier_state.borrow_mut().meta_left = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::MetaLeft) => {
                        modifier_state.borrow_mut().meta_left = false;
                        return;
                    }
                    EventType::KeyPress(rdev::Key::MetaRight) => {
                        modifier_state.borrow_mut().meta_right = true;
                        return;
                    }
                    EventType::KeyRelease(rdev::Key::MetaRight) => {
                        modifier_state.borrow_mut().meta_right = false;
                        return;
                    }
                    EventType::KeyPress(key) => key.into(),
                    _ => return,
                };
                if let Some(&notify_byte) = bindings.borrow().get(&(*modifier_state.borrow(), key))
                {
                    let mut stdout = std::io::stdout();
                    stdout.write_all(&[notify_byte]).expect("write to stdout");
                    stdout.flush().expect("flush stdout")
                }
            });
        });
    })
    .expect("listen failed")
}

impl From<rdev::Key> for Key {
    fn from(key: rdev::Key) -> Self {
        match key {
            rdev::Key::Alt => Self::Alt,
            rdev::Key::AltGr => Self::AltGr,
            rdev::Key::Backspace => Self::Backspace,
            rdev::Key::CapsLock => Self::CapsLock,
            rdev::Key::ControlLeft => Self::ControlLeft,
            rdev::Key::ControlRight => Self::ControlRight,
            rdev::Key::Delete => Self::Delete,
            rdev::Key::DownArrow => Self::DownArrow,
            rdev::Key::End => Self::End,
            rdev::Key::Escape => Self::Escape,
            rdev::Key::F1 => Self::F1,
            rdev::Key::F10 => Self::F10,
            rdev::Key::F11 => Self::F11,
            rdev::Key::F12 => Self::F12,
            rdev::Key::F2 => Self::F2,
            rdev::Key::F3 => Self::F3,
            rdev::Key::F4 => Self::F4,
            rdev::Key::F5 => Self::F5,
            rdev::Key::F6 => Self::F6,
            rdev::Key::F7 => Self::F7,
            rdev::Key::F8 => Self::F8,
            rdev::Key::F9 => Self::F9,
            rdev::Key::Home => Self::Home,
            rdev::Key::LeftArrow => Self::LeftArrow,
            rdev::Key::MetaLeft => Self::MetaLeft,
            rdev::Key::MetaRight => Self::MetaRight,
            rdev::Key::PageDown => Self::PageDown,
            rdev::Key::PageUp => Self::PageUp,
            rdev::Key::Return => Self::Return,
            rdev::Key::RightArrow => Self::RightArrow,
            rdev::Key::ShiftLeft => Self::ShiftLeft,
            rdev::Key::ShiftRight => Self::ShiftRight,
            rdev::Key::Space => Self::Space,
            rdev::Key::Tab => Self::Tab,
            rdev::Key::UpArrow => Self::UpArrow,
            rdev::Key::PrintScreen => Self::PrintScreen,
            rdev::Key::ScrollLock => Self::ScrollLock,
            rdev::Key::Pause => Self::Pause,
            rdev::Key::NumLock => Self::NumLock,
            rdev::Key::BackQuote => Self::BackQuote,
            rdev::Key::Num1 => Self::Num1,
            rdev::Key::Num2 => Self::Num2,
            rdev::Key::Num3 => Self::Num3,
            rdev::Key::Num4 => Self::Num4,
            rdev::Key::Num5 => Self::Num5,
            rdev::Key::Num6 => Self::Num6,
            rdev::Key::Num7 => Self::Num7,
            rdev::Key::Num8 => Self::Num8,
            rdev::Key::Num9 => Self::Num9,
            rdev::Key::Num0 => Self::Num0,
            rdev::Key::Minus => Self::Minus,
            rdev::Key::Equal => Self::Equal,
            rdev::Key::KeyQ => Self::KeyQ,
            rdev::Key::KeyW => Self::KeyW,
            rdev::Key::KeyE => Self::KeyE,
            rdev::Key::KeyR => Self::KeyR,
            rdev::Key::KeyT => Self::KeyT,
            rdev::Key::KeyY => Self::KeyY,
            rdev::Key::KeyU => Self::KeyU,
            rdev::Key::KeyI => Self::KeyI,
            rdev::Key::KeyO => Self::KeyO,
            rdev::Key::KeyP => Self::KeyP,
            rdev::Key::LeftBracket => Self::LeftBracket,
            rdev::Key::RightBracket => Self::RightBracket,
            rdev::Key::KeyA => Self::KeyA,
            rdev::Key::KeyS => Self::KeyS,
            rdev::Key::KeyD => Self::KeyD,
            rdev::Key::KeyF => Self::KeyF,
            rdev::Key::KeyG => Self::KeyG,
            rdev::Key::KeyH => Self::KeyH,
            rdev::Key::KeyJ => Self::KeyJ,
            rdev::Key::KeyK => Self::KeyK,
            rdev::Key::KeyL => Self::KeyL,
            rdev::Key::SemiColon => Self::SemiColon,
            rdev::Key::Quote => Self::Quote,
            rdev::Key::BackSlash => Self::BackSlash,
            rdev::Key::IntlBackslash => Self::IntlBackslash,
            rdev::Key::KeyZ => Self::KeyZ,
            rdev::Key::KeyX => Self::KeyX,
            rdev::Key::KeyC => Self::KeyC,
            rdev::Key::KeyV => Self::KeyV,
            rdev::Key::KeyB => Self::KeyB,
            rdev::Key::KeyN => Self::KeyN,
            rdev::Key::KeyM => Self::KeyM,
            rdev::Key::Comma => Self::Comma,
            rdev::Key::Dot => Self::Dot,
            rdev::Key::Slash => Self::Slash,
            rdev::Key::Insert => Self::Insert,
            rdev::Key::KpReturn => Self::KpReturn,
            rdev::Key::KpMinus => Self::KpMinus,
            rdev::Key::KpPlus => Self::KpPlus,
            rdev::Key::KpMultiply => Self::KpMultiply,
            rdev::Key::KpDivide => Self::KpDivide,
            rdev::Key::Kp0 => Self::Kp0,
            rdev::Key::Kp1 => Self::Kp1,
            rdev::Key::Kp2 => Self::Kp2,
            rdev::Key::Kp3 => Self::Kp3,
            rdev::Key::Kp4 => Self::Kp4,
            rdev::Key::Kp5 => Self::Kp5,
            rdev::Key::Kp6 => Self::Kp6,
            rdev::Key::Kp7 => Self::Kp7,
            rdev::Key::Kp8 => Self::Kp8,
            rdev::Key::Kp9 => Self::Kp9,
            rdev::Key::KpDelete => Self::KpDelete,
            rdev::Key::Function => Self::Function,
            rdev::Key::Unknown(code) => Self::Unknown(code),
        }
    }
}
