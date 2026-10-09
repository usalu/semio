use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::{char_cells, cluster_cells_in, clusters, scalar_mark, WidthMode};
use crate::tui::theme::Rgb;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// 🎨️ Bitflags for cell text attributes.
pub mod attr {
    pub const BOLD: u8 = 1;
    pub const DIM: u8 = 2;
    pub const ITALIC: u8 = 4;
    pub const UNDERLINE: u8 = 8;
    pub const REVERSE: u8 = 16;
}

/// 🔲️ One terminal cell: the first scalar of its grapheme cluster, colors, attributes, and cell width
/// (0 = continuation of the wide cluster to its left, 1 = narrow, 2 or more = lead of a wide cluster).
/// The remaining scalars of a multi-scalar cluster live beside the cell in its `CellBuffer`.
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

//#region 🧶️Cluster Tails
const TAIL_LIMIT: usize = 1 << 16;

struct Tails {
    ids: HashMap<Box<str>, u32>,
    texts: Vec<Box<str>>,
}

fn tails() -> &'static Mutex<Tails> {
    static TAILS: OnceLock<Mutex<Tails>> = OnceLock::new();
    TAILS.get_or_init(|| Mutex::new(Tails { ids: HashMap::new(), texts: Vec::new() }))
}

fn intern_tail(text: &str) -> u32 {
    let mut tails = tails().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(&id) = tails.ids.get(text) {
        return id;
    }
    if tails.texts.len() >= TAIL_LIMIT {
        return 0;
    }
    tails.texts.push(text.into());
    let id = tails.texts.len() as u32;
    tails.ids.insert(text.into(), id);
    id
}

fn push_tail(id: u32, out: &mut String) {
    let tails = tails().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(text) = id.checked_sub(1).and_then(|index| tails.texts.get(index as usize)) {
        out.push_str(text);
    }
}
//#endregion 🧶️Cluster Tails

/// 🧮️ A retained grid of `Cell`s whose wide and multi-scalar clusters stay whole.
#[derive(Clone)]
pub struct CellBuffer {
    pub size: Size,
    cells: Vec<Cell>,
    tails: Vec<u32>,
    mode: WidthMode,
}

impl CellBuffer {
    pub fn new(size: Size, fill: Cell) -> Self {
        let count = usize::from(size.width) * usize::from(size.height);
        Self { size, cells: vec![fill; count], tails: vec![0; count], mode: WidthMode::Cluster }
    }

    pub fn resize(&mut self, size: Size, fill: Cell) {
        let mode = self.mode;
        *self = Self::new(size, fill);
        self.mode = mode;
    }

    /// 🎚️ How `put_str` counts text; set it from the terminal's reported Unicode support.
    pub fn width_mode(&self) -> WidthMode {
        self.mode
    }

    /// 🪢 Also makes `mode` the active width mode of the thread, so `text::display_width`, truncation, cursor columns and
    /// hit tests count exactly as this buffer paints.
    pub fn set_width_mode(&mut self, mode: WidthMode) {
        self.mode = mode;
        crate::tui::text::set_active_width_mode(mode);
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

    /// 🧶️ The id of the trailing scalars of the cluster whose lead cell is at `(x, y)`; 0 when it has none.
    pub fn tail_id(&self, x: u16, y: u16) -> u32 {
        self.index(x, y).map_or(0, |i| self.tails[i])
    }

    /// 🔤️ Appends the whole cluster drawn by the cell at `(x, y)` to `out`; a continuation or NUL cell is a space.
    pub fn push_glyph(&self, x: u16, y: u16, out: &mut String) {
        let Some(i) = self.index(x, y) else { return };
        let cell = &self.cells[i];
        out.push(if cell.ch == '\0' { ' ' } else { cell.ch });
        if self.tails[i] != 0 {
            push_tail(self.tails[i], out);
        }
    }

    /// 📜️ The text of row `y` with continuation cells skipped and every cluster whole.
    pub fn row_text(&self, y: u16) -> String {
        let mut out = String::new();
        for x in 0..self.size.width {
            if self.get(x, y).is_some_and(|cell| cell.width != 0) {
                self.push_glyph(x, y, &mut out);
            }
        }
        out
    }

    fn blank_at(&mut self, x: u16, y: u16, blank: Cell) {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = blank;
            self.tails[i] = 0;
        }
    }

    fn lead_of(&self, x: u16, y: u16) -> Option<u16> {
        let row = usize::from(y) * usize::from(self.size.width);
        let mut lead = x;
        while lead > 0 && self.cells[row + usize::from(lead)].width == 0 {
            lead -= 1;
        }
        let covers = self.cells[row + usize::from(lead)].width >= 2 && u16::from(self.cells[row + usize::from(lead)].width) > x - lead;
        (lead < x && covers).then_some(lead)
    }

    fn release(&mut self, x: u16, y: u16, like: Cell) {
        let Some(i) = self.index(x, y) else { return };
        let old = self.cells[i];
        let blank = Cell::blank(like.fg, like.bg);
        if old.width >= 2 {
            for offset in 1..u16::from(old.width) {
                self.blank_at(x + offset, y, blank);
            }
        } else if old.width == 0 {
            if let Some(lead) = self.lead_of(x, y) {
                let width = u16::from(self.cells[usize::from(y) * usize::from(self.size.width) + usize::from(lead)].width);
                for covered in lead..lead + width {
                    if covered != x {
                        self.blank_at(covered, y, blank);
                    }
                }
            }
        }
    }

    fn write(&mut self, x: u16, y: u16, cell: Cell, tail: u32) {
        let Some(i) = self.index(x, y) else { return };
        let width = u16::from(cell.width);
        if width == 0 {
            if self.lead_of(x, y).is_some() {
                self.cells[i] = cell;
                self.tails[i] = 0;
            } else {
                self.release(x, y, cell);
                self.cells[i] = Cell { ch: ' ', width: 1, ..cell };
                self.tails[i] = 0;
            }
            return;
        }
        if width >= 2 && x + width > self.size.width {
            self.release(x, y, cell);
            self.cells[i] = Cell { ch: ' ', width: 1, ..cell };
            self.tails[i] = 0;
            return;
        }
        for covered in x..x + width {
            self.release(covered, y, cell);
        }
        self.cells[i] = cell;
        self.tails[i] = tail;
        for covered in x + 1..x + width {
            if let Some(next) = self.index(covered, y) {
                self.cells[next] = Cell { ch: '\0', width: 0, ..cell };
                self.tails[next] = 0;
            }
        }
    }

    /// ✍️ Writes one cell; a wide lead claims its continuation cells and any wide cluster it cuts in half is blanked.
    pub fn put(&mut self, x: u16, y: u16, cell: Cell) {
        self.write(x, y, cell, 0);
    }

    /// 🧩️ Writes one whole grapheme cluster at `at`; `template` supplies the colours, attributes and cell width, its character is replaced by the cluster's lead scalar.
    pub fn put_cluster(&mut self, at: Pos, cluster: &str, template: Cell) {
        let mut chars = cluster.chars();
        let Some(lead) = chars.next() else { return };
        let rest = chars.as_str();
        let tail = if rest.is_empty() { 0 } else if self.mode == WidthMode::Scalar { let marks: String = rest.chars().filter(|c| scalar_mark(*c)).collect(); if marks.is_empty() { 0 } else { intern_tail(&marks) } } else { intern_tail(rest) };
        self.write(at.x, at.y, Cell { ch: lead, ..template }, tail);
    }

    /// 🖋️ Writes a string starting at `pos`, clipped to `clip`; returns cells consumed.
    pub fn put_str(&mut self, pos: Pos, s: &str, fg: Rgb, bg: Rgb, attrs: u8, clip: Rect) -> u16 {
        let mut x = pos.x;
        let mut written = 0u16;
        if pos.y < clip.y || pos.y >= clip.y + clip.height {
            return 0;
        }
        for cluster in clusters(s) {
            if self.mode == WidthMode::Scalar {
                let mut scalars = cluster.chars().peekable();
                while let Some(c) = scalars.next() {
                    let w = char_cells(c);
                    if w == 0 {
                        continue;
                    }
                    let mut marks = String::new();
                    while scalars.peek().is_some_and(|c| char_cells(*c) == 0) { let mark = scalars.next().unwrap(); if scalar_mark(mark) { marks.push(mark); } }
                    if x + u16::from(w) > clip.x + clip.width {
                        return written;
                    }
                    if x >= clip.x {
                        self.write(x, pos.y, Cell { ch: c, fg, bg, attrs, width: w }, if marks.is_empty() { 0 } else { intern_tail(&marks) });
                    }
                    x += u16::from(w);
                    written += u16::from(w);
                }
                continue;
            }
            let w = cluster_cells_in(cluster, WidthMode::Cluster);
            if w == 0 {
                continue;
            }
            if x + u16::from(w) > clip.x + clip.width {
                break;
            }
            if x >= clip.x {
                self.put_cluster(Pos { x, y: pos.y }, cluster, Cell { ch: ' ', fg, bg, attrs, width: w });
            }
            x += u16::from(w);
            written += u16::from(w);
        }
        written
    }

    /// 🖍️ Restyles the cells of `rect` in place; glyphs, continuation pairing and cluster tails stay untouched.
    pub fn restyle(&mut self, rect: Rect, mut apply: impl FnMut(&mut Cell)) {
        let clipped = Rect::new(0, 0, self.size.width, self.size.height).intersect(rect);
        for y in clipped.y..clipped.y + clipped.height {
            for x in clipped.x..clipped.x + clipped.width {
                if let Some(i) = self.index(x, y) {
                    apply(&mut self.cells[i]);
                }
            }
        }
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

/// 🧱️ A contiguous run of changed cells on one row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffRun {
    pub y: u16,
    pub x: u16,
    pub len: u16,
}

/// 🔀 Computes the minimal set of changed-cell runs between two buffers; buffers of different size repaint every row in full.
pub fn diff(prev: &CellBuffer, next: &CellBuffer) -> Vec<DiffRun> {
    const MERGE_GAP: u16 = 4;
    let mut runs = Vec::new();
    if prev.size != next.size {
        return (0..next.size.height).map(|y| DiffRun { y, x: 0, len: next.size.width }).collect();
    }
    let width = usize::from(next.size.width);
    for y in 0..next.size.height {
        let row = usize::from(y) * width;
        let mut run_start: Option<u16> = None;
        let mut last_diff: Option<u16> = None;
        for x in 0..next.size.width {
            let i = row + usize::from(x);
            let changed = prev.cells[i] != next.cells[i] || prev.tails[i] != next.tails[i];
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
