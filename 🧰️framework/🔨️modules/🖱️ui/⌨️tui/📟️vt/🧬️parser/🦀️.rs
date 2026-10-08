use super::screen::{Charset, VtScreen};

const MAX_PARAMS: usize = 32;
const MAX_STRING: usize = 128 * 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Ground,
    Escape,
    EscapeIntermediate,
    Csi,
    CsiIgnore,
    Osc,
    Dcs,
    Ignored,
}

/// 🔠️ Incremental VT output decoder (C0, ESC, CSI with colon sub-parameters, OSC, DCS) driving a `VtScreen`.
#[derive(Clone)]
pub struct VtParser {
    state: State,
    string_esc: bool,
    params: Vec<i32>,
    subs: Vec<Vec<i32>>,
    current: i32,
    has_current: bool,
    main: i32,
    sub_values: Vec<i32>,
    in_sub: bool,
    marker: u8,
    intermediate: u8,
    osc: Vec<u8>,
    code: u32,
    need: u8,
}

impl Default for VtParser {
    fn default() -> Self {
        Self::new()
    }
}

fn decode_base64(text: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len() / 4 * 3);
    let (mut bits, mut have) = (0u32, 0u32);
    for &b in text {
        let value = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            _ => continue,
        };
        bits = (bits << 6) | u32::from(value);
        have += 6;
        if have >= 8 {
            have -= 8;
            out.push((bits >> have) as u8);
            bits &= (1 << have) - 1;
        }
    }
    out
}

impl VtParser {
    pub fn new() -> Self {
        Self { state: State::Ground, string_esc: false, params: Vec::new(), subs: Vec::new(), current: 0, has_current: false, main: 0, sub_values: Vec::new(), in_sub: false, marker: 0, intermediate: 0, osc: Vec::new(), code: 0, need: 0 }
    }

    fn reset_sequence(&mut self) {
        self.params.clear();
        self.subs.clear();
        self.current = 0;
        self.has_current = false;
        self.main = 0;
        self.sub_values.clear();
        self.in_sub = false;
        self.marker = 0;
        self.intermediate = 0;
    }

    fn push_param(&mut self) {
        if self.params.len() < MAX_PARAMS {
            if self.in_sub {
                self.sub_values.push(self.current);
                self.params.push(self.main);
                self.subs.push(std::mem::take(&mut self.sub_values));
            } else {
                self.params.push(self.current);
                self.subs.push(Vec::new());
            }
        }
        self.current = 0;
        self.has_current = false;
        self.main = 0;
        self.in_sub = false;
        self.sub_values.clear();
    }

    fn param(&self, index: usize, default: i32) -> i32 {
        match self.params.get(index).copied() {
            Some(0) | None => default,
            Some(value) => value,
        }
    }

    /// 🚰️ Feeds raw child bytes into `screen`.
    pub fn feed(&mut self, bytes: &[u8], screen: &mut VtScreen) {
        for &b in bytes {
            self.feed_byte(b, screen);
        }
    }

    fn feed_byte(&mut self, b: u8, screen: &mut VtScreen) {
        match self.state {
            State::Ground => self.feed_ground(b, screen),
            State::Escape => self.feed_escape(b, screen),
            State::EscapeIntermediate => self.feed_escape_intermediate(b, screen),
            State::Csi => self.feed_csi(b, screen),
            State::CsiIgnore => self.feed_csi_ignore(b, screen),
            State::Osc | State::Dcs | State::Ignored => self.feed_string(b, screen),
        }
    }

    fn execute(&mut self, b: u8, screen: &mut VtScreen) {
        match b {
            0x07 => screen.bell(),
            0x08 => screen.backspace(),
            0x09 => screen.tab_forward(1),
            0x0a..=0x0c => screen.linefeed(),
            0x0d => screen.carriage_return(),
            0x0e => screen.shift_charset(true),
            0x0f => screen.shift_charset(false),
            _ => {}
        }
    }

    fn feed_ground(&mut self, b: u8, screen: &mut VtScreen) {
        if self.need > 0 {
            if b & 0xc0 == 0x80 {
                self.code = (self.code << 6) | u32::from(b & 0x3f);
                self.need -= 1;
                if self.need == 0 {
                    screen.print(char::from_u32(self.code).unwrap_or('\u{fffd}'));
                }
                return;
            }
            self.need = 0;
            screen.print('\u{fffd}');
        }
        match b {
            0x1b => {
                self.reset_sequence();
                self.state = State::Escape;
            }
            0x00..=0x1f => self.execute(b, screen),
            0x20..=0x7e => screen.print(b as char),
            0x7f => {}
            0xc2..=0xdf => {
                self.code = u32::from(b & 0x1f);
                self.need = 1;
            }
            0xe0..=0xef => {
                self.code = u32::from(b & 0x0f);
                self.need = 2;
            }
            0xf0..=0xf4 => {
                self.code = u32::from(b & 0x07);
                self.need = 3;
            }
            _ => screen.print('\u{fffd}'),
        }
    }

    fn feed_escape(&mut self, b: u8, screen: &mut VtScreen) {
        self.state = State::Ground;
        match b {
            0x1b => {
                self.reset_sequence();
                self.state = State::Escape;
            }
            0x18 | 0x1a => {}
            0x00..=0x1f => {
                self.execute(b, screen);
                self.state = State::Escape;
            }
            b'[' => {
                self.reset_sequence();
                self.state = State::Csi;
            }
            b']' => {
                self.osc.clear();
                self.state = State::Osc;
            }
            b'P' => self.state = State::Dcs,
            b'X' | b'^' | b'_' => self.state = State::Ignored,
            0x20..=0x2f => {
                self.intermediate = b;
                self.state = State::EscapeIntermediate;
            }
            b'7' => screen.save_cursor(),
            b'8' => screen.restore_cursor(),
            b'=' => screen.input.app_keypad = true,
            b'>' => screen.input.app_keypad = false,
            b'D' => screen.index(),
            b'E' => screen.next_line(),
            b'H' => screen.set_tab_stop(),
            b'M' => screen.reverse_index(),
            b'Z' => screen.reply(b"\x1b[?62;22c"),
            b'c' => screen.hard_reset(),
            _ => {}
        }
    }

    fn feed_escape_intermediate(&mut self, b: u8, screen: &mut VtScreen) {
        match b {
            0x20..=0x2f => self.intermediate = b,
            0x30..=0x7e => {
                self.state = State::Ground;
                let charset = if b == b'0' { Charset::Graphics } else { Charset::Ascii };
                match (self.intermediate, b) {
                    (b'(', _) => screen.designate_charset(0, charset),
                    (b')', _) => screen.designate_charset(1, charset),
                    (b'#', b'8') => screen.alignment_test(),
                    _ => {}
                }
            }
            0x1b => {
                self.reset_sequence();
                self.state = State::Escape;
            }
            0x18 | 0x1a => self.state = State::Ground,
            _ => {}
        }
    }

    fn feed_csi(&mut self, b: u8, screen: &mut VtScreen) {
        match b {
            b'0'..=b'9' => {
                self.current = self.current.saturating_mul(10).saturating_add(i32::from(b - b'0'));
                self.has_current = true;
            }
            b':' => {
                if self.in_sub {
                    self.sub_values.push(self.current);
                } else {
                    self.main = self.current;
                    self.in_sub = true;
                }
                self.current = 0;
                self.has_current = false;
            }
            b';' => self.push_param(),
            0x3c..=0x3f => {
                if self.params.is_empty() && !self.has_current && !self.in_sub && self.intermediate == 0 && self.marker == 0 {
                    self.marker = b;
                } else {
                    self.state = State::CsiIgnore;
                }
            }
            0x20..=0x2f => self.intermediate = b,
            0x40..=0x7e => {
                self.push_param();
                self.state = State::Ground;
                self.dispatch_csi(b, screen);
            }
            0x1b => {
                self.reset_sequence();
                self.state = State::Escape;
            }
            0x18 | 0x1a => self.state = State::Ground,
            0x00..=0x1f => self.execute(b, screen),
            _ => {}
        }
    }

    fn feed_csi_ignore(&mut self, b: u8, screen: &mut VtScreen) {
        match b {
            0x40..=0x7e => self.state = State::Ground,
            0x1b => {
                self.reset_sequence();
                self.state = State::Escape;
            }
            0x18 | 0x1a => self.state = State::Ground,
            0x00..=0x1f => self.execute(b, screen),
            _ => {}
        }
    }

    fn feed_string(&mut self, b: u8, screen: &mut VtScreen) {
        if self.string_esc {
            self.string_esc = false;
            self.end_string(screen);
            if b == b'\\' {
                self.state = State::Ground;
            } else {
                self.state = State::Escape;
                self.feed_escape(b, screen);
            }
            return;
        }
        match b {
            0x1b => self.string_esc = true,
            0x07 => {
                self.end_string(screen);
                self.state = State::Ground;
            }
            0x18 | 0x1a => {
                self.osc.clear();
                self.state = State::Ground;
            }
            _ => {
                if self.state == State::Osc && self.osc.len() < MAX_STRING {
                    self.osc.push(b);
                }
            }
        }
    }

    fn end_string(&mut self, screen: &mut VtScreen) {
        if self.state == State::Osc {
            self.finish_osc(screen);
        }
        self.osc.clear();
    }

    fn finish_osc(&mut self, screen: &mut VtScreen) {
        let text = String::from_utf8_lossy(&self.osc).into_owned();
        let (code, payload) = text.split_once(';').unwrap_or((text.as_str(), ""));
        match code {
            "0" | "2" => screen.title = Some(payload.to_string()),
            "7" => screen.cwd = Some(payload.to_string()),
            "10" if payload == "?" => screen.report_color(10, false),
            "11" if payload == "?" => screen.report_color(11, true),
            "52" => {
                let data = payload.rsplit(';').next().unwrap_or("");
                if data != "?" && !data.is_empty() {
                    screen.set_clipboard(String::from_utf8_lossy(&decode_base64(data.as_bytes())).into_owned());
                }
            }
            _ => {}
        }
    }

    fn each_mode(&self, screen: &mut VtScreen, on: bool, private: bool) {
        for &mode in &self.params {
            if private {
                screen.set_private_mode(mode, on);
            } else {
                screen.set_ansi_mode(mode, on);
            }
        }
    }

    fn dispatch_csi(&mut self, final_byte: u8, screen: &mut VtScreen) {
        let p = |i: usize, d: i32| self.param(i, d);
        match (self.marker, self.intermediate, final_byte) {
            (0, 0, b'A') => screen.cursor_up(p(0, 1)),
            (0, 0, b'B' | b'e') => screen.cursor_down(p(0, 1)),
            (0, 0, b'C' | b'a') => screen.cursor_forward(p(0, 1)),
            (0, 0, b'D') => screen.cursor_back(p(0, 1)),
            (0, 0, b'E') => {
                screen.cursor_down(p(0, 1));
                screen.carriage_return();
            }
            (0, 0, b'F') => {
                screen.cursor_up(p(0, 1));
                screen.carriage_return();
            }
            (0, 0, b'G' | b'`') => screen.cursor_column(p(0, 1)),
            (0, 0, b'H' | b'f') => screen.cup(p(0, 1), p(1, 1)),
            (0, 0, b'I') => screen.tab_forward(p(0, 1)),
            (0, 0, b'J') | (b'?', 0, b'J') => screen.erase_display(p(0, 0)),
            (0, 0, b'K') | (b'?', 0, b'K') => screen.erase_line(p(0, 0)),
            (0, 0, b'L') => screen.insert_lines(p(0, 1)),
            (0, 0, b'M') => screen.delete_lines(p(0, 1)),
            (0, 0, b'P') => screen.delete_cells(p(0, 1)),
            (0, 0, b'S') if self.params.len() <= 1 => screen.scroll_up(p(0, 1).clamp(1, 65_535) as u16),
            (0, 0, b'T') if self.params.len() <= 1 => screen.scroll_down(p(0, 1).clamp(1, 65_535) as u16),
            (0, 0, b'X') => screen.erase_cells(p(0, 1)),
            (0, 0, b'Z') => screen.tab_back(p(0, 1)),
            (0, 0, b'@') => screen.insert_cells(p(0, 1)),
            (0, 0, b'b') => screen.repeat_last(p(0, 1)),
            (0, 0, b'c') if p(0, 0) == 0 => screen.reply(b"\x1b[?62;22c"),
            (0, 0, b'd') => screen.cursor_row(p(0, 1)),
            (0, 0, b'g') => match p(0, 0) {
                0 => screen.clear_tab_stops(false),
                3 => screen.clear_tab_stops(true),
                _ => {}
            },
            (0, 0, b'h') => self.each_mode(screen, true, false),
            (0, 0, b'l') => self.each_mode(screen, false, false),
            (0, 0, b'm') => screen.apply_sgr(&self.params, &self.subs),
            (0, 0, b'n') => match p(0, 0) {
                5 => screen.reply(b"\x1b[0n"),
                6 => screen.report_cursor(false),
                _ => {}
            },
            (0, 0, b'r') => screen.set_scroll_region(p(0, 1), p(1, i32::from(screen.size.height))),
            (0, 0, b's') => screen.save_cursor(),
            (0, 0, b't') => {
                if p(0, 0) == 18 {
                    screen.report_text_area();
                }
            }
            (0, 0, b'u') => screen.restore_cursor(),
            (0, b' ', b'q') => screen.set_cursor_style(p(0, 0)),
            (0, b'!', b'p') => screen.soft_reset(),
            (0, b'$', b'p') => {
                let mode = p(0, 0);
                let text = format!("\x1b[{mode};{}$y", screen.ansi_mode_state(mode));
                screen.reply(text.as_bytes());
            }
            (b'?', 0, b'h') => self.each_mode(screen, true, true),
            (b'?', 0, b'l') => self.each_mode(screen, false, true),
            (b'?', 0, b'n') => {
                if p(0, 0) == 6 {
                    screen.report_cursor(true);
                }
            }
            (b'?', b'$', b'p') => {
                let mode = p(0, 0);
                let text = format!("\x1b[?{mode};{}$y", screen.private_mode_state(mode));
                screen.reply(text.as_bytes());
            }
            (b'?', 0, b'u') => screen.reply(b"\x1b[?0u"),
            (b'>', 0, b'c') => screen.reply(b"\x1b[>0;0;0c"),
            (b'>', 0, b'q') => screen.reply(b"\x1bP>|semio-tui\x1b\\"),
            _ => {}
        }
    }
}
