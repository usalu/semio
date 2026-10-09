use super::*;

#[test]
fn neutral_database_validation_progress_advances_and_cancels_at_the_shared_frontier(){use std::io::Write;use std::process::{Command,Stdio};let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️database-control/🔣️.json")).unwrap();let count=f["rows"].as_u64().unwrap()as usize;let mut d=SqliteDatabase::from_schema(f["sql"].as_str().unwrap()).unwrap();for id in 1..=count{d.tables[0].rows.push(SqliteRow{rowid:id as i64,values:vec![SqliteValue::Integer(id as i64),SqliteValue::Null]});}let bytes=encode(&d);let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));console.log(String(d.query('SELECT count(*) AS n FROM entity').get().n));d.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap().trim().parse::<usize>().unwrap(),count);let mut observed=Vec::new();SqliteSnapshotControl::new(&mut|p|{assert_eq!(p.phase,SqliteSnapshotPhase::ReconstructSnapshot);observed.push((p.completed,p.total));true},SqliteDatabaseLimits::default()).check_database(&d,SqliteSnapshotPhase::ReconstructSnapshot).unwrap();let expected:Vec<_>=f["checkpoints"].as_array().unwrap().iter().map(|p|(p[0].as_u64().unwrap()as usize,p[1].as_u64().unwrap()as usize)).collect();assert_eq!(observed,expected);let mut observed=Vec::new();assert!(SqliteSnapshotControl::new(&mut|p|{observed.push(p.completed);p.completed!=f["cancelAt"].as_u64().unwrap()as usize},SqliteDatabaseLimits::default()).check_database(&d,SqliteSnapshotPhase::ReconstructSnapshot).is_err());assert_eq!(observed,vec![0,256]);}

fn corpus() -> SqliteDatabase {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏛️relational/🔣️.json")).unwrap();
    SqliteDatabase { tables: fixture["tables"].as_array().unwrap().iter().map(|table| SqliteTable {
        name: table["name"].as_str().unwrap().into(), sql: table["sql"].as_str().unwrap().into(), rows: table["rows"].as_array().unwrap().iter().map(|row| SqliteRow {
            rowid: row["rowid"].as_str().unwrap().parse().unwrap(), values: row["values"].as_array().unwrap().iter().map(|value| match value {
                serde_json::Value::Null => SqliteValue::Null,
                serde_json::Value::String(value) => SqliteValue::Text(value.clone()),
                serde_json::Value::Number(value) => SqliteValue::Real(value.as_f64().unwrap()),
                value if value.get("integer").is_some() => SqliteValue::Integer(value["integer"].as_str().unwrap().parse().unwrap()),
                value => SqliteValue::Blob(value["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()),
            }).collect(),
        }).collect(),
    }).collect() }
}

fn encode(database: &SqliteDatabase) -> Vec<u8> { export_sqlite_database(database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap() }
fn decode(bytes: &[u8]) -> Result<SqliteDatabase> { import_sqlite_database(bytes, SqliteDatabaseLimits::default(), &mut |_| true) }

#[test]
fn language_neutral_relational_corpus_round_trips_typed_entities() {
    let database = corpus(); assert_eq!(decode(&encode(&database)).unwrap(), database);
}

#[test]
fn schema_first_tables_and_typed_accessors_are_available_to_artifact_providers() {
    let fixture = corpus(); let schema = fixture.tables.iter().map(|table| table.sql.as_str()).collect::<Vec<_>>().join(";\n");
    let mut database = SqliteDatabase::from_schema(&schema).unwrap(); assert!(database.tables.iter().all(|table| table.rows.is_empty()));
    database.table_mut("building").unwrap().rows = fixture.table("building").unwrap().rows.clone();
    let row = &database.table("building").unwrap().rows[0]; assert_eq!(row.integer(0).unwrap(), 1); assert_eq!(row.text(1).unwrap(), "Bibliothek 🌠"); assert_eq!(row.real(2).unwrap(), 12.5);
    assert!(row.blob(1).is_err()); assert!(row.optional_text(99).is_err()); assert!(database.table("missing").is_err());
}

#[test]
fn several_leaf_and_interior_levels_preserve_sorted_signed_rowids() {
    let mut database = SqliteDatabase::from_schema("CREATE TABLE entity (id INTEGER PRIMARY KEY, value TEXT NOT NULL)").unwrap();
    for rowid in (-5000..5000).rev() { database.tables[0].rows.push(SqliteRow { rowid, values: vec![SqliteValue::Integer(rowid), SqliteValue::Text("v".repeat(300))] }); }
    let bytes = encode(&database); database.tables[0].rows.sort_by_key(|row| row.rowid); assert_eq!(decode(&bytes).unwrap(), database); assert_eq!(bytes[4096], 5);
}

#[test]
fn overflow_retains_arbitrary_blob_utf8_and_nullable_columns() {
    let mut database = SqliteDatabase::from_schema("CREATE TABLE entity (id INTEGER, text TEXT, bytes BLOB)").unwrap();
    database.tables[0].rows.push(SqliteRow { rowid: i64::MIN, values: vec![SqliteValue::Null, SqliteValue::Text("🌠\0é".repeat(10_000)), SqliteValue::Blob((0..=255).cycle().take(1_048_576).collect())] }); assert_eq!(decode(&encode(&database)).unwrap(), database);
}

#[test]
fn sqlite_integer_primary_key_and_real_affinity_are_semantic_values() {
    let mut database = SqliteDatabase::from_schema("CREATE TABLE \"Entity\" (\"id\" INTEGER NOT NULL, amount REAL, PRIMARY KEY (\"id\"))").unwrap();
    database.tables[0].rows.push(SqliteRow { rowid: -1, values: vec![SqliteValue::Integer(-1), SqliteValue::Integer(3)] });
    let decoded = decode(&encode(&database)).unwrap(); assert_eq!(decoded.tables[0].rows[0].integer(0).unwrap(), -1); assert_eq!(decoded.tables[0].rows[0].real(1).unwrap(), 3.0);
    database.tables[0].rows[0].values[0] = SqliteValue::Integer(0); assert!(export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).is_err());
}

#[test]
fn implicit_indexes_unsupported_objects_and_invalid_schemas_are_rejected() {
    for schema in ["CREATE TABLE entity (name TEXT PRIMARY KEY)", "CREATE TABLE entity (id INTEGER PRIMARY KEY DESC)", "CREATE TABLE entity (id INTEGER PRIMARY KEY AUTOINCREMENT)", "CREATE TABLE entity (id INTEGER PRIMARY KEY, name TEXT UNIQUE)", "CREATE TABLE entity (id INTEGER, name TEXT, PRIMARY KEY(id,name))", "CREATE TABLE entity (id INTEGER PRIMARY KEY) WITHOUT ROWID", "CREATE VIEW entity AS SELECT 1", "CREATE TABLE entity (id INTEGER); CREATE INDEX entity_index ON entity(id)", "CREATE TABLE entity (id INTEGER, id TEXT)", "CREATE TABLE entity ()"] { assert!(SqliteDatabase::from_schema(schema).is_err(), "{schema}"); }
    assert!(SqliteDatabase::from_schema("-- domain\nCREATE TABLE [entity] ([id] INTEGER PRIMARY KEY, name TEXT DEFAULT 'UNIQUE; DESC'); /* foreign keys */").is_ok());
}

#[test]
fn allocation_limits_and_page_boundary_cancellation_are_enforced() {
    let database = corpus(); let source = encode(&database); let limits = SqliteDatabaseLimits { max_file_bytes: 4096, ..SqliteDatabaseLimits::default() };
    assert!(export_sqlite_database(&database, limits, &mut |_| true).is_err()); assert!(import_sqlite_database(&source, limits, &mut |_| true).is_err());
    for limits in [SqliteDatabaseLimits { max_rows: 1, ..SqliteDatabaseLimits::default() }, SqliteDatabaseLimits { max_tables: 1, ..SqliteDatabaseLimits::default() }, SqliteDatabaseLimits { max_columns: 1, ..SqliteDatabaseLimits::default() }, SqliteDatabaseLimits { max_value_bytes: 1, ..SqliteDatabaseLimits::default() }] { assert!(export_sqlite_database(&database, limits, &mut |_| true).is_err()); assert!(import_sqlite_database(&source, limits, &mut |_| true).is_err()); }
    assert!(matches!(export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |progress| progress.completed < 2), Err(ValueError { kind: ValueRefusalKind::Canceled, .. })));
    assert!(matches!(import_sqlite_database(&source, SqliteDatabaseLimits::default(), &mut |progress| progress.completed < 2), Err(ValueError { kind: ValueRefusalKind::Canceled, .. })));
}

#[test]
fn malformed_headers_fragment_counts_repeated_cells_and_overflow_are_rejected() {
    let source = encode(&corpus());
    for (offset, byte) in [(0, 0), (60, 0xff), (68, 0), (18, 2), (4096, 0), (4096 + 7, 1)] { let mut bytes = source.clone(); bytes[offset] = byte; assert!(decode(&bytes).is_err(), "offset {offset}"); }
    let mut stale = source.clone(); put_u32(&mut stale, 28, 999); assert!(decode(&stale).is_err()); put_u32(&mut stale, 92, 0); assert!(decode(&stale).is_ok());
    for length in [0, 15, 99, source.len() - 1, source.len() - 4096] { assert!(decode(&source[..length]).is_err()); }
    let mut large = SqliteDatabase::from_schema("CREATE TABLE entity (bytes BLOB)").unwrap(); large.tables[0].rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Blob(vec![1; 20_000])] });
    let mut overflow = encode(&large); let cell = u16_at(&overflow[4096..], 8).unwrap(); let mut cursor = 4096 + cell; let length = read_varint(&overflow, &mut cursor).unwrap() as usize; read_varint(&overflow, &mut cursor).unwrap(); let first = u32_at(&overflow, cursor + local_payload(length, 4096)).unwrap();
    put_u32(&mut overflow, (first as usize - 1) * 4096, first); assert!(decode(&overflow).is_err());
}

#[test]
fn varints_preserve_full_width_signed_rowid_bits() {
    for number in [0, 127, 128, 16_383, 16_384, 0x00ff_ffff_ffff_ffff, 0x0100_0000_0000_0000, i64::MIN as u64, u64::MAX] { let bytes = varint(number); let mut cursor = 0; assert_eq!(read_varint(&bytes, &mut cursor).unwrap(), number); assert_eq!(cursor, bytes.len()); }
}

fn sqlite_oracle(bytes: &[u8], extra: &str) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let script = format!("import {{ Database }} from 'bun:sqlite'; const db = Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())); if (db.query('PRAGMA integrity_check').get().integrity_check !== 'ok') throw new Error('SQLite integrity failed'); {extra} db.close();");
    let mut child = Command::new("bun").args(["-e", &script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let result = child.wait_with_output().unwrap(); assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
}

#[test]
fn independent_sqlite_checks_real_entities_foreign_keys_and_large_schema_roots() {
    sqlite_oracle(&encode(&corpus()), "if (db.query('PRAGMA foreign_key_check').all().length) throw new Error('foreign keys failed'); if (db.query('SELECT b.name, r.name AS room FROM building b JOIN room r ON r.building_id = b.id ORDER BY r.ordinal').all().length !== 2) throw new Error('join failed');");
    let sql = format!("CREATE TABLE entity (id INTEGER, /* {} */ value TEXT)", "x".repeat(3850));
    let database = SqliteDatabase::from_schema(&sql).unwrap(); let bytes = encode(&database);
    assert_eq!(decode(&bytes).unwrap(), database);
    sqlite_oracle(&bytes, "db.query('SELECT * FROM entity').all();");
    let mut many = SqliteDatabase::from_schema("CREATE TABLE entity (id INTEGER PRIMARY KEY, name TEXT)").unwrap();
    for rowid in -500..500 { many.tables[0].rows.push(SqliteRow { rowid, values: vec![SqliteValue::Integer(rowid), SqliteValue::Text("m".repeat(1000))] }); }
    sqlite_oracle(&encode(&many), "if (db.query('SELECT count(*) AS count FROM entity').get().count !== 1000) throw new Error('row count failed');");
}

#[test]
fn schema_literals_nested_checks_and_provider_ordering_do_not_change_column_rules() {
    let mut database = SqliteDatabase::from_schema("CREATE TABLE entity (ordinal INTEGER, value TEXT DEFAULT ';' CHECK (ordinal IS NOT NULL))").unwrap();
    database.tables[0].rows = vec![SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Null] }, SqliteRow { rowid: 2, values: vec![SqliteValue::Integer(0), SqliteValue::Text("a".into())] }];
    assert_eq!(database.tables[0].ordered_rows(0).unwrap()[0].rowid, 2);
    assert_eq!(decode(&encode(&database)).unwrap(), database);
    database.tables[0].rows[0].values[0] = SqliteValue::Integer(2); assert!(database.tables[0].ordered_rows(0).is_err());
    assert!(database.tables[0].single_row().is_err());
    let mut callback = |progress: SqliteSnapshotProgress| progress.phase != SqliteSnapshotPhase::ProjectSnapshot;
    let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits { max_rows: 1, max_value_bytes: 2, ..SqliteDatabaseLimits::default() });
    assert!(control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1).is_err()); assert!(control.check_rows(2).is_err()); assert!(control.check_value_bytes(3).is_err()); assert_eq!(control.limits().max_rows, 1);
}

#[test]
fn independent_sqlite_freelists_are_validated_before_table_traversal() {
    use std::process::{Command, Stdio};
    let script = "import { Database } from 'bun:sqlite'; const db = new Database(':memory:'); db.exec('PRAGMA application_id=1397576526; PRAGMA user_version=1; CREATE TABLE unused(data BLOB); INSERT INTO unused VALUES(zeroblob(16384)); CREATE TABLE entity(id INTEGER PRIMARY KEY); DROP TABLE unused;'); await Bun.write(Bun.stdout, db.serialize()); db.close();";
    let child = Command::new("bun").args(["-e", script]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let mut bytes = output.stdout; assert!(decode(&bytes).is_ok()); let trunk = u32_at(&bytes, 32).unwrap(); assert_ne!(trunk, 0);
    let raw = u16_at(&bytes, 16).unwrap(); let page_size = if raw == 1 { 65536 } else { raw }; put_u32(&mut bytes, (trunk as usize - 1) * page_size, trunk);
    assert!(decode(&bytes).is_err());
}

#[test]
fn aggregate_value_budget_rejects_overflow_before_reading_or_allocating_it() {
    let mut database = SqliteDatabase::from_schema("CREATE TABLE entity (id INTEGER PRIMARY KEY, value BLOB)").unwrap();
    database.tables[0].rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Blob(vec![1; 1024 * 1024])] });
    let bytes = encode(&database); let mut visited = 0;
    let result = import_sqlite_database(&bytes, SqliteDatabaseLimits { max_value_bytes: 32, ..SqliteDatabaseLimits::default() }, &mut |progress| { visited = visited.max(progress.completed); true });
    assert!(matches!(result, Err(ValueError { kind: ValueRefusalKind::OwnershipLimit, .. }))); assert!(visited <= 2, "overflow pages were read before enforcing the value budget");
    let frontier=std::cell::Cell::new((0,0));let mut callback = |progress:SqliteSnapshotProgress| {frontier.set((progress.completed,progress.total));true}; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits { max_value_bytes: 32, ..SqliteDatabaseLimits::default() });
    assert!(control.check_database(&database, SqliteSnapshotPhase::ReconstructSnapshot).is_err());
    assert!(matches!(import_sqlite_database_controlled(&bytes,&mut control),Err(ValueError { kind: ValueRefusalKind::OwnershipLimit, .. })));assert!(control.allocation_remaining_bytes()<SqliteDatabaseLimits::default().max_allocation_bytes);control.checkpoint(SqliteSnapshotPhase::ReadPages,7,11).unwrap();assert_eq!(frontier.get(),(7,11));
}

#[test]
fn artifact_schema_validation_preserves_quoted_constraint_meanings() {
    let expected = "CREATE TABLE entity (id INTEGER PRIMARY KEY, value TEXT NOT NULL CHECK(value IS NOT NULL))";
    let quoted = SqliteDatabase::from_schema("create table \"entity\" (\"id\" integer primary key, \"value\" text not null check(\"value\" is not null))").unwrap();
    assert!(validate_sqlite_database_schema(&quoted, expected, SqliteDatabaseLimits::default()).is_ok());
    let weakened = SqliteDatabase::from_schema("CREATE TABLE entity (id INTEGER PRIMARY KEY, value TEXT \"NOT\" \"NULL\" CHECK(value IS NOT NULL))").unwrap();
    assert!(validate_sqlite_database_schema(&weakened, expected, SqliteDatabaseLimits::default()).is_err());
    let mut wrong = quoted.clone(); wrong.tables[0].name = "other".into(); assert!(validate_sqlite_database_schema(&wrong, expected, SqliteDatabaseLimits::default()).is_err());
    let mut wrong = quoted.clone(); wrong.tables.push(wrong.tables[0].clone()); assert!(validate_sqlite_database_schema(&wrong, expected, SqliteDatabaseLimits::default()).is_err());
    sqlite_oracle(&export_sqlite_database(&weakened, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(), "if (db.query('PRAGMA table_info(entity)').all()[1].notnull !== 0) throw new Error('quoted NOT NULL unexpectedly enforced');");
}

#[test]
fn schema_budget_rejects_large_schema_overflow_before_reading_it() {
    let database = SqliteDatabase::from_schema(&format!("CREATE TABLE entity (id INTEGER PRIMARY KEY /* {} */)", "x".repeat(1024 * 1024))).unwrap();
    let bytes = encode(&database); let mut visited = 0;
    let result = import_sqlite_database(&bytes, SqliteDatabaseLimits { max_schema_bytes: 8, ..SqliteDatabaseLimits::default() }, &mut |progress| { visited = visited.max(progress.completed); true });
    assert!(matches!(result, Err(ValueError { kind: ValueRefusalKind::OwnershipLimit, .. }))); assert_eq!(visited, 1, "schema overflow was traversed before applying its budget");
}


#[test]
fn keyed_projection_cancels_during_heap_ordering_and_rejects_duplicate_physical_identities(){use std::cell::Cell;let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏗️projection/🔣️.json")).unwrap();let count=f["keyedOrder"]["rows"].as_u64().unwrap()as usize;let cancel=count+f["keyedOrder"]["cancelAt"].as_u64().unwrap()as usize;let armed=Cell::new(false);let inside=Cell::new(false);let mut callback=|p:SqliteSnapshotProgress|{if armed.get()&&p.completed==cancel{inside.set(true);false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut p=artifact::Projection::new(f["schemaSql"].as_str().unwrap(),&mut control).unwrap();for key in(1..=count).rev(){p.insert_key(f["table"].as_str().unwrap(),key as i64,&[artifact::Cell::Text("")]).unwrap();}armed.set(true);assert!(p.finish().is_err());assert!(inside.get());let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut p=artifact::Projection::new(f["schemaSql"].as_str().unwrap(),&mut control).unwrap();for key in(1..=count).rev(){p.insert_key(f["table"].as_str().unwrap(),key as i64,&[artifact::Cell::Text("")]).unwrap();}let database=p.finish().unwrap();assert!(database.tables[0].rows.iter().enumerate().all(|(i,row)|row.rowid==i as i64+1));let bytes=encode(&database);assert_eq!(import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap().tables[0].rows.len(),count);let mut p=artifact::Projection::new(f["schemaSql"].as_str().unwrap(),&mut control).unwrap();for label in["a","b"]{p.insert_key(f["table"].as_str().unwrap(),1,&[artifact::Cell::Text(label)]).unwrap();}assert!(p.finish().unwrap_err().message.contains("identities must be unique"));}

#[test]
fn explicit_keyed_projection_completes_in_independent_sqlite_rowid_order_and_cancels_inside_sorting(){use std::io::Write;use std::process::{Command,Stdio};use std::cell::Cell;let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏗️projection/🔣️.json")).unwrap();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut p=artifact::Projection::new(f["schemaSql"].as_str().unwrap(),&mut control).unwrap();for key in f["keyedOrder"]["input"].as_array().unwrap(){let label=key.as_str().unwrap();p.insert_key(f["table"].as_str().unwrap(),label.parse().unwrap(),&[artifact::Cell::Text(label)]).unwrap();}let database=p.finish().unwrap();let expected:Vec<String>=f["keyedOrder"]["expected"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().into()).collect();assert_eq!(database.tables[0].rows.iter().map(|row|row.rowid.to_string()).collect::<Vec<_>>(),expected);let bytes=encode(&database);let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));console.log(JSON.stringify(d.query('SELECT label FROM explicit_entity ORDER BY id').all().map(row=>row.label)));d.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));assert_eq!(serde_json::from_slice::<Vec<String>>(&result.stdout).unwrap(),expected);let armed=Cell::new(false);let observed=Cell::new(false);let count=f["keyedOrder"]["rows"].as_u64().unwrap()as usize;let cancel=f["keyedOrder"]["cancelAt"].as_u64().unwrap()as usize;let mut callback=|p:SqliteSnapshotProgress|{if armed.get()&&p.completed==cancel{observed.set(true);false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut p=artifact::Projection::new(f["schemaSql"].as_str().unwrap(),&mut control).unwrap();for key in(1..=count).rev(){p.insert_key(f["table"].as_str().unwrap(),key as i64,&[artifact::Cell::Text("")]).unwrap();}armed.set(true);assert!(p.finish().is_err());assert!(observed.get());}


#[test]
fn sqlite_snapshot_allocation_cross_stage_frontier_is_independent_of_semantic_payload(){
    let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️allocation/🔣️.json")).unwrap();let maximum=f["maximumBytes"].as_u64().unwrap()as usize;let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:maximum,max_value_bytes:f["semanticBytes"].as_u64().unwrap()as usize,..Default::default()});assert_eq!(SqliteDatabaseLimits::default().max_allocation_bytes,f["defaultAllocationBytes"].as_u64().unwrap()as usize);let phases=[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::ReconstructSnapshot];let mut constructed=0;for(index,value)in f["admissions"].as_array().unwrap().iter().enumerate(){control.checkpoint(phases[index],0,0).unwrap();let bytes=value.as_u64().unwrap()as usize;control.admit_allocation_bytes(bytes).unwrap();let mut owned=Vec::<u8>::new();owned.try_reserve_exact(bytes).unwrap();owned.resize(bytes,0);constructed+=1;drop(owned);}assert_eq!(control.allocation_remaining_bytes(),0);assert!(control.admit_allocation_bytes(f["refusedBytes"].as_u64().unwrap()as usize).is_err());assert_eq!(control.allocation_remaining_bytes(),0);assert_eq!(constructed,f["admissions"].as_array().unwrap().len());control.check_value_bytes(0).unwrap();
}

#[test]
fn sqlite_snapshot_allocation_failed_stage_cannot_refund_owned_backing(){
    let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️allocation/🔣️.json")).unwrap();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:f["maximumBytes"].as_u64().unwrap()as usize,..Default::default()});let result=(||->std::result::Result<(),ValueError>{let count=f["failedStageBytes"].as_u64().unwrap()as usize;control.admit_allocation_bytes(count)?;let mut owned=Vec::<u8>::new();owned.try_reserve_exact(count).unwrap();owned.resize(count,0);Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned domain construction refused"))})();assert_eq!(result.unwrap_err().message,"owned domain construction refused");assert_eq!(control.allocation_remaining_bytes(),f["afterFailedStageRemaining"].as_u64().unwrap()as usize);
}


#[test]
fn sqlite_snapshot_allocation_real_native_bridge_settles_interior_failure_before_next_copy(){
    use semio_framework_value::NativeDecodeControl;use std::cell::Cell;
    let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️allocation/🔣️.json")).unwrap();let b=&f["nativeBridge"];let n=|name:&str|b[name].as_u64().unwrap()as usize;let cancel=Cell::new(false);let interior=Cell::new(false);let mut callback=|p:SqliteSnapshotProgress|{let stop=cancel.get()&&p.phase==SqliteSnapshotPhase::DecodeNative&&p.completed>=n("cancelAt")&&p.completed<p.total;interior.set(interior.get()||stop);!stop};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:n("maximumBytes"),max_value_bytes:0,..Default::default()});let source=vec![7u8;n("cancelBytes")];
    let success=control.allocation_stage(SqliteSnapshotPhase::ProjectSnapshot,|remaining,progress,allocation|{let mut callback=|p:semio_framework_value::native_decoding::NativeDecodeProgress|progress(p.completed,p.total);let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);let result=native.copy_bytes(&source[..n("successBytes")]);(result,native.owned_bytes())}).unwrap().unwrap();assert_eq!(success.len(),n("successBytes"));drop(success);cancel.set(true);
    let canceled=control.allocation_stage(SqliteSnapshotPhase::DecodeNative,|remaining,progress,allocation|{let mut callback=|p:semio_framework_value::native_decoding::NativeDecodeProgress|progress(p.completed,p.total);let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);let result=native.copy_bytes(&source);(result,native.owned_bytes())}).unwrap();assert!(canceled.is_err());assert!(interior.get());assert_eq!(control.allocation_remaining_bytes(),n("remainingAfterCancellation"));cancel.set(false);
    let refused=control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,progress,allocation|{let mut callback=|p:semio_framework_value::native_decoding::NativeDecodeProgress|progress(p.completed,p.total);let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);let result=native.copy_bytes(&source[..n("refusedBytes")]);assert_eq!(native.owned_bytes(),0,"next producer must refuse before owning its output buffer");(result,native.owned_bytes())}).unwrap();assert!(refused.is_err());assert_eq!(control.allocation_remaining_bytes(),n("remainingAfterCancellation"));control.check_value_bytes(0).unwrap();
}

#[test]
fn neutral_long_authored_sql_literal_and_formatting_preserve_independent_defaults(){
 use std::io::Write;use std::process::{Command,Stdio};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔎️frontiers/📏️schema/🔣️.json")).unwrap();
 let length=fixture["literalLength"].as_u64().unwrap()as usize;let prefix=fixture["literalUnit"].as_str().unwrap().repeat(length);let tail=fixture["literalTail"].as_str().unwrap();let sql=format!("CREATE TABLE entity (id INTEGER PRIMARY KEY,payload TEXT DEFAULT '{prefix}{tail}')");
 for item in fixture["cases"].as_array().unwrap(){let name=item["name"].as_str().unwrap();let actual_tail=if name=="lateLiteralMismatch"{fixture["mismatchTail"].as_str().unwrap()}else{tail};let mut received=format!("CREATE TABLE entity (id INTEGER PRIMARY KEY,payload TEXT DEFAULT '{prefix}{actual_tail}')");if name=="formatted"{received.insert_str(received.len()-1,fixture["formatSpacing"].as_str().unwrap());}
  let database=SqliteDatabase::from_schema(&received).unwrap();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let result=validate_sqlite_database_schema_controlled(&database,&sql,SqliteSnapshotPhase::ReconstructSnapshot,&mut control);assert_eq!(result.is_ok(),item["accept"].as_bool().unwrap(),"{name}");if let Err(error)=result{assert_eq!(error.kind,ValueRefusalKind::InvalidValue);}
  let script="import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text()),d=new Database(':memory:');try{d.exec(input.sql);d.exec('INSERT INTO entity DEFAULT VALUES');const value=d.query('SELECT payload FROM entity').get().payload;console.log(JSON.stringify({length:value.length,tail:value.slice(-1)}));}finally{d.close();}";
  let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"sql":received}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let result:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(result["length"].as_u64().unwrap()as usize,length+1);assert_eq!(result["tail"].as_str().unwrap(),actual_tail);
 }
}

#[test]
fn original_sqlite_physical_allocation_port_precedes_stage_reservations_and_matches_sqlite(){
 use std::{sync::atomic::{AtomicUsize,Ordering},io::Write,process::{Command,Stdio}};
 use semio_framework_value::native_encoding::NativeEncodeAllocation;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛂️allocation/🔣️.json")).unwrap();let mut database=SqliteDatabase::from_schema(law["schema"].as_str().unwrap()).unwrap();database.tables[0].rows.push(SqliteRow{rowid:law["rowid"].as_i64().unwrap(),values:vec![SqliteValue::Integer(law["rowid"].as_i64().unwrap()),SqliteValue::Text(law["value"].as_str().unwrap().into())]});let source=database.tables[0].rows[0].text(1).unwrap().as_ptr();let maximum=law["maximumBytes"].as_u64().unwrap()as usize;let limits=SqliteDatabaseLimits{max_allocation_bytes:maximum,..Default::default()};let owned=AtomicUsize::new(0);let calls=AtomicUsize::new(0);let mut port=|request:NativeEncodeAllocation|{assert_eq!(request.owned_bytes,owned.load(Ordering::Relaxed));assert_eq!(request.next_owned_bytes,request.owned_bytes+request.bytes);assert_eq!(request.maximum_bytes,maximum);owned.store(request.next_owned_bytes,Ordering::Relaxed);calls.fetch_add(1,Ordering::Relaxed);Ok(())};let mut progress=|_|true;let mut control=SqliteSnapshotControl::new_forwarded(&mut progress,limits,&mut port);let file=export_sqlite_database_controlled(&database,&mut control).unwrap();assert_eq!(maximum-control.allocation_remaining_bytes(),owned.load(Ordering::Relaxed));assert!(calls.load(Ordering::Relaxed)>1);assert_eq!(source,database.tables[0].rows[0].text(1).unwrap().as_ptr());
 let mut child=Command::new("bun").args(["-e","import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT id,value FROM entry').values()));db.close();"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),law["expectedRows"]);
 for kind in [ValueRefusalKind::OwnershipLimit,ValueRefusalKind::Canceled]{let refused=AtomicUsize::new(0);let mut port=|_:NativeEncodeAllocation|{refused.fetch_add(1,Ordering::Relaxed);Err(ValueError::literal(kind,"original SQL receiving authority refused"))};let mut progress=|_|true;let mut control=SqliteSnapshotControl::new_forwarded(&mut progress,limits,&mut port);assert_eq!(export_sqlite_database_controlled(&database,&mut control).unwrap_err().kind,kind);assert_eq!(control.allocation_remaining_bytes(),maximum);assert_eq!(refused.load(Ordering::Relaxed),1);assert_eq!(source,database.tables[0].rows[0].text(1).unwrap().as_ptr());}
 println!("[DEBUG] Actual SQL physical allocation receiving: everyStageReservationForwarded=true exactOriginalCumulativeLedger=true refusalCases=2 zeroOwnedOnRefusal=true sourcePointerPreserved=true independentBunSqlite=true");
}
