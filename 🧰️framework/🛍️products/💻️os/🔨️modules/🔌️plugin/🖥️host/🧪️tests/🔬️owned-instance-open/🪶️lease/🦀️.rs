//! 🪶️ Held real-component Count and selected refusal witnesses; attach through the owning host test module.
use super::*;
use semio_framework::sqlite_snapshot::{SqliteDatabaseLimits,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SnapshotEncoding};
use semio_framework::io_schema::ArtifactDialect;
use semio_framework_value::ValueRefusalKind;
const CORPUS:&str=include_str!("../../../🧫️fixtures/🪶️workspace-lease/🔣️.json");
fn corpus()->serde_json::Value{serde_json::from_str(CORPUS).expect("closed neutral component lease corpus")}
fn output_root()->PathBuf{let root=PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("owning component test requires an explicit task artifact root")).join("count-component");std::fs::create_dir_all(&root).expect("component output root");root}
fn independent_file(bytes:&[u8],count:i32,encoding:&str){
 let path=output_root().join(format!("count-{count}-{encoding}.sqlite"));std::fs::write(&path,bytes).expect("actual guest SQLite output");
 let oracle=r#"import{Database}from"bun:sqlite";import assert from"node:assert/strict";const[path,count,encoding]=process.argv.slice(1);const d=new Database(path,{readonly:true});try{assert.deepEqual(d.query("SELECT id,count,typeof(count) AS storage FROM fixture_counter").all(),[{id:1,count:Number(count),storage:"integer"}]);assert.deepEqual(d.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all(),[{name:"fixture_counter"},{name:"semio_snapshot"}]);assert.deepEqual(d.query("PRAGMA table_info(fixture_counter)").all().map(x=>x.name),["id","count"]);assert.deepEqual(d.query("SELECT artifact_kind,standard,subset,schema_version,native_encoding FROM semio_snapshot").all(),[{artifact_kind:"fixture.neutral-host-fixture.counter",standard:"1",subset:"*",schema_version:1,native_encoding:encoding}]);assert.deepEqual(d.query("PRAGMA integrity_check").get(),{integrity_check:"ok"});assert.deepEqual(d.query("PRAGMA foreign_key_check").all(),[])}finally{d.close()}"#;
 let result=std::process::Command::new("bun").args(["-e",oracle]).arg(&path).arg(count.to_string()).arg(encoding).output().expect("independent Bun SQLite");
 assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));
}
async fn export_file(runtime:&OwnedRuntime,compiled:&CompiledHandle,dialect:&str,encoding:&str,payload:&[u8])->sqlite_wire::SnapshotFile{
 match runtime.codec_sqlite_export(compiled,dialect,encoding,payload,SqliteDatabaseLimits::default(),codec_budget(),|_,_|{},&GuestCallCancellation::default()).await.expect("compiled SQLite export"){
  sqlite_wire::SnapshotFileResult::Done(file)=>file,sqlite_wire::SnapshotFileResult::Rejected(error)=>panic!("valid complete Count refused: {:?}",error)
 }
}
#[semio_framework_async_macros::async_test]
async fn count_component_full_i32_both_native_encodings_use_real_wasm_and_independent_sqlite(){
 let component=std::fs::read(fixture_component()).expect("required staged real component");let runtime=OwnedRuntime::new();
 let compiled=runtime.compile(&package_ref("semio:neutral-host-fixture",&component),&component).await.expect("actual component compile");
 let fixture=corpus();let dialect=fixture["owner"].as_str().unwrap();
 assert_eq!(runtime.codec_sqlite_schema(&compiled,dialect,codec_budget()).await.expect("guest own SQL declaration").trim(),include_str!("../../../🧪️testing/🧩️component/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql").trim());
 let hash=runtime.codec_pack_schema_hash(&compiled,FIXTURE_DOCUMENT_SCHEMA,codec_budget()).await.expect("actual guest record identity");assert_ne!(hash,[0;32]);
 let genesis=runtime.codec_genesis(&compiled,FIXTURE_DOCUMENT_SCHEMA,MINTED_DOCUMENT_ID,codec_budget()).await.expect("actual Count document context");
 for count in fixture["counts"].as_array().unwrap(){
  let count=i32::try_from(count.as_i64().unwrap()).unwrap();let text=format!("{{\"count\":{count}}}");
  let text_file=export_file(&runtime,&compiled,dialect,"text",text.as_bytes()).await;independent_file(&text_file.bytes,count,"text");
  let sqlite_wire::SnapshotPayloadResult::Done(text_again)=runtime.codec_sqlite_import(&compiled,dialect,&text_file.bytes,SqliteDatabaseLimits::default(),codec_budget(),|_,_|{},&GuestCallCancellation::default()).await.expect("compiled text import")else{panic!("valid text file refused")};
  assert_eq!(text_again.encoding,"text");assert_eq!(serde_json::from_slice::<serde_json::Value>(&text_again.bytes).unwrap(),serde_json::json!({"count":count}));
  let mut database=semio_framework::sqlite_snapshot::import_sqlite_database(&text_file.bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let metadata=database.table_mut("semio_snapshot").unwrap();metadata.rows[0].values[5]=SqliteValue::Text("binary".into());
  let binary_request=semio_framework::sqlite_snapshot::export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let sqlite_wire::SnapshotPayloadResult::Done(binary)=runtime.codec_sqlite_import(&compiled,dialect,&binary_request,SqliteDatabaseLimits::default(),codec_budget(),|_,_|{},&GuestCallCancellation::default()).await.expect("compiled binary import")else{panic!("valid binary file refused")};
  assert_eq!(binary.encoding,"binary");assert!(!binary.bytes.is_empty());
  let mirror=runtime.codec_print_mirror(&compiled,FIXTURE_DOCUMENT_SCHEMA,&binary.bytes,&genesis.spr,codec_budget()).await.expect("compiled Count record semantic decode");assert_eq!(serde_json::from_str::<serde_json::Value>(&mirror.dsl).unwrap(),serde_json::json!({"count":count}));
  let binary_file=export_file(&runtime,&compiled,dialect,"binary",&binary.bytes).await;independent_file(&binary_file.bytes,count,"binary");
  let sqlite_wire::SnapshotPayloadResult::Done(binary_again)=runtime.codec_sqlite_import(&compiled,dialect,&binary_file.bytes,SqliteDatabaseLimits::default(),codec_budget(),|_,_|{},&GuestCallCancellation::default()).await.expect("compiled binary roundtrip")else{panic!("valid binary file refused")};assert_eq!(binary_again,binary);
 }
}
#[semio_framework_async_macros::async_test]
async fn count_component_cancellation_occurs_during_real_export_and_import_interpretation(){
 let component=std::fs::read(fixture_component()).unwrap();let runtime=OwnedRuntime::new();let compiled=runtime.compile(&package_ref("semio:neutral-host-fixture",&component),&component).await.unwrap();
 let fixture=corpus();let dialect=fixture["owner"].as_str().unwrap();let input=b"{\"count\":2147483647}";
 let file=export_file(&runtime,&compiled,dialect,"text",input).await;
 for import in [false,true]{
  let cancel=GuestCallCancellation::default();let mut observed=0u64;
  let result=if import{runtime.codec_sqlite_import(&compiled,dialect,&file.bytes,SqliteDatabaseLimits::default(),codec_budget(),|_,_|{observed+=1;cancel.cancel()},&cancel).await.map(|_|())}else{runtime.codec_sqlite_export(&compiled,dialect,"text",input,SqliteDatabaseLimits::default(),codec_budget(),|_,_|{observed+=1;cancel.cancel()},&cancel).await.map(|_|())};
  assert!(observed>0);assert!(matches!(result,Err(TurnFault::Cancelled)));assert!(cancel.is_cancelled());
 }
}
fn refusal_file(dialect:&ArtifactDialect)->Vec<u8>{
 let mut database=SqliteDatabase::from_schema("CREATE TABLE fixture_refusal (id INTEGER PRIMARY KEY, value INTEGER NOT NULL);").unwrap();database.table_mut("fixture_refusal").unwrap().rows.push(SqliteRow{rowid:1,values:vec![SqliteValue::Integer(1),SqliteValue::Integer(0)]});
 semio_framework::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut database,dialect,SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 semio_framework::sqlite_snapshot::export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap()
}
#[semio_framework_async_macros::async_test]
async fn count_component_selected_compiled_refusal_owners_preserve_all_eight_causes_and_full_nul_diagnostics(){
 let component=std::fs::read(fixture_component()).unwrap();let runtime=OwnedRuntime::new();let compiled=runtime.compile(&package_ref("semio:neutral-host-fixture",&component),&component).await.unwrap();
 let fixture=corpus();let diagnostic:semio_framework::Diagnostic=semio_framework_pack_json::from_json_str(&fixture["diagnostic"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let kinds=[ValueRefusalKind::InvalidValue,ValueRefusalKind::Canceled,ValueRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed,ValueRefusalKind::WorkLimit,ValueRefusalKind::DepthLimit,ValueRefusalKind::UnsupportedOwner,ValueRefusalKind::InvariantViolated];
 for (row,kind)in fixture["refusals"].as_array().unwrap().iter().zip(kinds){
  let coordinate=row["owner"].as_str().unwrap();let dialect=ArtifactDialect::parse_coordinate(coordinate).unwrap();let file=refusal_file(&dialect);
  let sqlite_wire::SnapshotFileResult::Rejected(export)=runtime.codec_sqlite_export(&compiled,coordinate,"text",b"{\"value\":0}",SqliteDatabaseLimits::default(),codec_budget(),|_,_|{},&GuestCallCancellation::default()).await.expect("selected compiled provider export")else{panic!("explicit fault owner exported")};
  let sqlite_wire::SnapshotPayloadResult::Rejected(import)=runtime.codec_sqlite_import(&compiled,coordinate,&file,SqliteDatabaseLimits::default(),codec_budget(),|_,_|{},&GuestCallCancellation::default()).await.expect("selected compiled provider import")else{panic!("explicit fault owner imported")};
  for rejection in [export,import]{assert_eq!(rejection.kind,kind);let error=rejection.into_io_error().expect("packed guest diagnostics");assert_eq!(error.cause.kind,kind);assert_eq!(error.cause.message,fixture["message"].as_str().unwrap());assert_eq!(error.diagnostics,vec![diagnostic.clone()]);}
 }
}
