//! 🔬️ Actual enclosing typed SQL ownership is measured against full system allocator requests.
use crate::standards::v_ecma_376::subsets::base::io::sqlite::snapshot::tests::*;
#[path = "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observed;
#[test]
fn sqlite_snapshot_docx_relational_owned_requests_match_actual_allocator(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/💰️backing/🔬️requests/🔣️.json")).unwrap();
 assert_eq!(plan["phases"],serde_json::json!(["projectSnapshot","reconstructSnapshot"]));
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 assert_eq!(database.tables.len(),usize::try_from(plan["tableCount"].as_u64().unwrap()).unwrap());
 observed::verify_snapshot_backing::<DocxSnapshot>(&snapshot,&database);
 snapshot.retire_sqlite_snapshot();
}
