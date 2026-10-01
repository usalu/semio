use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};

fn fixture() -> XmlSnapshot { let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap(); XmlSnapshot::import_utf8(fixture["xmlText"].as_str().unwrap().as_bytes()).unwrap() }
fn project(snapshot: &XmlSnapshot) -> SqliteDatabase { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }
fn restore(database: &SqliteDatabase) -> Result<XmlSnapshot, String> { XmlSnapshot::from_sqlite_database(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) }

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_xml_exact_declared_valid_owned_io(){use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("XML SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let dialect=ArtifactDialect{artifact_kind:"s.stdio.xml".into(),standard:"1.0".into(),subset:"valid".into()};let mut snapshot=fixture();snapshot.schema="owned 世界".into();let mut phases=Vec::new();let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Text,SqliteDatabaseLimits::default(),&mut |p|{phases.push(p.phase);true}).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<XmlSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);assert!(!phases.iter().any(|p|matches!(p,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));snapshot.doc.doctype.as_mut().unwrap().name="mismatch".into();assert!(io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.is_err());}

#[test]
fn sqlite_snapshot_xml_domain_relations_roundtrip() {
    let snapshot = fixture(); let database = project(&snapshot);
    assert_eq!(database.table("xml_document") .unwrap().single_row().unwrap().integer(2).unwrap(), 1);
    assert_eq!(database.table("xml_attribute").unwrap().rows.len(), 3);
    assert_eq!(database.table("xml_entity").unwrap().rows.len(), 2);
    assert_eq!(database.table("xml_declaration").unwrap().single_row().unwrap().text(5).unwrap(), "single");
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = restore(&database).unwrap(); assert_eq!(restored, snapshot);
    assert_eq!(<XmlSnapshot as store::ArtifactPack>::encode_pack(&restored), <XmlSnapshot as store::ArtifactPack>::encode_pack(&snapshot));
    assert_eq!(<XmlSnapshot as store::ArtifactDsl>::print_dsl(&restored), <XmlSnapshot as store::ArtifactDsl>::print_dsl(&snapshot));
    assert_eq!(<XmlSnapshot as store::ArtifactPack>::decode_pack(&<XmlSnapshot as store::ArtifactPack>::encode_pack(&restored)).unwrap(), snapshot);
    assert_eq!(<XmlSnapshot as store::ArtifactDsl>::parse_dsl(&<XmlSnapshot as store::ArtifactDsl>::print_dsl(&restored)).unwrap(), snapshot);
    assert_eq!(restore(&project(&XmlSnapshot::default())).unwrap(), XmlSnapshot::default());
}

#[test]
fn sqlite_snapshot_xml_preserves_optional_declarations_and_external_identifier_variants() {
    for external_id in [None, Some(XmlExternalId::System { system_id: "Grüße.dtd".into() }), Some(XmlExternalId::Public { public_id: "public identifier".into(), system_id: "system.dtd".into() })] {
        for standalone in [None, Some(false), Some(true)] { for quote in [XmlQuote::Double, XmlQuote::Single] {
            let mut snapshot = fixture(); snapshot.schema = "custom.xml".into(); snapshot.doc.doctype.as_mut().unwrap().external_id = external_id.clone(); snapshot.doc.declaration = Some(XmlDeclaration { version: "1.0".into(), encoding: None, standalone, quote });
            assert_eq!(restore(&project(&snapshot)).unwrap(), snapshot);
        } }
    }
    let mut snapshot = fixture(); snapshot.doc.declaration = None; snapshot.doc.doctype = None; snapshot.doc.root = None; assert_eq!(restore(&project(&snapshot)).unwrap(), snapshot);
}

#[test]
fn sqlite_snapshot_xml_controls_bound_projection_and_reconstruction() {
    let snapshot = fixture(); let database = project(&snapshot);
    let rows: usize = database.tables.iter().map(|table| table.rows.len()).sum(); let bytes: usize = database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).map(|value| match value { SqliteValue::Null => 0, SqliteValue::Integer(_) | SqliteValue::Real(_) => 8, SqliteValue::Text(value) => value.len(), SqliteValue::Blob(value) => value.len() }).sum();
    let limits = SqliteDatabaseLimits { max_rows: rows, max_value_bytes: bytes, ..SqliteDatabaseLimits::default() };
    assert_eq!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), database);
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, limits)).is_err());
    for limits in [SqliteDatabaseLimits { max_rows: rows - 1, ..limits }, SqliteDatabaseLimits { max_value_bytes: bytes - 1, ..limits }] {
        assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        assert!(XmlSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    }
    let mut large = XmlSnapshot::default(); large.doc.root = Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: (0..1024).map(|_| XmlNode::Text { text: "value".into() }).collect() });
    assert!(large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |progress| progress.completed == 0, SqliteDatabaseLimits::default())).is_err());
    let database = project(&large); let mut checkpoints = 0;
    assert!(XmlSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| { checkpoints += 1; checkpoints < 3 }, SqliteDatabaseLimits::default())).is_err()); assert_eq!(checkpoints, 3);
}

#[test]
fn sqlite_snapshot_xml_independent_sql_queries_and_edits_restore_typed_nodes() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let snapshot = fixture(); let bytes = export_sqlite_database(&project(&snapshot), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import { Database } from 'bun:sqlite'; const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())); if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw new Error('integrity'); if(db.query('SELECT name FROM xml_attribute WHERE element_node_id=1 ORDER BY ordinal').all().map(r=>r.name).join(',')!=='z,a')throw new Error('attribute order'); const d=db.query('SELECT external_kind,public_id,system_id,prolog_position FROM xml_doctype').get(); if(d.external_kind!=='public'||d.prolog_position!==2)throw new Error('doctype'); if(db.query('SELECT parameter FROM xml_entity ORDER BY ordinal').all().map(r=>r.parameter).join(',')!=='0,1')throw new Error('entity'); db.query('UPDATE xml_attribute SET value=? WHERE element_node_id=1 AND ordinal=0').run('SQLite 🌠 & value'); db.exec('UPDATE xml_declaration SET standalone=0'); await Bun.write(Bun.stdout,db.serialize()); db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(&bytes).unwrap(); let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = restore(&import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()).unwrap();
    let mut expected = snapshot; if let Some(XmlNode::Element { attrs, .. }) = &mut expected.doc.root { attrs[0].value = "SQLite 🌠 & value".into(); } expected.doc.declaration.as_mut().unwrap().standalone = Some(false); assert_eq!(edited, expected);
    let before = String::from_utf8(expected.export_utf8().unwrap()).unwrap(); let after = String::from_utf8(edited.export_utf8().unwrap()).unwrap();
    let events = |source: &str| { let mut reader = quick_xml::Reader::from_str(source); let mut result = Vec::new(); loop { let event = reader.read_event().unwrap(); if matches!(event, quick_xml::events::Event::Eof) { break; } result.push(format!("{event:?}")); } result };
    assert_eq!(events(&before), events(&after)); assert!(events(&after).len() >= 15);
}

#[test]
fn sqlite_snapshot_xml_rejects_dangling_components_cycles_and_order() {
    let database = project(&fixture());
    let mut malformed = database.clone(); malformed.table_mut("xml_element").unwrap().rows.remove(0); assert!(restore(&malformed).is_err());
    let mut malformed = database.clone(); malformed.table_mut("xml_child").unwrap().rows[0].values[3] = SqliteValue::Integer(1); assert!(restore(&malformed).is_err());
    let mut malformed = database.clone(); malformed.table_mut("xml_attribute").unwrap().rows[0].values[2] = SqliteValue::Integer(12); assert!(restore(&malformed).is_err());
    let mut malformed = database.clone(); malformed.table_mut("xml_doctype").unwrap().rows[0].values[2] = SqliteValue::Integer(100); assert!(restore(&malformed).is_err());
}

#[test]
fn sqlite_snapshot_xml_owned_valid_guard_preserves_exact_warnings_and_errors(){let mut snapshot=fixture();let dialect=semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:"s.stdio.xml".into(),standard:"1.0".into(),subset:"valid".into()};for mutation in 0..3{let database=project(&snapshot);let mut callback=|_|true;let actual=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).unwrap();let expected=crate::standards::v1_0::subsets::valid::schema::check_valid_conformance(&snapshot);assert_eq!(format!("{:?}",actual.diagnostics),format!("{:?}",expected));assert_eq!(actual.diagnostics.iter().any(|d|matches!(d.severity,dsl::Severity::Error|dsl::Severity::Fatal)),mutation>0);if mutation==0{snapshot.doc.doctype.as_mut().unwrap().name="other".into();}else{snapshot.doc.doctype=None;}}}

#[test]
fn sqlite_snapshot_xml_erased_native_encoding_admission_and_large_field_limits() {
    let snapshot = fixture();
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.xml".into(), standard: "1.0".into(), subset: "*".into() };
    let codec = <XmlSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let restored = (codec.import)("XML preflight", &dialect, project(&snapshot), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let expected = match encoding { SnapshotEncoding::Text => semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), SnapshotEncoding::Binary => semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)) };
        assert_eq!(restored, expected);
    }
    let large = XmlSnapshot { schema: "stdio.xml".into(), doc: XmlDocument { root: Some(XmlNode::Element { name: "root".into(), attrs: vec![XmlAttr { name: "attribute".into(), value: "世界".repeat(30000) }], children: Vec::new() }), ..XmlDocument::default() } };
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 4096, ..SqliteDatabaseLimits::default() })).is_err());
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |p| p.phase != SqliteSnapshotPhase::EncodeNative, SqliteDatabaseLimits::default())).is_err());
}
