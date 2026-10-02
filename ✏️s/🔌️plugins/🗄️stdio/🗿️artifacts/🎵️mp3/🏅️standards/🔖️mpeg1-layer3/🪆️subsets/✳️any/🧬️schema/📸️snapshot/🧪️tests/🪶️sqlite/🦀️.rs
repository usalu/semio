use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database,import_sqlite_database,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteValue},ArtifactSqliteSnapshot};

fn fixture()->Mp3Snapshot{pack::json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
#[test]
fn sqlite_snapshot_mp3_owned_controlled_native_decoder_admits_and_cancels_physical_bytes(){
 use semio_framework_os_kernel::{io::IoPayload,sqlite_snapshot::SqliteSnapshotPhase};
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🎛️control.json")).unwrap();
 let limits=SqliteDatabaseLimits::default();let snapshot=fixture();
 for payload in[IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))]{
  assert_eq!(Mp3Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
  assert!(Mp3Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
  for limits in[SqliteDatabaseLimits{max_file_bytes:1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(Mp3Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}
 }
 let mut snapshot=snapshot;snapshot.frames[0].payload=vec![255;neutral["largeOctetBytes"].as_u64().unwrap()as usize];
 for payload in[IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))]{
  let mut reached=false;assert!(Mp3Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=neutral["cancelAt"].as_u64().unwrap()as usize{reached=true;false}else{true}},limits)).is_err());assert!(reached);
 }
}
#[test]
fn sqlite_snapshot_mp3_authored_grammar_and_protocol_admit_complete_logical_records(){let grammar=dsl::parse_grammar(include_str!("../../📝️text/📖️.grammar.semio")).unwrap();let recognizer=dsl::Recognizer::compile(&grammar);for snapshot in[fixture(),Mp3Snapshot::default(),crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🔊️.mp3")).unwrap()]{let text=<Mp3Snapshot as store::ArtifactDsl>::print_dsl(&snapshot);let (_,body)=store::semio_format::split_text_preamble(&text).unwrap();assert!(recognizer.recognize(body).unwrap(),"{body}");}let protocol=dsl::parse_protocol(include_str!("../../💾️binary/📡️.protocol.semio")).unwrap();assert_eq!(protocol.schema,"stdio.mp3");assert_eq!(protocol.version,1);}
#[test]
fn sqlite_snapshot_mp3_native_factory_declares_actual_structural_hash(){let value:serde_json::Value=serde_json::from_str(crate::ARTIFACT_DEFINITION_SCHEMA).unwrap();let codec=(crate::native_codecs()[0].codec)();assert_eq!(semio_framework_hash::hex_lower(&codec.pack_schema_hash),value["codecs"][0]["native_factory"]["pack_schema_hash"].as_str().unwrap());}
#[test]
fn sqlite_snapshot_mp3_shipped_demo_assets_match_owned_native_source(){let snapshot=crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🔊️.mp3")).unwrap();assert_eq!(<Mp3Snapshot as store::ArtifactDsl>::parse_dsl(include_str!("../../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio")).unwrap(),snapshot);assert_eq!(<Mp3Snapshot as store::ArtifactPack>::decode_pack(include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio")).unwrap(),snapshot);}
#[test]
fn sqlite_snapshot_mp3_actual_erased_capability_preserves_binary_and_text(){
 use semio_framework_os_kernel::{io::{ArtifactDialect,IoPayload},sqlite_snapshot::SnapshotEncoding};
 let snapshot=fixture();let codec=(crate::native_codecs()[0].codec)();let provider=codec.snapshot_sqlite.as_ref().unwrap();let dialect:ArtifactDialect=crate::MP3_DIALECT.into();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(<Mp3Snapshot as store::ArtifactPack>::encode_pack(&snapshot)),SnapshotEncoding::Text=>IoPayload::Text(<Mp3Snapshot as store::ArtifactDsl>::print_dsl(&snapshot))};let database=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).expect("full owned snapshot export").value;assert_eq!(Mp3Snapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),snapshot);let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let restored=(provider.import)(&codec.schema,&dialect,import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).expect("full owned snapshot import").value;assert_eq!(match restored{IoPayload::Binary(bytes)=><Mp3Snapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=><Mp3Snapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()},snapshot);}
}

#[test]
fn sqlite_snapshot_mp3_owned_encoding_admits_exact_physical_bounds_and_controls_owned_copies(){
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let snapshot=fixture();for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let output=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let physical=match &output{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};
  assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:physical,..SqliteDatabaseLimits::default()})).unwrap(),output);
  if encoding==SnapshotEncoding::Binary{assert!(physical<1024);assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1024,..SqliteDatabaseLimits::default()})).is_ok());}
  let mut phases=Vec::new();let error=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut|event|{phases.push(event.phase);true},SqliteDatabaseLimits{max_file_bytes:physical-1,..SqliteDatabaseLimits::default()})).unwrap_err();assert!(error.contains("limit")||error.contains("bound"),"{error}");assert!(phases.contains(&SqliteSnapshotPhase::EncodeNative));
  let mut large=snapshot.clone();large.frames[0].payload=vec![255;131073];let mut reached=false;assert!(large.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut|event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>0{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
 }
}
fn roundtrip(snapshot:&Mp3Snapshot)->Mp3Snapshot{let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();Mp3Snapshot::from_sqlite_database(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}

#[test]
fn sqlite_snapshot_mp3_complete_neutral_corpus_and_optional_empty_tags(){
 let snapshot=fixture();assert_eq!(roundtrip(&snapshot),snapshot);let oracle:serde_json::Value=serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&snapshot))).unwrap();assert_eq!(oracle,serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());
 for present in [false,true]{let snapshot=Mp3Snapshot{schema:"".into(),id3v2:present.then_some(Id3v2Tag{major_version:0,minor_version:255,flags:0,frames:Vec::new()}),frames:Vec::new(),id3v1:present.then_some(Id3v1Tag{raw:Vec::new()})};assert_eq!(roundtrip(&snapshot),snapshot);}
}

#[test]
fn sqlite_snapshot_mp3_independent_sqlite_domain_joins_and_edits(){
 use std::{io::Write,process::{Command,Stdio}};
 let mut expected=fixture();let database=expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('SELECT t.major_version,f.flags,o.octet FROM mp3_id3v2_tag t JOIN mp3_id3v2_frame f ON f.tag_id=t.id JOIN mp3_id3v2_frame_octet o ON o.frame_id=f.id WHERE f.ordinal=0 AND o.ordinal=1').get();if(row.major_version!==255||row.flags!==65535||row.octet!==255)throw Error('tag domain');if(db.query('SELECT layer FROM mp3_audio_frame WHERE ordinal=1').get().layer!==255)throw Error('audio header');db.run(\"UPDATE mp3_id3v2_frame SET frame_identifier='edited 世界' WHERE ordinal=0\");db.run('UPDATE mp3_audio_frame SET emphasis=42 WHERE ordinal=1');db.run('UPDATE mp3_audio_payload_octet SET octet=17 WHERE frame_id=1 AND ordinal=2');db.run('UPDATE mp3_id3v1_octet SET octet=99 WHERE ordinal=3');await Bun.write(Bun.stdout,db.serialize());db.close();";
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));expected.id3v2.as_mut().unwrap().frames[0].id="edited 世界".into();expected.frames[1].header.emphasis=42;expected.frames[0].payload[2]=17;expected.id3v1.as_mut().unwrap().raw[3]=99;
 assert_eq!(Mp3Snapshot::from_sqlite_database(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap(),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap(),expected);
}

#[test]
fn sqlite_snapshot_mp3_refuses_malformed_ownership_widths_and_bounds_work(){
 let snapshot=fixture();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 for(table,row,column,value)in [("mp3_id3v2_frame",0,1,SqliteValue::Integer(99)),("mp3_id3v2_frame",0,2,SqliteValue::Integer(99)),("mp3_id3v2_frame",0,4,SqliteValue::Integer(65536)),("mp3_audio_frame",0,3,SqliteValue::Integer(256)),("mp3_audio_frame",0,5,SqliteValue::Integer(2)),("mp3_audio_payload_octet",0,1,SqliteValue::Integer(99)),("mp3_audio_payload_octet",0,2,SqliteValue::Integer(99)),("mp3_id3v1_octet",0,3,SqliteValue::Integer(-1))]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows[row].values[column]=value;assert!(Mp3Snapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"{table}");}
 for table in ["mp3_id3v2_tag","mp3_id3v1_tag"]{let mut invalid=database.clone();invalid.table_mut(table).unwrap().rows.clear();assert!(Mp3Snapshot::from_sqlite_database(&invalid,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
 assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|false,SqliteDatabaseLimits::default())).is_err());assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:1,..SqliteDatabaseLimits::default()})).is_err());assert!(Mp3Snapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:0,..SqliteDatabaseLimits::default()})).is_err());let mut large=snapshot;large.frames[0].payload=vec![255;2000];let mut reached=false;assert!(large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |event|{if event.completed>=256{reached=true;false}else{true}},SqliteDatabaseLimits::default())).is_err());assert!(reached);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_mp3_actual_declaration_preserves_non_wire_owned_fields(){
 use semio_framework_os_kernel::{io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}},sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase}};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("MP3 SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();let snapshot=fixture();let dialect:ArtifactDialect=crate::MP3_DIALECT.into();let mut phases=Vec::new();let output=io_export_sqlite_snapshot(&dialect,&snapshot,SnapshotEncoding::Binary,SqliteDatabaseLimits::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap();assert_eq!(io_import_sqlite_snapshot::<Mp3Snapshot>(&dialect,&output.value,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
}

#[test]
fn sqlite_snapshot_mp3_genuine_output_admits_exact_row_and_file_frontiers(){
 use store::{ArtifactSqliteSnapshot as _,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl,SnapshotEncoding,export_sqlite_database}};
 use std::{io::Write,process::{Command,Stdio}};
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let rows=expected.tables.iter().map(|table|table.rows.len()).sum::<usize>();assert!(rows>0);
 let bytes=export_sqlite_database(&expected,limits,&mut |_|true).unwrap();
 let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();const rows=tables.reduce((n,t)=>n+Number(db.query('SELECT COUNT(*) AS count FROM "'+t.name.replaceAll('"','""')+'"').get().count),0);await Bun.write(Bun.stdout,String(rows));db.close();"#;
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(String::from_utf8(output.stdout).unwrap().parse::<usize>().unwrap(),rows);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..limits})).unwrap();
  let physical=match &payload{store::io::IoPayload::Binary(value)=>value.len(),store::io::IoPayload::Text(value)=>value.len()};assert!(physical>0);
  let repeated=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,max_file_bytes:physical,..limits})).unwrap();assert_eq!(repeated,payload);
  let restored=Mp3Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
  for restricted in[SqliteDatabaseLimits{max_rows:rows-1,..limits},SqliteDatabaseLimits{max_file_bytes:physical-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err(),"{encoding:?}: {restricted:?}");}
  assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
 }
}
