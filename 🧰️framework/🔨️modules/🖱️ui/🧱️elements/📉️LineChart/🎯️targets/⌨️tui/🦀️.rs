//! 📉️ tui LineChart: several multi-resolution series as Braille dots (2x4 per cell) or block bars, with min and max labels
//! on the y axis, an axis row with a tick at every tier boundary and the age of the oldest sample, the tier boundaries and
//! "now" below it. The x position is proportional to time, or to the bucket index with a separator at every tier
//! boundary. Pure function from data to a cell grid. Wired as a crate-root module of the widgets crate.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧮️ChartScale/🎯️targets/⌨️tui/🦀️.rs

use crate::chart_scale::{blank_grid, columns, dot_row, domain, extent, paint_grid, put_text, resolve_range, tier_columns, Aggregate, Bucket, ChartLabels, Mark, Range, Stats, XAxis};
use crate::sparkline::{sparkline_cells, SparkCell};
use crate::stat::{format_value, Unit};
use semio_ui_tui_core::cell::{attr, CellBuffer};
use semio_ui_tui_core::geometry::Rect;
use semio_ui_tui_core::theme::{GlyphSet, Role, Theme};

/// 〰️ How a series line is broken up so several series stay apart without colour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Dash {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl Dash {
    fn draws(self, column: usize) -> bool {
        match self {
            Dash::Solid => true,
            Dash::Dashed => column % 4 != 3,
            Dash::Dotted => column % 2 == 0,
        }
    }

    fn marker(self, glyphs: GlyphSet) -> char {
        match (self, glyphs) {
            (Dash::Solid, GlyphSet::Unicode) => '\u{2501}',
            (Dash::Dashed, GlyphSet::Unicode) => '\u{254d}',
            (Dash::Dotted, GlyphSet::Unicode) => '\u{2508}',
            (Dash::Solid, GlyphSet::Ascii) => '-',
            (Dash::Dashed, GlyphSet::Ascii) => '=',
            (Dash::Dotted, GlyphSet::Ascii) => '.',
        }
    }
}

/// 🔲 The mark a plot is drawn with: Braille dots (2 columns and 4 rows per cell) or block bars (1 column, 8 steps).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Marks {
    #[default]
    Braille,
    Blocks,
}

/// 📉️ One series of a chart.
pub struct ChartSeries<'a> {
    pub name: &'a str,
    pub buckets: &'a [Bucket],
    pub role: Role,
    pub dash: Dash,
    pub aggregate: Aggregate,
    pub band: bool,
}

/// 📉️ A chart: its series, the horizontal mapping, the vertical range and unit, the marks and the legend.
pub struct LineChartSpec<'a> {
    pub series: &'a [ChartSeries<'a>],
    pub axis: XAxis,
    pub range: Range,
    pub unit: Unit,
    pub marks: Marks,
    pub area: bool,
    pub legend: bool,
}

const DOT_BITS: [[u8; 4]; 2] = [[0x01, 0x02, 0x04, 0x40], [0x08, 0x10, 0x20, 0x80]];

fn braille(bits: u8) -> char {
    char::from_u32(0x2800 + u32::from(bits)).unwrap_or(' ')
}

fn series_domain(spec: &LineChartSpec, series: &ChartSeries, shared: (u64, u64)) -> (u64, u64) {
    match spec.axis {
        XAxis::Time => shared,
        XAxis::Index => domain(&[series.buckets], XAxis::Index).unwrap_or(shared),
    }
}

fn plot_braille(spec: &LineChartSpec, sampled: &[Vec<Option<Stats>>], low: f64, high: f64, width: usize, height: usize) -> Vec<Vec<Mark>> {
    let (dots_w, dots_h) = (width * 2, height * 4);
    let mut bits = vec![vec![0u8; width]; height];
    let mut owners: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    let mut set = |x: usize, y: usize, owner: usize| {
        bits[y / 4][x / 2] |= DOT_BITS[x % 2][y % 4];
        owners[y / 4][x / 2].get_or_insert(owner);
    };
    for (owner, (series, columns)) in spec.series.iter().zip(sampled).enumerate() {
        let mut previous: Option<usize> = None;
        for (x, column) in columns.iter().enumerate().take(dots_w) {
            let Some(stats) = column else {
                previous = None;
                continue;
            };
            let y = dot_row(stats.pick(series.aggregate), low, high, dots_h);
            let drawn = series.dash.draws(x);
            if drawn {
                let (from, to) = if spec.area { (y, dots_h - 1) } else { previous.map_or((y, y), |p| (p.min(y), p.max(y))) };
                (from..=to).for_each(|row| set(x, row, owner));
            }
            if series.band {
                let (top, bottom) = (dot_row(stats.max, low, high, dots_h), dot_row(stats.min, low, high, dots_h));
                (top..=bottom).filter(|row| (x + row) % 2 == 0).for_each(|row| set(x, row, owner));
            }
            previous = Some(y);
        }
    }
    (0..height).map(|y| (0..width).map(|x| owners[y][x].map_or_else(Mark::blank, |owner| Mark::new(braille(bits[y][x]), spec.series[owner].role))).collect()).collect()
}

fn plot_blocks(spec: &LineChartSpec, sampled: &[Vec<Option<Stats>>], low: f64, high: f64, width: usize, height: usize, glyphs: GlyphSet) -> Vec<Vec<Mark>> {
    let mut cells = blank_grid(width, height);
    let mut owners: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    for (owner, (series, columns)) in spec.series.iter().zip(sampled).enumerate() {
        let mut bars = sparkline_cells(columns, low, high, series.aggregate, series.band, height);
        for x in 0..width {
            let top = bars.iter().position(|row| matches!(row[x], SparkCell::Level(_)));
            for (y, row) in bars.iter_mut().enumerate() {
                if matches!(row[x], SparkCell::Gap) || !series.dash.draws(x) && !matches!(row[x], SparkCell::Band) || (!spec.area && matches!(row[x], SparkCell::Level(_)) && Some(y) != top) {
                    row[x] = SparkCell::Empty;
                }
                if row[x] != SparkCell::Empty && owners[y][x].is_none() {
                    owners[y][x] = Some(owner);
                    let role = if row[x] == SparkCell::Band { Role::MutedForeground } else { series.role };
                    cells[y][x] = Mark::new(row[x].glyph(glyphs), role);
                }
            }
        }
    }
    cells
}

fn fits(taken: &[(usize, usize)], start: usize, cells: usize) -> bool {
    taken.iter().all(|&(from, to)| start >= to + 1 || start + cells + 1 <= from)
}

/// 🧮️ The chart as a `width` by `height` grid: legend row, plot with y labels, axis row and the time label row.
pub fn line_chart_grid(spec: &LineChartSpec, width: usize, height: usize, labels: &ChartLabels, glyphs: GlyphSet) -> Vec<Vec<Mark>> {
    let mut grid = blank_grid(width, height);
    if width == 0 || height == 0 {
        return grid;
    }
    let sets: Vec<&[Bucket]> = spec.series.iter().map(|series| series.buckets).collect();
    let data = spec.series.iter().filter_map(|series| extent(&[series.buckets], series.aggregate, series.band)).reduce(|(lo, hi), (a, b)| (lo.min(a), hi.max(b)));
    let (Some((low, high)), Some(shared)) = (resolve_range(spec.range, data), domain(&sets, XAxis::Time)) else {
        put_text(&mut grid[height / 2], 0, labels.no_data.as_str(), Role::MutedForeground, attr::DIM);
        return grid;
    };
    let unicode = glyphs == GlyphSet::Unicode;
    let legend_rows = usize::from(spec.legend && height >= 4 && !spec.series.is_empty());
    let axis_rows = if height >= legend_rows + 3 { 2 } else { 0 };
    let plot_height = height - legend_rows - axis_rows;
    let (top_label, bottom_label) = (format_value(high, spec.unit, labels), format_value(low, spec.unit, labels));
    let label_width = top_label.chars().count().max(bottom_label.chars().count());
    let label_width = if width >= label_width + 1 + 4 { label_width } else { 0 };
    let plot_x = if label_width > 0 { label_width + 1 } else { 0 };
    let plot_width = width - plot_x;
    let per_cell = if spec.marks == Marks::Braille && unicode { 2 } else { 1 };
    let sampled: Vec<Vec<Option<Stats>>> = spec.series.iter().map(|series| columns(series.buckets, spec.axis, series_domain(spec, series, shared), plot_width * per_cell)).collect();
    let plot = if per_cell == 2 { plot_braille(spec, &sampled, low, high, plot_width, plot_height) } else { plot_blocks(spec, &sampled, low, high, plot_width, plot_height, glyphs) };
    let (vline, corner, hline, tick, gap_mark, separator) = if unicode { ('\u{2502}', '\u{2514}', '\u{2500}', '\u{2534}', '\u{254c}', '\u{250a}') } else { ('|', '+', '-', '+', '.', ':') };
    let first = spec.series.first();
    let boundaries = first.map_or_else(Vec::new, |series| tier_columns(series.buckets, spec.axis, series_domain(spec, series, shared), plot_width));
    for (y, row) in plot.into_iter().enumerate() {
        for (x, mark) in row.into_iter().enumerate() {
            let separated = spec.axis == XAxis::Index && mark.ch == ' ' && boundaries.iter().any(|&(column, _)| column == x && column > 0);
            grid[legend_rows + y][plot_x + x] = if separated { Mark::new(separator, Role::BorderNormal) } else { mark };
        }
    }
    if label_width > 0 {
        let pad = |text: &str| format!("{text:>label_width$}");
        put_text(&mut grid[legend_rows], 0, &pad(&top_label), Role::MutedForeground, 0);
        if plot_height >= 2 {
            put_text(&mut grid[legend_rows + plot_height - 1], 0, &pad(&bottom_label), Role::MutedForeground, 0);
        }
        for y in 0..plot_height {
            grid[legend_rows + y][label_width] = Mark::new(vline, Role::BorderNormal);
        }
    }
    if axis_rows == 2 {
        let axis_y = legend_rows + plot_height;
        let axis_x = if label_width > 0 { label_width } else { 0 };
        if label_width > 0 {
            grid[axis_y][axis_x] = Mark::new(corner, Role::BorderNormal);
        }
        for x in 0..plot_width {
            let gap = sampled.iter().all(|columns| columns[x * per_cell..(x + 1) * per_cell].iter().all(Option::is_none));
            grid[axis_y][plot_x + x] = if gap { Mark::new(gap_mark, Role::MutedForeground) } else { Mark::new(hline, Role::BorderNormal) };
        }
        for &(column, _) in &boundaries {
            grid[axis_y][plot_x + column] = Mark::new(tick, Role::BorderNormal);
        }
        let real = domain(&sets, XAxis::Time).unwrap_or(shared);
        let age = |start: u64| format!("-{}", format_value(real.1.saturating_sub(start) as f64, Unit::DurationMs, labels));
        let now = labels.now.as_str();
        let now_column = plot_width.saturating_sub(now.chars().count());
        let mut taken = vec![(now_column, plot_width)];
        let mut placed = vec![(now_column, now.to_string())];
        let mut wanted = vec![(0, age(real.0))];
        if let Some(series) = first {
            wanted.extend(boundaries.iter().map(|&(column, index)| (column, age(series.buckets[index].start))));
        }
        for (column, text) in wanted {
            let cells = text.chars().count();
            if column + cells <= plot_width && fits(&taken, column, cells) {
                taken.push((column, column + cells));
                placed.push((column, text));
            }
        }
        for (column, text) in placed {
            put_text(&mut grid[axis_y + 1], plot_x + column, &text, Role::MutedForeground, 0);
        }
    }
    if legend_rows == 1 {
        let mut column = 0;
        for series in spec.series {
            column += put_text(&mut grid[0], column, &series.dash.marker(glyphs).to_string(), series.role, 0);
            column += put_text(&mut grid[0], column + 1, series.name, Role::Foreground, 0) + 3;
        }
    }
    grid
}

/// 🖌️ Paints the chart into `rect`.
pub fn paint_line_chart(spec: &LineChartSpec, labels: &ChartLabels, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    paint_grid(&line_chart_grid(spec, usize::from(rect.width), usize::from(rect.height), labels, theme.glyphs), theme, rect, buf);
}
