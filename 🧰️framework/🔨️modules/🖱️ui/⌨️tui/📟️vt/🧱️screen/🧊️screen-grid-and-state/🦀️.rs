//! 🧊️ Screen grid and state: the scrollback and match limits, the cell point, match, modes, charset, pen and saved cursor, the `Grid` and the `VtScreen` state with its default tab stops. See [`super`].

use super::*;

pub(super) const DEFAULT_SCROLLBACK: usize = 10_000;

pub(super) const MAX_MATCHES: usize = 5_000;

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
pub(in super::super) enum Charset {
    Ascii,
    Graphics,
}

#[derive(Clone, Copy)]
pub(super) struct Pen {
    pub(super) fg: Rgb,
    pub(super) bg: Rgb,
    pub(super) attrs: u8,
}

impl Pen {
    pub(super) const DEFAULT: Pen = Pen { fg: XTERM_FG, bg: XTERM_BG, attrs: flag::MASK };

    pub(super) fn glyph(&self, ch: char, width: u8) -> Cell {
        Cell { ch, fg: self.fg, bg: self.bg, attrs: self.attrs, width }
    }

    pub(super) fn erased(&self) -> Cell {
        Cell { ch: ' ', fg: XTERM_FG, bg: self.bg, attrs: flag::DEFAULT_FG | (self.attrs & flag::DEFAULT_BG), width: 1 }
    }
}

pub(super) fn default_blank() -> Cell {
    Pen::DEFAULT.erased()
}

#[derive(Clone, Copy)]
pub(super) struct SavedCursor {
    pub(super) pos: Pos,
    pub(super) pen: Pen,
    pub(super) origin: bool,
    pub(super) wrap_pending: bool,
    pub(super) charsets: [Charset; 2],
    pub(super) shifted: bool,
}

#[derive(Clone)]
pub(super) struct Grid {
    pub(super) rows: Vec<Row>,
}

impl Grid {
    pub(super) fn new(size: Size, blank: Cell) -> Self {
        Self { rows: (0..size.height).map(|_| Row::blank(size.width, blank)).collect() }
    }

    pub(super) fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        self.rows.get(usize::from(y))?.cells.get(usize::from(x))
    }

    pub(super) fn put(&mut self, x: u16, y: u16, cell: Cell) {
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

    pub(super) fn fix_boundaries(&mut self, y: u16, x: u16) {
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
    pub(super) primary: Grid,
    pub(super) alt: Grid,
    pub alt_active: bool,
    pub(super) history: History,
    pub(super) anchor: Option<u64>,
    pub cursor: Pos,
    pub(super) saved: SavedCursor,
    pub(super) pen: Pen,
    pub scroll_top: u16,
    pub scroll_bottom: u16,
    pub modes: Modes,
    pub input: InputModes,
    pub(super) wrap_pending: bool,
    pub(super) tab_stops: Vec<bool>,
    pub(super) charsets: [Charset; 2],
    pub(super) shifted: bool,
    pub(super) last_char: Option<char>,
    pub title: Option<String>,
    pub cwd: Option<String>,
    pub palette: Palette,
    pub(super) replies: Vec<u8>,
    pub(super) clipboard: Option<String>,
    pub bells: u32,
    pub(super) parser: VtParser,
}

pub(super) fn default_tab_stops(width: u16) -> Vec<bool> {
    (0..width).map(|x| x % 8 == 0 && x != 0).collect()
}
