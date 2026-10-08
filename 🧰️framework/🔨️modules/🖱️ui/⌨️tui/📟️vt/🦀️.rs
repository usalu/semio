use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::char_cells;
use crate::tui::theme::Rgb;
use std::collections::VecDeque;

const DEFAULT_FG: Rgb = [192, 192, 192];
const DEFAULT_BG: Rgb = [0, 0, 0];
const DEFAULT_SCROLLBACK: usize = 10_000;

//#region ???Palette
fn ansi_16(n: u8) -> Rgb {
    match n {
        0 => [0, 0, 0],
        1 => [205, 0, 0],
        2 => [0, 205, 0],
        3 => [205, 205, 0],
        4 => [0, 0, 238],
        5 => [205, 0, 205],
        6 => [0, 205, 205],
        7 => [229, 229, 229],
        8 => [127, 127, 127],
        9 => [255, 0, 0],
        10 => [0, 255, 0],
        11 => [255, 255, 0],
        12 => [92, 92, 255],
        13 => [255, 0, 255],
        14 => [0, 255, 255],
        _ => [255, 255, 255],
    }
}

/// ??? Maps a 256-color index onto an approximate truecolor RGB.
pub fn color_256(n: u8) -> Rgb {
    if n < 16 {
        return ansi_16(n);
    }
    if n < 232 {
        let i = n - 16;
        let r = i / 36;
        let g = (i % 36) / 6;
        let b = i % 6;
        let level = |c: u8| if c == 0 { 0 } else { 55 + 40 * c };
        [level(r), level(g), level(b)]
    } else {
        let v = 8 + 10 * (n - 232);
        [v, v, v]
    }
}
//#endregion ???Palette

//#region ???Parser
#[derive(Clone, Copy, PartialEq, Eq)]
enum ParserState {
    Ground,
    Escape,
    Csi,
    Osc,
    Dcs,
    SosPmApc,
}

/// ?? Incremental VT output decoder (CSI/SGR/OSC/DCS) driving a `VtScreen`.
#[derive(Clone)]
pub struct VtParser {
    state: ParserState,
    params: Vec<i32>,
    current: i32,
    has_current: bool,
    intermediate: u8,
    private: bool,
    osc: Vec<u8>,
    utf8_buf: [u8; 4],
    utf8_len: u8,
    utf8_need: u8,
    ignore_esc: bool,
}

impl Default for VtParser {
    fn default() -> Self {
        Self::new()
    }
}

impl VtParser {
    pub fn new() -> Self {
        Self { state: ParserState::Ground, params: Vec::new(), current: 0, has_current: false, intermediate: 0, private: false, osc: Vec::new(), utf8_buf: [0; 4], utf8_len: 0, utf8_need: 0, ignore_esc: false }
    }

    fn reset_seq(&mut self) {
        self.params.clear();
        self.current = 0;
        self.has_current = false;
        self.intermediate = 0;
        self.private = false;
    }

    fn push_param(&mut self) {
        self.params.push(if self.has_current { self.current } else { 0 });
        self.current = 0;
        self.has_current = false;
    }

    fn param(&self, i: usize, default: i32) -> i32 {
        match self.params.get(i).copied() {
            Some(0) | None => default,
            Some(v) => v,
        }
    }

    /// ??? Feeds raw PTY bytes into `screen`.
    pub fn feed(&mut self, bytes: &[u8], screen: &mut VtScreen) {
        for &b in bytes {
            self.feed_byte(b, screen);
        }
    }

    fn feed_byte(&mut self, b: u8, screen: &mut VtScreen) {
        if self.utf8_need > 0 && self.state == ParserState::Ground {
            self.utf8_buf[self.utf8_len as usize] = b;
            self.utf8_len += 1;
            self.utf8_need -= 1;
            if self.utf8_need == 0 {
                if let Ok(s) = std::str::from_utf8(&self.utf8_buf[..self.utf8_len as usize]) {
                    if let Some(c) = s.chars().next() {
                        screen.put_char(c);
                    }
                }
                self.utf8_len = 0;
            }
            return;
        }
        match self.state {
            ParserState::Ground => self.feed_ground(b, screen),
            ParserState::Escape => self.feed_escape(b, screen),
            ParserState::Csi => self.feed_csi(b, screen),
            ParserState::Osc => self.feed_osc(b, screen),
            ParserState::Dcs | ParserState::SosPmApc => {
                if b == 0x1b {
                    self.ignore_esc = true;
                    self.state = ParserState::Escape;
                } else if b == 0x07 {
                    self.state = ParserState::Ground;
                }
            }
        }
    }

    fn feed_ground(&mut self, b: u8, screen: &mut VtScreen) {
        match b {
            0x1b => {
                self.reset_seq();
                self.state = ParserState::Escape;
            }
            0x07 => {}
            0x08 => screen.backspace(),
            0x09 => screen.tab(),
            0x0a => screen.linefeed(),
            0x0d => screen.carriage_return(),
            0x00..=0x1f | 0x7f => {}
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
            _ => screen.put_char(b as char),
        }
    }

    fn feed_escape(&mut self, b: u8, screen: &mut VtScreen) {
        if self.ignore_esc {
            self.ignore_esc = false;
            if b == b'\\' {
                self.state = ParserState::Ground;
                return;
            }
            self.state = ParserState::Ground;
            self.feed_ground(b, screen);
            return;
        }
        match b {
            b'[' => {
                self.reset_seq();
                self.state = ParserState::Csi;
            }
            b']' => {
                self.osc.clear();
                self.state = ParserState::Osc;
            }
            b'P' => self.state = ParserState::Dcs,
            b'X' | b'^' | b'_' => self.state = ParserState::SosPmApc,
            b'7' => {
                screen.save_cursor();
                self.state = ParserState::Ground;
            }
            b'8' => {
                screen.restore_cursor();
                self.state = ParserState::Ground;
            }
            b'c' => {
                screen.reset();
                self.state = ParserState::Ground;
            }
            _ => self.state = ParserState::Ground,
        }
    }

    fn feed_csi(&mut self, b: u8, screen: &mut VtScreen) {
        match b {
            b'0'..=b'9' => {
                self.current = self.current.saturating_mul(10).saturating_add(i32::from(b - b'0'));
                self.has_current = true;
            }
            b';' => self.push_param(),
            b'?' if self.params.is_empty() && !self.has_current && self.intermediate == 0 => self.private = true,
            0x20..=0x2f => self.intermediate = b,
            0x40..=0x7e => {
                self.push_param();
                self.state = ParserState::Ground;
                self.finish_csi(b, screen);
            }
            _ => self.state = ParserState::Ground,
        }
    }

    fn finish_csi(&mut self, final_byte: u8, screen: &mut VtScreen) {
        if self.private {
            match final_byte {
                b'h' => {
                    for p in self.params.clone() {
                        screen.decset(p, true);
                    }
                }
                b'l' => {
                    for p in self.params.clone() {
                        screen.decset(p, false);
                    }
                }
                _ => {}
            }
            return;
        }
        match final_byte {
            b'A' => screen.move_cursor(0, -self.param(0, 1)),
            b'B' => screen.move_cursor(0, self.param(0, 1)),
            b'C' => screen.move_cursor(self.param(0, 1), 0),
            b'D' => screen.move_cursor(-self.param(0, 1), 0),
            b'H' | b'f' => {
                let row = self.param(0, 1);
                let col = self.param(1, 1);
                screen.cup(row, col);
            }
            b'J' => screen.erase_display(self.param(0, 0)),
            b'K' => screen.erase_line(self.param(0, 0)),
            b'L' => screen.insert_lines(self.param(0, 1) as u16),
            b'M' => screen.delete_lines(self.param(0, 1) as u16),
            b'@' => screen.insert_cells(self.param(0, 1) as u16),
            b'P' => screen.delete_cells(self.param(0, 1) as u16),
            b'X' => screen.erase_cells(self.param(0, 1) as u16),
            b'S' => screen.scroll_up(self.param(0, 1) as u16),
            b'T' => screen.scroll_down(self.param(0, 1) as u16),
            b'r' => {
                let top = self.param(0, 1);
                let bottom = self.param(1, i32::from(screen.size.height));
                screen.set_scroll_region(top, bottom);
            }
            b'm' => screen.apply_sgr(&self.params),
            _ => {}
        }
    }

    fn feed_osc(&mut self, b: u8, screen: &mut VtScreen) {
        match b {
            0x07 => {
                self.finish_osc(screen);
                self.state = ParserState::Ground;
            }
            0x1b => {
                self.ignore_esc = true;
                self.state = ParserState::Escape;
                self.finish_osc(screen);
            }
            _ => {
                if self.osc.len() < 4096 {
                    self.osc.push(b);
                }
            }
        }
    }

    fn finish_osc(&mut self, screen: &mut VtScreen) {
        let text = String::from_utf8_lossy(&self.osc);
        let mut parts = text.splitn(2, ';');
        let code = parts.next().unwrap_or("");
        let payload = parts.next().unwrap_or("");
        if code == "0" || code == "2" {
            screen.title = Some(payload.to_string());
        }
        self.osc.clear();
    }
}
//#endregion ???Parser

//#region ???Screen
#[derive(Clone, Copy)]
struct SavedCursor {
    pos: Pos,
    fg: Rgb,
    bg: Rgb,
    attrs: u8,
    origin: bool,
}

/// ??? VT screen: primary/alt buffers, scrollback, cursor, SGR, scroll region, modes.
pub struct VtScreen {
    pub size: Size,
    primary: CellBuffer,
    alt: CellBuffer,
    pub alt_active: bool,
    scrollback: VecDeque<Vec<Cell>>,
    scrollback_cap: usize,
    pub cursor: Pos,
    saved: SavedCursor,
    fg: Rgb,
    bg: Rgb,
    attrs: u8,
    pub scroll_top: u16,
    pub scroll_bottom: u16,
    pub origin_mode: bool,
    pub wrap_mode: bool,
    pub cursor_visible: bool,
    pub mouse_tracking: bool,
    pub mouse_button_event: bool,
    pub mouse_sgr: bool,
    pub bracketed_paste: bool,
    wrap_pending: bool,
    pub title: Option<String>,
    parser: VtParser,
}

impl VtScreen {
    /// ?? Creates a blank VT screen of `size` with `scrollback_cap` (0 ? default 10000).
    pub fn new(size: Size, scrollback_cap: usize) -> Self {
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        let height = size.height.max(1);
        let width = size.width.max(1);
        let size = Size { width, height };
        Self {
            size,
            primary: CellBuffer::new(size, blank),
            alt: CellBuffer::new(size, blank),
            alt_active: false,
            scrollback: VecDeque::new(),
            scrollback_cap: if scrollback_cap == 0 { DEFAULT_SCROLLBACK } else { scrollback_cap },
            cursor: Pos { x: 0, y: 0 },
            saved: SavedCursor { pos: Pos { x: 0, y: 0 }, fg: DEFAULT_FG, bg: DEFAULT_BG, attrs: 0, origin: false },
            fg: DEFAULT_FG,
            bg: DEFAULT_BG,
            attrs: 0,
            scroll_top: 0,
            scroll_bottom: height - 1,
            origin_mode: false,
            wrap_mode: true,
            cursor_visible: true,
            mouse_tracking: false,
            mouse_button_event: false,
            mouse_sgr: false,
            bracketed_paste: false,
            wrap_pending: false,
            title: None,
            parser: VtParser::new(),
        }
    }

    fn active_buf(&self) -> &CellBuffer {
        if self.alt_active {
            &self.alt
        } else {
            &self.primary
        }
    }

    fn active_buf_mut(&mut self) -> &mut CellBuffer {
        if self.alt_active {
            &mut self.alt
        } else {
            &mut self.primary
        }
    }

    fn clamp_cursor(&mut self) {
        let max_x = self.size.width.saturating_sub(1);
        let (min_y, max_y) = if self.origin_mode { (self.scroll_top, self.scroll_bottom) } else { (0, self.size.height.saturating_sub(1)) };
        self.cursor.x = self.cursor.x.min(max_x);
        self.cursor.y = self.cursor.y.clamp(min_y, max_y);
        self.wrap_pending = false;
    }

    /// ?? Resizes both buffers; clamps cursor into the new grid.
    pub fn resize(&mut self, size: Size) {
        let width = size.width.max(1);
        let height = size.height.max(1);
        let size = Size { width, height };
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        let mut next_primary = CellBuffer::new(size, blank);
        let mut next_alt = CellBuffer::new(size, blank);
        let copy_h = self.size.height.min(height);
        let copy_w = self.size.width.min(width);
        for y in 0..copy_h {
            for x in 0..copy_w {
                if let Some(c) = self.primary.get(x, y) {
                    next_primary.put(x, y, *c);
                }
                if let Some(c) = self.alt.get(x, y) {
                    next_alt.put(x, y, *c);
                }
            }
        }
        self.primary = next_primary;
        self.alt = next_alt;
        self.size = size;
        if self.scroll_bottom >= height || self.scroll_top >= height {
            self.scroll_top = 0;
            self.scroll_bottom = height - 1;
        } else {
            self.scroll_bottom = self.scroll_bottom.min(height - 1);
        }
        self.clamp_cursor();
    }

    /// ??? Feeds raw bytes through the owned incremental parser.
    pub fn feed(&mut self, bytes: &[u8]) {
        let mut parser = std::mem::replace(&mut self.parser, VtParser::new());
        parser.feed(bytes, self);
        self.parser = parser;
    }

    /// ?? Visible viewport row count.
    pub fn visible_line_count(&self) -> u16 {
        self.size.height
    }

    /// ?? Lines currently held in scrollback.
    pub fn scrollback_len(&self) -> usize {
        self.scrollback.len()
    }

    /// ?? Reads one cell from the active buffer.
    pub fn cell_at(&self, x: u16, y: u16) -> Option<&Cell> {
        self.active_buf().get(x, y)
    }

    /// ??? Composites the visible viewport (optionally offset into scrollback) into `dest`.
    pub fn blit_to(&self, dest: &mut CellBuffer, dest_rect: Rect, scrollback_offset: usize) {
        let clip = Rect::new(0, 0, dest.size.width, dest.size.height).intersect(dest_rect);
        if clip.width == 0 || clip.height == 0 {
            return;
        }
        let sb = self.scrollback.len();
        let offset = scrollback_offset.min(sb);
        let buf = self.active_buf();
        for row in 0..clip.height {
            let abs = (sb as isize - offset as isize) + row as isize;
            if abs < 0 {
                continue;
            }
            let abs_u = abs as usize;
            let cells = if abs_u < sb {
                Some(self.scrollback[abs_u].clone())
            } else {
                let vy = (abs_u - sb) as u16;
                if vy < self.size.height {
                    Some((0..self.size.width).map(|x| buf.get(x, vy).copied().unwrap_or_else(|| Cell::blank(DEFAULT_FG, DEFAULT_BG))).collect::<Vec<_>>())
                } else {
                    None
                }
            };
            let Some(cells) = cells else { continue };
            for col in 0..clip.width {
                if (col as usize) < cells.len() {
                    dest.put(clip.x + col, clip.y + row, cells[col as usize]);
                }
            }
        }
    }

    fn push_scrollback_row(&mut self, y: u16) {
        if self.alt_active || self.scroll_top != 0 {
            return;
        }
        let buf = self.active_buf();
        let row: Vec<Cell> = (0..self.size.width).map(|x| buf.get(x, y).copied().unwrap_or_else(|| Cell::blank(DEFAULT_FG, DEFAULT_BG))).collect();
        if self.scrollback.len() >= self.scrollback_cap {
            self.scrollback.pop_front();
        }
        self.scrollback.push_back(row);
    }

    fn copy_row(&mut self, from: u16, to: u16) {
        if from == to {
            return;
        }
        let width = self.size.width;
        let cells: Vec<Cell> = (0..width).map(|x| self.active_buf().get(x, from).copied().unwrap_or_else(|| Cell::blank(DEFAULT_FG, DEFAULT_BG))).collect();
        for (x, cell) in cells.into_iter().enumerate() {
            self.active_buf_mut().put(x as u16, to, cell);
        }
    }

    fn clear_row(&mut self, y: u16) {
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        let width = self.size.width;
        let buf = self.active_buf_mut();
        for x in 0..width {
            buf.put(x, y, blank);
        }
    }

    fn scroll_up(&mut self, n: u16) {
        let n = n.max(1);
        let top = self.scroll_top;
        let bottom = self.scroll_bottom;
        if top > bottom {
            return;
        }
        for _ in 0..n {
            self.push_scrollback_row(top);
            for y in top..bottom {
                self.copy_row(y + 1, y);
            }
            self.clear_row(bottom);
        }
    }

    fn scroll_down(&mut self, n: u16) {
        let n = n.max(1);
        let top = self.scroll_top;
        let bottom = self.scroll_bottom;
        if top > bottom {
            return;
        }
        for _ in 0..n {
            for y in (top..bottom).rev() {
                self.copy_row(y, y + 1);
            }
            self.clear_row(top);
        }
    }

    fn carriage_return(&mut self) {
        self.cursor.x = 0;
        self.wrap_pending = false;
    }

    fn linefeed(&mut self) {
        self.wrap_pending = false;
        if self.cursor.y == self.scroll_bottom {
            self.scroll_up(1);
        } else if self.cursor.y < self.size.height.saturating_sub(1) {
            self.cursor.y += 1;
        }
    }

    fn backspace(&mut self) {
        self.wrap_pending = false;
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
    }

    fn tab(&mut self) {
        self.wrap_pending = false;
        let next = ((self.cursor.x / 8) + 1) * 8;
        self.cursor.x = next.min(self.size.width.saturating_sub(1));
    }

    fn put_char(&mut self, c: char) {
        let w = char_cells(c);
        if w == 0 {
            return;
        }
        let w = u16::from(w);
        if self.wrap_pending {
            self.carriage_return();
            self.linefeed();
        }
        if self.cursor.x + w > self.size.width {
            if self.wrap_mode {
                self.carriage_return();
                self.linefeed();
            } else {
                self.cursor.x = self.size.width.saturating_sub(w.max(1));
            }
        }
        let cell = Cell { ch: c, fg: self.fg, bg: self.bg, attrs: self.attrs, width: w as u8 };
        let x = self.cursor.x;
        let y = self.cursor.y;
        self.active_buf_mut().put(x, y, cell);
        self.cursor.x = x + w;
        if self.cursor.x >= self.size.width {
            self.cursor.x = self.size.width.saturating_sub(1);
            self.wrap_pending = self.wrap_mode;
        }
    }

    fn move_cursor(&mut self, dx: i32, dy: i32) {
        self.wrap_pending = false;
        let nx = (i32::from(self.cursor.x) + dx).clamp(0, i32::from(self.size.width.saturating_sub(1)));
        let (min_y, max_y) = (i32::from(self.scroll_top), i32::from(self.scroll_bottom));
        let ny = (i32::from(self.cursor.y) + dy).clamp(min_y, max_y);
        self.cursor.x = nx as u16;
        self.cursor.y = ny as u16;
    }

    fn cup(&mut self, row: i32, col: i32) {
        self.wrap_pending = false;
        let row = row.max(1) as u16;
        let col = col.max(1) as u16;
        let (y_base, y_max) = if self.origin_mode { (self.scroll_top, self.scroll_bottom) } else { (0, self.size.height.saturating_sub(1)) };
        let y = y_base.saturating_add(row.saturating_sub(1)).min(y_max);
        let x = col.saturating_sub(1).min(self.size.width.saturating_sub(1));
        self.cursor = Pos { x, y };
    }

    fn erase_display(&mut self, mode: i32) {
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        let size = self.size;
        let cursor = self.cursor;
        let buf = self.active_buf_mut();
        match mode {
            0 => {
                for x in cursor.x..size.width {
                    buf.put(x, cursor.y, blank);
                }
                for y in cursor.y + 1..size.height {
                    for x in 0..size.width {
                        buf.put(x, y, blank);
                    }
                }
            }
            1 => {
                for y in 0..cursor.y {
                    for x in 0..size.width {
                        buf.put(x, y, blank);
                    }
                }
                for x in 0..=cursor.x {
                    buf.put(x, cursor.y, blank);
                }
            }
            _ => {
                for y in 0..size.height {
                    for x in 0..size.width {
                        buf.put(x, y, blank);
                    }
                }
            }
        }
    }

    fn erase_line(&mut self, mode: i32) {
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        let size = self.size;
        let cursor = self.cursor;
        let buf = self.active_buf_mut();
        match mode {
            0 => {
                for x in cursor.x..size.width {
                    buf.put(x, cursor.y, blank);
                }
            }
            1 => {
                for x in 0..=cursor.x {
                    buf.put(x, cursor.y, blank);
                }
            }
            _ => {
                for x in 0..size.width {
                    buf.put(x, cursor.y, blank);
                }
            }
        }
    }

    fn insert_lines(&mut self, n: u16) {
        let n = n.max(1);
        let y = self.cursor.y;
        if y < self.scroll_top || y > self.scroll_bottom {
            return;
        }
        let bottom = self.scroll_bottom;
        for _ in 0..n {
            for row in (y..bottom).rev() {
                self.copy_row(row, row + 1);
            }
            self.clear_row(y);
        }
    }

    fn delete_lines(&mut self, n: u16) {
        let n = n.max(1);
        let y = self.cursor.y;
        if y < self.scroll_top || y > self.scroll_bottom {
            return;
        }
        let bottom = self.scroll_bottom;
        for _ in 0..n {
            for row in y..bottom {
                self.copy_row(row + 1, row);
            }
            self.clear_row(bottom);
        }
    }

    fn insert_cells(&mut self, n: u16) {
        let n = n.max(1).min(self.size.width.saturating_sub(self.cursor.x));
        let y = self.cursor.y;
        let x0 = self.cursor.x;
        let width = self.size.width;
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        for _ in 0..n {
            for x in (x0..width.saturating_sub(1)).rev() {
                let cell = self.active_buf().get(x, y).copied().unwrap_or(blank);
                self.active_buf_mut().put(x + 1, y, cell);
            }
            self.active_buf_mut().put(x0, y, blank);
        }
    }

    fn delete_cells(&mut self, n: u16) {
        let n = n.max(1).min(self.size.width.saturating_sub(self.cursor.x));
        let y = self.cursor.y;
        let x0 = self.cursor.x;
        let width = self.size.width;
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        for _ in 0..n {
            for x in x0..width.saturating_sub(1) {
                let cell = self.active_buf().get(x + 1, y).copied().unwrap_or(blank);
                self.active_buf_mut().put(x, y, cell);
            }
            self.active_buf_mut().put(width.saturating_sub(1), y, blank);
        }
    }

    fn erase_cells(&mut self, n: u16) {
        let blank = Cell::blank(DEFAULT_FG, DEFAULT_BG);
        let y = self.cursor.y;
        let x0 = self.cursor.x;
        let end = (x0 + n.max(1)).min(self.size.width);
        let buf = self.active_buf_mut();
        for x in x0..end {
            buf.put(x, y, blank);
        }
    }

    fn set_scroll_region(&mut self, top: i32, bottom: i32) {
        let top = (top.max(1) as u16).saturating_sub(1);
        let bottom = (bottom.max(1) as u16).saturating_sub(1).min(self.size.height.saturating_sub(1));
        if top < bottom {
            self.scroll_top = top;
            self.scroll_bottom = bottom;
        } else {
            self.scroll_top = 0;
            self.scroll_bottom = self.size.height.saturating_sub(1);
        }
        self.cup(1, 1);
    }

    fn save_cursor(&mut self) {
        self.saved = SavedCursor { pos: self.cursor, fg: self.fg, bg: self.bg, attrs: self.attrs, origin: self.origin_mode };
    }

    fn restore_cursor(&mut self) {
        self.cursor = self.saved.pos;
        self.fg = self.saved.fg;
        self.bg = self.saved.bg;
        self.attrs = self.saved.attrs;
        self.origin_mode = self.saved.origin;
        self.clamp_cursor();
    }

    fn reset(&mut self) {
        let size = self.size;
        let cap = self.scrollback_cap;
        *self = Self::new(size, cap);
    }

    fn decset(&mut self, mode: i32, enable: bool) {
        match mode {
            1 => {}
            6 => {
                self.origin_mode = enable;
                self.cup(1, 1);
            }
            7 => self.wrap_mode = enable,
            25 => self.cursor_visible = enable,
            1000 => self.mouse_tracking = enable,
            1002 => self.mouse_button_event = enable,
            1006 => self.mouse_sgr = enable,
            1049 => {
                if enable {
                    self.save_cursor();
                    self.alt_active = true;
                    self.erase_display(2);
                    self.cursor = Pos { x: 0, y: 0 };
                    self.wrap_pending = false;
                } else {
                    self.alt_active = false;
                    self.restore_cursor();
                }
            }
            2004 => self.bracketed_paste = enable,
            _ => {}
        }
    }

    fn apply_sgr(&mut self, params: &[i32]) {
        if params.is_empty() || (params.len() == 1 && params[0] == 0) {
            self.fg = DEFAULT_FG;
            self.bg = DEFAULT_BG;
            self.attrs = 0;
            return;
        }
        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => {
                    self.fg = DEFAULT_FG;
                    self.bg = DEFAULT_BG;
                    self.attrs = 0;
                }
                1 => self.attrs |= attr::BOLD,
                2 => self.attrs |= attr::DIM,
                3 => self.attrs |= attr::ITALIC,
                4 => self.attrs |= attr::UNDERLINE,
                7 => self.attrs |= attr::REVERSE,
                22 => self.attrs &= !(attr::BOLD | attr::DIM),
                23 => self.attrs &= !attr::ITALIC,
                24 => self.attrs &= !attr::UNDERLINE,
                27 => self.attrs &= !attr::REVERSE,
                30..=37 => self.fg = ansi_16((params[i] - 30) as u8),
                39 => self.fg = DEFAULT_FG,
                40..=47 => self.bg = ansi_16((params[i] - 40) as u8),
                49 => self.bg = DEFAULT_BG,
                90..=97 => self.fg = ansi_16((params[i] - 90 + 8) as u8),
                100..=107 => self.bg = ansi_16((params[i] - 100 + 8) as u8),
                38 => {
                    if i + 1 < params.len() {
                        match params[i + 1] {
                            5 if i + 2 < params.len() => {
                                self.fg = color_256(params[i + 2].clamp(0, 255) as u8);
                                i += 2;
                            }
                            2 if i + 4 < params.len() => {
                                self.fg = [params[i + 2].clamp(0, 255) as u8, params[i + 3].clamp(0, 255) as u8, params[i + 4].clamp(0, 255) as u8];
                                i += 4;
                            }
                            _ => {}
                        }
                    }
                }
                48 => {
                    if i + 1 < params.len() {
                        match params[i + 1] {
                            5 if i + 2 < params.len() => {
                                self.bg = color_256(params[i + 2].clamp(0, 255) as u8);
                                i += 2;
                            }
                            2 if i + 4 < params.len() => {
                                self.bg = [params[i + 2].clamp(0, 255) as u8, params[i + 3].clamp(0, 255) as u8, params[i + 4].clamp(0, 255) as u8];
                                i += 4;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}
//#endregion ???Screen
