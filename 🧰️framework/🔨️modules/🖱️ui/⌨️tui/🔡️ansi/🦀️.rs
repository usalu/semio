use crate::tui::cell::{Cell, CellBuffer, DiffRun};
use crate::tui::theme::Rgb;

//#region ???Emit
/// ??? A batch of raw ANSI bytes ready to write to a terminal (or feed to xterm.js).
#[derive(Default, Clone)]
pub struct AnsiPatch(pub String);

#[derive(Clone, Copy, PartialEq)]
struct SgrState {
    fg: Option<Rgb>,
    bg: Option<Rgb>,
    attrs: u8,
}

fn push_sgr(out: &mut String, state: &mut SgrState, cell: &Cell) {
    if state.fg == Some(cell.fg) && state.bg == Some(cell.bg) && state.attrs == cell.attrs {
        return;
    }
    out.push_str("\x1b[0");
    if cell.attrs & crate::tui::cell::attr::BOLD != 0 {
        out.push_str(";1");
    }
    if cell.attrs & crate::tui::cell::attr::DIM != 0 {
        out.push_str(";2");
    }
    if cell.attrs & crate::tui::cell::attr::ITALIC != 0 {
        out.push_str(";3");
    }
    if cell.attrs & crate::tui::cell::attr::UNDERLINE != 0 {
        out.push_str(";4");
    }
    if cell.attrs & crate::tui::cell::attr::REVERSE != 0 {
        out.push_str(";7");
    }
    out.push_str(&format!(";38;2;{};{};{}", cell.fg[0], cell.fg[1], cell.fg[2]));
    out.push_str(&format!(";48;2;{};{};{}", cell.bg[0], cell.bg[1], cell.bg[2]));
    out.push('m');
    *state = SgrState { fg: Some(cell.fg), bg: Some(cell.bg), attrs: cell.attrs };
}

/// ??? Emits the minimal ANSI needed to repaint `runs` of `next` onto a terminal.
pub fn emit_runs(next: &CellBuffer, runs: &[DiffRun], out: &mut AnsiPatch) {
    let mut state = SgrState { fg: None, bg: None, attrs: u8::MAX };
    for run in runs {
        out.0.push_str(&format!("\x1b[{};{}H", run.y + 1, run.x + 1));
        let mut x = run.x;
        while x < run.x + run.len {
            let Some(c) = next.get(x, run.y) else { break };
            if c.width == 0 {
                x += 1;
                continue;
            }
            push_sgr(&mut out.0, &mut state, c);
            out.0.push(if c.ch == '\0' { ' ' } else { c.ch });
            x += u16::from(c.width.max(1));
        }
    }
}

/// ??? Enters the alternate screen, hides the cursor, and enables mouse/paste reporting.
pub fn setup_sequence() -> &'static str {
    "\x1b[?1049h\x1b[?25l\x1b[?1002h\x1b[?1006h\x1b[?2004h\x1b[2J"
}

/// ??? Restores the primary screen and default modes.
pub fn teardown_sequence() -> &'static str {
    "\x1b[?2004l\x1b[?1006l\x1b[?1002l\x1b[?25h\x1b[?1049l\x1b[0m"
}
//#endregion ???Emit

//#region ???Parse
use crate::tui::event::{mods, Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::Pos;

#[derive(Clone, Copy, PartialEq)]
enum ParserState {
    Ground,
    Escape,
    Csi,
    Ss3,
    Osc,
    Paste,
}

/// ?? Handcrafted incremental ANSI input decoder (keys, mouse, paste, focus, UTF-8).
pub struct AnsiParser {
    state: ParserState,
    params: Vec<u16>,
    current: u16,
    has_current: bool,
    private: Option<u8>,
    utf8_buf: [u8; 4],
    utf8_len: u8,
    utf8_need: u8,
    paste_buf: String,
    paste_close: Vec<u8>,
    pending_esc: bool,
}

impl Default for AnsiParser {
    fn default() -> Self {
        Self::new()
    }
}

fn modifier_bits(code: u16) -> u8 {
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

impl AnsiParser {
    pub fn new() -> Self {
        Self { state: ParserState::Ground, params: Vec::new(), current: 0, has_current: false, private: None, utf8_buf: [0; 4], utf8_len: 0, utf8_need: 0, paste_buf: String::new(), paste_close: Vec::new(), pending_esc: false }
    }

    fn reset_seq(&mut self) {
        self.params.clear();
        self.current = 0;
        self.has_current = false;
        self.private = None;
    }

    fn push_param(&mut self) {
        self.params.push(if self.has_current { self.current } else { 0 });
        self.current = 0;
        self.has_current = false;
    }

    fn param(&self, i: usize, default: u16) -> u16 {
        self.params.get(i).copied().unwrap_or(default)
    }

    /// ??? Feeds raw input bytes, appending any decoded events to `out`.
    pub fn feed(&mut self, bytes: &[u8], out: &mut Vec<Event>) {
        for &b in bytes {
            self.feed_byte(b, out);
        }
    }

    /// ?? Resolves a lone pending ESC (no follow-up byte arrived before a poll timeout).
    pub fn flush_escape(&mut self, out: &mut Vec<Event>) {
        if self.pending_esc && self.state == ParserState::Escape {
            out.push(Event::Key(KeyEvent { key: Key::Esc, mods: 0 }));
            self.state = ParserState::Ground;
            self.pending_esc = false;
        }
    }

    fn feed_byte(&mut self, b: u8, out: &mut Vec<Event>) {
        if self.utf8_need > 0 {
            self.utf8_buf[self.utf8_len as usize] = b;
            self.utf8_len += 1;
            self.utf8_need -= 1;
            if self.utf8_need == 0 {
                if let Ok(s) = std::str::from_utf8(&self.utf8_buf[..self.utf8_len as usize]) {
                    if let Some(c) = s.chars().next() {
                        out.push(Event::Key(KeyEvent { key: Key::Char(c), mods: 0 }));
                    }
                }
                self.utf8_len = 0;
            }
            return;
        }
        match self.state {
            ParserState::Ground => self.feed_ground(b, out),
            ParserState::Escape => self.feed_escape(b, out),
            ParserState::Csi => self.feed_csi(b, out),
            ParserState::Ss3 => self.feed_ss3(b, out),
            ParserState::Osc => {
                if b == 0x07 || b == 0x1b {
                    self.state = ParserState::Ground;
                }
            }
            ParserState::Paste => self.feed_paste(b, out),
        }
    }

    fn feed_ground(&mut self, b: u8, out: &mut Vec<Event>) {
        match b {
            0x1b => {
                self.state = ParserState::Escape;
                self.pending_esc = true;
            }
            0x0d | 0x0a => out.push(Event::Key(KeyEvent { key: Key::Enter, mods: 0 })),
            0x09 => out.push(Event::Key(KeyEvent { key: Key::Tab, mods: 0 })),
            0x7f | 0x08 => out.push(Event::Key(KeyEvent { key: Key::Backspace, mods: 0 })),
            0x00 => out.push(Event::Key(KeyEvent { key: Key::Char(' '), mods: mods::CTRL })),
            0x01..=0x1a => {
                let c = (b'a' + b - 1) as char;
                out.push(Event::Key(KeyEvent { key: Key::Char(c), mods: mods::CTRL }));
            }
            0x00..=0x7f => out.push(Event::Key(KeyEvent { key: Key::Char(b as char), mods: 0 })),
            0xc0..=0xdf => {
                self.utf8_buf[0] = b;
                self.utf8_len = 1;
                self.utf8_need = 1;
            }
            0xe0..=0xef => {
                self.utf8_buf[0] = b;
                self.utf8_len = 1;
                self.utf8_need = 2;
            }
            0xf0..=0xf7 => {
                self.utf8_buf[0] = b;
                self.utf8_len = 1;
                self.utf8_need = 3;
            }
            _ => {}
        }
    }

    fn feed_escape(&mut self, b: u8, out: &mut Vec<Event>) {
        self.pending_esc = false;
        match b {
            b'[' => {
                self.reset_seq();
                self.state = ParserState::Csi;
            }
            b'O' => self.state = ParserState::Ss3,
            b']' => self.state = ParserState::Osc,
            0x00..=0x7f => {
                let c = b as char;
                out.push(Event::Key(KeyEvent { key: Key::Char(c), mods: mods::ALT }));
                self.state = ParserState::Ground;
            }
            _ => self.state = ParserState::Ground,
        }
    }

    fn feed_ss3(&mut self, b: u8, out: &mut Vec<Event>) {
        self.state = ParserState::Ground;
        let key = match b {
            b'P' => Some(Key::F(1)),
            b'Q' => Some(Key::F(2)),
            b'R' => Some(Key::F(3)),
            b'S' => Some(Key::F(4)),
            _ => None,
        };
        if let Some(key) = key {
            out.push(Event::Key(KeyEvent { key, mods: 0 }));
        }
    }

    fn feed_csi(&mut self, b: u8, out: &mut Vec<Event>) {
        match b {
            b'0'..=b'9' => {
                self.current = self.current.saturating_mul(10).saturating_add(u16::from(b - b'0'));
                self.has_current = true;
            }
            b';' => self.push_param(),
            b'<' if self.params.is_empty() && !self.has_current => self.private = Some(b'<'),
            _ => {
                self.push_param();
                self.state = ParserState::Ground;
                self.finish_csi(b, out);
            }
        }
    }

    fn finish_csi(&mut self, final_byte: u8, out: &mut Vec<Event>) {
        if self.private == Some(b'<') {
            self.finish_sgr_mouse(final_byte, out);
            return;
        }
        match final_byte {
            b'A' => self.emit_arrow(Key::Up, out),
            b'B' => self.emit_arrow(Key::Down, out),
            b'C' => self.emit_arrow(Key::Right, out),
            b'D' => self.emit_arrow(Key::Left, out),
            b'H' => self.emit_arrow(Key::Home, out),
            b'F' => self.emit_arrow(Key::End, out),
            b'Z' => out.push(Event::Key(KeyEvent { key: Key::BackTab, mods: 0 })),
            b'I' => out.push(Event::FocusGained),
            b'O' => out.push(Event::FocusLost),
            b'~' => self.finish_tilde(out),
            _ => {}
        }
    }

    fn emit_arrow(&self, key: Key, out: &mut Vec<Event>) {
        let m = modifier_bits(self.param(1, 0));
        out.push(Event::Key(KeyEvent { key, mods: m }));
    }

    fn finish_tilde(&mut self, out: &mut Vec<Event>) {
        let code = self.param(0, 0);
        if code == 200 {
            self.paste_buf.clear();
            self.paste_close = b"\x1b[201~".to_vec();
            self.state = ParserState::Paste;
            return;
        }
        let m = modifier_bits(self.param(1, 0));
        let key = match code {
            1 | 7 => Some(Key::Home),
            2 => Some(Key::Insert),
            3 => Some(Key::Delete),
            4 | 8 => Some(Key::End),
            5 => Some(Key::PageUp),
            6 => Some(Key::PageDown),
            11 => Some(Key::F(1)),
            12 => Some(Key::F(2)),
            13 => Some(Key::F(3)),
            14 => Some(Key::F(4)),
            15 => Some(Key::F(5)),
            17 => Some(Key::F(6)),
            18 => Some(Key::F(7)),
            19 => Some(Key::F(8)),
            20 => Some(Key::F(9)),
            21 => Some(Key::F(10)),
            23 => Some(Key::F(11)),
            24 => Some(Key::F(12)),
            _ => None,
        };
        if let Some(key) = key {
            out.push(Event::Key(KeyEvent { key, mods: m }));
        }
    }

    fn finish_sgr_mouse(&mut self, final_byte: u8, out: &mut Vec<Event>) {
        let b = self.param(0, 0);
        let x = self.param(1, 1).saturating_sub(1);
        let y = self.param(2, 1).saturating_sub(1);
        let pos = Pos { x, y };
        let m = modifier_bits(((b >> 2) & 0x7).saturating_add(1));
        let btn = (b & 0x3) as u8;
        let button = match btn {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            _ => MouseButton::Right,
        };
        let kind = if b & 0x40 != 0 {
            if btn == 0 {
                MouseKind::Scroll { dx: 0, dy: -1 }
            } else {
                MouseKind::Scroll { dx: 0, dy: 1 }
            }
        } else if b & 0x20 != 0 {
            MouseKind::Drag(button)
        } else if final_byte == b'm' {
            MouseKind::Up(button)
        } else {
            MouseKind::Down(button)
        };
        out.push(Event::Mouse(MouseEvent { kind, pos, mods: m, clicks: 1 }));
    }

    fn feed_paste(&mut self, b: u8, out: &mut Vec<Event>) {
        self.paste_buf.push(b as char);
        if self.paste_buf.as_bytes().ends_with(self.paste_close.as_slice()) {
            let end = self.paste_buf.len() - self.paste_close.len();
            let content = self.paste_buf[..end].to_string();
            out.push(Event::Paste(content));
            self.state = ParserState::Ground;
        }
    }
}
//#endregion ???Parse
