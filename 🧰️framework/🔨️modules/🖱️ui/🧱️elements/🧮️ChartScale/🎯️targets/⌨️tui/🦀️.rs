//! 🧮️ tui chart scale: the domain-neutral data model and the scaling math shared by the Sparkline, LineChart, Gauge and
//! StatCell elements. Pure functions from data to numbers, no painting. The Python oracle in the monitoring ticket
//! (`🧰️scripts/M1/oracle.py`) re-implements every function here and the fixture `🧫️fixtures/📈️charts/🔣️.json` pins both.
//!
//! @see 🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧩️labels/🦀️.rs

pub use ui_locale::Locale;
use semio_ui_tui_core::cell::{Cell, CellBuffer};
use semio_ui_tui_core::geometry::{Pos, Rect};
use semio_ui_tui_core::theme::{Role, Theme};
use ui_locale::{AppLabels, Terminology};

ui_locale::app_labels! {
    pub struct ChartLabels {
        no_data: native_en "no data", native_de "keine Daten", reuse_en "no data", reuse_de "keine Daten";
        no_value: native_en "-", native_de "-", reuse_en "-", reuse_de "-";
        now: native_en "now", native_de "jetzt", reuse_en "now", reuse_de "jetzt";
        level_ok: native_en "ok", native_de "ok", reuse_en "ok", reuse_de "ok";
        level_warn: native_en "warn", native_de "Warnung", reuse_en "warn", reuse_de "Warnung";
        level_critical: native_en "critical", native_de "kritisch", reuse_en "critical", reuse_de "kritisch";
        decimal: native_en ".", native_de ",", reuse_en ".", reuse_de ",";
        unit_ms: native_en "ms", native_de "ms", reuse_en "ms", reuse_de "ms";
        unit_s: native_en "s", native_de "s", reuse_en "s", reuse_de "s";
        unit_min: native_en "min", native_de "Min", reuse_en "min", reuse_de "Min";
        unit_h: native_en "h", native_de "Std", reuse_en "h", reuse_de "Std";
        unit_d: native_en "d", native_de "T", reuse_en "d", reuse_de "T";
    }
}

/// 🏷️ The chart label set of a locale; no locale is assumed.
pub fn chart_labels(locale: Locale) -> &'static ChartLabels {
    <ChartLabels as AppLabels>::labels(locale, Terminology::Native)
}

/// 📊 What one bucket of a multi-resolution series knows about its time span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub last: f64,
}

impl Stats {
    /// 📍 A bucket that saw exactly one value.
    pub fn point(value: f64) -> Self {
        Self { min: value, max: value, mean: value, last: value }
    }

    fn finite(self) -> bool {
        self.min.is_finite() && self.max.is_finite() && self.mean.is_finite() && self.last.is_finite()
    }

    /// 🎯 The value a line or bar shows for this bucket.
    pub fn pick(self, aggregate: Aggregate) -> f64 {
        match aggregate {
            Aggregate::Mean => self.mean,
            Aggregate::Last => self.last,
            Aggregate::Min => self.min,
            Aggregate::Max => self.max,
        }
    }
}

/// 🪣 One half-open time bucket `[start, end)` in milliseconds; `stats` is `None` for a gap (no data).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bucket {
    pub start: u64,
    pub end: u64,
    pub stats: Option<Stats>,
}

impl Bucket {
    pub fn new(start: u64, end: u64, stats: Stats) -> Self {
        Self { start, end, stats: Some(stats) }
    }

    pub fn gap(start: u64, end: u64) -> Self {
        Self { start, end, stats: None }
    }

    fn valid(&self) -> Option<Stats> {
        self.stats.filter(|stats| stats.finite())
    }

    fn effective_end(&self) -> u64 {
        self.end.max(self.start.saturating_add(1))
    }
}

/// 🎯 Which statistic of a bucket is drawn as the value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Aggregate {
    #[default]
    Mean,
    Last,
    Min,
    Max,
}

/// 📏 The value range a chart maps to its height: derived from the data or given.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Range {
    #[default]
    Auto,
    Fixed(f64, f64),
}

/// ⏱️ How a bucket maps to the horizontal axis: proportional to its time, or one equal share per bucket index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum XAxis {
    #[default]
    Time,
    Index,
}

/// 🚦 How a value compares with its thresholds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Ok,
    Warn,
    Critical,
}

/// 🧭 Whether a high or a low value is the bad one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Polarity {
    #[default]
    HighIsBad,
    LowIsBad,
}

/// 🚥 The level of a fraction of capacity; thresholds are inclusive and given as fractions.
pub fn level_of(fraction: f64, warn: f64, critical: f64, polarity: Polarity) -> Level {
    let (crossed_critical, crossed_warn) = match polarity {
        Polarity::HighIsBad => (fraction >= critical, fraction >= warn),
        Polarity::LowIsBad => (fraction <= critical, fraction <= warn),
    };
    if crossed_critical {
        Level::Critical
    } else if crossed_warn {
        Level::Warn
    } else {
        Level::Ok
    }
}

/// 🏷️ The localized word of a level.
pub fn level_label(level: Level, labels: &ChartLabels) -> &'static str {
    match level {
        Level::Ok => labels.level_ok.as_str(),
        Level::Warn => labels.level_warn.as_str(),
        Level::Critical => labels.level_critical.as_str(),
    }
}

/// 📐 The extent of the drawn statistic over all buckets (and the whole min..max band when `band`), or none without data.
pub fn extent(sets: &[&[Bucket]], aggregate: Aggregate, band: bool) -> Option<(f64, f64)> {
    let mut found: Option<(f64, f64)> = None;
    for stats in sets.iter().flat_map(|set| set.iter()).filter_map(Bucket::valid) {
        let (low, high) = if band { (stats.min.min(stats.pick(aggregate)), stats.max.max(stats.pick(aggregate))) } else { (stats.pick(aggregate), stats.pick(aggregate)) };
        found = Some(found.map_or((low, high), |(lo, hi)| (lo.min(low), hi.max(high))));
    }
    found
}

/// 📏 The range a chart uses: the given one when it is finite, else the data extent.
pub fn resolve_range(range: Range, data: Option<(f64, f64)>) -> Option<(f64, f64)> {
    match range {
        Range::Fixed(a, b) if a.is_finite() && b.is_finite() => Some((a.min(b), a.max(b))),
        _ => data,
    }
}

/// 🔢 The quantized height of `value` in `1..=levels` (one level is "present, at the bottom"); a flat range sits in the middle.
pub fn eighths(value: f64, low: f64, high: f64, levels: usize) -> usize {
    if high <= low || levels == 0 {
        return levels.div_ceil(2).max(1);
    }
    let norm = ((value - low) / (high - low)).clamp(0.0, 1.0);
    1 + (norm * (levels - 1) as f64 + 0.5).floor() as usize
}

/// 🔲 How many of the 8 sub-rows of row `row` (counted from the bottom) a bar of height `height` fills.
pub fn row_fill(height: usize, row: usize) -> usize {
    height.saturating_sub(row * 8).min(8)
}

/// 🎯 The dot row (0 = top) of `value` on a canvas `dots` rows high; a flat range sits in the middle.
pub fn dot_row(value: f64, low: f64, high: f64, dots: usize) -> usize {
    let norm = if high <= low { 0.5 } else { ((value - low) / (high - low)).clamp(0.0, 1.0) };
    ((1.0 - norm) * dots.saturating_sub(1) as f64 + 0.5).floor() as usize
}

/// 🧭 The horizontal domain `[v0, v1)` of the given series: shared time span, or the bucket count of the first series.
pub fn domain(sets: &[&[Bucket]], axis: XAxis) -> Option<(u64, u64)> {
    match axis {
        XAxis::Index => sets.first().filter(|set| !set.is_empty()).map(|set| (0, set.len() as u64)),
        XAxis::Time => {
            let all = sets.iter().flat_map(|set| set.iter());
            let (start, end) = all.fold((u64::MAX, 0u64), |(s, e), bucket| (s.min(bucket.start), e.max(bucket.effective_end())));
            (start != u64::MAX).then(|| (start, end.max(start.saturating_add(1))))
        }
    }
}

fn virtual_span(index: usize, bucket: &Bucket, axis: XAxis) -> (u64, u64) {
    match axis {
        XAxis::Time => (bucket.start, bucket.effective_end()),
        XAxis::Index => (index as u64, index as u64 + 1),
    }
}

/// 🪜 Resamples the buckets onto `width` equal columns of the domain; a column with no data in it is a gap.
/// A column aggregates the buckets overlapping it: min of mins, max of maxes, overlap-weighted mean, last of the latest.
pub fn columns(buckets: &[Bucket], axis: XAxis, domain: (u64, u64), width: usize) -> Vec<Option<Stats>> {
    let (v0, v1) = domain;
    let span = u128::from(v1.saturating_sub(v0).max(1));
    let mut first = 0usize;
    (0..width)
        .map(|column| {
            let c0 = u128::from(v0) + span * column as u128 / width as u128;
            let c1 = (u128::from(v0) + span * (column as u128 + 1) / width as u128).max(c0 + 1);
            while first < buckets.len() && u128::from(virtual_span(first, &buckets[first], axis).1) <= c0 {
                first += 1;
            }
            let mut found: Option<(Stats, f64, f64)> = None;
            for (index, bucket) in buckets.iter().enumerate().skip(first) {
                let (start, end) = virtual_span(index, bucket, axis);
                if u128::from(start) >= c1 {
                    break;
                }
                let overlap = u128::from(end).min(c1).saturating_sub(u128::from(start).max(c0));
                let Some(stats) = bucket.valid().filter(|_| overlap > 0) else { continue };
                let weight = overlap as f64;
                found = Some(match found {
                    None => (stats, weight, stats.mean * weight),
                    Some((acc, total, sum)) => (Stats { min: acc.min.min(stats.min), max: acc.max.max(stats.max), mean: 0.0, last: stats.last }, total + weight, sum + stats.mean * weight),
                });
            }
            found.map(|(acc, total, sum)| Stats { mean: sum / total, ..acc })
        })
        .collect()
}

/// 🪜 The tiers of a series: the bucket indices where the bucket duration changes (dense recent, coarse older).
pub fn tier_starts(buckets: &[Bucket]) -> Vec<usize> {
    (1..buckets.len()).filter(|&index| buckets[index].end.saturating_sub(buckets[index].start) != buckets[index - 1].end.saturating_sub(buckets[index - 1].start)).collect()
}

/// 📍 The column of a domain position on a `width` wide axis.
pub fn position_column(position: u64, domain: (u64, u64), width: usize) -> usize {
    let span = u128::from(domain.1.saturating_sub(domain.0).max(1));
    ((u128::from(position.saturating_sub(domain.0)) * width as u128 / span) as usize).min(width.saturating_sub(1))
}

/// 📍 Each tier boundary of the series as its column on a `width` wide axis, with the bucket index it starts at.
pub fn tier_columns(buckets: &[Bucket], axis: XAxis, domain: (u64, u64), width: usize) -> Vec<(usize, usize)> {
    tier_starts(buckets).into_iter().map(|index| (position_column(virtual_span(index, &buckets[index], axis).0, domain, width), index)).collect()
}

/// 🧹 Sorts buckets by start so they satisfy the ascending, non-overlapping contract of this module.
pub fn normalize(buckets: &[Bucket]) -> Vec<Bucket> {
    let mut sorted = buckets.to_vec();
    sorted.sort_by_key(|bucket| (bucket.start, bucket.end));
    sorted
}

/// 🖍️ One painted cell of a chart grid: a glyph, the colour role it is drawn in and its text attributes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mark {
    pub ch: char,
    pub role: Role,
    pub attrs: u8,
}

impl Mark {
    pub fn new(ch: char, role: Role) -> Self {
        Self { ch, role, attrs: 0 }
    }

    pub fn blank() -> Self {
        Self::new(' ', Role::Foreground)
    }
}

/// 📜️ The text of a grid, one string per row; this is what the snapshot tests compare.
pub fn grid_text(grid: &[Vec<Mark>]) -> Vec<String> {
    grid.iter().map(|row| row.iter().map(|mark| mark.ch).collect()).collect()
}

/// 🖌️ Paints a grid into `rect`, clipped to it; pure data to cell buffer.
pub fn paint_grid(grid: &[Vec<Mark>], theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = buf.owner_background();
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    for (y, row) in grid.iter().enumerate().take(usize::from(rect.height)) {
        for (x, mark) in row.iter().enumerate().take(usize::from(rect.width)) {
            let mut utf8 = [0u8; 4];
            buf.put_str(Pos { x: rect.x + x as u16, y: rect.y + y as u16 }, mark.ch.encode_utf8(&mut utf8), theme.role(mark.role), bg, mark.attrs, rect);
        }
    }
}

/// ✍️ Writes `text` into a grid row starting at `column`, clipped to the row; returns the cells written.
pub fn put_text(row: &mut [Mark], column: usize, text: &str, role: Role, attrs: u8) -> usize {
    let mut written = 0;
    for (offset, ch) in text.chars().enumerate() {
        if let Some(slot) = row.get_mut(column + offset) {
            *slot = Mark { ch, role, attrs };
            written += 1;
        }
    }
    written
}

/// 🧮️ A blank grid of `width` by `height` cells.
pub fn blank_grid(width: usize, height: usize) -> Vec<Vec<Mark>> {
    vec![vec![Mark::blank(); width]; height]
}
