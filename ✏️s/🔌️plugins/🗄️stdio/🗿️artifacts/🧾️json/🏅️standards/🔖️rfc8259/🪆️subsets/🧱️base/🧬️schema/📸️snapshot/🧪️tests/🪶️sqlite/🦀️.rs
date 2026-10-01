use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> (serde_json::Value, JsonSnapshot) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let snapshot = JsonSnapshot { schema: fixture["schema"].as_str().unwrap().into(), value: parse_json_text(fixture["jsonText"].as_str().unwrap()).unwrap() };
    (fixture, snapshot)
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_json_exact_declared_i_json_owned_io() {
    use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("JSON SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let dialect=ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"i-json".into()};
    let snapshot=JsonSnapshot{schema:"owned 世界".into(),value:parse_json_text("{\"member\":[true,1,\"value\"]}").unwrap()};
    let mut phases=Vec::new();let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Text,SqliteDatabaseLimits::default(),&mut |p|{phases.push(p.phase);true}).await.unwrap().value;
    assert_eq!(io_import_sqlite_snapshot::<JsonSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);
    assert!(!phases.iter().any(|p|matches!(p,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
    let invalid=JsonSnapshot{value:parse_json_text("{\"a\":1,\"a\":2}").unwrap(),..snapshot};assert!(io_export_sqlite_snapshot(&dialect,&invalid,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.is_err());
}

#[test]
fn sqlite_snapshot_json_syntax_preserves_order_arbitrary_numbers_and_primitive_kinds() {
    let (fixture, snapshot) = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("json_value").unwrap().rows.len(), fixture["valueCount"].as_u64().unwrap() as usize);
    assert_eq!(database.table("json_object_member").unwrap().rows.len(), fixture["memberCount"].as_u64().unwrap() as usize);
    assert_eq!(database.table("json_array_element").unwrap().rows.len(), fixture["elementCount"].as_u64().unwrap() as usize);
    let keys: Vec<_> = database.table("json_object_member").unwrap().ordered_rows(2).unwrap().into_iter().map(|row| row.text(3).unwrap()).collect();
    assert_eq!(keys, fixture["memberKeys"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>());
    let numbers: Vec<_> = database.table("json_value").unwrap().rows.iter().filter(|row| row.text(1).unwrap() == "number").map(|row| row.text(3).unwrap()).collect();
    assert_eq!(numbers, fixture["numberLexemes"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>());
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = JsonSnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    assert_eq!(<JsonSnapshot as store::ArtifactPack>::encode_pack(&restored), <JsonSnapshot as store::ArtifactPack>::encode_pack(&snapshot));
    assert_eq!(<JsonSnapshot as store::ArtifactDsl>::print_dsl(&restored), <JsonSnapshot as store::ArtifactDsl>::print_dsl(&snapshot));
    let oracle_before: serde_json::Value = serde_json::from_str(&write_json_text(&snapshot.value)).unwrap();
    let oracle_after: serde_json::Value = serde_json::from_str(&write_json_text(&restored.value)).unwrap(); assert_eq!(oracle_before, oracle_after);
}

#[test]
fn sqlite_snapshot_json_rejects_dangling_cycles_ownership_ordinals_and_primitive_shape() {
    let (_, snapshot) = fixture(); let original = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    for alteration in 0..5 { let mut database = original.clone(); match alteration {
        0 => database.table_mut("json_document").unwrap().rows[0].values[2] = SqliteValue::Integer(999),
        1 => database.table_mut("json_object_member").unwrap().rows[0].values[4] = SqliteValue::Integer(1),
        2 => database.table_mut("json_object_member").unwrap().rows[1].values[4] = SqliteValue::Integer(2),
        3 => database.table_mut("json_array_element").unwrap().rows[0].values[2] = SqliteValue::Integer(99),
        _ => database.table_mut("json_value").unwrap().rows[0].values[3] = SqliteValue::Text("serialized object".into()),
    } assert!(JsonSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err(), "alteration {alteration}"); }
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
    let limits = SqliteDatabaseLimits { max_rows: 2, ..SqliteDatabaseLimits::default() }; assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    let limits = SqliteDatabaseLimits { max_value_bytes: 1, ..SqliteDatabaseLimits::default() }; assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(JsonSnapshot::from_sqlite_database(&original, &mut SqliteSnapshotControl::new(&mut |p| p.phase != SqliteSnapshotPhase::ReconstructSnapshot, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_json_independent_queries_and_edits_reconstruct_the_native_model() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let (fixture, snapshot) = fixture(); let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import { Database } from 'bun:sqlite'; const db = Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())); if (db.query('PRAGMA integrity_check').get().integrity_check !== 'ok' || db.query('PRAGMA foreign_key_check').all().length) throw new Error('integrity'); const members=db.query('SELECT m.key,v.kind,v.number_lexeme FROM json_object_member m JOIN json_value v ON v.id=m.value_id ORDER BY m.ordinal').all(); if (members.map(m=>m.key).join(',') !== 'z,a,z' || members[0].number_lexeme !== '123456789012345678901234567890.123456789e+42') throw new Error('semantic query'); db.query('UPDATE json_value SET string_value=? WHERE kind=?').run('Aus SQLite geändert 🌠','string'); await Bun.write(Bun.stdout,db.serialize()); db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let edited = JsonSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let mut expected = snapshot; if let JsonValue::Object { members } = &mut expected.value { if let JsonValue::Array { items } = &mut members[1].value { items[3] = JsonValue::String { value: fixture["editedString"].as_str().unwrap().into() }; } }
    assert_eq!(edited, expected);
    let sample = demo_json_snapshot(); let projected = sample.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(JsonSnapshot::from_sqlite_database(&projected, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), sample);
}

#[test]
fn sqlite_snapshot_json_owned_i_json_guard_matches_native_rules_and_cancels(){let(fixture,_)=fixture();for case in fixture["iJsonCases"].as_array().unwrap(){let snapshot=JsonSnapshot{schema:"stdio.json".into(),value:parse_json_text(case["text"].as_str().unwrap()).unwrap()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let database=snapshot.to_sqlite_database(&mut control).unwrap();let dialect=semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"i-json".into()};let actual=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut control).unwrap();let expected=crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance(&snapshot);assert_eq!(format!("{:?}",actual.diagnostics),format!("{:?}",expected));assert_eq!(actual.diagnostics.iter().any(|d|matches!(d.severity,dsl::Severity::Error|dsl::Severity::Fatal)),case["hard"].as_bool().unwrap());}let snapshot=JsonSnapshot{schema:"stdio.json".into(),value:JsonValue::String{value:"x".repeat(100000)}};let mut calls=0;let mut callback=|_|{calls+=1;calls<3};assert!(crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance_controlled(&snapshot,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());}

#[test]
fn sqlite_snapshot_json_native_encoding_preflight_bounds_escaping_indentation_and_cancellation() {
    let (_, snapshot) = fixture();
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let codec = <JsonSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
        let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.json".into(), standard: "rfc8259".into(), subset: "*".into() };
        let restored = (codec.import)("JSON preflight", &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let expected = match encoding { SnapshotEncoding::Text => semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), SnapshotEncoding::Binary => semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)) };
        assert_eq!(restored, expected);
    }
    let large = JsonSnapshot { schema: "stdio.json".into(), value: JsonValue::String { value: "\u{0001}".repeat(100000) } };
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 4096, ..SqliteDatabaseLimits::default() })).is_err());
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |p| p.phase != SqliteSnapshotPhase::EncodeNative, SqliteDatabaseLimits::default())).is_err());
    let mut nested = JsonValue::Null; for _ in 0..256 { nested = JsonValue::Array { items: vec![nested] }; }
    let deep = JsonSnapshot { schema: "stdio.json".into(), value: nested };
    let limits = SqliteDatabaseLimits { max_value_bytes: 50000, ..SqliteDatabaseLimits::default() };
    deep.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert!(deep.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}
