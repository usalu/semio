//! 🖥️ Handcrafted retained-mode terminal UI: semio-styled scene, cell renderer, and ANSI backend.

// #region 🔖️Geometry
pub mod geometry {
    /// 📍️ A cell coordinate on the terminal grid.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct Pos {
        pub x: u16,
        pub y: u16,
    }

    /// 📏️ A cell-grid size.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct Size {
        pub width: u16,
        pub height: u16,
    }

    /// 🔲️ An axis-aligned cell-grid rectangle.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct Rect {
        pub x: u16,
        pub y: u16,
        pub width: u16,
        pub height: u16,
    }

    impl Rect {
        pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
            Self { x, y, width, height }
        }

        /// 🧭️ Whether `pos` lies within this rect.
        pub fn contains(&self, pos: Pos) -> bool {
            pos.x >= self.x && pos.x < self.x + self.width && pos.y >= self.y && pos.y < self.y + self.height
        }

        /// ✂️ The overlap between two rects (an empty rect on miss).
        pub fn intersect(&self, other: Rect) -> Rect {
            let x0 = self.x.max(other.x);
            let y0 = self.y.max(other.y);
            let x1 = (self.x + self.width).min(other.x + other.width);
            let y1 = (self.y + self.height).min(other.y + other.height);
            if x1 <= x0 || y1 <= y0 {
                Rect::default()
            } else {
                Rect::new(x0, y0, x1 - x0, y1 - y0)
            }
        }

        /// 🧊️ Shrinks the rect by `margin` cells on every side.
        pub fn inset(&self, margin: u16) -> Rect {
            self.inset_sides(margin, margin, margin, margin)
        }

        /// 🧊️ Shrinks the rect by `top`/`right`/`bottom`/`left` cells.
        pub fn inset_sides(&self, top: u16, right: u16, bottom: u16, left: u16) -> Rect {
            let width = self.width.saturating_sub(left + right);
            let height = self.height.saturating_sub(top + bottom);
            Rect::new(self.x + left.min(self.width), self.y + top.min(self.height), width, height)
        }

        /// ✂️ Splits off `rows` rows from the top, returning `(top, rest)`.
        pub fn split_top(&self, rows: u16) -> (Rect, Rect) {
            let rows = rows.min(self.height);
            let top = Rect::new(self.x, self.y, self.width, rows);
            let rest = Rect::new(self.x, self.y + rows, self.width, self.height - rows);
            (top, rest)
        }

        /// ✂️ Splits off `rows` rows from the bottom, returning `(rest, bottom)`.
        pub fn split_bottom(&self, rows: u16) -> (Rect, Rect) {
            let rows = rows.min(self.height);
            let bottom = Rect::new(self.x, self.y + self.height - rows, self.width, rows);
            let rest = Rect::new(self.x, self.y, self.width, self.height - rows);
            (rest, bottom)
        }
    }
}
// #endregion 🔖️Geometry

// #region 🔖️Theme
pub mod theme {
    use ui_styling::appearance::AppearanceName;
    use ui_styling::color::linear_to_rgba8;
    use ui_styling::ChromePalette;

    /// 🎨️ An 8-bit truecolor triple.
    pub type Rgb = [u8; 3];

    /// 🪟️ The six nested semio chrome surfaces (base → window → pane → panel → dialog → menu).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Surface {
        Base,
        Window,
        Pane,
        Panel,
        Dialog,
        Menu,
    }

    /// 🏷️ A semantic foreground/border/state role, resolved against the active palette.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Role {
        Foreground,
        MutedForeground,
        Accent,
        AccentForeground,
        ActiveBase,
        ActiveForeground,
        BorderNormal,
        BorderEmphasized,
        BorderElement,
        HoverInteractive,
    }

    fn rgb(channel: [f32; 4]) -> Rgb {
        let [r, g, b, _a] = linear_to_rgba8(channel[0], channel[1], channel[2], channel[3]);
        [r, g, b]
    }

    /// 🖌️ A resolved semio theme: every chrome color precomputed once as 8-bit truecolor.
    pub struct Theme {
        pub appearance: AppearanceName,
        level_base: Rgb,
        level_window: Rgb,
        level_pane: Rgb,
        level_panel: Rgb,
        level_dialog: Rgb,
        level_menu: Rgb,
        foreground: Rgb,
        muted_foreground: Rgb,
        accent: Rgb,
        accent_foreground: Rgb,
        active_base: Rgb,
        active_foreground: Rgb,
        border_normal: Rgb,
        border_emphasized: Rgb,
        border_element: Rgb,
        hover_interactive: Rgb,
    }

    impl Theme {
        pub fn new(appearance: AppearanceName) -> Self {
            let p: &ChromePalette = appearance.chrome();
            Self {
                appearance,
                level_base: rgb(p.level_base),
                level_window: rgb(p.level_window),
                level_pane: rgb(p.level_pane),
                level_panel: rgb(p.level_panel),
                level_dialog: rgb(p.level_dialog),
                level_menu: rgb(p.level_menu),
                foreground: rgb(p.foreground),
                muted_foreground: rgb(p.muted_foreground),
                accent: rgb(p.accent),
                accent_foreground: rgb(p.accent_foreground),
                active_base: rgb(p.active_base),
                active_foreground: rgb(p.active_foreground),
                border_normal: rgb(p.border_normal),
                border_emphasized: rgb(p.border_emphasized),
                border_element: rgb(p.border_element),
                hover_interactive: rgb(p.hover_interactive_fill),
            }
        }

        pub fn surface(&self, surface: Surface) -> Rgb {
            match surface {
                Surface::Base => self.level_base,
                Surface::Window => self.level_window,
                Surface::Pane => self.level_pane,
                Surface::Panel => self.level_panel,
                Surface::Dialog => self.level_dialog,
                Surface::Menu => self.level_menu,
            }
        }

        pub fn role(&self, role: Role) -> Rgb {
            match role {
                Role::Foreground => self.foreground,
                Role::MutedForeground => self.muted_foreground,
                Role::Accent => self.accent,
                Role::AccentForeground => self.accent_foreground,
                Role::ActiveBase => self.active_base,
                Role::ActiveForeground => self.active_foreground,
                Role::BorderNormal => self.border_normal,
                Role::BorderEmphasized => self.border_emphasized,
                Role::BorderElement => self.border_element,
                Role::HoverInteractive => self.hover_interactive,
            }
        }

        pub fn set_appearance(&mut self, appearance: AppearanceName) {
            *self = Theme::new(appearance);
        }
    }
}
// #endregion 🔖️Theme

// #region 🔖️Text
pub mod text {
    /// 📐️ Terminal cell width of one `char` (0 for zero-width, 1 normal, 2 wide).
    pub(crate) fn char_cells(c: char) -> u8 {
        match unicode_width::UnicodeWidthChar::width(c) {
            Some(w) => w.min(2) as u8,
            None => 0,
        }
    }

    /// 📏️ Total display width in cells of a string.
    pub fn display_width(s: &str) -> u16 {
        s.chars().map(|c| u16::from(char_cells(c))).sum()
    }

    /// ✂️ Truncates `s` to at most `max_cells` display cells, returning the slice and its width.
    pub fn truncate_to(s: &str, max_cells: u16) -> (&str, u16) {
        let mut used = 0u16;
        let mut end = 0usize;
        for (idx, c) in s.char_indices() {
            let w = u16::from(char_cells(c));
            if used + w > max_cells {
                break;
            }
            used += w;
            end = idx + c.len_utf8();
        }
        (&s[..end], used)
    }
}
// #endregion 🔖️Text

// #region 🔖️Cell
pub mod cell {
    use crate::tui::geometry::{Pos, Rect, Size};
    use crate::tui::text::char_cells;
    use crate::tui::theme::Rgb;

    /// 🎛️ Bitflags for cell text attributes.
    pub mod attr {
        pub const BOLD: u8 = 1;
        pub const DIM: u8 = 2;
        pub const ITALIC: u8 = 4;
        pub const UNDERLINE: u8 = 8;
        pub const REVERSE: u8 = 16;
    }

    /// 🧱️ One terminal cell: a glyph, its colors, attributes, and cell width (0 = wide-char continuation).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Cell {
        pub ch: char,
        pub fg: Rgb,
        pub bg: Rgb,
        pub attrs: u8,
        pub width: u8,
    }

    impl Cell {
        pub fn blank(fg: Rgb, bg: Rgb) -> Self {
            Self { ch: ' ', fg, bg, attrs: 0, width: 1 }
        }
    }

    /// 🗺️ A retained grid of `Cell`s.
    #[derive(Clone)]
    pub struct CellBuffer {
        pub size: Size,
        cells: Vec<Cell>,
    }

    impl CellBuffer {
        pub fn new(size: Size, fill: Cell) -> Self {
            let count = usize::from(size.width) * usize::from(size.height);
            Self { size, cells: vec![fill; count] }
        }

        pub fn resize(&mut self, size: Size, fill: Cell) {
            *self = Self::new(size, fill);
        }

        fn index(&self, x: u16, y: u16) -> Option<usize> {
            if x < self.size.width && y < self.size.height {
                Some(usize::from(y) * usize::from(self.size.width) + usize::from(x))
            } else {
                None
            }
        }

        pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
            self.index(x, y).map(|i| &self.cells[i])
        }

        /// ✍️ Writes one cell, blanking an orphaned wide-char continuation on either side.
        pub fn put(&mut self, x: u16, y: u16, mut cell: Cell) {
            let Some(i) = self.index(x, y) else { return };
            if cell.width == 0 && x > 0 {
                if let Some(prev) = self.index(x - 1, y) {
                    if self.cells[prev].width == 2 {
                        // keep continuation paired with its lead cell
                    } else {
                        cell.width = 1;
                    }
                }
            }
            if cell.width == 2 && x + 1 >= self.size.width {
                cell.width = 1;
            }
            self.cells[i] = cell;
            if cell.width == 2 {
                if let Some(next) = self.index(x + 1, y) {
                    self.cells[next] = Cell { ch: '\0', width: 0, ..cell };
                }
            }
        }

        /// ✍️ Writes a string starting at `pos`, clipped to `clip`; returns cells consumed.
        pub fn put_str(&mut self, pos: Pos, s: &str, fg: Rgb, bg: Rgb, attrs: u8, clip: Rect) -> u16 {
            let mut x = pos.x;
            let mut written = 0u16;
            for c in s.chars() {
                let w = char_cells(c);
                if w == 0 {
                    continue;
                }
                if x + u16::from(w) > clip.x + clip.width || pos.y < clip.y || pos.y >= clip.y + clip.height {
                    break;
                }
                if x >= clip.x {
                    self.put(x, pos.y, Cell { ch: c, fg, bg, attrs, width: w });
                    if w == 2 {
                        self.put(x + 1, pos.y, Cell { ch: '\0', fg, bg, attrs, width: 0 });
                    }
                }
                x += u16::from(w);
                written += u16::from(w);
            }
            written
        }

        pub fn fill_rect(&mut self, rect: Rect, cell: Cell) {
            let clipped = Rect::new(0, 0, self.size.width, self.size.height).intersect(rect);
            for y in clipped.y..clipped.y + clipped.height {
                for x in clipped.x..clipped.x + clipped.width {
                    self.put(x, y, cell);
                }
            }
        }

        pub fn hline(&mut self, pos: Pos, len: u16, ch: char, fg: Rgb, bg: Rgb) {
            for i in 0..len {
                self.put(pos.x + i, pos.y, Cell { ch, fg, bg, attrs: 0, width: 1 });
            }
        }

        pub fn vline(&mut self, pos: Pos, len: u16, ch: char, fg: Rgb, bg: Rgb) {
            for i in 0..len {
                self.put(pos.x, pos.y + i, Cell { ch, fg, bg, attrs: 0, width: 1 });
            }
        }
    }

    /// 🩹️ A contiguous run of changed cells on one row.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct DiffRun {
        pub y: u16,
        pub x: u16,
        pub len: u16,
    }

    /// 🔍️ Computes the minimal set of changed-cell runs between two same-sized buffers.
    pub fn diff(prev: &CellBuffer, next: &CellBuffer) -> Vec<DiffRun> {
        const MERGE_GAP: u16 = 4;
        let mut runs = Vec::new();
        if prev.size != next.size {
            return vec![DiffRun { y: 0, x: 0, len: next.size.width * next.size.height }];
        }
        for y in 0..next.size.height {
            let mut run_start: Option<u16> = None;
            let mut last_diff: Option<u16> = None;
            for x in 0..next.size.width {
                let changed = prev.get(x, y) != next.get(x, y);
                if changed {
                    match (run_start, last_diff) {
                        (None, _) => run_start = Some(x),
                        (Some(_), Some(last)) if x - last > MERGE_GAP => {
                            runs.push(DiffRun { y, x: run_start.unwrap(), len: last - run_start.unwrap() + 1 });
                            run_start = Some(x);
                        }
                        _ => {}
                    }
                    last_diff = Some(x);
                }
            }
            if let (Some(start), Some(last)) = (run_start, last_diff) {
                runs.push(DiffRun { y, x: start, len: last - start + 1 });
            }
        }
        runs
    }
}
// #endregion 🔖️Cell

// #region 🔖️Ansi
pub mod ansi {
    use crate::tui::cell::{Cell, CellBuffer, DiffRun};
    use crate::tui::theme::Rgb;

    //#region 🔖️Emit
    /// 📦️ A batch of raw ANSI bytes ready to write to a terminal (or feed to xterm.js).
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

    /// 🖨️ Emits the minimal ANSI needed to repaint `runs` of `next` onto a terminal.
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

    /// 🚪️ Enters the alternate screen, hides the cursor, and enables mouse/paste reporting.
    pub fn setup_sequence() -> &'static str {
        "\x1b[?1049h\x1b[?25l\x1b[?1002h\x1b[?1006h\x1b[?2004h\x1b[2J"
    }

    /// 🚪️ Restores the primary screen and default modes.
    pub fn teardown_sequence() -> &'static str {
        "\x1b[?2004l\x1b[?1006l\x1b[?1002l\x1b[?25h\x1b[?1049l\x1b[0m"
    }
    //#endregion 🔖️Emit

    //#region 🔖️Parse
    use crate::tui::event::{mods, Event, Key, KeyEvent, MouseEvent, MouseKind};
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

    /// ⌨️ Handcrafted incremental ANSI input decoder (keys, mouse, paste, focus, UTF-8).
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

        /// 📥️ Feeds raw input bytes, appending any decoded events to `out`.
        pub fn feed(&mut self, bytes: &[u8], out: &mut Vec<Event>) {
            for &b in bytes {
                self.feed_byte(b, out);
            }
        }

        /// ⏱️ Resolves a lone pending ESC (no follow-up byte arrived before a poll timeout).
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
                0x00..=0x1a if b != 0x1b => {
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
            let kind = if b & 0x40 != 0 {
                if btn == 0 {
                    MouseKind::ScrollUp
                } else {
                    MouseKind::ScrollDown
                }
            } else if b & 0x20 != 0 {
                MouseKind::Drag(btn)
            } else if final_byte == b'm' {
                MouseKind::Up(btn)
            } else {
                MouseKind::Down(btn)
            };
            out.push(Event::Mouse(MouseEvent { kind, pos, mods: m }));
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
    //#endregion 🔖️Parse
}
// #endregion 🔖️Ansi

// #region 🔖️Vt
pub mod vt {
    use crate::tui::cell::{attr, Cell, CellBuffer};
    use crate::tui::geometry::{Pos, Rect, Size};
    use crate::tui::text::char_cells;
    use crate::tui::theme::Rgb;
    use std::collections::VecDeque;

    const DEFAULT_FG: Rgb = [192, 192, 192];
    const DEFAULT_BG: Rgb = [0, 0, 0];
    const DEFAULT_SCROLLBACK: usize = 10_000;

    //#region 🔖️Palette
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

    /// 🎨️ Maps a 256-color index onto an approximate truecolor RGB.
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
    //#endregion 🔖️Palette

    //#region 🔖️Parser
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ParserState {
        Ground,
        Escape,
        Csi,
        Osc,
        Dcs,
        SosPmApc,
    }

    /// 📟 Incremental VT output decoder (CSI/SGR/OSC/DCS) driving a `VtScreen`.
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
            Self {
                state: ParserState::Ground,
                params: Vec::new(),
                current: 0,
                has_current: false,
                intermediate: 0,
                private: false,
                osc: Vec::new(),
                utf8_buf: [0; 4],
                utf8_len: 0,
                utf8_need: 0,
                ignore_esc: false,
            }
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

        /// 📥️ Feeds raw PTY bytes into `screen`.
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
    //#endregion 🔖️Parser

    //#region 🔖️Screen
    #[derive(Clone, Copy)]
    struct SavedCursor {
        pos: Pos,
        fg: Rgb,
        bg: Rgb,
        attrs: u8,
        origin: bool,
    }

    /// 🖥️ VT screen: primary/alt buffers, scrollback, cursor, SGR, scroll region, modes.
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
        /// 🆕 Creates a blank VT screen of `size` with `scrollback_cap` (0 → default 10000).
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
            let (min_y, max_y) = if self.origin_mode {
                (self.scroll_top, self.scroll_bottom)
            } else {
                (0, self.size.height.saturating_sub(1))
            };
            self.cursor.x = self.cursor.x.min(max_x);
            self.cursor.y = self.cursor.y.clamp(min_y, max_y);
            self.wrap_pending = false;
        }

        /// ↔️ Resizes both buffers; clamps cursor into the new grid.
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

        /// 📥️ Feeds raw bytes through the owned incremental parser.
        pub fn feed(&mut self, bytes: &[u8]) {
            let mut parser = std::mem::replace(&mut self.parser, VtParser::new());
            parser.feed(bytes, self);
            self.parser = parser;
        }

        /// 👁 Visible viewport row count.
        pub fn visible_line_count(&self) -> u16 {
            self.size.height
        }

        /// 📜 Lines currently held in scrollback.
        pub fn scrollback_len(&self) -> usize {
            self.scrollback.len()
        }

        /// 🔎 Reads one cell from the active buffer.
        pub fn cell_at(&self, x: u16, y: u16) -> Option<&Cell> {
            self.active_buf().get(x, y)
        }

        /// 🖼️ Composites the visible viewport (optionally offset into scrollback) into `dest`.
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
                        Some(
                            (0..self.size.width)
                                .map(|x| buf.get(x, vy).copied().unwrap_or_else(|| Cell::blank(DEFAULT_FG, DEFAULT_BG)))
                                .collect::<Vec<_>>(),
                        )
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
            let (y_base, y_max) = if self.origin_mode {
                (self.scroll_top, self.scroll_bottom)
            } else {
                (0, self.size.height.saturating_sub(1))
            };
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
    //#endregion 🔖️Screen
}
// #endregion 🔖️Vt

// #region 🔖️Event
pub mod event {
    use crate::tui::geometry::{Pos, Size};

    /// ⌨️ A decoded terminal key.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Key {
        Char(char),
        Enter,
        Esc,
        Tab,
        BackTab,
        Backspace,
        Delete,
        Insert,
        Up,
        Down,
        Left,
        Right,
        Home,
        End,
        PageUp,
        PageDown,
        F(u8),
    }

    /// 🎛️ Modifier bitflags.
    pub mod mods {
        pub const SHIFT: u8 = 1;
        pub const ALT: u8 = 2;
        pub const CTRL: u8 = 4;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct KeyEvent {
        pub key: Key,
        pub mods: u8,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MouseKind {
        Down(u8),
        Up(u8),
        Drag(u8),
        Move,
        ScrollUp,
        ScrollDown,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MouseEvent {
        pub kind: MouseKind,
        pub pos: Pos,
        pub mods: u8,
    }

    /// 📡️ Any input the terminal can report to the retained-mode engine.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Event {
        Key(KeyEvent),
        Mouse(MouseEvent),
        Paste(String),
        Resize(Size),
        FocusGained,
        FocusLost,
    }
}
// #endregion 🔖️Event

// #region 🔖️Scene
pub mod scene {
    use crate::tui::chrome::ChromeState;
    use crate::tui::geometry::{Pos, Rect};
    use crate::tui::layout::Constraint;
    use crate::tui::theme::{Role, Surface};
    use crate::tui::widget::WidgetState;

    const LAYOUT_DIRTY: u8 = 1;
    const PAINT_DIRTY: u8 = 2;

    /// 🪪️ A stable, generation-checked handle to a scene node.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NodeId {
        index: u32,
        generation: u32,
    }

    /// 🧱️ The payload a node carries.
    pub enum NodeContent {
        Box,
        Text(String),
        Widget(WidgetState),
        Chrome(ChromeState),
    }

    /// 🎨️ A node's visual role (independent of its content).
    #[derive(Default, Clone, Copy)]
    pub struct Style {
        pub surface: Option<Surface>,
        pub fg: Option<Role>,
        pub attrs: u8,
    }

    /// 🌳️ One retained scene node.
    pub struct Node {
        pub content: NodeContent,
        pub style: Style,
        pub constraint: Constraint,
        pub visible: bool,
        pub(crate) children: Vec<NodeId>,
        pub(crate) parent: Option<NodeId>,
        pub(crate) rect: Rect,
        pub(crate) dirty: u8,
    }

    impl Node {
        pub fn new(content: NodeContent) -> Self {
            Self { content, style: Style::default(), constraint: Constraint::default(), visible: true, children: Vec::new(), parent: None, rect: Rect::default(), dirty: LAYOUT_DIRTY | PAINT_DIRTY }
        }

        pub fn children(&self) -> &[NodeId] {
            &self.children
        }
    }

    struct Slot {
        node: Option<Node>,
        generation: u32,
    }

    /// 🗂️ A generational-arena retained scene tree, renderer-agnostic.
    pub struct Scene {
        slots: Vec<Slot>,
        free: Vec<u32>,
        root: NodeId,
    }

    impl Scene {
        pub fn new() -> Self {
            let mut slots = Vec::new();
            slots.push(Slot { node: Some(Node::new(NodeContent::Box)), generation: 0 });
            Self { slots, free: Vec::new(), root: NodeId { index: 0, generation: 0 } }
        }

        pub fn root(&self) -> NodeId {
            self.root
        }

        pub fn add(&mut self, parent: NodeId, mut node: Node) -> NodeId {
            node.parent = Some(parent);
            let id = if let Some(index) = self.free.pop() {
                let slot = &mut self.slots[index as usize];
                slot.generation += 1;
                let id = NodeId { index, generation: slot.generation };
                slot.node = Some(node);
                id
            } else {
                let index = self.slots.len() as u32;
                self.slots.push(Slot { node: Some(node), generation: 0 });
                NodeId { index, generation: 0 }
            };
            if let Some(p) = self.slots[parent.index as usize].node.as_mut() {
                p.children.push(id);
            }
            self.mark_dirty(parent, LAYOUT_DIRTY | PAINT_DIRTY);
            id
        }

        pub fn remove(&mut self, id: NodeId) {
            let children = self.node(id).children.clone();
            for child in children {
                self.remove(child);
            }
            if let Some(parent) = self.node(id).parent {
                if let Some(p) = self.slots[parent.index as usize].node.as_mut() {
                    p.children.retain(|c| *c != id);
                }
                self.mark_dirty(parent, LAYOUT_DIRTY | PAINT_DIRTY);
            }
            self.slots[id.index as usize].node = None;
            self.free.push(id.index);
        }

        fn valid(&self, id: NodeId) -> bool {
            self.slots.get(id.index as usize).is_some_and(|s| s.generation == id.generation && s.node.is_some())
        }

        pub fn node(&self, id: NodeId) -> &Node {
            self.slots[id.index as usize].node.as_ref().expect("stale NodeId")
        }

        pub(crate) fn node_raw_mut(&mut self, id: NodeId) -> &mut Node {
            self.slots[id.index as usize].node.as_mut().expect("stale NodeId")
        }

        pub fn rect(&self, id: NodeId) -> Rect {
            self.node(id).rect
        }

        pub(crate) fn mark_dirty(&mut self, id: NodeId, flags: u8) {
            let mut cursor = Some(id);
            while let Some(cur) = cursor {
                if !self.valid(cur) {
                    break;
                }
                let node = self.node_raw_mut(cur);
                if node.dirty & flags == flags {
                    break;
                }
                node.dirty |= flags;
                cursor = node.parent;
            }
        }

        pub(crate) fn take_dirty(&mut self, id: NodeId) -> u8 {
            let d = self.node(id).dirty;
            self.node_raw_mut(id).dirty = 0;
            d
        }

        pub fn node_mut(&mut self, id: NodeId) -> NodeMut<'_> {
            NodeMut { scene: self, id }
        }

        /// 🎯️ The deepest visible node whose rect contains `pos`.
        pub fn hit(&self, pos: Pos) -> Option<NodeId> {
            fn walk(scene: &Scene, id: NodeId, pos: Pos) -> Option<NodeId> {
                let node = scene.node(id);
                if !node.visible || !node.rect.contains(pos) {
                    return None;
                }
                for &child in node.children.iter().rev() {
                    if let Some(hit) = walk(scene, child, pos) {
                        return Some(hit);
                    }
                }
                Some(id)
            }
            walk(self, self.root, pos)
        }
    }

    impl Default for Scene {
        fn default() -> Self {
            Self::new()
        }
    }

    /// ✍️ A scoped mutation handle: every setter marks layout/paint dirty up the parent chain.
    pub struct NodeMut<'a> {
        scene: &'a mut Scene,
        id: NodeId,
    }

    impl<'a> NodeMut<'a> {
        pub fn set_text(&mut self, text: impl Into<String>) {
            self.scene.node_raw_mut(self.id).content = NodeContent::Text(text.into());
            self.scene.mark_dirty(self.id, LAYOUT_DIRTY | PAINT_DIRTY);
        }

        pub fn set_constraint(&mut self, constraint: Constraint) {
            self.scene.node_raw_mut(self.id).constraint = constraint;
            self.scene.mark_dirty(self.id, LAYOUT_DIRTY);
        }

        pub fn set_style(&mut self, style: Style) {
            self.scene.node_raw_mut(self.id).style = style;
            self.scene.mark_dirty(self.id, PAINT_DIRTY);
        }

        pub fn set_visible(&mut self, visible: bool) {
            self.scene.node_raw_mut(self.id).visible = visible;
            self.scene.mark_dirty(self.id, LAYOUT_DIRTY | PAINT_DIRTY);
        }

        pub fn widget(&mut self) -> Option<&mut WidgetState> {
            self.scene.mark_dirty(self.id, PAINT_DIRTY);
            match &mut self.scene.node_raw_mut(self.id).content {
                NodeContent::Widget(w) => Some(w),
                _ => None,
            }
        }

        pub fn chrome(&mut self) -> Option<&mut ChromeState> {
            self.scene.mark_dirty(self.id, PAINT_DIRTY);
            match &mut self.scene.node_raw_mut(self.id).content {
                NodeContent::Chrome(c) => Some(c),
                _ => None,
            }
        }

        pub fn id(&self) -> NodeId {
            self.id
        }
    }
}
// #endregion 🔖️Scene

// #region 🔖️Layout
pub mod layout {
    use crate::tui::geometry::Rect;
    use crate::tui::scene::{NodeContent, NodeId, Scene};

    /// 📐️ How one axis of a node's size is determined.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Dimension {
        Auto,
        Cells(u16),
        Weight(u16),
    }

    impl Default for Dimension {
        fn default() -> Self {
            Dimension::Auto
        }
    }

    /// ↔ How a node arranges its children.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum Direction {
        #[default]
        Row,
        Column,
        Stack,
    }

    /// 📦️ A node's layout intent.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Constraint {
        pub direction: Direction,
        pub width: Dimension,
        pub height: Dimension,
        pub gap: u16,
        pub padding: [u16; 4],
    }

    fn measure(scene: &Scene, id: NodeId) -> (u16, u16) {
        match &scene.node(id).content {
            NodeContent::Text(s) => (crate::tui::text::display_width(s), 1),
            NodeContent::Widget(w) => {
                let size = w.preferred_size();
                (size.width, size.height)
            }
            _ => (0, 0),
        }
    }

    fn distribute(dims: &[Dimension], measured: &[u16], total: u16, gap: u16) -> Vec<u16> {
        let n = dims.len();
        if n == 0 {
            return Vec::new();
        }
        let gaps = gap.saturating_mul(n.saturating_sub(1) as u16);
        let mut sizes = vec![0u16; n];
        let mut weight_total = 0u32;
        let mut fixed_total = gaps;
        for (i, d) in dims.iter().enumerate() {
            match d {
                Dimension::Cells(c) => {
                    sizes[i] = *c;
                    fixed_total += c;
                }
                Dimension::Auto => {
                    sizes[i] = measured[i];
                    fixed_total += measured[i];
                }
                Dimension::Weight(w) => weight_total += u32::from(*w),
            }
        }
        let remaining = u32::from(total).saturating_sub(u32::from(fixed_total));
        if weight_total > 0 {
            let mut remainders: Vec<(usize, u32)> = Vec::new();
            let mut used = 0u32;
            for (i, d) in dims.iter().enumerate() {
                if let Dimension::Weight(w) = d {
                    let share = remaining * u32::from(*w) / weight_total;
                    sizes[i] = share as u16;
                    used += share;
                    remainders.push((i, remaining * u32::from(*w) % weight_total));
                }
            }
            let mut leftover = remaining.saturating_sub(used);
            remainders.sort_by(|a, b| b.1.cmp(&a.1));
            for (i, _) in remainders {
                if leftover == 0 {
                    break;
                }
                sizes[i] += 1;
                leftover -= 1;
            }
        }
        sizes
    }

    /// 🧮️ Recomputes rects for the whole tree from `viewport` down (no-operation-safe to call every frame).
    pub fn solve(scene: &mut Scene, viewport: Rect) {
        layout_node(scene, scene.root(), viewport);
    }

    fn layout_node(scene: &mut Scene, id: NodeId, rect: Rect) {
        scene.node_raw_mut(id).rect = rect;
        let constraint = scene.node(id).constraint;
        let [top, right, bottom, left] = constraint.padding;
        let inner = rect.inset_sides(top, right, bottom, left);
        let children: Vec<NodeId> = scene.node(id).children().to_vec();
        if children.is_empty() {
            return;
        }
        match constraint.direction {
            Direction::Stack => {
                for &child in &children {
                    layout_node(scene, child, inner);
                }
            }
            Direction::Row => {
                let dims: Vec<Dimension> = children.iter().map(|c| scene.node(*c).constraint.width).collect();
                let measured: Vec<u16> = children.iter().map(|c| measure(scene, *c).0).collect();
                let sizes = distribute(&dims, &measured, inner.width, constraint.gap);
                let mut x = inner.x;
                for (i, &child) in children.iter().enumerate() {
                    let w = sizes[i];
                    layout_node(scene, child, Rect::new(x, inner.y, w, inner.height));
                    x += w + constraint.gap;
                }
            }
            Direction::Column => {
                let dims: Vec<Dimension> = children.iter().map(|c| scene.node(*c).constraint.height).collect();
                let measured: Vec<u16> = children.iter().map(|c| measure(scene, *c).1).collect();
                let sizes = distribute(&dims, &measured, inner.height, constraint.gap);
                let mut y = inner.y;
                for (i, &child) in children.iter().enumerate() {
                    let h = sizes[i];
                    layout_node(scene, child, Rect::new(inner.x, y, inner.width, h));
                    y += h + constraint.gap;
                }
            }
        }
    }

    //#region 🔖️WindowLayout
    /// 🪟️ One tiled window leaf: which content it hosts and its display title.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowLayoutWindowNode {
        pub window_kind_id: String,
        pub title: Option<String>,
    }

    /// 🗂️ A stack of windows sharing one area; only `active_window_kind_id` is visible.
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct WindowLayoutStackNode {
        pub size: Option<f64>,
        pub active_window_kind_id: Option<String>,
        pub children: Vec<WindowLayoutWindowNode>,
    }

    /// ↔ A row or column of tiled children.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowLayoutAxisNode {
        pub kind: String,
        pub size: Option<f64>,
        pub children: Vec<WindowLayoutChild>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum WindowLayoutChild {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum WindowLayoutRoot {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    /// 🌳️ A full tiling window arrangement (rows/columns/stacks with weights).
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowLayout {
        pub root: WindowLayoutRoot,
    }

    /// 📐️ The resolved on-screen placement of one visible window.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WindowMeasure {
        pub window_kind_id: String,
        pub rect: Rect,
        pub active: bool,
        pub stack_tabs: Vec<String>,
    }

    fn axis_child_size(child: &WindowLayoutChild) -> f64 {
        match child {
            WindowLayoutChild::Axis(a) => a.size.unwrap_or(1.0),
            WindowLayoutChild::Stack(s) => s.size.unwrap_or(1.0),
        }
    }

    fn solve_axis(node: &WindowLayoutAxisNode, area: Rect, out: &mut Vec<WindowMeasure>) {
        let is_row = node.kind == "row";
        let total_weight: f64 = node.children.iter().map(axis_child_size).sum::<f64>().max(1e-6);
        let extent = if is_row { area.width } else { area.height };
        let mut offset = 0u16;
        for child in &node.children {
            let weight = axis_child_size(child);
            let size = ((f64::from(extent) * weight / total_weight).round() as u16).min(extent - offset);
            let child_rect = if is_row { Rect::new(area.x + offset, area.y, size, area.height) } else { Rect::new(area.x, area.y + offset, area.width, size) };
            match child {
                WindowLayoutChild::Axis(a) => solve_axis(a, child_rect, out),
                WindowLayoutChild::Stack(s) => solve_stack(s, child_rect, out),
            }
            offset += size;
        }
    }

    fn solve_stack(node: &WindowLayoutStackNode, area: Rect, out: &mut Vec<WindowMeasure>) {
        if node.children.is_empty() {
            return;
        }
        let tabs: Vec<String> = node.children.iter().map(|c| c.window_kind_id.clone()).collect();
        let active = node.active_window_kind_id.clone().unwrap_or_else(|| node.children[0].window_kind_id.clone());
        out.push(WindowMeasure { window_kind_id: active, rect: area, active: true, stack_tabs: tabs });
    }

    /// 🧮️ Resolves a `WindowLayout` into concrete on-screen `WindowMeasure`s.
    pub fn solve_window_layout(layout: &WindowLayout, area: Rect) -> Vec<WindowMeasure> {
        let mut out = Vec::new();
        match &layout.root {
            WindowLayoutRoot::Axis(a) => solve_axis(a, area, &mut out),
            WindowLayoutRoot::Stack(s) => solve_stack(s, area, &mut out),
        }
        out
    }

    /// 🏗️ Builds a row/column layout of individually-sized windows.
    pub fn create_default_layout(window_ids: &[String], direction: &str, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
        let children = window_ids
            .iter()
            .enumerate()
            .map(|(i, id)| {
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    size: sizes.and_then(|s| s.get(i)).copied(),
                    active_window_kind_id: Some(id.clone()),
                    children: vec![WindowLayoutWindowNode { window_kind_id: id.clone(), title: titles.and_then(|t| t.get(i)).cloned() }],
                })
            })
            .collect();
        WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: direction.to_string(), size: None, children }) }
    }

    /// 🏗️ Builds an evenly-weighted row layout.
    pub fn even_window_layout(window_ids: &[String]) -> WindowLayout {
        create_default_layout(window_ids, "row", None, None)
    }
    //#endregion 🔖️WindowLayout
}
// #endregion 🔖️Layout

// #region 🔖️Widget

pub mod widget {
    use crate::tui::cell::CellBuffer;
    use crate::tui::chip::paint_chip;
    use crate::tui::divider::paint_divider;
    use crate::tui::event::KeyEvent;
    use crate::tui::geometry::{Rect, Size};
    use crate::tui::input::{input_on_key, paint_input};
    use crate::tui::label::paint_label;
    use crate::tui::list::{list_on_key, paint_list};
    use crate::tui::log::{log_on_key, paint_log};
    use crate::tui::select::{paint_select, select_on_key};
    use crate::tui::table::{paint_table, table_on_key};
    use crate::tui::tabs::{paint_tabs, tabs_on_key};
    use crate::tui::text::display_width;
    use crate::tui::theme::{Role, Theme};
    use std::collections::VecDeque;

    /// 📣️ A widget- or window-chrome-level result of handling input, surfaced to the app.
    #[derive(Clone, Debug, PartialEq)]
    pub enum WidgetSignal {
        Activated(usize),
        SelectionChanged(usize),
        ValueChanged(String),
        Toggled(bool),
        TabChanged(usize),
        WindowClose,
        WindowMaximize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Align {
        Left,
        Center,
        Right,
    }

    /// 🏷️ Static or dynamic single-line text.
    pub struct LabelState {
        pub text: String,
        pub align: Align,
        pub role: Role,
    }

    /// 📃️ A scrollable, selectable, optionally multi-marked list.
    pub struct ListState {
        pub items: Vec<String>,
        pub selected: usize,
        pub offset: usize,
        pub marks: Vec<bool>,
    }

    impl ListState {
        pub fn new(items: Vec<String>) -> Self {
            let marks = vec![false; items.len()];
            Self { items, selected: 0, offset: 0, marks }
        }
    }

    /// 🔁️ A cycler for an `All | Individual(value)` style option pick.
    pub struct SelectState {
        pub label: String,
        pub options: Vec<String>,
        pub index: usize,
    }

    pub struct TabsState {
        pub tabs: Vec<String>,
        pub active: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum LogScroll {
        Follow,
        At(usize),
    }

    /// 📜️ A bounded scrollback log view.
    pub struct LogState {
        lines: VecDeque<String>,
        pub capacity: usize,
        pub scroll: LogScroll,
    }

    impl LogState {
        pub fn new(capacity: usize) -> Self {
            Self { lines: VecDeque::with_capacity(capacity), capacity, scroll: LogScroll::Follow }
        }

        pub fn push(&mut self, line: &str) {
            if self.lines.len() >= self.capacity {
                self.lines.pop_front();
            }
            self.lines.push_back(line.to_string());
        }

        pub fn clear(&mut self) {
            self.lines.clear();
            self.scroll = LogScroll::Follow;
        }

        pub fn lines(&self) -> &VecDeque<String> {
            &self.lines
        }
    }

    pub struct InputState {
        pub value: String,
        pub cursor: usize,
        pub placeholder: String,
    }

    #[derive(Default)]
    pub struct DividerState {
        pub label: Option<String>,
    }

    pub struct ChipState {
        pub label: String,
        pub on: bool,
    }

    //#region 🔖️Table
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TableAlign {
        Left,
        Right,
    }

    /// 📐️ One table column; `width == 0` means "flex" — split the remaining space evenly.
    pub struct TableColumn {
        pub label: String,
        pub width: u16,
        pub align: TableAlign,
    }

    impl TableColumn {
        pub fn new(label: impl Into<String>, width: u16, align: TableAlign) -> Self {
            Self { label: label.into(), width, align }
        }
    }

    /// 🌳️ One row, flat in display order; `level` and `has_children` express the tree — a row is
    /// hidden whenever a preceding, still-nesting ancestor has `expanded == false`.
    pub struct TableRow {
        pub id: String,
        pub cells: Vec<String>,
        pub level: u16,
        pub has_children: bool,
        pub expanded: bool,
    }

    impl TableRow {
        pub fn parent(id: impl Into<String>, cells: Vec<String>) -> Self {
            Self { id: id.into(), cells, level: 0, has_children: true, expanded: true }
        }

        pub fn child(id: impl Into<String>, cells: Vec<String>, level: u16) -> Self {
            Self { id: id.into(), cells, level, has_children: false, expanded: true }
        }
    }

    /// 🗂️ A semio-styled table: bold muted header with a hairline underline, hairline row
    /// separators, no vertical rules, no striping — mirrors `ui/js/react`'s `Table` and
    /// `print/tex/🖋️semio-table.sty`. Tree rows are plain indented rows in the same table.
    pub struct TableState {
        pub columns: Vec<TableColumn>,
        pub rows: Vec<TableRow>,
        pub selected: usize,
    }

    impl TableState {
        pub fn new(columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
            Self { columns, rows, selected: 0 }
        }

        /// 👁️ Row indices in display order, skipping any row nested under a collapsed ancestor.
        pub fn visible_indices(&self) -> Vec<usize> {
            let mut out = Vec::new();
            let mut collapsed_from: Option<u16> = None;
            for (i, row) in self.rows.iter().enumerate() {
                if let Some(level) = collapsed_from {
                    if row.level > level {
                        continue;
                    }
                    collapsed_from = None;
                }
                out.push(i);
                if row.has_children && !row.expanded {
                    collapsed_from = Some(row.level);
                }
            }
            out
        }
    }
    //#endregion 🔖️Table

    /// 🧩️ The concrete state of any core widget.
    pub enum WidgetState {
        Label(LabelState),
        List(ListState),
        Select(SelectState),
        Tabs(TabsState),
        Log(LogState),
        Input(InputState),
        Divider(DividerState),
        Chip(ChipState),
        Table(TableState),
    }

    impl WidgetState {
        pub fn preferred_size(&self) -> Size {
            match self {
                WidgetState::Label(l) => Size { width: display_width(&l.text), height: 1 },
                WidgetState::Select(s) => {
                    let text = format!("{} \u{2039} {} \u{203a}", s.label, s.options.get(s.index).map(String::as_str).unwrap_or(""));
                    Size { width: display_width(&text), height: 1 }
                }
                WidgetState::Chip(c) => Size { width: display_width(&c.label) + 2, height: 1 },
                WidgetState::Divider(_) => Size { width: 1, height: 1 },
                _ => Size { width: 0, height: 0 },
            }
        }

        /// ⌨️ Handles one key press, returning a signal when it changes visible state.
        pub fn on_key(&mut self, ev: &KeyEvent) -> Option<WidgetSignal> {
            match self {
                WidgetState::List(l) => list_on_key(l, ev),
                WidgetState::Select(s) => select_on_key(s, ev),
                WidgetState::Tabs(t) => tabs_on_key(t, ev),
                WidgetState::Input(i) => input_on_key(i, ev),
                WidgetState::Table(t) => table_on_key(t, ev),
                WidgetState::Log(log) => {
                    log_on_key(log, ev);
                    None
                }
                _ => None,
            }
        }

        /// 🖌️ Paints this widget's content into `rect` of `buf`.
        pub fn paint(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
            match self {
                WidgetState::Label(l) => paint_label(l, theme, rect, buf),
                WidgetState::List(l) => paint_list(l, theme, rect, buf, focused),
                WidgetState::Select(s) => paint_select(s, theme, rect, buf, focused),
                WidgetState::Tabs(t) => paint_tabs(t, theme, rect, buf),
                WidgetState::Log(log) => paint_log(log, theme, rect, buf),
                WidgetState::Input(i) => paint_input(i, theme, rect, buf, focused),
                WidgetState::Divider(d) => paint_divider(d, theme, rect, buf),
                WidgetState::Chip(c) => paint_chip(c, theme, rect, buf),
                WidgetState::Table(t) => paint_table(t, theme, rect, buf, focused),
            }
        }
    }
}
// #endregion 🔖️Widget

// #region 🔖️Chrome

pub mod chrome {
    use crate::tui::cell::{Cell, CellBuffer};
    use crate::tui::footer::paint_footer;
    use crate::tui::geometry::{Pos, Rect};
    use crate::tui::layout::{solve_window_layout, WindowLayout};
    use crate::tui::navbar::paint_navbar;
    use crate::tui::scene::{Node, NodeContent, NodeId, Scene};
    use crate::tui::text::{display_width, truncate_to};
    use crate::tui::theme::{Role, Surface, Theme};
    use crate::tui::window::paint_window;

    #[derive(Clone)]
    pub struct NavItem {
        pub id: String,
        pub label: String,
        pub active: bool,
    }

    pub struct NavbarState {
        pub left: Vec<NavItem>,
        pub center: Vec<NavItem>,
        pub right: Vec<NavItem>,
    }

    #[derive(Clone)]
    pub struct KeyHint {
        pub key: String,
        pub label: String,
    }

    pub struct FooterState {
        pub hints: Vec<KeyHint>,
        pub status: String,
    }

    pub struct WindowState {
        pub title: String,
        pub number: Option<String>,
        pub focused: bool,
        pub closable: bool,
        pub maximizable: bool,
    }

    impl WindowState {
        pub fn new(title: impl Into<String>) -> Self {
            Self { title: title.into(), number: None, focused: false, closable: true, maximizable: true }
        }
    }

    /// 🧩️ The concrete state of any semio chrome node.
    pub enum ChromeState {
        Navbar(NavbarState),
        Footer(FooterState),
        Canvas,
        Window(WindowState),
    }

    impl ChromeState {
        /// 🖌️ Paints the chrome background/frame; window/content children paint over it.
        pub fn paint(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
            match self {
                ChromeState::Navbar(n) => paint_navbar(n, theme, rect, buf),
                ChromeState::Footer(f) => paint_footer(f, theme, rect, buf),
                ChromeState::Canvas => {
                    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), theme.surface(Surface::Base)));
                }
                ChromeState::Window(w) => paint_window(w, theme, rect, buf),
            }
        }

        /// 🖱️ Resolves a click on a window's close/maximize tab, if any (tab text-row hit only).
        pub fn window_control_at(&self, rect: Rect, pos: Pos) -> Option<crate::tui::widget::WidgetSignal> {
            let ChromeState::Window(w) = self else { return None };
            let layout = window_chip_layout(w, rect);
            if !layout.has_tabs || pos.y != rect.y + 1 {
                return None;
            }
            let controls = layout.controls?;
            let maximize_x = controls.x + 1 + WINDOW_CONTROLS_MAXIMIZE_OFFSET;
            let close_x = controls.x + 1 + WINDOW_CONTROLS_CLOSE_OFFSET;
            if pos.x == close_x && w.closable {
                Some(crate::tui::widget::WidgetSignal::WindowClose)
            } else if pos.x == maximize_x && w.maximizable {
                Some(crate::tui::widget::WidgetSignal::WindowMaximize)
            } else {
                None
            }
        }
    }

    /// 🔘️ The controls tab's interior content: enlarge glyph then close glyph, padded to one cell each.
    const WINDOW_CONTROLS_INTERIOR: &str = " \u{2922} \u{2715} ";
    const WINDOW_CONTROLS_MAXIMIZE_OFFSET: u16 = 1;
    const WINDOW_CONTROLS_CLOSE_OFFSET: u16 = 3;

    /// 🪟️ One 2-row tab recessed into a top corner of the window: `x` is its own left-wall column,
    /// `interior` is the padded text between its walls (the tab is `interior_width + 2` cells wide).
    /// `pub(crate)` (struct and fields): shared with `crate::tui::window`'s `paint_window`/`paint_corner_tab`.
    pub(crate) struct WindowTab {
        pub(crate) x: u16,
        pub(crate) interior: String,
        pub(crate) interior_width: u16,
    }

    /// 🪟️ `pub(crate)` (struct and fields): shared with `crate::tui::window`'s `paint_window`.
    pub(crate) struct WindowChipLayout {
        pub(crate) has_tabs: bool,
        pub(crate) title: WindowTab,
        pub(crate) controls: Option<WindowTab>,
    }

    /// 📐️ Shared by paint and click hit-testing so the two can never drift apart. The title tab's own
    /// left wall is the window's left wall; the controls tab's own right wall is the window's right
    /// wall — both 2 rows tall, each bending down into the main body's top edge one row below.
    /// `pub(crate)`: called from `crate::tui::window`'s `paint_window` (extracted to its own element file;
    /// this fn stays here because `ChromeState::window_control_at`, also in this mod, needs it too).
    pub(crate) fn window_chip_layout(w: &WindowState, rect: Rect) -> WindowChipLayout {
        let controls_interior_width = display_width(WINDOW_CONTROLS_INTERIOR);
        let controls_width = controls_interior_width + 2;
        let number_prefix = w.number.as_ref().map(|n| format!("{n} ")).unwrap_or_default();
        let title_interior_full = format!(" {number_prefix}{} ", w.title);
        let show_controls = w.closable || w.maximizable;
        let title_room = rect.width.saturating_sub(2 + if show_controls { controls_width + 1 } else { 0 });
        let (title_interior, title_interior_width) = truncate_to(&title_interior_full, title_room);
        let title_width = title_interior_width + 2;
        let has_tabs = rect.height >= 4 && title_width >= 3 && rect.width >= title_width + 2;
        let controls_fits = show_controls && rect.width >= title_width + controls_width + 3;
        let controls = controls_fits.then(|| WindowTab { x: rect.x + rect.width - controls_width, interior: WINDOW_CONTROLS_INTERIOR.to_string(), interior_width: controls_interior_width });
        WindowChipLayout { has_tabs, title: WindowTab { x: rect.x, interior: title_interior.to_string(), interior_width: title_interior_width }, controls }
    }

    /// 🏛️ The three fixed shell regions plus one Window node per resolved `WindowMeasure`.
    pub struct Shell {
        pub navbar: NodeId,
        pub canvas: NodeId,
        pub footer: NodeId,
        pub windows: Vec<(String, NodeId)>,
    }

    /// 🏗️ Builds navbar(top) + canvas(fill) + footer(bottom), then one Window per tiled slot.
    pub fn shell(scene: &mut Scene, navbar: NavbarState, footer: FooterState, layout: &WindowLayout) -> Shell {
        let root = scene.root();
        let navbar_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Navbar(navbar))));
        let canvas_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Canvas)));
        let footer_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Footer(footer))));
        {
            let mut root_mut = scene.node_mut(root);
            root_mut.set_constraint(crate::tui::layout::Constraint { direction: crate::tui::layout::Direction::Column, ..Default::default() });
        }
        scene.node_mut(navbar_id).set_constraint(crate::tui::layout::Constraint { height: crate::tui::layout::Dimension::Cells(2), ..Default::default() });
        scene.node_mut(canvas_id).set_constraint(crate::tui::layout::Constraint { height: crate::tui::layout::Dimension::Weight(1), direction: crate::tui::layout::Direction::Stack, ..Default::default() });
        scene.node_mut(footer_id).set_constraint(crate::tui::layout::Constraint { height: crate::tui::layout::Dimension::Cells(2), ..Default::default() });
        let mut windows = Vec::new();
        for measure in solve_window_layout(layout, Rect::default()) {
            let id = scene.add(canvas_id, Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new(measure.window_kind_id.clone())))));
            windows.push((measure.window_kind_id, id));
        }
        Shell { navbar: navbar_id, canvas: canvas_id, footer: footer_id, windows }
    }

    impl Shell {
        /// 🔁️ Re-measures `layout` against the canvas rect and repositions each window node.
        pub fn apply_layout(&self, scene: &mut Scene, layout: &WindowLayout) {
            let area = scene.rect(self.canvas);
            let measures = solve_window_layout(layout, area);
            for (kind_id, id) in &self.windows {
                if let Some(m) = measures.iter().find(|m| &m.window_kind_id == kind_id) {
                    scene.node_mut(*id).set_constraint(crate::tui::layout::Constraint { width: crate::tui::layout::Dimension::Cells(m.rect.width), height: crate::tui::layout::Dimension::Cells(m.rect.height), ..Default::default() });
                }
            }
        }
    }
}
// #endregion 🔖️Chrome

// #region 🔖️Engine
pub mod engine {
    use crate::tui::ansi::{emit_runs, AnsiPatch};
    use crate::tui::cell::{diff, Cell, CellBuffer};
    use crate::tui::event::{Event, Key, KeyEvent};
    use crate::tui::geometry::Size;
    use crate::tui::scene::{NodeContent, NodeId, Scene};
    use crate::tui::theme::Theme;
    use crate::tui::widget::WidgetSignal;
    use ui_styling::appearance::AppearanceName;

    fn focusable(scene: &Scene, id: NodeId) -> bool {
        matches!(scene.node(id).content, NodeContent::Widget(_))
    }

    fn dfs_focusables(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
        if focusable(scene, id) {
            out.push(id);
        }
        for &child in scene.node(id).children() {
            dfs_focusables(scene, child, out);
        }
    }

    /// 🖥️ The retained-mode pipeline: layout-if-dirty → paint-if-dirty → damage-diff → ANSI.
    pub struct Tui {
        pub scene: Scene,
        pub theme: Theme,
        size: Size,
        front: CellBuffer,
        back: CellBuffer,
        focus: Option<NodeId>,
        full_redraw: bool,
    }

    impl Tui {
        pub fn new(size: Size, theme: Theme) -> Self {
            let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
            Self { scene: Scene::new(), theme, size, front: CellBuffer::new(size, blank), back: CellBuffer::new(size, blank), focus: None, full_redraw: true }
        }

        /// 🔍️ The last fully-composed frame, for hosts/tests that need to inspect the actual render.
        pub fn frame(&self) -> &CellBuffer {
            &self.front
        }

        pub fn resize(&mut self, size: Size) {
            self.size = size;
            let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
            self.front.resize(size, blank);
            self.back.resize(size, blank);
            self.full_redraw = true;
        }

        pub fn set_appearance(&mut self, appearance: AppearanceName) {
            self.theme.set_appearance(appearance);
            self.full_redraw = true;
        }

        pub fn focus(&self) -> Option<NodeId> {
            self.focus
        }

        pub fn set_focus(&mut self, id: Option<NodeId>) {
            self.focus = id;
        }

        pub fn focus_next(&mut self) {
            let mut list = Vec::new();
            dfs_focusables(&self.scene, self.scene.root(), &mut list);
            if list.is_empty() {
                return;
            }
            let next = match self.focus.and_then(|f| list.iter().position(|x| *x == f)) {
                Some(i) => (i + 1) % list.len(),
                None => 0,
            };
            self.focus = Some(list[next]);
        }

        pub fn focus_prev(&mut self) {
            let mut list = Vec::new();
            dfs_focusables(&self.scene, self.scene.root(), &mut list);
            if list.is_empty() {
                return;
            }
            let prev = match self.focus.and_then(|f| list.iter().position(|x| *x == f)) {
                Some(i) => (i + list.len() - 1) % list.len(),
                None => list.len() - 1,
            };
            self.focus = Some(list[prev]);
        }

        /// 📡️ Routes one input event to the focused widget (keys) or the hit node (mouse).
        pub fn dispatch(&mut self, event: &Event) -> Vec<(NodeId, WidgetSignal)> {
            let mut signals = Vec::new();
            match event {
                Event::Resize(size) => self.resize(*size),
                Event::Key(KeyEvent { key: Key::Tab, .. }) => self.focus_next(),
                Event::Key(KeyEvent { key: Key::BackTab, .. }) => self.focus_prev(),
                Event::Key(key_ev) => {
                    if let Some(id) = self.focus {
                        if let Some(widget) = self.scene.node_mut(id).widget() {
                            if let Some(signal) = widget.on_key(key_ev) {
                                signals.push((id, signal));
                            }
                        }
                    }
                }
                Event::Mouse(m) => {
                    if let Some(id) = self.scene.hit(m.pos) {
                        self.focus = Some(id);
                        if matches!(m.kind, crate::tui::event::MouseKind::Down(_)) {
                            let rect = self.scene.node(id).rect;
                            if let NodeContent::Chrome(chrome) = &self.scene.node(id).content {
                                if let Some(signal) = chrome.window_control_at(rect, m.pos) {
                                    signals.push((id, signal));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            signals
        }

        fn paint(&mut self) {
            fn walk(scene: &Scene, theme: &Theme, focus: Option<NodeId>, id: NodeId, buf: &mut CellBuffer) {
                let node = scene.node(id);
                if !node.visible {
                    return;
                }
                let rect = node.rect;
                match &node.content {
                    NodeContent::Chrome(c) => c.paint(theme, rect, buf),
                    NodeContent::Widget(w) => w.paint(theme, rect, buf, Some(id) == focus),
                    NodeContent::Text(s) => {
                        let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(crate::tui::theme::Surface::Base));
                        buf.put_str(crate::tui::geometry::Pos { x: rect.x, y: rect.y }, s, theme.role(crate::tui::theme::Role::Foreground), bg, 0, rect);
                    }
                    NodeContent::Box => {}
                }
                for &child in node.children() {
                    walk(scene, theme, focus, child, buf);
                }
            }
            walk(&self.scene, &self.theme, self.focus, self.scene.root(), &mut self.back);
        }

        /// 🎬️ Solves layout if dirty, repaints, diffs against the last frame, and emits a patch.
        pub fn render(&mut self) -> AnsiPatch {
            let root_dirty = self.scene.take_dirty(self.scene.root()) != 0;
            if !root_dirty && !self.full_redraw {
                return AnsiPatch::default();
            }
            crate::tui::layout::solve(&mut self.scene, crate::tui::geometry::Rect::new(0, 0, self.size.width, self.size.height));
            self.paint();
            let mut patch = AnsiPatch::default();
            if self.full_redraw {
                let full = vec![crate::tui::cell::DiffRun { y: 0, x: 0, len: self.size.width }; usize::from(self.size.height)]
                    .into_iter()
                    .enumerate()
                    .map(|(y, mut r)| {
                        r.y = y as u16;
                        r
                    })
                    .collect::<Vec<_>>();
                emit_runs(&self.back, &full, &mut patch);
                self.full_redraw = false;
            } else {
                let runs = diff(&self.front, &self.back);
                emit_runs(&self.back, &runs, &mut patch);
            }
            self.front = self.back.clone();
            patch
        }

        /// 🎬️ Forces a full-frame repaint regardless of dirty state.
        pub fn render_full(&mut self) -> AnsiPatch {
            self.full_redraw = true;
            self.render()
        }
    }
}
// #endregion 🔖️Engine

// #region 🔖️Backend
pub mod backend {
    use crate::tui::ansi::AnsiPatch;
    use crate::tui::event::Event;
    use crate::tui::geometry::Size;
    use std::time::Duration;

    #[derive(Debug)]
    pub struct BackendError {
        pub message: String,
    }

    impl std::fmt::Display for BackendError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    /// 🔌️ A platform terminal I/O implementation, kept out of the retained-mode core.
    pub trait TerminalBackend {
        fn size(&mut self) -> Result<Size, BackendError>;
        fn enter(&mut self) -> Result<(), BackendError>;
        fn leave(&mut self) -> Result<(), BackendError>;
        fn present(&mut self, patch: &AnsiPatch) -> Result<(), BackendError>;
        fn poll(&mut self, timeout: Duration) -> Result<Vec<Event>, BackendError>;
    }

    #[cfg(all(feature = "tui-terminal", unix, not(target_arch = "wasm32")))]
    mod native_unix {
        use super::*;
        use crate::tui::ansi::{setup_sequence, teardown_sequence, AnsiParser};
        use std::io::Write;
        use std::os::unix::io::RawFd;

        fn err(message: impl Into<String>) -> BackendError {
            BackendError { message: message.into() }
        }

        /// 🖥️ Raw-mode terminal backend for unix (macOS/Linux), driven by `libc` alone.
        pub struct NativeTerminal {
            fd: RawFd,
            original: libc::termios,
            parser: AnsiParser,
            entered: bool,
        }

        impl NativeTerminal {
            pub fn new() -> Result<Self, BackendError> {
                let fd = libc::STDIN_FILENO;
                let original = unsafe {
                    let mut t: libc::termios = std::mem::zeroed();
                    if libc::tcgetattr(fd, &mut t) != 0 {
                        return Err(err("tcgetattr failed"));
                    }
                    t
                };
                Ok(Self { fd, original, parser: AnsiParser::new(), entered: false })
            }
        }

        impl TerminalBackend for NativeTerminal {
            fn size(&mut self) -> Result<Size, BackendError> {
                unsafe {
                    let mut ws: libc::winsize = std::mem::zeroed();
                    if libc::ioctl(self.fd, libc::TIOCGWINSZ, &mut ws) != 0 {
                        return Err(err("TIOCGWINSZ failed"));
                    }
                    Ok(Size { width: ws.ws_col, height: ws.ws_row })
                }
            }

            fn enter(&mut self) -> Result<(), BackendError> {
                let mut raw = self.original;
                raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG | libc::IEXTEN);
                raw.c_iflag &= !(libc::IXON | libc::ICRNL | libc::BRKINT | libc::INPCK | libc::ISTRIP);
                raw.c_oflag &= !libc::OPOST;
                raw.c_cc[libc::VMIN] = 0;
                raw.c_cc[libc::VTIME] = 0;
                unsafe {
                    if libc::tcsetattr(self.fd, libc::TCSANOW, &raw) != 0 {
                        return Err(err("tcsetattr failed"));
                    }
                }
                self.entered = true;
                std::io::stdout().write_all(setup_sequence().as_bytes()).map_err(|e| err(e.to_string()))?;
                std::io::stdout().flush().map_err(|e| err(e.to_string()))
            }

            fn leave(&mut self) -> Result<(), BackendError> {
                if !self.entered {
                    return Ok(());
                }
                std::io::stdout().write_all(teardown_sequence().as_bytes()).map_err(|e| err(e.to_string()))?;
                std::io::stdout().flush().map_err(|e| err(e.to_string()))?;
                unsafe {
                    libc::tcsetattr(self.fd, libc::TCSANOW, &self.original);
                }
                self.entered = false;
                Ok(())
            }

            fn present(&mut self, patch: &AnsiPatch) -> Result<(), BackendError> {
                std::io::stdout().write_all(patch.0.as_bytes()).map_err(|e| err(e.to_string()))?;
                std::io::stdout().flush().map_err(|e| err(e.to_string()))
            }

            fn poll(&mut self, timeout: Duration) -> Result<Vec<Event>, BackendError> {
                let mut pfd = libc::pollfd { fd: self.fd, events: libc::POLLIN, revents: 0 };
                let ready = unsafe { libc::poll(&mut pfd, 1, timeout.as_millis() as i32) };
                let mut events = Vec::new();
                if ready > 0 && pfd.revents & libc::POLLIN != 0 {
                    let mut buf = [0u8; 4096];
                    let n = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut _, buf.len()) };
                    if n > 0 {
                        self.parser.feed(&buf[..n as usize], &mut events);
                    }
                } else {
                    self.parser.flush_escape(&mut events);
                }
                Ok(events)
            }
        }

        impl Drop for NativeTerminal {
            fn drop(&mut self) {
                let _ = self.leave();
            }
        }
    }
    #[cfg(all(feature = "tui-terminal", unix, not(target_arch = "wasm32")))]
    pub use native_unix::NativeTerminal;

    #[cfg(all(feature = "tui-terminal", windows))]
    mod native_windows {
        use super::*;
        use crate::tui::ansi::{setup_sequence, teardown_sequence, AnsiParser};
        use windows_sys::Win32::Foundation::HANDLE;
        use windows_sys::Win32::Storage::FileSystem::{ReadFile, WriteFile};
        use windows_sys::Win32::System::Console::{
            GetConsoleMode, GetConsoleScreenBufferInfo, GetStdHandle, SetConsoleMode, CONSOLE_SCREEN_BUFFER_INFO, DISABLE_NEWLINE_AUTO_RETURN, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_VIRTUAL_TERMINAL_INPUT,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
        };
        use windows_sys::Win32::System::Threading::WaitForSingleObject;

        fn err(message: impl Into<String>) -> BackendError {
            BackendError { message: message.into() }
        }

        /// 🖥️ VT-mode terminal backend for Windows consoles, driven by `windows-sys` alone.
        pub struct NativeTerminal {
            stdin: HANDLE,
            stdout: HANDLE,
            original_in: u32,
            original_out: u32,
            parser: AnsiParser,
            entered: bool,
        }

        impl NativeTerminal {
            pub fn new() -> Result<Self, BackendError> {
                unsafe {
                    let stdin = GetStdHandle(STD_INPUT_HANDLE);
                    let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
                    let mut original_in = 0u32;
                    let mut original_out = 0u32;
                    if GetConsoleMode(stdin, &mut original_in) == 0 || GetConsoleMode(stdout, &mut original_out) == 0 {
                        return Err(err("GetConsoleMode failed"));
                    }
                    Ok(Self { stdin, stdout, original_in, original_out, parser: AnsiParser::new(), entered: false })
                }
            }
        }

        impl TerminalBackend for NativeTerminal {
            fn size(&mut self) -> Result<Size, BackendError> {
                unsafe {
                    let mut info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
                    if GetConsoleScreenBufferInfo(self.stdout, &mut info) == 0 {
                        return Err(err("GetConsoleScreenBufferInfo failed"));
                    }
                    let width = (info.srWindow.Right - info.srWindow.Left + 1).max(0) as u16;
                    let height = (info.srWindow.Bottom - info.srWindow.Top + 1).max(0) as u16;
                    Ok(Size { width, height })
                }
            }

            fn enter(&mut self) -> Result<(), BackendError> {
                unsafe {
                    let out_mode = self.original_out | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;
                    let in_mode = (self.original_in | ENABLE_VIRTUAL_TERMINAL_INPUT) & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);
                    if SetConsoleMode(self.stdout, out_mode) == 0 || SetConsoleMode(self.stdin, in_mode) == 0 {
                        return Err(err("SetConsoleMode failed"));
                    }
                }
                self.entered = true;
                self.write_raw(setup_sequence().as_bytes())
            }

            fn leave(&mut self) -> Result<(), BackendError> {
                if !self.entered {
                    return Ok(());
                }
                self.write_raw(teardown_sequence().as_bytes())?;
                unsafe {
                    SetConsoleMode(self.stdin, self.original_in);
                    SetConsoleMode(self.stdout, self.original_out);
                }
                self.entered = false;
                Ok(())
            }

            fn present(&mut self, patch: &AnsiPatch) -> Result<(), BackendError> {
                self.write_raw(patch.0.as_bytes())
            }

            fn poll(&mut self, timeout: Duration) -> Result<Vec<Event>, BackendError> {
                let mut events = Vec::new();
                let wait = unsafe { WaitForSingleObject(self.stdin, timeout.as_millis() as u32) };
                if wait == 0 {
                    let mut buf = [0u8; 4096];
                    let mut read = 0u32;
                    unsafe {
                        if ReadFile(self.stdin, buf.as_mut_ptr(), buf.len() as u32, &mut read, std::ptr::null_mut()) != 0 && read > 0 {
                            self.parser.feed(&buf[..read as usize], &mut events);
                        }
                    }
                } else {
                    self.parser.flush_escape(&mut events);
                }
                Ok(events)
            }
        }

        impl NativeTerminal {
            fn write_raw(&self, bytes: &[u8]) -> Result<(), BackendError> {
                let mut written = 0u32;
                unsafe {
                    if WriteFile(self.stdout, bytes.as_ptr(), bytes.len() as u32, &mut written, std::ptr::null_mut()) == 0 {
                        return Err(err("WriteFile failed"));
                    }
                }
                Ok(())
            }
        }

        impl Drop for NativeTerminal {
            fn drop(&mut self) {
                let _ = self.leave();
            }
        }
    }
    #[cfg(all(feature = "tui-terminal", windows))]
    pub use native_windows::NativeTerminal;
}
// #endregion 🔖️Backend

// #region 🔖️Pty
/// 🧵 Pseudo-terminal child process spawn and byte I/O for the native TUI host.
#[cfg(feature = "tui-terminal")]
pub mod pty {
    use std::io::Write;
    use std::path::Path;

    /// 📐 Pseudo-terminal geometry in character cells.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PtySize {
        pub cols: u16,
        pub rows: u16,
    }

    /// 💥 Failure from pseudo-terminal spawn or I/O.
    #[derive(Debug)]
    pub struct PtyError {
        pub message: String,
    }

    impl std::fmt::Display for PtyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for PtyError {}

    fn err(message: impl Into<String>) -> PtyError {
        PtyError {
            message: message.into(),
        }
    }

    #[cfg(all(unix, not(target_arch = "wasm32")))]
    mod unix_impl {
        use super::*;
        use std::fs::File;
        use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
        use std::os::unix::process::CommandExt;
        use std::process::{Child, Command};

        /// 🧵 Unix PTY master plus child process.
        pub struct Pty {
            master: File,
            child: Child,
        }

        impl Pty {
            pub fn spawn(
                cmd: &str,
                args: &[&str],
                env: &[(&str, &str)],
                cwd: Option<&Path>,
                size: PtySize,
            ) -> Result<Self, PtyError> {
                let mut master: RawFd = -1;
                let mut slave: RawFd = -1;
                let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
                ws.ws_col = size.cols;
                ws.ws_row = size.rows;
                unsafe {
                    if libc::openpty(
                        &mut master,
                        &mut slave,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &ws,
                    ) != 0
                    {
                        return Err(err(format!(
                            "openpty failed: {}",
                            std::io::Error::last_os_error()
                        )));
                    }
                }

                let flags = unsafe { libc::fcntl(master, libc::F_GETFL) };
                if flags < 0
                    || unsafe { libc::fcntl(master, libc::F_SETFL, flags | libc::O_NONBLOCK) } != 0
                {
                    unsafe {
                        libc::close(master);
                        libc::close(slave);
                    }
                    return Err(err("fcntl O_NONBLOCK failed"));
                }

                let mut command = Command::new(cmd);
                command.args(args);
                for (k, v) in env {
                    command.env(k, v);
                }
                if let Some(dir) = cwd {
                    command.current_dir(dir);
                }
                command.stdin(std::process::Stdio::null());
                command.stdout(std::process::Stdio::null());
                command.stderr(std::process::Stdio::null());
                unsafe {
                    command.pre_exec(move || {
                        if libc::setsid() < 0 {
                            return Err(std::io::Error::last_os_error());
                        }
                        if libc::ioctl(slave, libc::TIOCSCTTY as _, 0) < 0 {
                            return Err(std::io::Error::last_os_error());
                        }
                        if libc::dup2(slave, libc::STDIN_FILENO) < 0
                            || libc::dup2(slave, libc::STDOUT_FILENO) < 0
                            || libc::dup2(slave, libc::STDERR_FILENO) < 0
                        {
                            return Err(std::io::Error::last_os_error());
                        }
                        if slave > libc::STDERR_FILENO {
                            libc::close(slave);
                        }
                        if master > libc::STDERR_FILENO {
                            libc::close(master);
                        }
                        Ok(())
                    });
                }

                let child = match command.spawn() {
                    Ok(child) => child,
                    Err(e) => {
                        unsafe {
                            libc::close(master);
                            libc::close(slave);
                        }
                        return Err(err(format!("spawn failed: {e}")));
                    }
                };
                unsafe {
                    libc::close(slave);
                }
                let master = unsafe { File::from_raw_fd(master) };
                Ok(Self { master, child })
            }

            pub fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
                let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
                ws.ws_col = size.cols;
                ws.ws_row = size.rows;
                unsafe {
                    if libc::ioctl(self.master.as_raw_fd(), libc::TIOCSWINSZ, &ws) != 0 {
                        return Err(err(format!(
                            "TIOCSWINSZ failed: {}",
                            std::io::Error::last_os_error()
                        )));
                    }
                }
                Ok(())
            }

            pub fn writer(&mut self) -> &mut impl Write {
                self
            }

            pub fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, PtyError> {
                let n = unsafe {
                    libc::read(
                        self.master.as_raw_fd(),
                        buf.as_mut_ptr() as *mut _,
                        buf.len(),
                    )
                };
                if n < 0 {
                    let e = std::io::Error::last_os_error();
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        return Ok(0);
                    }
                    return Err(err(format!("read failed: {e}")));
                }
                Ok(n as usize)
            }

            pub fn write_all(&mut self, data: &[u8]) -> Result<(), PtyError> {
                Write::write_all(self, data).map_err(|e| err(e.to_string()))
            }

            pub fn try_wait(&mut self) -> Result<Option<i32>, PtyError> {
                match self.child.try_wait() {
                    Ok(Some(status)) => Ok(Some(status.code().unwrap_or(-1))),
                    Ok(None) => Ok(None),
                    Err(e) => Err(err(format!("try_wait failed: {e}"))),
                }
            }

            pub fn kill(&mut self) -> Result<(), PtyError> {
                self.child.kill().map_err(|e| err(format!("kill failed: {e}")))?;
                let _ = self.child.wait();
                Ok(())
            }
        }

        impl Write for Pty {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                let n = unsafe {
                    libc::write(
                        self.master.as_raw_fd(),
                        buf.as_ptr() as *const _,
                        buf.len(),
                    )
                };
                if n < 0 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(n as usize)
                }
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        impl Drop for Pty {
            fn drop(&mut self) {
                let _ = self.child.kill();
                let _ = self.child.wait();
            }
        }
    }

    #[cfg(all(unix, not(target_arch = "wasm32")))]
    pub use unix_impl::Pty;

    #[cfg(windows)]
    mod windows_impl {
        use super::*;
        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;
        use windows_sys::Win32::Foundation::{
            CloseHandle, SetHandleInformation, HANDLE, HANDLE_FLAG_INHERIT, INVALID_HANDLE_VALUE,
            STILL_ACTIVE, WAIT_OBJECT_0, WAIT_TIMEOUT,
        };
        use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
        use windows_sys::Win32::Storage::FileSystem::{ReadFile, WriteFile};
        use windows_sys::Win32::System::Console::{
            ClosePseudoConsole, CreatePseudoConsole, ResizePseudoConsole, COORD, HPCON,
        };
        use windows_sys::Win32::System::Pipes::{CreatePipe, PeekNamedPipe};
        use windows_sys::Win32::System::Threading::{
            CreateProcessW, DeleteProcThreadAttributeList, GetExitCodeProcess,
            InitializeProcThreadAttributeList, TerminateProcess, UpdateProcThreadAttribute,
            WaitForSingleObject, CREATE_UNICODE_ENVIRONMENT, EXTENDED_STARTUPINFO_PRESENT,
            PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, PROCESS_INFORMATION, STARTUPINFOEXW,
        };

        /// 🧵 Windows ConPTY master pipes plus child process.
        pub struct Pty {
            hpcon: HPCON,
            input_write: HANDLE,
            output_read: HANDLE,
            process: HANDLE,
            thread: HANDLE,
            closed: bool,
        }

        fn close_handle(handle: HANDLE) {
            if handle != 0 && handle != INVALID_HANDLE_VALUE {
                unsafe {
                    CloseHandle(handle);
                }
            }
        }

        fn to_wide(s: &str) -> Vec<u16> {
            OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
        }

        fn build_cmdline(cmd: &str, args: &[&str]) -> Vec<u16> {
            let mut line = String::new();
            line.push('"');
            line.push_str(cmd);
            line.push('"');
            for arg in args {
                line.push(' ');
                if arg.chars().any(|c| c.is_whitespace()) {
                    line.push('"');
                    line.push_str(arg);
                    line.push('"');
                } else {
                    line.push_str(arg);
                }
            }
            to_wide(&line)
        }

        fn build_env_block(env: &[(&str, &str)]) -> Option<Vec<u16>> {
            if env.is_empty() {
                return None;
            }
            let mut block = Vec::new();
            for (k, v) in env {
                block.extend(OsStr::new(k).encode_wide());
                block.push(b'=' as u16);
                block.extend(OsStr::new(v).encode_wide());
                block.push(0);
            }
            block.push(0);
            Some(block)
        }

        impl Pty {
            pub fn spawn(
                cmd: &str,
                args: &[&str],
                env: &[(&str, &str)],
                cwd: Option<&Path>,
                size: PtySize,
            ) -> Result<Self, PtyError> {
                unsafe {
                    let mut sa: SECURITY_ATTRIBUTES = std::mem::zeroed();
                    sa.nLength = std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32;
                    sa.bInheritHandle = 1;

                    let mut input_read = INVALID_HANDLE_VALUE;
                    let mut input_write = INVALID_HANDLE_VALUE;
                    let mut output_read = INVALID_HANDLE_VALUE;
                    let mut output_write = INVALID_HANDLE_VALUE;
                    if CreatePipe(&mut input_read, &mut input_write, &sa, 0) == 0 {
                        return Err(err("CreatePipe input failed"));
                    }
                    if CreatePipe(&mut output_read, &mut output_write, &sa, 0) == 0 {
                        close_handle(input_read);
                        close_handle(input_write);
                        return Err(err("CreatePipe output failed"));
                    }
                    SetHandleInformation(input_write, HANDLE_FLAG_INHERIT, 0);
                    SetHandleInformation(output_read, HANDLE_FLAG_INHERIT, 0);

                    let coord = COORD {
                        X: size.cols as i16,
                        Y: size.rows as i16,
                    };
                    let mut hpcon: HPCON = 0;
                    let hr = CreatePseudoConsole(coord, input_read, output_write, 0, &mut hpcon);
                    close_handle(input_read);
                    close_handle(output_write);
                    if hr < 0 || hpcon == 0 {
                        close_handle(input_write);
                        close_handle(output_read);
                        return Err(err(format!("CreatePseudoConsole failed: HRESULT {hr}")));
                    }

                    let mut attr_size = 0usize;
                    InitializeProcThreadAttributeList(std::ptr::null_mut(), 1, 0, &mut attr_size);
                    let mut attr_buf = vec![0u8; attr_size];
                    let attr_list = attr_buf.as_mut_ptr() as _;
                    if InitializeProcThreadAttributeList(attr_list, 1, 0, &mut attr_size) == 0 {
                        ClosePseudoConsole(hpcon);
                        close_handle(input_write);
                        close_handle(output_read);
                        return Err(err("InitializeProcThreadAttributeList failed"));
                    }
                    if UpdateProcThreadAttribute(
                        attr_list,
                        0,
                        PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE as usize,
                        &hpcon as *const _ as *const _,
                        std::mem::size_of::<HPCON>(),
                        std::ptr::null_mut(),
                        std::ptr::null(),
                    ) == 0
                    {
                        DeleteProcThreadAttributeList(attr_list);
                        ClosePseudoConsole(hpcon);
                        close_handle(input_write);
                        close_handle(output_read);
                        return Err(err("UpdateProcThreadAttribute failed"));
                    }

                    let mut si: STARTUPINFOEXW = std::mem::zeroed();
                    si.StartupInfo.cb = std::mem::size_of::<STARTUPINFOEXW>() as u32;
                    si.lpAttributeList = attr_list;

                    let mut cmdline = build_cmdline(cmd, args);
                    let cwd_wide = cwd.map(|p| to_wide(&p.to_string_lossy()));
                    let env_block = build_env_block(env);
                    let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
                    let mut flags = EXTENDED_STARTUPINFO_PRESENT;
                    if env_block.is_some() {
                        flags |= CREATE_UNICODE_ENVIRONMENT;
                    }
                    let ok = CreateProcessW(
                        std::ptr::null(),
                        cmdline.as_mut_ptr(),
                        std::ptr::null(),
                        std::ptr::null(),
                        0,
                        flags,
                        env_block
                            .as_ref()
                            .map(|b| b.as_ptr() as *const _)
                            .unwrap_or(std::ptr::null()),
                        cwd_wide
                            .as_ref()
                            .map(|b| b.as_ptr())
                            .unwrap_or(std::ptr::null()),
                        &si.StartupInfo,
                        &mut pi,
                    );
                    DeleteProcThreadAttributeList(attr_list);
                    if ok == 0 {
                        ClosePseudoConsole(hpcon);
                        close_handle(input_write);
                        close_handle(output_read);
                        return Err(err(format!(
                            "CreateProcessW failed: {}",
                            std::io::Error::last_os_error()
                        )));
                    }

                    Ok(Self {
                        hpcon,
                        input_write,
                        output_read,
                        process: pi.hProcess,
                        thread: pi.hThread,
                        closed: false,
                    })
                }
            }

            pub fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
                let coord = COORD {
                    X: size.cols as i16,
                    Y: size.rows as i16,
                };
                let hr = unsafe { ResizePseudoConsole(self.hpcon, coord) };
                if hr < 0 {
                    return Err(err(format!("ResizePseudoConsole failed: HRESULT {hr}")));
                }
                Ok(())
            }

            pub fn writer(&mut self) -> &mut impl Write {
                self
            }

            pub fn try_read(&mut self, buf: &mut [u8]) -> Result<usize, PtyError> {
                unsafe {
                    let mut available = 0u32;
                    if PeekNamedPipe(
                        self.output_read,
                        std::ptr::null_mut(),
                        0,
                        std::ptr::null_mut(),
                        &mut available,
                        std::ptr::null_mut(),
                    ) == 0
                    {
                        return Err(err(format!(
                            "PeekNamedPipe failed: {}",
                            std::io::Error::last_os_error()
                        )));
                    }
                    if available == 0 {
                        return Ok(0);
                    }
                    let to_read = (buf.len() as u32).min(available);
                    let mut read = 0u32;
                    if ReadFile(
                        self.output_read,
                        buf.as_mut_ptr(),
                        to_read,
                        &mut read,
                        std::ptr::null_mut(),
                    ) == 0
                    {
                        return Err(err(format!(
                            "ReadFile failed: {}",
                            std::io::Error::last_os_error()
                        )));
                    }
                    Ok(read as usize)
                }
            }

            pub fn write_all(&mut self, data: &[u8]) -> Result<(), PtyError> {
                Write::write_all(self, data).map_err(|e| err(e.to_string()))
            }

            pub fn try_wait(&mut self) -> Result<Option<i32>, PtyError> {
                unsafe {
                    let wait = WaitForSingleObject(self.process, 0);
                    if wait == WAIT_TIMEOUT {
                        return Ok(None);
                    }
                    if wait != WAIT_OBJECT_0 {
                        return Err(err("WaitForSingleObject failed"));
                    }
                    let mut code = 0u32;
                    if GetExitCodeProcess(self.process, &mut code) == 0 {
                        return Err(err("GetExitCodeProcess failed"));
                    }
                    if code == STILL_ACTIVE as u32 {
                        return Ok(None);
                    }
                    Ok(Some(code as i32))
                }
            }

            pub fn kill(&mut self) -> Result<(), PtyError> {
                unsafe {
                    if TerminateProcess(self.process, 1) == 0 {
                        return Err(err(format!(
                            "TerminateProcess failed: {}",
                            std::io::Error::last_os_error()
                        )));
                    }
                    WaitForSingleObject(self.process, 5000);
                }
                Ok(())
            }

            fn close_conpty(&mut self) {
                if self.closed {
                    return;
                }
                self.closed = true;
                unsafe {
                    ClosePseudoConsole(self.hpcon);
                }
                close_handle(self.input_write);
                close_handle(self.output_read);
                close_handle(self.thread);
                close_handle(self.process);
            }
        }

        impl Write for Pty {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                let mut written = 0u32;
                let ok = unsafe {
                    WriteFile(
                        self.input_write,
                        buf.as_ptr(),
                        buf.len() as u32,
                        &mut written,
                        std::ptr::null_mut(),
                    )
                };
                if ok == 0 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(written as usize)
                }
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        impl Drop for Pty {
            fn drop(&mut self) {
                let _ = self.kill();
                self.close_conpty();
            }
        }
    }

    #[cfg(windows)]
    pub use windows_impl::Pty;
}
// #endregion 🔖️Pty

// #region 🔖️WasmHost
pub mod host {
    use crate::tui::ansi::{setup_sequence, teardown_sequence, AnsiParser};
    use crate::tui::engine::Tui;
    use crate::tui::event::Event;
    use crate::tui::geometry::Size;
    use crate::tui::theme::Theme;
    use ui_styling::appearance::AppearanceName;

    /// 🌐️ A pure bytes-in/string-out host: feed terminal input, get an ANSI patch back.
    pub struct WasmHost {
        pub tui: Tui,
        parser: AnsiParser,
    }

    impl WasmHost {
        pub fn new(width: u16, height: u16, dark: bool) -> Self {
            let appearance = if dark { AppearanceName::Dark } else { AppearanceName::Light };
            Self { tui: Tui::new(Size { width, height }, Theme::new(appearance)), parser: AnsiParser::new() }
        }

        pub fn feed(&mut self, bytes: &[u8]) -> Vec<Event> {
            let mut events = Vec::new();
            self.parser.feed(bytes, &mut events);
            for event in &events {
                self.tui.dispatch(event);
            }
            events
        }

        pub fn resize(&mut self, width: u16, height: u16) {
            self.tui.dispatch(&Event::Resize(Size { width, height }));
        }

        pub fn render(&mut self) -> String {
            self.tui.render().0
        }

        pub fn setup(&self) -> String {
            setup_sequence().to_string()
        }

        pub fn teardown(&self) -> String {
            teardown_sequence().to_string()
        }
    }

    #[cfg(all(target_arch = "wasm32", feature = "tui-bindgen"))]
    mod bindgen_host {
        use super::WasmHost;
        use wasm_bindgen::prelude::*;

        /// 🌐️ The `wasm-bindgen` surface for browser hosts (e.g. an xterm.js terminal).
        #[wasm_bindgen]
        pub struct TuiHost(WasmHost);

        #[wasm_bindgen]
        impl TuiHost {
            #[wasm_bindgen(constructor)]
            pub fn new(width: u16, height: u16, dark: bool) -> TuiHost {
                TuiHost(WasmHost::new(width, height, dark))
            }

            pub fn feed(&mut self, bytes: &[u8]) {
                self.0.feed(bytes);
            }

            pub fn resize(&mut self, width: u16, height: u16) {
                self.0.resize(width, height);
            }

            pub fn render(&mut self) -> String {
                self.0.render()
            }

            pub fn setup(&self) -> String {
                self.0.setup()
            }

            pub fn teardown(&self) -> String {
                self.0.teardown()
            }
        }
    }
    #[cfg(all(target_arch = "wasm32", feature = "tui-bindgen"))]
    pub use bindgen_host::TuiHost;
}
// #endregion 🔖️WasmHost

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use crate::tui::ansi::{emit_runs, setup_sequence, teardown_sequence, AnsiParser, AnsiPatch};
    use crate::tui::cell::{attr, diff, Cell, CellBuffer, DiffRun};
    use crate::tui::chrome::{shell, ChromeState, FooterState, KeyHint, NavItem, NavbarState, WindowState};
    use crate::tui::event::{Event, Key, KeyEvent, MouseEvent, MouseKind};
    use crate::tui::geometry::{Pos, Rect, Size};
    use crate::tui::layout::{create_default_layout, even_window_layout, solve, solve_window_layout, Constraint, Dimension, Direction, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};
    use crate::tui::scene::{Node, NodeContent, Scene};
    use crate::tui::text::{display_width, truncate_to};
    use crate::tui::theme::{Role, Surface, Theme};
    use crate::tui::widget::{Align, ChipState, DividerState, InputState, LabelState, ListState, LogScroll, LogState, SelectState, TableAlign, TableColumn, TableRow, TableState, TabsState, WidgetSignal, WidgetState};
    use ui_styling::appearance::AppearanceName;

    fn row_text(buf: &CellBuffer, y: u16) -> String {
        (0..buf.size.width).filter_map(|x| buf.get(x, y)).map(|c| c.ch).filter(|&c| c != '\0').collect()
    }

    #[test]
    fn layout_row_weights_fill_exactly() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
        let a = scene.add(root, Node::new(NodeContent::Box));
        let b = scene.add(root, Node::new(NodeContent::Box));
        scene.node_mut(a).set_constraint(Constraint { width: Dimension::Weight(1), ..Default::default() });
        scene.node_mut(b).set_constraint(Constraint { width: Dimension::Weight(2), ..Default::default() });
        solve(&mut scene, Rect::new(0, 0, 30, 10));
        assert_eq!(scene.rect(a).width + scene.rect(b).width, 30);
        assert!(scene.rect(b).width > scene.rect(a).width);
    }

    #[test]
    fn layout_auto_measures_text_width() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
        let text = scene.add(root, Node::new(NodeContent::Text("hi".into())));
        solve(&mut scene, Rect::new(0, 0, 30, 10));
        assert_eq!(scene.rect(text).width, 2);
    }

    #[test]
    fn layout_padding_and_gap() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, gap: 1, padding: [1, 1, 1, 1], ..Default::default() });
        let a = scene.add(root, Node::new(NodeContent::Box));
        let b = scene.add(root, Node::new(NodeContent::Box));
        scene.node_mut(a).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
        scene.node_mut(b).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
        solve(&mut scene, Rect::new(0, 0, 20, 10));
        assert_eq!(scene.rect(a).x, 1);
        assert_eq!(scene.rect(b).x, 4);
        assert_eq!(scene.rect(root).height, 10);
        assert_eq!(scene.rect(a).height, 8);
    }

    #[test]
    fn window_layout_row_of_stacks_tiles_without_gaps() {
        let layout = create_default_layout(&["a".to_string(), "b".to_string()], "row", Some(&[1.0, 1.0]), None);
        let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
        assert_eq!(measures.len(), 2);
        assert_eq!(measures[0].rect.width + measures[1].rect.width, 100);
        assert_eq!(measures[0].rect.x, 0);
        assert_eq!(measures[1].rect.x, measures[0].rect.width);
    }

    #[test]
    fn window_layout_stack_exposes_tabs() {
        let layout = WindowLayout {
            root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
                size: None,
                active_window_kind_id: Some("b".into()),
                children: vec![WindowLayoutWindowNode { window_kind_id: "a".into(), title: None }, WindowLayoutWindowNode { window_kind_id: "b".into(), title: None }],
            }),
        };
        let measures = solve_window_layout(&layout, Rect::new(0, 0, 40, 10));
        assert_eq!(measures.len(), 1);
        assert_eq!(measures[0].window_kind_id, "b");
        assert_eq!(measures[0].stack_tabs, vec!["a", "b"]);
    }

    #[test]
    fn window_chrome_recesses_tabs_into_the_top_corners_of_a_closed_shape() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 40, 5);
        let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let mut w = WindowState::new("Puzzle 3D");
        w.focused = true;
        ChromeState::Window(w).paint(&theme, rect, &mut buf);

        // exact shape: both tabs are real 2-row boxes sharing the window's own left/right wall, each
        // bending its short inner wall down into the main body's top edge one row below
        assert_eq!(row_text(&buf, 0), "┌️─️─️─️─️─️─️─️─️─️─️─️┐️                    ┌️─️─️─️─️─️┐️");
        assert_eq!(row_text(&buf, 1), "│️ Puzzle 3D │️                    │️ ⤢ ✕️ │️");
        assert_eq!(row_text(&buf, 2), "│️           └️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️┘️     │️");
        assert_eq!(row_text(&buf, 3), "│️                                      │️");
        assert_eq!(row_text(&buf, 4), "└️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️─️┘️");

        let title_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == 'P').expect("title text rendered");
        assert_eq!(buf.get(title_x, 1).unwrap().fg, theme.role(Role::Accent));
    }

    #[test]
    fn window_chrome_flattens_the_right_side_when_no_controls_tab_is_wanted() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 40, 6);
        let mut buf = CellBuffer::new(Size { width: 40, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let mut w = WindowState::new("Log");
        w.closable = false;
        w.maximizable = false;
        ChromeState::Window(w).paint(&theme, rect, &mut buf);

        // the title tab still rises, but the right side has no tab: its corner sits flat at the
        // main body's top row instead of rising — the window stays one closed shape either way
        assert_eq!(row_text(&buf, 0), "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}                                 ");
        assert_eq!(row_text(&buf, 1), "\u{2502} Log \u{2502}                                 ");
        assert_eq!(row_text(&buf, 2), "\u{2502}     \u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}");
        assert_eq!(row_text(&buf, 3), "\u{2502}                                      \u{2502}");
        assert_eq!(row_text(&buf, 4), "\u{2502}                                      \u{2502}");
        assert_eq!(row_text(&buf, 5), "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}");
        assert!(!row_text(&buf, 0).contains('\u{2922}') && !row_text(&buf, 1).contains('\u{2922}'), "no controls tab was requested");
    }

    #[test]
    fn window_chrome_hides_both_tabs_when_too_narrow_for_even_the_title() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 10, 5);
        let mut buf = CellBuffer::new(Size { width: 10, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let window = ChromeState::Window(WindowState::new("X"));
        window.paint(&theme, rect, &mut buf);
        // plain flat box: a single top row, no raised tabs at all
        assert_eq!(row_text(&buf, 0), "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}");
        assert_eq!(window.window_control_at(rect, Pos { x: 5, y: 0 }), None);
        assert_eq!(window.window_control_at(rect, Pos { x: 5, y: 1 }), None);
    }

    #[test]
    fn window_control_clicks_resolve_to_close_and_maximize_signals() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 40, 5);
        let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let window = ChromeState::Window(WindowState::new("Plugins"));
        window.paint(&theme, rect, &mut buf);
        let maximize_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2922}').expect("maximize glyph rendered");
        let close_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
        assert_eq!(window.window_control_at(rect, Pos { x: maximize_x, y: 1 }), Some(WidgetSignal::WindowMaximize));
        assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 1 }), Some(WidgetSignal::WindowClose));
        assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 0 }), None, "clicks on the tab's own top edge must not trigger a control");
        assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 2 }), None, "clicks below the tab row must not trigger a control");
    }

    #[test]
    fn tui_dispatch_emits_window_close_signal_on_click() {
        let mut tui = crate::tui::engine::Tui::new(Size { width: 40, height: 12 }, Theme::new(AppearanceName::Dark));
        let navbar = NavbarState { left: vec![], center: vec![], right: vec![] };
        let footer = FooterState { hints: vec![], status: String::new() };
        let layout = even_window_layout(&["plugins".to_string()]);
        let built = shell(&mut tui.scene, navbar, footer, &layout);
        tui.render_full();
        let (_, window_id) = built.windows[0].clone();
        let rect = tui.scene.rect(window_id);
        let mut buf = CellBuffer::new(Size { width: rect.width, height: rect.height }, Cell::blank([0, 0, 0], [0, 0, 0]));
        ChromeState::Window(WindowState::new("plugins")).paint(&Theme::new(AppearanceName::Dark), Rect::new(0, 0, rect.width, rect.height), &mut buf);
        let close_x = (0..rect.width).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
        let signals = tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Down(0), pos: Pos { x: rect.x + close_x, y: rect.y + 1 }, mods: 0 }));
        assert_eq!(signals, vec![(window_id, WidgetSignal::WindowClose)]);
    }

    fn sample_table() -> TableState {
        let columns = vec![TableColumn::new("Plugin / App", 0, TableAlign::Left), TableColumn::new("React", 6, TableAlign::Right)];
        let rows = vec![
            TableRow::parent("puzzle", vec!["puzzle".into(), "".into()]),
            TableRow::child("puzzle2d", vec!["puzzle2d".into(), "6012".into()], 1),
            TableRow::child("puzzle3d", vec!["puzzle3d".into(), "6013".into()], 1),
            TableRow::parent("draw", vec!["draw".into(), "".into()]),
            TableRow::child("draw", vec!["draw".into(), "6064".into()], 1),
        ];
        TableState::new(columns, rows)
    }

    #[test]
    fn table_header_is_bold_muted_with_a_hairline_underline_and_row_separators() {
        let theme = Theme::new(AppearanceName::Dark);
        let table = sample_table();
        let rect = Rect::new(0, 0, 40, 12);
        let mut buf = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(table).paint(&theme, rect, &mut buf, false);

        assert_eq!(row_text(&buf, 0).trim_end(), "Plugin / App                       React");
        assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));
        assert_eq!(buf.get(0, 0).unwrap().attrs & attr::BOLD, attr::BOLD, "header must be bold");
        assert_eq!(row_text(&buf, 1), "\u{2500}".repeat(40), "hairline underline missing below the header");
        // no vertical rules anywhere in the header/underline rows
        assert!(!row_text(&buf, 0).contains('\u{2502}'));

        assert_eq!(row_text(&buf, 2).trim_end(), "\u{25be} puzzle");
        assert_eq!(row_text(&buf, 3), "\u{2500}".repeat(40), "row separator missing after the first row");
        assert_eq!(row_text(&buf, 4).trim_end(), "    puzzle2d                        6012");
    }

    #[test]
    fn table_visible_indices_skips_children_of_a_collapsed_parent() {
        let mut table = sample_table();
        assert_eq!(table.visible_indices(), vec![0, 1, 2, 3, 4]);
        table.rows[0].expanded = false;
        assert_eq!(table.visible_indices(), vec![0, 3, 4], "puzzle's children must be hidden while collapsed");
    }

    #[test]
    fn table_on_key_navigates_visible_rows_toggles_and_activates_leaves() {
        let mut table = sample_table();
        table.rows[0].expanded = false;
        table.selected = 0;
        let mut widget = WidgetState::Table(table);
        // Down should skip the hidden puzzle2d/puzzle3d children and land on "draw"
        let down = KeyEvent { key: Key::Down, mods: 0 };
        assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(3)));
        let WidgetState::Table(table) = &mut widget else { unreachable!() };
        assert_eq!(table.selected, 3);
        table.selected = 0;

        let enter = KeyEvent { key: Key::Enter, mods: 0 };
        assert_eq!(widget.on_key(&enter), Some(WidgetSignal::SelectionChanged(0)));
        let WidgetState::Table(table) = &mut widget else { unreachable!() };
        assert!(table.rows[0].expanded, "Enter on a collapsed parent should expand it");
        table.selected = 1;

        assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)), "Enter on a leaf should activate it");
    }

    #[test]
    fn table_selected_row_uses_active_base_only_when_focused() {
        let theme = Theme::new(AppearanceName::Dark);
        let mut table = sample_table();
        table.selected = 1;
        let rect = Rect::new(0, 0, 40, 12);

        let mut unfocused = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(TableState::new(std::mem::take(&mut table.columns), std::mem::take(&mut table.rows))).paint(&theme, rect, &mut unfocused, false);
        assert_ne!(unfocused.get(4, 4).unwrap().bg, theme.role(Role::ActiveBase), "unfocused selection must not fill with the accent");

        let mut table2 = sample_table();
        table2.selected = 1;
        let mut focused = CellBuffer::new(Size { width: 40, height: 12 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(table2).paint(&theme, rect, &mut focused, true);
        assert_eq!(focused.get(4, 4).unwrap().bg, theme.role(Role::ActiveBase));
        assert_eq!(focused.get(4, 4).unwrap().fg, theme.role(Role::ActiveForeground));
    }

    #[test]
    fn table_flex_column_fills_remaining_width_and_right_aligns_numeric_column() {
        let theme = Theme::new(AppearanceName::Dark);
        let table = sample_table();
        let rect = Rect::new(0, 0, 20, 6);
        let mut buf = CellBuffer::new(Size { width: 20, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Table(table).paint(&theme, rect, &mut buf, false);
        let header = row_text(&buf, 0);
        assert_eq!(header.len(), 20);
        assert!(header.trim_end().ends_with("React"), "the fixed-width numeric column should sit right-aligned at the row's end: {header:?}");
        let _ = theme.surface(Surface::Window);
    }

    #[test]
    fn diff_emits_minimal_runs() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let prev = CellBuffer::new(Size { width: 10, height: 2 }, blank);
        let mut next = prev.clone();
        next.put(3, 0, Cell { ch: 'x', ..blank });
        let runs = diff(&prev, &next);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0], crate::tui::cell::DiffRun { y: 0, x: 3, len: 1 });
    }

    #[test]
    fn diff_merges_nearby_runs() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let prev = CellBuffer::new(Size { width: 20, height: 1 }, blank);
        let mut next = prev.clone();
        next.put(0, 0, Cell { ch: 'a', ..blank });
        next.put(3, 0, Cell { ch: 'b', ..blank });
        let runs = diff(&prev, &next);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].len, 4);
    }

    #[test]
    fn parser_roundtrip_arrow_keys_with_modifiers() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[1;5C", &mut events);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::Key(k) => {
                assert_eq!(k.key, Key::Right);
                assert_eq!(k.mods, crate::tui::event::mods::CTRL);
            }
            _ => panic!("expected key event"),
        }
    }

    #[test]
    fn parser_sgr_mouse_click() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[<0;10;5M", &mut events);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::Mouse(m) => {
                assert_eq!(m.pos.x, 9);
                assert_eq!(m.pos.y, 4);
            }
            _ => panic!("expected mouse event"),
        }
    }

    #[test]
    fn parser_bracketed_paste() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[200~hello\x1b[201~", &mut events);
        assert_eq!(events, vec![Event::Paste("hello".to_string())]);
    }

    #[test]
    fn parser_lone_esc_flushes_on_timeout() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b", &mut events);
        assert!(events.is_empty());
        parser.flush_escape(&mut events);
        assert_eq!(events, vec![Event::Key(crate::tui::event::KeyEvent { key: Key::Esc, mods: 0 })]);
    }

    #[test]
    fn parser_split_utf8_across_feeds() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        let bytes = "ü".as_bytes();
        parser.feed(&bytes[..1], &mut events);
        assert!(events.is_empty());
        parser.feed(&bytes[1..], &mut events);
        assert_eq!(events, vec![Event::Key(crate::tui::event::KeyEvent { key: Key::Char('ü'), mods: 0 })]);
    }

    #[test]
    fn text_display_width_and_truncate() {
        assert_eq!(display_width("abc"), 3);
        let (s, w) = truncate_to("abcdef", 3);
        assert_eq!(s, "abc");
        assert_eq!(w, 3);
    }

    #[test]
    fn theme_light_and_dark_differ() {
        use crate::tui::theme::{Role, Surface};
        let light = Theme::new(AppearanceName::Light);
        let dark = Theme::new(AppearanceName::Dark);
        assert_ne!(light.surface(Surface::Base), dark.surface(Surface::Base));
        for surface in [Surface::Base, Surface::Window, Surface::Pane, Surface::Panel, Surface::Dialog, Surface::Menu] {
            let _ = light.surface(surface);
            let _ = dark.surface(surface);
        }
        assert_ne!(light.surface(Surface::Base), light.surface(Surface::Menu));
        let _ = light.role(Role::HoverInteractive);
    }

    #[test]
    fn wasm_host_feed_and_render_smoke() {
        let mut host = crate::tui::host::WasmHost::new(40, 10, true);
        host.feed(b"\r");
        let patch = host.render();
        assert!(!patch.is_empty());
    }

    //#region 🔖️Geometry
    #[test]
    fn rect_contains_checks_boundaries() {
        let r = Rect::new(2, 2, 3, 3);
        assert!(r.contains(Pos { x: 2, y: 2 }));
        assert!(r.contains(Pos { x: 4, y: 4 }));
        assert!(!r.contains(Pos { x: 5, y: 4 }), "the right edge is exclusive");
        assert!(!r.contains(Pos { x: 1, y: 2 }), "left of the rect is outside");
    }

    #[test]
    fn rect_intersect_returns_overlap_or_default_on_miss() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);
        assert_eq!(a.intersect(b), Rect::new(5, 5, 5, 5));
        let miss = Rect::new(20, 20, 5, 5);
        assert_eq!(a.intersect(miss), Rect::default());
    }

    #[test]
    fn rect_inset_shrinks_and_clamps_when_margin_exceeds_size() {
        let r = Rect::new(0, 0, 10, 10);
        assert_eq!(r.inset(2), Rect::new(2, 2, 6, 6));
        let small = Rect::new(0, 0, 3, 3);
        let shrunk = small.inset(5);
        assert_eq!(shrunk, Rect { x: 3, y: 3, width: 0, height: 0 }, "an oversized margin clamps size to zero but the origin still shifts by min(margin, size)");
    }

    #[test]
    fn rect_split_top_and_split_bottom_partition_rect() {
        let r = Rect::new(1, 1, 10, 10);
        let (top, rest) = r.split_top(3);
        assert_eq!(top, Rect::new(1, 1, 10, 3));
        assert_eq!(rest, Rect::new(1, 4, 10, 7));
        let (rest2, bottom) = r.split_bottom(4);
        assert_eq!(bottom, Rect::new(1, 7, 10, 4));
        assert_eq!(rest2, Rect::new(1, 1, 10, 6));
        let (top2, rest3) = r.split_top(50);
        assert_eq!(top2.height, 10, "rows beyond the rect's height clamp to the full height");
        assert_eq!(rest3.height, 0);
    }
    //#endregion 🔖️Geometry

    //#region 🔖️Text
    #[test]
    fn text_char_cells_zero_and_wide() {
        assert_eq!(crate::tui::text::char_cells('a'), 1);
        assert_eq!(crate::tui::text::char_cells('世'), 2);
        assert_eq!(crate::tui::text::char_cells('\u{200b}'), 0, "a zero-width space occupies no cells");
    }

    #[test]
    fn truncate_to_stops_before_splitting_a_wide_char() {
        let (s, w) = truncate_to("a世b", 2);
        assert_eq!(s, "a", "the wide char doesn't fit in the remaining 1 cell, so it's dropped whole");
        assert_eq!(w, 1);
        let (s2, w2) = truncate_to("世", 2);
        assert_eq!(s2, "世");
        assert_eq!(w2, 2);
    }
    //#endregion 🔖️Text

    //#region 🔖️Cell
    #[test]
    fn cell_buffer_get_returns_none_out_of_bounds() {
        let buf = CellBuffer::new(Size { width: 3, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        assert!(buf.get(3, 0).is_none());
        assert!(buf.get(0, 3).is_none());
        assert!(buf.get(2, 2).is_some());
    }

    #[test]
    fn cell_buffer_put_pairs_and_orphans_wide_char_continuations() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut buf = CellBuffer::new(Size { width: 5, height: 1 }, blank);
        buf.put(1, 0, Cell { ch: '世', width: 2, ..blank });
        assert_eq!(buf.get(1, 0).unwrap().width, 2);
        assert_eq!(buf.get(2, 0).unwrap().width, 0, "the wide char auto-populates its continuation cell");

        buf.put(2, 0, Cell { ch: '\0', width: 0, ..blank });
        assert_eq!(buf.get(2, 0).unwrap().width, 0, "a width-0 write next to a wide lead stays paired");

        buf.put(0, 0, Cell { ch: '\0', width: 0, ..blank });
        assert_eq!(buf.get(0, 0).unwrap().width, 0, "column 0 has no left neighbor to check, so width is left untouched");
    }

    #[test]
    fn cell_buffer_put_clamps_wide_char_at_right_edge() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut buf = CellBuffer::new(Size { width: 3, height: 1 }, blank);
        buf.put(2, 0, Cell { ch: '世', width: 2, ..blank });
        assert_eq!(buf.get(2, 0).unwrap().width, 1, "a wide char at the last column clamps to width 1 instead of overflowing");
    }

    #[test]
    fn cell_buffer_put_str_counts_clipped_cells_but_only_draws_inside_clip() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut buf = CellBuffer::new(Size { width: 10, height: 1 }, blank);
        let clip = Rect::new(2, 0, 4, 1);
        let written = buf.put_str(Pos { x: 0, y: 0 }, "abcdefgh", [1, 1, 1], [2, 2, 2], 0, clip);
        assert_eq!(written, 6, "cells advance past the clip's left edge even though nothing is drawn there");
        assert_eq!(buf.get(0, 0).unwrap().ch, ' ', "cells left of the clip stay untouched");
        assert_eq!(buf.get(2, 0).unwrap().ch, 'c');
        assert_eq!(buf.get(5, 0).unwrap().ch, 'f');
        assert_eq!(buf.get(6, 0).unwrap().ch, ' ', "cells right of the clip stay untouched");

        let out_of_row = buf.put_str(Pos { x: 2, y: 1 }, "zzz", [0, 0, 0], [0, 0, 0], 0, clip);
        assert_eq!(out_of_row, 0, "a row outside the clip's y-range writes nothing");
    }

    #[test]
    fn cell_buffer_fill_rect_clips_to_buffer_bounds() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut buf = CellBuffer::new(Size { width: 4, height: 4 }, blank);
        buf.fill_rect(Rect::new(2, 2, 10, 10), Cell { ch: '#', ..blank });
        assert_eq!(buf.get(2, 2).unwrap().ch, '#');
        assert_eq!(buf.get(3, 3).unwrap().ch, '#');
    }

    #[test]
    fn cell_buffer_hline_and_vline_paint_expected_cells() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut buf = CellBuffer::new(Size { width: 5, height: 5 }, blank);
        buf.hline(Pos { x: 1, y: 1 }, 3, '-', [1, 1, 1], [2, 2, 2]);
        assert_eq!(buf.get(1, 1).unwrap().ch, '-');
        assert_eq!(buf.get(3, 1).unwrap().ch, '-');
        assert_eq!(buf.get(0, 1).unwrap().ch, ' ');
        assert_eq!(buf.get(4, 1).unwrap().ch, ' ');

        buf.vline(Pos { x: 2, y: 0 }, 3, '|', [1, 1, 1], [2, 2, 2]);
        assert_eq!(buf.get(2, 0).unwrap().ch, '|');
        assert_eq!(buf.get(2, 2).unwrap().ch, '|');
        assert_eq!(buf.get(2, 3).unwrap().ch, ' ');
    }

    #[test]
    fn diff_full_redraw_when_sizes_differ() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let a = CellBuffer::new(Size { width: 5, height: 5 }, blank);
        let b = CellBuffer::new(Size { width: 6, height: 5 }, blank);
        let runs = diff(&a, &b);
        assert_eq!(runs, vec![DiffRun { y: 0, x: 0, len: 30 }]);
    }
    //#endregion 🔖️Cell

    //#region 🔖️Ansi
    #[test]
    fn emit_runs_writes_cursor_move_and_truecolor_sgr() {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        let mut buf = CellBuffer::new(Size { width: 5, height: 1 }, blank);
        buf.put(2, 0, Cell { ch: 'Q', fg: [10, 20, 30], bg: [40, 50, 60], attrs: attr::BOLD, width: 1 });
        let runs = vec![DiffRun { y: 0, x: 2, len: 1 }];
        let mut patch = AnsiPatch::default();
        emit_runs(&buf, &runs, &mut patch);
        assert!(patch.0.contains("\x1b[1;3H"), "cursor moves to the 1-indexed row/col");
        assert!(patch.0.contains(";38;2;10;20;30"), "foreground truecolor sgr");
        assert!(patch.0.contains(";48;2;40;50;60"), "background truecolor sgr");
        assert!(patch.0.contains(";1"), "bold attribute sgr");
        assert!(patch.0.ends_with('Q'));
    }

    #[test]
    fn ansi_setup_and_teardown_sequences_contain_expected_escapes() {
        assert!(setup_sequence().contains("\x1b[?1049h"), "enters the alternate screen");
        assert!(setup_sequence().contains("\x1b[?25l"), "hides the cursor");
        assert!(teardown_sequence().contains("\x1b[?1049l"), "restores the primary screen");
        assert!(teardown_sequence().contains("\x1b[?25h"), "shows the cursor");
    }

    #[test]
    fn parser_ctrl_char_maps_to_char_with_ctrl_mod() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(&[0x01, 0x03], &mut events);
        assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Char('a'), mods: crate::tui::event::mods::CTRL }), Event::Key(KeyEvent { key: Key::Char('c'), mods: crate::tui::event::mods::CTRL })]);
    }

    #[test]
    fn parser_alt_prefixed_char_sets_alt_mod() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1bj", &mut events);
        assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Char('j'), mods: crate::tui::event::mods::ALT })]);
    }

    #[test]
    fn parser_backtab_and_focus_gained_lost() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[Z\x1b[I\x1b[O", &mut events);
        assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::BackTab, mods: 0 }), Event::FocusGained, Event::FocusLost]);
    }

    #[test]
    fn parser_tilde_navigation_and_function_keys() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[3~\x1b[15~", &mut events);
        assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Delete, mods: 0 }), Event::Key(KeyEvent { key: Key::F(5), mods: 0 })]);
    }

    #[test]
    fn parser_ss3_function_keys() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1bOP\x1bOQ", &mut events);
        assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::F(1), mods: 0 }), Event::Key(KeyEvent { key: Key::F(2), mods: 0 })]);
    }

    #[test]
    fn parser_mouse_scroll_drag_and_release_kinds() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(b"\x1b[<64;5;5M\x1b[<97;5;5M\x1b[<32;5;5M\x1b[<0;5;5m", &mut events);
        let kinds: Vec<MouseKind> = events
            .iter()
            .map(|e| match e {
                Event::Mouse(m) => m.kind,
                _ => panic!("expected mouse event"),
            })
            .collect();
        assert_eq!(kinds, vec![MouseKind::ScrollUp, MouseKind::ScrollDown, MouseKind::Drag(0), MouseKind::Up(0)]);
    }

    #[test]
    fn parser_control_keys_enter_tab_backspace() {
        let mut parser = AnsiParser::new();
        let mut events = Vec::new();
        parser.feed(&[0x0d, 0x0a, 0x09, 0x7f, 0x08], &mut events);
        assert_eq!(
            events,
            vec![
                Event::Key(KeyEvent { key: Key::Enter, mods: 0 }),
                Event::Key(KeyEvent { key: Key::Enter, mods: 0 }),
                Event::Key(KeyEvent { key: Key::Tab, mods: 0 }),
                Event::Key(KeyEvent { key: Key::Backspace, mods: 0 }),
                Event::Key(KeyEvent { key: Key::Backspace, mods: 0 }),
            ]
        );
    }
    //#endregion 🔖️Ansi

    //#region 🔖️Vt
    fn vt_screen(w: u16, h: u16) -> crate::tui::vt::VtScreen {
        crate::tui::vt::VtScreen::new(Size { width: w, height: h }, 100)
    }

    fn vt_row(screen: &crate::tui::vt::VtScreen, y: u16) -> String {
        (0..screen.size.width)
            .filter_map(|x| screen.cell_at(x, y))
            .map(|c| c.ch)
            .filter(|&c| c != '\0')
            .collect()
    }

    #[test]
    fn vt_cursor_motion_cup_and_cuu() {
        let mut s = vt_screen(10, 5);
        s.feed(b"\x1b[3;4H");
        assert_eq!(s.cursor, Pos { x: 3, y: 2 });
        s.feed(b"\x1b[2A");
        assert_eq!(s.cursor, Pos { x: 3, y: 0 });
        s.feed(b"\x1b[1B\x1b[2C\x1b[1D");
        assert_eq!(s.cursor, Pos { x: 4, y: 1 });
    }

    #[test]
    fn vt_wrap_at_edge() {
        let mut s = vt_screen(4, 3);
        s.feed(b"abcdX");
        assert_eq!(&vt_row(&s, 0)[..4], "abcd");
        assert_eq!(s.cell_at(0, 1).map(|c| c.ch), Some('X'));
    }

    #[test]
    fn vt_scroll_region_decstbm_newline_scrolls_inside() {
        let mut s = vt_screen(5, 5);
        s.feed(b"AAAAA\nBBBBB\nCCCCC\nDDDDD\nEEEEE");
        s.feed(b"\x1b[2;4r");
        assert_eq!((s.scroll_top, s.scroll_bottom), (1, 3));
        s.feed(b"\x1b[4;1H\n");
        assert_eq!(&vt_row(&s, 0)[..5], "AAAAA");
        assert_eq!(&vt_row(&s, 1)[..5], "CCCCC");
        assert_eq!(&vt_row(&s, 2)[..5], "DDDDD");
        assert_eq!(vt_row(&s, 3).trim(), "");
        assert_eq!(&vt_row(&s, 4)[..5], "EEEEE");
    }

    #[test]
    fn vt_sgr_truecolor_sets_cell_fg_bg() {
        let mut s = vt_screen(5, 2);
        s.feed(b"\x1b[38;2;10;20;30;48;2;40;50;60mZ");
        let cell = s.cell_at(0, 0).expect("cell");
        assert_eq!(cell.ch, 'Z');
        assert_eq!(cell.fg, [10, 20, 30]);
        assert_eq!(cell.bg, [40, 50, 60]);
    }

    #[test]
    fn vt_alt_screen_1049_preserves_primary() {
        let mut s = vt_screen(5, 3);
        s.feed(b"HELLO");
        assert_eq!(&vt_row(&s, 0)[..5], "HELLO");
        s.feed(b"\x1b[?1049h");
        assert!(s.alt_active);
        s.feed(b"ALT");
        assert_eq!(&vt_row(&s, 0)[..3], "ALT");
        s.feed(b"\x1b[?1049l");
        assert!(!s.alt_active);
        assert_eq!(&vt_row(&s, 0)[..5], "HELLO");
    }

    #[test]
    fn vt_resize_clamps_cursor() {
        let mut s = vt_screen(10, 10);
        s.feed(b"\x1b[8;8H");
        assert_eq!(s.cursor, Pos { x: 7, y: 7 });
        s.resize(Size { width: 4, height: 3 });
        assert_eq!(s.size, Size { width: 4, height: 3 });
        assert_eq!(s.cursor, Pos { x: 3, y: 2 });
    }
    //#endregion 🔖️Vt

    //#region 🔖️Scene
    #[test]
    fn scene_node_mut_setters_update_content_and_visibility() {
        let mut scene = Scene::new();
        let root = scene.root();
        let id = scene.add(root, Node::new(NodeContent::Box));
        scene.node_mut(id).set_text("hi");
        match &scene.node(id).content {
            NodeContent::Text(s) => assert_eq!(s, "hi"),
            _ => panic!("expected text content"),
        }
        scene.node_mut(id).set_visible(false);
        assert!(!scene.node(id).visible);
    }

    #[test]
    fn scene_widget_and_chrome_return_none_for_mismatched_content() {
        let mut scene = Scene::new();
        let root = scene.root();
        let id = scene.add(root, Node::new(NodeContent::Box));
        assert!(scene.node_mut(id).widget().is_none());
        assert!(scene.node_mut(id).chrome().is_none());
    }

    #[test]
    fn scene_hit_finds_deepest_visible_node_topmost_child_wins() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Stack, ..Default::default() });
        let a = scene.add(root, Node::new(NodeContent::Box));
        let b = scene.add(root, Node::new(NodeContent::Box));
        solve(&mut scene, Rect::new(0, 0, 10, 10));
        assert_eq!(scene.hit(Pos { x: 5, y: 5 }), Some(b), "the last-added, topmost child wins on overlap");
        scene.node_mut(b).set_visible(false);
        assert_eq!(scene.hit(Pos { x: 5, y: 5 }), Some(a), "an invisible node is skipped by hit-testing");
        assert_eq!(scene.hit(Pos { x: 20, y: 20 }), None, "outside every rect");
    }

    #[test]
    fn scene_remove_detaches_subtree_from_parent() {
        let mut scene = Scene::new();
        let root = scene.root();
        let a = scene.add(root, Node::new(NodeContent::Box));
        scene.add(a, Node::new(NodeContent::Box));
        scene.remove(a);
        assert!(!scene.node(root).children().contains(&a));
    }

    #[test]
    #[should_panic(expected = "stale NodeId")]
    fn scene_node_panics_after_its_id_is_removed() {
        let mut scene = Scene::new();
        let root = scene.root();
        let a = scene.add(root, Node::new(NodeContent::Box));
        scene.remove(a);
        scene.node(a);
    }
    //#endregion 🔖️Scene

    //#region 🔖️Layout
    #[test]
    fn layout_column_direction_stacks_children_vertically() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Column, ..Default::default() });
        let a = scene.add(root, Node::new(NodeContent::Box));
        let b = scene.add(root, Node::new(NodeContent::Box));
        scene.node_mut(a).set_constraint(Constraint { height: Dimension::Cells(3), ..Default::default() });
        scene.node_mut(b).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });
        solve(&mut scene, Rect::new(0, 0, 10, 10));
        assert_eq!(scene.rect(a), Rect::new(0, 0, 10, 3));
        assert_eq!(scene.rect(b), Rect::new(0, 3, 10, 7));
    }

    #[test]
    fn layout_distribute_weight_remainder_goes_to_earliest_child_on_ties() {
        let mut scene = Scene::new();
        let root = scene.root();
        scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
        let a = scene.add(root, Node::new(NodeContent::Box));
        let b = scene.add(root, Node::new(NodeContent::Box));
        let c = scene.add(root, Node::new(NodeContent::Box));
        for id in [a, b, c] {
            scene.node_mut(id).set_constraint(Constraint { width: Dimension::Weight(1), ..Default::default() });
        }
        solve(&mut scene, Rect::new(0, 0, 10, 5));
        let widths = [scene.rect(a).width, scene.rect(b).width, scene.rect(c).width];
        assert_eq!(widths.iter().sum::<u16>(), 10);
        assert_eq!(widths[0], 4, "equal fractional remainders are broken by original order");
    }
    //#endregion 🔖️Layout

    //#region 🔖️Widget
    #[test]
    fn list_on_key_boundaries_toggle_and_activate() {
        let mut widget = WidgetState::List(ListState::new(vec!["a".to_string(), "b".to_string()]));
        let up = KeyEvent { key: Key::Up, mods: 0 };
        assert_eq!(widget.on_key(&up), None, "already at the top");
        let down = KeyEvent { key: Key::Down, mods: 0 };
        assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(1)));
        assert_eq!(widget.on_key(&down), None, "already at the bottom");
        let space = KeyEvent { key: Key::Char(' '), mods: 0 };
        assert_eq!(widget.on_key(&space), Some(WidgetSignal::Toggled(true)));
        assert_eq!(widget.on_key(&space), Some(WidgetSignal::Toggled(false)));
        let enter = KeyEvent { key: Key::Enter, mods: 0 };
        assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)));

        let mut empty_widget = WidgetState::List(ListState::new(vec![]));
        assert_eq!(empty_widget.on_key(&space), None, "toggling with no items is a no-op");
    }

    #[test]
    fn select_on_key_wraps_and_ignores_empty_options() {
        let mut widget = WidgetState::Select(SelectState { label: "L".to_string(), options: vec!["x".to_string(), "y".to_string()], index: 0 });
        let left = KeyEvent { key: Key::Left, mods: 0 };
        assert_eq!(widget.on_key(&left), Some(WidgetSignal::SelectionChanged(1)), "Left from index 0 wraps to the last option");
        let right = KeyEvent { key: Key::Right, mods: 0 };
        assert_eq!(widget.on_key(&right), Some(WidgetSignal::SelectionChanged(0)));

        let mut empty_widget = WidgetState::Select(SelectState { label: "L".to_string(), options: vec![], index: 0 });
        assert_eq!(empty_widget.on_key(&right), None);
    }

    #[test]
    fn tabs_on_key_wraps_and_ignores_empty_tabs() {
        let mut widget = WidgetState::Tabs(TabsState { tabs: vec!["one".to_string(), "two".to_string()], active: 0 });
        let left = KeyEvent { key: Key::Left, mods: 0 };
        assert_eq!(widget.on_key(&left), Some(WidgetSignal::TabChanged(1)), "Left from index 0 wraps to the last tab");
        let right = KeyEvent { key: Key::Right, mods: 0 };
        assert_eq!(widget.on_key(&right), Some(WidgetSignal::TabChanged(0)));

        let mut empty_widget = WidgetState::Tabs(TabsState { tabs: vec![], active: 0 });
        assert_eq!(empty_widget.on_key(&right), None);
    }

    #[test]
    fn input_on_key_inserts_utf8_and_respects_cursor_bounds() {
        let mut widget = WidgetState::Input(InputState { value: String::new(), cursor: 0, placeholder: "ph".to_string() });
        let type_u = KeyEvent { key: Key::Char('ü'), mods: 0 };
        assert_eq!(widget.on_key(&type_u), Some(WidgetSignal::ValueChanged("ü".to_string())));
        let cursor_after_insert = match &widget {
            WidgetState::Input(i) => i.cursor,
            _ => unreachable!(),
        };
        assert_eq!(cursor_after_insert, 'ü'.len_utf8());

        let left = KeyEvent { key: Key::Left, mods: 0 };
        assert_eq!(widget.on_key(&left), None);
        let cursor_after_left = match &widget {
            WidgetState::Input(i) => i.cursor,
            _ => unreachable!(),
        };
        assert_eq!(cursor_after_left, 1, "Left steps by one byte, not a full UTF-8 char boundary");
        assert_eq!(widget.on_key(&left), None);
        let cursor_after_second_left = match &widget {
            WidgetState::Input(i) => i.cursor,
            _ => unreachable!(),
        };
        assert_eq!(cursor_after_second_left, 0);
        assert_eq!(widget.on_key(&left), None, "already at cursor 0");

        let backspace = KeyEvent { key: Key::Backspace, mods: 0 };
        assert_eq!(widget.on_key(&backspace), None, "nothing before the cursor to delete");

        let right = KeyEvent { key: Key::Right, mods: 0 };
        assert_eq!(widget.on_key(&right), None);
        assert_eq!(widget.on_key(&right), None);
        assert_eq!(widget.on_key(&right), None, "already at the end");

        assert_eq!(widget.on_key(&backspace), Some(WidgetSignal::ValueChanged(String::new())), "backspace at the end removes the multi-byte char");
    }

    #[test]
    fn log_state_push_evicts_oldest_beyond_capacity() {
        let mut log = LogState::new(3);
        log.push("a");
        log.push("b");
        log.push("c");
        log.push("d");
        assert_eq!(log.lines().len(), 3);
        assert_eq!(log.lines().front(), Some(&"b".to_string()), "the oldest line is evicted once capacity is exceeded");
        log.clear();
        assert!(log.lines().is_empty());
        assert_eq!(log.scroll, LogScroll::Follow, "clearing resets scroll to Follow");
    }

    #[test]
    fn log_on_key_page_and_home_end_scroll_states() {
        let mut log = LogState::new(20);
        for i in 0..20 {
            log.push(&format!("l{i}"));
        }
        let mut widget = WidgetState::Log(log);

        let page_down = KeyEvent { key: Key::PageDown, mods: 0 };
        widget.on_key(&page_down);
        assert_eq!(log_scroll(&widget), LogScroll::Follow, "PageDown while already following is a no-op");

        let home = KeyEvent { key: Key::Home, mods: 0 };
        widget.on_key(&home);
        assert_eq!(log_scroll(&widget), LogScroll::At(0));

        widget.on_key(&page_down);
        assert_eq!(log_scroll(&widget), LogScroll::At(10), "PageDown short of the end scrolls forward by 10");

        widget.on_key(&page_down);
        assert_eq!(log_scroll(&widget), LogScroll::Follow, "PageDown past the end resumes following");

        let page_up = KeyEvent { key: Key::PageUp, mods: 0 };
        widget.on_key(&page_up);
        assert_eq!(log_scroll(&widget), LogScroll::At(19), "PageUp from Follow jumps to the last line");

        widget.on_key(&page_up);
        assert_eq!(log_scroll(&widget), LogScroll::At(9), "a second PageUp scrolls back 10 more lines");

        let end = KeyEvent { key: Key::End, mods: 0 };
        widget.on_key(&end);
        assert_eq!(log_scroll(&widget), LogScroll::Follow);
    }

    fn log_scroll(widget: &WidgetState) -> LogScroll {
        match widget {
            WidgetState::Log(l) => l.scroll,
            _ => unreachable!(),
        }
    }

    #[test]
    fn table_on_key_left_right_expand_collapse_and_boundaries() {
        let mut table = sample_table();
        table.selected = 0;
        let mut widget = WidgetState::Table(table);
        let up = KeyEvent { key: Key::Up, mods: 0 };
        assert_eq!(widget.on_key(&up), None, "already at the first visible row");

        let left = KeyEvent { key: Key::Left, mods: 0 };
        assert_eq!(widget.on_key(&left), Some(WidgetSignal::SelectionChanged(0)), "Left on an expanded parent collapses it");
        assert_eq!(widget.on_key(&left), None, "Left on an already-collapsed parent is a no-op");

        let right = KeyEvent { key: Key::Right, mods: 0 };
        assert_eq!(widget.on_key(&right), Some(WidgetSignal::SelectionChanged(0)));
        assert_eq!(widget.on_key(&right), None, "Right on an already-expanded parent is a no-op");

        let WidgetState::Table(t) = &mut widget else { unreachable!() };
        t.selected = 1;
        assert_eq!(widget.on_key(&left), None, "Left on a leaf row is a no-op");
        assert_eq!(widget.on_key(&right), None, "Right on a leaf row is a no-op");

        let mut empty_widget = WidgetState::Table(TableState::new(vec![], vec![]));
        assert_eq!(empty_widget.on_key(&up), None, "no rows at all means every key is a no-op");
    }

    #[test]
    fn widget_preferred_size_for_label_select_chip_and_divider() {
        let label = WidgetState::Label(LabelState { text: "hello".to_string(), align: Align::Left, role: Role::Foreground });
        assert_eq!(label.preferred_size(), Size { width: 5, height: 1 });

        let select = WidgetState::Select(SelectState { label: "L".to_string(), options: vec!["opt".to_string()], index: 0 });
        let expected = display_width("L \u{2039} opt \u{203a}");
        assert_eq!(select.preferred_size(), Size { width: expected, height: 1 });

        let chip = WidgetState::Chip(ChipState { label: "tag".to_string(), on: true });
        assert_eq!(chip.preferred_size(), Size { width: 5, height: 1 });

        let divider = WidgetState::Divider(DividerState::default());
        assert_eq!(divider.preferred_size(), Size { width: 1, height: 1 });

        let list = WidgetState::List(ListState::new(vec!["x".to_string()]));
        assert_eq!(list.preferred_size(), Size { width: 0, height: 0 }, "list has no measured preferred size");
    }

    #[test]
    fn paint_list_highlights_selected_row_only_when_focused() {
        let theme = Theme::new(AppearanceName::Dark);
        let list = ListState { items: vec!["a".to_string(), "b".to_string()], selected: 1, offset: 0, marks: vec![false, true] };
        let rect = Rect::new(0, 0, 10, 2);
        let mut buf = CellBuffer::new(Size { width: 10, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::List(list).paint(&theme, rect, &mut buf, true);
        assert_eq!(buf.get(0, 1).unwrap().bg, theme.role(Role::ActiveBase));
        assert_ne!(buf.get(0, 0).unwrap().bg, theme.role(Role::ActiveBase));
        assert!(row_text(&buf, 1).starts_with('\u{2713}'), "a marked row shows a check prefix");
        assert!(row_text(&buf, 0).starts_with(' '), "an unmarked row has a blank prefix");
    }

    #[test]
    fn paint_select_renders_label_and_current_option() {
        let theme = Theme::new(AppearanceName::Dark);
        let select = SelectState { label: "Mode".to_string(), options: vec!["A".to_string(), "B".to_string()], index: 1 };
        let rect = Rect::new(0, 0, 20, 1);
        let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Select(select).paint(&theme, rect, &mut buf, true);
        assert_eq!(row_text(&buf, 0).trim_end(), "Mode: \u{2039} B \u{203a}");
        assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::Accent), "a focused select uses the accent color");
    }

    #[test]
    fn paint_tabs_bolds_the_active_tab() {
        let theme = Theme::new(AppearanceName::Dark);
        let tabs = TabsState { tabs: vec!["One".to_string(), "Two".to_string()], active: 1 };
        let rect = Rect::new(0, 0, 20, 1);
        let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Tabs(tabs).paint(&theme, rect, &mut buf, false);
        let t_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'T').expect("active tab rendered");
        assert_eq!(buf.get(t_x, 0).unwrap().attrs & attr::BOLD, attr::BOLD);
        let o_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'O').expect("inactive tab rendered");
        assert_eq!(buf.get(o_x, 0).unwrap().attrs & attr::BOLD, 0, "inactive tab is not bold");
    }

    #[test]
    fn paint_log_shows_the_tail_when_following() {
        let theme = Theme::new(AppearanceName::Dark);
        let mut log = LogState::new(10);
        for i in 0..5 {
            log.push(&format!("line{i}"));
        }
        let rect = Rect::new(0, 0, 10, 3);
        let mut buf = CellBuffer::new(Size { width: 10, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Log(log).paint(&theme, rect, &mut buf, false);
        assert_eq!(row_text(&buf, 0).trim_end(), "line2");
        assert_eq!(row_text(&buf, 1).trim_end(), "line3");
        assert_eq!(row_text(&buf, 2).trim_end(), "line4");
    }

    #[test]
    fn paint_input_shows_placeholder_and_draws_cursor_when_focused() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 20, 1);
        let empty_input = InputState { value: String::new(), cursor: 0, placeholder: "type here".to_string() };
        let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Input(empty_input).paint(&theme, rect, &mut buf, false);
        assert_eq!(row_text(&buf, 0).trim_end(), "type here");
        assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));

        let filled = InputState { value: "hi".to_string(), cursor: 2, placeholder: "ph".to_string() };
        let mut buf2 = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Input(filled).paint(&theme, rect, &mut buf2, true);
        assert_eq!(buf2.get(2, 0).unwrap().ch, '\u{2588}', "a focused input draws a cursor block at the caret position");
    }

    #[test]
    fn paint_divider_centers_its_label() {
        let theme = Theme::new(AppearanceName::Dark);
        let divider = DividerState { label: Some("Hi".to_string()) };
        let rect = Rect::new(0, 0, 10, 1);
        let mut buf = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Divider(divider).paint(&theme, rect, &mut buf, false);
        let row = row_text(&buf, 0);
        assert!(row.contains("Hi"));
        assert_eq!(row.chars().next().unwrap(), '\u{2500}', "the hairline still draws under the label");
    }

    #[test]
    fn paint_chip_reflects_on_and_off_colors() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 10, 1);
        let on = ChipState { label: "x".to_string(), on: true };
        let mut buf_on = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Chip(on).paint(&theme, rect, &mut buf_on, false);
        assert_eq!(buf_on.get(0, 0).unwrap().bg, theme.role(Role::Accent));

        let off = ChipState { label: "x".to_string(), on: false };
        let mut buf_off = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        WidgetState::Chip(off).paint(&theme, rect, &mut buf_off, false);
        assert_eq!(buf_off.get(0, 0).unwrap().bg, theme.surface(Surface::Panel));
    }
    //#endregion 🔖️Widget

    //#region 🔖️Chrome
    #[test]
    fn paint_navbar_places_left_center_and_right_items() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 30, 2);
        let mut buf = CellBuffer::new(Size { width: 30, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let navbar = NavbarState {
            left: vec![NavItem { id: "l".to_string(), label: "L".to_string(), active: false }],
            center: vec![NavItem { id: "c".to_string(), label: "C".to_string(), active: false }],
            right: vec![NavItem { id: "r".to_string(), label: "R".to_string(), active: true }],
        };
        ChromeState::Navbar(navbar).paint(&theme, rect, &mut buf);
        let row = row_text(&buf, 0);
        assert!(row.trim_start().starts_with('L'));
        assert!(row.contains('C'));
        assert!(row.trim_end().ends_with('R'));
        assert_eq!(row_text(&buf, 1), "\u{2500}".repeat(30), "hairline drawn below the navbar");
        let right_x = (0..30).find(|&x| buf.get(x, 0).unwrap().ch == 'R').expect("right item rendered");
        assert_eq!(buf.get(right_x, 0).unwrap().fg, theme.role(Role::Accent), "the active item uses the accent color");
    }

    #[test]
    fn paint_footer_renders_hints_then_status() {
        let theme = Theme::new(AppearanceName::Dark);
        let rect = Rect::new(0, 0, 30, 2);
        let mut buf = CellBuffer::new(Size { width: 30, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        let footer = FooterState { hints: vec![KeyHint { key: "q".to_string(), label: "quit".to_string() }], status: "OK".to_string() };
        ChromeState::Footer(footer).paint(&theme, rect, &mut buf);
        assert_eq!(row_text(&buf, 0), "\u{2500}".repeat(30), "hairline drawn above the footer");
        let row1 = row_text(&buf, 1);
        assert!(row1.contains('q'));
        assert!(row1.contains("quit"));
        assert!(row1.trim_end().ends_with("OK"));
    }
    //#endregion 🔖️Chrome

    //#region 🔖️Engine
    #[test]
    fn tui_focus_next_and_prev_cycle_through_focusables() {
        let mut tui = crate::tui::engine::Tui::new(Size { width: 20, height: 5 }, Theme::new(AppearanceName::Dark));
        let root = tui.scene.root();
        let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "a".to_string(), align: Align::Left, role: Role::Foreground }))));
        let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "b".to_string(), align: Align::Left, role: Role::Foreground }))));
        assert_eq!(tui.focus(), None);
        tui.focus_next();
        assert_eq!(tui.focus(), Some(a));
        tui.focus_next();
        assert_eq!(tui.focus(), Some(b));
        tui.focus_next();
        assert_eq!(tui.focus(), Some(a), "focus wraps back to the first focusable");
        tui.focus_prev();
        assert_eq!(tui.focus(), Some(b), "focus wraps backward past the first focusable");
    }

    #[test]
    fn tui_dispatch_tab_and_backtab_move_focus() {
        let mut tui = crate::tui::engine::Tui::new(Size { width: 20, height: 5 }, Theme::new(AppearanceName::Dark));
        let root = tui.scene.root();
        let a = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "a".to_string(), align: Align::Left, role: Role::Foreground }))));
        let b = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::Label(LabelState { text: "b".to_string(), align: Align::Left, role: Role::Foreground }))));
        let tab_ev = Event::Key(KeyEvent { key: Key::Tab, mods: 0 });
        assert_eq!(tui.dispatch(&tab_ev), vec![]);
        assert_eq!(tui.focus(), Some(a));
        let backtab_ev = Event::Key(KeyEvent { key: Key::BackTab, mods: 0 });
        tui.dispatch(&backtab_ev);
        assert_eq!(tui.focus(), Some(b));
    }

    #[test]
    fn tui_render_skips_repaint_when_nothing_dirty() {
        let mut tui = crate::tui::engine::Tui::new(Size { width: 10, height: 3 }, Theme::new(AppearanceName::Dark));
        let first = tui.render_full();
        assert!(!first.0.is_empty());
        let second = tui.render();
        assert!(second.0.is_empty(), "no dirty state and no forced redraw emits nothing");
    }

    #[test]
    fn tui_set_appearance_and_resize_force_full_redraw() {
        let mut tui = crate::tui::engine::Tui::new(Size { width: 10, height: 3 }, Theme::new(AppearanceName::Dark));
        tui.render_full();
        tui.set_appearance(AppearanceName::Light);
        let patch = tui.render();
        assert!(!patch.0.is_empty(), "set_appearance forces a full repaint even with no dirty nodes");
        tui.render();
        tui.resize(Size { width: 12, height: 4 });
        let patch2 = tui.render();
        assert!(!patch2.0.is_empty(), "resize forces a full repaint too");
    }
    //#endregion 🔖️Engine

    //#region 🔖️WasmHost
    #[test]
    fn wasm_host_setup_and_teardown_match_ansi_sequences() {
        let host = crate::tui::host::WasmHost::new(10, 5, true);
        assert_eq!(host.setup(), setup_sequence());
        assert_eq!(host.teardown(), teardown_sequence());
    }

    #[test]
    fn wasm_host_resize_updates_engine_size() {
        let mut host = crate::tui::host::WasmHost::new(10, 5, true);
        host.render();
        host.resize(20, 8);
        let patch = host.render();
        assert!(!patch.is_empty(), "resizing triggers a full repaint on the next render");
    }
    //#endregion 🔖️WasmHost


    //#region 🔖️Pty
    #[cfg(all(unix, feature = "tui-terminal"))]
    #[test]
    fn pty_spawn_echo_hello() {
        use crate::tui::pty::{Pty, PtySize};
        use std::time::{Duration, Instant};

        let mut pty = Pty::spawn(
            "/bin/echo",
            &["hello"],
            &[],
            None,
            PtySize { cols: 80, rows: 24 },
        )
        .expect("spawn echo");
        let mut out = Vec::new();
        let mut buf = [0u8; 1024];
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            match pty.try_read(&mut buf) {
                Ok(0) => {
                    if pty.try_wait().ok().flatten().is_some() {
                        break;
                    }
                    if Instant::now() >= deadline {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Ok(n) => out.extend_from_slice(&buf[..n]),
                Err(e) => panic!("try_read failed: {}", e.message),
            }
        }
        let text = String::from_utf8_lossy(&out);
        assert!(text.contains("hello"), "PTY output missing hello: {text:?}");
    }

    #[cfg(all(unix, feature = "tui-terminal"))]
    #[test]
    fn pty_resize_ok() {
        use crate::tui::pty::{Pty, PtySize};

        let mut pty = Pty::spawn(
            "/bin/sleep",
            &["1"],
            &[],
            None,
            PtySize { cols: 80, rows: 24 },
        )
        .expect("spawn sleep");
        pty.resize(PtySize { cols: 100, rows: 40 })
            .expect("resize");
        pty.kill().expect("kill");
    }
    //#endregion 🔖️Pty
}
// #endregion 🔖️Tests
