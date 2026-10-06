use crate::standards::v_utf_8::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn native_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🛂️native.json")).unwrap()
}

fn native_case(value: &serde_json::Value) -> TxtSnapshot {
    TxtSnapshot { schema: crate::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: value["lines"].as_array().unwrap().iter().map(|line| line.as_str().unwrap().into()).collect(), trailing_newline: value["trailingNewline"].as_bool().unwrap(), line_ending: if value["lineEnding"] == "crLf" { LineEnding::CrLf } else { LineEnding::Lf } }
}

fn native_payload(snapshot: &TxtSnapshot, encoding: store::sqlite_snapshot::SnapshotEncoding) -> store::io_schema::IoPayload {
    match encoding { store::sqlite_snapshot::SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(<TxtSnapshot as store::ArtifactPack>::encode_pack(snapshot)), store::sqlite_snapshot::SnapshotEncoding::Text => store::io_schema::IoPayload::Text(<TxtSnapshot as store::ArtifactDsl>::print_dsl(snapshot)) }
}

#[test]
fn sqlite_snapshot_txt_controlled_native_matches_the_real_external_carrier() {
    use store::sqlite_snapshot::SnapshotEncoding;
    for case in native_fixture()["nativeCases"].as_array().unwrap() {
        let snapshot = native_case(case);
        assert_eq!(snapshot.to_body(), case["body"].as_str().unwrap());
        assert_eq!(TxtSnapshot::from_body(case["body"].as_str().unwrap()), snapshot);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let ordinary = native_payload(&snapshot, encoding);
            assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), ordinary);
            assert_eq!(TxtSnapshot::decode_sqlite_snapshot_native(&ordinary, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
        }
    }
    println!("[DEBUG] TXT controlled native codecs retain the authored raw text and pack carriers");
}

#[test]
fn sqlite_snapshot_txt_native_controls_refuse_inside_copy_and_at_exact_limits() {
    use store::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    let fixture = native_fixture();
    let text = fixture["controls"]["longText"].as_str().unwrap().repeat(fixture["controls"]["longRepeat"].as_u64().unwrap() as usize);
    let snapshot = TxtSnapshot { lines: vec![text, String::new(), "끝\0".into()], trailing_newline: true, line_ending: LineEnding::CrLf, ..TxtSnapshot::default() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = native_payload(&snapshot, encoding);
        let size = match &payload { store::io_schema::IoPayload::Binary(bytes) => bytes.len(), store::io_schema::IoPayload::Text(text) => text.len() };
        let exact = SqliteDatabaseLimits { max_file_bytes: size, max_rows: fixture["controls"]["domainRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
        assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap(), payload);
        assert_eq!(TxtSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, exact)).unwrap(), snapshot);
        for limits in [SqliteDatabaseLimits { max_file_bytes: size - 1, ..exact }, SqliteDatabaseLimits { max_rows: exact.max_rows - 1, ..exact }, SqliteDatabaseLimits { max_value_bytes: fixture["controls"]["smallValueBytes"].as_u64().unwrap() as usize, ..exact }] {
            assert!(snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
            assert!(TxtSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        }
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
            let mut interior = false;
            let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { let cancel = event.phase == phase && event.completed >= fixture["controls"]["cancelAtBytes"].as_u64().unwrap() as usize && event.total > event.completed; interior |= cancel; !cancel };
            let result = if phase == SqliteSnapshotPhase::DecodeNative { TxtSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut callback, exact)).map(|_| ()) } else { snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut callback, exact)).map(|_| ()) };
            let error = result.unwrap_err();
            assert!(interior && error.kind == semio_framework_value::ValueRefusalKind::Canceled, "{phase:?}: {error}");
        }
    }
    println!("[DEBUG] TXT native transfer admits exact file and domain row bounds and cancels during large copies");
}

#[test]
fn sqlite_snapshot_txt_projection_and_reconstruction_cancel_inside_long_literal_fields() {
    use store::sqlite_snapshot::SqliteSnapshotPhase;
    let fixture = native_fixture();
    let text = fixture["controls"]["longText"].as_str().unwrap().repeat(fixture["controls"]["longRepeat"].as_u64().unwrap() as usize);
    for schema_field in [false, true] {
        let snapshot = if schema_field { TxtSnapshot { schema: text.clone(), ..TxtSnapshot::default() } } else { TxtSnapshot { lines: vec![text.clone()], ..TxtSnapshot::default() } };
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
            let mut interior = false;
            let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { let cancel = event.phase == phase && event.completed >= fixture["controls"]["cancelAtBytes"].as_u64().unwrap() as usize && event.total > event.completed; interior |= cancel; !cancel };
            let result = if phase == SqliteSnapshotPhase::ProjectSnapshot { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()) } else { TxtSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ()) };
            let error = result.unwrap_err();
            assert!(interior && error.kind == semio_framework_value::ValueRefusalKind::Canceled, "{phase:?}: {error}");
        }
    }
    println!("[DEBUG] TXT SQL copies cancel inside both document schema and line content");
}

#[test]
fn sqlite_snapshot_txt_surrogate_ids_and_literal_metadata_are_not_native_headers() {
    let fixture = native_fixture();
    for schema in fixture["literalSchemas"].as_array().unwrap() {
        let snapshot = TxtSnapshot { schema: schema.as_str().unwrap().into(), lines: vec!["任意\0\nembedded".into(), String::new()], ..TxtSnapshot::default() };
        let mut database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let key = fixture["controls"]["rootIdentity"].as_i64().unwrap();
        database.table_mut("text_document").unwrap().rows[0].rowid = key;
        database.table_mut("text_document").unwrap().rows[0].values[0] = SqliteValue::Integer(key);
        for row in &mut database.table_mut("text_line").unwrap().rows { row.values[1] = SqliteValue::Integer(key); }
        assert_eq!(TxtSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
        let valid = store::io_schema::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
        assert!(snapshot.validate_sqlite_snapshot_subset(&valid, &database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_ok());
        for invalid in [store::io_schema::ArtifactDialect { artifact_kind: "s.stdio.csv".into(), ..valid.clone() }, store::io_schema::ArtifactDialect { standard: "other".into(), ..valid.clone() }, store::io_schema::ArtifactDialect { subset: "other".into(), ..valid.clone() }] {
            assert!(snapshot.validate_sqlite_snapshot_subset(&invalid, &database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
        }
        for corruption in 0..6 {
            let mut broken = database.clone();
            match corruption { 0 => broken.table_mut("text_document").unwrap().rows[0].rowid = -1, 1 => broken.table_mut("text_document").unwrap().rows[0].values[0] = SqliteValue::Integer(key + 1), 2 => broken.table_mut("text_document").unwrap().rows[0].values.push(SqliteValue::Null), 3 => broken.table_mut("text_line").unwrap().rows[0].values[1] = SqliteValue::Integer(key + 1), 4 => broken.table_mut("text_line").unwrap().rows[0].values[2] = SqliteValue::Integer(-1), _ => broken.table_mut("text_line").unwrap().rows[0].values.push(SqliteValue::Null) }
            assert!(TxtSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
        }
    }
    println!("[DEBUG] TXT semantic SQL restores arbitrary literal metadata and positive independent surrogate identities");
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_txt_actual_typed_io_preserves_fields_without_native_phases() {
    use store::{io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}, sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("TXT SQLite Test").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let fixture = native_fixture();
    let source = TxtSnapshot { schema: fixture["literalSchemas"][2].as_str().unwrap().into(), lines: vec!["任意\0\ninside one entity".into(), String::new(), "tail".into()], trailing_newline: true, line_ending: LineEnding::CrLf };
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
    let limits = SqliteDatabaseLimits { max_rows: fixture["controls"]["finalRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let mut phases = Vec::new();
        let bytes = io_export_sqlite_snapshot(&dialect, &source, encoding, limits, &mut |event| { phases.push(event.phase); true }).await.unwrap().value;
        assert!(!phases.iter().any(|phase| matches!(phase, SqliteSnapshotPhase::DecodeNative | SqliteSnapshotPhase::EncodeNative)));
        assert_eq!(import_sqlite_database(&bytes, limits, &mut |_| true).unwrap().tables.iter().map(|table| table.rows.len()).sum::<usize>(), limits.max_rows);
        assert_eq!(io_import_sqlite_snapshot::<TxtSnapshot>(&dialect, &bytes, limits, &mut |_| true).await.unwrap().value, source);
        assert!(io_export_sqlite_snapshot(&dialect, &source, encoding, SqliteDatabaseLimits { max_rows: limits.max_rows - 1, ..limits }, &mut |_| true).await.is_err());
    }
    println!("[DEBUG] TXT actual typed I/O preserves every persisted field independently of external carrier limitations");
}

#[test]
fn sqlite_snapshot_txt_actual_bare_factory_transfers_native_carriers() {
    use store::sqlite_snapshot::SnapshotEncoding;
    let codec = store::ArtifactCodec::bare::<TxtSnapshot, crate::standards::v_utf_8::subsets::any::schema::mutations::TxtMutation>(crate::STDIO_TXT_DOCUMENT_SCHEMA);
    let capability = codec.snapshot_sqlite.expect("TXT bare owner publishes its semantic SQLite capability");
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.stdio.txt".into(), standard: "utf-8".into(), subset: "*".into() };
    for case in native_fixture()["nativeCases"].as_array().unwrap() {
        let snapshot = native_case(case);
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = native_payload(&snapshot, encoding);
            let database = (capability.export)(crate::STDIO_TXT_DOCUMENT_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.table("text_line").unwrap().rows.len(), snapshot.lines.len());
            assert_eq!((capability.import)(crate::STDIO_TXT_DOCUMENT_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value, payload);
        }
    }
    println!("[DEBUG] TXT actual erased bare codec transfers queryable entities in both native encodings");
}

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
    assert!(error.kind == semio_framework_value::ValueRefusalKind::Canceled, "{error}");
}

#[test]
fn sqlite_snapshot_text_lines_preserve_order_and_line_endings() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = TxtSnapshot { schema: fixture["schema"].as_str().unwrap().into(), lines: fixture["lines"].as_array().unwrap().iter().map(|line| line.as_str().unwrap().into()).collect(), trailing_newline: true, line_ending: LineEnding::CrLf };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("text_line").unwrap().rows.len(), 3);
    assert_eq!(database.table("text_document").unwrap().rows[0].values[3], SqliteValue::Text("crlf".into()));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(TxtSnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, fixture);
    let mut broken = reopened;
    broken.table_mut("text_line").unwrap().rows[1].values[2] = SqliteValue::Integer(0);
    assert!(TxtSnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}
