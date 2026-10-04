//! 🔬️ Actual enclosing typed SQL ownership is measured against full system allocator requests.
use super::*;
#[path = "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observed;
#[test]
fn sqlite_snapshot_pptx_relational_owned_requests_match_actual_allocator() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🔣️.json")).unwrap();
    assert_eq!(plan["phases"], serde_json::json!(["projectSnapshot", "reconstructSnapshot"]));
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.tables.len(), usize::try_from(plan["tableCount"].as_u64().unwrap()).unwrap());
    observed::verify_snapshot_backing::<PptxSnapshot>(&snapshot, &database);
    snapshot.retire_sqlite_snapshot();
}
