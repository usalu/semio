use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

#[test]
fn sqlite_snapshot_text_reconstruction_respects_value_budget() {
    let database = TxtSnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(TxtSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}

#[test]
fn sqlite_snapshot_text_value_preflight_can_be_cancelled() {
    let snapshot = TxtSnapshot { lines: vec![String::new(); 1024], ..TxtSnapshot::default() };
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    let mut checkpoints = 0;
    let error = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| { checkpoints += 1; checkpoints < 2 }, limits)).unwrap_err();
    assert!(error.contains("cancelled"), "{error}");
}

#[test]
fn sqlite_snapshot_text_lines_preserve_order_and_line_endings() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let snapshot = TxtSnapshot { schema: fixture["schema"].as_str().unwrap().into(), lines: fixture["lines"].as_array().unwrap().iter().map(|line| line.as_str().unwrap().into()).collect(), trailing_newline: true, line_ending: LineEnding::CrLf };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("text_line").unwrap().rows.len(), 3);
    assert_eq!(database.table("text_document").unwrap().rows[0].values[3], SqliteValue::Text("crlf".into()));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(TxtSnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, fixture);
    let mut broken = reopened;
    broken.table_mut("text_line").unwrap().rows[1].values[2] = SqliteValue::Integer(0);
    assert!(TxtSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}
