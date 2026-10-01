use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

#[test]
fn sqlite_snapshot_csv_reconstruction_respects_value_budget() {
    let database = CsvSnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(CsvSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}

#[test]
fn sqlite_snapshot_csv_wide_record_preflight_can_be_cancelled() {
    let snapshot = CsvSnapshot { records: vec![CsvRecord { fields: vec![CsvField { value: String::new(), quoted: false }; 1024] }], ..CsvSnapshot::default() };
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    let mut checkpoints = 0;
    let error = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |progress| { if progress.total == 1025 { checkpoints += 1; } checkpoints < 3 }, limits)).unwrap_err();
    assert!(error.contains("cancelled"), "{error}");
}

#[test]
fn sqlite_snapshot_csv_records_fields_and_quoting_are_relational() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let snapshot = CsvSnapshot {
        schema: fixture["schema"].as_str().unwrap().into(),
        has_header: fixture["hasHeader"].as_bool().unwrap(),
        records: fixture["records"].as_array().unwrap().iter().map(|row| CsvRecord { fields: row["fields"].as_array().unwrap().iter().map(|field| CsvField { value: field["value"].as_str().unwrap().into(), quoted: field["quoted"].as_bool().unwrap() }).collect() }).collect(),
    };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("csv_record").unwrap().rows.len(), 4);
    assert_eq!(database.table("csv_field").unwrap().rows.len(), 5);
    assert_eq!(database.table("csv_field").unwrap().rows[3].values[3], SqliteValue::Text("Grüße,\nLesesaal".into()));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(CsvSnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, fixture);
    let mut broken = reopened;
    broken.table_mut("csv_field").unwrap().rows[0].values[1] = SqliteValue::Integer(999);
    assert!(CsvSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}
