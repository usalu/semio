use super::encode::{InputModes, MouseEncoding, MouseReporting};
use super::history::{reflow, History, Row, Spot};
use super::palette::{flag, Palette, XTERM_BG, XTERM_FG};
use super::parser::VtParser;
use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::char_cells;
use crate::tui::theme::Rgb;
use crate::tui::widget::CursorShape;

const DEFAULT_SCROLLBACK: usize = 10_000;
const MAX_MATCHES: usize = 5_000;

/// 📌️ A cell in absolute row numbering, stable while output scrolls the screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellPoint {
    pub row: u64,
    pub col: u16,
}

/// 🎣️ One search hit: where it starts and how many cells it spans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Match {
    pub row: u64,
    pub col: u16,
    pub cells: u16,
}

/// 🎛️ Display modes the child switched on or off.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modes {
    pub origin: bool,
    pub wrap: bool,
    pub insert: bool,
    pub cursor_visible: bool,
    pub cursor_blink: bool,
    pub cursor_shape: CursorShape,
    pub synchronized: bool,
    pub reverse_video: bool,
}

impl Default for Modes {
    fn default() -> Self {
        Self { origin: false, wrap: true, insert: false, cursor_visible: true, cursor_blink: true, cursor_shape: CursorShape::Block, synchronized: false, reverse_video: false }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Charset {
    Ascii,
    Graphics,
}

#[derive(Clone, Copy)]
struct Pen {
    fg: Rgb,
    bg: Rgb,
    attrs: u8,
}

impl Pen {
    const DEFAULT: Pen = Pen { fg: XTERM_FG, bg: XTERM_BG, attrs: flag::MASK };

    fn glyph(&self, ch: char, width: u8) -> Cell {
        Cell { ch, fg: self.fg, bg: self.bg, attrs: self.attrs, width }
    }

    fn erased(&self) -> Cell {
        Cell { ch: ' ', fg: XTERM_FG, bg: self.bg, attrs: flag::DEFAULT_FG | (self.attrs & flag::DEFAULT_BG), width: 1 }
    }
}

fn default_blank() -> Cell {
    Pen::DEFAULT.erased()
}

#[derive(Clone, Copy)]
struct SavedCursor {
    pos: Pos,
    pen: Pen,
    origin: bool,
    wrap_pending: bool,
    charsets: [Charset; 2],
    shifted: bool,
}

#[derive(Clone)]
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn new(size: Size, blank: Cell) -> Self {
        Self { rows: (0..size.height).map(|_| Row::blank(size.width, blank)).collect() }
    }

    fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        self.rows.get(usize::from(y))?.cells.get(usize::from(x))
    }

    fn put(&mut self, x: u16, y: u16, cell: Cell) {
        let Some(row) = self.rows.get_mut(usize::from(y)) else { return };
        let x = usize::from(x);
        if x >= row.cells.len() {
            return;
        }
        let blank_of = |cell: Cell| Cell { ch: ' ', width: 1, ..cell };
        let current = row.cells[x];
        if current.width == 0 && x > 0 && row.cells[x - 1].width == 2 {
            row.cells[x - 1] = blank_of(row.cells[x - 1]);
        }
        if current.width == 2 && x + 1 < row.cells.len() && row.cells[x + 1].width == 0 {
            row.cells[x + 1] = blank_of(row.cells[x + 1]);
        }
        if cell.width == 2 {
            if x + 1 >= row.cells.len() {
                row.cells[x] = blank_of(cell);
                return;
            }
            if row.cells[x + 1].width == 2 && x + 2 < row.cells.len() && row.cells[x + 2].width == 0 {
                row.cells[x + 2] = blank_of(row.cells[x + 2]);
            }
            row.cells[x] = cell;
            row.cells[x + 1] = Cell { ch: '\0', width: 0, ..cell };
        } else {
            row.cells[x] = cell;
        }
    }

    fn fix_boundaries(&mut self, y: u16, x: u16) {
        let Some(row) = self.rows.get_mut(usize::from(y)) else { return };
        let x = usize::from(x);
        if x < row.cells.len() && row.cells[x].width == 0 {
            if x > 0 && row.cells[x - 1].width == 2 {
                row.cells[x - 1] = Cell { ch: ' ', width: 1, ..row.cells[x - 1] };
            }
            row.cells[x] = Cell { ch: ' ', width: 1, ..row.cells[x] };
        }
        if let Some(last) = row.cells.last_mut() {
            if last.width == 2 {
                *last = Cell { ch: ' ', width: 1, ..*last };
            }
        }
    }
}

/// 📟️ A VT screen: primary and alternate grids, history, cursor, pen, modes, and the replies the child is owed.
pub struct VtScreen {
    pub size: Size,
    primary: Grid,
    alt: Grid,
    pub alt_active: bool,
    history: History,
    anchor: Option<u64>,
    pub cursor: Pos,
    saved: SavedCursor,
    pen: Pen,
    pub scroll_top: u16,
    pub scroll_bottom: u16,
    pub modes: Modes,
    pub input: InputModes,
    wrap_pending: bool,
    tab_stops: Vec<bool>,
    charsets: [Charset; 2],
    shifted: bool,
    last_char: Option<char>,
    pub title: Option<String>,
    pub cwd: Option<String>,
    pub palette: Palette,
    replies: Vec<u8>,
    clipboard: Option<String>,
    pub bells: u32,
    parser: VtParser,
}

fn default_tab_stops(width: u16) -> Vec<bool> {
    (0..width).map(|x| x % 8 == 0 && x != 0).collect()
}

impl VtScreen {
    /// 🌱️ Creates a blank screen of `size` that keeps `scrollback_cap` history rows (0 means 10000).
    pub fn new(size: Size, scrollback_cap: usize) -> Self {
        let size = Size { width: size.width.max(1), height: size.height.max(1) };
        let blank = default_blank();
        Self {
            size,
            primary: Grid::new(size, blank),
            alt: Grid::new(size, blank),
            alt_active: false,
            history: History::new(if scrollback_cap == 0 { DEFAULT_SCROLLBACK } else { scrollback_cap }),
            anchor: None,
            cursor: Pos { x: 0, y: 0 },
            saved: SavedCursor { pos: Pos { x: 0, y: 0 }, pen: Pen::DEFAULT, origin: false, wrap_pending: false, charsets: [Charset::Ascii; 2], shifted: false },
            pen: Pen::DEFAULT,
            scroll_top: 0,
            scroll_bottom: size.height - 1,
            modes: Modes::default(),
            input: InputModes::default(),
            wrap_pending: false,
            tab_stops: default_tab_stops(size.width),
            charsets: [Charset::Ascii; 2],
            shifted: false,
            last_char: None,
            title: None,
            cwd: None,
            palette: Palette::xterm(),
            replies: Vec::new(),
            clipboard: None,
            bells: 0,
            parser: VtParser::new(),
        }
    }

    fn grid(&self) -> &Grid {
        if self.alt_active {
            &self.alt
        } else {
            &self.primary
        }
    }

    fn grid_mut(&mut self) -> &mut Grid {
        if self.alt_active {
            &mut self.alt
        } else {
            &mut self.primary
        }
    }

    /// 🪣️ Feeds child output through the owned incremental parser.
    pub fn feed(&mut self, bytes: &[u8]) {
        let mut parser = std::mem::take(&mut self.parser);
        parser.feed(bytes, self);
        self.parser = parser;
    }

    /// 📤️ Takes the bytes the child is owed as answers to its queries.
    pub fn take_replies(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.replies)
    }

    /// 🗒️ Takes the text the child asked the host to put on the clipboard.
    pub fn take_clipboard(&mut self) -> Option<String> {
        self.clipboard.take()
    }

    /// 🧩️ The child's input modes including whether it is on the alternate screen.
    pub fn input_modes(&self) -> InputModes {
        InputModes { alt_screen: self.alt_active, ..self.input }
    }

    pub fn visible_line_count(&self) -> u16 {
        self.size.height
    }

    pub fn scrollback_len(&self) -> usize {
        self.history.len()
    }

    pub fn cell_at(&self, x: u16, y: u16) -> Option<&Cell> {
        self.grid().get(x, y)
    }

    pub(super) fn reply(&mut self, bytes: &[u8]) {
        if self.replies.len() < 64 * 1024 {
            self.replies.extend_from_slice(bytes);
        }
    }

    pub(super) fn set_clipboard(&mut self, text: String) {
        self.clipboard = Some(text);
    }

    /// 🪜️ The absolute id of the screen's first row.
    pub fn screen_top(&self) -> u64 {
        self.history.base() + self.history.len() as u64
    }

    /// 🔝️ The absolute id of the oldest retained row.
    pub fn first_row(&self) -> u64 {
        self.history.base()
    }

    /// 👁️ The absolute id of the row at the top of the viewport.
    pub fn view_top(&self) -> u64 {
        match (self.alt_active, self.anchor) {
            (false, Some(anchor)) => anchor.clamp(self.history.base(), self.screen_top()),
            _ => self.screen_top(),
        }
    }

    /// ⬆️ How many rows the viewport sits above the live screen.
    pub fn view_offset(&self) -> usize {
        (self.screen_top() - self.view_top()) as usize
    }

    /// ⏬️ Whether the viewport follows new output.
    pub fn following(&self) -> bool {
        self.view_offset() == 0
    }

    fn set_view_top(&mut self, top: u64) {
        let top = top.clamp(self.history.base(), self.screen_top());
        self.anchor = if top >= self.screen_top() { None } else { Some(top) };
    }

    /// 🖱️ Moves the viewport `lines` rows toward older output (negative toward newer), clamped to the history.
    pub fn scroll_view(&mut self, lines: i64) {
        if self.alt_active {
            return;
        }
        let top = i128::from(self.view_top()) - i128::from(lines);
        self.set_view_top(top.clamp(0, i128::from(u64::MAX)) as u64);
    }

    pub fn scroll_view_to_bottom(&mut self) {
        self.anchor = None;
    }

    pub fn scroll_view_to_top(&mut self) {
        if !self.alt_active {
            self.set_view_top(self.history.base());
        }
    }

    /// 🧲️ Puts the viewport `offset` rows above the live screen.
    pub fn set_view_offset(&mut self, offset: usize) {
        if !self.alt_active {
            self.set_view_top(self.screen_top().saturating_sub(offset as u64));
        }
    }

    /// 🧾️ The row with absolute id `abs`, from history or the visible grid.
    pub fn row_at(&self, abs: u64) -> Option<&Row> {
        let top = self.screen_top();
        if abs < top {
            if self.alt_active {
                None
            } else {
                self.history.get(abs)
            }
        } else {
            self.grid().rows.get(usize::try_from(abs - top).ok()?)
        }
    }

    /// 🖼️ Paints the viewport into `rect` of `dest` with the colours of `palette`.
    pub fn blit(&self, dest: &mut CellBuffer, rect: Rect, palette: &Palette) {
        let clip = Rect::new(0, 0, dest.size.width, dest.size.height).intersect(rect);
        if clip.width == 0 || clip.height == 0 {
            return;
        }
        let top = self.view_top();
        let (dx, dy) = (clip.x - rect.x, clip.y - rect.y);
        for row in 0..clip.height {
            let Some(line) = self.row_at(top + u64::from(dy + row)) else { continue };
            for col in 0..clip.width {
                let Some(cell) = line.cells.get(usize::from(dx + col)) else { break };
                let (fg, bg) = palette.resolve(cell);
                let cut = (cell.width == 0 && col == 0) || (cell.width == 2 && col + 1 >= clip.width);
                let shown = if cut { Cell { ch: ' ', width: 1, ..*cell } } else { *cell };
                dest.put(clip.x + col, clip.y + row, Cell { fg, bg, attrs: shown.attrs & !flag::MASK, ..shown });
            }
        }
    }

    /// 🔄️ Resizes the screen, re-wrapping the primary text to the new width and cropping the alternate screen.
    pub fn resize(&mut self, size: Size) {
        let size = Size { width: size.width.max(1), height: size.height.max(1) };
        if size == self.size {
            return;
        }
        let old = self.size;
        let alt_active = self.alt_active;
        let primary_cursor = if alt_active { self.saved.pos } else { self.cursor };
        let history_len = self.history.len();
        let base = self.history.base();
        let mut rows = self.history.take_rows();
        rows.extend(std::mem::take(&mut self.primary.rows));
        let was_pending = if alt_active { self.saved.wrap_pending } else { self.wrap_pending };
        let mut spots = vec![Spot { row: history_len + usize::from(primary_cursor.y), col: primary_cursor.x + u16::from(was_pending) }, Spot { row: history_len, col: 0 }];
        if let Some(anchor) = self.anchor {
            spots.push(Spot { row: (anchor.saturating_sub(base) as usize).min(rows.len().saturating_sub(1)), col: 0 });
        }
        let out = reflow(rows, size.width, &spots, default_blank());
        let mut rows = out.rows;
        let (cursor_spot, top_spot) = (out.spots[0], out.spots[1]);
        while rows.len() > cursor_spot.row + 1 && rows.last().is_some_and(|row| row.is_blank() && !row.wrapped) {
            rows.pop();
        }
        let height = usize::from(size.height);
        let mut top = top_spot.row.min(rows.len().saturating_sub(1));
        if cursor_spot.row >= top + height {
            top = cursor_spot.row + 1 - height;
        }
        if rows.len() < top + height && top > 0 && usize::from(primary_cursor.y) + 1 == usize::from(old.height) {
            top = rows.len().saturating_sub(height);
        }
        if cursor_spot.row < top {
            top = cursor_spot.row;
        }
        while rows.len() < top + height {
            rows.push(Row::blank(size.width, default_blank()));
        }
        rows.truncate(top + height);
        let screen_rows = rows.split_off(top);
        let excess = rows.len().saturating_sub(self.history.cap());
        self.history.replace(rows);
        self.primary = Grid { rows: screen_rows };
        self.anchor = out.spots.get(2).and_then(|spot| if spot.row >= top { None } else { Some(self.history.base() + spot.row.saturating_sub(excess) as u64) });
        let new_cursor = Pos { x: cursor_spot.col.min(size.width - 1), y: (cursor_spot.row - top) as u16 };
        let pending = cursor_spot.col >= size.width;

        let blank = default_blank();
        let mut next_alt = Grid::new(size, blank);
        for y in 0..old.height.min(size.height) {
            for x in 0..old.width.min(size.width) {
                if let Some(cell) = self.alt.get(x, y) {
                    next_alt.rows[usize::from(y)].cells[usize::from(x)] = *cell;
                }
            }
            next_alt.fix_boundaries(y, old.width.min(size.width));
        }
        self.alt = next_alt;
        self.size = size;
        self.scroll_top = 0;
        self.scroll_bottom = size.height - 1;
        let mut stops = default_tab_stops(size.width);
        for (x, stop) in self.tab_stops.iter().enumerate().take(stops.len()) {
            stops[x] = *stop;
        }
        self.tab_stops = stops;
        if alt_active {
            self.saved.pos = new_cursor;
            self.saved.wrap_pending = pending;
            self.cursor = Pos { x: self.cursor.x.min(size.width - 1), y: self.cursor.y.min(size.height - 1) };
            self.wrap_pending = false;
        } else {
            self.cursor = new_cursor;
            self.wrap_pending = pending;
            self.saved.pos = Pos { x: self.saved.pos.x.min(size.width - 1), y: self.saved.pos.y.min(size.height - 1) };
        }
    }

    fn clamp_cursor(&mut self) {
        let (min_y, max_y) = if self.modes.origin { (self.scroll_top, self.scroll_bottom) } else { (0, self.size.height - 1) };
        self.cursor.x = self.cursor.x.min(self.size.width - 1);
        self.cursor.y = self.cursor.y.clamp(min_y, max_y);
        self.wrap_pending = false;
    }

    fn erased_row(&self) -> Row {
        Row::blank(self.size.width, self.pen.erased())
    }

    fn erase_span(&mut self, y: u16, from: u16, to: u16) {
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

    fn clear_wrapped(&mut self, y: u16) {
        if let Some(row) = self.grid_mut().rows.get_mut(usize::from(y)) {
            row.wrapped = false;
        }
    }

    pub(super) fn scroll_up(&mut self, n: u16) {
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

    pub(super) fn scroll_down(&mut self, n: u16) {
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

    pub(super) fn carriage_return(&mut self) {
        self.cursor.x = 0;
        self.wrap_pending = false;
    }

    pub(super) fn linefeed(&mut self) {
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

    pub(super) fn index(&mut self) {
        self.linefeed();
    }

    pub(super) fn next_line(&mut self) {
        self.carriage_return();
        self.linefeed();
    }

    pub(super) fn reverse_index(&mut self) {
        self.wrap_pending = false;
        if self.cursor.y == self.scroll_top {
            self.scroll_down(1);
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
        }
    }

    pub(super) fn backspace(&mut self) {
        self.wrap_pending = false;
        self.cursor.x = self.cursor.x.saturating_sub(1);
    }

    pub(super) fn bell(&mut self) {
        self.bells = self.bells.wrapping_add(1);
    }

    pub(super) fn tab_forward(&mut self, n: i32) {
        self.wrap_pending = false;
        for _ in 0..n.clamp(1, i32::from(self.size.width)) {
            let next = (self.cursor.x + 1..self.size.width).find(|x| self.tab_stops[usize::from(*x)]);
            self.cursor.x = next.unwrap_or(self.size.width - 1);
        }
    }

    pub(super) fn tab_back(&mut self, n: i32) {
        self.wrap_pending = false;
        for _ in 0..n.clamp(1, i32::from(self.size.width)) {
            let previous = (0..self.cursor.x).rev().find(|x| self.tab_stops[usize::from(*x)]);
            self.cursor.x = previous.unwrap_or(0);
        }
    }

    pub(super) fn set_tab_stop(&mut self) {
        let x = usize::from(self.cursor.x);
        self.tab_stops[x] = true;
    }

    pub(super) fn clear_tab_stops(&mut self, all: bool) {
        if all {
            self.tab_stops.iter_mut().for_each(|stop| *stop = false);
        } else {
            let x = usize::from(self.cursor.x);
            self.tab_stops[x] = false;
        }
    }

    pub(super) fn shift_charset(&mut self, shifted: bool) {
        self.shifted = shifted;
    }

    pub(super) fn designate_charset(&mut self, slot: usize, charset: Charset) {
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

    pub(super) fn print(&mut self, c: char) {
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

    pub(super) fn repeat_last(&mut self, n: i32) {
        if let Some(c) = self.last_char {
            for _ in 0..n.clamp(1, 65_535) {
                self.print(c);
            }
        }
    }

    pub(super) fn cursor_up(&mut self, n: i32) {
        self.wrap_pending = false;
        let limit = if self.cursor.y >= self.scroll_top { self.scroll_top } else { 0 };
        self.cursor.y = (i32::from(self.cursor.y) - n.max(1)).max(i32::from(limit)) as u16;
    }

    pub(super) fn cursor_down(&mut self, n: i32) {
        self.wrap_pending = false;
        let limit = if self.cursor.y <= self.scroll_bottom { self.scroll_bottom } else { self.size.height - 1 };
        self.cursor.y = i32::from(self.cursor.y).saturating_add(n.max(1)).min(i32::from(limit)) as u16;
    }

    pub(super) fn cursor_forward(&mut self, n: i32) {
        self.wrap_pending = false;
        self.cursor.x = i32::from(self.cursor.x).saturating_add(n.max(1)).min(i32::from(self.size.width - 1)) as u16;
    }

    pub(super) fn cursor_back(&mut self, n: i32) {
        self.wrap_pending = false;
        self.cursor.x = (i32::from(self.cursor.x) - n.max(1)).max(0) as u16;
    }

    pub(super) fn cursor_column(&mut self, col: i32) {
        self.wrap_pending = false;
        self.cursor.x = (col.max(1) - 1).min(i32::from(self.size.width - 1)) as u16;
    }

    pub(super) fn cursor_row(&mut self, row: i32) {
        self.wrap_pending = false;
        let (base, max) = if self.modes.origin { (self.scroll_top, self.scroll_bottom) } else { (0, self.size.height - 1) };
        self.cursor.y = i32::from(base).saturating_add(row.max(1) - 1).min(i32::from(max)) as u16;
    }

    pub(super) fn cup(&mut self, row: i32, col: i32) {
        self.cursor_row(row);
        self.cursor_column(col);
    }

    pub(super) fn save_cursor(&mut self) {
        self.saved = SavedCursor { pos: self.cursor, pen: self.pen, origin: self.modes.origin, wrap_pending: self.wrap_pending, charsets: self.charsets, shifted: self.shifted };
    }

    pub(super) fn restore_cursor(&mut self) {
        let saved = self.saved;
        self.cursor = saved.pos;
        self.pen = saved.pen;
        self.modes.origin = saved.origin;
        self.charsets = saved.charsets;
        self.shifted = saved.shifted;
        self.clamp_cursor();
        self.wrap_pending = saved.wrap_pending && self.cursor.x == self.size.width - 1;
    }

    pub(super) fn set_scroll_region(&mut self, top: i32, bottom: i32) {
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

    pub(super) fn erase_display(&mut self, mode: i32) {
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

    pub(super) fn erase_line(&mut self, mode: i32) {
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

    pub(super) fn erase_cells(&mut self, n: i32) {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let end = i32::from(x).saturating_add(n.max(1)).min(i32::from(self.size.width)) as u16;
        self.erase_span(y, x, end);
    }

    pub(super) fn insert_lines(&mut self, n: i32) {
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

    pub(super) fn delete_lines(&mut self, n: i32) {
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

    pub(super) fn insert_cells(&mut self, n: i32) {
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

    pub(super) fn delete_cells(&mut self, n: i32) {
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

    pub(super) fn alignment_test(&mut self) {
        let cell = self.pen.glyph('E', 1);
        let (width, height) = (self.size.width, self.size.height);
        for y in 0..height {
            for x in 0..width {
                self.grid_mut().put(x, y, cell);
            }
        }
        self.cup(1, 1);
    }

    pub(super) fn set_ansi_mode(&mut self, mode: i32, on: bool) {
        match mode {
            4 => self.modes.insert = on,
            20 => self.input.newline = on,
            _ => {}
        }
    }

    pub(super) fn ansi_mode_state(&self, mode: i32) -> u8 {
        match mode {
            4 => 1 + u8::from(!self.modes.insert),
            20 => 1 + u8::from(!self.input.newline),
            _ => 0,
        }
    }

    pub(super) fn set_private_mode(&mut self, mode: i32, on: bool) {
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

    pub(super) fn private_mode_state(&self, mode: i32) -> u8 {
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

    fn enter_alternate(&mut self, save: bool) {
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

    fn leave_alternate(&mut self, restore: bool, clear: bool) {
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

    pub(super) fn set_cursor_style(&mut self, style: i32) {
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

    pub(super) fn soft_reset(&mut self) {
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

    pub(super) fn hard_reset(&mut self) {
        let replies = std::mem::take(&mut self.replies);
        let history = std::mem::replace(&mut self.history, History::new(1));
        let cap = history.cap();
        let palette = self.palette;
        *self = Self::new(self.size, cap);
        self.history = history;
        self.replies = replies;
        self.palette = palette;
    }

    pub(super) fn apply_sgr(&mut self, params: &[i32], subs: &[Vec<i32>]) {
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
                30..=37 => self.set_fg(super::palette::XTERM_ANSI[(params[i] - 30) as usize]),
                39 => {
                    self.pen.fg = XTERM_FG;
                    self.pen.attrs |= flag::DEFAULT_FG;
                }
                40..=47 => self.set_bg(super::palette::XTERM_ANSI[(params[i] - 40) as usize]),
                49 => {
                    self.pen.bg = XTERM_BG;
                    self.pen.attrs |= flag::DEFAULT_BG;
                }
                90..=97 => self.set_fg(super::palette::XTERM_ANSI[(params[i] - 90 + 8) as usize]),
                100..=107 => self.set_bg(super::palette::XTERM_ANSI[(params[i] - 100 + 8) as usize]),
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
            [5, n, ..] => (Some(super::palette::color_256(channel(*n))), 2),
            [2, r, g, b, ..] => (Some([channel(*r), channel(*g), channel(*b)]), 4),
            _ => (None, 0),
        }
    }

    fn colon_color(sub: &[i32]) -> Option<Rgb> {
        let channel = |v: i32| v.clamp(0, 255) as u8;
        match sub {
            [5, n, ..] => Some(super::palette::color_256(channel(*n))),
            [2, r, g, b] => Some([channel(*r), channel(*g), channel(*b)]),
            [2, _, r, g, b, ..] => Some([channel(*r), channel(*g), channel(*b)]),
            _ => None,
        }
    }

    pub(super) fn report_cursor(&mut self, private: bool) {
        let row = i32::from(self.cursor.y) + 1 - if self.modes.origin { i32::from(self.scroll_top) } else { 0 };
        let text = format!("\x1b[{}{};{}R", if private { "?" } else { "" }, row, i32::from(self.cursor.x) + 1);
        self.reply(text.as_bytes());
    }

    pub(super) fn report_text_area(&mut self) {
        let text = format!("\x1b[8;{};{}t", self.size.height, self.size.width);
        self.reply(text.as_bytes());
    }

    pub(super) fn report_color(&mut self, slot: u8, background: bool) {
        let color = if background { self.palette.bg } else { self.palette.fg };
        let text = format!("\x1b]{slot};rgb:{0:02x}{0:02x}/{1:02x}{1:02x}/{2:02x}{2:02x}\x1b\\", color[0], color[1], color[2]);
        self.reply(text.as_bytes());
    }

    fn cell_text(row: &Row, from: usize, to: usize) -> String {
        let mut out = String::new();
        for cell in row.cells.iter().take(to + 1).skip(from) {
            if cell.width != 0 && cell.ch != '\0' {
                out.push(cell.ch);
            }
        }
        out
    }

    /// 🗞️ The text between two cells, inclusive, with soft-wrapped rows joined and trailing blanks trimmed.
    pub fn text_between(&self, a: CellPoint, b: CellPoint) -> String {
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        let mut out = String::new();
        for abs in a.row..=b.row {
            let Some(row) = self.row_at(abs) else { continue };
            let from = if abs == a.row { usize::from(a.col) } else { 0 };
            let to = if abs == b.row { usize::from(b.col) } else { row.cells.len().saturating_sub(1) };
            let mut text = Self::cell_text(row, from, to);
            if !row.wrapped {
                text.truncate(text.trim_end().len());
            }
            out.push_str(&text);
            if !row.wrapped && abs != b.row {
                out.push('\n');
            }
        }
        out
    }

    fn char_class(cell: &Cell) -> u8 {
        let c = cell.ch;
        if c == ' ' || c == '\0' {
            0
        } else if "()[]{}<>\"'`|,;".contains(c) {
            1
        } else {
            2
        }
    }

    /// 🔡️ The word, delimiter run or blank run around `point`.
    pub fn word_span(&self, point: CellPoint) -> (CellPoint, CellPoint) {
        let Some(row) = self.row_at(point.row) else { return (point, point) };
        let mut at = usize::from(point.col).min(row.cells.len().saturating_sub(1));
        while at > 0 && row.cells[at].width == 0 {
            at -= 1;
        }
        let class = Self::char_class(&row.cells[at]);
        let mut start = at;
        while start > 0 && (row.cells[start - 1].width == 0 || Self::char_class(&row.cells[start - 1]) == class) {
            start -= 1;
        }
        let mut end = at;
        while end + 1 < row.cells.len() && (row.cells[end + 1].width == 0 || Self::char_class(&row.cells[end + 1]) == class) {
            end += 1;
        }
        (CellPoint { row: point.row, col: start as u16 }, CellPoint { row: point.row, col: end as u16 })
    }

    /// 🧵️ The whole logical line (all soft-wrapped rows) holding `point`.
    pub fn line_span(&self, point: CellPoint) -> (CellPoint, CellPoint) {
        let mut first = point.row;
        while first > self.first_row() && self.row_at(first - 1).is_some_and(|row| row.wrapped) {
            first -= 1;
        }
        let mut last = point.row;
        while self.row_at(last).is_some_and(|row| row.wrapped) && self.row_at(last + 1).is_some() {
            last += 1;
        }
        (CellPoint { row: first, col: 0 }, CellPoint { row: last, col: self.size.width - 1 })
    }

    /// 🔗️ The URL under `point`, if the blank-delimited word there contains `://`.
    pub fn url_at(&self, point: CellPoint) -> Option<String> {
        let row = self.row_at(point.row)?;
        let at = usize::from(point.col);
        let blank = |i: usize| row.cells.get(i).is_none_or(|cell| cell.ch == ' ');
        if blank(at) {
            return None;
        }
        let mut start = at;
        while start > 0 && !blank(start - 1) {
            start -= 1;
        }
        let mut end = at;
        while !blank(end + 1) {
            end += 1;
        }
        let word = Self::cell_text(row, start, end);
        let word = word.trim_end_matches(['.', ',', ';', ':', '!', '?', ')', ']', '}', '>', '\'', '"']);
        let scheme = word.find("://")?;
        (scheme > 0 && word[..scheme].chars().all(|c| c.is_ascii_alphabetic()) && word.len() > scheme + 3).then(|| word.to_string())
    }

    /// 🔭️ Every case-insensitive occurrence of `needle` in a single row, oldest first.
    pub fn find(&self, needle: &str) -> Vec<Match> {
        let needle: Vec<char> = needle.chars().flat_map(char::to_lowercase).collect();
        if needle.is_empty() {
            return Vec::new();
        }
        let mut hits = Vec::new();
        let last = self.screen_top() + u64::from(self.size.height);
        let first = if self.alt_active { self.screen_top() } else { self.first_row() };
        for abs in first..last {
            let Some(row) = self.row_at(abs) else { continue };
            let glyphs: Vec<(char, u16)> = row.cells.iter().enumerate().filter(|(_, cell)| cell.width != 0).map(|(x, cell)| (cell.ch.to_lowercase().next().unwrap_or(cell.ch), x as u16)).collect();
            if glyphs.len() < needle.len() {
                continue;
            }
            for start in 0..=glyphs.len() - needle.len() {
                if glyphs[start..start + needle.len()].iter().map(|(c, _)| *c).eq(needle.iter().copied()) {
                    let col = glyphs[start].1;
                    let end = glyphs[start + needle.len() - 1].1;
                    let tail = u16::from(row.cells[usize::from(end)].width.max(1));
                    hits.push(Match { row: abs, col, cells: end - col + tail });
                    if hits.len() >= MAX_MATCHES {
                        return hits;
                    }
                }
            }
        }
        hits
    }
}
