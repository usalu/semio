use crate::tui::cell::Cell;
use std::collections::VecDeque;

/// 📏️ One screen row; `wrapped` says the text continues on the next row without a line break.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub wrapped: bool,
}

impl Row {
    pub fn blank(width: u16, cell: Cell) -> Self {
        Self { cells: vec![cell; usize::from(width)], wrapped: false }
    }

    /// 🫥️ Whether every cell is an unstyled space, so trimming it loses nothing.
    pub fn is_blank(&self) -> bool {
        self.cells.iter().all(is_blank_cell)
    }

    /// ✂️ The cell count up to the last cell that carries content.
    pub fn content_len(&self) -> usize {
        self.cells.iter().rposition(|cell| !is_blank_cell(cell)).map_or(0, |index| index + 1)
    }
}

fn is_blank_cell(cell: &Cell) -> bool {
    cell.ch == ' ' && cell.attrs & !super::palette::flag::MASK == 0 && cell.attrs & super::palette::flag::DEFAULT_BG != 0
}

/// 📚️ Rows that scrolled off the primary screen, numbered by absolute row id so a viewport can stay put.
pub struct History {
    rows: VecDeque<Row>,
    base: u64,
    cap: usize,
}

impl History {
    pub fn new(cap: usize) -> Self {
        Self { rows: VecDeque::new(), base: 0, cap: cap.max(1) }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// 🏷️ The absolute id of the oldest retained row.
    pub fn base(&self) -> u64 {
        self.base
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn push(&mut self, row: Row) {
        if self.rows.len() >= self.cap {
            self.rows.pop_front();
            self.base += 1;
        }
        self.rows.push_back(row);
    }

    pub fn get(&self, abs: u64) -> Option<&Row> {
        let index = usize::try_from(abs.checked_sub(self.base)?).ok()?;
        self.rows.get(index)
    }

    /// 🧹️ Drops every row while keeping later ids monotonic.
    pub fn clear(&mut self) {
        self.base += self.rows.len() as u64;
        self.rows.clear();
    }

    pub fn take_rows(&mut self) -> Vec<Row> {
        self.rows.drain(..).collect()
    }

    /// 🔁️ Replaces the history with `rows`, oldest first, trimming to the cap.
    pub fn replace(&mut self, rows: Vec<Row>) {
        let excess = rows.len().saturating_sub(self.cap);
        self.base += excess as u64;
        self.rows = rows.into_iter().skip(excess).collect();
    }
}

/// 📍️ A spot that must survive a reflow: a row index in the old layout and a column in that row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spot {
    pub row: usize,
    pub col: u16,
}

/// 🏞️ The re-wrapped rows plus where each tracked `Spot` landed; a column equal to the new width means "just past the last cell".
pub struct Reflowed {
    pub rows: Vec<Row>,
    pub spots: Vec<Spot>,
}

struct Line {
    cells: Vec<Cell>,
    row_starts: Vec<usize>,
    first_row: usize,
}

fn unit_len(cells: &[Cell], index: usize) -> usize {
    if cells[index].width == 2 && cells.get(index + 1).is_some_and(|next| next.width == 0) {
        2
    } else {
        1
    }
}

/// 🌊️ Re-wraps `rows` (oldest first) to `width` columns, joining soft-wrapped rows into lines and splitting them again, and maps `spots` through it.
pub fn reflow(rows: Vec<Row>, width: u16, spots: &[Spot], blank: Cell) -> Reflowed {
    let width = width.max(1);
    let last = rows.len().saturating_sub(1);
    let mut lines: Vec<Line> = Vec::new();
    let mut current: Option<Line> = None;
    for (index, row) in rows.into_iter().enumerate() {
        let line = current.get_or_insert_with(|| Line { cells: Vec::new(), row_starts: Vec::new(), first_row: index });
        line.row_starts.push(line.cells.len());
        let continues = row.wrapped && index != last;
        line.cells.extend(row.cells);
        if !continues {
            lines.extend(current.take());
        }
    }
    lines.extend(current.take());

    let mut pending: Vec<Vec<(usize, usize)>> = vec![Vec::new(); lines.len()];
    for (spot_index, spot) in spots.iter().enumerate() {
        let line_index = lines.partition_point(|line| line.first_row <= spot.row).saturating_sub(1);
        if let Some(line) = lines.get(line_index) {
            let start = line.row_starts.get(spot.row - line.first_row).copied().unwrap_or(0);
            pending[line_index].push((start + usize::from(spot.col), spot_index));
        }
    }

    let mut out: Vec<Row> = Vec::new();
    let mut mapped = vec![Spot { row: 0, col: 0 }; spots.len()];
    for (line_index, line) in lines.into_iter().enumerate() {
        let mut cells = line.cells;
        let mut spots_here = std::mem::take(&mut pending[line_index]);
        spots_here.sort_unstable();
        let keep = spots_here.iter().map(|(offset, _)| *offset).max().unwrap_or(0);
        let content = cells.iter().rposition(|cell| !is_blank_cell(cell)).map_or(0, |index| index + 1);
        cells.truncate(content.max(keep));
        let mut row = Row::blank(width, blank);
        let mut col: u16 = 0;
        let mut index = 0;
        let mut next = 0;
        while index < cells.len() {
            let unit = unit_len(&cells, index);
            let mut cell = cells[index];
            if (cell.width == 2 && (unit == 1 || width < 2)) || cell.width == 0 {
                cell = Cell { ch: ' ', width: 1, ..cell };
            }
            let cell_width = u16::from(cell.width.max(1));
            if col + cell_width > width {
                row.wrapped = true;
                out.push(std::mem::replace(&mut row, Row::blank(width, blank)));
                col = 0;
            }
            while next < spots_here.len() && spots_here[next].0 < index + unit {
                mapped[spots_here[next].1] = Spot { row: out.len(), col };
                next += 1;
            }
            row.cells[usize::from(col)] = cell;
            if cell.width == 2 && unit == 2 {
                row.cells[usize::from(col) + 1] = cells[index + 1];
            }
            col += cell_width;
            index += unit;
        }
        while next < spots_here.len() {
            let beyond = (spots_here[next].0 - cells.len()) as u64;
            mapped[spots_here[next].1] = Spot { row: out.len(), col: (u64::from(col) + beyond).min(u64::from(width)) as u16 };
            next += 1;
        }
        out.push(row);
    }
    Reflowed { rows: out, spots: mapped }
}

