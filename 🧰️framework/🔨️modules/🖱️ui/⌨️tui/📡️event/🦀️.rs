use crate::tui::geometry::{Pos, Size};

/// ⌨️ A decoded terminal key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Esc,
    Tab,
    BackTab,
    Backspace,
    Delete,
    Insert,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
    Keypad(KeypadKey),
}

/// 🔢️ A key on the numeric keypad, which a terminal in application keypad mode spells differently from the main block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeypadKey {
    Digit(u8),
    Decimal,
    Plus,
    Minus,
    Multiply,
    Divide,
    Enter,
    Equal,
    Separator,
}

impl KeypadKey {
    /// 🔡️ The character the main block spells for this key (`Enter` is a carriage return).
    pub fn character(self) -> char {
        match self {
            Self::Digit(n) => char::from(b'0' + n.min(9)),
            Self::Decimal => '.',
            Self::Plus => '+',
            Self::Minus => '-',
            Self::Multiply => '*',
            Self::Divide => '/',
            Self::Enter => '\r',
            Self::Equal => '=',
            Self::Separator => ',',
        }
    }
}

impl Key {
    /// ✏️ The key a text widget sees: keypad keys become the characters (or Enter) they type, every other key is unchanged.
    pub fn text_equivalent(self) -> Self {
        match self {
            Self::Keypad(KeypadKey::Enter) => Self::Enter,
            Self::Keypad(key) => Self::Char(key.character()),
            other => other,
        }
    }
}

/// 🎛️ Modifier bitflags.
pub mod mods {
    pub const SHIFT: u8 = 1;
    pub const ALT: u8 = 2;
    pub const CTRL: u8 = 4;
}

/// 🎹️ One key press with its modifier bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub mods: u8,
}

impl KeyEvent {
    /// 🪄️ The event a text widget sees, see `Key::text_equivalent`.
    pub fn text_equivalent(self) -> Self {
        Self { key: self.key.text_equivalent(), mods: self.mods }
    }
}

/// 🖲️ A pointer button the terminal reports by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

/// 🐁️ What the pointer did; `Scroll` counts wheel steps, `dy > 0` down and `dx > 0` right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Move,
    Scroll { dx: i16, dy: i16 },
}

/// 🖱️ One pointer report; `clicks` counts consecutive presses on the same cell (1 single, 2 double).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MouseEvent {
    pub kind: MouseKind,
    pub pos: Pos,
    pub mods: u8,
    pub clicks: u8,
}

/// 📡️ Any input the terminal can report to the retained-mode engine.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Resize(Size),
    FocusGained,
    FocusLost,
    Wake,
}
