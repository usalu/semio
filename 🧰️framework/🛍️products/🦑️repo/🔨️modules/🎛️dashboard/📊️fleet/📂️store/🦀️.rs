//! 📂️ The reader of the shared store: it turns `origins/<origin>/…` into the rows and the drill-down of a snapshot. Nothing
//! here writes, waits for a lock or follows a path outside the origin folder; a file that cannot be read leaves its part of
//! the row empty and the row says what is missing.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/📊️fleet/🔣️.json
//! @see 🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🦀️.rs

use super::model::{group_table, overview_ids, Detail, Freshness, Group, HostName, Point, Policy, Ready, Request, Row, Scope, Series, SeriesKind, Snapshot, StoreState, Totals};
use semio_framework_process_metrics::{read_series, MetricsError, Point as RingPoint};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const CATALOGUE: &str = include_str!("../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/📇️catalogue/🔣️.json");
const MAXIMUM_FILE_BYTES: u64 = 1 << 20;
const PRODUCTION_TIER_MS: [u64; 5] = [1000, 10_000, 60_000, 600_000, 3_600_000];

#[derive(Deserialize)]
struct CatalogueRow {
    id: String,
    kind: String,
    unit: String,
    scope: String,
    source: String,
    #[serde(default)]
    platforms: Vec<String>,
    label: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct Catalogue {
    metrics: Vec<CatalogueRow>,
}

fn catalogue() -> &'static BTreeMap<String, CatalogueRow> {
    static ROWS: OnceLock<BTreeMap<String, CatalogueRow>> = OnceLock::new();
    ROWS.get_or_init(|| serde_json::from_str::<Catalogue>(CATALOGUE).expect("the metrics catalogue is closed").metrics.into_iter().map(|row| (row.id.clone(), row)).collect())
}

/// 🏷️ The label of a metric in `language` from the metrics catalogue, if the catalogue knows the metric.
pub fn metric_label(id: &str, language: &str) -> Option<&'static str> {
    catalogue().get(id).and_then(|row| row.label.get(language)).map(String::as_str)
}

/// 📏️ The unit a metric of the catalogue is measured in.
pub fn metric_unit(id: &str) -> Option<&'static str> {
    catalogue().get(id).map(|row| row.unit.as_str())
}

/// 🌐️ Whether the catalogue says a metric is measured per origin or once for the fleet.
pub fn metric_scope(id: &str) -> Option<Scope> {
    catalogue().get(id).map(|row| if row.scope == "fleet" { Scope::Fleet } else { Scope::Origin })
}

/// 🧭️ Whether a machine of `platform` (`darwin`/`macos`, `linux`, `win32`/`windows`) can measure a metric at all: the catalogue lists `platforms` only for a metric that some platform has no source for, so an absent series there is `n/a`, not a gap. A metric the catalogue does not know counts as supported.
pub fn metric_supported(id: &str, platform: &str) -> bool {
    let name = match platform { "macos" => "darwin", "windows" => "win32", other => other };
    catalogue().get(id).is_none_or(|row| row.platforms.is_empty() || row.platforms.iter().any(|listed| listed == name))
}

/// 🗄️ Where the origins of the fleet publish: the explicit store root, else the `storeRoot` that `<state>/f/machine.json` of this machine names, below `v1/origins`.
/// `state` is the declared machine state directory (it stands for `<workspace>/.cache`), `None` keeps the state in the workspace.
pub fn origins_directory(workspace: &Path, explicit: Option<&str>, state: Option<&Path>) -> Option<PathBuf> {
    let named = explicit.filter(|root| !root.is_empty()).map(PathBuf::from).or_else(|| {
        let machine = state.map_or_else(|| workspace.join(".cache"), Path::to_path_buf).join("f").join("machine.json");
        let value: serde_json::Value = serde_json::from_slice(&read_small(&machine)?).ok()?;
        value.get("storeRoot")?.as_str().filter(|root| !root.is_empty()).map(PathBuf::from)
    })?;
    Some(named.join("v1").join("origins"))
}

fn read_small(path: &Path) -> Option<Vec<u8>> {
    let length = std::fs::metadata(path).ok()?.len();
    if length > MAXIMUM_FILE_BYTES { return None; }
    std::fs::read(path).ok()
}

fn read_json(path: &Path) -> Option<serde_json::Value> {
    serde_json::from_slice(&read_small(path)?).ok()
}

fn round(value: f64) -> f64 {
    let scale = 10f64.powi(Policy::get().decimals as i32);
    (value * scale).round() / scale
}

fn ready_of(value: &serde_json::Value) -> Option<Ready> {
    let text = |key: &str| value.get(key).and_then(|found| found.as_str()).map(str::to_string);
    let number = |key: &str| value.get(key).and_then(serde_json::Value::as_u64);
    let refusal = match value.get("refusal") {
        Some(serde_json::Value::String(code)) => Some(code.clone()),
        Some(serde_json::Value::Object(map)) => map.get("code").and_then(|code| code.as_str()).map(str::to_string),
        _ => None,
    };
    Some(Ready {
        state: text("state")?,
        ready: value.get("ready")?.as_bool()?,
        stage: text("stage"),
        stage_index: number("stageIndex").unwrap_or(0),
        stage_count: number("stageCount").unwrap_or(0),
        updated_ms: number("updatedMs")?,
        beat_ms: number("beatMilliseconds").unwrap_or(Policy::get().default_beat_ms),
        platform: text("platform"),
        refusal,
    })
}

fn host_name_of(origin: &Path) -> (HostName, u32) {
    let Some(value) = read_json(&origin.join("addresses.json")) else { return (HostName::Unknown, 0) };
    let advertise = value.get("advertise");
    let enabled = advertise.and_then(|found| found.get("enabled")).and_then(serde_json::Value::as_bool).unwrap_or(true);
    let duplicates = advertise.and_then(|found| found.get("duplicates")).and_then(|found| found.as_array()).map_or(0, Vec::len) as u32;
    (if enabled && duplicates == 0 { HostName::Ok } else { HostName::Warning }, duplicates)
}

fn memory_percent_from_record(origin: &Path) -> Option<f64> {
    let value = read_json(&origin.join("memory.json"))?;
    let host = value.get("host")?;
    let total = host.get("totalMegabytes")?.as_f64()?;
    let available = host.get("availableMegabytes")?.as_f64()?;
    (total > 0.0).then(|| round(((total - available) / total * 100.0).clamp(0.0, 100.0)))
}

struct Samples {
    tier_ms: Vec<u64>,
    series: BTreeMap<String, Vec<RingPoint>>,
}

impl Samples {
    fn merge(&mut self, other: Samples) {
        if self.tier_ms.len() < other.tier_ms.len() { self.tier_ms.resize(other.tier_ms.len(), 0); }
        for (index, width) in other.tier_ms.into_iter().enumerate() { self.tier_ms[index] = self.tier_ms[index].max(width); }
        self.series.extend(other.series);
    }

    fn bucket_ms(&self, point: &RingPoint) -> u64 {
        self.tier_ms.get(usize::from(point.tier)).copied().filter(|width| *width > 0).unwrap_or_else(|| PRODUCTION_TIER_MS.get(usize::from(point.tier)).copied().unwrap_or(point.width.max(1)))
    }

    fn rows(&self, id: &str, kind: SeriesKind) -> Vec<Bucket> {
        self.series.get(id).map_or_else(Vec::new, |points| {
            points.iter().filter(|point| point.count > 0).map(|point| {
                let mean = point.sum / f64::from(point.count);
                if kind == SeriesKind::Counter {
                    let rate = point.sum / self.bucket_ms(point) as f64 * 60_000.0;
                    Bucket { start: point.start, width: point.width, tier: point.tier, min: rate, max: rate, mean: rate, last: rate }
                } else {
                    Bucket { start: point.start, width: point.width, tier: point.tier, min: point.min, max: point.max, mean, last: point.last }
                }
            }).collect()
        })
    }

    fn total(&self, id: &str) -> Option<f64> {
        let points = self.series.get(id)?;
        let seen = points.iter().any(|point| point.count > 0);
        seen.then(|| points.iter().map(|point| point.sum).sum())
    }

    fn last(&self, id: &str) -> Option<f64> {
        self.series.get(id)?.iter().rev().find(|point| point.count > 0).map(|point| point.last)
    }

    fn newest_end(&self) -> Option<u64> {
        self.series.values().flat_map(|points| points.iter().filter(|point| point.count > 0).map(|point| point.start + point.width)).max()
    }
}

/// 🪣️ One bucket of a series, counters already turned into a rate per minute.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bucket {
    pub start: u64,
    pub width: u64,
    pub tier: u16,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub last: f64,
}

fn load_samples(directory: &Path, ids: &[String], from: u64, to: u64) -> Option<Samples> {
    if !directory.is_dir() { return None; }
    let mut wanted: Vec<String> = ids.to_vec();
    for _ in 0..2 {
        if wanted.is_empty() { return None; }
        match read_series(&directory, from, to, Some(&wanted)) {
            Ok(series) => {
                let mut tier_ms: Vec<u64> = Vec::new();
                for (_, points) in &series.metrics {
                    for point in points.iter().filter(|point| !point.partial) {
                        let index = usize::from(point.tier);
                        if tier_ms.len() <= index { tier_ms.resize(index + 1, 0); }
                        tier_ms[index] = tier_ms[index].max(point.width);
                    }
                }
                return Some(Samples { tier_ms, series: series.metrics.into_iter().collect() });
            }
            Err(MetricsError::Unknown(missing)) => wanted.retain(|id| !missing.contains(id)),
            Err(_) => return None,
        }
    }
    None
}

/// 🌍️ The series of the fleet-scope metrics among `ids`: each lives in the fleet rings of its catalogue source, `<store>/v1/fleet/metrics/<source>/`; `origins` is the origins folder, so the fleet folder is its sibling.
fn fleet_samples(origins: &Path, ids: &[String], from: u64, to: u64) -> Option<Samples> {
    let base = origins.parent()?.join("fleet").join("metrics");
    let mut sources: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for id in ids { if let Some(row) = catalogue().get(id).filter(|row| row.scope == "fleet") { sources.entry(row.source.as_str()).or_default().push(id.clone()); } }
    let mut merged: Option<Samples> = None;
    for (source, wanted) in sources {
        let Some(found) = load_samples(&base.join(source), &wanted, from, to) else { continue };
        match merged.as_mut() { Some(all) => all.merge(found), None => merged = Some(found) }
    }
    merged
}

fn totals_of(origins: &Path, from: u64, to: u64) -> Option<Totals> {
    let names = overview_ids();
    let ids: Vec<String> = ["offloadIn", "queueRunning", "queueQueued", "remoteEntries", "remoteBytes"].iter().filter_map(|role| names.get(*role).cloned()).collect();
    let samples = fleet_samples(origins, &ids, from, to)?;
    let name = |role: &str| names.get(role).map_or("", String::as_str);
    Some(Totals { offload_in: samples.total(name("offloadIn")).map(round), queue_running: samples.last(name("queueRunning")).map(round), queue_queued: samples.last(name("queueQueued")).map(round), remote_entries: samples.last(name("remoteEntries")).map(round), remote_bytes: samples.last(name("remoteBytes")).map(round) })
}

/// 🪜 Folds buckets into `columns` equal time columns of `[from, to)`: min of mins, max of maxes, overlap-weighted mean, last of the latest; a column without data is left out.
pub fn fold_columns(rows: &[Bucket], from: u64, to: u64, columns: usize) -> Vec<Point> {
    let span = u128::from(to.saturating_sub(from).max(1));
    let mut out = Vec::new();
    for column in 0..columns {
        let c0 = u128::from(from) + span * column as u128 / columns as u128;
        let c1 = (u128::from(from) + span * (column as u128 + 1) / columns as u128).max(c0 + 1);
        let mut inside: Vec<(&Bucket, f64)> = Vec::new();
        for row in rows {
            let (start, end) = (u128::from(row.start), u128::from(row.start) + u128::from(row.width.max(1)));
            let overlap = end.min(c1).saturating_sub(start.max(c0));
            if overlap > 0 { inside.push((row, overlap as f64)); }
        }
        let Some(latest) = inside.iter().map(|(row, _)| *row).max_by_key(|row| row.start) else { continue };
        let weight: f64 = inside.iter().map(|(_, overlap)| overlap).sum();
        let mean = inside.iter().map(|(row, overlap)| row.mean * overlap).sum::<f64>() / weight;
        let min = inside.iter().map(|(row, _)| row.min).fold(f64::INFINITY, f64::min);
        let max = inside.iter().map(|(row, _)| row.max).fold(f64::NEG_INFINITY, f64::max);
        out.push([c0 as f64, (c1 - c0) as f64, round(min), round(max), round(mean), round(latest.last)]);
    }
    out
}

/// ✂️ Thins buckets to about `budget` points: every tier run keeps its own width (equal chunks from the newest end, the odd oldest buckets of a run are dropped), so tier boundaries stay where they are.
pub fn thin(rows: &[Bucket], budget: usize) -> Vec<Point> {
    let mut runs: Vec<&[Bucket]> = Vec::new();
    let mut begin = 0;
    for index in 1..=rows.len() {
        if index == rows.len() || rows[index].tier != rows[begin].tier {
            runs.push(&rows[begin..index]);
            begin = index;
        }
    }
    let roots: Vec<f64> = runs.iter().map(|run| (run.iter().map(|row| row.width as f64).sum::<f64>()).sqrt()).collect();
    let total: f64 = roots.iter().sum::<f64>().max(f64::EPSILON);
    let floor = Policy::get().minimum_run_columns;
    let mut out = Vec::new();
    for (run, root) in runs.iter().zip(roots) {
        let share = ((budget as f64 * root / total).round() as usize).max(floor);
        let group = run.len().div_ceil(share).max(1);
        let skip = run.len() % group;
        for chunk in run[skip..].chunks(group) {
            let width: u64 = chunk.iter().map(|row| row.width).sum();
            let weight: f64 = chunk.iter().map(|row| row.width as f64).sum::<f64>().max(1.0);
            let mean = chunk.iter().map(|row| row.mean * row.width as f64).sum::<f64>() / weight;
            let min = chunk.iter().map(|row| row.min).fold(f64::INFINITY, f64::min);
            let max = chunk.iter().map(|row| row.max).fold(f64::NEG_INFINITY, f64::max);
            out.push([chunk[0].start as f64, width as f64, round(min), round(max), round(mean), round(chunk[chunk.len() - 1].last)]);
        }
    }
    out
}

fn percent_used(free: Option<f64>, total: Option<f64>) -> Option<f64> {
    let (free, total) = (free?, total?);
    (total > 0.0).then(|| round(((total - free) / total * 100.0).clamp(0.0, 100.0)))
}

fn row_of(origins: &Path, origin: &str, from: u64, to: u64, columns: usize, now: u64) -> Row {
    let directory = origins.join(origin);
    let mut row = Row::empty(origin);
    row.ready = read_json(&directory.join("ready.json")).as_ref().and_then(ready_of);
    (row.host_name, row.duplicates) = host_name_of(&directory);
    let names = overview_ids();
    if let Some(platform) = row.ready.as_ref().and_then(|ready| ready.platform.as_deref()) {
        row.unsupported = names.values().filter(|id| !metric_supported(id, platform)).cloned().collect();
        row.unsupported.sort();
    }
    let ids: Vec<String> = names.values().filter(|id| metric_scope(id) == Some(Scope::Origin) && !row.unsupported.contains(id)).cloned().collect();
    let samples = load_samples(&directory.join("metrics"), &ids, from, to);
    let name = |role: &str| names.get(role).map_or("", String::as_str);
    if let Some(samples) = &samples {
        row.metrics_ms = samples.newest_end();
        row.cpu = samples.last(name("cpu")).map(round);
        row.memory = percent_used(samples.last(name("memoryAvailable")), samples.last(name("memoryTotal")));
        row.disk = percent_used(samples.last(name("diskFree")), samples.last(name("diskTotal")));
        let units = samples.rows(name("units"), SeriesKind::Counter);
        row.units_per_minute = (!units.is_empty()).then(|| round(units.iter().map(|point| point.mean * point.width as f64).sum::<f64>() / units.iter().map(|point| point.width as f64).sum::<f64>().max(1.0)));
        row.cpu_series = fold_columns(&samples.rows(name("cpu"), SeriesKind::Gauge), from, to, columns);
        row.units_series = fold_columns(&units, from, to, columns);
        let total = |role: &str| samples.total(name(role)).map(round);
        (row.local_hit, row.local_miss, row.remote_hit, row.remote_miss, row.offload_out) = (total("localHit"), total("localMiss"), total("remoteHit"), total("remoteMiss"), total("offloadOut"));
    }
    if row.memory.is_none() { row.memory = memory_percent_from_record(&directory); }
    classify(&mut row, now);
    row
}

fn classify(row: &mut Row, now: u64) {
    row.freshness = match &row.ready {
        Some(ready) => {
            let allowed = Policy::get().stale_beats.saturating_mul(ready.beat_ms.max(1));
            if now.saturating_sub(ready.updated_ms) <= allowed { Freshness::Live } else { Freshness::Stale }
        }
        None if row.metrics_ms.is_some() => Freshness::Stale,
        None => Freshness::Missing,
    };
}

type Stamp = Vec<Option<(u128, u64)>>;

fn stamp_of(directory: &Path) -> Stamp {
    let mut files = vec![directory.join("ready.json"), directory.join("addresses.json"), directory.join("memory.json")];
    files.extend((0..8).map(|tier| directory.join("metrics").join(format!("t{tier}.ring"))));
    files.iter().map(|path| std::fs::metadata(path).ok().map(|meta| (meta.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |span| span.as_nanos()), meta.len()))).collect()
}

struct Remembered {
    stamp: Stamp,
    window: (u64, u64, u16),
    read_ms: u64,
    row: Row,
}

/// 🧠️ What the daemon remembers between two reads: the row of every origin with the modification stamp of the files behind it. A read of an origin none of whose files changed (and that is younger than `rememberMs`) costs one `stat` per file instead of the rings; the freshness of a remembered row is judged again at every read.
#[derive(Default)]
pub struct Reader {
    rows: BTreeMap<String, Remembered>,
}

fn detail_of(origins: &Path, origin: &str, from: u64, to: u64, columns: usize) -> Option<Detail> {
    let directory = origins.join(origin);
    if !directory.is_dir() { return None; }
    let table = group_table();
    let platform = read_json(&directory.join("ready.json")).as_ref().and_then(ready_of).and_then(|ready| ready.platform);
    let unsupported = |id: &str| platform.as_deref().is_some_and(|platform| !metric_supported(id, platform));
    let ids: Vec<String> = table.iter().flat_map(|(_, _, metrics)| metrics.iter().cloned()).collect();
    let own: Vec<String> = ids.iter().filter(|id| metric_scope(id) == Some(Scope::Origin) && !unsupported(id)).cloned().collect();
    let mut samples = load_samples(&directory.join("metrics"), &own, from, to);
    if let Some(fleet) = fleet_samples(origins, &ids, from, to) { match samples.as_mut() { Some(all) => all.merge(fleet), None => samples = Some(fleet) } }
    let groups = table.into_iter().map(|(id, _, metrics)| {
        let series = metrics.into_iter().filter_map(|metric| {
            let kind = match catalogue().get(&metric)?.kind.as_str() { "counter" => SeriesKind::Counter, "state" => SeriesKind::State, _ => SeriesKind::Gauge };
            let unit: String = metric_unit(&metric).unwrap_or("count").into();
            let scope = metric_scope(&metric).unwrap_or(Scope::Origin);
            if unsupported(&metric) { return Some(Series { id: metric, kind, unit, scope, unsupported: true, points: Vec::new() }); }
            let rows = samples.as_ref()?.rows(&metric, kind);
            (!rows.is_empty()).then(|| Series { id: metric.clone(), kind, unit, scope, unsupported: false, points: thin(&rows, columns) })
        }).collect();
        Group { id, series }
    }).collect();
    Some(Detail { origin: origin.into(), groups })
}

/// 📸️ Reads the snapshot a request asks for at `now_ms`: one row per origin folder (sorted by id) and, when the request names an origin, its drill-down.
/// The window ends at the end of the current chart column so the columns stay put between two reads; `origins` is `None` when no store is configured.
pub fn read_snapshot(origins: Option<&Path>, request: &Request, now_ms: u64) -> Snapshot {
    read_snapshot_with(&mut Reader::default(), origins, request, now_ms)
}

/// 📸️ Like `read_snapshot`, reusing what `reader` remembers of the origins that did not change since its last read.
pub fn read_snapshot_with(reader: &mut Reader, origins: Option<&Path>, request: &Request, now_ms: u64) -> Snapshot {
    let request = request.clone().clamped();
    let span = request.range.span_ms();
    let step = (span / u64::from(request.columns)).max(1000);
    let to = (now_ms / step + 1) * step;
    let from = to.saturating_sub(span);
    let mut snapshot = Snapshot { at_ms: now_ms, range: request.range, columns: request.columns, from_ms: from, to_ms: to, store: StoreState::Unavailable, rows: Vec::new(), fleet: None, detail: None };
    let Some(origins) = origins else { return snapshot };
    let Ok(entries) = std::fs::read_dir(origins) else { return snapshot };
    snapshot.store = StoreState::Available;
    let mut names: Vec<String> = entries.filter_map(Result::ok).filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir())).filter_map(|entry| entry.file_name().into_string().ok()).filter(|name| !name.starts_with('.') && name.len() <= 128).collect();
    names.sort();
    let wanted: Vec<&String> = names.iter().filter(|name| request.origin.as_ref().is_none_or(|origin| origin == *name)).collect();
    let window = (from, to, request.columns);
    snapshot.rows = wanted.into_iter().map(|name| {
        let stamp = stamp_of(&origins.join(name));
        if let Some(kept) = reader.rows.get(name).filter(|kept| kept.window == window && kept.stamp == stamp && now_ms.saturating_sub(kept.read_ms) < Policy::get().remember_ms) {
            let mut row = kept.row.clone();
            classify(&mut row, now_ms);
            return row;
        }
        let row = row_of(origins, name, from, to, usize::from(request.columns), now_ms);
        reader.rows.insert(name.clone(), Remembered { stamp, window, read_ms: now_ms, row: row.clone() });
        row
    }).collect();
    reader.rows.retain(|name, _| names.contains(name));
    if request.origin.is_none() { snapshot.fleet = totals_of(origins, from, to); }
    if let Some(origin) = &request.origin { snapshot.detail = detail_of(origins, origin, from, to, usize::from(request.columns)); }
    snapshot
}
