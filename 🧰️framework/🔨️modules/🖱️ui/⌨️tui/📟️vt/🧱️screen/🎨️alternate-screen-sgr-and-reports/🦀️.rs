//! 🎨️ Alternate screen, sgr and reports: entering and leaving the alternate screen, the cursor style, soft and hard reset, the sgr colours and attributes and the cursor, text area and colour reports. See [`super`].

use super::*;

impl VtScreen {
    pub(super) fn enter_alternate(&mut self, save: bool) {
        if self.alt_active {
            return;
        }
        if save {
            self.save_cursor();
        }
        self.alt_active = true;
        let blank = default_blank();
        self.alt = Grid::new(self.size, blank);
        if save {
            self.cursor = Pos { x: 0, y: 0 };
            self.wrap_pending = false;
        }
    }

    pub(super) fn leave_alternate(&mut self, restore: bool, clear: bool) {
        if !self.alt_active {
            return;
        }
        if clear {
            self.alt = Grid::new(self.size, default_blank());
        }
        self.alt_active = false;
        if restore {
            self.restore_cursor();
        }
    }

    pub(in super::super) fn set_cursor_style(&mut self, style: i32) {
        let (shape, blink) = match style {
            0 | 1 => (CursorShape::Block, true),
            2 => (CursorShape::Block, false),
            3 => (CursorShape::Underline, true),
            4 => (CursorShape::Underline, false),
            5 => (CursorShape::Bar, true),
            6 => (CursorShape::Bar, false),
            _ => return,
        };
        self.modes.cursor_shape = shape;
        self.modes.cursor_blink = blink;
    }

    pub(in super::super) fn soft_reset(&mut self) {
        self.modes = Modes::default();
        self.input = InputModes::default();
        self.pen = Pen::DEFAULT;
        self.charsets = [Charset::Ascii; 2];
        self.shifted = false;
        self.scroll_top = 0;
        self.scroll_bottom = self.size.height - 1;
        self.saved = SavedCursor { pos: Pos { x: 0, y: 0 }, pen: Pen::DEFAULT, origin: false, wrap_pending: false, charsets: [Charset::Ascii; 2], shifted: false };
        self.wrap_pending = false;
    }

    pub(in super::super) fn hard_reset(&mut self) {
        let replies = std::mem::take(&mut self.replies);
        let history = std::mem::replace(&mut self.history, History::new(1));
        let cap = history.cap();
        let palette = self.palette;
        *self = Self::new(self.size, cap);
        self.history = history;
        self.replies = replies;
        self.palette = palette;
    }

    pub(in super::super) fn apply_sgr(&mut self, params: &[i32], subs: &[Vec<i32>]) {
        let mut i = 0;
        while i < params.len() {
            let sub = subs.get(i).map_or(&[][..], Vec::as_slice);
            match params[i] {
                0 => self.pen = Pen::DEFAULT,
                1 => self.pen.attrs |= attr::BOLD,
                2 => self.pen.attrs |= attr::DIM,
                3 => self.pen.attrs |= attr::ITALIC,
                4 => {
                    if sub.first() == Some(&0) {
                        self.pen.attrs &= !attr::UNDERLINE;
                    } else {
                        self.pen.attrs |= attr::UNDERLINE;
                    }
                }
                7 => self.pen.attrs |= attr::REVERSE,
                21 => self.pen.attrs |= attr::UNDERLINE,
                22 => self.pen.attrs &= !(attr::BOLD | attr::DIM),
                23 => self.pen.attrs &= !attr::ITALIC,
                24 => self.pen.attrs &= !attr::UNDERLINE,
                27 => self.pen.attrs &= !attr::REVERSE,
                30..=37 => self.set_fg(super::super::palette::XTERM_ANSI[(params[i] - 30) as usize]),
                39 => {
                    self.pen.fg = XTERM_FG;
                    self.pen.attrs |= flag::DEFAULT_FG;
                }
                40..=47 => self.set_bg(super::super::palette::XTERM_ANSI[(params[i] - 40) as usize]),
                49 => {
                    self.pen.bg = XTERM_BG;
                    self.pen.attrs |= flag::DEFAULT_BG;
                }
                90..=97 => self.set_fg(super::super::palette::XTERM_ANSI[(params[i] - 90 + 8) as usize]),
                100..=107 => self.set_bg(super::super::palette::XTERM_ANSI[(params[i] - 100 + 8) as usize]),
                code @ (38 | 48 | 58) => {
                    let (color, used) = if sub.is_empty() { Self::semicolon_color(&params[i + 1..]) } else { (Self::colon_color(sub), 0) };
                    i += used;
                    match (code, color) {
                        (38, Some(color)) => self.set_fg(color),
                        (48, Some(color)) => self.set_bg(color),
                        _ => {}
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn set_fg(&mut self, color: Rgb) {
        self.pen.fg = color;
        self.pen.attrs &= !flag::DEFAULT_FG;
    }

    fn set_bg(&mut self, color: Rgb) {
        self.pen.bg = color;
        self.pen.attrs &= !flag::DEFAULT_BG;
    }

    fn semicolon_color(rest: &[i32]) -> (Option<Rgb>, usize) {
        let channel = |v: i32| v.clamp(0, 255) as u8;
        match rest {
            [5, n, ..] => (Some(super::super::palette::color_256(channel(*n))), 2),
            [2, r, g, b, ..] => (Some([channel(*r), channel(*g), channel(*b)]), 4),
            _ => (None, 0),
        }
    }

    fn colon_color(sub: &[i32]) -> Option<Rgb> {
        let channel = |v: i32| v.clamp(0, 255) as u8;
        match sub {
            [5, n, ..] => Some(super::super::palette::color_256(channel(*n))),
            [2, r, g, b] => Some([channel(*r), channel(*g), channel(*b)]),
            [2, _, r, g, b, ..] => Some([channel(*r), channel(*g), channel(*b)]),
            _ => None,
        }
    }

    pub(in super::super) fn report_cursor(&mut self, private: bool) {
        let row = i32::from(self.cursor.y) + 1 - if self.modes.origin { i32::from(self.scroll_top) } else { 0 };
        let text = format!("\x1b[{}{};{}R", if private { "?" } else { "" }, row, i32::from(self.cursor.x) + 1);
        self.reply(text.as_bytes());
    }

    pub(in super::super) fn report_text_area(&mut self) {
        let text = format!("\x1b[8;{};{}t", self.size.height, self.size.width);
        self.reply(text.as_bytes());
    }

    pub(in super::super) fn report_color(&mut self, slot: u8, background: bool) {
        let color = if background { self.palette.bg } else { self.palette.fg };
        let text = format!("\x1b]{slot};rgb:{0:02x}{0:02x}/{1:02x}{1:02x}/{2:02x}{2:02x}\x1b\\", color[0], color[1], color[2]);
        self.reply(text.as_bytes());
    }

}
