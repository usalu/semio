use super::*;

const FIXTURE: &str = include_str!("../../../🧫️fixtures/📊️observability-v1/🔣️.json");

/// 🧫️ The fixture's catalog progress, read field by field into the schema projection.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CatalogProgressFixtureV1 {
    packages: Vec<PackageProgressFixtureV1>,
    packages_total: u64,
    packages_ready: u64,
    packages_refused: u64,
    component_bytes_total: u64,
    component_bytes_read: u64,
    rows_total: u64,
    rows_pinned: u64,
    rows_verified: u64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PackageProgressFixtureV1 {
    plugin_id: String,
    component_bytes: u64,
    phase: String,
    rows: u64,
    rows_pinned: u64,
    rows_verified: u64,
}

impl From<CatalogProgressFixtureV1> for TrustedCatalogLoadProgressV1 {
    fn from(fixture: CatalogProgressFixtureV1) -> Self {
        use crate::artifact_authority::trusted_catalog::schema::{TrustedCatalogPackagePhaseV1, TrustedCatalogPackageProgressV1};
        let phase = |phase: &str| match phase {
            "pending" => TrustedCatalogPackagePhaseV1::Pending,
            "reading" => TrustedCatalogPackagePhaseV1::Reading,
            "staged" => TrustedCatalogPackagePhaseV1::Staged,
            "verifying" => TrustedCatalogPackagePhaseV1::Verifying,
            "ready" => TrustedCatalogPackagePhaseV1::Ready,
            "refused" => TrustedCatalogPackagePhaseV1::Refused,
            other => panic!("undeclared phase {other}"),
        };
        Self {
            packages: fixture.packages.into_iter().map(|package| TrustedCatalogPackageProgressV1 { plugin_id: package.plugin_id, component_bytes: package.component_bytes, phase: phase(&package.phase), rows: package.rows, rows_pinned: package.rows_pinned, rows_verified: package.rows_verified }).collect(),
            packages_total: fixture.packages_total,
            packages_ready: fixture.packages_ready,
            packages_refused: fixture.packages_refused,
            component_bytes_total: fixture.component_bytes_total,
            component_bytes_read: fixture.component_bytes_read,
            rows_total: fixture.rows_total,
            rows_pinned: fixture.rows_pinned,
            rows_verified: fixture.rows_verified,
        }
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("observability fixture")
}

fn required(def: &str) -> std::collections::BTreeSet<String> {
    let module: serde_json::Value = serde_json::from_str(HUB_OBSERVABILITY_SCHEMA_JSON).expect("observability schema");
    module["$defs"][def]["required"].as_array().expect("required fields").iter().map(|field| field.as_str().expect("field name").to_string()).collect()
}

fn keys(value: &serde_json::Value) -> std::collections::BTreeSet<String> {
    value.as_object().expect("object").keys().cloned().collect()
}

/// 🛣️ The language-neutral route law: the fixture's observations, fed in order, yield exactly its route rows —
/// answers by class, and each route's p50/p95/p99 over its last 64 clamped samples.
#[test]
fn route_metrics_count_answers_by_class_and_take_percentiles_of_the_last_64_samples() {
    let fixture = fixture();
    let metrics = HubRouteMetricsV1::default();
    for observation in fixture["observations"].as_array().expect("observations") {
        metrics.observe(observation["method"].as_str().unwrap(), observation["route"].as_str().unwrap(), observation["status"].as_u64().unwrap() as u16, observation["durationUs"].as_u64().unwrap());
    }
    assert_eq!(serde_json::to_value(metrics.rows()).unwrap(), fixture["routes"]);
    assert_eq!(metrics.overflowed(), 0);
}

/// 🧯️ The route table never grows past its bound: an answer of a route beyond it is only counted as overflow.
#[test]
fn route_metrics_are_bounded_by_the_declared_route_count() {
    let metrics = HubRouteMetricsV1::default();
    for route in 0..=HUB_ROUTE_METRICS_MAX_ROUTES {
        metrics.observe("GET", &format!("/route-{route}"), 200, 1);
    }
    metrics.observe("GET", "/route-0", 200, 1);
    assert_eq!(metrics.rows().len(), HUB_ROUTE_METRICS_MAX_ROUTES);
    assert_eq!(metrics.overflowed(), 1, "only the route beyond the bound overflowed; a known route keeps counting");
    let module: serde_json::Value = serde_json::from_str(HUB_OBSERVABILITY_SCHEMA_JSON).unwrap();
    assert_eq!(module["$defs"]["HubObservabilityV1"]["properties"]["routes"]["maxItems"].as_u64(), Some(HUB_ROUTE_METRICS_MAX_ROUTES as u64));
}

/// 📊️ The body this projection serializes is the fixture's complete `HubObservabilityV1` for the same inputs,
/// and every struct serializes exactly the fields its schema definition requires.
#[test]
fn the_observability_body_is_the_declared_schema() {
    let fixture = fixture();
    let body = &fixture["body"];
    let metrics = HubRouteMetricsV1::default();
    for observation in fixture["observations"].as_array().unwrap() {
        metrics.observe(observation["method"].as_str().unwrap(), observation["route"].as_str().unwrap(), observation["status"].as_u64().unwrap() as u16, observation["durationUs"].as_u64().unwrap());
    }
    let residency: TrustedCatalogGuestResidencyStateV1 = {
        let state = &body["residency"];
        let field = |name: &str| state[name].as_u64().unwrap();
        TrustedCatalogGuestResidencyStateV1 {
            budget_bytes: field("budgetBytes"),
            registered_guests: field("registeredGuests"),
            resident_guests: field("residentGuests"),
            resident_bytes: field("residentBytes"),
            hits: field("hits"),
            compiles: field("compiles"),
            admitted: field("admitted"),
            bypassed: field("bypassed"),
            released: field("released"),
            compile_micros: field("compileMicros"),
        }
    };
    let count = |value: &serde_json::Value, field: &str| value[field].as_u64().unwrap();
    let db_io = HubDbIoCensusV1 {
        tasks: count(&body["dbIo"], "tasks"),
        steps: count(&body["dbIo"], "steps"),
        turns: count(&body["dbIo"], "turns"),
        admission_waits: count(&body["dbIo"], "admissionWaits"),
        admission_wait_micros: count(&body["dbIo"], "admissionWaitMicros"),
        admission_refusals: count(&body["dbIo"], "admissionRefusals"),
        kinds: body["dbIo"]["kinds"]
            .as_array()
            .unwrap()
            .iter()
            .map(|kind| HubDbIoKindV1 {
                kind: kind["kind"].as_str().unwrap().into(),
                tasks: count(kind, "tasks"),
                steps: count(kind, "steps"),
                turns: count(kind, "turns"),
                admission_waits: count(kind, "admissionWaits"),
                admission_wait_micros: count(kind, "admissionWaitMicros"),
                admission_refusals: count(kind, "admissionRefusals"),
            })
            .collect(),
    };
    let reading = HubObservabilityV1 {
        schema: HUB_OBSERVABILITY_SCHEMA,
        level: "info",
        uptime_ms: body["uptimeMs"].as_u64().unwrap(),
        dropped_events: 0,
        declared_events: SERVER_SPAN_EVENTS.to_vec(),
        rows: Vec::new(),
        routes: metrics.rows(),
        routes_overflowed: metrics.overflowed(),
        residency: Some(residency),
        catalog: Some(serde_json::from_value::<CatalogProgressFixtureV1>(body["catalog"].clone()).unwrap().into()),
        db_io,
    };
    let serialized = serde_json::to_value(&reading).unwrap();
    assert_eq!(&serialized, body);
    assert_eq!(keys(&serialized), required("HubObservabilityV1"));
    assert_eq!(keys(&serialized["routes"][0]), required("HubRouteLatencyV1"));
    assert_eq!(keys(&serialized["dbIo"]), required("HubDbIoCensusV1"));
    assert_eq!(keys(&serialized["dbIo"]["kinds"][0]), required("HubDbIoKindV1"));
    let row = serde_json::to_value(HubTraceEventRowV1 { event: "server.boot".into(), started: 0, ok: 1, refused: 0, failed: 0, cancelled: 0, total: 1, samples: 0, p50_us: 0, p95_us: 0, p99_us: 0 }).unwrap();
    assert_eq!(keys(&row), required("HubTraceEventRowV1"));
    assert_eq!(keys(&serialized["catalog"]["packages"][0]).len(), 6);
    let absent = serde_json::to_value(HubObservabilityV1 { residency: None, catalog: None, ..reading }).unwrap();
    assert!(absent["residency"].is_null() && absent["catalog"].is_null(), "a hub without a catalog reports residency and catalog null");
}

/// 🗄️ The census reading is the db crate's own census: totals are the sums over every kind, and a kind is listed
/// exactly when it saw I/O.
#[test]
fn the_db_io_census_reading_lists_every_kind_that_saw_io() {
    let reading = HubDbIoCensusV1::read();
    assert_eq!(reading.tasks, reading.kinds.iter().map(|kind| kind.tasks).sum::<u64>());
    assert_eq!(reading.steps, reading.kinds.iter().map(|kind| kind.steps).sum::<u64>());
    assert_eq!(reading.turns, reading.kinds.iter().map(|kind| kind.turns).sum::<u64>());
    assert_eq!(reading.admission_waits, reading.kinds.iter().map(|kind| kind.admission_waits).sum::<u64>());
    assert_eq!(reading.admission_refusals, reading.kinds.iter().map(|kind| kind.admission_refusals).sum::<u64>());
    assert!(reading.kinds.iter().all(|kind| kind.tasks != 0 || kind.steps != 0 || kind.turns != 0 || kind.admission_waits != 0));
    assert!(reading.kinds.iter().all(|kind| kind.admission_refusals <= kind.admission_waits));
    assert!(reading.kinds.iter().all(|kind| db::db_storage::DbIoTaskKind::ALL.iter().any(|declared| format!("{declared:?}") == kind.kind)));
}
