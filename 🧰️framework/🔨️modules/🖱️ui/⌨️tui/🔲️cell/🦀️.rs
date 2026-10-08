use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::char_cells;
use crate::tui::theme::Rgb;

/// ??? Bitflags for cell text attributes.
pub mod attr {
    pub const BOLD: u8 = 1;
    pub const DIM: u8 = 2;
    pub const ITALIC: u8 = 4;
    pub const UNDERLINE: u8 = 8;
    pub const REVERSE: u8 = 16;
}

/// ??? One terminal cell: a glyph, its colors, attributes, and cell width (0 = wide-char continuation).
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

/// ??? A retained grid of `Cell`s.
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

    /// ?? Writes one cell, blanking an orphaned wide-char continuation on either side.
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

    /// ?? Writes a string starting at `pos`, clipped to `clip`; returns cells consumed.
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

/// ??? A contiguous run of changed cells on one row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffRun {
    pub y: u16,
    pub x: u16,
    pub len: u16,
}

/// ??? Computes the minimal set of changed-cell runs between two same-sized buffers.
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
