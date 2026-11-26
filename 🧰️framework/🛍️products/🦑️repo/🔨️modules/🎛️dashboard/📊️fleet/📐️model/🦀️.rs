//! 📐️ The fleet model: ranges, the request, the rows and the snapshot, as they travel on the daemon wire. Every number that
//! decides what a row is comes from the schema, so another implementation reads the same policy.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/📊️fleet/🔣️.json

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

const SCHEMA: &str = include_str!("../../🧬️schema/📊️fleet/🔣️.json");

/// 📍️ One point of a series: `[start, width, min, max, mean, last]` in milliseconds and the unit of its metric.
pub type Point = [f64; 6];

fn definition(name: &str) -> serde_json::Value {
    static PARSED: OnceLock<serde_json::Value> = OnceLock::new();
    PARSED.get_or_init(|| serde_json::from_str(SCHEMA).expect("the fleet schema is valid JSON"))["definitions"][name]["const"].clone()
}

/// 🧮️ The numbers of the fleet policy.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    pub stale_beats: u64,
    pub default_beat_ms: u64,
    pub stale_metrics_ms: u64,
    pub overview_columns: u16,
    pub minimum_columns: u16,
    pub maximum_columns: u16,
    pub minimum_run_columns: usize,
    pub heartbeat_ms: u64,
    pub remember_ms: u64,
    pub decimals: u32,
}

impl Policy {
    /// 📜️ The policy the schema declares.
    pub fn get() -> &'static Policy {
        static POLICY: OnceLock<Policy> = OnceLock::new();
        POLICY.get_or_init(|| serde_json::from_value(definition("Policy")).expect("the fleet policy is closed"))
    }
}

#[derive(Deserialize)]
struct RangeRow {
    id: String,
    #[serde(rename = "spanMs")]
    span_ms: u64,
    #[serde(rename = "refreshMs")]
    refresh_ms: u64,
    label: std::collections::BTreeMap<String, String>,
}

fn range_rows() -> &'static Vec<RangeRow> {
    static ROWS: OnceLock<Vec<RangeRow>> = OnceLock::new();
    ROWS.get_or_init(|| serde_json::from_value(definition("Ranges")).expect("the fleet ranges are closed"))
}

/// 🔭️ A zoom level of the fleet window, finest first.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Range {
    #[default]
    #[serde(rename = "5m")]
    Minutes5,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "24h")]
    Hours24,
    #[serde(rename = "7d")]
    Days7,
    #[serde(rename = "90d")]
    Days90,
}

impl Range {
    pub const ALL: [Range; 5] = [Range::Minutes5, Range::Hour1, Range::Hours24, Range::Days7, Range::Days90];

    fn index(self) -> usize {
        Self::ALL.iter().position(|range| *range == self).unwrap_or(0)
    }

    /// 🔤️ The id the wire and the schema use: `5m`, `1h`, `24h`, `7d`, `90d`.
    pub fn id(self) -> &'static str {
        &range_rows()[self.index()].id
    }

    /// ⏱️ How much time the range shows.
    pub fn span_ms(self) -> u64 {
        range_rows()[self.index()].span_ms
    }

    /// 🔁️ The least time between two reads of the rings for this range.
    pub fn refresh_ms(self) -> u64 {
        range_rows()[self.index()].refresh_ms.max(1000)
    }

    /// 🗣️ The label of the range in `language` (`en`, `de`); an unknown language shows the id.
    pub fn label(self, language: &str) -> &'static str {
        range_rows()[self.index()].label.get(language).map_or_else(|| self.id(), String::as_str)
    }

    /// ➡️ The next coarser range, or the same one at the end.
    pub fn coarser(self) -> Range {
        Self::ALL[(self.index() + 1).min(Self::ALL.len() - 1)]
    }

    /// ⬅️ The next finer range, or the same one at the start.
    pub fn finer(self) -> Range {
        Self::ALL[self.index().saturating_sub(1)]
    }
}

/// 🙋️ What the dashboard wants to see: a range, how many columns its charts have and optionally one origin to drill into.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub range: Range,
    pub columns: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

impl Request {
    /// 📏️ The request with its column budget inside the policy.
    pub fn clamped(mut self) -> Self {
        let policy = Policy::get();
        self.columns = self.columns.clamp(policy.minimum_columns, policy.maximum_columns);
        self
    }

    /// ✅️ Whether the request is well formed: a known column budget and an origin id that is a single path segment.
    pub fn validate(&self) -> std::io::Result<()> {
        let policy = Policy::get();
        let invalid = |message: &str| std::io::Error::new(std::io::ErrorKind::InvalidInput, message.to_string());
        if self.columns < policy.minimum_columns || self.columns > policy.maximum_columns { return Err(invalid("invalid fleet column budget")); }
        if let Some(origin) = &self.origin {
            if origin.is_empty() || origin.len() > 128 || origin.starts_with('.') || origin.contains(['/', '\\', '\0']) { return Err(invalid("invalid fleet origin")); }
        }
        Ok(())
    }
}

/// 🌿️ How alive an origin is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Freshness { Live, Stale, Missing }

/// 🏷️ What the published addresses say about the machine name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostName { Ok, Warning, Unknown }

/// 🗄️ Whether the shared store could be read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoreState { Available, Unavailable }

/// 🚦️ The readiness record of an origin.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Ready {
    pub state: String,
    pub ready: bool,
    pub stage: Option<String>,
    pub stage_index: u64,
    pub stage_count: u64,
    pub updated_ms: u64,
    pub beat_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
}

/// 🧾️ One origin of the overview.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Row {
    pub origin: String,
    pub freshness: Freshness,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<Ready>,
    pub host_name: HostName,
    pub duplicates: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units_per_minute: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_hit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_miss: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_hit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_miss: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offload_in: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offload_out: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cpu_series: Vec<Point>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub units_series: Vec<Point>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unsupported: Vec<String>,
}

impl Row {
    /// 🕳️ A row that knows nothing but the origin id.
    pub fn empty(origin: &str) -> Self {
        Self { origin: origin.into(), freshness: Freshness::Missing, ready: None, host_name: HostName::Unknown, duplicates: 0, metrics_ms: None, cpu: None, memory: None, disk: None, units_per_minute: None, local_hit: None, local_miss: None, remote_hit: None, remote_miss: None, offload_in: None, offload_out: None, cpu_series: Vec::new(), units_series: Vec::new(), unsupported: Vec::new() }
    }
}

/// 🔢️ How a metric is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeriesKind { Gauge, Counter, State }

/// 📈️ One metric of the drill-down; a counter is already a rate per minute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Series {
    pub id: String,
    pub kind: SeriesKind,
    pub unit: String,
    pub scope: Scope,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unsupported: bool,
    pub points: Vec<Point>,
}

/// 🌐️ Where a metric is measured: by every machine for itself, or once for the whole fleet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope { Origin, Fleet }

/// 🌍️ The fleet-scope facts of a snapshot.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Totals {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offload_in: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_running: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_queued: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_entries: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_bytes: Option<f64>,
}

/// 🧩️ The metrics of one group of the drill-down.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    pub id: String,
    pub series: Vec<Series>,
}

/// 🔬️ The drill-down of one origin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Detail {
    pub origin: String,
    pub groups: Vec<Group>,
}

/// 📸️ What the daemon tells the dashboard: the rows of the overview and, on request, the drill-down of one origin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Snapshot {
    pub at_ms: u64,
    pub range: Range,
    pub columns: u16,
    pub from_ms: u64,
    pub to_ms: u64,
    pub store: StoreState,
    pub rows: Vec<Row>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fleet: Option<Totals>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Detail>,
}

/// ⚖️ A snapshot carries finite numbers only (every number is rounded and travels as JSON), so equality is total.
impl Eq for Snapshot {}

impl Snapshot {
    /// 🪞 Whether two snapshots tell the same thing; the time they were read at does not count.
    pub fn same_content(&self, other: &Snapshot) -> bool {
        self.range == other.range && self.columns == other.columns && self.from_ms == other.from_ms && self.to_ms == other.to_ms && self.store == other.store && self.rows == other.rows && self.fleet == other.fleet && self.detail == other.detail
    }

    /// 🎯️ Whether this snapshot answers `request`: the same range and the same origin (a drill-down of an origin the store does not have answers with no rows and no detail).
    pub fn answers(&self, request: &Request) -> bool {
        self.range == request.range && match &request.origin { None => self.detail.is_none(), Some(origin) => self.detail.as_ref().map_or(self.rows.is_empty(), |detail| &detail.origin == origin) }
    }

    /// 🧐️ The row of an origin.
    pub fn row(&self, origin: &str) -> Option<&Row> {
        self.rows.iter().find(|row| row.origin == origin)
    }
}

/// 🧱️ The group table of the schema: id, English and German label, metric ids.
pub fn group_table() -> Vec<(String, std::collections::BTreeMap<String, String>, Vec<String>)> {
    #[derive(Deserialize)]
    struct GroupRow { id: String, label: std::collections::BTreeMap<String, String>, metrics: Vec<String> }
    serde_json::from_value::<Vec<GroupRow>>(definition("Groups")).expect("the fleet groups are closed").into_iter().map(|row| (row.id, row.label, row.metrics)).collect()
}

/// 🧮️ The metric ids the overview reads, by role.
pub fn overview_ids() -> std::collections::BTreeMap<String, String> {
    serde_json::from_value(definition("Overview")).expect("the fleet overview is closed")
}
