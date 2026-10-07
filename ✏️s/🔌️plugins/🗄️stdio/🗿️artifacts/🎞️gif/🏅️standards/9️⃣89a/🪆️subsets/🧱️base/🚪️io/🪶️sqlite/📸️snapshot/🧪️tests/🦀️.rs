use crate::standards::v89a::subsets::any::io::sqlite::snapshot::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> GifSnapshot { semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }

#[test]
fn sqlite_snapshot_gif89_independent_dimensions_and_index_sequence(){
 let owned:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🫳️ownership/🔣️.json")).unwrap();let mut snapshot=fixture();let image=&mut snapshot.frames[0];image.width=owned["width"].as_u64().unwrap() as u32;image.height=owned["height"].as_u64().unwrap() as u32;image.indices=owned["indices"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
}

#[test]
fn sqlite_snapshot_gif89_controlled_output_reaches_literal_string_interior(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🚦️control.json")).unwrap();
 let mut snapshot=fixture();snapshot.schema="世界\0".repeat(plan["largeTextBytes"].as_u64().unwrap() as usize);
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
  assert_eq!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
  let mut reached=false;let mut callback=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==snapshot.schema.len()&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true}};
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default())).is_err());assert!(reached);
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:128,..SqliteDatabaseLimits::default()})).is_err());
 }
}

#[test]
fn sqlite_snapshot_gif89_large_plain_text_cancel_precedes_owned_copy(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🚦️control.json")).unwrap();let mut snapshot=fixture();snapshot.comments.clear();snapshot.frames[0].plain_text.as_mut().unwrap().text="x".repeat(cases["largeTextBytes"].as_u64().unwrap() as usize);let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut reached=false;
 let result=GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |event|{if event.completed==1&&event.total==0{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(reached,"large owned text copy must offer a cancellation checkpoint");assert!(result.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_gif89_owned_io_retains_animation_and_extensions_without_wire_normalization(){
 use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 crate::register_sqlite_test_declaration();
 let dialect:ArtifactDialect=crate::GIF_89A_DIALECT.into();let mut snapshot=fixture();snapshot.schema="GIF89 vollständiger Snapshot 世界".into();let mut phases=Vec::new();
 let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;
 assert_eq!(io_import_sqlite_snapshot::<GifSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);
 assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
 let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(database.table("semio_snapshot").unwrap().rows[0].text(2).unwrap(),"89a");
}

#[test]
fn sqlite_snapshot_gif89_full_animation_and_extension_entities_roundtrip() {
 let snapshot=fixture(); let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(); let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_| true).unwrap();let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_| true).unwrap();
 assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
 let oracle:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();assert_eq!(oracle,serde_json::from_str::<serde_json::Value>(include_str!("../🧫️fixtures/🔣️.json")).unwrap());
 assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value,SqliteValue::Blob(_))));
 for disposal in [GifDisposal::Unspecified,GifDisposal::DoNotDispose,GifDisposal::RestoreToBackground,GifDisposal::RestoreToPrevious] {let mut value=snapshot.clone();value.frames[0].disposal=disposal;value.loop_count=Some(0);let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(),value);}
 for value in [GifSnapshot::default(),GifSnapshot {gct:Some(GifColorTable::default()),..GifSnapshot::default()}] {let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(),value);}
 let mut invalid=database.clone();invalid.table_mut("gif89_plain_text").unwrap().rows[0].values[0]=SqliteValue::Integer(999);invalid.table_mut("gif89_plain_text").unwrap().rows[0].rowid=999;assert!(GifSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).is_err());
 let limits=SqliteDatabaseLimits {max_rows:1,..SqliteDatabaseLimits::default()};assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,limits)).is_err());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_gif89_independent_frame_text_and_application_edits() {
 use std::{io::Write,process::{Command,Stdio}};
 let mut snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_| true).unwrap();
 let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('SELECT f.delay_centiseconds,t.text,a.authentication_1,b.value FROM gif89_frame f JOIN gif89_plain_text t ON t.id=f.id CROSS JOIN gif89_application a JOIN gif89_application_byte b ON b.application_id=a.id WHERE f.ordinal=0 AND a.ordinal=0 AND b.ordinal=1').get();if(row.delay_centiseconds!==65535||row.authentication_1!==255||row.value!==255||!row.text.includes('世界'))throw Error('semantic join');db.query('UPDATE gif89_frame SET disposal=? WHERE ordinal=0').run('restore_to_background');db.query('UPDATE gif89_plain_text SET text=? WHERE id=1').run('SQL 世界');db.query('UPDATE gif89_application_byte SET value=42 WHERE application_id=1 AND ordinal=1').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_| true).unwrap();snapshot.frames[0].disposal=GifDisposal::RestoreToBackground;snapshot.frames[0].plain_text.as_mut().unwrap().text="SQL 世界".into();snapshot.app_extensions[0].data[1]=42;
 assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_gif89_actual_erased_declaration_preserves_owned_state(){
 use store::{ArtifactSqliteSnapshot,ArtifactDsl};use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding};
 crate::register_sqlite_test_declaration();let codec=store::document_codec(crate::STDIO_GIF89A_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.unwrap();let dialect:ArtifactDialect=crate::GIF_89A_DIALECT.into();let mut snapshot=fixture();snapshot.schema="owned GIF89 世界\0".into();snapshot.width=u32::MAX;
 let mut intermediate=snapshot.clone();intermediate.frames[0].width=0;intermediate.frames[0].height=u32::MAX;intermediate.frames[0].indices=vec![0,255,1];for snapshot in [snapshot,GifSnapshot{schema:"empty palette".into(),width:u32::MAX,gct:Some(GifColorTable::default()),..GifSnapshot::default()},intermediate]{let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=(provider.import)(&codec.schema,&dialect,expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(restored,expected);}}
}
#[test]
fn sqlite_snapshot_gif89_controlled_owned_record_decoder(){
 use store::{ArtifactSqliteSnapshot,ArtifactDsl};use semio_framework_os_kernel::io_schema::IoPayload;
 let mut snapshot=fixture();snapshot.schema="controlled GIF89 世界\0".into();let record=snapshot.__dsl_to_record();let spec=GifSnapshot::__dsl_spec();let body=semio_framework_dsl_record::print(&record,&spec,semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<GifSnapshot as ArtifactDsl>::envelope_id(),store::semio_format::Component::Dsl,1).unwrap();let payload=IoPayload::Text(store::semio_format::wrap_text(&envelope,&body));
 assert_eq!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);assert!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_gif89_native_controls_reach_interior_owned_construction(){
 use store::{ArtifactDsl,ArtifactPack};use semio_framework_os_kernel::{io_schema::IoPayload,sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase}};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🚦️control.json")).unwrap();let count=cases["nativeItems"].as_u64().unwrap() as usize;let mut snapshot=fixture();snapshot.frames=vec![snapshot.frames[0].clone();count];
 for payload in [IoPayload::Text(snapshot.print_dsl()),IoPayload::Binary(snapshot.encode_pack())]{
  assert_eq!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
  let mut reached=false;let result=GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.total==count&&event.completed==cases["cancelCompleted"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(reached,"typed collection construction must checkpoint inside the known workload");assert!(result.is_err());
  assert!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:cases["nativeValueBudget"].as_u64().unwrap() as usize,..SqliteDatabaseLimits::default()})).is_err());
 }
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:cases["nativeEncodingBudget"].as_u64().unwrap() as usize,..SqliteDatabaseLimits::default()})).is_err());assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());}
}

#[test]
fn sqlite_snapshot_gif89_declared_record_grammar_covers_neutral_full_model(){
 use store::ArtifactDsl;
 let grammar=semio_framework_dsl::parse_grammar(include_str!("../../../📝️text/📸️snapshot/📖️.grammar.semio")).unwrap();let recognizer=semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");let mut snapshot=fixture();snapshot.schema="schema 世界\0".into();
 for snapshot in [snapshot,GifSnapshot{schema:"empty optional palette".into(),gct:Some(GifColorTable::default()),..GifSnapshot::default()}]{let text=snapshot.print_dsl();let(envelope,body)=store::semio_format::split_text_preamble(&text).unwrap();assert!(recognizer.recognize(&format!("{}\n{body}",envelope.envelope_id())).unwrap(),"full neutral model grammar did not recognize its owned record");assert_eq!(GifSnapshot::parse_dsl(&text).unwrap(),snapshot);}
}
