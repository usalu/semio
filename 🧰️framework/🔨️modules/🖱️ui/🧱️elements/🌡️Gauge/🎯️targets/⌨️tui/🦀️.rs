//! 🌡️ tui Gauge: a value against a capacity as one row, `label bar value / capacity percent state`, with warn and critical
//! thresholds. The level is also written as a word and the thresholds as tick marks, so it never depends on colour alone.
//! Pure function from data to a cell grid. Wired as a crate-root module of the widgets crate.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧮️ChartScale/🎯️targets/⌨️tui/🦀️.rs

use crate::chart_scale::{blank_grid, level_label, level_of, paint_grid, put_text, ChartLabels, Level, Mark, Polarity};
use crate::stat::{format_value, level_role, Unit};
use semio_ui_tui_core::cell::{attr, CellBuffer};
use semio_ui_tui_core::geometry::Rect;
use semio_ui_tui_core::theme::{Glyph, GlyphSet, Role, Theme};

const EIGHTHS: [char; 8] = [' ', '\u{258f}', '\u{258e}', '\u{258d}', '\u{258c}', '\u{258b}', '\u{258a}', '\u{2589}'];

/// 🌡️ A value against a capacity; `warn` and `critical` are fractions of the capacity, inclusive.
pub struct GaugeSpec<'a> {
    pub label: &'a str,
    pub value: f64,
    pub capacity: f64,
    pub unit: Unit,
    pub warn: f64,
    pub critical: f64,
    pub polarity: Polarity,
}

/// 🚥 The fraction of the capacity in use and its level, or none while value or capacity is not usable.
pub fn gauge_state(spec: &GaugeSpec) -> Option<(f64, Level)> {
    let usable = spec.value.is_finite() && spec.capacity.is_finite() && spec.capacity > 0.0;
    usable.then(|| {
        let fraction = spec.value / spec.capacity;
        (fraction, level_of(fraction, spec.warn, spec.critical, spec.polarity))
    })
}

/// 🔢 The filled eighths of a bar `cells` wide; the fraction is clamped to 0..=1.
pub fn filled_eighths(fraction: f64, cells: usize) -> usize {
    (fraction.clamp(0.0, 1.0) * cells as f64 * 8.0 + 0.5).floor() as usize
}

fn clip(text: &str, cells: usize, ellipsis: &str) -> String {
    if text.chars().count() <= cells {
        text.to_string()
    } else {
        text.chars().take(cells.saturating_sub(1)).chain(ellipsis.chars()).collect()
    }
}

/// 🧮️ The gauge as a one row grid of `width` cells; narrow widths drop the numbers first and the bar last.
pub fn gauge_grid(spec: &GaugeSpec, width: usize, labels: &ChartLabels, glyphs: GlyphSet) -> Vec<Vec<Mark>> {
    let mut grid = blank_grid(width, 1);
    if width == 0 {
        return grid;
    }
    let ellipsis = glyphs.ellipsis();
    let label = clip(spec.label, width / 3, ellipsis);
    let label_cells = label.chars().count() + usize::from(!label.is_empty());
    put_text(&mut grid[0], 0, &label, Role::Foreground, 0);
    let Some((fraction, level)) = gauge_state(spec) else {
        put_text(&mut grid[0], label_cells, &clip(labels.no_data.as_str(), width.saturating_sub(label_cells), ellipsis), Role::MutedForeground, attr::DIM);
        return grid;
    };
    let state = level_label(level, labels);
    let percent = format_value(fraction * 100.0, Unit::Percent, labels);
    let numbers = format!(" {} / {}", format_value(spec.value, spec.unit, labels), format_value(spec.capacity, spec.unit, labels));
    let variants = [format!("{numbers} {percent} {state}"), format!(" {percent} {state}"), format!(" {state}"), String::new()];
    let available = width.saturating_sub(label_cells);
    let suffix = variants.iter().find(|suffix| available >= suffix.chars().count() + 4).unwrap_or(&variants[3]);
    let bar = available.saturating_sub(suffix.chars().count());
    let role = level_role(Some(level));
    let eighths = filled_eighths(fraction, bar);
    let (whole, rest) = (eighths / 8, eighths % 8);
    let unicode = glyphs == GlyphSet::Unicode;
    for column in 0..bar {
        let (ch, role) = match column.cmp(&whole) {
            std::cmp::Ordering::Less => (glyphs.glyph(Glyph::BarFull).chars().next().unwrap_or('#'), role),
            std::cmp::Ordering::Equal if rest > 0 && unicode => (EIGHTHS[rest], role),
            _ => (glyphs.glyph(Glyph::BarEmpty).chars().next().unwrap_or('-'), Role::BorderNormal),
        };
        grid[0][label_cells + column] = Mark::new(ch, role);
    }
    for threshold in [spec.warn, spec.critical] {
        let tick = (threshold.clamp(0.0, 1.0) * bar as f64) as usize;
        let slot = &mut grid[0][(label_cells + tick).min(width - 1)];
        if tick > 0 && tick < bar && slot.role == Role::BorderNormal {
            *slot = Mark::new(if unicode { '\u{254e}' } else { '|' }, Role::MutedForeground);
        }
    }
    put_text(&mut grid[0], label_cells + bar, suffix, Role::MutedForeground, 0);
    if suffix.ends_with(state) {
        let at = label_cells + bar + suffix.chars().count() - state.chars().count();
        put_text(&mut grid[0], at, state, role, attr::BOLD);
    }
    grid
}

/// 🖌️ Paints the gauge into the first row of `rect`.
pub fn paint_gauge(spec: &GaugeSpec, labels: &ChartLabels, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    paint_grid(&gauge_grid(spec, usize::from(rect.width), labels, theme.glyphs), theme, rect, buf);
}
