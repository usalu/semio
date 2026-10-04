use super::Puzzle2dSnapshot;
fn fixture()->Puzzle2dSnapshot{let laws:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();semio_framework_pack_json::from_json_str(&laws["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn words(snapshot:&Puzzle2dSnapshot)->Vec<u64>{
 let mut values=vec![snapshot.camera.x,snapshot.camera.y,snapshot.camera.zoom];
 for row in &snapshot.nodes{values.extend([row.x,row.y]);values.extend(row.radius);values.extend(row.width);values.extend(row.height);values.extend(row.scale);for h in &row.handles{values.push(h.angle);values.extend(h.radius);values.extend(h.scale);}}
 for row in &snapshot.edges{values.extend([row.gap,row.shift,row.rise,row.rotation,row.turn,row.tilt,row.x,row.y]);}
 for row in &snapshot.target_regions{values.extend([row.x,row.y,row.width,row.height]);}
 if let Some(catalogs)=&snapshot.meta.kind_catalogs{for row in &catalogs.nodes{for h in &row.handles{values.push(h.angle);values.extend(h.t);values.extend(h.radius);}}}
 values.into_iter().map(f64::to_bits).collect()
}
fn fill_words(snapshot:&mut Puzzle2dSnapshot,value:f64){
 snapshot.camera.x=value;snapshot.camera.y=value;snapshot.camera.zoom=value;
 for row in &mut snapshot.nodes{row.x=value;row.y=value;if row.radius.is_some(){row.radius=Some(value)}if row.width.is_some(){row.width=Some(value)}if row.height.is_some(){row.height=Some(value)}if row.scale.is_some(){row.scale=Some(value)}for h in &mut row.handles{h.angle=value;if h.radius.is_some(){h.radius=Some(value)}if h.scale.is_some(){h.scale=Some(value)}}}
 for row in &mut snapshot.edges{row.gap=value;row.shift=value;row.rise=value;row.rotation=value;row.turn=value;row.tilt=value;row.x=value;row.y=value;}
 for row in &mut snapshot.target_regions{row.x=value;row.y=value;row.width=value;row.height=value;}
 if let Some(catalogs)=&mut snapshot.meta.kind_catalogs{for row in &mut catalogs.nodes{for h in &mut row.handles{h.angle=value;if h.t.is_some(){h.t=Some(value)}if h.radius.is_some(){h.radius=Some(value)}}}}
}
#[test]
fn sqlite_snapshot_puzzle2d_typed_native_codec_exposes_owned_semantic_capability(){
 assert!(store::ArtifactCodec::bare::<Puzzle2dSnapshot,crate::Puzzle2dMutation>(crate::PUZZLE_2D_SCHEMA).snapshot_sqlite.is_some(),"Puzzle 2D's actual typed native codec requires owned relational SQLite");
}
#[test]
fn sqlite_snapshot_puzzle2d_complete_neutral_fields_match_independent_serde_and_both_native_directions(){
 let expected=fixture();let value:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();let independent:Puzzle2dSnapshot=serde_json::from_value(value["snapshot"].clone()).unwrap();assert_eq!(expected,independent);assert_eq!(words(&expected).len(),43);
 let binary=<Puzzle2dSnapshot as store::ArtifactPack>::encode_pack_with(&expected,&store::PackEncodeOptions::default()).unwrap();assert_eq!(<Puzzle2dSnapshot as store::ArtifactPack>::decode_pack_with(&binary,&store::PackDecodeOptions::default()).unwrap(),expected);
 let text=<Puzzle2dSnapshot as store::ArtifactDsl>::print_dsl(&expected);assert_eq!(<Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),expected);
}
#[test]
fn sqlite_snapshot_puzzle2d_every_board_and_catalog_number_preserves_exact_native_words(){
 let laws:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();for hex in laws["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=fixture();fill_words(&mut expected,f64::from_bits(bits));
 let binary=<Puzzle2dSnapshot as store::ArtifactPack>::encode_pack_with(&expected,&store::PackEncodeOptions::default()).unwrap();assert_eq!(words(&<Puzzle2dSnapshot as store::ArtifactPack>::decode_pack_with(&binary,&store::PackDecodeOptions::default()).unwrap()),vec![bits;43],"Pack {hex}");
 let text=<Puzzle2dSnapshot as store::ArtifactDsl>::print_dsl(&expected);assert_eq!(words(&<Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap()),vec![bits;43],"Text {hex}");}
}

#[test]
fn sqlite_snapshot_puzzle2d_play_codec_exposes_the_same_owned_persisted_snapshot(){
 assert!(store::ArtifactCodec::bare::<crate::Puzzle2dPlaySnapshot,crate::Puzzle2dMutation>(crate::PUZZLE_2D_SCHEMA).snapshot_sqlite.is_some(),"Puzzle 2D play snapshot requires the same owned semantic SQLite capability");
}

use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
fn laws()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn database(value:&Puzzle2dSnapshot)->SqliteDatabase{value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn restore(value:&SqliteDatabase)->Puzzle2dSnapshot{Puzzle2dSnapshot::from_sqlite_database(value,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap()}
fn file(value:&Puzzle2dSnapshot)->Vec<u8>{export_sqlite_database(&database(value),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()}
fn payload(value:&Puzzle2dSnapshot,encoding:SnapshotEncoding)->store::io_schema::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(value)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(value))}}
fn dialect()->store::io_schema::ArtifactDialect{store::io_schema::ArtifactDialect{artifact_kind:"s.puzzle.puzzle2d".into(),standard:"1".into(),subset:"*".into()}}
fn oracle(script:&str,input:&[u8])->Vec<u8>{use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(input).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));output.stdout}
#[test]
fn sqlite_snapshot_puzzle2d_full_native_relational_fields_survive_physical_files(){
 let expected=fixture();let db=database(&expected);assert_eq!(db.tables.len(),20);assert_eq!(db.tables.iter().map(|t|t.rows.len()).sum::<usize>(),42);assert_eq!(restore(&db),expected);assert_eq!(restore(&import_sqlite_database(&file(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}
#[test]
fn sqlite_snapshot_puzzle2d_every_board_catalog_word_reaches_both_erased_outputs(){
 let codec=store::ArtifactCodec::bare::<Puzzle2dSnapshot,crate::Puzzle2dMutation>(crate::PUZZLE_2D_SCHEMA);let owner=codec.snapshot_sqlite.unwrap();for hex in laws()["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=fixture();fill_words(&mut expected,f64::from_bits(bits));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let d=(owner.export)(&codec.schema,&dialect(),&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(words(&restore(&d)),vec![bits;43]);let output=(owner.import)(&codec.schema,&dialect(),d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let actual=Puzzle2dSnapshot::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(words(&actual),vec![bits;43]);assert_eq!(actual.meta.manifest_id,expected.meta.manifest_id);}}
}
#[test]
fn sqlite_snapshot_puzzle2d_independent_queries_and_surrogate_edits_retain_literal_domains(){
 let mut expected=fixture();expected.nodes[1].id=expected.nodes[0].id.clone();let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const nodes=db.query('SELECT node_id,anchor,radius_bits FROM puzzle2d_node ORDER BY ordinal').all();if(nodes.length!==2||nodes[0].node_id!==nodes[1].node_id)throw Error('ordered duplicate node domains');db.run('UPDATE puzzle2d_document SET schema=?',['new\0😀']);db.run('UPDATE puzzle2d_node SET id=id+90');db.run('UPDATE puzzle2d_handle SET node_id=node_id+90,id=id+90');await Bun.write(Bun.stdout,db.serialize());db.close();"#;let edited=oracle(script,&file(&expected));expected.schema="new\0😀".into();assert_eq!(restore(&import_sqlite_database(&edited,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}
#[test]
fn sqlite_snapshot_puzzle2d_independent_malformed_owned_relations_refuse(){
 let bytes=file(&fixture());let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const results=[];for(const sql of input.sql){const db=Database.deserialize(new Uint8Array(input.bytes));db.run('PRAGMA ignore_check_constraints=ON');db.run(sql);results.push(Array.from(db.serialize()));db.close()}await Bun.write(Bun.stdout,JSON.stringify(results));"#;let laws=laws();let edited:Vec<Vec<u8>>=serde_json::from_slice(&oracle(script,serde_json::json!({"bytes":bytes,"sql":laws["malformedSql"]}).to_string().as_bytes())).unwrap();for(sql,bytes)in laws["malformedSql"].as_array().unwrap().iter().zip(edited){assert!(import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).map_err(|e|e.to_string()).and_then(|d|Puzzle2dSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).map_err(|e|e.to_string())).is_err(),"{sql}")}
}
#[test]
fn sqlite_snapshot_puzzle2d_exact_owned_rows_and_tiny_native_admission(){
 let expected=fixture();let d=database(&expected);let limits=SqliteDatabaseLimits::default();assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:42,..limits})).is_ok());assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:41,..limits})).is_err());assert!(Puzzle2dSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for restricted in[SqliteDatabaseLimits{max_rows:41,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits},SqliteDatabaseLimits{max_file_bytes:1,..limits}]{assert!(Puzzle2dSnapshot::decode_sqlite_snapshot_native(&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err())}}
}
#[test]
fn sqlite_snapshot_puzzle2d_initial_controls_refuse_every_native_and_relational_direction(){
 let expected=fixture();let d=database(&expected);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut reject=|_|false;let mut control=SqliteSnapshotControl::new(&mut reject,SqliteDatabaseLimits::default());let refused=match phase{SqliteSnapshotPhase::ProjectSnapshot=>expected.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>Puzzle2dSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::DecodeNative=>Puzzle2dSnapshot::decode_sqlite_snapshot_native(&payload(&expected,SnapshotEncoding::Text),&mut control).is_err(),SqliteSnapshotPhase::EncodeNative=>expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).is_err(),_=>panic!("unexpected phase")};assert!(refused,"{phase:?}")}
}
#[test]
fn sqlite_snapshot_puzzle2d_play_literal_text_preserves_every_typed_native_word(){
 let laws:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();for hex in laws["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=fixture();fill_words(&mut expected,f64::from_bits(bits));let binary=store::ArtifactPack::encode_pack(&expected);let play=<crate::Puzzle2dPlaySnapshot as store::ArtifactPack>::decode_pack(&binary).unwrap();let text=store::ArtifactDsl::print_dsl(&play);let restored=<crate::Puzzle2dPlaySnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap();assert_eq!(words(restored.typed()),vec![bits;43],"Play native Text {hex}");}
}

#[test]
fn sqlite_snapshot_puzzle2d_play_both_erased_directions_keep_every_native_word(){
 let codec=store::ArtifactCodec::bare::<crate::Puzzle2dPlaySnapshot,crate::Puzzle2dMutation>(crate::PUZZLE_2D_SCHEMA);let owner=codec.snapshot_sqlite.unwrap();
 for hex in laws()["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=fixture();fill_words(&mut expected,f64::from_bits(bits));
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let d=(owner.export)(&codec.schema,&dialect(),&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;assert_eq!(words(&restore(&d)),vec![bits;43]);let output=(owner.import)(&codec.schema,&dialect(),d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;let actual=<crate::Puzzle2dPlaySnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(words(actual.typed()),vec![bits;43]);assert_eq!(actual.typed().meta.manifest_id,expected.meta.manifest_id);}
 }
}
#[test]
fn sqlite_snapshot_puzzle2d_play_admits_exact_semantic_rows_and_physical_output_bytes(){
 let expected=fixture();let play=<crate::Puzzle2dPlaySnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&expected)).unwrap();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let output=play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();let bytes=match &output{store::io_schema::IoPayload::Binary(value)=>value.len(),store::io_schema::IoPayload::Text(value)=>value.len()};assert!(play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:42,max_file_bytes:bytes,..limits})).is_ok());for restricted in[SqliteDatabaseLimits{max_rows:41,..limits},SqliteDatabaseLimits{max_file_bytes:bytes-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());assert!(<crate::Puzzle2dPlaySnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,restricted)).is_err());}assert!(play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());}
}
#[cfg(feature="component-app-assembly")]
use semio_framework_plugin::{PluginApp,VcsArtifactApp,EditorApp,ViewerApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
#[cfg(feature="component-app-assembly")]
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 🎲️ The actual Puzzle editor and viewer app roster.
 enum SqliteApps:PluginApp{
  Editor(VcsArtifactApp<EditorApp<crate::editor::puzzle2d::Puzzle2dPlayApp>>),
  Viewer(VcsArtifactApp<ViewerApp<crate::viewer::puzzle2d::Puzzle2dViewer>>),
 }
}
#[cfg(feature="component-app-assembly")]
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_puzzle2d_actual_editor_viewer_declaration_routes_queryable_files(){
 semio_framework_plugin::Plugin::<SqliteApps>::builder("puzzle").label("Puzzle owned SQLite").version("0.0.1").package_id("semio:puzzle").declare_artifact(crate::artifact::<SqliteApps>()).try_build().unwrap();let codec=store::document_codec(crate::PUZZLE_2D_SCHEMA).await.unwrap().unwrap();assert_eq!(codec.snapshot_sqlite.as_ref().unwrap().snapshot_type,Some(std::any::TypeId::of::<crate::Puzzle2dSnapshot>()));let expected=fixture();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=store::io::io_mechanism::io_export_sqlite_snapshot(&dialect(),&expected,encoding,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));let query=oracle(r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT count(*) AS count FROM puzzle2d_node').get()));db.close();"#,&bytes);assert_eq!(serde_json::from_slice::<serde_json::Value>(&query).unwrap()["count"],2);let actual=store::io::io_mechanism::io_import_sqlite_snapshot::<crate::Puzzle2dSnapshot>(&dialect(),&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value;assert_eq!(actual,expected);}
}
