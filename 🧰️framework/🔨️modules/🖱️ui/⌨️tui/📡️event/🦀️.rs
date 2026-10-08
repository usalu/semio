use crate::tui::geometry::{Pos, Size};

/// ?? A decoded terminal key.
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
}

/// ??? Modifier bitflags.
pub mod mods {
    pub const SHIFT: u8 = 1;
    pub const ALT: u8 = 2;
    pub const CTRL: u8 = 4;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub mods: u8,
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

/// ??? Any input the terminal can report to the retained-mode engine.
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
