use semio_framework_test_fixtures::fixture_text;
use semio_framework_test_original_snapshot_owners as original_owners;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
fn snapshot()->Puzzle2dSnapshot{let laws:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();semio_framework_pack_json::from_json_str(&laws["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
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
 assert!(store::bare_artifact_codec::<Puzzle2dSnapshot,crate::Puzzle2dMutation>(crate::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA).snapshot_sqlite.is_some(),"Puzzle 2D's actual typed native codec requires owned relational SQLite");
}
/// 🔬️ Compares every authored neutral field through independent JSON, preserving exact numeric words.
fn assert_neutral_fields(actual:&serde_json::Value,expected:&serde_json::Value,path:&str){
 match(actual,expected){
  (serde_json::Value::Number(a),serde_json::Value::Number(b))=>assert_eq!(a.as_f64().unwrap().to_bits(),b.as_f64().unwrap().to_bits(),"{path}"),
  (serde_json::Value::Array(a),serde_json::Value::Array(b))=>{assert_eq!(a.len(),b.len(),"{path}");for(index,(a,b))in a.iter().zip(b).enumerate(){assert_neutral_fields(a,b,&format!("{path}/{index}"));}},
  (serde_json::Value::Object(a),serde_json::Value::Object(b))=>{assert_eq!(a.len(),b.len(),"{path}");for(key,value)in b{assert_neutral_fields(a.get(key).unwrap_or_else(||panic!("missing {path}/{key}")),value,&format!("{path}/{key}"));}},
  _=>assert_eq!(actual,expected,"{path}"),
 }
}
/// 📜️ Builds the independent declared wire specimen from authored typed float pointers.
fn declared_json_specimen(word:Option<u64>)->serde_json::Value{
 let laws=laws();let mut expected=laws["snapshot"].clone();
 let paths=laws["declaredJson"]["floatPointers"].as_array().unwrap();assert_eq!(paths.len(),43);
 for path in paths{
  let value=expected.pointer_mut(path.as_str().unwrap()).expect("authored float slot");
  let bits=word.unwrap_or_else(||value.as_f64().expect("neutral native number").to_bits());
  *value=serde_json::json!({"bits":format!("{bits:016x}")});
 }
 expected
}
/// 🧳️ Retires the genuine test snapshot on success and on a failed JSON assertion.
struct JsonSnapshotOwner(Option<Puzzle2dSnapshot>);
impl JsonSnapshotOwner {
 fn get(&self)->&Puzzle2dSnapshot{self.0.as_ref().unwrap()}
}
impl Drop for JsonSnapshotOwner {
 fn drop(&mut self){if let Some(snapshot)=self.0.take(){snapshot.retire_sqlite_snapshot();}}
}
#[test]
fn sqlite_snapshot_puzzle2d_declared_json_preserves_complete_fields_and_every_word(){
 use crate::standards::v1::subsets::any::io::{export::serializers::artifacts::json::v_rfc8259::any as output,import::deserializers::artifacts::json::v_rfc8259::any as input};
 let source=JsonSnapshotOwner(Some(snapshot()));
 let bytes=output::serialize_bytes(source.get()).expect("declared JSON export");
 let independent:serde_json::Value=serde_json::from_slice(&bytes).expect("independent JSON parse");
 assert_eq!(independent,declared_json_specimen(None),"complete declared wire fields");
 assert_registered_json_route(source.get(),&independent);
 let restored=JsonSnapshotOwner(Some(input::deserialize_bytes(&serde_json::to_vec(&independent).unwrap()).expect("independently encoded declared JSON import")));
 assert_eq!(restored.get(),source.get());
 for hex in laws()["binary64Words"].as_array().unwrap(){
  let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();
  let mut snapshot=snapshot();fill_words(&mut snapshot,f64::from_bits(bits));
  let source=JsonSnapshotOwner(Some(snapshot));
  let bytes=output::serialize_bytes(source.get()).expect("declared JSON word export");
  let independent:serde_json::Value=serde_json::from_slice(&bytes).expect("independent JSON word parse");
  assert_eq!(independent,declared_json_specimen(Some(bits)),"complete declared JSON {hex}");
  assert_registered_json_route(source.get(),&independent);
  let restored=JsonSnapshotOwner(Some(input::deserialize_bytes(&serde_json::to_vec(&independent).unwrap()).expect("independently encoded declared JSON word import")));
  assert_eq!(words(restored.get()),vec![bits;43],"declared JSON {hex}");
 }
}

#[test]
fn sqlite_snapshot_puzzle2d_complete_neutral_fields_match_independent_serde_and_both_native_directions(){
 let expected=snapshot();let value:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let independent=&value["snapshot"];let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&expected)))).unwrap();assert_neutral_fields(&actual,independent,"snapshot");assert_eq!(words(&expected).len(),43);
 let binary=<Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack_with(&expected,&store::PackEncodeOptions::default()).unwrap();assert_eq!(<Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(&binary,&store::PackDecodeOptions::default()).unwrap(),expected);
 let text=<Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&expected);assert_eq!(<Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).unwrap(),expected);
}
#[test]
fn sqlite_snapshot_puzzle2d_every_board_and_catalog_number_preserves_exact_native_words(){
 let laws:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for hex in laws["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=snapshot();fill_words(&mut expected,f64::from_bits(bits));
 let binary=<Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack_with(&expected,&store::PackEncodeOptions::default()).unwrap();assert_eq!(words(&<Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(&binary,&store::PackDecodeOptions::default()).unwrap()),vec![bits;43],"Pack {hex}");
 let text=<Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&expected);assert_eq!(words(&<Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).unwrap()),vec![bits;43],"Text {hex}");}
}

#[test]
fn sqlite_snapshot_puzzle2d_play_codec_exposes_the_same_owned_persisted_snapshot(){
 assert!(store::bare_artifact_codec::<crate::Puzzle2dPlaySnapshot,crate::Puzzle2dMutation>(crate::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA).snapshot_sqlite.is_some(),"Puzzle 2D play snapshot requires the same owned semantic SQLite capability");
}

use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,export_sqlite_database,import_sqlite_database}};
fn laws()->serde_json::Value{serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn database(value:&Puzzle2dSnapshot)->SqliteDatabase{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|value.to_sqlite_database(sqlite_control)).unwrap()}
fn restore(value:&SqliteDatabase)->Puzzle2dSnapshot{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|Puzzle2dSnapshot::from_sqlite_database(value, sqlite_control)).unwrap()}
fn file(value:&Puzzle2dSnapshot)->Vec<u8>{export_sqlite_database(&database(value),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()}
fn payload(value:&Puzzle2dSnapshot,encoding:SnapshotEncoding)->store::io_schema::IoPayload{match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(value)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(value))}}
fn dialect()->semio_framework_artifact_reference::ArtifactDialect{semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.puzzle.puzzle2d".into(),standard:"1".into(),subset:"*".into()}}
fn oracle(script:&str,input:&[u8])->Vec<u8>{use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(input).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));output.stdout}
#[test]
fn sqlite_snapshot_puzzle2d_full_native_relational_fields_survive_physical_files(){
 let expected=snapshot();let db=database(&expected);assert_eq!(db.tables.len(),20);assert_eq!(db.tables.iter().map(|t|t.rows.len()).sum::<usize>(),42);assert_eq!(restore(&db),expected);assert_eq!(restore(&import_sqlite_database(&file(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}
#[test]
fn sqlite_snapshot_puzzle2d_every_board_catalog_word_reaches_both_erased_outputs(){
 let codec=store::bare_artifact_codec::<Puzzle2dSnapshot,crate::Puzzle2dMutation>(crate::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA);let owner=codec.snapshot_sqlite.unwrap();for hex in laws()["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=snapshot();fill_words(&mut expected,f64::from_bits(bits));for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let d=original_owners::with_decoding(|native_owner|(owner.export)(&codec.schema,&dialect(),&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;assert_eq!(words(&restore(&d)),vec![bits;43]);let output=original_owners::with_encoding(|native_owner|(owner.import)(&codec.schema,&dialect(), &mut Some(d),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;let actual=original_owners::with_decoding(|native_owner|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap();assert_eq!(words(&actual),vec![bits;43]);assert_eq!(actual.meta.manifest_id,expected.meta.manifest_id);}}
}
#[test]
fn sqlite_snapshot_puzzle2d_independent_queries_and_surrogate_edits_retain_literal_domains(){
 let mut expected=snapshot();expected.nodes[1].id=expected.nodes[0].id.clone();let script=r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const nodes=db.query('SELECT node_id,anchor,radius_bits FROM puzzle2d_node ORDER BY ordinal').all();if(nodes.length!==2||nodes[0].node_id!==nodes[1].node_id)throw Error('ordered duplicate node domains');db.run('UPDATE puzzle2d_document SET schema=?',['new\0😀']);db.run('UPDATE puzzle2d_node SET id=id+90');db.run('UPDATE puzzle2d_handle SET node_id=node_id+90,id=id+90');await Bun.write(Bun.stdout,db.serialize());db.close();"#;let edited=oracle(script,&file(&expected));expected.schema="new\0😀".into();assert_eq!(restore(&import_sqlite_database(&edited,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()),expected);
}
#[test]
fn sqlite_snapshot_puzzle2d_independent_malformed_owned_relations_refuse(){
 let bytes=file(&snapshot());let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text());const results=[];for(const sql of input.sql){const db=Database.deserialize(new Uint8Array(input.bytes));db.run('PRAGMA ignore_check_constraints=ON');db.run(sql);results.push(Array.from(db.serialize()));db.close()}await Bun.write(Bun.stdout,JSON.stringify(results));"#;let laws=laws();let edited:Vec<Vec<u8>>=serde_json::from_slice(&oracle(script,serde_json::json!({"bytes":bytes,"sql":laws["malformedSql"]}).to_string().as_bytes())).unwrap();for(sql,bytes)in laws["malformedSql"].as_array().unwrap().iter().zip(edited){assert!(import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).map_err(|e|e.to_string()).and_then(|d|original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|Puzzle2dSnapshot::from_sqlite_database(&d, sqlite_control)).map_err(|e|e.to_string())).is_err(),"{sql}")}
}
#[test]
fn sqlite_snapshot_puzzle2d_exact_owned_rows_and_tiny_native_admission(){
 let expected=snapshot();let d=database(&expected);let limits=SqliteDatabaseLimits::default();assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits{max_rows:42,..limits},&mut |_|true,|sqlite_control|expected.to_sqlite_database(sqlite_control)).is_ok());assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits{max_rows:41,..limits},&mut |_|true,|sqlite_control|expected.to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::with_sqlite_recipient(SqliteDatabaseLimits{max_value_bytes:1,..limits},&mut |_|true,|sqlite_control|Puzzle2dSnapshot::from_sqlite_database(&d, sqlite_control)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{for restricted in[SqliteDatabaseLimits{max_rows:41,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits},SqliteDatabaseLimits{max_file_bytes:1,..limits}]{assert!(original_owners::with_decoding(|native_owner|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,restricted), native_owner)).is_err());assert!(original_owners::with_encoding(|native_owner|expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted), native_owner)).is_err())}}
}
#[test]
fn sqlite_snapshot_puzzle2d_initial_controls_refuse_every_native_and_relational_direction(){
 let expected=snapshot();let d=database(&expected);for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut reject=|_|false;let mut control=SqliteSnapshotControl::new(&mut reject,SqliteDatabaseLimits::default());let refused=match phase{SqliteSnapshotPhase::ProjectSnapshot=>expected.to_sqlite_database(&mut control).is_err(),SqliteSnapshotPhase::ReconstructSnapshot=>Puzzle2dSnapshot::from_sqlite_database(&d,&mut control).is_err(),SqliteSnapshotPhase::DecodeNative=>original_owners::with_decoding(|native_owner|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&payload(&expected,SnapshotEncoding::Text),&mut control, native_owner)).is_err(),SqliteSnapshotPhase::EncodeNative=>original_owners::with_encoding(|native_owner|expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control, native_owner)).is_err(),_=>panic!("unexpected phase")};assert!(refused,"{phase:?}")}
}
#[test]
fn sqlite_snapshot_puzzle2d_play_literal_text_preserves_every_typed_native_word(){
 let laws:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for hex in laws["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=snapshot();fill_words(&mut expected,f64::from_bits(bits));let binary=pack::snapshot::ArtifactPack::encode_pack(&expected);let play=<crate::Puzzle2dPlaySnapshot as pack::snapshot::ArtifactPack>::decode_pack(&binary).unwrap();let text=pack::snapshot::ArtifactDsl::print_dsl(&play);let restored=<crate::Puzzle2dPlaySnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).unwrap();assert_eq!(words(restored.typed()),vec![bits;43],"Play native Text {hex}");}
}

#[test]
fn sqlite_snapshot_puzzle2d_play_both_erased_directions_keep_every_native_word(){
 let codec=store::bare_artifact_codec::<crate::Puzzle2dPlaySnapshot,crate::Puzzle2dMutation>(crate::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA);let owner=codec.snapshot_sqlite.unwrap();
 for hex in laws()["binary64Words"].as_array().unwrap(){let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let mut expected=snapshot();fill_words(&mut expected,f64::from_bits(bits));
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let d=original_owners::with_decoding(|native_owner|(owner.export)(&codec.schema,&dialect(),&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;assert_eq!(words(&restore(&d)),vec![bits;43]);let output=original_owners::with_encoding(|native_owner|(owner.import)(&codec.schema,&dialect(), &mut Some(d),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;let actual=original_owners::with_decoding(|native_owner|<crate::Puzzle2dPlaySnapshot as ArtifactSqliteSnapshotHost>::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap();assert_eq!(words(actual.typed()),vec![bits;43]);assert_eq!(actual.typed().meta.manifest_id,expected.meta.manifest_id);}
 }
}
#[test]
fn sqlite_snapshot_puzzle2d_play_admits_exact_semantic_rows_and_physical_output_bytes(){
 let expected=snapshot();let play=<crate::Puzzle2dPlaySnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack::snapshot::ArtifactPack::encode_pack(&expected)).unwrap();let limits=SqliteDatabaseLimits::default();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let output=original_owners::with_encoding(|native_owner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).unwrap();let bytes=match &output{store::io_schema::IoPayload::Binary(value)=>value.len(),store::io_schema::IoPayload::Text(value)=>value.len()};assert!(original_owners::with_encoding(|native_owner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:42,max_file_bytes:bytes,..limits}), native_owner)).is_ok());for restricted in[SqliteDatabaseLimits{max_rows:41,..limits},SqliteDatabaseLimits{max_file_bytes:bytes-1,..limits},SqliteDatabaseLimits{max_value_bytes:1,..limits}]{assert!(original_owners::with_encoding(|native_owner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,restricted), native_owner)).is_err());assert!(original_owners::with_decoding(|native_owner|<crate::Puzzle2dPlaySnapshot as ArtifactSqliteSnapshotHost>::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,restricted), native_owner)).is_err());}assert!(original_owners::with_encoding(|native_owner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,limits), native_owner)).is_err());}
}
use semio_plugin::PluginApp;
use semio_plugin::VcsArtifactApp;
use semio_plugin::EditorApp;
use semio_plugin::ViewerApp;
use semio_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 🎲️ The actual Puzzle editor and viewer app roster.
 enum SqliteApps:PluginApp{
  Editor(VcsArtifactApp<EditorApp<crate::editor::puzzle2d::Puzzle2dPlayApp>>),
  Viewer(VcsArtifactApp<ViewerApp<crate::viewer::puzzle2d::Puzzle2dViewer>>),
 }
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_puzzle2d_actual_editor_viewer_declaration_routes_queryable_files(){
 semio_plugin::PluginBuilder::<_, SqliteApps>::new("puzzle").label("Puzzle owned SQLite").version("0.0.1").package_id("semio:puzzle").declare_artifact(crate::artifact::<SqliteApps>()).try_build().unwrap();let codec=store::os_store::codec_registry::document_codec(crate::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA).await.unwrap().unwrap();assert_eq!(codec.snapshot_sqlite.as_ref().unwrap().snapshot_type,Some(std::any::TypeId::of::<crate::Puzzle2dSnapshot>()));let expected=snapshot();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=async{let original_file_dialect=&dialect();let original_file_input=&expected;let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_decoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|store::io::io_mechanism::io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));let query=oracle(r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT count(*) AS count FROM puzzle2d_node').get()));db.close();"#,&bytes);assert_eq!(serde_json::from_slice::<serde_json::Value>(&query).unwrap()["count"],2);let actual=async{let original_file_dialect=&dialect();let original_file_input=&bytes;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|store::io::io_mechanism::io_import_sqlite_snapshot::<crate::Puzzle2dSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value;assert_eq!(actual,expected);}
}

#[test]
fn sqlite_snapshot_puzzle2d_complete_cells_have_independent_exact_typed_and_play_native_admission(){
 let laws=laws();let cases=std::iter::once(None).chain(laws["binary64Words"].as_array().unwrap().iter().map(|word|Some(word.as_str().unwrap())));
 for word in cases{
  let mut source=snapshot();if let Some(word)=word{fill_words(&mut source,f64::from_bits(u64::from_str_radix(word,16).unwrap()));}
  let database=database(&source);let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let tables:serde_json::Value=serde_json::from_slice(&oracle(r####"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(({name})=>{const columns=db.query('PRAGMA table_info('+name+')').all(),expression=columns.map(({name})=>{const c='"'+name.replaceAll('"','""')+'"';return "CASE typeof("+c+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+c+" AS BLOB)) WHEN 'blob' THEN length("+c+") WHEN 'null' THEN 0 ELSE NULL END"}).join('+'),r=db.query('SELECT COUNT(*) AS rows,COALESCE(SUM('+expression+'),0) AS bytes FROM '+name).get();return{table:name,columns:columns.length,rows:Number(r.rows),semanticBytes:Number(r.bytes)}});await Bun.write(Bun.stdout,JSON.stringify(tables));}finally{db.close()}"####,&bytes)).unwrap();let tables=tables.as_array().unwrap();let widths:Vec<_>=tables.iter().map(|t|serde_json::json!([t["table"],t["columns"]])).collect();
  assert_eq!(serde_json::Value::Array(widths),laws["semanticCells"]["tableWidths"]);assert!(tables.iter().all(|t|t["rows"].as_u64().unwrap()>0));
  let rows=tables.iter().map(|t|t["rows"].as_u64().unwrap()as usize).sum::<usize>();let cells=tables.iter().map(|t|t["semanticBytes"].as_u64().unwrap()as usize).sum::<usize>();assert_eq!(rows,laws["semanticCells"]["fullRows"].as_u64().unwrap()as usize);
  let exact=SqliteDatabaseLimits{max_rows:rows,max_value_bytes:cells,..Default::default()};
  assert_eq!(original_owners::with_sqlite_recipient(exact,&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).unwrap(),database);
  let play=<crate::Puzzle2dPlaySnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack::snapshot::ArtifactPack::encode_pack(&source)).unwrap();
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let expected=payload(&source,encoding);
   source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();
   play.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();
   assert_eq!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap(),expected);
   assert_eq!(original_owners::with_encoding(|native_owner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap(),expected);
   let actual=original_owners::with_decoding(|native_owner|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();assert_eq!(words(&actual),words(&source));assert_eq!(payload(&actual,encoding),expected);actual.retire_sqlite_snapshot();
   let actual=original_owners::with_decoding(|native_owner|<crate::Puzzle2dPlaySnapshot as ArtifactSqliteSnapshotHost>::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,exact), native_owner)).unwrap();assert_eq!(words(actual.typed()),words(&source));assert_eq!(payload(actual.typed(),encoding),expected);actual.retire_sqlite_snapshot();
   for short in[SqliteDatabaseLimits{max_rows:rows-1,..exact},SqliteDatabaseLimits{max_value_bytes:cells-1,..exact}]{
    assert!(source.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err());assert!(play.preflight_sqlite_snapshot_encoding(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err());
    assert!(original_owners::with_encoding(|native_owner|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err());assert!(original_owners::with_encoding(|native_owner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err());
    assert!(original_owners::with_decoding(|native_owner|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err());assert!(original_owners::with_decoding(|native_owner|<crate::Puzzle2dPlaySnapshot as ArtifactSqliteSnapshotHost>::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,short), native_owner)).is_err());
   }
  }
  for short in[SqliteDatabaseLimits{max_rows:rows-1,..exact},SqliteDatabaseLimits{max_value_bytes:cells-1,..exact}]{assert!(original_owners::with_sqlite_recipient(short,&mut |_|true,|sqlite_control|source.to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::with_sqlite_recipient(short,&mut |_|true,|sqlite_control|Puzzle2dSnapshot::from_sqlite_database(&database, sqlite_control)).is_err());}
 }
}

/// 🚏️ Executes the canonical native entry and independently encoded foreign carrier over every field.
fn assert_registered_json_route(source:&Puzzle2dSnapshot,expected:&serde_json::Value){
 use pack::snapshot::{ArtifactNativeSnapshot,NativeSnapshotInput,NativeSnapshotEncoding};
 use store::io::io_mechanism::{IoRunControl,IoEntryDirection};
 use store::io_schema::IoPayload;
 let entries=crate::standards::v1::subsets::any::io::routes::entries();
 let export=entries.iter().find(|entry|entry.direction==IoEntryDirection::Export&&entry.into.artifact_kind=="s.stdio.json").expect("canonical JSON export");
 let import=entries.iter().find(|entry|entry.direction==IoEntryDirection::Import&&entry.from.artifact_kind=="s.stdio.json").expect("canonical JSON import");
 let native=original_owners::with_encoding(|owner|source.encode_native_snapshot(NativeSnapshotEncoding::Binary,owner.native())).expect("original native carrier");
 let exported=original_owners::with_decoding(|owner|(export.run)(&native,&mut IoRunControl::decoding(owner.native(),original_owners::original_grant()))).expect("actual registered JSON export").value;
 let IoPayload::Text(exported)=exported else{panic!("canonical JSON entry preserves its text carrier")};let exported=exported.into_bytes();
 assert_eq!(&serde_json::from_slice::<serde_json::Value>(&exported).expect("independent registered JSON parse"),expected,"complete registered wire");
 let foreign=IoPayload::Binary(serde_json::to_vec(expected).expect("independent registered JSON encode"));
 let restored=original_owners::with_encoding(|owner|(import.run)(&foreign,&mut IoRunControl::encoding(owner.native(),original_owners::original_grant()))).expect("actual registered JSON import").value;
 let IoPayload::Binary(restored)=restored else{panic!("native binary entry output")};
 let restored=RouteSnapshotOwner(Some(original_owners::with_decoding(|owner|Puzzle2dSnapshot::decode_native_snapshot(NativeSnapshotInput::Binary(&restored),owner.native())).expect("original restored native carrier")));
 let project=|snapshot:&Puzzle2dSnapshot|original_owners::with_sqlite_recipient(store::sqlite_snapshot::SqliteDatabaseLimits::default(),&mut |_|true,|control|snapshot.to_sqlite_database(control)).unwrap();
 assert_eq!(project(restored.0.as_ref().unwrap()),project(source),"all registered native entities and exact words");
}
struct RouteSnapshotOwner(Option<Puzzle2dSnapshot>);
impl Drop for RouteSnapshotOwner{fn drop(&mut self){if let Some(snapshot)=self.0.take(){snapshot.retire_sqlite_snapshot();}}}
