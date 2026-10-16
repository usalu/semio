//! 🔬️ Actual enclosing typed SQL ownership is measured against full system allocator requests.
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::tests::*;
use semio_framework_io_sqlite_snapshot_testing_backing as observed;
#[test]
fn sqlite_snapshot_xlsx_relational_owned_requests_match_actual_allocator() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/💰️backing/🔬️requests/🔣️.json")).unwrap();
    assert_eq!(plan["phases"], serde_json::json!(["projectSnapshot", "reconstructSnapshot"]));
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.tables.len(), usize::try_from(plan["tableCount"].as_u64().unwrap()).unwrap());
    observed::verify_snapshot_backing::<XlsxSnapshot>(&snapshot, &database);
    snapshot.retire_sqlite_snapshot();
}
