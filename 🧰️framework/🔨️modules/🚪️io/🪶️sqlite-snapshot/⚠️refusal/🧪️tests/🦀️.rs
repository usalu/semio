use super::*;
use semio_framework_value::ValueError;

fn specimen() -> SqliteDatabase {
    SqliteDatabase { tables: vec![SqliteTable { name: "entity".into(), sql: "CREATE TABLE entity (id INTEGER PRIMARY KEY, value TEXT)".into(), rows: vec![SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text("owned".into())] }] }] }
}

fn refusal(operation: &str) -> ValueError {
    let mut limits = SqliteDatabaseLimits::default();
    match operation {
        "encoding" => SnapshotEncoding::parse("unknown").unwrap_err(),
        "schema" => SqliteDatabase::from_schema("CREATE VIEW entity AS SELECT 1").unwrap_err(),
        "nan" => export_sqlite_database(&SqliteDatabase { tables: vec![SqliteTable { name: "entity".into(), sql: "CREATE TABLE entity (value REAL)".into(), rows: vec![SqliteRow { rowid: 1, values: vec![SqliteValue::Real(f64::NAN)] }] }] }, limits, &mut |_| true).unwrap_err(),
        "pages" => export_sqlite_database(&specimen(), limits, &mut |_| false).unwrap_err(),
        "projection" => match artifact::Projection::new("CREATE TABLE entity (id INTEGER PRIMARY KEY, value TEXT)", &mut SqliteSnapshotControl::new(&mut |_| false, limits)) { Err(error) => error, Ok(_) => panic!("canceled projection accepted") },
        "allocation" => { limits.max_allocation_bytes = 0; SqliteSnapshotControl::new(&mut |_| true, limits).admit_allocation_bytes(1).unwrap_err() }
        "value" | "schemaBytes" | "file" | "rows" | "columns" | "tables" | "pageCount" => {
            match operation { "value" => limits.max_value_bytes = 0, "schemaBytes" => limits.max_schema_bytes = 0, "file" => limits.max_file_bytes = 0, "rows" => limits.max_rows = 0, "columns" => limits.max_columns = 0, "tables" => limits.max_tables = 0, "pageCount" => limits.max_pages = 0, _ => unreachable!() }
            export_sqlite_database(&specimen(), limits, &mut |_| true).unwrap_err()
        }
        _ => panic!("unowned SQLite fixture operation"),
    }
}

#[test]
fn sqlite_refusal_producers_retain_language_neutral_owned_authorities() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let rows = corpus["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 13);
    for row in rows {
        let error = refusal(row["operation"].as_str().unwrap());
        assert_eq!(error.kind.as_str(), row["expectedKind"].as_str().unwrap(), "{}", row["id"]);
        assert!(!error.message.is_empty());
        assert_eq!(error.clone().under("document").kind, error.kind);
    }
    eprintln!("[DEBUG] Thirteen actual SQLite native producer kinds retained");
}

