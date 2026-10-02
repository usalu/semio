use super::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot, sqlite_snapshot::*};
#[test]
fn sqlite_snapshot_semio_value_controlled_native_output(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let mut snapshot=fixture();snapshot.root=SemioValue::Str{value:"x".repeat(f["nativeOutputBytes"].as_u64().unwrap() as usize)};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let actual=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let expected=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(<SemioValueSnapshot as store::ArtifactDsl>::print_dsl(&snapshot))};assert!(actual==expected,"controlled value output differs from authored wire");
 for measured in [false,true]{let mut interior=false;let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{let cancel=event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=65536&&(if measured{event.total==0}else{event.total>65536&&event.completed<event.total});interior|=cancel;!cancel},SqliteDatabaseLimits::default())).unwrap_err();assert!(interior&&error.contains("cancel"),"{error}");}
 for limits in [SqliteDatabaseLimits{max_value_bytes:4096,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_file_bytes:4096,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_rows:0,..SqliteDatabaseLimits::default()}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());}
}
fn fixture() -> SemioValueSnapshot {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let entries = vec![SemioValueEntry { key: fixture["duplicateKey"].as_str().unwrap().into(), value: SemioValue::Int { lexeme: fixture["integerLexeme"].as_str().unwrap().into() } }, SemioValueEntry { key: fixture["duplicateKey"].as_str().unwrap().into(), value: SemioValue::Float { lexeme: fixture["floatLexeme"].as_str().unwrap().into() } }];
    SemioValueSnapshot { schema: fixture["schema"].as_str().unwrap().into(), root: SemioValue::List { items: vec![SemioValue::Null, SemioValue::Bool { value: true }, SemioValue::Bool { value: false }, SemioValue::Str { value: fixture["string"].as_str().unwrap().into() }, SemioValue::Bytes { value: vec![0,1,127,255] }, SemioValue::Map { entries }, SemioValue::Ref { id: ValueId::new("cycle") }] }, nodes: vec![SemioValueNode { id: ValueId::new("cycle"), value: SemioValue::Ref { id: ValueId::new("cycle") } }] }
}
fn project(value: &SemioValueSnapshot) -> SqliteDatabase { value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }
fn restore(database: &SqliteDatabase) -> Result<SemioValueSnapshot,String> { SemioValueSnapshot::from_sqlite_database(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) }
#[test]
fn sqlite_snapshot_semio_value_all_variants_lexemes_order_and_graph_cycle_roundtrip() {
    let snapshot=fixture(); let db=project(&snapshot); let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(); let restored=restore(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap(); assert_eq!(restored,snapshot);
    assert_eq!(<SemioValueSnapshot as store::ArtifactPack>::encode_pack(&restored),<SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snapshot)); assert_eq!(<SemioValueSnapshot as store::ArtifactDsl>::print_dsl(&restored),<SemioValueSnapshot as store::ArtifactDsl>::print_dsl(&snapshot)); assert_eq!(restore(&project(&SemioValueSnapshot::default())).unwrap(),SemioValueSnapshot::default());
}
#[test]
fn sqlite_snapshot_semio_value_rejects_dangling_structure_variant_shapes_and_limits() {
    let original=project(&fixture());
    for alteration in 0..6 { let mut db=original.clone(); match alteration { 0=>db.table_mut("semio_value_value").unwrap().rows[0].values[2]=SqliteValue::Integer(1),1=>db.table_mut("semio_value_list_element").unwrap().rows[0].values[3]=SqliteValue::Integer(1),2=>db.table_mut("semio_value_list_element").unwrap().rows[0].values[2]=SqliteValue::Integer(99),3=>db.table_mut("semio_value_node").unwrap().rows[0].values[4]=SqliteValue::Integer(999),4=>db.table_mut("semio_value_value").unwrap().rows.last_mut().unwrap().values[7]=SqliteValue::Integer(999),_=> { db.table_mut("semio_value_list_element").unwrap().rows.remove(0); } } assert!(restore(&db).is_err(),"alteration {alteration}"); }
    let mut invalid=fixture(); invalid.nodes.clear(); assert!(invalid.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
    let limits=SqliteDatabaseLimits { max_rows:2,max_value_bytes:2,..SqliteDatabaseLimits::default() }; assert!(fixture().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err()); assert!(SemioValueSnapshot::from_sqlite_database(&original,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err()); assert!(fixture().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
}
#[test]
fn sqlite_snapshot_semio_value_independent_sql_queries_edits_and_foreign_keys() {
    use std::io::Write; use std::process::{Command,Stdio};
    let snapshot=fixture(); let bytes=export_sqlite_database(&project(&snapshot),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
    let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(db.query(\"SELECT integer_lexeme FROM semio_value_value WHERE kind='int'\").get().integer_lexeme!=='-184467440737095516160001')throw Error('lexeme');if(db.query('SELECT member_key FROM semio_value_map_entry ORDER BY ordinal').all().map(r=>r.member_key).join(',')!=='same,same')throw Error('ordered duplicate keys');db.query(\"UPDATE semio_value_value SET string_value='independently edited' WHERE kind='str'\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));let restored=restore(&import_sqlite_database(&out.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()).unwrap();let mut expected=snapshot;if let SemioValue::List{items}=&mut expected.root {items[3]=SemioValue::Str{value:"independently edited".into()};}assert_eq!(restored,expected);
}

#[test]
fn sqlite_snapshot_semio_value_bare_owner_capability_native_payload_bridge(){let snapshot=fixture();let payload=store::os_io::IoPayload::Binary(<SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snapshot));let codec=store::ArtifactCodec::bare::<SemioValueSnapshot,crate::standards::v1::subsets::value::schema::mutations::SemioValueMutation>("s.stdio.semio.value");let provider=codec.snapshot_sqlite.expect("owner must publish typed SQLite capability");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<SemioValueSnapshot>()));let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"value".into()};let db=(provider.export)("s.stdio.semio.value",&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let db=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let actual=(provider.import)("s.stdio.semio.value",&dialect,db,SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(actual,payload);}

#[test]
fn sqlite_snapshot_semio_value_typed_exact_subset_dialect_and_document_guard(){let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let snapshot=fixture();let database=project(&snapshot);let dialect=|v:&serde_json::Value|store::os_io::ArtifactDialect{artifact_kind:v["artifactKind"].as_str().unwrap().into(),standard:v["standard"].as_str().unwrap().into(),subset:v["subset"].as_str().unwrap().into()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(&f["sqliteDialect"]),&database,&mut control).unwrap().diagnostics.is_empty());for invalid in f["invalidSqliteDialects"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(invalid),&database,&mut control).is_err());}let mut malformed=database;malformed.table_mut("semio_value_document").unwrap().rows[0].values[1]=SqliteValue::Text("other schema".into());assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(&f["sqliteDialect"]),&malformed,&mut control).is_err());}

#[test]
fn sqlite_snapshot_semio_value_bounded_native_text_binary_admission() {
    let snapshot=fixture();let dialect=store::os_io::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"value".into()};
    let provider=<SemioValueSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
        snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
        let actual=(provider.import)("Semio native admission",&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;
        let expected=match encoding{SnapshotEncoding::Text=>store::os_io::IoPayload::Text(<SemioValueSnapshot as store::ArtifactDsl>::print_dsl(&snapshot)),SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<SemioValueSnapshot as store::ArtifactPack>::encode_pack(&snapshot))};assert_eq!(actual,expected);
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:16,..SqliteDatabaseLimits::default()})).is_err());
        assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
    }
    let large= SemioValueSnapshot{schema:"x".repeat(100000),..snapshot};
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:4096,..SqliteDatabaseLimits::default()})).is_err());
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |p|p.phase!=SqliteSnapshotPhase::EncodeNative||p.completed==0,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_semio_value_explicit_controlled_native_owner(){
 use store::{ArtifactPack,ArtifactDsl};let f:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let expected=fixture();
 for payload in [store::os_io::IoPayload::Binary(expected.encode_pack()),store::os_io::IoPayload::Text(expected.print_dsl())]{
  let actual=SemioValueSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(actual,expected);assert!(SemioValueSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(SemioValueSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:f["nativeAdmission"]["tinyBudgetBytes"].as_u64().unwrap() as usize,..SqliteDatabaseLimits::default()})).is_err());assert!(SemioValueSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:2,..SqliteDatabaseLimits::default()})).is_err());
 }
 let count=f["nativeAdmission"]["collectionItems"].as_u64().unwrap() as usize;let canceled=f["nativeAdmission"]["cancelAfter"].as_u64().unwrap() as usize;let large=SemioValueSnapshot{schema:expected.schema,root:SemioValue::List{items:(0..count).map(|_|SemioValue::Map{entries:vec![SemioValueEntry{key:"same".into(),value:SemioValue::Bytes{value:vec![0,255]}}]}).collect()},nodes:Vec::new()};
 for payload in [store::os_io::IoPayload::Binary(large.encode_pack()),store::os_io::IoPayload::Text(large.print_dsl())]{let mut interior=false;let mut callback=|p:SqliteSnapshotProgress|{if p.phase==SqliteSnapshotPhase::DecodeNative&&p.total==count&&p.completed==canceled{interior=true;false}else{true}};assert!(SemioValueSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());assert!(interior);}
 let depth=f["nativeAdmission"]["maximumDepth"].as_u64().unwrap() as usize;let body=format!("[{},{}Z{},[]]",crate::standards::v1::subsets::value::schema::diff::enc_str(STDIO_SEMIOVALUE_DOCUMENT_SCHEMA),"L[".repeat(depth+1),"]".repeat(depth+1));let envelope=store::semio_format::SemioEnvelope::from_envelope_id(STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1).unwrap();let payload=store::os_io::IoPayload::Text(store::semio_format::wrap_text(&envelope,&body));assert!(SemioValueSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_semio_value_recursive_retirement_uses_the_owner_cursor(){
 use std::io::Write;use std::process::{Command,Stdio};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let depth=fixture["retirementDepth"].as_u64().unwrap() as usize;
 let mut root=SemioValue::Null;for _ in 0..depth{root=SemioValue::List{items:vec![root]};}
 let snapshot=SemioValueSnapshot{schema:"retirement".into(),root,nodes:Vec::new()};let database=project(&snapshot);let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const n=Number(process.argv[1]);if(d.query('SELECT count(*) AS n FROM semio_value_value').get().n!==n+1||d.query('SELECT count(*) AS n FROM semio_value_list_element').get().n!==n||d.query('PRAGMA foreign_key_check').all().length)throw Error('typed ownership topology');d.close();";
 let mut child=Command::new("bun").args(["-e",script,&depth.to_string()]).stdin(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();assert!(child.wait().unwrap().success());std::thread::Builder::new().stack_size(fixture["retirementStackBytes"].as_u64().unwrap() as usize).spawn(move||snapshot.retire_sqlite_snapshot()).unwrap().join().unwrap();
}

#[test]
fn sqlite_snapshot_semio_value_partial_reconstruction_retires_recursive_owned_fields(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let depth=fixture["retirementDepth"].as_u64().unwrap() as usize;let mut root=SemioValue::Null;for _ in 0..depth{root=SemioValue::List{items:vec![root]}}let snapshot=SemioValueSnapshot{schema:"partial retirement".into(),root,nodes:Vec::new()};let database=project(&snapshot);drop(crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned::new(snapshot));let cancel_after=4*depth-fixture["retirementCancelRemaining"].as_u64().unwrap() as usize;
 std::thread::Builder::new().stack_size(fixture["retirementStackBytes"].as_u64().unwrap() as usize).spawn(move||{let mut reached=false;let error=SemioValueSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |progress|{let canceled=progress.phase==SqliteSnapshotPhase::ReconstructSnapshot&&progress.total==0&&progress.completed>=cancel_after;reached|=canceled;!canceled},SqliteDatabaseLimits::default())).unwrap_err();assert!(reached&&error.contains("cancel"),"{error}");}).unwrap().join().unwrap();
}
