//! 📊️ Rust projection of `HubObservabilityV1` ([`🔣️.json`](🧬️schema/🔣️.json)): the counters and latency
//! percentiles `GET /admin/api/observability` answers an administrator — per trace event, per registered
//! route, for the compiled-guest residency and for the process's DB I/O. Counters, never records: the
//! records go to the sink the operator configured.

use crate::artifact_authority::trusted_catalog::schema::{TrustedCatalogGuestResidencyStateV1, TrustedCatalogLoadProgressV1};
use semio_framework_trace::record::{EventCount, Tracer, SERVER_SPAN_EVENTS};
use semio_framework_trace::PercentileRing;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// 🧬️ The draft-07 module this projection is checked against.
pub const HUB_OBSERVABILITY_SCHEMA_JSON: &str = include_str!("🧬️schema/🔣️.json");

/// 🏷️ The `schema` of one `HubObservabilityV1` body.
pub const HUB_OBSERVABILITY_SCHEMA: &str = "semio.hub.observability/v1";

/// 🧯️ Most method and route-template pairs [`HubRouteMetricsV1`] keeps rows for (`routes.maxItems`).
pub const HUB_ROUTE_METRICS_MAX_ROUTES: usize = 256;

/// 📊️ `HubObservabilityV1`: one administrator's reading of this hub.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HubObservabilityV1 {
    pub schema: &'static str,
    pub level: &'static str,
    pub uptime_ms: u64,
    pub dropped_events: u64,
    pub declared_events: Vec<&'static str>,
    pub rows: Vec<HubTraceEventRowV1>,
    pub routes: Vec<HubRouteLatencyV1>,
    pub routes_overflowed: u64,
    pub residency: Option<TrustedCatalogGuestResidencyStateV1>,
    pub catalog: Option<TrustedCatalogLoadProgressV1>,
    pub db_io: HubDbIoCensusV1,
}

/// 📝️ `HubTraceEventRowV1`: one trace event's counters since boot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HubTraceEventRowV1 {
    pub event: String,
    pub started: u64,
    pub ok: u64,
    pub refused: u64,
    pub failed: u64,
    pub cancelled: u64,
    pub total: u64,
    pub samples: u64,
    pub p50_us: u32,
    pub p95_us: u32,
    pub p99_us: u32,
}

impl From<EventCount> for HubTraceEventRowV1 {
    fn from(EventCount { event, counters }: EventCount) -> Self {
        let (p50_us, p95_us, p99_us) = counters.percentiles_us();
        Self { event, started: counters.started, ok: counters.ok, refused: counters.refused, failed: counters.failed, cancelled: counters.cancelled, total: counters.total(), samples: counters.samples() as u64, p50_us, p95_us, p99_us }
    }
}

/// 🛣️ `HubRouteLatencyV1`: one registered route's answers since boot, by class, and its latency percentiles.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HubRouteLatencyV1 {
    pub method: String,
    pub route: String,
    pub requests: u64,
    pub successes: u64,
    pub client_refusals: u64,
    pub rate_limited: u64,
    pub unavailable: u64,
    pub server_failures: u64,
    pub samples: u64,
    pub p50_us: u32,
    pub p95_us: u32,
    pub p99_us: u32,
    pub max_us: u32,
}

/// 🗄️ `HubDbIoKindV1`: the DB I/O one task kind cost since the process started, and its admission waits.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HubDbIoKindV1 {
    pub kind: String,
    pub tasks: u64,
    pub steps: u64,
    pub turns: u64,
    pub admission_waits: u64,
    pub admission_wait_micros: u64,
    pub admission_refusals: u64,
}

/// 🗄️ `HubDbIoCensusV1`: the process-wide DB I/O census.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HubDbIoCensusV1 {
    pub tasks: u64,
    pub steps: u64,
    pub turns: u64,
    pub admission_waits: u64,
    pub admission_wait_micros: u64,
    pub admission_refusals: u64,
    pub kinds: Vec<HubDbIoKindV1>,
}

impl HubDbIoCensusV1 {
    /// 🧮️ The census `db::db_storage::db_io_census` reads now: totals and every kind that saw I/O or waited.
    pub fn read() -> Self {
        let census = db::db_storage::db_io_census();
        let kinds = db::db_storage::DbIoTaskKind::ALL
            .iter()
            .map(|kind| HubDbIoKindV1 {
                kind: format!("{kind:?}"),
                tasks: census.tasks(*kind),
                steps: census.steps(*kind),
                turns: census.turns(*kind),
                admission_waits: census.admission_waits(*kind),
                admission_wait_micros: census.admission_wait_micros(*kind),
                admission_refusals: census.admission_refusals(*kind),
            })
            .filter(|kind| kind.tasks != 0 || kind.steps != 0 || kind.turns != 0 || kind.admission_waits != 0)
            .collect();
        Self {
            tasks: census.total_tasks(),
            steps: census.total_steps(),
            turns: census.total_turns(),
            admission_waits: census.total_admission_waits(),
            admission_wait_micros: census.total_admission_wait_micros(),
            admission_refusals: census.total_admission_refusals(),
            kinds,
        }
    }
}

/// 🛣️ One route's running counters.
#[derive(Clone)]
struct RouteCountersV1 {
    requests: u64,
    successes: u64,
    client_refusals: u64,
    rate_limited: u64,
    unavailable: u64,
    server_failures: u64,
    latency: PercentileRing,
    max_us: u32,
}

/// 🛣️ Per-route answer counters and latency rings, keyed by method and route template. Bounded by
/// [`HUB_ROUTE_METRICS_MAX_ROUTES`]; an answer of a route beyond the bound is only counted as overflow.
#[derive(Default)]
pub struct HubRouteMetricsV1 {
    rows: Mutex<BTreeMap<(String, String), RouteCountersV1>>,
    overflowed: std::sync::atomic::AtomicU64,
}

impl HubRouteMetricsV1 {
    /// 📥️ Counts one answer of `method` `route` with `status`, taken `duration_us`.
    pub fn observe(&self, method: &str, route: &str, status: u16, duration_us: u64) {
        let mut rows = self.rows.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = (method.to_string(), route.to_string());
        if !rows.contains_key(&key) && rows.len() >= HUB_ROUTE_METRICS_MAX_ROUTES {
            self.overflowed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return;
        }
        let row = rows.entry(key).or_insert_with(|| RouteCountersV1 { requests: 0, successes: 0, client_refusals: 0, rate_limited: 0, unavailable: 0, server_failures: 0, latency: PercentileRing::new(), max_us: 0 });
        row.requests = row.requests.saturating_add(1);
        let class = match status {
            0..=399 => &mut row.successes,
            429 => &mut row.rate_limited,
            400..=499 => &mut row.client_refusals,
            503 => &mut row.unavailable,
            _ => &mut row.server_failures,
        };
        *class = class.saturating_add(1);
        row.latency.record(duration_us);
        row.max_us = row.max_us.max(u32::try_from(duration_us).unwrap_or(u32::MAX));
    }

    /// 📋️ Every route's row, sorted by route template then method.
    pub fn rows(&self) -> Vec<HubRouteLatencyV1> {
        let rows = self.rows.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut answered: Vec<HubRouteLatencyV1> = rows
            .iter()
            .map(|((method, route), row)| HubRouteLatencyV1 {
                method: method.clone(),
                route: route.clone(),
                requests: row.requests,
                successes: row.successes,
                client_refusals: row.client_refusals,
                rate_limited: row.rate_limited,
                unavailable: row.unavailable,
                server_failures: row.server_failures,
                samples: row.latency.len() as u64,
                p50_us: row.latency.p50(),
                p95_us: row.latency.p95(),
                p99_us: row.latency.p99(),
                max_us: row.max_us,
            })
            .collect();
        answered.sort_by(|left, right| (&left.route, &left.method).cmp(&(&right.route, &right.method)));
        answered
    }

    /// 🧯️ Answers of routes beyond the table's bound.
    pub fn overflowed(&self) -> u64 {
        self.overflowed.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl HubObservabilityV1 {
    /// 📊️ One reading: the tracer's event table, the route table, the residency and catalog progress (when a
    /// catalog is loaded) and the DB I/O census, `uptime_ms` after boot.
    pub fn read(tracer: &Tracer, routes: &HubRouteMetricsV1, catalog: Option<&crate::artifact_authority::trusted_catalog::VerifiedTrustedCatalog>, uptime_ms: u64) -> Self {
        Self {
            schema: HUB_OBSERVABILITY_SCHEMA,
            level: tracer.level().as_str(),
            uptime_ms,
            dropped_events: tracer.dropped_event_count(),
            declared_events: SERVER_SPAN_EVENTS.to_vec(),
            rows: tracer.counters().into_iter().map(HubTraceEventRowV1::from).collect(),
            routes: routes.rows(),
            routes_overflowed: routes.overflowed(),
            residency: catalog.map(|catalog| catalog.guest_residency()),
            catalog: catalog.map(|catalog| catalog.load_progress()),
            db_io: HubDbIoCensusV1::read(),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
