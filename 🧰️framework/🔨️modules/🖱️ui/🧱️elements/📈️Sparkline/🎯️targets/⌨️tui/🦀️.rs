//! 📈️ tui Sparkline: a fixed-width strip of Unicode block glyphs (one row of ▁▂▃▄▅▆▇█, or more rows for finer steps) over
//! a multi-resolution series. Each bucket carries min, max, mean and last; the optional band shades the min..max extent
//! of each column, and a gap (no data) has its own glyph. Pure function from data to a cell grid; `paint_sparkline` copies
//! the grid into a cell buffer. Wired as a crate-root module of the widgets crate.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧮️ChartScale/🎯️targets/⌨️tui/🦀️.rs

use crate::chart_scale::{blank_grid, columns, domain, eighths, extent, paint_grid, resolve_range, row_fill, Aggregate, Bucket, Mark, Range, Stats, XAxis};
use semio_ui_tui_core::cell::{attr, CellBuffer};
use semio_ui_tui_core::geometry::Rect;
use semio_ui_tui_core::theme::{GlyphSet, Role, Theme};

const BLOCK_RAMP: [char; 8] = ['\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}', '\u{2588}'];
const ASCII_RAMP: [char; 8] = ['.', ':', '-', '=', '+', '*', '#', '@'];

/// 🔲 What one cell of a sparkline holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SparkCell {
    Empty,
    Level(u8),
    Band,
    Gap,
}

impl SparkCell {
    /// 🔠 The glyph of this cell in a glyph repertoire.
    pub fn glyph(self, glyphs: GlyphSet) -> char {
        let unicode = glyphs == GlyphSet::Unicode;
        match self {
            SparkCell::Empty => ' ',
            SparkCell::Level(level) => (if unicode { BLOCK_RAMP } else { ASCII_RAMP })[usize::from(level.clamp(1, 8)) - 1],
            SparkCell::Band => if unicode { '\u{2591}' } else { ',' },
            SparkCell::Gap => if unicode { '\u{00b7}' } else { '?' },
        }
    }
}

/// 📈️ A series and how to draw it: one value per column (`aggregate`), the range it is scaled to and the optional band.
pub struct SparklineSpec<'a> {
    pub buckets: &'a [Bucket],
    pub axis: XAxis,
    pub range: Range,
    pub aggregate: Aggregate,
    pub band: bool,
    pub role: Role,
}

/// 🧮️ The cells of already resampled columns, top row first; `rows` rows give `8 * rows` steps. A band shows in the
/// empty cells between a column's min and max, so it needs two or more rows to be visible next to the value.
pub fn sparkline_cells(columns: &[Option<Stats>], low: f64, high: f64, aggregate: Aggregate, band: bool, rows: usize) -> Vec<Vec<SparkCell>> {
    let levels = rows * 8;
    let mut cells = vec![vec![SparkCell::Empty; columns.len()]; rows];
    for (x, column) in columns.iter().enumerate() {
        let Some(stats) = column else {
            if let Some(bottom) = cells.last_mut() {
                bottom[x] = SparkCell::Gap;
            }
            continue;
        };
        let height = eighths(stats.pick(aggregate), low, high, levels);
        let (band_low, band_high) = (eighths(stats.min, low, high, levels), eighths(stats.max, low, high, levels));
        for row in 0..rows {
            let fill = row_fill(height, row);
            cells[rows - 1 - row][x] = if fill > 0 {
                SparkCell::Level(fill as u8)
            } else if band && band_high > row * 8 && band_low <= row * 8 + 8 {
                SparkCell::Band
            } else {
                SparkCell::Empty
            };
        }
    }
    cells
}

/// 🧮️ The sparkline of a series as a `width` by `rows` grid; a series without data is all gaps.
pub fn sparkline_grid(spec: &SparklineSpec, width: usize, rows: usize, glyphs: GlyphSet) -> Vec<Vec<Mark>> {
    let mut grid = blank_grid(width, rows);
    if width == 0 || rows == 0 {
        return grid;
    }
    let sets = [spec.buckets];
    let resolved = resolve_range(spec.range, extent(&sets, spec.aggregate, spec.band));
    let sampled = match (domain(&sets, spec.axis), resolved) {
        (Some(domain), Some(_)) => columns(spec.buckets, spec.axis, domain, width),
        _ => vec![None; width],
    };
    let (low, high) = resolved.unwrap_or((0.0, 0.0));
    for (y, row) in sparkline_cells(&sampled, low, high, spec.aggregate, spec.band, rows).into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            grid[y][x] = match cell {
                SparkCell::Empty => Mark::blank(),
                SparkCell::Level(_) => Mark::new(cell.glyph(glyphs), spec.role),
                SparkCell::Band => Mark::new(cell.glyph(glyphs), Role::MutedForeground),
                SparkCell::Gap => Mark { ch: cell.glyph(glyphs), role: Role::MutedForeground, attrs: attr::DIM },
            };
        }
    }
    grid
}

/// 🖌️ Paints the sparkline into `rect`; the rect height is the number of rows.
pub fn paint_sparkline(spec: &SparklineSpec, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    paint_grid(&sparkline_grid(spec, usize::from(rect.width), usize::from(rect.height), theme.glyphs), theme, rect, buf);
}
