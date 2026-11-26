//! 🔢️ tui StatCell: a compact title and value with unit formatting (bytes, percent, count, rate per second, durations),
//! optionally flagged with a level. `format_value` is the single formatter every chart axis and gauge uses, and the
//! Python oracle (`🧰️scripts/M1/oracle.py`) re-implements it. Wired as a crate-root module of the widgets crate.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧮️ChartScale/🎯️targets/⌨️tui/🦀️.rs

use crate::chart_scale::{blank_grid, paint_grid, put_text, ChartLabels, Level, Mark};
use semio_ui_tui_core::cell::{attr, CellBuffer};
use semio_ui_tui_core::geometry::Rect;
use semio_ui_tui_core::theme::{Glyph, GlyphSet, Role, Theme};

/// 📏 What a number measures; decides the suffix and the scaling steps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Count,
    Bytes,
    Percent,
    Rate,
    ByteRate,
    DurationMs,
}

const COUNT_STEPS: [&str; 5] = ["", "k", "M", "G", "T"];
const BYTE_STEPS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

fn digits(magnitude: f64) -> usize {
    if magnitude >= 100.0 {
        0
    } else if magnitude >= 10.0 {
        1
    } else {
        2
    }
}

fn scaled(magnitude: f64, base: f64, steps: &[&str], whole_first_step: bool) -> String {
    let mut step = 0usize;
    while step + 1 < steps.len() && magnitude >= base.powi(step as i32 + 1) {
        step += 1;
    }
    loop {
        let value = magnitude / base.powi(step as i32);
        let places = if step == 0 && (whole_first_step || value.fract() == 0.0) { 0 } else { digits(value) };
        let text = format!("{value:.places$}");
        if step + 1 < steps.len() && text.parse::<f64>().is_ok_and(|rounded| rounded >= base) {
            step += 1;
            continue;
        }
        break format!("{text}{}", steps[step]);
    }
}

fn duration(total_ms: u64, labels: &ChartLabels) -> String {
    let (ms, s, min, h, d) = (labels.unit_ms.as_str(), labels.unit_s.as_str(), labels.unit_min.as_str(), labels.unit_h.as_str(), labels.unit_d.as_str());
    let pair = |major: u64, major_unit: &str, minor: u64, minor_unit: &str| if minor == 0 { format!("{major}{major_unit}") } else { format!("{major}{major_unit} {minor}{minor_unit}") };
    match total_ms {
        0..=999 => format!("{total_ms}{ms}"),
        1000..=9999 => format!("{}.{}{s}", total_ms / 1000, total_ms % 1000 / 100),
        10_000..=59_999 => format!("{}{s}", total_ms / 1000),
        60_000..=3_599_999 => pair(total_ms / 60_000, min, total_ms % 60_000 / 1000, s),
        3_600_000..=86_399_999 => pair(total_ms / 3_600_000, h, total_ms % 3_600_000 / 60_000, min),
        _ => pair(total_ms / 86_400_000, d, total_ms % 86_400_000 / 3_600_000, h),
    }
}

/// 🔤️ A number with its unit in the decimal separator of the labels' locale; a non-finite number is the "no value" mark.
pub fn format_value(value: f64, unit: Unit, labels: &ChartLabels) -> String {
    if !value.is_finite() {
        return labels.no_value.as_str().to_string();
    }
    let magnitude = value.abs();
    let body = match unit {
        Unit::Count => scaled(magnitude, 1000.0, &COUNT_STEPS, false),
        Unit::Bytes => scaled(magnitude, 1024.0, &BYTE_STEPS, true),
        Unit::Percent => {
            let places = usize::from(magnitude < 10.0);
            format!("{magnitude:.places$}%")
        }
        Unit::Rate => format!("{}/s", scaled(magnitude, 1000.0, &COUNT_STEPS, false)),
        Unit::ByteRate => format!("{}/s", scaled(magnitude, 1024.0, &BYTE_STEPS, true)),
        Unit::DurationMs => duration(magnitude.round() as u64, labels),
    };
    let sign = if value < 0.0 { "-" } else { "" };
    format!("{sign}{body}").replace('.', labels.decimal.as_str())
}

/// 🎨️ The colour role of a level; no level is the plain foreground.
pub fn level_role(level: Option<Level>) -> Role {
    match level {
        None => Role::Foreground,
        Some(Level::Ok) => Role::Success,
        Some(Level::Warn) => Role::Warning,
        Some(Level::Critical) => Role::Danger,
    }
}

/// 🔢️ What a stat cell shows: a title and a value, `None` while there is no value.
pub struct StatSpec<'a> {
    pub title: &'a str,
    pub value: Option<f64>,
    pub unit: Unit,
    pub level: Option<Level>,
}

fn clip(text: &str, cells: usize, ellipsis: &str) -> String {
    if text.chars().count() <= cells {
        text.to_string()
    } else {
        text.chars().take(cells.saturating_sub(1)).chain(ellipsis.chars()).collect()
    }
}

/// 🧮️ The cell grid of a stat: one row is `title … value`, two or more rows stack the value under the title.
pub fn stat_grid(spec: &StatSpec, width: usize, height: usize, labels: &ChartLabels, glyphs: GlyphSet) -> Vec<Vec<Mark>> {
    let mut grid = blank_grid(width, height);
    let flag = match spec.level {
        Some(Level::Warn) => format!(" {}", glyphs.glyph(Glyph::Warning)),
        Some(Level::Critical) => format!(" {}", glyphs.glyph(Glyph::Fault)),
        _ => String::new(),
    };
    let value = spec.value.map_or_else(|| labels.no_value.as_str().to_string(), |value| format_value(value, spec.unit, labels)) + &flag;
    let value_role = if spec.value.is_some() { level_role(spec.level) } else { Role::MutedForeground };
    let ellipsis = glyphs.ellipsis();
    match height {
        0 => {}
        1 => {
            let value = clip(&value, width, ellipsis);
            let value_cells = value.chars().count();
            let title = clip(spec.title, width.saturating_sub(value_cells + 1), ellipsis);
            put_text(&mut grid[0], 0, &title, Role::MutedForeground, 0);
            put_text(&mut grid[0], width.saturating_sub(value_cells), &value, value_role, attr::BOLD);
        }
        _ => {
            put_text(&mut grid[0], 0, &clip(spec.title, width, ellipsis), Role::MutedForeground, 0);
            put_text(&mut grid[1], 0, &clip(&value, width, ellipsis), value_role, attr::BOLD);
        }
    }
    grid
}

/// 🖌️ Paints the stat into `rect`.
pub fn paint_stat(spec: &StatSpec, labels: &ChartLabels, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    paint_grid(&stat_grid(spec, usize::from(rect.width), usize::from(rect.height), labels, theme.glyphs), theme, rect, buf);
}
