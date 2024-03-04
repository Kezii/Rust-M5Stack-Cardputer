use esp_idf_hal::gpio::{AnyIOPin, AnyOutputPin, PinDriver};

type KeyboardState = [u8; 8];
pub struct CardputerKeyboard<'a> {
    mux: [PinDriver<'a, AnyOutputPin, esp_idf_hal::gpio::Output>; 3],
    columns: [PinDriver<'a, AnyIOPin, esp_idf_hal::gpio::Input>; 7],
    state: KeyboardState,
}

impl<'a> CardputerKeyboard<'a> {
    pub fn new(
        mux: [PinDriver<'a, AnyOutputPin, esp_idf_hal::gpio::Output>; 3],
        columns: [PinDriver<'a, AnyIOPin, esp_idf_hal::gpio::Input>; 7],
    ) -> Self {
        Self {
            mux,
            columns,
            state: [0; 8],
        }
    }

    pub fn init(&mut self) {
        for pin in self.columns.iter_mut() {
            pin.set_pull(esp_idf_hal::gpio::Pull::Up).unwrap();
        }
    }

    pub fn read_columns(&self) -> u8 {
        let mut result = 0;
        for (i, column) in self.columns.iter().enumerate() {
            if column.is_low() {
                result |= 1 << i;
            }
        }
        result
    }

    pub fn set_mux(&mut self, index: u8) {
        for i in 0..3 {
            if index & (1 << i) != 0 {
                self.mux[i].set_high().unwrap();
            } else {
                self.mux[i].set_low().unwrap();
            }
        }
    }

    /// Reads the raw state of the keyboard.
    pub fn read_keys_raw(&mut self) -> KeyboardState {
        let mut result = [0; 8];
        for i in 0..8 {
            self.set_mux(i);
            result[i as usize] = self.read_columns();
        }
        result
    }

    /// Reads the state of the keyboard and returns a list of pressed keys.
    pub fn read_keys(&mut self) -> Vec<Key> {
        let raw = self.read_keys_raw();
        let mut result = Vec::new();
        for (i, byte) in raw.iter().enumerate() {
            for j in 0..8 {
                if byte & (1 << j) != 0 {
                    result.push(KEY_MAP[i * 7 + j]);
                }
            }
        }
        result
    }

    /// Returns the derivative of the keyboard state since the last call.
    pub fn read_events_raw(&mut self) -> KeyboardState {
        let keys = self.read_keys_raw();
        let mut result = [0; 8];
        for i in 0..8 {
            result[i] = keys[i] ^ self.state[i];
            self.state[i] = keys[i];
        }
        result
    }

    /// Returns the Pressed/Released events since the last call.
    /// WARNING: This assumes that only one key is pressed / released at a time.
    pub fn read_events(&mut self) -> Option<(KeyEvent, Key)> {
        let raw = self.read_events_raw();
        for (i, byte) in raw.iter().enumerate() {
            for j in 0..8 {
                if byte & (1 << j) != 0 {
                    return Some((
                        if self.state[i] & (1 << j) == 0 {
                            KeyEvent::Released
                        } else {
                            KeyEvent::Pressed
                        },
                        KEY_MAP[i * 7 + j],
                    ));
                }
            }
        }

        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    Space, // register 0 msb
    Period,
    M,
    B,
    C,
    Z,
    Opt,

    Enter, // register 1 msb
    Semicolon,
    K,
    H,
    F,
    S,
    Shift,

    BackSlash,
    LeftSquareBracket,
    O,
    U,
    T,
    E,
    Q,

    Backspace,
    Underscore,
    _9,
    _7,
    _5,
    _3,
    _1,

    Slash, //register 5 msb
    Comma,
    N,
    V,
    X,
    Alt,
    Ctrl,

    Quote,
    L,
    J,
    G,
    D,
    A,
    Fn,

    RightSquareBracket,
    P,
    I,
    Y,
    R,
    W,
    Tab,

    Equal, // register 7 msb
    _0,
    _8,
    _6,
    _4,
    _2,
    Tilde,
}

#[derive(Debug, Copy, Clone)]
pub enum KeyEvent {
    Pressed,
    Released,
}

const KEY_MAP: [Key; 56] = [
    Key::Opt,
    Key::Z,
    Key::C,
    Key::B,
    Key::M,
    Key::Period,
    Key::Space,
    Key::Shift,
    Key::S,
    Key::F,
    Key::H,
    Key::K,
    Key::Semicolon,
    Key::Enter,
    Key::Q,
    Key::E,
    Key::T,
    Key::U,
    Key::O,
    Key::LeftSquareBracket,
    Key::BackSlash,
    Key::_1,
    Key::_3,
    Key::_5,
    Key::_7,
    Key::_9,
    Key::Underscore,
    Key::Backspace,
    Key::Ctrl,
    Key::Alt,
    Key::X,
    Key::V,
    Key::N,
    Key::Comma,
    Key::Slash,
    Key::Fn,
    Key::A,
    Key::D,
    Key::G,
    Key::J,
    Key::L,
    Key::Quote,
    Key::Tab,
    Key::W,
    Key::R,
    Key::Y,
    Key::I,
    Key::P,
    Key::RightSquareBracket,
    Key::Tilde,
    Key::_2,
    Key::_4,
    Key::_6,
    Key::_8,
    Key::_0,
    Key::Equal,
];
