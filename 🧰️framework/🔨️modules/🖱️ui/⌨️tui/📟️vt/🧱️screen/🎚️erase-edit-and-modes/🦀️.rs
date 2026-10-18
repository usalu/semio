//! 🎚️ Erase, edit and modes: the scroll region, display, line and cell erase, line and cell insert and delete, the alignment test and the ansi and private modes. See [`super`].

use super::*;

impl VtScreen {
    pub(in super::super) fn set_scroll_region(&mut self, top: i32, bottom: i32) {
        let top = (top.clamp(1, 65_535) as u16) - 1;
        let bottom = ((bottom.clamp(1, 65_535) as u16) - 1).min(self.size.height - 1);
        if top < bottom {
            self.scroll_top = top;
            self.scroll_bottom = bottom;
        } else {
            self.scroll_top = 0;
            self.scroll_bottom = self.size.height - 1;
        }
        self.cup(1, 1);
    }

    pub(in super::super) fn erase_display(&mut self, mode: i32) {
        let (x, y, width, height) = (self.cursor.x, self.cursor.y, self.size.width, self.size.height);
        match mode {
            0 => {
                self.erase_span(y, x, width);
                self.clear_wrapped(y);
                for row in y + 1..height {
                    self.erase_span(row, 0, width);
                    self.clear_wrapped(row);
                }
            }
            1 => {
                for row in 0..y {
                    self.erase_span(row, 0, width);
                    self.clear_wrapped(row);
                }
                self.erase_span(y, 0, x + 1);
            }
            3 => {
                self.history.clear();
                self.anchor = None;
            }
            _ => {
                for row in 0..height {
                    self.erase_span(row, 0, width);
                    self.clear_wrapped(row);
                }
            }
        }
    }

    pub(in super::super) fn erase_line(&mut self, mode: i32) {
        let (x, y, width) = (self.cursor.x, self.cursor.y, self.size.width);
        match mode {
            0 => {
                self.erase_span(y, x, width);
                self.clear_wrapped(y);
            }
            1 => self.erase_span(y, 0, x + 1),
            _ => {
                self.erase_span(y, 0, width);
                self.clear_wrapped(y);
            }
        }
    }

    pub(in super::super) fn erase_cells(&mut self, n: i32) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let end = i32::from(x).saturating_add(n.max(1)).min(i32::from(self.size.width)) as u16;
        self.erase_span(y, x, end);
    }

    pub(in super::super) fn insert_lines(&mut self, n: i32) {
        let y = usize::from(self.cursor.y);
        if self.cursor.y < self.scroll_top || self.cursor.y > self.scroll_bottom {
            return;
        }
        let bottom = usize::from(self.scroll_bottom);
        for _ in 0..n.max(1).min((bottom - y + 1) as i32) {
            let blank = self.erased_row();
            let rows = &mut self.grid_mut().rows;
            rows.remove(bottom);
            rows.insert(y, blank);
        }
        self.carriage_return();
    }

    pub(in super::super) fn delete_lines(&mut self, n: i32) {
        let y = usize::from(self.cursor.y);
        if self.cursor.y < self.scroll_top || self.cursor.y > self.scroll_bottom {
            return;
        }
        let bottom = usize::from(self.scroll_bottom);
        for _ in 0..n.max(1).min((bottom - y + 1) as i32) {
            let blank = self.erased_row();
            let rows = &mut self.grid_mut().rows;
            rows.remove(y);
            rows.insert(bottom, blank);
        }
        self.carriage_return();
    }

    pub(in super::super) fn insert_cells(&mut self, n: i32) {
        self.wrap_pending = false;
        let (x, y, width) = (self.cursor.x, self.cursor.y, self.size.width);
        let n = n.max(1).min(i32::from(width - x)) as usize;
        let blank = self.pen.erased();
        let grid = self.grid_mut();
        if let Some(row) = grid.rows.get_mut(usize::from(y)) {
            let at = usize::from(x);
            row.cells.splice(at..at, std::iter::repeat_n(blank, n));
            row.cells.truncate(usize::from(width));
        }
        grid.fix_boundaries(y, x);
    }

    pub(in super::super) fn delete_cells(&mut self, n: i32) {
        self.wrap_pending = false;
        let (x, y, width) = (self.cursor.x, self.cursor.y, self.size.width);
        let n = n.max(1).min(i32::from(width - x)) as usize;
        let blank = self.pen.erased();
        let grid = self.grid_mut();
        if let Some(row) = grid.rows.get_mut(usize::from(y)) {
            let at = usize::from(x);
            row.cells.drain(at..at + n);
            row.cells.extend(std::iter::repeat_n(blank, n));
        }
        grid.fix_boundaries(y, x);
    }

    pub(in super::super) fn alignment_test(&mut self) {
        let cell = self.pen.glyph('E', 1);
        let (width, height) = (self.size.width, self.size.height);
        for y in 0..height {
            for x in 0..width {
                self.grid_mut().put(x, y, cell);
            }
        }
        self.cup(1, 1);
    }

    pub(in super::super) fn set_ansi_mode(&mut self, mode: i32, on: bool) {
        match mode {
            4 => self.modes.insert = on,
            20 => self.input.newline = on,
            _ => {}
        }
    }

    pub(in super::super) fn ansi_mode_state(&self, mode: i32) -> u8 {
        match mode {
            4 => 1 + u8::from(!self.modes.insert),
            20 => 1 + u8::from(!self.input.newline),
            _ => 0,
        }
    }

    pub(in super::super) fn set_private_mode(&mut self, mode: i32, on: bool) {
        let reporting = |screen: &mut Self, level: MouseReporting| {
            if on {
                screen.input.mouse = level;
            } else if screen.input.mouse == level {
                screen.input.mouse = MouseReporting::Off;
            }
        };
        let encoding = |screen: &mut Self, kind: MouseEncoding| {
            if on {
                screen.input.mouse_encoding = kind;
            } else if screen.input.mouse_encoding == kind {
                screen.input.mouse_encoding = MouseEncoding::X10;
            }
        };
        match mode {
            1 => self.input.app_cursor = on,
            5 => self.modes.reverse_video = on,
            6 => {
                self.modes.origin = on;
                self.cup(1, 1);
            }
            7 => self.modes.wrap = on,
            9 => reporting(self, MouseReporting::Click),
            12 => self.modes.cursor_blink = on,
            25 => self.modes.cursor_visible = on,
            47 | 1047 => {
                if on {
                    self.enter_alternate(false);
                } else {
                    self.leave_alternate(false, mode == 1047);
                }
            }
            66 => self.input.app_keypad = on,
            1000 => reporting(self, MouseReporting::Press),
            1002 => reporting(self, MouseReporting::Drag),
            1003 => reporting(self, MouseReporting::Motion),
            1004 => self.input.focus_reporting = on,
            1005 => encoding(self, MouseEncoding::Utf8),
            1006 => encoding(self, MouseEncoding::Sgr),
            1007 => self.input.alt_scroll = on,
            1015 => encoding(self, MouseEncoding::Urxvt),
            1048 => {
                if on {
                    self.save_cursor();
                } else {
                    self.restore_cursor();
                }
            }
            1049 => {
                if on {
                    self.enter_alternate(true);
                } else {
                    self.leave_alternate(true, false);
                }
            }
            2004 => self.input.bracketed_paste = on,
            2026 => self.modes.synchronized = on,
            _ => {}
        }
    }

    pub(in super::super) fn private_mode_state(&self, mode: i32) -> u8 {
        let flag = |on: bool| 1 + u8::from(!on);
        match mode {
            1 => flag(self.input.app_cursor),
            5 => flag(self.modes.reverse_video),
            6 => flag(self.modes.origin),
            7 => flag(self.modes.wrap),
            9 => flag(self.input.mouse == MouseReporting::Click),
            12 => flag(self.modes.cursor_blink),
            25 => flag(self.modes.cursor_visible),
            47 | 1047 | 1049 => flag(self.alt_active),
            66 => flag(self.input.app_keypad),
            1000 => flag(self.input.mouse == MouseReporting::Press),
            1002 => flag(self.input.mouse == MouseReporting::Drag),
            1003 => flag(self.input.mouse == MouseReporting::Motion),
            1004 => flag(self.input.focus_reporting),
            1005 => flag(self.input.mouse_encoding == MouseEncoding::Utf8),
            1006 => flag(self.input.mouse_encoding == MouseEncoding::Sgr),
            1007 => flag(self.input.alt_scroll),
            1015 => flag(self.input.mouse_encoding == MouseEncoding::Urxvt),
            2004 => flag(self.input.bracketed_paste),
            2026 => flag(self.modes.synchronized),
            3 | 8 | 40 | 45 | 1001 | 1016 | 2027 => 4,
            _ => 0,
        }
    }

}
