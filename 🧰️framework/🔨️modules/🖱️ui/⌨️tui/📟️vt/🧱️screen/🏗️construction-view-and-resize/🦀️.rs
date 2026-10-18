//! 🏗️ Construction, view and resize: the `VtScreen` constructor, grid access, replies, the scrollback view anchoring, row access, blit and the reflowing resize. See [`super`].

use super::*;

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

    pub(super) fn grid(&self) -> &Grid {
        if self.alt_active {
            &self.alt
        } else {
            &self.primary
        }
    }

    pub(super) fn grid_mut(&mut self) -> &mut Grid {
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

    pub(in super::super) fn reply(&mut self, bytes: &[u8]) {
        if self.replies.len() < 64 * 1024 {
            self.replies.extend_from_slice(bytes);
        }
    }

    pub(in super::super) fn set_clipboard(&mut self, text: String) {
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

}
