//! 🧫️ The published Host Count owner requires its own semantic SQLite capability.
use crate::{Snapshot,Mutation};
use semio_framework_os_kernel as store;
use store::{ArtifactDsl,ArtifactPack};
use store::sqlite_snapshot::*;
use semio_framework_value::ValueRefusalKind;
const KIND:&str="fixture.neutral-host-fixture.counter";
const SQL:&str=include_str!("../../🪶️sqlite/🗄️.sql");
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn counts()->Vec<i32>{corpus()["counts"].as_array().unwrap().iter().map(|value|i32::try_from(value.as_i64().unwrap()).unwrap()).collect()}
fn dialect()->semio_framework_artifact_reference::ArtifactDialect{semio_framework_artifact_reference::ArtifactDialect{artifact_kind:KIND.into(),standard:"1".into(),subset:"*".into()}}
fn codec()->store::ArtifactSqliteSnapshotCodec{
 store::ArtifactCodec::bare::<Snapshot,Mutation>(KIND).snapshot_sqlite.expect("the actual published Count owner must opt its bare native codec into semantic SQLite")
}
fn original_native_grant()->semio_framework_value::RetainedCloneGrant{
 let request:semio_framework_plugin::sqlite_wire::SnapshotInput=serde_json::from_str(include_str!("../../../../../../../🧬️schema/🪶️sqlite/🧫️fixtures/🔣️.json")).unwrap();request.native.native().unwrap()
}
fn export_native(codec:&store::ArtifactSqliteSnapshotCodec,schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,payload:&store::io::IoPayload,sql:&mut SqliteSnapshotControl<'_>)->semio_framework::io_schema::IoResult<SqliteDatabase>{
 let grant=original_native_grant();let mut observe=|_|true;let mut native=semio_framework_value::NativeDecodeControl::new(grant.maximum_capacity_bytes,&mut observe);let mut owner=store::io::control::NativeSnapshotDecodeOwner::new(&mut native,grant);(codec.export)(schema,dialect,payload,sql,&mut owner)
}
fn import_native(codec:&store::ArtifactSqliteSnapshotCodec,schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:SqliteDatabase,encoding:SnapshotEncoding,sql:&mut SqliteSnapshotControl<'_>)->semio_framework::io_schema::IoResult<store::io::IoPayload>{
 let grant=original_native_grant();let mut observe=|_|true;let mut native=semio_framework_value::NativeEncodeControl::new(grant.maximum_capacity_bytes,&mut observe);let mut owner=store::io::control::NativeSnapshotEncodeOwner::new(&mut native,grant);(codec.import)(schema,dialect,database,encoding,sql,&mut owner)
}
fn database(count:i32)->SqliteDatabase{
 let mut database=SqliteDatabase::from_schema(SQL).unwrap();
 database.table_mut("fixture_counter").unwrap().rows.push(SqliteRow{rowid:1,values:vec![SqliteValue::Integer(1),SqliteValue::Integer(i64::from(count))]});
 database
}
fn payload(owner:&Snapshot,encoding:SnapshotEncoding)->store::io::IoPayload{
 match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(owner.encode_pack()),SnapshotEncoding::Text=>store::io::IoPayload::Text(owner.print_dsl())}
}
fn decode(payload:store::io::IoPayload)->Snapshot{
 match payload{store::io::IoPayload::Binary(bytes)=>Snapshot::decode_pack(&bytes).unwrap(),store::io::IoPayload::Text(text)=>Snapshot::parse_dsl(&text).unwrap()}
}
#[test]
fn sqlite_snapshot_host_count_actual_capability_and_both_declared_surfaces_share_the_owner(){
 use semio_framework_plugin::app::ArtifactApp;
 let codec=codec();assert_eq!(codec.snapshot_type,Some(std::any::TypeId::of::<Snapshot>()));assert_eq!(codec.schema,SQL);
 assert_eq!(<crate::assembly::CounterApp<false> as ArtifactApp>::DOCUMENT_SCHEMA,KIND);
 assert_eq!(<crate::assembly::CounterApp<true> as ArtifactApp>::DOCUMENT_SCHEMA,KIND);
 let editor=semio_framework_plugin::app::artifact_codec_table::<crate::assembly::CounterApp<false>>();
 let viewer=semio_framework_plugin::app::artifact_codec_table::<crate::assembly::CounterApp<true>>();
 assert_eq!((editor.pack_schema_hash)(),(viewer.pack_schema_hash)());
 assert!((editor.pack_schema_hash)().is_some());
 assert!(<Mutation as store::SemanticMutation<Snapshot>>::kinds().is_empty());
}
#[test]
fn sqlite_snapshot_host_count_erased_native_and_physical_roundtrip_preserve_full_i32(){
 let codec=codec();let limits=SqliteDatabaseLimits::default();
 for count in counts(){let source=Snapshot{count};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let native=payload(&source,encoding);assert_eq!(decode(payload(&source,encoding)),source);
  let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);
  let projected=export_native(&codec,KIND,&dialect(),&native,&mut control).unwrap().value;
  assert_eq!(projected,database(count));
  let bytes=export_sqlite_database(&projected,limits,&mut |_|true).unwrap();assert_eq!(&bytes[..16],b"SQLite format 3\0");
  let imported=import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();assert_eq!(imported,projected);
  let output=import_native(&codec,KIND,&dialect(),imported,encoding,&mut control).unwrap().value;assert_eq!(decode(output),source);
 }}
}
#[test]
fn sqlite_snapshot_host_count_reconstruction_rejects_structural_and_integer_domain_corruption(){
 let codec=codec();let valid=database(0);let mut cases=Vec::new();
 let mut wrong=valid.clone();wrong.table_mut("fixture_counter").unwrap().rows.clear();cases.push(wrong);
 let mut wrong=valid.clone();wrong.table_mut("fixture_counter").unwrap().rows.push(SqliteRow{rowid:2,values:vec![SqliteValue::Integer(2),SqliteValue::Integer(0)]});cases.push(wrong);
 for value in[SqliteValue::Integer(-2147483649),SqliteValue::Integer(2147483648),SqliteValue::Real(1.0),SqliteValue::Text("1".into()),SqliteValue::Null]{
  let mut wrong=valid.clone();wrong.table_mut("fixture_counter").unwrap().rows[0].values[1]=value;cases.push(wrong);
 }
 let mut wrong=valid.clone();wrong.table_mut("fixture_counter").unwrap().rows[0].rowid=2;cases.push(wrong);
 let mut wrong=valid.clone();wrong.table_mut("fixture_counter").unwrap().rows[0].values[0]=SqliteValue::Integer(2);cases.push(wrong);
 cases.push(SqliteDatabase::from_schema("CREATE TABLE fixture_counter(id INTEGER PRIMARY KEY,count TEXT NOT NULL);").unwrap());
 let mut wrong=SqliteDatabase::from_schema(&(SQL.to_owned()+"CREATE TABLE foreign_owner(id INTEGER PRIMARY KEY);")).unwrap();wrong.table_mut("fixture_counter").unwrap().rows=valid.table("fixture_counter").unwrap().rows.clone();cases.push(wrong);
 for malformed in cases{for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits::default());
  assert_eq!(import_native(&codec,KIND,&dialect(),malformed.clone(),encoding,&mut control).unwrap_err().cause.kind,ValueRefusalKind::InvalidValue);
 }}
}
#[test]
fn sqlite_snapshot_host_count_full_erased_backing_limits_and_phase_cancellation_use_the_caller(){
 let codec=codec();let coordinate=dialect();let source=Snapshot{count:i32::MIN};let expected=database(source.count);
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let native=payload(&source,encoding);let defaults=SqliteDatabaseLimits::default();
  for reconstruct in[false,true]{
   let operation=|control:&mut SqliteSnapshotControl<'_>,input:&mut Option<SqliteDatabase>|{
    if reconstruct{import_native(&codec,KIND,&coordinate,input.take().unwrap(),encoding,control).map(|output|{drop(output.value);})}
    else{export_native(&codec,KIND,&coordinate,&native,control).map(|output|{drop(output.value);})}
   };
   let mut accept=|_|true;
   let mut control=SqliteSnapshotControl::new(&mut accept,defaults);
   let mut input=Some(expected.clone());
   let (result,requests)=crate::test_allocation::observe(||operation(&mut control,&mut input));result.unwrap();
   let admitted=defaults.max_allocation_bytes-control.allocation_remaining_bytes();drop(control);
   assert!(requests>0);assert_eq!(admitted,requests,"full real erased provider requests need exact cumulative admission: encoding={encoding:?} reconstruct={reconstruct}");
   for maximum in[requests,0,requests-1]{
    let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});
    let mut input=Some(expected.clone());let result=operation(&mut control,&mut input);
    if maximum==requests{result.unwrap();assert_eq!(control.allocation_remaining_bytes(),0);}
    else{assert_eq!(result.unwrap_err().cause.kind,ValueRefusalKind::OwnershipLimit);assert!(maximum-control.allocation_remaining_bytes()<requests);}
   }
   let maximum=requests.checked_mul(2).unwrap();let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});
   for _ in 0..2{let mut input=Some(expected.clone());operation(&mut control,&mut input).unwrap();}assert_eq!(control.allocation_remaining_bytes(),0);
   assert_eq!(operation(&mut control,&mut Some(expected.clone())).unwrap_err().cause.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  }
  for(limits,kind)in[(SqliteDatabaseLimits{max_rows:0,..defaults},ValueRefusalKind::WorkLimit),(SqliteDatabaseLimits{max_schema_bytes:SQL.len()-1,..defaults},ValueRefusalKind::OwnershipLimit)]{
   let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(export_native(&codec,KIND,&dialect(),&native,&mut control).unwrap_err().cause.kind,kind);
   let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);assert_eq!(import_native(&codec,KIND,&dialect(),expected.clone(),encoding,&mut control).unwrap_err().cause.kind,kind);
  }
  for phase in[SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::EncodeNative]{
   let mut reached=false;let mut cancel=|progress:SqliteSnapshotProgress|{let stop=progress.phase==phase;reached|=stop;!stop};
   let mut control=SqliteSnapshotControl::new(&mut cancel,defaults);
   let error=if matches!(phase,SqliteSnapshotPhase::DecodeNative|SqliteSnapshotPhase::ProjectSnapshot){export_native(&codec,KIND,&dialect(),&native,&mut control).unwrap_err()}else{import_native(&codec,KIND,&dialect(),expected.clone(),encoding,&mut control).unwrap_err()};
   assert_eq!(error.cause.kind,ValueRefusalKind::Canceled);drop(control);assert!(reached);
  }
  assert_eq!(source,Snapshot{count:i32::MIN});
 }
}
#[test]
fn sqlite_snapshot_host_count_actual_declared_plugin_wire_preserves_count_and_file_metadata(){
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

 let _codec=codec();let _plugin=crate::assembly::plugin().unwrap();
 use semio_framework_plugin::{plugin_runtime,sqlite_wire::{SnapshotInput,SnapshotLimits,SnapshotFileResult,SnapshotPayloadResult}};
 let coordinate=dialect().to_coordinate();
 assert_eq!(plugin_runtime::plugin_snapshot_sqlite_schema(&coordinate).unwrap(),SQL);
 let neutral:SnapshotInput=serde_json::from_str(include_str!("../../../../../../../🧬️schema/🪶️sqlite/🧫️fixtures/🔣️.json")).unwrap();
 let grant=neutral.native.native().unwrap();let limits=neutral.limits.native().unwrap();
 for count in counts(){let source=Snapshot{count};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut receive=|_|true;let mut publish=|_|true;let mut progress=|_|true;
  let mut decoder=semio_framework_value::NativeDecodeControl::new(grant.maximum_capacity_bytes,&mut receive);
  let mut encoder=semio_framework_value::NativeEncodeControl::new(grant.maximum_capacity_bytes,&mut publish);
  let mut native_control=semio_framework_os_kernel::io::io_mechanism::IoRunControl::new(&mut decoder,&mut encoder,grant);
  let mut snapshot_control=SqliteSnapshotControl::new(&mut progress,limits);
  let native=payload(&source,encoding);let payload=match native{store::io::IoPayload::Binary(bytes)=>bytes,store::io::IoPayload::Text(text)=>text.into_bytes()};
  let output=semio_framework_async::poll::resolve_ready(plugin_runtime::plugin_snapshot_sqlite_export(SnapshotInput{dialect:coordinate.clone(),encoding:encoding.as_str().into(),payload,limits:SnapshotLimits::from(limits),native:grant.into()},&mut native_control,&mut snapshot_control)).unwrap();
  let SnapshotFileResult::Done(file)=output else{panic!("actual published Count export must succeed")};assert_eq!(&file.bytes[..16],b"SQLite format 3\0");
  let database=import_sqlite_database(&file.bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  assert_eq!(store::io::io_mechanism::sqlite_snapshot_metadata(&database).unwrap(),(dialect(),encoding));
  let output=semio_framework_async::poll::resolve_ready(plugin_runtime::plugin_snapshot_sqlite_import(SnapshotInput{dialect:coordinate.clone(),encoding:encoding.as_str().into(),payload:file.bytes,limits:SnapshotLimits::from(limits),native:grant.into()},&mut native_control,&mut snapshot_control)).unwrap();
  let SnapshotPayloadResult::Done(output)=output else{panic!("actual published Count import must succeed")};assert_eq!(output.encoding,encoding.as_str());
  assert_eq!(decode(match encoding{SnapshotEncoding::Binary=>store::io::IoPayload::Binary(output.bytes),SnapshotEncoding::Text=>store::io::IoPayload::Text(String::from_utf8(output.bytes).unwrap())}),source);
 }}
}

#[path="💰️release/🦀️.rs"]
mod retained_release;
