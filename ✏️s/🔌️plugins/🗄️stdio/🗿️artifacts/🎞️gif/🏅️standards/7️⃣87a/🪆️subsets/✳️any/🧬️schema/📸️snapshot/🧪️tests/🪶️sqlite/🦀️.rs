use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> GifSnapshot { semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }

#[test]
fn sqlite_snapshot_gif87_independent_dimensions_and_index_sequence(){
 let owned:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🫳️ownership/🔣️.json")).unwrap();let mut snapshot=fixture();let image=&mut snapshot.images[0];image.width=owned["width"].as_u64().unwrap() as u32;image.height=owned["height"].as_u64().unwrap() as u32;image.indices=owned["indices"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect();
 let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
}

#[test]
fn sqlite_snapshot_gif87_controlled_output_reaches_literal_string_interior(){
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
fn sqlite_snapshot_gif87_entity_validation_honors_bounded_cancel_before_palette_ownership(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🚦️control.json")).unwrap();let count=cases["paletteColors"].as_u64().unwrap() as usize;let snapshot=GifSnapshot{gct:Some(GifColorTable{sorted:false,colors:vec![GifRgb{r:1,g:2,b:3};count]}),..GifSnapshot::default()};let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let mut reached=false;
 let result=GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |event|{if event.completed==cases["cancelCompleted"].as_u64().unwrap() as usize&&event.total==count{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(reached,"entity scan must offer cancellation before palette ownership");assert!(result.is_err());
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_gif87_owned_io_preserves_full_source_metadata() {
 use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 crate::register_sqlite_test_declaration();
 let dialect:ArtifactDialect=crate::GIF_87A_DIALECT.into();let mut snapshot=fixture();snapshot.schema="GIF87 vollständiger Snapshot 世界".into();let mut phases=Vec::new();
 let bytes=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;
 assert_eq!(io_import_sqlite_snapshot::<GifSnapshot>(&dialect,&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);
 assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
 let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(database.table("semio_snapshot").unwrap().rows[0].text(2).unwrap(),"87a");
}

#[test]
fn sqlite_snapshot_gif87_preserves_indexed_images_optional_palettes_and_full_widths() {
 let snapshot = fixture();
 let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
 let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
 let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
 assert_eq!(GifSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
 let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
 assert_eq!(oracle, serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());
 assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
 for value in [GifSnapshot::default(), GifSnapshot {gct: Some(GifColorTable::default()), ..GifSnapshot::default()}] { let database=value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(); assert_eq!(GifSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(),value); }
 let mut invalid=database.clone(); invalid.table_mut("gif87_pixel").unwrap().rows[0].values[1]=SqliteValue::Integer(999);
 assert!(GifSnapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).is_err());
 let limits=SqliteDatabaseLimits {max_rows:1,..SqliteDatabaseLimits::default()}; assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,limits)).is_err());
 assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_gif87_independent_palette_and_pixel_edits() {
 use std::{io::Write,process::{Command,Stdio}};
 let mut snapshot=fixture(); let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap();
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_| true).unwrap();
 let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const p=db.query('SELECT p.color_index,c.red FROM gif87_pixel p JOIN gif87_image i ON i.id=p.image_id JOIN gif87_global_color c ON c.ordinal=p.color_index WHERE i.ordinal=0 AND p.ordinal=0').get();if(p.color_index!==0||p.red!==255)throw Error('semantic join');db.query('UPDATE gif87_global_color SET red=42 WHERE ordinal=0').run();db.query('UPDATE gif87_pixel SET color_index=1 WHERE image_id=1 AND ordinal=0').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(&bytes).unwrap(); let output=child.wait_with_output().unwrap(); assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 let database=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_| true).unwrap(); snapshot.gct.as_mut().unwrap().colors[0].r=42;snapshot.images[0].indices[0]=1;
 assert_eq!(GifSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_| true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_gif87_actual_erased_declaration_preserves_owned_state(){
 use store::{ArtifactSqliteSnapshot,ArtifactDsl};use semio_framework_os_kernel::{io_schema::ArtifactDialect,sqlite_snapshot::SnapshotEncoding};
 crate::register_sqlite_test_declaration();let codec=store::document_codec(crate::STDIO_GIF_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.unwrap();let dialect:ArtifactDialect=crate::GIF_87A_DIALECT.into();let mut snapshot=fixture();snapshot.schema="owned GIF87 世界\0".into();snapshot.width=u32::MAX;
 let mut intermediate=snapshot.clone();intermediate.images[0].width=0;intermediate.images[0].height=u32::MAX;intermediate.images[0].indices=vec![0,255,1];for snapshot in [snapshot,GifSnapshot{schema:"empty palette".into(),width:u32::MAX,gct:Some(GifColorTable::default()),..GifSnapshot::default()},intermediate]{let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=(provider.import)(&codec.schema,&dialect,expected.clone(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let restored=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(restored,expected);}}
}
#[test]
fn sqlite_snapshot_gif87_controlled_owned_record_decoder(){
 use store::{ArtifactSqliteSnapshot,ArtifactDsl};use semio_framework_os_kernel::io_schema::IoPayload;
 let mut snapshot=fixture();snapshot.schema="controlled GIF87 世界\0".into();let record=snapshot.__dsl_to_record();let spec=GifSnapshot::__dsl_spec();let body=semio_framework_dsl_record::print(&record,&spec,semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<GifSnapshot as ArtifactDsl>::envelope_id(),store::semio_format::Component::Dsl,1).unwrap();let payload=IoPayload::Text(store::semio_format::wrap_text(&envelope,&body));
 assert_eq!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);assert!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_gif87_native_controls_reach_interior_owned_construction(){
 use store::{ArtifactDsl,ArtifactPack};use semio_framework_os_kernel::{io_schema::IoPayload,sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase}};
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧫️fixtures/🪶️sqlite/🚦️control.json")).unwrap();let count=cases["nativeItems"].as_u64().unwrap() as usize;let mut snapshot=fixture();snapshot.images=vec![snapshot.images[0].clone();count];
 for payload in [IoPayload::Text(snapshot.print_dsl()),IoPayload::Binary(snapshot.encode_pack())]{
  assert_eq!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);
  let mut reached=false;let result=GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.total==count&&event.completed==cases["cancelCompleted"].as_u64().unwrap() as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default()));assert!(reached,"typed collection construction must checkpoint inside the known workload");assert!(result.is_err());
  assert!(GifSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:cases["nativeValueBudget"].as_u64().unwrap() as usize,..SqliteDatabaseLimits::default()})).is_err());
 }
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:cases["nativeEncodingBudget"].as_u64().unwrap() as usize,..SqliteDatabaseLimits::default()})).is_err());assert!(snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());}
}

#[test]
fn sqlite_snapshot_gif87_declared_record_grammar_covers_neutral_full_model(){
 use store::ArtifactDsl;
 let grammar=semio_framework_dsl::parse_grammar(include_str!("../../📝️text/📖️.grammar.semio")).unwrap();let recognizer=semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");let mut snapshot=fixture();snapshot.schema="schema 世界\0".into();
 for snapshot in [snapshot,GifSnapshot{schema:"empty optional palette".into(),gct:Some(GifColorTable::default()),..GifSnapshot::default()}]{let text=snapshot.print_dsl();let(envelope,body)=store::semio_format::split_text_preamble(&text).unwrap();assert!(recognizer.recognize(&format!("{}\n{body}",envelope.envelope_id())).unwrap(),"full neutral model grammar did not recognize its owned record");assert_eq!(GifSnapshot::parse_dsl(&text).unwrap(),snapshot);}
}
