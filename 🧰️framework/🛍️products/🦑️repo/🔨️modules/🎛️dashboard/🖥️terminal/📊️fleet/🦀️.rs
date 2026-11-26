//! 📊️ The Fleet window: one row per origin of the shared store with its readiness, gauges, sparklines, cache hit ratios,
//! offload and machine-name warning, a fleet total, five zoom levels and a drill-down per origin with one chart per metric of
//! every group. Every state is written as a word or a symbol next to its colour; the text of a row is a pure function of the
//! snapshot, so the same snapshot renders the same text on every terminal.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/📊️fleet/🔣️.json
//! @see 🧰️framework/🔨️modules/🖱️ui/⌨️tui/📈️charts/🦀️.rs

use super::launcher::Input;
use super::panes::RowAction;
use super::windows::{Body, Dashboard};
use dashboard_fleet::{metric_label, Freshness, HostName, Point, Policy, Range, Request, Row, SeriesKind, Snapshot, StoreState};
use dashboard_labels::DashboardLabels;
use dashboard_wire::ipc::ClientMsg;
use semio_ui_tui_charts::chart_scale::{blank_grid, chart_labels, grid_text, position_column, put_text, Aggregate, Bucket, ChartLabels, Polarity, Range as ChartRange, Stats, XAxis};
use semio_ui_tui_charts::gauge::{gauge_grid, GaugeSpec};
use semio_ui_tui_charts::sparkline::{sparkline_grid, SparklineSpec};
use semio_ui_tui_charts::stat::{format_value, Unit};
use semio_ui_tui_core::theme::{Glyph, GlyphSet, Role, Status};
use semio_ui_tui_engine::Tui;
use ui_locale::Locale;

const GAUGE_CELLS: usize = 14;
const TREND_CELLS: usize = 16;
const RATE_SPARK_CELLS: usize = 9;
const LABEL_CELLS: usize = 34;
const STAT_CELLS: usize = 34;
const DETAIL_ROWS: usize = 2;
const MAXIMUM_CHART_CELLS: usize = 120;

/// 🔀️ How the rows are ordered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Sort { Origin, #[default] Problems, Cpu, Memory, Disk, Units, Beat }

impl Sort {
    pub const ALL: [Sort; 7] = [Sort::Origin, Sort::Problems, Sort::Cpu, Sort::Memory, Sort::Disk, Sort::Units, Sort::Beat];

    pub fn next(self) -> Sort { Self::ALL[(Self::ALL.iter().position(|sort| *sort == self).unwrap_or(0) + 1) % Self::ALL.len()] }

    fn label(self, text: &DashboardLabels) -> &'static str {
        (match self { Sort::Origin => text.fleet_sort_origin, Sort::Problems => text.fleet_sort_problems, Sort::Cpu => text.fleet_sort_cpu, Sort::Memory => text.fleet_sort_memory, Sort::Disk => text.fleet_sort_disk, Sort::Units => text.fleet_sort_units, Sort::Beat => text.fleet_sort_beat }).as_str()
    }
}

/// 🧱️ One column of the overview.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Column { Origin, Ready, Beat, Cpu, Memory, Disk, CpuTrend, Units, Cache, Offload, Host }

impl Column {
    fn cells(self) -> usize {
        match self { Column::Origin => 10, Column::Ready => 26, Column::Beat => 10, Column::Cpu | Column::Memory | Column::Disk => GAUGE_CELLS, Column::CpuTrend => TREND_CELLS, Column::Units => RATE_SPARK_CELLS + 1 + 9, Column::Cache => 11, Column::Offload => 14, Column::Host => 8 }
    }

    fn title(self, text: &DashboardLabels) -> &'static str {
        (match self { Column::Origin => text.fleet_col_origin, Column::Ready => text.fleet_col_ready, Column::Beat => text.fleet_col_beat, Column::Cpu => text.fleet_col_cpu, Column::Memory => text.fleet_col_memory, Column::Disk => text.fleet_col_disk, Column::CpuTrend => text.fleet_col_cpu_trend, Column::Units => text.fleet_col_units, Column::Cache => text.fleet_col_cache, Column::Offload => text.fleet_col_offload, Column::Host => text.fleet_col_host }).as_str()
    }
}

/// 🎚️ Which columns are shown: a preset the person cycles through.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Preset { #[default] Full, Host, Build, Compact }

impl Preset {
    pub const ALL: [Preset; 4] = [Preset::Full, Preset::Host, Preset::Build, Preset::Compact];

    pub fn next(self) -> Preset { Self::ALL[(Self::ALL.iter().position(|preset| *preset == self).unwrap_or(0) + 1) % Self::ALL.len()] }

    pub fn columns(self) -> &'static [Column] {
        use Column::*;
        match self {
            Preset::Full => &[Origin, Ready, Beat, Cpu, Memory, Disk, CpuTrend, Units, Cache, Offload, Host],
            Preset::Host => &[Origin, Ready, Beat, Cpu, Memory, Disk, CpuTrend, Host],
            Preset::Build => &[Origin, Ready, Units, Cache, Offload],
            Preset::Compact => &[Origin, Ready, Cpu, Memory],
        }
    }

    fn label(self, text: &DashboardLabels) -> &'static str {
        (match self { Preset::Full => text.fleet_preset_full, Preset::Host => text.fleet_preset_host, Preset::Build => text.fleet_preset_build, Preset::Compact => text.fleet_preset_compact }).as_str()
    }
}

/// 🪟️ What one Fleet window shows: the range, the order, the columns and the origin it drills into.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FleetView {
    pub range: Range,
    pub sort: Sort,
    pub preset: Preset,
    pub drill: Option<String>,
}

/// 📸️ What the dashboard knows from the daemon: the newest snapshot and the request the daemon was last told to serve.
#[derive(Default)]
pub struct FleetState {
    pub snapshot: Option<Snapshot>,
    pub watching: Option<Request>,
}

/// 📜️ One line of the window: its text, the origin it stands for, its status symbol and whether it is dimmed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub origin: Option<String>,
    pub status: Option<Status>,
    pub dim: bool,
}

impl Line {
    fn plain(text: String) -> Self { Self { text, origin: None, status: None, dim: false } }
}

fn fit(text: &str, cells: usize) -> String {
    let count = text.chars().count();
    if count > cells { text.chars().take(cells.saturating_sub(1)).chain(std::iter::once('\u{2026}')).collect() } else { format!("{text}{}", " ".repeat(cells - count)) }
}

fn join(cells: &[String]) -> String { cells.join(" ") }

fn buckets_of(points: &[Point], from: u64, to: u64) -> Vec<Bucket> {
    let mut buckets = vec![Bucket::gap(from, from + 1)];
    buckets.extend(points.iter().map(|point| Bucket::new(point[0] as u64, (point[0] + point[1]) as u64, Stats { min: point[2], max: point[3], mean: point[4], last: point[5] })));
    buckets.push(Bucket::gap(to.saturating_sub(1).max(from + 1), to.max(from + 2)));
    semio_ui_tui_charts::chart_scale::normalize(&buckets)
}

fn spark(points: &[Point], snapshot: &Snapshot, range: ChartRange, size: (usize, usize), glyphs: GlyphSet, band: bool, axis: XAxis) -> Vec<String> {
    let buckets = if axis == XAxis::Time { buckets_of(points, snapshot.from_ms, snapshot.to_ms) } else { points.iter().map(|point| Bucket::new(point[0] as u64, (point[0] + point[1]) as u64, Stats { min: point[2], max: point[3], mean: point[4], last: point[5] })).collect() };
    let spec = SparklineSpec { buckets: &buckets, axis, range, aggregate: Aggregate::Mean, band, role: Role::Accent };
    grid_text(&sparkline_grid(&spec, size.0, size.1, glyphs))
}

fn gauge(value: Option<f64>, warn: f64, critical: f64, labels: &ChartLabels, glyphs: GlyphSet) -> String {
    let spec = GaugeSpec { label: "", value: value.unwrap_or(f64::NAN), capacity: 100.0, unit: Unit::Percent, warn, critical, polarity: Polarity::HighIsBad };
    fit(&grid_text(&gauge_grid(&spec, GAUGE_CELLS, labels, glyphs))[0], GAUGE_CELLS)
}

fn ratio(hit: Option<f64>, miss: Option<f64>) -> Option<f64> {
    let (hit, miss) = (hit?, miss?);
    (hit + miss > 0.0).then(|| hit / (hit + miss) * 100.0)
}

fn percent(value: Option<f64>, labels: &ChartLabels) -> String { value.map_or_else(|| labels.no_value.as_str().to_string(), |value| format!("{:.0}%", value)) }

fn age_text(row: &Row, at_ms: u64, labels: &ChartLabels, text: &DashboardLabels) -> String {
    row.ready.as_ref().map_or_else(|| text.fleet_beat_none.as_str().to_string(), |ready| format_value(at_ms.saturating_sub(ready.updated_ms) as f64, Unit::DurationMs, labels))
}

fn state_text(row: &Row, text: &DashboardLabels, glyphs: GlyphSet) -> String {
    let word = |state: &str| match state { "ready" => text.fleet_ready_ready.as_str().to_string(), "running" => text.fleet_ready_running.as_str().to_string(), "waiting" => text.fleet_ready_waiting.as_str().to_string(), other => other.to_string() };
    let base = match &row.ready {
        Some(ready) => {
            let stage = ready.stage.as_deref().map(|stage| format!(" {stage} {}/{}", ready.stage_index, ready.stage_count)).unwrap_or_default();
            format!("{}{stage}", word(&ready.state))
        }
        None => String::new(),
    };
    match row.freshness {
        Freshness::Live => base,
        Freshness::Stale => format!("{} {}{}", glyphs.glyph(Glyph::Waiting), text.fleet_state_stale.as_str(), if base.is_empty() { String::new() } else { format!(" · {base}") }),
        Freshness::Missing => format!("{} {}", glyphs.glyph(Glyph::Fault), text.fleet_state_missing.as_str()),
    }
}

fn host_text(row: &Row, text: &DashboardLabels, glyphs: GlyphSet) -> String {
    match row.host_name {
        HostName::Ok => text.fleet_host_ok.as_str().to_string(),
        HostName::Unknown => text.fleet_host_unknown.as_str().to_string(),
        HostName::Warning if row.duplicates > 0 => format!("{} {}", glyphs.glyph(Glyph::Warning), text.fleet_host_duplicate.fill(&[("count", &row.duplicates.to_string())]).as_str()),
        HostName::Warning => format!("{} {}", glyphs.glyph(Glyph::Warning), text.fleet_host_silent.as_str()),
    }
}

/// 🚦️ The status symbol of a row: live and ready succeed, live and working run or wait, a stale row waits and a missing one is a fault.
pub fn status_of(row: &Row) -> Status {
    match (row.freshness, row.ready.as_ref()) {
        (Freshness::Missing, _) => Status::Fault,
        (Freshness::Stale, _) => Status::Waiting,
        (Freshness::Live, Some(ready)) if ready.ready => Status::Success,
        (Freshness::Live, Some(ready)) if ready.state == "running" => Status::Running,
        (Freshness::Live, Some(ready)) if ready.refusal.is_some() => Status::Warning,
        (Freshness::Live, _) => Status::Waiting,
    }
}

fn rank(row: &Row) -> u8 {
    match (row.freshness, row.ready.as_ref().is_some_and(|ready| ready.ready)) { (Freshness::Missing, _) => 0, (Freshness::Stale, _) => 1, (Freshness::Live, false) => 2, (Freshness::Live, true) => 3 }
}

/// 🔀️ The rows in the order of `sort`: values descending (a row without a value last), problems first, origins ascending.
pub fn sorted(rows: &[Row], sort: Sort, at_ms: u64) -> Vec<&Row> {
    let mut found: Vec<&Row> = rows.iter().collect();
    let descending = |value: Option<f64>| value.map_or(f64::INFINITY, |value| -value);
    let key = |row: &Row| -> f64 {
        match sort {
            Sort::Origin => 0.0,
            Sort::Problems => f64::from(rank(row)),
            Sort::Cpu => descending(row.cpu),
            Sort::Memory => descending(row.memory),
            Sort::Disk => descending(row.disk),
            Sort::Units => descending(row.units_per_minute),
            Sort::Beat => descending(row.ready.as_ref().map(|ready| at_ms.saturating_sub(ready.updated_ms) as f64)),
        }
    };
    found.sort_by(|left, right| key(left).total_cmp(&key(right)).then_with(|| left.origin.cmp(&right.origin)));
    found
}

fn na_mark(glyphs: GlyphSet) -> char { if glyphs == GlyphSet::Unicode { '\u{2298}' } else { '~' } }

fn na_text(text: &DashboardLabels, glyphs: GlyphSet) -> String { format!("{} {}", na_mark(glyphs), text.fleet_na.as_str()) }

fn na_roles(column: Column) -> &'static [&'static str] {
    match column {
        Column::Cpu | Column::CpuTrend => &["cpu"],
        Column::Memory => &["memoryAvailable", "memoryTotal"],
        Column::Disk => &["diskFree", "diskTotal"],
        Column::Units => &["units"],
        Column::Cache => &["localHit", "remoteHit"],
        Column::Offload => &["offloadOut"],
        Column::Origin | Column::Ready | Column::Beat | Column::Host => &[],
    }
}

fn unsupported_cell(column: Column, row: &Row) -> bool {
    let names = dashboard_fleet::model::overview_ids();
    na_roles(column).iter().any(|role| names.get(*role).is_some_and(|id| row.unsupported.contains(id)))
}

fn cell(column: Column, row: &Row, snapshot: &Snapshot, text: &DashboardLabels, labels: &ChartLabels, glyphs: GlyphSet) -> String {
    if unsupported_cell(column, row) { return fit(&na_text(text, glyphs), column.cells()); }
    match column {
        Column::Origin => fit(&row.origin, column.cells()),
        Column::Ready => fit(&state_text(row, text, glyphs), column.cells()),
        Column::Beat => fit(&age_text(row, snapshot.at_ms, labels, text), column.cells()),
        Column::Cpu => gauge(row.cpu, 0.75, 0.9, labels, glyphs),
        Column::Memory => gauge(row.memory, 0.8, 0.9, labels, glyphs),
        Column::Disk => gauge(row.disk, 0.8, 0.9, labels, glyphs),
        Column::CpuTrend => spark(&row.cpu_series, snapshot, ChartRange::Fixed(0.0, 100.0), (TREND_CELLS, 1), glyphs, false, XAxis::Time).remove(0),
        Column::Units => {
            let line = spark(&row.units_series, snapshot, ChartRange::Auto, (RATE_SPARK_CELLS, 1), glyphs, false, XAxis::Time).remove(0);
            let rate = row.units_per_minute.map_or_else(|| labels.no_value.as_str().to_string(), |rate| format!("{}{}", format_value(rate, Unit::Count, labels), text.fleet_per_minute.as_str()));
            fit(&format!("{line} {rate}"), column.cells())
        }
        Column::Cache => fit(&format!("{} / {}", percent(ratio(row.local_hit, row.local_miss), labels), percent(ratio(row.remote_hit, row.remote_miss), labels)), column.cells()),
        Column::Offload => fit(&match (row.offload_in, row.offload_out) { (None, None) => labels.no_value.as_str().to_string(), (inbound, outbound) => format!("{}/{}", inbound.unwrap_or(0.0), outbound.unwrap_or(0.0)) }, column.cells()),
        Column::Host => fit(&host_text(row, text, glyphs), column.cells()),
    }
}

fn total_row(rows: &[Row]) -> Row {
    let live: Vec<&Row> = rows.iter().filter(|row| row.freshness == Freshness::Live).collect();
    let mean = |pick: fn(&Row) -> Option<f64>| { let values: Vec<f64> = live.iter().filter_map(|row| pick(row)).collect(); (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64) };
    let sum = |pick: fn(&Row) -> Option<f64>| { let values: Vec<f64> = live.iter().filter_map(|row| pick(row)).collect(); (!values.is_empty()).then(|| values.iter().sum::<f64>()) };
    let series = |pick: fn(&Row) -> &Vec<Point>, average: bool| -> Vec<Point> {
        let mut columns: std::collections::BTreeMap<u64, (Point, f64)> = std::collections::BTreeMap::new();
        for row in &live {
            for point in pick(row) {
                let entry = columns.entry(point[0] as u64).or_insert(([point[0], point[1], 0.0, 0.0, 0.0, 0.0], 0.0));
                for index in 2..6 { entry.0[index] += point[index]; }
                entry.1 += 1.0;
            }
        }
        columns.into_values().map(|(mut point, count)| { if average { for index in 2..6 { point[index] /= count; } } point }).collect()
    };
    let mut total = Row::empty("");
    total.freshness = Freshness::Live;
    (total.cpu, total.memory, total.disk, total.units_per_minute) = (mean(|row| row.cpu), mean(|row| row.memory), mean(|row| row.disk), sum(|row| row.units_per_minute));
    (total.local_hit, total.local_miss, total.remote_hit, total.remote_miss, total.offload_in, total.offload_out) = (sum(|row| row.local_hit), sum(|row| row.local_miss), sum(|row| row.remote_hit), sum(|row| row.remote_miss), sum(|row| row.offload_in), sum(|row| row.offload_out));
    total.cpu_series = series(|row| &row.cpu_series, true);
    total.units_series = series(|row| &row.units_series, false);
    total.host_name = if rows.iter().any(|row| row.host_name == HostName::Warning) { HostName::Warning } else { HostName::Ok };
    total.duplicates = rows.iter().filter(|row| row.host_name == HostName::Warning).count() as u32;
    total
}

/// 🧾️ The lines of the overview: summary, hint, column titles, the fleet total and one line per origin.
pub fn overview_lines(snapshot: Option<&Snapshot>, view: &FleetView, text: &DashboardLabels, locale: Locale, glyphs: GlyphSet) -> Vec<Line> {
    let language = locale.as_str();
    let summary = text.fleet_summary.fill(&[("range", view.range.label(language)), ("sort", view.sort.label(text)), ("columns", view.preset.label(text))]).as_str().to_string();
    let mut lines = vec![Line::plain(summary), Line::plain(text.fleet_hint.as_str().to_string())];
    let wanted = Request { range: view.range, columns: 0, origin: None };
    let Some(snapshot) = snapshot.filter(|snapshot| snapshot.answers(&wanted)) else { lines.push(Line::plain(text.fleet_loading.as_str().to_string())); return lines };
    if snapshot.store == StoreState::Unavailable { lines.push(Line::plain(text.fleet_store_unavailable.as_str().to_string())); return lines; }
    if snapshot.rows.is_empty() { lines.push(Line::plain(text.fleet_no_origins.as_str().to_string())); return lines; }
    let labels = chart_labels(locale);
    let columns = view.preset.columns();
    lines.push(Line::plain(join(&columns.iter().map(|column| fit(column.title(text), column.cells())).collect::<Vec<_>>())));
    let mut total = total_row(&snapshot.rows);
    total.offload_in = snapshot.fleet.as_ref().and_then(|fleet| fleet.offload_in);
    let live = snapshot.rows.iter().filter(|row| row.freshness == Freshness::Live && row.ready.as_ref().is_some_and(|ready| ready.ready)).count();
    let mut cells: Vec<String> = columns.iter().map(|column| cell(*column, &total, snapshot, text, labels, glyphs)).collect();
    for (index, column) in columns.iter().enumerate() {
        match column {
            Column::Origin => cells[index] = fit(text.fleet_total.as_str(), column.cells()),
            Column::Ready => cells[index] = fit(text.fleet_total_ready.fill(&[("ready", &live.to_string()), ("total", &snapshot.rows.len().to_string())]).as_str(), column.cells()),
            Column::Beat => cells[index] = fit("", column.cells()),
            Column::Host => cells[index] = fit(&if total.duplicates > 0 { format!("{} {}", glyphs.glyph(Glyph::Warning), total.duplicates) } else { text.fleet_host_ok.as_str().to_string() }, column.cells()),
            _ => {}
        }
    }
    lines.push(Line::plain(join(&cells)));
    if let Some(fleet) = &snapshot.fleet {
        let show = |value: Option<f64>, unit: Unit| value.map_or_else(|| labels.no_value.as_str().to_string(), |value| format_value(value, unit, labels));
        lines.push(Line::plain(text.fleet_facts.fill(&[("running", &show(fleet.queue_running, Unit::Count)), ("queued", &show(fleet.queue_queued, Unit::Count)), ("offloaded", &show(fleet.offload_in, Unit::Count)), ("entries", &show(fleet.remote_entries, Unit::Count)), ("bytes", &show(fleet.remote_bytes, Unit::Bytes))]).as_str().to_string()));
    }
    for row in sorted(&snapshot.rows, view.sort, snapshot.at_ms) {
        let cells: Vec<String> = columns.iter().map(|column| cell(*column, row, snapshot, text, labels, glyphs)).collect();
        lines.push(Line { text: join(&cells), origin: Some(row.origin.clone()), status: Some(status_of(row)), dim: row.freshness != Freshness::Live });
    }
    lines
}

fn unit_of(unit: &str, kind: SeriesKind) -> Unit {
    match (unit, kind) {
        (_, SeriesKind::Counter) => Unit::Count,
        ("percent", _) => Unit::Percent,
        ("bytes", _) => Unit::Bytes,
        ("bytesPerSecond", _) => Unit::ByteRate,
        ("milliseconds", _) => Unit::DurationMs,
        _ => Unit::Count,
    }
}

fn scaled_for_duration(unit: &str, value: f64) -> f64 {
    match unit { "seconds" => value * 1000.0, "microseconds" => value / 1000.0, _ => value }
}

fn ruler(points: &[Point], snapshot: &Snapshot, cells: usize, labels: &ChartLabels, text: &DashboardLabels, glyphs: GlyphSet) -> [String; 2] {
    let buckets: Vec<Bucket> = points.iter().map(|point| Bucket::new(point[0] as u64, (point[0] + point[1]) as u64, Stats::point(0.0))).collect();
    let line_mark = if glyphs == GlyphSet::Unicode { '\u{2500}' } else { '-' };
    let tick_mark = if glyphs == GlyphSet::Unicode { '\u{252c}' } else { '+' };
    let mut axis = blank_grid(cells, 1);
    for mark in axis[0].iter_mut() { mark.ch = line_mark; mark.role = Role::MutedForeground; }
    let mut names = blank_grid(cells, 1);
    let domain = (0, buckets.len().max(1) as u64);
    let mut marks: Vec<(usize, String)> = semio_ui_tui_charts::chart_scale::tier_columns(&buckets, XAxis::Index, domain, cells).into_iter().map(|(column, index)| (column, format_value(snapshot.to_ms.saturating_sub(buckets[index].start) as f64, Unit::DurationMs, labels))).collect();
    marks.insert(0, (0, format_value(snapshot.to_ms.saturating_sub(buckets.first().map_or(snapshot.from_ms, |bucket| bucket.start)) as f64, Unit::DurationMs, labels)));
    marks.push((position_column(domain.1.saturating_sub(1), domain, cells).max(cells.saturating_sub(1)), text.fleet_detail_now.as_str().to_string()));
    let mut taken_until = 0usize;
    for (column, label) in marks {
        if let Some(mark) = axis[0].get_mut(column) { mark.ch = tick_mark; }
        let column = if column + label.chars().count() > cells { cells.saturating_sub(label.chars().count()) } else { column };
        if column >= taken_until || taken_until == 0 { put_text(&mut names[0], column, &label, Role::MutedForeground, 0); taken_until = column + label.chars().count() + 1; }
    }
    [grid_text(&axis).remove(0), grid_text(&names).remove(0)]
}

/// 🔬️ The chart width of a drill-down in a window `width` cells wide.
pub fn chart_cells(width: u16) -> usize { usize::from(width).saturating_sub(LABEL_CELLS + STAT_CELLS + 2).clamp(usize::from(Policy::get().minimum_columns), MAXIMUM_CHART_CELLS) }

/// 🔬️ The lines of the drill-down of one origin: the header, the tier ruler and one chart per metric of every group.
pub fn detail_lines(snapshot: Option<&Snapshot>, view: &FleetView, text: &DashboardLabels, locale: Locale, glyphs: GlyphSet, width: u16) -> Vec<Line> {
    let language = locale.as_str();
    let origin = view.drill.clone().unwrap_or_default();
    let mut lines = Vec::new();
    let wanted = Request { range: view.range, columns: 0, origin: Some(origin.clone()) };
    let Some(snapshot) = snapshot.filter(|snapshot| snapshot.answers(&wanted)) else {
        lines.push(Line::plain(text.fleet_detail_title.fill(&[("origin", &origin), ("range", view.range.label(language)), ("state", text.fleet_loading.as_str())]).as_str().to_string()));
        return lines;
    };
    let Some(row) = snapshot.row(&origin) else {
        lines.push(Line::plain(text.fleet_detail_missing.fill(&[("origin", &origin)]).as_str().to_string()));
        return lines;
    };
    let labels = chart_labels(locale);
    let state = state_text(row, text, glyphs);
    lines.push(Line { text: text.fleet_detail_title.fill(&[("origin", &origin), ("range", view.range.label(language)), ("state", &state)]).as_str().to_string(), origin: Some(origin.clone()), status: Some(status_of(row)), dim: row.freshness != Freshness::Live });
    lines.push(Line::plain(text.fleet_hint.as_str().to_string()));
    let cells = chart_cells(width);
    let Some(detail) = &snapshot.detail else { return lines };
    let table = dashboard_fleet::model::group_table();
    let first = detail.groups.iter().flat_map(|group| group.series.iter()).find(|series| !series.points.is_empty());
    if let Some(series) = first {
        let [axis, names] = ruler(&series.points, snapshot, cells, labels, text, glyphs);
        lines.push(Line::plain(format!("{} {axis}", fit("", LABEL_CELLS))));
        lines.push(Line::plain(format!("{} {names}", fit("", LABEL_CELLS))));
    }
    for group in &detail.groups {
        let heading = table.iter().find(|(id, _, _)| *id == group.id).and_then(|(_, label, _)| label.get(language)).cloned().unwrap_or_else(|| group.id.clone());
        lines.push(Line::plain(format!("{} {heading}", glyphs.glyph(Glyph::Expanded))));
        if group.series.is_empty() { lines.push(Line::plain(format!("  {}", text.fleet_detail_empty.as_str()))); continue; }
        for series in &group.series {
            if series.unsupported {
                let platform = row.ready.as_ref().and_then(|ready| ready.platform.as_deref()).unwrap_or_default();
                let title: &str = metric_label(&series.id, language).unwrap_or(series.id.as_str());
                lines.push(Line::plain(format!("{} {} {} · {}", fit(title, LABEL_CELLS), na_mark(glyphs), text.fleet_na.as_str(), text.fleet_detail_unsupported.fill(&[("platform", platform)]).as_str())));
                continue;
            }
            let unit = unit_of(&series.unit, series.kind);
            let shown = |value: f64| match series.unit.as_str() { "seconds" | "microseconds" => format_value(scaled_for_duration(&series.unit, value), Unit::DurationMs, labels), _ => format_value(value, unit, labels) };
            let suffix = if series.kind == SeriesKind::Counter { text.fleet_per_minute.as_str() } else { "" };
            let title: &str = metric_label(&series.id, language).unwrap_or(series.id.as_str());
            let titled = if series.scope == dashboard_fleet::Scope::Fleet { format!("{title} {}", text.fleet_scope_fleet.as_str()) } else { title.to_string() };
            let name = fit(&titled, LABEL_CELLS);
            let low = series.points.iter().map(|point| point[2]).fold(f64::INFINITY, f64::min);
            let high = series.points.iter().map(|point| point[3]).fold(f64::NEG_INFINITY, f64::max);
            let last = series.points.last().map_or(f64::NAN, |point| point[5]);
            let range = if series.unit == "percent" { ChartRange::Fixed(0.0, 100.0) } else { ChartRange::Auto };
            let rows = spark(&series.points, snapshot, range, (cells, DETAIL_ROWS), glyphs, true, XAxis::Index);
            let stat = fit(&format!("{} {}{suffix}  {} {}  {} {}", text.fleet_stat_last.as_str(), shown(last), text.fleet_stat_min.as_str(), shown(low), text.fleet_stat_max.as_str(), shown(high)), STAT_CELLS);
            lines.push(Line::plain(format!("{name} {} {stat}", rows[0])));
            for extra in &rows[1..] { lines.push(Line::plain(format!("{} {extra}", fit("", LABEL_CELLS)))); }
        }
    }
    lines
}

impl Dashboard {
    fn fleet_view(&self, index: usize) -> Option<&FleetView> { match &self.windows.get(index)?.body { Body::Fleet(view) => Some(view), _ => None } }

    /// 🧾️ The rows of the Fleet window `index`.
    pub fn fleet_rows(&self, index: usize, width: u16) -> Vec<(String, RowAction)> {
        let Some(view) = self.fleet_view(index) else { return Vec::new() };
        let lines = if view.drill.is_some() { detail_lines(self.fleet.snapshot.as_ref(), view, self.text(), self.locale, self.glyphs, width) } else { overview_lines(self.fleet.snapshot.as_ref(), view, self.text(), self.locale, self.glyphs) };
        lines.into_iter().map(|line| { let action = line.origin.filter(|_| view.drill.is_none()).map_or(RowAction::None, RowAction::FleetOrigin); (line.text, action) }).collect()
    }

    /// 🚦️ The status symbol and the dimming of the line behind a row action.
    pub fn fleet_marks(&self, action: &RowAction) -> (Option<Status>, bool) {
        let RowAction::FleetOrigin(origin) = action else { return (None, false) };
        self.fleet.snapshot.as_ref().and_then(|snapshot| snapshot.row(origin)).map_or((None, false), |row| (Some(status_of(row)), row.freshness != Freshness::Live))
    }

    /// 🙋️ What the daemon should serve: the range and drill-down of the focused Fleet window, else of the first one.
    pub fn fleet_request(&self, tui: &Tui) -> Option<Request> {
        let focused = self.focused_index(tui);
        let index = if self.fleet_view(focused).is_some() { focused } else { (0..self.windows.len()).find(|index| self.fleet_view(*index).is_some())? };
        let view = self.fleet_view(index)?;
        let width = tui.scene.rect(self.windows[index].list).width;
        let columns = if view.drill.is_some() { chart_cells(width) as u16 } else { Policy::get().overview_columns };
        Some(Request { range: view.range, columns, origin: view.drill.clone() }.clamped())
    }

    /// 🔄️ Tells the daemon what to serve when it changed; a lost connection forgets what the daemon was told.
    pub fn sync_fleet(&mut self, tui: &Tui) {
        if !self.online() { self.fleet.watching = None; return; }
        let wanted = self.fleet_request(tui);
        if wanted == self.fleet.watching { return; }
        let message = match &wanted { Some(request) => ClientMsg::FleetWatch { request: request.clone() }, None => ClientMsg::FleetUnwatch {} };
        if self.send(&message).is_ok() { self.fleet.watching = wanted; }
    }

    /// 📥️ Takes a snapshot of the daemon; one that answers an older request is dropped.
    pub fn apply_fleet(&mut self, tui: &mut Tui, snapshot: Snapshot) {
        if self.fleet.watching.as_ref().is_some_and(|request| !snapshot.answers(request)) { return; }
        self.fleet.snapshot = Some(snapshot);
        self.refresh_panes(tui);
    }

    /// ⌨️ Keys of the Fleet window; true when the key was one of them.
    pub fn fleet_input(&mut self, tui: &mut Tui, index: usize, input: &Input) -> bool {
        let selected = match self.windows[index].pane.current() { Some(RowAction::FleetOrigin(origin)) => Some(origin.clone()), _ => None };
        let mut reselect = false;
        {
            let Body::Fleet(view) = &mut self.windows[index].body else { return false };
            match input {
                Input::Left => view.range = view.range.finer(),
                Input::Right => view.range = view.range.coarser(),
                Input::Char(digit @ '1'..='5') => view.range = Range::ALL[usize::from(*digit as u8 - b'1')],
                Input::Char('s') if view.drill.is_none() => view.sort = view.sort.next(),
                Input::Char('c') if view.drill.is_none() => view.preset = view.preset.next(),
                Input::Activate if view.drill.is_none() => { let Some(origin) = selected else { return true }; view.drill = Some(origin); reselect = true; }
                Input::Back if view.drill.is_some() => { view.drill = None; reselect = true; }
                Input::Back => return false,
                Input::Char(_) | Input::Activate | Input::Backspace => return true,
                _ => return false,
            }
        }
        if reselect { self.windows[index].pane.selected = 0; self.windows[index].pane.offset = 0; }
        self.sync_fleet(tui);
        true
    }
}
