//! 🔤️ Parser state and feeding: the paste and timeout limits, the parser state, the key tables and the `AnsiParser` constructor, byte feeding, escape and ss3 handling. See [`super`].

use super::*;

//#region 🔤️Parse
/// 🪣️ Most bytes one bracketed paste may carry; the rest is dropped until the paste ends.
pub const PASTE_LIMIT: usize = 1 << 20;

/// ⏱️ How long a lone ESC waits for a follow-up byte before it is the Escape key.
pub const ESCAPE_TIMEOUT: Duration = Duration::from_millis(30);

/// ⌛️ How long a half-received control sequence waits before it is dropped.
pub const SEQUENCE_TIMEOUT: Duration = Duration::from_millis(250);

/// 🛑️ How long a bracketed paste may stay silent before what arrived is delivered.
pub const PASTE_TIMEOUT: Duration = Duration::from_secs(2);

pub(super) const PASTE_END: &[u8] = b"\x1b[201~";

const MAX_PARAMS: usize = 16;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum ParserState {
    Ground,
    Escape,
    Csi,
    Ss3,
    Osc,
    OscEscape,
    X10Mouse,
    Paste,
}

/// 🔤️ Handcrafted incremental ANSI input decoder (keys, mouse, paste, focus, UTF-8).
pub struct AnsiParser {
    pub(super) state: ParserState,
    pub(super) params: [u16; MAX_PARAMS],
    pub(super) subs: [[u16; 2]; MAX_PARAMS],
    pub(super) count: usize,
    pub(super) current: u16,
    pub(super) current_subs: [u16; 2],
    pub(super) sub_index: usize,
    pub(super) has_current: bool,
    pub(super) private: Option<u8>,
    pub(super) intermediate: bool,
    pub(super) invalid: bool,
    pub(super) linux_function: bool,
    utf8_buf: [u8; 4],
    utf8_len: u8,
    utf8_need: u8,
    utf8_mods: u8,
    pub(super) paste: Vec<u8>,
    pub(super) paste_matched: usize,
    pub(super) paste_truncated: bool,
    pub(super) x10: [u8; 3],
    pub(super) x10_len: usize,
    pub(super) last_button: MouseButton,
}

impl Default for AnsiParser {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) fn modifier_bits(code: u16) -> u8 {
    if code == 0 {
        return 0;
    }
    let m = code.saturating_sub(1);
    let mut bits = 0u8;
    if m & 1 != 0 {
        bits |= mods::SHIFT;
    }
    if m & 2 != 0 {
        bits |= mods::ALT;
    }
    if m & 4 != 0 {
        bits |= mods::CTRL;
    }
    bits
}

fn c0_key(byte: u8) -> Option<(Key, u8)> {
    Some(match byte {
        0x00 => (Key::Char(' '), mods::CTRL),
        0x08 | 0x7f => (Key::Backspace, 0),
        0x09 => (Key::Tab, 0),
        0x0a => (Key::Char('j'), mods::CTRL),
        0x0d => (Key::Enter, 0),
        0x01..=0x1a => (Key::Char((b'a' + byte - 1) as char), mods::CTRL),
        0x1c => (Key::Char('\\'), mods::CTRL),
        0x1d => (Key::Char(']'), mods::CTRL),
        0x1e => (Key::Char('^'), mods::CTRL),
        0x1f => (Key::Char('_'), mods::CTRL),
        _ => return None,
    })
}

fn keypad_key(byte: u8) -> Option<KeypadKey> {
    match byte {
        b'j' => Some(KeypadKey::Multiply),
        b'k' => Some(KeypadKey::Plus),
        b'l' => Some(KeypadKey::Separator),
        b'm' => Some(KeypadKey::Minus),
        b'n' => Some(KeypadKey::Decimal),
        b'o' => Some(KeypadKey::Divide),
        b'p'..=b'y' => Some(KeypadKey::Digit(byte - b'p')),
        b'X' => Some(KeypadKey::Equal),
        b'M' => Some(KeypadKey::Enter),
        _ => None,
    }
}

fn keypad_from_codepoint(code: u16) -> Option<KeypadKey> {
    Some(match code {
        57399..=57408 => KeypadKey::Digit((code - 57399) as u8),
        57409 => KeypadKey::Decimal,
        57410 => KeypadKey::Divide,
        57411 => KeypadKey::Multiply,
        57412 => KeypadKey::Minus,
        57413 => KeypadKey::Plus,
        57414 => KeypadKey::Enter,
        57415 => KeypadKey::Equal,
        57416 => KeypadKey::Separator,
        _ => return None,
    })
}

pub(super) fn function_key_from_tilde(code: u16) -> Option<Key> {
    Some(Key::F(match code {
        11..=15 => (code - 10) as u8,
        17..=21 => (code - 11) as u8,
        23..=26 => (code - 12) as u8,
        28 | 29 => (code - 13) as u8,
        31..=34 => (code - 14) as u8,
        _ => return None,
    }))
}

pub(super) fn key_from_codepoint(code: u16) -> Option<Key> {
    match code {
        9 => Some(Key::Tab),
        13 => Some(Key::Enter),
        27 => Some(Key::Esc),
        8 | 127 => Some(Key::Backspace),
        57399..=57416 => keypad_from_codepoint(code).map(Key::Keypad),
        57344..=63743 => None,
        _ => char::from_u32(u32::from(code)).filter(|c| !c.is_control()).map(Key::Char),
    }
}

impl AnsiParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Ground,
            params: [0; MAX_PARAMS],
            subs: [[0; 2]; MAX_PARAMS],
            count: 0,
            current: 0,
            current_subs: [0; 2],
            sub_index: 0,
            has_current: false,
            private: None,
            intermediate: false,
            invalid: false,
            linux_function: false,
            utf8_buf: [0; 4],
            utf8_len: 0,
            utf8_need: 0,
            utf8_mods: 0,
            paste: Vec::new(),
            paste_matched: 0,
            paste_truncated: false,
            x10: [0; 3],
            x10_len: 0,
            last_button: MouseButton::Left,
        }
    }

    fn reset_seq(&mut self) {
        self.count = 0;
        self.current = 0;
        self.current_subs = [0; 2];
        self.sub_index = 0;
        self.has_current = false;
        self.private = None;
        self.intermediate = false;
        self.invalid = false;
        self.linux_function = false;
    }

    pub(super) fn end_param(&mut self) {
        if self.count < MAX_PARAMS {
            self.params[self.count] = self.current;
            self.subs[self.count] = self.current_subs;
            self.count += 1;
        }
        self.current = 0;
        self.current_subs = [0; 2];
        self.sub_index = 0;
        self.has_current = false;
    }

    pub(super) fn param(&self, i: usize, default: u16) -> u16 {
        if i < self.count {
            self.params[i]
        } else {
            default
        }
    }

    pub(super) fn mods_param(&self, i: usize) -> u8 {
        modifier_bits(self.param(i, 0))
    }

    pub(super) fn is_release(&self, i: usize) -> bool {
        i < self.count && self.subs[i][0] == 3
    }

    /// 📥️ Feeds raw input bytes, appending any decoded events to `out`.
    pub fn feed(&mut self, bytes: &[u8], out: &mut Vec<Event>) {
        for &b in bytes {
            self.feed_byte(b, out);
        }
    }

    /// ⏳️ How long the parser may stay silent in its current state before `expire` must run, if it holds a partial input.
    pub fn pending_timeout(&self) -> Option<Duration> {
        if self.utf8_need > 0 {
            return Some(SEQUENCE_TIMEOUT);
        }
        match self.state {
            ParserState::Ground => None,
            ParserState::Escape => Some(ESCAPE_TIMEOUT),
            ParserState::Paste => Some(PASTE_TIMEOUT),
            _ => Some(SEQUENCE_TIMEOUT),
        }
    }

    /// 🕰️ Resolves a partial input after `pending_timeout` of silence: ESC becomes Escape, a paste is delivered, other fragments are dropped.
    pub fn expire(&mut self, out: &mut Vec<Event>) {
        if self.utf8_need > 0 {
            self.drop_utf8();
        }
        match self.state {
            ParserState::Escape => out.push(Event::Key(KeyEvent { key: Key::Esc, mods: 0 })),
            ParserState::Paste => self.finish_paste(out),
            _ => {}
        }
        self.state = ParserState::Ground;
    }

    /// ⏏️ Resolves a lone pending ESC (no follow-up byte arrived before a poll timeout).
    pub fn flush_escape(&mut self, out: &mut Vec<Event>) {
        if self.state == ParserState::Escape {
            out.push(Event::Key(KeyEvent { key: Key::Esc, mods: 0 }));
            self.state = ParserState::Ground;
        }
    }

    /// 📋️ Whether the parser is inside a bracketed paste.
    pub fn in_paste(&self) -> bool {
        self.state == ParserState::Paste
    }

    pub(super) fn key(&self, key: Key, mods: u8, out: &mut Vec<Event>) {
        out.push(Event::Key(KeyEvent { key, mods }));
    }

    fn drop_utf8(&mut self) {
        self.utf8_len = 0;
        self.utf8_need = 0;
        self.utf8_mods = 0;
    }

    fn start_utf8(&mut self, lead: u8, need: u8, mods: u8) {
        self.utf8_buf[0] = lead;
        self.utf8_len = 1;
        self.utf8_need = need;
        self.utf8_mods = mods;
    }

    fn finish_utf8(&mut self, out: &mut Vec<Event>) {
        if let Ok(text) = std::str::from_utf8(&self.utf8_buf[..usize::from(self.utf8_len)]) {
            if let Some(c) = text.chars().next() {
                self.key(Key::Char(c), self.utf8_mods, out);
            }
        }
        self.drop_utf8();
    }

    fn feed_byte(&mut self, b: u8, out: &mut Vec<Event>) {
        if self.utf8_need > 0 {
            if b & 0xc0 == 0x80 {
                self.utf8_buf[usize::from(self.utf8_len)] = b;
                self.utf8_len += 1;
                self.utf8_need -= 1;
                if self.utf8_need == 0 {
                    self.finish_utf8(out);
                }
                return;
            }
            self.drop_utf8();
        }
        match self.state {
            ParserState::Ground => self.feed_ground(b, out),
            ParserState::Escape => self.feed_escape(b, out),
            ParserState::Csi => self.feed_csi(b, out),
            ParserState::Ss3 => self.feed_ss3(b, out),
            ParserState::Osc => match b {
                0x07 | 0x18 | 0x1a => self.state = ParserState::Ground,
                0x1b => self.state = ParserState::OscEscape,
                _ => {}
            },
            ParserState::OscEscape => {
                if b == b'\\' {
                    self.state = ParserState::Ground;
                } else {
                    self.state = ParserState::Escape;
                    self.feed_escape(b, out);
                }
            }
            ParserState::X10Mouse => self.feed_x10(b, out),
            ParserState::Paste => self.feed_paste(b, out),
        }
    }

    fn feed_utf8_lead(&mut self, b: u8, mods: u8) {
        match b {
            0xc2..=0xdf => self.start_utf8(b, 1, mods),
            0xe0..=0xef => self.start_utf8(b, 2, mods),
            0xf0..=0xf4 => self.start_utf8(b, 3, mods),
            _ => {}
        }
    }

    pub(super) fn feed_ground(&mut self, b: u8, out: &mut Vec<Event>) {
        if b == 0x1b {
            self.state = ParserState::Escape;
        } else if let Some((key, mods)) = c0_key(b) {
            self.key(key, mods, out);
        } else if (0x20..=0x7e).contains(&b) {
            self.key(Key::Char(b as char), 0, out);
        } else {
            self.feed_utf8_lead(b, 0);
        }
    }

    fn feed_escape(&mut self, b: u8, out: &mut Vec<Event>) {
        self.state = ParserState::Ground;
        match b {
            b'[' => {
                self.reset_seq();
                self.state = ParserState::Csi;
            }
            b'O' => {
                self.reset_seq();
                self.state = ParserState::Ss3;
            }
            b']' => self.state = ParserState::Osc,
            0x1b => {
                self.key(Key::Esc, 0, out);
                self.state = ParserState::Escape;
            }
            0x20..=0x7e => self.key(Key::Char(b as char), mods::ALT, out),
            _ => {
                if let Some((key, mods)) = c0_key(b) {
                    self.key(key, mods | mods::ALT, out);
                } else {
                    self.feed_utf8_lead(b, mods::ALT);
                }
            }
        }
    }

    fn feed_ss3(&mut self, b: u8, out: &mut Vec<Event>) {
        if b.is_ascii_digit() {
            self.current = self.current.saturating_mul(10).saturating_add(u16::from(b - b'0'));
            return;
        }
        let mods = modifier_bits(self.current);
        self.state = ParserState::Ground;
        let key = match b {
            b'A' => Some(Key::Up),
            b'B' => Some(Key::Down),
            b'C' => Some(Key::Right),
            b'D' => Some(Key::Left),
            b'H' => Some(Key::Home),
            b'F' => Some(Key::End),
            b'P' => Some(Key::F(1)),
            b'Q' => Some(Key::F(2)),
            b'R' => Some(Key::F(3)),
            b'S' => Some(Key::F(4)),
            b'I' => Some(Key::Tab),
            0x1b => {
                self.state = ParserState::Escape;
                None
            }
            _ => keypad_key(b).map(Key::Keypad),
        };
        if let Some(key) = key {
            self.key(key, mods, out);
        }
    }

}
//#endregion 🔤️Parse
