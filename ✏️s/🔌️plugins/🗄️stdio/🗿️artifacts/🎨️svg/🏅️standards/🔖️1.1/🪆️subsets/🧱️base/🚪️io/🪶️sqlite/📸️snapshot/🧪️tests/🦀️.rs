use crate::standards::v1_1::subsets::base::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }

fn native_control_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🛂️native.json")).unwrap() }

#[test]
fn sqlite_snapshot_svg_literal_metadata_native_state_is_distinct_from_external_wire() {
    let mut value = snapshot(); let f = native_control_fixture();
    value.schema = f["literalSchema"].as_str().unwrap().into();
    value.doc.declaration.as_mut().unwrap().version = f["metadata"]["version"].as_str().unwrap().into();
    value.doc.declaration.as_mut().unwrap().encoding = Some(f["metadata"]["encoding"].as_str().unwrap().into());
    value.doc.doctype.as_mut().unwrap().prolog_position = f["metadata"]["prologPosition"].as_str().unwrap().parse().unwrap();
    assert_eq!(restore(&project(&value)).unwrap(), value);
    assert_eq!(<SvgSnapshot as store::ArtifactPack>::decode_pack(&<SvgSnapshot as store::ArtifactPack>::encode_pack(&value)).expect("own semantic snapshot pack must retain literal metadata"), value);
    assert_eq!(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(&<SvgSnapshot as store::ArtifactDsl>::print_dsl(&value)).expect("own semantic snapshot text must retain literal metadata"), value);
}

fn native_unicode_snapshot() -> SvgSnapshot {
    let f = native_control_fixture();
    SvgSnapshot { schema: f["literalSchema"].as_str().unwrap().into(), doc: SvgDocument { root: Some(SvgNode::Element { name: "svg".into(), attrs: vec![], children: vec![SvgNode::Text { text: f["text"].as_str().unwrap().repeat(f["repeat"].as_u64().unwrap() as usize) }] }), ..SvgDocument::default() } }
}
fn native_payload(snapshot: &SvgSnapshot, encoding: SnapshotEncoding) -> semio_framework_os_kernel::io_schema::IoPayload {
    match encoding { SnapshotEncoding::Binary => semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(snapshot)), SnapshotEncoding::Text => semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(snapshot)) }
}

#[test]
fn sqlite_snapshot_svg_native_long_unicode_carriers_and_interior_cancellation() {
    let snapshot = native_unicode_snapshot(); let f = native_control_fixture();
    let at = f["cancelAtBytes"].as_u64().unwrap() as usize;
    let codec = <SvgSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().expect("actual SVG owner capability");
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.svg".into(), standard: "1.1".into(), subset: "*".into() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let payload = native_payload(&snapshot, encoding);
        let actual = (codec.export)("SVG controlled input", &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).expect("actual owner must decode under native control").value;
        assert_eq!(restore(&actual).unwrap(), snapshot);
        assert_eq!((codec.import)("SVG controlled output", &dialect, actual, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).expect("actual owner must encode under native control").value, payload);
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
            let mut interrupted = false;
            let mut callback = |p: SqliteSnapshotProgress| { let cancel = p.phase == phase && p.completed >= at && p.completed < p.total; interrupted |= cancel; !cancel };
            let result = if phase == SqliteSnapshotPhase::DecodeNative {
                (codec.export)("SVG interior input", &dialect, &payload, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ())
            } else {
                (codec.import)("SVG interior output", &dialect, project(&snapshot), encoding, &mut SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default())).map(|_| ())
            };
            assert!(result.is_err(), "native {phase:?} must honor interior cancellation");
            assert!(interrupted, "native {phase:?} must report a known nonzero interior frontier");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_svg_native_limits_and_literal_typed_file_io() {
    use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot};
    let snapshot = native_unicode_snapshot(); let f = native_control_fixture();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.svg".into(), standard: "1.1".into(), subset: "*".into() };
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("SVG owned SQLite controls").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let codec = <SvgSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    let wide = SvgSnapshot { schema: snapshot.schema.clone(), doc: SvgDocument { root: Some(SvgNode::Element { name: "svg".into(), attrs: vec![], children: (0..f["wideChildren"].as_u64().unwrap()).map(|_| SvgNode::Element { name: "g".into(), attrs: vec![], children: vec![] }).collect() }), ..SvgDocument::default() } };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let bytes = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value;
        assert_eq!(io_import_sqlite_snapshot::<SvgSnapshot>(&dialect, &bytes, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value, snapshot);
        let payload = native_payload(&snapshot, encoding);
        let limits = SqliteDatabaseLimits { max_file_bytes: match &payload { semio_framework_os_kernel::io_schema::IoPayload::Text(text) => text.len(), semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes) => bytes.len() } - 1, ..SqliteDatabaseLimits::default() };
        assert!((codec.export)("SVG file admission", &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        let limits = SqliteDatabaseLimits { max_value_bytes: f["maxValueBytes"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
        assert!((codec.export)("SVG owned input", &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        assert!((codec.import)("SVG owned output", &dialect, project(&snapshot), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
        let limits = SqliteDatabaseLimits { max_rows: f["maxDomainRows"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() };
        let error = (codec.export)("SVG domain census", &dialect, &native_payload(&wide, encoding), &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap_err();
        assert_eq!(error.cause.kind, semio_framework_value::ValueRefusalKind::WorkLimit, "domain row census must refuse before materialization: {error:?}");
    }
}

fn snapshot() -> SvgSnapshot { SvgSnapshot::import_utf8(fixture()["svgText"].as_str().unwrap().as_bytes()).unwrap() }
fn project(snapshot: &SvgSnapshot) -> SqliteDatabase { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }
fn restore(database: &SqliteDatabase) -> Result<SvgSnapshot, semio_framework_value::ValueError> { SvgSnapshot::from_sqlite_database(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) }

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_svg_exact_declared_tiny_basic_owned_io(){use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("SVG SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();for(subset,valid,blocked)in [("tiny","tinyText","tinyBlockedText"),("basic","basicText","basicBlockedText")]{let f=fixture();let dialect=ArtifactDialect{artifact_kind:"s.stdio.svg".into(),standard:"1.1".into(),subset:subset.into()};let mut snapshot=SvgSnapshot::import_utf8(f[valid].as_str().unwrap().as_bytes()).unwrap();snapshot.schema="owned 世界".into();let mut phases=Vec::new();let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Text,SqliteDatabaseLimits::default(),&mut |p|{phases.push(p.phase);true}).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<SvgSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);assert!(!phases.iter().any(|p|matches!(p,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));let invalid=SvgSnapshot::import_utf8(f[blocked].as_str().unwrap().as_bytes()).unwrap();assert!(io_export_sqlite_snapshot(&dialect,&invalid,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |_|true).await.is_err());}}

#[test]
fn sqlite_snapshot_svg_domain_names_and_all_native_document_fields_roundtrip() {
    let snapshot = snapshot(); let database = project(&snapshot);
    assert!(database.tables.iter().all(|table| table.name.starts_with("svg_")));
    assert_eq!(database.table("svg_cdata").unwrap().rows.len(), 1);
    assert_eq!(database.table("svg_entity").unwrap().rows.len(), 1);
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(); let restored = restore(&import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()).unwrap(); assert_eq!(restored, snapshot);
    assert_eq!(restored.export_utf8().unwrap(), snapshot.export_utf8().unwrap());
    assert_eq!(<SvgSnapshot as store::ArtifactPack>::encode_pack(&restored), <SvgSnapshot as store::ArtifactPack>::encode_pack(&snapshot));
    assert_eq!(<SvgSnapshot as store::ArtifactDsl>::print_dsl(&restored), <SvgSnapshot as store::ArtifactDsl>::print_dsl(&snapshot));
    assert_eq!(restore(&project(&SvgSnapshot::default())).unwrap(), SvgSnapshot::default());
}

#[test]
fn sqlite_snapshot_svg_rejects_foreign_roots_shapes_graphs_and_budgets() {
    let original = project(&snapshot());
    for alteration in 0..5 { let mut database = original.clone(); match alteration {
        0 => database.table_mut("svg_element").unwrap().rows[0].values[1] = SqliteValue::Text("html".into()),
        1 => database.table_mut("svg_child").unwrap().rows[0].values[3] = SqliteValue::Integer(1),
        2 => database.table_mut("svg_document").unwrap().rows[0].values[2] = SqliteValue::Null,
        3 => database.table_mut("svg_attribute").unwrap().rows[0].values[2] = SqliteValue::Integer(999),
        _ => { database.table_mut("svg_cdata").unwrap().rows.clear(); },
    } assert!(restore(&database).is_err(), "alteration {alteration}"); }
    let mut foreign = SvgSnapshot::default(); foreign.doc.root = None; assert!(foreign.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    let source = snapshot(); assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
    let limits = SqliteDatabaseLimits { max_rows: 1, max_value_bytes: 1, ..SqliteDatabaseLimits::default() }; assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err()); assert!(SvgSnapshot::from_sqlite_database(&original, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}

#[test]
fn sqlite_snapshot_svg_independent_geometry_queries_and_edits_preserve_native_state() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let source = snapshot(); let bytes = export_sqlite_database(&project(&source), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import { Database } from 'bun:sqlite'; const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())); if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw new Error('integrity'); const p=db.query(\"SELECT a.value FROM svg_attribute a JOIN svg_element e ON e.node_id=a.element_node_id WHERE e.name='path' AND a.name='d'\").get(); if(p.value!=='M 0 0 L 10 20 Z')throw new Error('path'); if(db.query('SELECT text FROM svg_cdata').get().text!=='path { fill: red; }')throw new Error('CDATA'); if(db.query('SELECT system_id FROM svg_doctype').get().system_id!=='svg.dtd')throw new Error('doctype'); db.query('UPDATE svg_attribute SET value=? WHERE name=?').run('M1 2C3 4 5 6 7 8Z','d'); await Bun.write(Bun.stdout,db.serialize()); db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(&bytes).unwrap(); let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let restored = restore(&import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()).unwrap();
    let mut expected = source; fn edit(node: &mut SvgNode) { if let SvgNode::Element { attrs, children, .. } = node { for attr in attrs { if attr.name == "d" { attr.value = crate::schema::snapshot::SvgAttributeValue::PathData(vec![crate::schema::snapshot::PathCommand::MoveTo{x:1.0,y:2.0,relative:false},crate::schema::snapshot::PathCommand::CurveTo{x1:3.0,y1:4.0,x2:5.0,y2:6.0,x:7.0,y:8.0,relative:false},crate::schema::snapshot::PathCommand::ClosePath]); } } for child in children { edit(child); } } } edit(expected.doc.root.as_mut().unwrap()); assert_eq!(restored, expected);
    assert_eq!(<SvgSnapshot as store::ArtifactPack>::decode_pack(&<SvgSnapshot as store::ArtifactPack>::encode_pack(&restored)).unwrap(), restored);
    assert_eq!(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(&<SvgSnapshot as store::ArtifactDsl>::print_dsl(&restored)).unwrap(), restored);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_svg_tiny_and_basic_io_validators_recheck_reconstructed_entities() {
    use semio_framework_plugin::{IoPayload, Severity, SubsetValidator};
    use crate::standards::v1_1::subsets::{tiny::io::SvgTinyValidator, basic::io::SvgBasicValidator};
    for (key, tiny) in [("tinyText", true), ("basicText", false)] {
        let source = SvgSnapshot::import_utf8(fixture()[key].as_str().unwrap().as_bytes()).unwrap(); let restored = restore(&project(&source)).unwrap(); assert_eq!(restored, source);
        for payload in [IoPayload::Binary(<SvgSnapshot as store::ArtifactPack>::encode_pack(&restored)), IoPayload::Text(<SvgSnapshot as store::ArtifactDsl>::print_dsl(&restored))] {
            let diagnostics = if tiny { SvgTinyValidator::validate(&payload).await } else { SvgBasicValidator::validate(&payload).await }; assert!(diagnostics.iter().all(|diagnostic| diagnostic.severity != Severity::Error), "{diagnostics:?}");
        }
        let mut edited = project(&source); edited.table_mut("svg_element").unwrap().rows.iter_mut().find(|row| row.text(1).unwrap() == "rect").unwrap().values[1] = SqliteValue::Text(if tiny { "linearGradient" } else { "feTurbulence" }.into()); let edited = restore(&edited).unwrap();
        let diagnostics = if tiny { SvgTinyValidator::validate(&IoPayload::Binary(<SvgSnapshot as store::ArtifactPack>::encode_pack(&edited))).await } else { SvgBasicValidator::validate(&IoPayload::Binary(<SvgSnapshot as store::ArtifactPack>::encode_pack(&edited))).await }; assert!(diagnostics.iter().any(|diagnostic| diagnostic.severity == Severity::Error), "{diagnostics:?}");
    }
}

#[test]
fn sqlite_snapshot_svg_owned_tiny_basic_guard_matches_all_existing_conformance_rules(){let f=fixture();for(subset,key)in [("tiny","tinyText"),("tiny","tinyBlockedText"),("basic","basicText"),("basic","basicBlockedText"),("basic","basicClippedText")]{let snapshot=SvgSnapshot::import_utf8(f[key].as_str().unwrap().as_bytes()).unwrap();let database=project(&snapshot);let mut callback=|_|true;let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.svg".into(),standard:"1.1".into(),subset:subset.into()};let actual=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).unwrap();let expected=if subset=="tiny"{crate::standards::v1_1::subsets::tiny::schema::conformance::check_svg_tiny_conformance(&snapshot)}else{crate::standards::v1_1::subsets::basic::schema::conformance::check_svg_basic_conformance(&snapshot)};assert_eq!(format!("{:?}",actual.diagnostics),format!("{:?}",expected));assert_eq!(actual.diagnostics.iter().any(|d|matches!(d.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)),key.contains("Blocked")||key.contains("Clipped"));}let mut snapshot=SvgSnapshot::default();if let Some(SvgNode::Element{children,..})=&mut snapshot.doc.root{children.extend((0..1024).map(|_|SvgNode::Element{name:"g".into(),attrs:vec![],children:vec![]}));}let mut calls=0;let mut callback=|_|{calls+=1;calls<2};assert!(crate::standards::v1_1::subsets::tiny::io::check_svg_tiny_conformance_controlled(&snapshot,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());}

#[test]
fn sqlite_snapshot_svg_erased_native_preflight_admission_and_limits() {
    let snapshot = snapshot();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.svg".into(), standard: "1.1".into(), subset: "*".into() };
    let codec = <SvgSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let actual = (codec.import)("svg preflight", &dialect, project(&snapshot), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let expected = match encoding { SnapshotEncoding::Text => semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), SnapshotEncoding::Binary => semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)) };
        assert_eq!(actual, expected);
    }
    let large = SvgSnapshot { schema: "stdio.svg".into(), doc: SvgDocument { root: Some(SvgNode::Element { name: "svg".into(), attrs: Vec::new(), children: vec![SvgNode::Text { text: "x".repeat(100000) }] }), ..SvgDocument::default() } };
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 4096, ..SqliteDatabaseLimits::default() })).is_err());
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |p| p.phase != SqliteSnapshotPhase::EncodeNative, SqliteDatabaseLimits::default())).is_err());
}
