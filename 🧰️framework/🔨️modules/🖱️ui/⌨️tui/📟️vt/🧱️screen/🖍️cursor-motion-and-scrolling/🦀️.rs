//! 🖍️ Cursor motion and scrolling: cursor clamping, erase spans, scroll up and down, line feeds, tab stops, charsets, printing with wrap, cursor moves and cursor save and restore. See [`super`].

use super::*;

impl VtScreen {
    fn clamp_cursor(&mut self) {
        let (min_y, max_y) = if self.modes.origin { (self.scroll_top, self.scroll_bottom) } else { (0, self.size.height - 1) };
        self.cursor.x = self.cursor.x.min(self.size.width - 1);
        self.cursor.y = self.cursor.y.clamp(min_y, max_y);
        self.wrap_pending = false;
    }

    pub(super) fn erased_row(&self) -> Row {
        Row::blank(self.size.width, self.pen.erased())
    }

    pub(super) fn erase_span(&mut self, y: u16, from: u16, to: u16) {
        let blank = self.pen.erased();
        let width = self.size.width;
        let (from, to) = (from.min(width), to.min(width));
        if from >= to {
            return;
        }
        let grid = self.grid_mut();
        if let Some(row) = grid.rows.get_mut(usize::from(y)) {
            for cell in &mut row.cells[usize::from(from)..usize::from(to)] {
                *cell = blank;
            }
        }
        grid.fix_boundaries(y, to);
        if from > 0 {
            if let Some(row) = grid.rows.get_mut(usize::from(y)) {
                if row.cells[usize::from(from - 1)].width == 2 {
                    row.cells[usize::from(from - 1)] = Cell { ch: ' ', width: 1, ..row.cells[usize::from(from - 1)] };
                }
            }
        }
    }

    pub(super) fn clear_wrapped(&mut self, y: u16) {
        if let Some(row) = self.grid_mut().rows.get_mut(usize::from(y)) {
            row.wrapped = false;
        }
    }

    pub(in super::super) fn scroll_up(&mut self, n: u16) {
        let (top, bottom) = (usize::from(self.scroll_top), usize::from(self.scroll_bottom));
        if top > bottom {
            return;
        }
        let n = usize::from(n.max(1)).min(bottom - top + 1);
        let keep = !self.alt_active && top == 0;
        for _ in 0..n {
            let blank = self.erased_row();
            let rows = &mut self.grid_mut().rows;
            let removed = rows.remove(top);
            rows.insert(bottom, blank);
            if keep {
                self.history.push(removed);
            }
        }
    }

    pub(in super::super) fn scroll_down(&mut self, n: u16) {
        let (top, bottom) = (usize::from(self.scroll_top), usize::from(self.scroll_bottom));
        if top > bottom {
            return;
        }
        let n = usize::from(n.max(1)).min(bottom - top + 1);
        for _ in 0..n {
            let blank = self.erased_row();
            let rows = &mut self.grid_mut().rows;
            rows.remove(bottom);
            rows.insert(top, blank);
        }
    }

    pub(in super::super) fn carriage_return(&mut self) {
        self.cursor.x = 0;
        self.wrap_pending = false;
    }

    pub(in super::super) fn linefeed(&mut self) {
        self.wrap_pending = false;
        if self.cursor.y == self.scroll_bottom {
            self.scroll_up(1);
        } else if self.cursor.y + 1 < self.size.height {
            self.cursor.y += 1;
        }
        if self.input.newline {
            self.cursor.x = 0;
        }
    }

    pub(in super::super) fn index(&mut self) {
        self.linefeed();
    }

    pub(in super::super) fn next_line(&mut self) {
        self.carriage_return();
        self.linefeed();
    }

    pub(in super::super) fn reverse_index(&mut self) {
        self.wrap_pending = false;
        if self.cursor.y == self.scroll_top {
            self.scroll_down(1);
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
        }
    }

    pub(in super::super) fn backspace(&mut self) {
        self.wrap_pending = false;
        self.cursor.x = self.cursor.x.saturating_sub(1);
    }

    pub(in super::super) fn bell(&mut self) {
        self.bells = self.bells.wrapping_add(1);
    }

    pub(in super::super) fn tab_forward(&mut self, n: i32) {
        self.wrap_pending = false;
        for _ in 0..n.clamp(1, i32::from(self.size.width)) {
            let next = (self.cursor.x + 1..self.size.width).find(|x| self.tab_stops[usize::from(*x)]);
            self.cursor.x = next.unwrap_or(self.size.width - 1);
        }
    }

    pub(in super::super) fn tab_back(&mut self, n: i32) {
        self.wrap_pending = false;
        for _ in 0..n.clamp(1, i32::from(self.size.width)) {
            let previous = (0..self.cursor.x).rev().find(|x| self.tab_stops[usize::from(*x)]);
            self.cursor.x = previous.unwrap_or(0);
        }
    }

    pub(in super::super) fn set_tab_stop(&mut self) {
        let x = usize::from(self.cursor.x);
        self.tab_stops[x] = true;
    }

    pub(in super::super) fn clear_tab_stops(&mut self, all: bool) {
        if all {
            self.tab_stops.iter_mut().for_each(|stop| *stop = false);
        } else {
            let x = usize::from(self.cursor.x);
            self.tab_stops[x] = false;
        }
    }

    pub(in super::super) fn shift_charset(&mut self, shifted: bool) {
        self.shifted = shifted;
    }

    pub(in super::super) fn designate_charset(&mut self, slot: usize, charset: Charset) {
        self.charsets[slot.min(1)] = charset;
    }

    fn map_charset(&self, c: char) -> char {
        let active = self.charsets[usize::from(self.shifted)];
        if active == Charset::Graphics && ('\u{60}'..='\u{7e}').contains(&c) {
            const GRAPHICS: [char; 31] = ['◆', '▒', '␉', '␌', '␍', '␊', '°', '±', '␤', '␋', '┘', '┐', '┌', '└', '┼', '⎺', '⎻', '─', '⎼', '⎽', '├', '┤', '┴', '┬', '│', '≤', '≥', 'π', '≠', '£', '·'];
            GRAPHICS[(c as usize) - 0x60]
        } else {
            c
        }
    }

    pub(in super::super) fn print(&mut self, c: char) {
        let c = self.map_charset(c);
        let w = u16::from(char_cells(c)).min(self.size.width);
        if w == 0 {
            return;
        }
        self.last_char = Some(c);
        if self.wrap_pending {
            self.wrap_line();
        }
        if self.cursor.x + w > self.size.width {
            if self.modes.wrap {
                self.wrap_line();
            } else {
                self.cursor.x = self.size.width - w;
            }
        }
        if self.modes.insert {
            self.insert_cells(w as i32);
        }
        let (x, y) = (self.cursor.x, self.cursor.y);
        let cell = self.pen.glyph(c, w as u8);
        self.grid_mut().put(x, y, cell);
        self.cursor.x = x + w;
        if self.cursor.x >= self.size.width {
            self.cursor.x = self.size.width - 1;
            self.wrap_pending = self.modes.wrap;
        }
    }

    fn wrap_line(&mut self) {
        let y = self.cursor.y;
        if let Some(row) = self.grid_mut().rows.get_mut(usize::from(y)) {
            row.wrapped = true;
        }
        self.carriage_return();
        self.linefeed();
    }

    pub(in super::super) fn repeat_last(&mut self, n: i32) {
        if let Some(c) = self.last_char {
            for _ in 0..n.clamp(1, 65_535) {
                self.print(c);
            }
        }
    }

    pub(in super::super) fn cursor_up(&mut self, n: i32) {
        self.wrap_pending = false;
        let limit = if self.cursor.y >= self.scroll_top { self.scroll_top } else { 0 };
        self.cursor.y = (i32::from(self.cursor.y) - n.max(1)).max(i32::from(limit)) as u16;
    }

    pub(in super::super) fn cursor_down(&mut self, n: i32) {
        self.wrap_pending = false;
        let limit = if self.cursor.y <= self.scroll_bottom { self.scroll_bottom } else { self.size.height - 1 };
        self.cursor.y = i32::from(self.cursor.y).saturating_add(n.max(1)).min(i32::from(limit)) as u16;
    }

    pub(in super::super) fn cursor_forward(&mut self, n: i32) {
        self.wrap_pending = false;
        self.cursor.x = i32::from(self.cursor.x).saturating_add(n.max(1)).min(i32::from(self.size.width - 1)) as u16;
    }

    pub(in super::super) fn cursor_back(&mut self, n: i32) {
        self.wrap_pending = false;
        self.cursor.x = (i32::from(self.cursor.x) - n.max(1)).max(0) as u16;
    }

    pub(in super::super) fn cursor_column(&mut self, col: i32) {
        self.wrap_pending = false;
        self.cursor.x = (col.max(1) - 1).min(i32::from(self.size.width - 1)) as u16;
    }

    pub(in super::super) fn cursor_row(&mut self, row: i32) {
        self.wrap_pending = false;
        let (base, max) = if self.modes.origin { (self.scroll_top, self.scroll_bottom) } else { (0, self.size.height - 1) };
        self.cursor.y = i32::from(base).saturating_add(row.max(1) - 1).min(i32::from(max)) as u16;
    }

    pub(in super::super) fn cup(&mut self, row: i32, col: i32) {
        self.cursor_row(row);
        self.cursor_column(col);
    }

    pub(in super::super) fn save_cursor(&mut self) {
        self.saved = SavedCursor { pos: self.cursor, pen: self.pen, origin: self.modes.origin, wrap_pending: self.wrap_pending, charsets: self.charsets, shifted: self.shifted };
    }

    pub(in super::super) fn restore_cursor(&mut self) {
        let saved = self.saved;
        self.cursor = saved.pos;
        self.pen = saved.pen;
        self.modes.origin = saved.origin;
        self.charsets = saved.charsets;
        self.shifted = saved.shifted;
        self.clamp_cursor();
        self.wrap_pending = saved.wrap_pending && self.cursor.x == self.size.width - 1;
    }

}
