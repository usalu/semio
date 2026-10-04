use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};

fn fixture() -> XmlSnapshot { let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap(); XmlSnapshot::import_utf8(fixture["xmlText"].as_str().unwrap().as_bytes()).unwrap() }
fn project(snapshot: &XmlSnapshot) -> SqliteDatabase { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }

#[test]
fn sqlite_snapshot_xml_multiple_document_graph_retains_every_owned_root(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🕸️documents/🔣️.json")).unwrap();let snapshot=fixture();let documents:Vec<_>=plan["keys"].as_array().unwrap().iter().map(|key|(key.as_str().unwrap(),&snapshot.doc)).collect();let limits=SqliteDatabaseLimits::default();
 let database=sqlite::project_xml_documents(&documents,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 let snapshots=sqlite::reconstruct_xml_documents(&database,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(snapshots.len(),documents.len());for(restored,(key,doc))in snapshots.into_iter().zip(&documents){assert_eq!(&restored.schema,key);assert_eq!(&restored.doc,*doc);restored.retire_sqlite_snapshot();}
 let mut reused=database;let root=reused.table("xml_document").unwrap().rows[0].values[2].clone();reused.table_mut("xml_document").unwrap().rows[1].values[2]=root;assert!(sqlite::reconstruct_xml_documents(&reused,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
}

#[test]
fn sqlite_snapshot_xml_multiple_empty_documents_have_known_cancellable_workload(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🕸️documents/🔣️.json")).unwrap();let count=plan["documents"].as_u64().unwrap() as usize;let at=plan["cancelAt"].as_u64().unwrap() as usize;let doc=XmlDocument::default();let keys:Vec<_>=(0..count).map(|ordinal|ordinal.to_string()).collect();let documents:Vec<_>=keys.iter().map(|key|(key.as_str(),&doc)).collect();let limits=SqliteDatabaseLimits::default();let database=sqlite::project_xml_documents(&documents,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for phase in [SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut reached=false;let mut callback=|event:SqliteSnapshotProgress|{if event.phase==phase&&event.total==count&&event.completed==at{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,limits);let failed=if phase==SqliteSnapshotPhase::ProjectSnapshot{sqlite::project_xml_documents(&documents,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut control).is_err()}else{sqlite::reconstruct_xml_documents(&database,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut control).is_err()};assert!(failed);assert!(reached);}
}

#[test]
fn sqlite_snapshot_xml_multiple_document_late_cancel_retires_completed_deep_roots(){
 std::thread::Builder::new().stack_size(256*1024).spawn(||{
  let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🕸️documents/🔣️.json")).unwrap();let depth=plan["depth"].as_u64().unwrap() as usize;let bytes=plan["lateTextBytes"].as_u64().unwrap() as usize;let mut node=XmlNode::Text{text:"leaf".into()};for _ in 0..depth{node=XmlNode::Element{name:"node".into(),attrs:Vec::new(),children:vec![node]};}let first=sqlite::XmlDocumentOwner(Some(XmlDocument{root:Some(node),..XmlDocument::default()}));let second=XmlDocument::default();let text="z".repeat(bytes);let documents=[("first",first.0.as_ref().unwrap()),(text.as_str(),&second)];let limits=SqliteDatabaseLimits::default();let database=sqlite::project_xml_documents(&documents,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let mut reached=false;
  let restored=sqlite::reconstruct_xml_documents(&database,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::ReconstructSnapshot&&event.total==bytes&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true}},limits));assert!(restored.is_err());assert!(reached);
  let restored=sqlite::reconstruct_xml_documents(&database,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.len(),2);for snapshot in restored{snapshot.retire_sqlite_snapshot();}
 }).unwrap().join().unwrap();
}

#[test]
fn sqlite_snapshot_xml_cancels_inside_each_owned_long_text_copy() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🫳️ownership/🔣️.json")).unwrap();
    let text = fixture["unit"].as_str().unwrap().repeat(fixture["repeats"].as_u64().unwrap() as usize);
    let total = fixture["utf8Bytes"].as_u64().unwrap() as usize; assert_eq!(text.len(), total);
    let stop = fixture["cancelAfter"].as_u64().unwrap() as usize; let mut failures = Vec::new();
    for field in fixture["fields"].as_array().unwrap() {
        let field = field.as_str().unwrap(); let mut snapshot = XmlSnapshot::default();
        let mut attrs = Vec::new(); let mut children = Vec::new();
        match field {
            "schema" => snapshot.schema = text.clone(),
            "attribute" => attrs.push(XmlAttr { name: "key".into(), value: text.clone() }),
            "text" => children.push(XmlNode::Text { text: text.clone() }),
            "cdata" => children.push(XmlNode::CData { text: text.clone() }),
            "comment" => children.push(XmlNode::Comment { text: text.clone() }),
            "instruction" => children.push(XmlNode::ProcessingInstruction { target: "target".into(), data: text.clone() }),
            "doctype" => snapshot.doc.doctype = Some(XmlDoctype { prolog_position: 0, name: "root".into(), external_id: Some(XmlExternalId::System { system_id: text.clone() }), declarations: Vec::new() }),
            "entity" => snapshot.doc.doctype = Some(XmlDoctype { prolog_position: 0, name: "root".into(), external_id: None, declarations: vec![XmlDtdDeclaration::Entity { parameter: false, name: "entity".into(), value: text.clone() }] }),
            _ => unreachable!(),
        }
        snapshot.doc.root = Some(XmlNode::Element { name: "root".into(), attrs, children });
        let database = project(&snapshot); assert_eq!(restore(&database).unwrap(), snapshot);
        for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
            let mut observed = false;
            let mut callback = |progress: SqliteSnapshotProgress| { if progress.phase == phase && progress.total == total && progress.completed >= stop && progress.completed < total { observed = true; false } else { true } };
            let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
            let refused = match phase { SqliteSnapshotPhase::ProjectSnapshot => snapshot.to_sqlite_database(&mut control).is_err(), _ => XmlSnapshot::from_sqlite_database(&database, &mut control).is_err() };
            if !refused || !observed { failures.push(format!("{field}:{phase:?}")); }
        }
    }
    assert!(failures.is_empty(), "missing interior owned text cancellation: {failures:?}");
}
fn restore(database: &SqliteDatabase) -> Result<XmlSnapshot, semio_framework_value::ValueError> { XmlSnapshot::from_sqlite_database(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) }

#[test]
fn sqlite_snapshot_xml_whole_native_controls_owned_long_fields() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🫳️ownership/🔣️.json")).unwrap();
    let mut snapshot = fixture(); snapshot.schema = "owned schema 世界".into();
    let XmlNode::Element { attrs, .. } = snapshot.doc.root.as_mut().unwrap() else { unreachable!() };
    attrs.push(XmlAttr { name: "long".into(), value: corpus["unit"].as_str().unwrap().repeat(corpus["repeats"].as_u64().unwrap() as usize) });
    let mut failures = Vec::new();
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        let payload = match encoding { SnapshotEncoding::Text => semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), SnapshotEncoding::Binary => semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)) };
        match XmlSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) { Ok(restored) => assert_eq!(restored, snapshot), Err(error) => failures.push(format!("decode {encoding:?}: {error}")) }
        match snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) { Ok(encoded) => assert_eq!(encoded, payload), Err(error) => failures.push(format!("encode {encoding:?}: {error}")) }
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
            let mut observed = false; let mut callback = |event: SqliteSnapshotProgress| { if event.phase == phase && event.total > 65536 && event.completed >= 65536 && event.completed < event.total { observed = true; false } else { true } };
            let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
            let refused = match phase { SqliteSnapshotPhase::DecodeNative => XmlSnapshot::decode_sqlite_snapshot_native(&payload, &mut control).is_err(), _ => snapshot.encode_sqlite_snapshot_native(encoding, &mut control).is_err() };
            if !refused || !observed { failures.push(format!("interior {encoding:?}:{phase:?}")); }
        }
    }
    assert!(failures.is_empty(), "missing whole native owner controls: {failures:?}");
}

#[test]
fn sqlite_snapshot_xml_owned_declaration_metadata_is_independent_of_utf8_wire() {
    let snapshot = XmlSnapshot { schema: "owned UTF-16 declaration".into(), doc: XmlDocument { declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-16".into()), standalone: Some(false), quote: XmlQuote::Single }), root: Some(XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: Vec::new() }), ..Default::default() } };
    let database = project(&snapshot);
    assert_eq!(database.table("xml_declaration").unwrap().single_row().unwrap().text(3).unwrap(), "UTF-16");
    assert_eq!(restore(&database).unwrap(), snapshot);
}

fn retire_fixture_document(mut document: XmlDocument) {
    let mut nodes = std::mem::take(&mut document.prolog); nodes.extend(std::mem::take(&mut document.epilog)); nodes.extend(document.root.take());
    while let Some(node) = nodes.pop() { if let XmlNode::Element { children, .. } = node { nodes.extend(children); } }
}

#[test]
fn sqlite_snapshot_xml_whole_native_deep_partial_lifecycle() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🫳️ownership/🔣️.json")).unwrap();
    let depth = corpus["depth"].as_u64().unwrap() as usize; let total = corpus["utf8Bytes"].as_u64().unwrap() as usize;
    let long = corpus["unit"].as_str().unwrap().repeat(corpus["repeats"].as_u64().unwrap() as usize);
    let mut root = XmlNode::Text { text: "leaf".into() }; for _ in 0..depth { root = XmlNode::Element { name: "n".into(), attrs: Vec::new(), children: vec![root] }; }
    let snapshot = XmlSnapshot { schema: "owned deep".into(), doc: XmlDocument { root: Some(root), doctype: Some(XmlDoctype { prolog_position: 0, name: "n".into(), external_id: Some(XmlExternalId::System { system_id: long }), declarations: Vec::new() }), ..Default::default() } };
    let payloads = [semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot))];
    retire_fixture_document(snapshot.doc);
    std::thread::Builder::new().stack_size(256 * 1024).spawn(move || {
        for payload in payloads {
            let decoded = XmlSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
            let mut node = decoded.doc.root.as_ref().unwrap(); let mut count = 0; while let XmlNode::Element { children, .. } = node { assert_eq!(children.len(), 1); count += 1; node = &children[0]; } assert_eq!(count, depth);
            let encoding = match &payload { semio_framework_os_kernel::io_schema::IoPayload::Text(_) => SnapshotEncoding::Text, _ => SnapshotEncoding::Binary };
            assert!(decoded.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() == payload);
            decoded.retire_sqlite_snapshot();
            let mut reached = false; let mut callback = |event: SqliteSnapshotProgress| { if event.phase == SqliteSnapshotPhase::DecodeNative && event.total == total && event.completed >= 65536 && event.completed < total { reached = true; false } else { true } };
            assert!(XmlSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).is_err()); assert!(reached);
        }
    }).unwrap().join().unwrap();
}

#[test]
fn sqlite_snapshot_xml_long_schema_cancellation_retires_deep_partial_document() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🫳️ownership/🔣️.json")).unwrap();
    let text = fixture["unit"].as_str().unwrap().repeat(fixture["repeats"].as_u64().unwrap() as usize);
    let mut root = XmlNode::Text { text: "leaf".into() };
    for _ in 0..fixture["depth"].as_u64().unwrap() { root = XmlNode::Element { name: "n".into(), attrs: Vec::new(), children: vec![root] }; }
    let snapshot = XmlSnapshot { schema: text, doc: XmlDocument { root: Some(root), ..Default::default() } };
    let database = project(&snapshot); retire_fixture_document(snapshot.doc);
    let refused = std::thread::Builder::new().stack_size(256 * 1024).spawn(move || {
        let total = fixture["utf8Bytes"].as_u64().unwrap() as usize; let stop = fixture["cancelAfter"].as_u64().unwrap() as usize; let mut observed = false;
        let mut callback = |progress: SqliteSnapshotProgress| { if progress.phase == SqliteSnapshotPhase::ReconstructSnapshot && progress.total == total && progress.completed >= stop && progress.completed < total { observed = true; false } else { true } };
        let result = XmlSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default()));
        let refused = result.is_err(); if let Ok(snapshot) = result { retire_fixture_document(snapshot.doc); }
        refused && observed
    }).unwrap().join().unwrap();
    assert!(refused, "the owned deep document must stop inside the final schema copy and retire partial construction safely");
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_xml_exact_declared_valid_owned_io(){use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("XML SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let dialect=ArtifactDialect{artifact_kind:"s.stdio.xml".into(),standard:"1.0".into(),subset:"valid".into()};let mut snapshot=fixture();snapshot.schema="owned 世界".into();let mut phases=Vec::new();let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Text,SqliteDatabaseLimits::default(),&mut |p|{phases.push(p.phase);true}).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<XmlSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);assert!(!phases.iter().any(|p|matches!(p,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));snapshot.doc.doctype.as_mut().unwrap().name="mismatch".into();assert!(io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.is_err());}

#[test]
fn sqlite_snapshot_xml_schema_admitted_intermediate_boundaries_preserve_owned_fields() {
    let source = include_str!("../../🧫️fixtures/🫳️ownership/📸️logical/🔣️.json");
    let value: pack::value::DslValue = serde_json::from_str(source).unwrap();
    let snapshot = <XmlSnapshot as pack::value::FromValue>::from_value(value).unwrap();
    let mut failures = Vec::new();
    match snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).and_then(|database| restore(&database)) { Ok(restored) => assert_eq!(restored, snapshot), Err(error) => failures.push(format!("SQLite: {error}")) }
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        let payload = snapshot.encode_sqlite_snapshot_native(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        match XmlSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) { Ok(restored) => assert_eq!(restored, snapshot), Err(error) => failures.push(format!("controlled {encoding:?}: {error}")) }
        let ordinary = match payload { semio_framework_os_kernel::io_schema::IoPayload::Text(text) => <XmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|error| error.to_string()), semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes) => <XmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| error.to_string()) };
        match ordinary { Ok(restored) => assert_eq!(restored, snapshot), Err(error) => failures.push(format!("owned state {encoding:?}: {error}")) }
    }
    assert!(failures.is_empty(), "logical XML state was refused by wire policy: {failures:?}");
}

#[test]
fn sqlite_snapshot_xml_named_valid_refuses_missing_document_element() {
    let mut snapshot = fixture(); snapshot.doc.root = Some(XmlNode::Comment { text: "pending element".into() });
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.xml".into(), standard: "1.0".into(), subset: "valid".into() };
    let database = project(&snapshot);
    let outcome = snapshot.validate_sqlite_snapshot_subset(&dialect, &database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert!(outcome.diagnostics.iter().any(|value| matches!(value.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)), "named XML validity must reject a missing document element");
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_xml_actual_declaration_preserves_logical_state_and_rejects_named_valid() {
    use semio_framework_os_kernel::io::{ArtifactDialect, io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot}};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("XML logical state").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let source = include_str!("../../🧫️fixtures/🫳️ownership/📸️logical/🔣️.json");
    let snapshot = <XmlSnapshot as pack::value::FromValue>::from_value(serde_json::from_str(source).unwrap()).unwrap();
    let dialect = ArtifactDialect { artifact_kind: "s.stdio.xml".into(), standard: "1.0".into(), subset: "*".into() }; let valid = ArtifactDialect { subset: "valid".into(), ..dialect.clone() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let bytes = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(io_import_sqlite_snapshot::<XmlSnapshot>(&dialect, &bytes, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value, snapshot);
        assert!(io_export_sqlite_snapshot(&valid, &snapshot, encoding, SqliteDatabaseLimits::default(), &mut |_| true).await.is_err());
        let mut database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        database.table_mut("semio_snapshot").unwrap().rows[0].values[3] = SqliteValue::Text("valid".into());
        let forged = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        assert!(io_import_sqlite_snapshot::<XmlSnapshot>(&valid, &forged, SqliteDatabaseLimits::default(), &mut |_| true).await.is_err());
    }
}

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
    let script = "import { Database } from 'bun:sqlite'; const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())); if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw new Error('integrity'); if(db.query('SELECT name FROM xml_attribute WHERE element_node_id=1 ORDER BY ordinal').all().map(r=>r.name).join(',')!=='z,a')throw new Error('attribute order'); const d=db.query('SELECT external_kind,public_id,system_id,prolog_position_decimal FROM xml_doctype').get(); if(d.external_kind!=='public'||d.prolog_position_decimal!=='2')throw new Error('doctype'); if(db.query('SELECT parameter FROM xml_entity ORDER BY ordinal').all().map(r=>r.parameter).join(',')!=='0,1')throw new Error('entity'); db.query('UPDATE xml_attribute SET value=? WHERE element_node_id=1 AND ordinal=0').run('SQLite 🌠 & value'); db.exec('UPDATE xml_declaration SET standalone=0'); await Bun.write(Bun.stdout,db.serialize()); db.close();";
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
    let mut malformed = database.clone(); malformed.table_mut("xml_doctype").unwrap().rows[0].values[2] = SqliteValue::Integer(-1); assert!(restore(&malformed).is_err());
}

#[test]
fn sqlite_snapshot_xml_owned_valid_guard_preserves_exact_warnings_and_errors(){let mut snapshot=fixture();let dialect=semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:"s.stdio.xml".into(),standard:"1.0".into(),subset:"valid".into()};for mutation in 0..3{let database=project(&snapshot);let mut callback=|_|true;let actual=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).unwrap();let expected=crate::standards::v1_0::subsets::valid::schema::check_valid_conformance(&snapshot);assert_eq!(format!("{:?}",actual.diagnostics),format!("{:?}",expected));assert_eq!(actual.diagnostics.iter().any(|d|matches!(d.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)),mutation>0);if mutation==0{snapshot.doc.doctype.as_mut().unwrap().name="other".into();}else{snapshot.doc.doctype=None;}}}

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

#[test]
fn sqlite_snapshot_xml_full_width_position_is_queryable_and_exact(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧭️position/🔣️.json")).unwrap();
 for position in plan["positions"].as_array().unwrap(){let text=position.as_str().unwrap();let snapshot=XmlSnapshot{schema:"literal".into(),doc:XmlDocument{doctype:Some(XmlDoctype{prolog_position:text.parse().unwrap(),name:"root".into(),..Default::default()}),..Default::default()}};
 let database=project(&snapshot);let row=database.table("xml_doctype").unwrap().rows.first().unwrap();assert_eq!(row.text(2).unwrap(),text);let restored=XmlSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(restored,snapshot);
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(XmlSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);}
 for invalid in plan["invalid"].as_array().unwrap(){let mut edited=database.clone();edited.table_mut("xml_doctype").unwrap().rows[0].values[2]=SqliteValue::Text(invalid.as_str().unwrap().into());assert!(XmlSnapshot::from_sqlite_database(&edited,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e","import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));await Bun.write(Bun.stdout,db.query('SELECT prolog_position_decimal AS position FROM xml_doctype').get().position);db.close();"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap(),text);
 }
}

#[test]
fn sqlite_snapshot_xml_borrowed_node_component_has_exact_owned_root(){
 let snapshot=fixture();let root=snapshot.doc.root.as_ref().unwrap();let limits=SqliteDatabaseLimits::default();let views=[("literal node",sqlite::XmlDocumentView::node(root))];let measured=sqlite::measure_xml_document_views(&views,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let database=sqlite::project_xml_document_views(&views,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(measured.0,database.tables.iter().map(|table|table.rows.len()).sum::<usize>());let restored=sqlite::reconstruct_xml_documents(&database,XmlSnapshot::SQLITE_SCHEMA,sqlite::XML_SQLITE_TABLES,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.len(),1);assert_eq!(restored[0].doc.root.as_ref(),Some(root));assert!(restored[0].doc.prolog.is_empty());assert!(restored[0].doc.epilog.is_empty());assert!(restored[0].doc.doctype.is_none());assert!(restored[0].doc.declaration.is_none());for item in restored{item.retire_sqlite_snapshot();}
}

#[test]
fn sqlite_snapshot_xml_native_component_cursor_preserves_exact_stream_and_owner_counts(){let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧩️native-component/🔣️.json")).unwrap();for case in plan["cases"].as_array().unwrap(){let doc=XmlDocument{root:case["nodeText"].as_str().map(|text|XmlNode::Text{text:text.into()}),..XmlDocument::default()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let body:Vec<u8>=if encoding==SnapshotEncoding::Binary{case["binary"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect()}else{case["text"].as_str().unwrap().as_bytes().to_vec()};let limits=SqliteDatabaseLimits::default();let(mut count,mut rows)=(3,7);let mut encode_callback=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes,&mut encode_callback);let mut output=native.allocate_vec::<u8>(3+body.len()+4).unwrap();output.extend_from_slice(b"pre");native.begin_stage(body.len()).unwrap();emit_xml_native_document(sqlite::XmlDocumentView::from(&doc),XmlNativeEmission{control:&mut native,output:Some(&mut output),count:&mut count,rows:&mut rows,limits,encoding}).unwrap();assert_eq!(&output[3..],body);assert_eq!(count,3+body.len());assert_eq!(rows,7+case["rows"].as_u64().unwrap()as usize);output.extend_from_slice(b"tail");let(mut position,mut read_rows)=(3,7);let mut decode_callback=|_|true;let mut native=semio_framework_value::NativeDecodeControl::new(limits.max_value_bytes,&mut decode_callback);native.begin_stage(output.len()).unwrap();native.advance(3).unwrap();let restored=read_xml_native_document(XmlNativeInput{bytes:&output,position:&mut position,rows:&mut read_rows,limits,binary:encoding==SnapshotEncoding::Binary,control:&mut native}).unwrap();assert_eq!(restored,doc);assert_eq!(position,3+body.len());assert_eq!(read_rows,rows);assert_eq!(&output[position..],b"tail");}}}

#[test]
fn sqlite_snapshot_xml_native_component_interior_cancellation_keeps_cumulative_cursor(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧩️native-component/🔣️.json")).unwrap();let bounded=&plan["control"];let cancel_after=bounded["cancelAfter"].as_u64().unwrap()as usize;let text=bounded["unit"].as_str().unwrap().repeat(bounded["repeats"].as_u64().unwrap()as usize);assert_eq!(text.len(),bounded["utf8Bytes"].as_u64().unwrap()as usize);let doc=XmlDocument{root:Some(XmlNode::Text{text}),..XmlDocument::default()};let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let(mut count,mut rows)=(0,0);let mut accepted=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes,&mut accepted);
  emit_xml_native_document(sqlite::XmlDocumentView::from(&doc),XmlNativeEmission{control:&mut native,output:None,count:&mut count,rows:&mut rows,limits,encoding}).unwrap();assert_eq!(rows,3);
  let total=count;let(mut count,mut rows)=(0,0);let mut bytes=native.allocate_vec::<u8>(total).unwrap();native.begin_stage(total).unwrap();
  emit_xml_native_document(sqlite::XmlDocumentView::from(&doc),XmlNativeEmission{control:&mut native,output:Some(&mut bytes),count:&mut count,rows:&mut rows,limits,encoding}).unwrap();assert_eq!(count,total);
  let mut reached=false;let mut cancel=|event:pack::value::native_encoding::NativeEncodeProgress|if event.total==total&&event.completed>=cancel_after&&event.completed<total{reached=true;false}else{true};let mut native=semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes,&mut cancel);let mut output=native.allocate_vec::<u8>(total).unwrap();native.begin_stage(total).unwrap();let(mut count,mut rows)=(0,0);
  assert_eq!(emit_xml_native_document(sqlite::XmlDocumentView::from(&doc),XmlNativeEmission{control:&mut native,output:Some(&mut output),count:&mut count,rows:&mut rows,limits,encoding}).unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert!(reached);assert_eq!(count,output.len());assert!(count<total);
  let(mut position,mut rows)=(0,0);let mut reached=false;let mut cancel=|event:pack::value::native_decoding::NativeDecodeProgress|if event.total==total&&event.completed>=cancel_after&&event.completed<total{reached=true;false}else{true};let mut native=semio_framework_value::NativeDecodeControl::new(limits.max_value_bytes,&mut cancel);native.begin_stage(total).unwrap();
  assert_eq!(read_xml_native_document(XmlNativeInput{bytes:&bytes,position:&mut position,rows:&mut rows,limits,binary:encoding==SnapshotEncoding::Binary,control:&mut native}).unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert!(reached);assert!(position<total);
  let limited=SqliteDatabaseLimits{max_rows:bounded["refusedMaxRows"].as_u64().unwrap()as usize,..limits};let(mut count,mut rows)=(0,7);let mut accepted=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes,&mut accepted);assert_eq!(emit_xml_native_document(sqlite::XmlDocumentView::from(&doc),XmlNativeEmission{control:&mut native,output:None,count:&mut count,rows:&mut rows,limits:limited,encoding}).unwrap_err().kind,semio_framework_value::ValueRefusalKind::WorkLimit);
 }
}

#[test]
fn sqlite_snapshot_xml_native_component_deep_completed_owner_retires_after_late_error(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧩️native-component/🔣️.json")).unwrap();let depth=plan["control"]["deepNodes"].as_u64().unwrap()as usize;let stack=plan["control"]["smallStackBytes"].as_u64().unwrap()as usize;
 std::thread::Builder::new().stack_size(stack).spawn(move||{
  let mut node=XmlNode::Text{text:"leaf".into()};for _ in 0..depth{node=XmlNode::Element{name:"node".into(),attrs:Vec::new(),children:vec![node]};}let owner=sqlite::XmlDocumentOwner(Some(XmlDocument{root:Some(node),..XmlDocument::default()}));let doc=owner.0.as_ref().unwrap();let limits=SqliteDatabaseLimits::default();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let(mut count,mut rows)=(0,0);let mut accepted=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes,&mut accepted);emit_xml_native_document(sqlite::XmlDocumentView::from(doc),XmlNativeEmission{control:&mut native,output:None,count:&mut count,rows:&mut rows,limits,encoding}).unwrap();let total=count;let expected_rows=rows;let mut bytes=native.allocate_vec::<u8>(total).unwrap();native.begin_stage(total).unwrap();let(mut count,mut rows)=(0,0);emit_xml_native_document(sqlite::XmlDocumentView::from(doc),XmlNativeEmission{control:&mut native,output:Some(&mut bytes),count:&mut count,rows:&mut rows,limits,encoding}).unwrap();
   let mut accepted=|_|true;let mut native=semio_framework_value::NativeDecodeControl::new(limits.max_value_bytes,&mut accepted);native.begin_stage(total).unwrap();let(mut position,mut rows)=(0,0);let restored=sqlite::XmlDocumentOwner(Some(read_xml_native_document(XmlNativeInput{bytes:&bytes,position:&mut position,rows:&mut rows,limits,binary:encoding==SnapshotEncoding::Binary,control:&mut native}).unwrap()));assert_eq!(position,total);assert_eq!(rows,expected_rows);let mut current=restored.0.as_ref().unwrap().root.as_ref().unwrap();let mut observed=0;while let XmlNode::Element{children,..}=current{assert_eq!(children.len(),1);current=&children[0];observed+=1;}assert_eq!(observed,depth);assert!(matches!(current,XmlNode::Text{text}if text=="leaf"));drop(restored);
   *bytes.last_mut().unwrap()=if encoding==SnapshotEncoding::Binary{1}else{b'!'};let mut accepted=|_|true;let mut native=semio_framework_value::NativeDecodeControl::new(limits.max_value_bytes,&mut accepted);native.begin_stage(total).unwrap();let(mut position,mut rows)=(0,0);assert!(read_xml_native_document(XmlNativeInput{bytes:&bytes,position:&mut position,rows:&mut rows,limits,binary:encoding==SnapshotEncoding::Binary,control:&mut native}).is_err());assert!(position>total/2);
  }
 }).unwrap().join().unwrap();
}

#[test]
fn sqlite_snapshot_xml_actual_node_native_cursor_excludes_document_fields(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🌱️native-node/🔣️.json")).unwrap();let limits=SqliteDatabaseLimits::default();for case in plan["cases"].as_array().unwrap(){let node=XmlNode::Text{text:case["text"].as_str().unwrap().into()};for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let expected=if encoding==SnapshotEncoding::Binary{case["wireBinary"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<u8>>()}else{case["wireText"].as_str().unwrap().as_bytes().to_vec()};let(mut count,mut rows)=(0,0);let mut accepted=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes,&mut accepted);let mut bytes=native.allocate_vec::<u8>(expected.len()+4).unwrap();native.begin_stage(expected.len()).unwrap();emit_xml_native_node(&node,XmlNativeEmission{control:&mut native,output:Some(&mut bytes),count:&mut count,rows:&mut rows,limits,encoding}).unwrap();assert_eq!(bytes,expected);assert_eq!(count,expected.len());assert_eq!(rows,2);bytes.extend_from_slice(b"tail");let(mut position,mut read_rows)=(0,0);let mut accepted=|_|true;let mut native=semio_framework_value::NativeDecodeControl::new(limits.max_value_bytes,&mut accepted);native.begin_stage(bytes.len()).unwrap();let restored=read_xml_native_node(XmlNativeInput{bytes:&bytes,position:&mut position,rows:&mut read_rows,limits,binary:encoding==SnapshotEncoding::Binary,control:&mut native}).unwrap();assert_eq!(restored,node);assert_eq!(position,expected.len());assert_eq!(read_rows,2);assert_eq!(&bytes[position..],b"tail");}}
}
