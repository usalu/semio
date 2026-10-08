use crate::tui::backend::ColorDepth;
use crate::tui::cell::{Cell, CellBuffer, DiffRun};
use crate::tui::theme::Rgb;
use crate::tui::widget::{CursorShape, CursorSpec};
use std::borrow::Cow;
use std::time::Duration;

//#region 📤️Emit
/// 📤️ A batch of raw ANSI bytes ready to write to a terminal (or feed to xterm.js).
#[derive(Default, Clone)]
pub struct AnsiPatch(pub String);

impl AnsiPatch {
    /// 🕳️ Whether the patch repaints nothing.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

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

/// ✍️ Emits the minimal ANSI needed to repaint `runs` of `next` onto a terminal.
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
            next.push_glyph(x, run.y, &mut out.0);
            x += u16::from(c.width.max(1));
        }
    }
}
//#endregion 📤️Emit

//#region 🎚️Modes
struct SessionMode {
    code: u16,
    held: bool,
}

const SESSION_MODES: &[SessionMode] = &[
    SessionMode { code: 1049, held: true },
    SessionMode { code: 25, held: false },
    SessionMode { code: 1000, held: true },
    SessionMode { code: 1003, held: true },
    SessionMode { code: 1006, held: true },
    SessionMode { code: 1004, held: true },
    SessionMode { code: 2004, held: true },
];

/// 🔄️ Begin of a synchronized-output frame (DEC private mode 2026).
pub const SYNC_BEGIN: &str = "\x1b[?2026h";

/// 🔚️ End of a synchronized-output frame (DEC private mode 2026).
pub const SYNC_END: &str = "\x1b[?2026l";

fn mode_sequence(code: u16, set: bool) -> String {
    format!("\x1b[?{code}{}", if set { 'h' } else { 'l' })
}

/// 🚪️ Enters the alternate screen, hides the cursor, resets cursor-key modes, and enables any-motion SGR mouse, focus and paste reporting.
pub fn setup_sequence() -> &'static str {
    static SETUP: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    SETUP.get_or_init(|| {
        let mut out = String::from("\x1b[?1l\x1b>");
        for mode in SESSION_MODES {
            out.push_str(&mode_sequence(mode.code, mode.held));
        }
        out.push_str("\x1b[2J");
        out
    })
}

/// 🏠️ Restores the primary screen and every default mode, in reverse order of setup.
pub fn teardown_sequence() -> &'static str {
    static TEARDOWN: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    TEARDOWN.get_or_init(|| {
        let mut out = String::from(SYNC_END);
        for mode in SESSION_MODES.iter().rev() {
            out.push_str(&mode_sequence(mode.code, !mode.held));
        }
        out.push_str("\x1b[0 q\x1b[0m");
        out
    })
}
//#endregion 🎚️Modes

//#region 🖍️Cursor
/// 🖍️ Tracks what the terminal cursor shows and emits only the escapes that change it, wrapped around a frame.
#[derive(Default)]
pub struct CursorEmitter {
    visible: bool,
    style: Option<(CursorShape, bool)>,
    pos: Option<Pos>,
}

fn cursor_style_code(shape: CursorShape, blink: bool) -> u8 {
    let base = match shape {
        CursorShape::Block => 1,
        CursorShape::Underline => 3,
        CursorShape::Bar => 5,
    };
    base + u8::from(!blink)
}

impl CursorEmitter {
    /// 🧼️ Forgets everything known about the terminal cursor, as after entering the alternate screen.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// 🎞️ Composes one frame: hide while drawing, paint `patch`, then place, shape and show (or hide) the cursor.
    pub fn frame(&mut self, patch: &str, cursor: Option<CursorSpec>, synchronized: bool) -> String {
        let mut body = String::with_capacity(patch.len() + 48);
        let draws = !patch.is_empty();
        if draws && self.visible {
            body.push_str("\x1b[?25l");
            self.visible = false;
        }
        body.push_str(patch);
        if draws {
            self.pos = None;
        }
        match cursor {
            Some(spec) => {
                if self.style != Some((spec.shape, spec.blink)) {
                    body.push_str(&format!("\x1b[{} q", cursor_style_code(spec.shape, spec.blink)));
                    self.style = Some((spec.shape, spec.blink));
                }
                if self.pos != Some(spec.pos) {
                    body.push_str(&format!("\x1b[{};{}H", spec.pos.y + 1, spec.pos.x + 1));
                    self.pos = Some(spec.pos);
                }
                if !self.visible {
                    body.push_str("\x1b[?25h");
                    self.visible = true;
                }
            }
            None if self.visible => {
                body.push_str("\x1b[?25l");
                self.visible = false;
            }
            None => {}
        }
        if body.is_empty() || !synchronized {
            return body;
        }
        format!("{SYNC_BEGIN}{body}{SYNC_END}")
    }
}
//#endregion 🖍️Cursor

//#region 🌈️Color
fn scale_to_cube(value: u8) -> u8 {
    (f64::from(value) / 255.0 * 5.0).round() as u8
}

/// 🎨️ Nearest xterm 256-colour palette index for a truecolor value (greys use the 24-step ramp).
pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return ((f64::from(r) - 8.0) / 247.0 * 24.0).round() as u8 + 232;
    }
    16 + 36 * scale_to_cube(r) + 6 * scale_to_cube(g) + scale_to_cube(b)
}

/// 🔅️ The SGR foreground code (30-37 or 90-97) nearest to a truecolor value.
pub fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> u8 {
    let brightness = (f64::from(r.max(g).max(b)) / 255.0 * 100.0 / 50.0).round() as u8;
    if brightness == 0 {
        return 30;
    }
    let half = |channel: u8| (f64::from(channel) / 255.0).round() as u8;
    let code = 30 + ((half(b) << 2) | (half(g) << 1) | half(r));
    if brightness == 2 {
        code + 60
    } else {
        code
    }
}

/// 🧊️ The RGB value of an xterm 256-colour palette index above the 16 system colours.
pub fn ansi256_to_rgb(index: u8) -> Rgb {
    if index >= 232 {
        let level = 8 + 10 * (index - 232);
        return [level, level, level];
    }
    const LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];
    let cube = index.saturating_sub(16);
    [LEVELS[usize::from(cube / 36)], LEVELS[usize::from(cube % 36 / 6)], LEVELS[usize::from(cube % 6)]]
}

fn ansi16_code(base: u8, background: bool) -> String {
    (base + if background { 10 } else { 0 }).to_string()
}

fn indexed_to_ansi16(index: u8, background: bool) -> String {
    let base = match index {
        0..=7 => 30 + index,
        8..=15 => 90 + index - 8,
        _ => {
            let [r, g, b] = ansi256_to_rgb(index);
            rgb_to_ansi16(r, g, b)
        }
    };
    ansi16_code(base, background)
}

fn push_token(out: &mut String, wrote: &mut bool, token: &str) {
    if *wrote {
        out.push(';');
    }
    out.push_str(token);
    *wrote = true;
}

fn rewrite_sgr(params: &str, depth: ColorDepth, out: &mut String) {
    let mut wrote = false;
    let mut parts = params.split(';');
    out.push_str("\x1b[");
    while let Some(token) = parts.next() {
        let background = match token {
            "38" => false,
            "48" => true,
            _ => {
                let number = token.parse::<u16>().unwrap_or(u16::MAX);
                let basic = matches!(number, 30..=37 | 40..=47 | 90..=97 | 100..=107);
                if !(basic && depth == ColorDepth::Monochrome) {
                    push_token(out, &mut wrote, token);
                }
                continue;
            }
        };
        let mode = parts.next();
        let channel = |part: Option<&str>| part.and_then(|value| value.parse::<u8>().ok());
        let rewritten = match mode {
            Some("2") => {
                let (r, g, b) = (channel(parts.next()), channel(parts.next()), channel(parts.next()));
                match (r, g, b) {
                    (Some(r), Some(g), Some(b)) => Some(match depth {
                        ColorDepth::Ansi256 => Some(format!("{};5;{}", if background { 48 } else { 38 }, rgb_to_ansi256(r, g, b))),
                        ColorDepth::Ansi16 => Some(ansi16_code(rgb_to_ansi16(r, g, b), background)),
                        _ => None,
                    }),
                    _ => None,
                }
            }
            Some("5") => channel(parts.next()).map(|index| match depth {
                ColorDepth::Ansi256 => Some(format!("{};5;{index}", if background { 48 } else { 38 })),
                ColorDepth::Ansi16 => Some(indexed_to_ansi16(index, background)),
                _ => None,
            }),
            _ => None,
        };
        if let Some(Some(text)) = rewritten {
            push_token(out, &mut wrote, &text);
        }
    }
    out.push('m');
}

/// 🌈️ Rewrites the truecolor SGR sequences of a patch down to the colours a terminal of `depth` renders.
pub fn quantize_patch(patch: &str, depth: ColorDepth) -> Cow<'_, str> {
    if depth == ColorDepth::TrueColor {
        return Cow::Borrowed(patch);
    }
    let mut out = String::with_capacity(patch.len());
    let mut rest = patch;
    while let Some(start) = rest.find("\x1b[") {
        out.push_str(&rest[..start]);
        let tail = &rest[start + 2..];
        match tail.bytes().position(|byte| (0x40..=0x7e).contains(&byte)) {
            Some(end) if tail.as_bytes()[end] == b'm' => {
                rewrite_sgr(&tail[..end], depth, &mut out);
                rest = &tail[end + 1..];
            }
            Some(end) => {
                out.push_str(&rest[start..start + 2 + end + 1]);
                rest = &tail[end + 1..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    Cow::Owned(out)
}
//#endregion 🌈️Color

//#region 🔤️Parse
use crate::tui::event::{mods, Event, Key, KeyEvent, KeypadKey, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::Pos;

/// 🪣️ Most bytes one bracketed paste may carry; the rest is dropped until the paste ends.
pub const PASTE_LIMIT: usize = 1 << 20;

/// ⏱️ How long a lone ESC waits for a follow-up byte before it is the Escape key.
pub const ESCAPE_TIMEOUT: Duration = Duration::from_millis(30);

/// ⌛️ How long a half-received control sequence waits before it is dropped.
pub const SEQUENCE_TIMEOUT: Duration = Duration::from_millis(250);

/// 🛑️ How long a bracketed paste may stay silent before what arrived is delivered.
pub const PASTE_TIMEOUT: Duration = Duration::from_secs(2);

const PASTE_END: &[u8] = b"\x1b[201~";
const MAX_PARAMS: usize = 16;

#[derive(Clone, Copy, PartialEq, Debug)]
enum ParserState {
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
    state: ParserState,
    params: [u16; MAX_PARAMS],
    subs: [[u16; 2]; MAX_PARAMS],
    count: usize,
    current: u16,
    current_subs: [u16; 2],
    sub_index: usize,
    has_current: bool,
    private: Option<u8>,
    intermediate: bool,
    invalid: bool,
    linux_function: bool,
    utf8_buf: [u8; 4],
    utf8_len: u8,
    utf8_need: u8,
    utf8_mods: u8,
    paste: Vec<u8>,
    paste_matched: usize,
    paste_truncated: bool,
    x10: [u8; 3],
    x10_len: usize,
    last_button: MouseButton,
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

fn function_key_from_tilde(code: u16) -> Option<Key> {
    Some(Key::F(match code {
        11..=15 => (code - 10) as u8,
        17..=21 => (code - 11) as u8,
        23..=26 => (code - 12) as u8,
        28 | 29 => (code - 13) as u8,
        31..=34 => (code - 14) as u8,
        _ => return None,
    }))
}

fn key_from_codepoint(code: u16) -> Option<Key> {
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

    fn end_param(&mut self) {
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

    fn param(&self, i: usize, default: u16) -> u16 {
        if i < self.count {
            self.params[i]
        } else {
            default
        }
    }

    fn mods_param(&self, i: usize) -> u8 {
        modifier_bits(self.param(i, 0))
    }

    fn is_release(&self, i: usize) -> bool {
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

    fn key(&self, key: Key, mods: u8, out: &mut Vec<Event>) {
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

    fn feed_ground(&mut self, b: u8, out: &mut Vec<Event>) {
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

    fn feed_csi(&mut self, b: u8, out: &mut Vec<Event>) {
        match b {
            b'0'..=b'9' => {
                if self.intermediate {
                    self.invalid = true;
                } else if self.sub_index == 0 {
                    self.current = self.current.saturating_mul(10).saturating_add(u16::from(b - b'0'));
                    self.has_current = true;
                } else if self.sub_index <= 2 {
                    let slot = &mut self.current_subs[self.sub_index - 1];
                    *slot = slot.saturating_mul(10).saturating_add(u16::from(b - b'0'));
                    self.has_current = true;
                }
            }
            b';' => {
                if self.intermediate {
                    self.invalid = true;
                }
                self.end_param();
            }
            b':' => {
                if self.intermediate {
                    self.invalid = true;
                }
                self.sub_index += 1;
                self.has_current = true;
            }
            b'<'..=b'?' => {
                if self.private.is_none() && self.count == 0 && !self.has_current && !self.intermediate {
                    self.private = Some(b);
                } else {
                    self.invalid = true;
                }
            }
            b'[' if self.count == 0 && !self.has_current && self.private.is_none() && !self.intermediate && !self.linux_function => self.linux_function = true,
            0x20..=0x2f => self.intermediate = true,
            0x40..=0x7e => {
                if self.has_current || self.count > 0 {
                    self.end_param();
                }
                self.state = ParserState::Ground;
                if !self.invalid {
                    self.finish_csi(b, out);
                }
            }
            0x1b => self.state = ParserState::Escape,
            0x18 | 0x1a => self.state = ParserState::Ground,
            0x80..=0xff => {
                self.state = ParserState::Ground;
                self.feed_ground(b, out);
            }
            _ => {}
        }
    }

    fn finish_csi(&mut self, final_byte: u8, out: &mut Vec<Event>) {
        if self.private == Some(b'<') {
            if matches!(final_byte, b'M' | b'm') {
                let (code, col, row) = (self.param(0, 0), self.param(1, 1), self.param(2, 1));
                self.mouse(code, col, row, final_byte == b'm', false, out);
            }
            return;
        }
        if self.private.is_some() || self.intermediate {
            return;
        }
        if self.linux_function {
            if let b'A'..=b'E' = final_byte {
                self.key(Key::F(final_byte - b'A' + 1), 0, out);
            }
            return;
        }
        let with_mods = self.count >= 2;
        match final_byte {
            b'A' | b'B' | b'C' | b'D' | b'H' | b'F' => {
                if self.is_release(1) {
                    return;
                }
                let key = match final_byte {
                    b'A' => Key::Up,
                    b'B' => Key::Down,
                    b'C' => Key::Right,
                    b'D' => Key::Left,
                    b'H' => Key::Home,
                    _ => Key::End,
                };
                self.key(key, self.mods_param(1), out);
            }
            b'P' | b'Q' | b'S' if with_mods => {
                let number = match final_byte {
                    b'P' => 1,
                    b'Q' => 2,
                    _ => 4,
                };
                self.key(Key::F(number), self.mods_param(1), out);
            }
            b'R' if with_mods && self.param(0, 0) == 1 && self.param(1, 0) >= 2 => self.key(Key::F(3), self.mods_param(1), out),
            b'Z' => self.key(Key::BackTab, self.mods_param(1) & !mods::SHIFT, out),
            b'I' if self.count == 0 => out.push(Event::FocusGained),
            b'O' if self.count == 0 => out.push(Event::FocusLost),
            b'M' if self.count == 0 => {
                self.x10_len = 0;
                self.state = ParserState::X10Mouse;
            }
            b'M' if self.count == 3 => {
                let (code, col, row) = (self.param(0, 32).saturating_sub(32), self.param(1, 1), self.param(2, 1));
                self.mouse(code, col, row, false, true, out);
            }
            b'u' => self.finish_codepoint(self.param(0, 0), self.subs[0][0], 1, out),
            b'~' => self.finish_tilde(out),
            _ => {}
        }
    }

    fn finish_codepoint(&mut self, code: u16, shifted: u16, mods_index: usize, out: &mut Vec<Event>) {
        if self.is_release(mods_index) {
            return;
        }
        let m = self.mods_param(mods_index);
        let Some(key) = key_from_codepoint(code) else { return };
        match key {
            Key::Tab if m & mods::SHIFT != 0 => self.key(Key::BackTab, m & !mods::SHIFT, out),
            Key::Char(_) if shifted != 0 && m & mods::SHIFT != 0 => match key_from_codepoint(shifted) {
                Some(shifted_key) => self.key(shifted_key, m & !mods::SHIFT, out),
                None => self.key(key, m, out),
            },
            _ => self.key(key, m, out),
        }
    }

    fn finish_tilde(&mut self, out: &mut Vec<Event>) {
        let code = self.param(0, 0);
        match code {
            200 => {
                self.paste.clear();
                self.paste_matched = 0;
                self.paste_truncated = false;
                self.state = ParserState::Paste;
            }
            201 => {}
            27 => self.finish_codepoint(self.param(2, 0), 0, 1, out),
            _ => {
                if self.is_release(1) {
                    return;
                }
                let key = match code {
                    1 | 7 => Some(Key::Home),
                    2 => Some(Key::Insert),
                    3 => Some(Key::Delete),
                    4 | 8 => Some(Key::End),
                    5 => Some(Key::PageUp),
                    6 => Some(Key::PageDown),
                    _ => function_key_from_tilde(code),
                };
                if let Some(key) = key {
                    self.key(key, self.mods_param(1), out);
                }
            }
        }
    }

    fn feed_x10(&mut self, b: u8, out: &mut Vec<Event>) {
        self.x10[self.x10_len] = b;
        self.x10_len += 1;
        if self.x10_len < 3 {
            return;
        }
        self.state = ParserState::Ground;
        let [code, col, row] = self.x10.map(|byte| u16::from(byte).saturating_sub(32));
        self.mouse(code, col, row, false, true, out);
    }

    fn mouse(&mut self, code: u16, col: u16, row: u16, sgr_release: bool, legacy: bool, out: &mut Vec<Event>) {
        if code & 0x80 != 0 {
            return;
        }
        let pos = Pos { x: col.saturating_sub(1), y: row.saturating_sub(1) };
        let mods = modifier_bits(((code >> 2) & 0x7) + 1);
        let low = code & 0x3;
        let named = match low {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => self.last_button,
        };
        let kind = if code & 0x40 != 0 {
            match low {
                0 => MouseKind::Scroll { dx: 0, dy: -1 },
                1 => MouseKind::Scroll { dx: 0, dy: 1 },
                2 => MouseKind::Scroll { dx: -1, dy: 0 },
                _ => MouseKind::Scroll { dx: 1, dy: 0 },
            }
        } else if code & 0x20 != 0 {
            if low == 3 {
                MouseKind::Move
            } else {
                self.last_button = named;
                MouseKind::Drag(named)
            }
        } else if sgr_release || (legacy && low == 3) {
            MouseKind::Up(named)
        } else {
            self.last_button = named;
            MouseKind::Down(named)
        };
        out.push(Event::Mouse(MouseEvent { kind, pos, mods, clicks: 1 }));
    }

    fn push_paste(&mut self, bytes: &[u8]) {
        if self.paste.len() + bytes.len() > PASTE_LIMIT {
            self.paste_truncated = true;
        } else {
            self.paste.extend_from_slice(bytes);
        }
    }

    fn feed_paste(&mut self, b: u8, out: &mut Vec<Event>) {
        if b == PASTE_END[self.paste_matched] {
            self.paste_matched += 1;
            if self.paste_matched == PASTE_END.len() {
                self.paste_matched = 0;
                self.finish_paste(out);
                self.state = ParserState::Ground;
            }
            return;
        }
        if self.paste_matched > 0 {
            let matched = self.paste_matched;
            self.paste_matched = 0;
            self.push_paste(&PASTE_END[..matched]);
            if b == PASTE_END[0] {
                self.paste_matched = 1;
                return;
            }
        }
        self.push_paste(&[b]);
    }

    fn finish_paste(&mut self, out: &mut Vec<Event>) {
        let bytes = std::mem::take(&mut self.paste);
        let text = match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(error) if self.paste_truncated && error.utf8_error().error_len().is_none() => {
                let valid = error.utf8_error().valid_up_to();
                String::from_utf8_lossy(&error.as_bytes()[..valid]).into_owned()
            }
            Err(error) => String::from_utf8_lossy(error.as_bytes()).into_owned(),
        };
        self.paste_truncated = false;
        self.paste_matched = 0;
        out.push(Event::Paste(text));
    }
}
//#endregion 🔤️Parse

//#region 🖱️Clicks
/// 🖱️ Longest gap between two presses that still count as a double or triple click.
pub const CLICK_WINDOW_MS: u64 = 400;

/// 🔢️ Counts consecutive presses of one button on one cell and stamps `MouseEvent::clicks`.
#[derive(Default)]
pub struct ClickCounter {
    last: Option<(MouseButton, Pos, u64)>,
    count: u8,
}

impl ClickCounter {
    /// 🏷️ Sets `clicks` on one mouse event observed at `now_ms` (monotonic milliseconds).
    pub fn stamp(&mut self, event: &mut MouseEvent, now_ms: u64) {
        match event.kind {
            MouseKind::Down(button) => {
                let repeated = self.last.is_some_and(|(last_button, last_pos, last_ms)| last_button == button && last_pos == event.pos && now_ms.saturating_sub(last_ms) <= CLICK_WINDOW_MS);
                self.count = if repeated && self.count < 3 { self.count + 1 } else { 1 };
                self.last = Some((button, event.pos, now_ms));
                event.clicks = self.count;
            }
            MouseKind::Up(_) | MouseKind::Drag(_) => event.clicks = self.count.max(1),
            MouseKind::Move | MouseKind::Scroll { .. } => event.clicks = 1,
        }
    }

    /// 🧷️ Stamps every mouse event of a decoded batch.
    pub fn stamp_all(&mut self, events: &mut [Event], now_ms: u64) {
        for event in events {
            if let Event::Mouse(mouse) = event {
                self.stamp(mouse, now_ms);
            }
        }
    }
}
//#endregion 🖱️Clicks

#[cfg(test)]
include!("../🧪️tests/🔬️ansi-unit/🦀️.rs");

#[cfg(test)]
include!("../🧪️tests/⌨️input-decoding/🦀️.rs");
