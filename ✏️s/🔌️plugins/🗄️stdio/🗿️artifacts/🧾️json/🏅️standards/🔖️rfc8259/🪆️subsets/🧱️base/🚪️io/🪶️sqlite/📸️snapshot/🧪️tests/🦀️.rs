use crate::standards::v_rfc8259::subsets::base::io::sqlite::snapshot::*;
use crate::standards::v_rfc8259::subsets::base::{io::{binary::snapshot::owned_pack,text::snapshot::{parse_json_text,write_json_text}},schema::snapshot::demo_json_snapshot};
#[path="💰️allocation/🦀️.rs"]
mod owned_input_allocation;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> (serde_json::Value, JsonSnapshot) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let snapshot = JsonSnapshot { schema: fixture["schema"].as_str().unwrap().into(), value: parse_json_text(fixture["jsonText"].as_str().unwrap()).unwrap() };
    (fixture, snapshot)
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_json_exact_declared_i_json_owned_io() {
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
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
fn sqlite_snapshot_json_owned_i_json_guard_matches_native_rules_and_cancels(){let(fixture,_)=fixture();for case in fixture["iJsonCases"].as_array().unwrap(){let snapshot=JsonSnapshot{schema:"stdio.json".into(),value:parse_json_text(case["text"].as_str().unwrap()).unwrap()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let database=snapshot.to_sqlite_database(&mut control).unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"i-json".into()};let actual=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut control).unwrap();let expected=crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance(&snapshot);assert_eq!(format!("{:?}",actual.diagnostics),format!("{:?}",expected));assert_eq!(actual.diagnostics.iter().any(|d|matches!(d.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)),case["hard"].as_bool().unwrap());}let snapshot=JsonSnapshot{schema:"stdio.json".into(),value:JsonValue::String{value:"x".repeat(100000)}};let mut calls=0;let mut callback=|_|{calls+=1;calls<3};assert!(crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance_controlled(&snapshot,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());}

#[test]
fn sqlite_snapshot_json_native_encoding_preflight_bounds_escaping_indentation_and_cancellation() {
    let (_, snapshot) = fixture();
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let codec = <JsonSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.json".into(), standard: "rfc8259".into(), subset: "*".into() };
        let restored = (codec.import)("JSON preflight", &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let expected = match encoding { SnapshotEncoding::Text => semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), SnapshotEncoding::Binary => semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)) };
        assert_eq!(restored, expected);
    }
    let large = JsonSnapshot { schema: "stdio.json".into(), value: JsonValue::String { value: "\u{0001}".repeat(100000) } };
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 4096, ..SqliteDatabaseLimits::default() })).is_err());
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |p| p.phase != SqliteSnapshotPhase::EncodeNative, SqliteDatabaseLimits::default())).is_err());
    let mut nested = JsonValue::Null; for _ in 0..256 { nested = JsonValue::Array { items: vec![nested] }; }
    let deep = JsonSnapshot { schema: "stdio.json".into(), value: nested };
    let limits = SqliteDatabaseLimits { max_value_bytes: 250000, ..SqliteDatabaseLimits::default() };
    deep.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    deep.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
    assert!(deep.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes:4096, ..limits })).is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_json_exact_geojson_owned_io_and_borrowed_profile() {
 use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌍️geojson/🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("GeoJSON SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"geojson".into()};
 for item in fixture["cases"].as_array().unwrap(){let snapshot=JsonSnapshot{schema:"owned GeoJSON 世界".into(),value:parse_json_text(item["text"].as_str().unwrap()).unwrap()};let expected=crate::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance(&snapshot);let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let outcome=snapshot.validate_sqlite_snapshot_subset(&dialect,&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(outcome.diagnostics,expected,"{}",item["id"]);let exported=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Text,SqliteDatabaseLimits::default(),&mut |_|true).await;if expected.iter().any(|d|matches!(d.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)){assert!(exported.is_err());}else{let bytes=exported.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<JsonSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);}}
}

#[test]
fn sqlite_snapshot_json_erased_native_preserves_owned_schema_lexemes_and_duplicate_members() {
 let(fixture,mut snapshot)=fixture();snapshot.schema=fixture["logicalNative"]["schema"].as_str().unwrap().into();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 let codec=<JsonSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"*".into()};
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=(codec.import)("JSON erased fidelity",&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=match payload{semio_framework_os_kernel::io_schema::IoPayload::Text(text)=><JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes)=><JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap()};assert_eq!(restored.schema,snapshot.schema,"{encoding:?}");assert_eq!(restored.value,snapshot.value,"{encoding:?}");let roundtrip=(codec.export)("JSON erased fidelity",&dialect,&match encoding{SnapshotEncoding::Text=>semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&restored)),SnapshotEncoding::Binary=>semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&restored))},&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(roundtrip,database);}
}

#[test]
fn sqlite_snapshot_json_erased_native_deep_tree_has_no_wire_depth_or_indentation_quota(){
 let(fixture,_)=fixture();let mut value=JsonValue::Number{lexeme:fixture["logicalNative"]["numberLexemes"][0].as_str().unwrap().into()};for _ in 0..fixture["logicalNative"]["depth"].as_u64().unwrap(){value=JsonValue::Array{items:vec![value]}}let snapshot=JsonSnapshot{schema:fixture["logicalNative"]["schema"].as_str().unwrap().into(),value};let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let codec=<JsonSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"*".into()};for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=(codec.import)("JSON erased depth",&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=match payload{semio_framework_os_kernel::io_schema::IoPayload::Text(text)=><JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes)=><JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap()};assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),database);}}


#[test]
fn sqlite_snapshot_json_handwritten_logical_records_validate_owned_topology_and_identity(){
 let valid=r#"schema="owned" nodes=[{kind=object members=[{key="dup" value=1} {key="dup" value=2}] items=[]} {kind=number number-lexeme="-0.00e-2" items=[] members=[]} {kind=null items=[] members=[]}]"#;
 let parsed=<JsonSnapshot as store::ArtifactDsl>::parse_dsl(valid).unwrap();assert_eq!(parsed.schema,"owned");assert_eq!(parsed.value,JsonValue::Object{members:vec![JsonMember{key:"dup".into(),value:JsonValue::Number{lexeme:"-0.00e-2".into()}},JsonMember{key:"dup".into(),value:JsonValue::Null}]});
 for invalid in [valid.replace("value=1","value=0"),valid.replace("value=2","value=1"),valid.replace("value=2","value=999"),valid.replace("kind=null","kind=null string-value=\"unused\""),String::from("schema=\"owned\" nodes=[{kind=null items=[] members=[]} {kind=null items=[] members=[]}]"),format!("semio stdio.md.dsl v1\n{valid}"),format!("{valid} unexpected-field=1")]{assert!(<JsonSnapshot as store::ArtifactDsl>::parse_dsl(&invalid).is_err(),"{invalid}");}
}

#[test]
fn sqlite_snapshot_json_typed_intermediate_number_strings_independent_numeric_query_and_erased_fidelity(){
 use std::io::Write;use std::process::{Command,Stdio};let(fixture,_)=fixture();let members=fixture["typedNumberCases"].as_array().unwrap().iter().map(|case|JsonMember{key:fixture["logicalNative"]["duplicateKey"].as_str().unwrap().into(),value:JsonValue::Number{lexeme:case["lexeme"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap_or(1)as usize)}}).collect();let snapshot=JsonSnapshot{schema:fixture["logicalNative"]["schema"].as_str().unwrap().into(),value:JsonValue::Object{members}};let limits=SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(JsonSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
 let bytes=export_sqlite_database(&database,limits,&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');for(const row of db.query('SELECT m.key,v.number_lexeme,v.number_value FROM json_object_member m JOIN json_value v ON v.id=m.value_id ORDER BY m.ordinal').all()){let expected=null;try{const value=JSON.parse(row.number_lexeme);if(typeof value==='number'&&Number.isFinite(value))expected=value===0?0:value;}catch{}if(row.key!=='z'||row.number_value!==expected)throw Error('numeric cell');}db.query('UPDATE json_value SET number_lexeme=?,number_value=NULL WHERE id=2').run('arbitrary intermediate lexeme');await Bun.write(Bun.stdout,db.serialize());db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let restored=JsonSnapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let JsonValue::Object{members}=&restored.value else{panic!("object")};assert_eq!(members[0].value,JsonValue::Number{lexeme:"arbitrary intermediate lexeme".into()});
 let codec=<JsonSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.json".into(),standard:"rfc8259".into(),subset:"*".into()};for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=(codec.import)(&snapshot.schema,&dialect,database.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value;assert_eq!((codec.export)(&snapshot.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap().value,database);}
 for(column,value)in[(5,SqliteValue::Real(1.0)),(3,SqliteValue::Text("1".into()))]{let mut changed=database.clone();changed.table_mut("json_value").unwrap().rows[1].values[column]=value;assert!(JsonSnapshot::from_sqlite_database(&changed,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}
}


#[test]
fn sqlite_snapshot_json_typed_number_meaning_is_owned_by_named_profile_guards(){
 let many=JsonSnapshot{schema:"owned".into(),value:JsonValue::Array{items:(0..600).map(|_|JsonValue::Number{lexeme:"1".into()}).collect()}};
 assert!(crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance_controlled(&many,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:10,..SqliteDatabaseLimits::default()})).is_err());
 let mut visited=0;let mut proceed=|progress:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{visited=progress.completed;visited<256};assert!(crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance_controlled(&many,&mut SqliteSnapshotControl::new(&mut proceed,SqliteDatabaseLimits::default())).is_err());assert_eq!(visited,256);
 let(fixture,_)=fixture();let geo:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌍️geojson/🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let limits=SqliteDatabaseLimits::default();
 for case in fixture["iJsonTypedNumberCases"].as_array().unwrap(){let snapshot=JsonSnapshot{schema:"owned".into(),value:JsonValue::Object{members:vec![JsonMember{key:"number".into(),value:JsonValue::Number{lexeme:case["lexeme"].as_str().unwrap().into()}}]}};let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(JsonSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);let actual=crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance_controlled(&snapshot,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(actual.iter().any(|d|matches!(d.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)),case["hard"].as_bool().unwrap(),"{}",case["lexeme"]);}
 for property in [false,true]{let cases=if property{&geo["foreignNumberCases"]}else{&geo["typedNumberCases"]};for case in cases.as_array().unwrap(){let number=JsonValue::Number{lexeme:case["lexeme"].as_str().unwrap().into()};let value=if property{JsonValue::Object{members:vec![JsonMember{key:"type".into(),value:JsonValue::String{value:"Feature".into()}},JsonMember{key:"geometry".into(),value:JsonValue::Null},JsonMember{key:"properties".into(),value:JsonValue::Object{members:vec![JsonMember{key:"number".into(),value:number}]}}]}}else{JsonValue::Object{members:vec![JsonMember{key:"type".into(),value:JsonValue::String{value:"Point".into()}},JsonMember{key:"coordinates".into(),value:JsonValue::Array{items:vec![number,JsonValue::Number{lexeme:"0".into()}]}}]}};let snapshot=JsonSnapshot{schema:"owned".into(),value};let actual=crate::standards::v_rfc8259::subsets::geojson::io::sqlite::snapshot::check_geojson_conformance_controlled(&snapshot,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(actual.iter().any(|d|matches!(d.severity,semio_framework_diagnostic::Severity::Error|semio_framework_diagnostic::Severity::Fatal)),case["hard"].as_bool().unwrap(),"{}",case["lexeme"]);}}
}

#[test]
fn sqlite_snapshot_json_owned_record_identity_matches_handwritten_definition() {
    let graph = store::os_pack::PackSchemaGraph::of(&<JsonSnapshot as store::ArtifactPack>::record_spec().unwrap());
    let bytes = graph.canonical_bytes();
    let independent = blake3::hash(&bytes);
    let factory = crate::native_codecs().pop().unwrap();
    let codec = (factory.codec)();
    assert_eq!(codec.pack_schema_hash, *independent.as_bytes());
    let definition: serde_json::Value = serde_json::from_str(crate::ARTIFACT_DEFINITION_SCHEMA).unwrap();
    let binding = &definition["codecs"][0]["native_factory"];
    assert_eq!(binding["pack_schema_hash"].as_str().unwrap(), independent.to_hex().as_str());
    assert_eq!(binding["artifact_schema"].as_str().unwrap(), codec.schema);
    assert_eq!(binding["extension"].as_str().unwrap(), codec.extension);
    crate::definition().unwrap();
}

#[test]
fn sqlite_snapshot_json_native_decode_owner_restores_typed_state_with_interior_control(){
 let(fixture,_)=fixture();let limits=SqliteDatabaseLimits::default();let scalar=JsonValue::Number{lexeme:"NaN".into()};let snapshot=JsonSnapshot{schema:fixture["logicalNative"]["schema"].as_str().unwrap().into(),value:JsonValue::Array{items:(0..fixture["controlledNative"]["nodes"].as_u64().unwrap()).map(|_|scalar.clone()).collect()}};
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=match encoding{SnapshotEncoding::Text=>semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)),SnapshotEncoding::Binary=>semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot))};let restored=JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored,snapshot);assert!(JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:fixture["controlledNative"]["maxRows"].as_u64().unwrap()as usize,..limits})).is_err());}
 let snapshot=JsonSnapshot{schema:"owned".into(),value:JsonValue::String{value:"x".repeat(fixture["controlledNative"]["stringRepeat"].as_u64().unwrap()as usize)}};
 for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{let payload=match encoding{SnapshotEncoding::Text=>semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)),SnapshotEncoding::Binary=>semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot))};let refusal=JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_allocation_bytes:fixture["controlledNative"]["maxAllocationBytes"].as_u64().unwrap()as usize,..limits})).expect_err("controlled Native backing must refuse at the authored allocation ceiling");assert_eq!(refusal.kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit,"{encoding:?}: {refusal:?}");let mut interior=false;let threshold=fixture["controlledNative"]["cancelCompleted"].as_u64().unwrap()as usize;let mut proceed=|progress:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{if progress.phase==SqliteSnapshotPhase::DecodeNative&&progress.completed>=threshold&&progress.completed<progress.total{interior=true;false}else{true}};assert!(JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut proceed,limits)).is_err());assert!(interior,"{encoding:?}: controlled native primitive decoding must admit cancellation during its actual work");}
}

#[test]
fn sqlite_snapshot_json_controlled_flat_binding_admits_storage_and_topology_before_copy(){
 let(fixture,_)=fixture();let count=fixture["controlledNative"]["nodes"].as_u64().unwrap()as usize;
 let source=JsonSnapshot{schema:fixture["logicalNative"]["schema"].as_str().unwrap().into(),value:JsonValue::Array{items:(0..count).map(|_|JsonValue::Number{lexeme:"NaN".into()}).collect()}};
 let payload=store::ArtifactDsl::print_dsl(&source);let(_,body)=store::semio_format::split_text_preamble(&payload).unwrap();let record=semio_framework_dsl_record::parse(body,&<JsonSnapshot as store::ArtifactPack>::record_spec().unwrap(),&semio_framework_dsl_record::ParseOptions::default()).unwrap();
 let mut proceed=|_:semio_framework_value::native_decoding::NativeDecodeProgress|true;
 assert_eq!(owned_pack::reconstruct_record(&record,&mut semio_framework_value::native_decoding::NativeDecodeControl::new(1_000_000,&mut proceed),10_000).unwrap(),source);
 assert!(owned_pack::reconstruct_record(&record,&mut semio_framework_value::native_decoding::NativeDecodeControl::new(1_000_000,&mut proceed),10).is_err());
 assert!(owned_pack::reconstruct_record(&record,&mut semio_framework_value::native_decoding::NativeDecodeControl::new(4096,&mut proceed),10_000).is_err());
 let mut observed=0;let mut cancel=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{observed=event.completed;event.completed<256};assert!(owned_pack::reconstruct_record(&record,&mut semio_framework_value::native_decoding::NativeDecodeControl::new(1_000_000,&mut cancel),10_000).is_err());assert_eq!(observed,256);
 let payload="schema=owned nodes=[{kind=array items=[0] members=[]}]";let record=semio_framework_dsl_record::parse(payload,&<JsonSnapshot as store::ArtifactPack>::record_spec().unwrap(),&semio_framework_dsl_record::ParseOptions::default()).unwrap();assert!(owned_pack::reconstruct_record(&record,&mut semio_framework_value::native_decoding::NativeDecodeControl::new(1_000_000,&mut proceed),10_000).is_err());
}

#[test]
fn sqlite_snapshot_json_authored_grammar_recognizes_its_own_full_state_logical_frame(){
 let grammar=semio_framework_dsl::grammar::parse_grammar(include_str!("../../../📝️text/📸️snapshot/📖️.grammar.semio")).unwrap();let recognizer=semio_framework_dsl::grammar::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");let(fixture,mut snapshot)=fixture();let text=store::ArtifactDsl::print_dsl(&snapshot);assert!(recognizer.recognize(&text).unwrap(),"JSON authored output frame: {text}");for case in fixture["typedNumberCases"].as_array().unwrap(){snapshot.value=JsonValue::Number{lexeme:case["lexeme"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap_or(1)as usize)};let text=store::ArtifactDsl::print_dsl(&snapshot);assert!(recognizer.recognize(&text).unwrap(),"JSON numeric output frame: {text}");}
}

#[test]
fn sqlite_snapshot_json_controlled_flat_output_preserves_full_tree_and_cancels_during_unicode(){
 let(neutral,_)=fixture();let mut value=JsonValue::Object{members:neutral["typedNumberCases"].as_array().unwrap().iter().map(|case|JsonMember{key:neutral["logicalNative"]["duplicateKey"].as_str().unwrap().into(),value:JsonValue::Number{lexeme:case["lexeme"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap_or(1)as usize)}}).collect()};for _ in 0..neutral["logicalNative"]["depth"].as_u64().unwrap(){value=JsonValue::Array{items:vec![value]};}let mut source=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(JsonSnapshot{schema:neutral["logicalNative"]["schema"].as_str().unwrap().repeat(neutral["controlledNative"]["stringRepeat"].as_u64().unwrap()as usize),value},owned_pack::retire);let length=source.as_mut().schema.len();let limits=SqliteDatabaseLimits::default();let expected=source.as_mut().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=source.as_mut().encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).expect("JSON actual controlled logical output owner");let mut restored=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),owned_pack::retire);assert_eq!(restored.as_mut().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);let mut interior=false;assert!(source.as_mut().encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut|event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==length&&event.completed>=neutral["controlledNative"]["cancelCompleted"].as_u64().unwrap()as usize&&event.completed<event.total{interior=true;false}else{true}},limits)).is_err());assert!(interior);for limited in[SqliteDatabaseLimits{max_rows:neutral["controlledNative"]["maxRows"].as_u64().unwrap()as usize,..limits},SqliteDatabaseLimits{max_allocation_bytes:neutral["controlledNative"]["maxAllocationBytes"].as_u64().unwrap()as usize,..limits}]{let expected_kind=if limited.max_rows<limits.max_rows{store::sqlite_snapshot::ValueRefusalKind::WorkLimit}else{store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit};assert_eq!(source.as_mut().encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limited)).expect_err("controlled Native output must refuse the authored work or backing ceiling").kind,expected_kind,"{encoding:?}: limits {limited:?}");}}
}

#[test]
fn sqlite_snapshot_json_physical_writers_cancel_inside_owned_unicode_without_projection() {
    let (neutral, mut snapshot) = fixture();
    snapshot.schema = neutral["logicalNative"]["schema"].as_str().unwrap().repeat(neutral["controlledNative"]["stringRepeat"].as_u64().unwrap() as usize);
    snapshot.value = JsonValue::Null;
    let length = snapshot.schema.len();
    let threshold = neutral["controlledNative"]["cancelCompleted"].as_u64().unwrap() as usize;
    let (_, body) = store::semio_format::unwrap_binary(&store::ArtifactPack::encode_pack(&snapshot)).unwrap();
    let spec = <JsonSnapshot as store::ArtifactPack>::record_spec().unwrap();
    let record = store::pack_rt::decode_document(&body, &spec, &store::PackDecodeOptions::default()).unwrap().0;
    let limits = SqliteDatabaseLimits::default();
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let mut observed = false;
        let mut proceed = |event: semio_framework_value::native_encoding::NativeEncodeProgress| {
            if event.total == length && event.completed >= threshold && event.completed < event.total {
                observed = true;
                false
            } else {
                true
            }
        };
        let mut native = semio_framework_value::NativeEncodeControl::new(limits.max_value_bytes, &mut proceed);
        let result = match encoding {
            SnapshotEncoding::Binary => store::pack_rt::encode_document_controlled(&spec, &record, &store::PackEncodeOptions::default(), &mut native).map(|_| ()).map_err(|error| error.to_string()),
            SnapshotEncoding::Text => semio_framework_dsl_record::print_controlled(&record, &spec, semio_framework_dsl_record::JoinMode::Document, limits.max_file_bytes, &mut native).map(|_| ()).map_err(|error| error.message),
        };
        assert!(result.is_err(), "{encoding:?}: actual writer must stop on interior cancellation");
        assert!(observed, "{encoding:?}: physical writer must visit interior owned text after projection setup");
    }
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_json_public_allocation_budget_is_independent_of_semantic_values() {
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_route,semio_framework_os_kernel::io::io_mechanism::io_run_with_snapshot_control};
    use semio_framework_os_kernel::io_schema::{IoPayload, SQLITE_SNAPSHOT};
    use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("json allocation declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
    let dialect: ArtifactDialect = ArtifactDialect { artifact_kind: "s.stdio.json".into(), standard: "rfc8259".into(), subset: "*".into() };
    let snapshot = fixture().1; let limits = SqliteDatabaseLimits::default();
    let route = io_route(&ArtifactDialect::from(SQLITE_SNAPSHOT), &dialect, 1).await.unwrap().value;
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let mut native_phase = false;
        let bytes = io_export_sqlite_snapshot(&dialect, &snapshot, encoding, limits, &mut |event| { native_phase |= matches!(event.phase, SqliteSnapshotPhase::DecodeNative | SqliteSnapshotPhase::EncodeNative); true }).await.unwrap().value;
        assert!(!native_phase, "typed full-file export must bypass native carrier production");
        let database = import_sqlite_database(&bytes, limits, &mut |_| true).unwrap();
        let semantic: usize = database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).map(|value| match value { SqliteValue::Null => 0, SqliteValue::Integer(_) | SqliteValue::Real(_) => 8, SqliteValue::Text(text) => text.len(), SqliteValue::Blob(bytes) => bytes.len() }).sum();
        use std::{io::Write, process::{Command, Stdio}};
        let script = "import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');let count=0;for(const{name}of db.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all())for(const row of db.query('SELECT * FROM \"'+name.replaceAll('\"','\"\"')+'\"').all())for(const value of Object.values(row))count+=value===null?0:typeof value==='number'||typeof value==='bigint'?8:typeof value==='string'?new TextEncoder().encode(value).length:value.byteLength;await Bun.write(Bun.stdout,String(count));db.close();";
        let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(&bytes).unwrap(); let oracle = child.wait_with_output().unwrap(); assert!(oracle.status.success(), "{}", String::from_utf8_lossy(&oracle.stderr)); assert_eq!(String::from_utf8(oracle.stdout).unwrap().parse::<usize>().unwrap(), semantic);
        let exact = SqliteDatabaseLimits { max_value_bytes: semantic, ..limits };
        let restored = io_import_sqlite_snapshot::<JsonSnapshot>(&dialect, &bytes, exact, &mut |event| { native_phase |= matches!(event.phase, SqliteSnapshotPhase::DecodeNative | SqliteSnapshotPhase::EncodeNative); true }).await.unwrap().value;
        assert_eq!(restored, snapshot); assert!(!native_phase); restored.retire_sqlite_snapshot();
        let output = io_run_with_snapshot_control(&route, IoPayload::Binary(bytes.clone()), exact, &mut |_| true).await.expect("semantic SQL bytes must not become the native backing ceiling").value;
        assert_eq!(matches!(output, IoPayload::Text(_)), encoding == SnapshotEncoding::Text);
        let refused = io_run_with_snapshot_control(&route, IoPayload::Binary(bytes), SqliteDatabaseLimits { max_allocation_bytes: 1, ..exact }, &mut |_| true).await;
        assert!(refused.is_err(), "actual public native output must enforce independent backing admission");
    }
    snapshot.retire_sqlite_snapshot();
}
