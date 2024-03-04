use crate::keyboard::{Key, KeyEvent};

pub struct Typing {
    pub mod_shift: bool,
    pub mod_ctrl: bool,
    pub mod_fn: bool,
}

impl Typing {
    pub fn new() -> Self {
        Self {
            mod_shift: false,
            mod_ctrl: false,
            mod_fn: false,
        }
    }

    pub fn eat_keyboard_events(&mut self, event: (KeyEvent, Key)) -> Option<KeyboardEvent> {
        if let (KeyEvent::Pressed, key) = event {
            match key {
                Key::Shift => {
                    self.mod_shift = !self.mod_shift;
                    return None;
                }
                Key::Ctrl => {
                    self.mod_ctrl = !self.mod_ctrl;
                    return None;
                }
                Key::Fn => {
                    self.mod_fn = !self.mod_fn;
                    return None;
                }
                Key::Backspace => return Some(KeyboardEvent::Backspace),
                Key::Enter => return Some(KeyboardEvent::Enter),
                Key::Tab => return Some(KeyboardEvent::Tab),
                _ => {}
            }

            if self.mod_fn {
                match key {
                    Key::Tilde => {
                        self.mod_fn = false;
                        return Some(KeyboardEvent::Esc);
                    }
                    Key::Backspace => {
                        self.mod_fn = false;
                        return Some(KeyboardEvent::Canc);
                    }
                    Key::Semicolon => return Some(KeyboardEvent::ArrowUp),
                    Key::Period => return Some(KeyboardEvent::ArrowDown),
                    Key::Comma => return Some(KeyboardEvent::ArrowLeft),
                    Key::Slash => return Some(KeyboardEvent::ArrowRight),
                    _ => {}
                }
            }

            if self.mod_ctrl && key == Key::C {
                self.mod_ctrl = false;
                return Some(KeyboardEvent::CtrlC);
            }

            let keys = [
                Key::Tilde,
                Key::_1,
                Key::_2,
                Key::_3,
                Key::_4,
                Key::_5,
                Key::_6,
                Key::_7,
                Key::_8,
                Key::_9,
                Key::_0,
                Key::Underscore,
                Key::Equal,
                Key::Q,
                Key::W,
                Key::E,
                Key::R,
                Key::T,
                Key::Y,
                Key::U,
                Key::I,
                Key::O,
                Key::P,
                Key::LeftSquareBracket,
                Key::RightSquareBracket,
                Key::BackSlash,
                Key::A,
                Key::S,
                Key::D,
                Key::F,
                Key::G,
                Key::H,
                Key::J,
                Key::K,
                Key::L,
                Key::Semicolon,
                Key::Quote,
                Key::Z,
                Key::X,
                Key::C,
                Key::V,
                Key::B,
                Key::N,
                Key::M,
                Key::Comma,
                Key::Period,
                Key::Slash,
                Key::Space,
            ];

            let ascii_lowercase = [
                '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'q', 'w', 'e',
                'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\\', 'a', 's', 'd', 'f', 'g', 'h',
                'j', 'k', 'l', ';', '\'', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', ' ',
            ];

            let ascii_uppercase = [
                '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', 'Q', 'W', 'E',
                'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '|', 'A', 'S', 'D', 'F', 'G', 'H',
                'J', 'K', 'L', ':', '"', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', ' ',
            ];

            assert!(keys.len() == ascii_lowercase.len() && keys.len() == ascii_uppercase.len());

            if let Some(ind) = keys.iter().position(|&k| k == key) {
                if self.mod_shift {
                    self.mod_shift = false;
                    return Some(KeyboardEvent::Ascii(ascii_uppercase[ind]));
                } else {
                    return Some(KeyboardEvent::Ascii(ascii_lowercase[ind]));
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KeyboardEvent {
    Ascii(char),
    Backspace,
    Enter,
    Tab,
    Esc,
    Canc,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    CtrlC,
}
