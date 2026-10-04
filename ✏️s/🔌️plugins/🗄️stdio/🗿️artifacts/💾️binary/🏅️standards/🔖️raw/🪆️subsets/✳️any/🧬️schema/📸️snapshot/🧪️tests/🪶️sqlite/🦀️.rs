use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

#[test]
fn sqlite_snapshot_binary_reconstruction_respects_value_budget() {
    let database = BinarySnapshot::default().to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(BinarySnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
}


#[test]
fn sqlite_snapshot_binary_borrowed_native_preflight_is_exact_cancellable_and_unowned(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase,ValueRefusalKind};
 let fixture=binary_native_control_fixture();let policy=&fixture["preflight"];assert_eq!(policy["ownershipBytes"],0);assert_eq!(policy["retirementRefund"],false);
 let snapshot=binary_canonical_carrier_snapshot();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=binary_native_payload(&snapshot,encoding);let length=match &payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};assert!(length>0);
  let exact=SqliteDatabaseLimits{max_file_bytes:length,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,exact);
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).expect("actual owner preflight must admit its exact literal output with zero owned admission");assert_eq!(control.allocation_remaining_bytes(),0);
  let short=SqliteDatabaseLimits{max_file_bytes:length-1,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,short);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let rows=SqliteDatabaseLimits{max_rows:0,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,rows);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let initial=usize::try_from(fixture["maxAllocationBytes"].as_u64().unwrap()).unwrap();let limits=SqliteDatabaseLimits{max_allocation_bytes:initial,..Default::default()};let mut reached=false;
  let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,limits);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!(control.allocation_remaining_bytes(),initial);drop(control);assert!(reached,"actual borrowed preflight cancellation checkpoint required");
 }
}

#[test]
fn sqlite_snapshot_binary_bytes_are_queryable_ordered_integers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let snapshot = BinarySnapshot { schema: fixture["schema"].as_str().unwrap().into(), bytes: fixture["bytes"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect() };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("binary_byte").unwrap().rows.len(), 6);
    assert_eq!(database.table("binary_byte").unwrap().rows[4].values[3], SqliteValue::Integer(255));
    assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let reopened = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    assert_eq!(BinarySnapshot::from_sqlite_database(&reopened, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
    assert_eq!(serde_json::to_value(&snapshot.bytes).unwrap(), fixture["bytes"]);
    let mut broken = reopened;
    broken.table_mut("binary_byte").unwrap().rows[0].values[3] = SqliteValue::Integer(256);
    assert!(BinarySnapshot::from_sqlite_database(&broken, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

fn binary_native_control_fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🛬️native-control/🔣️.json")).unwrap()}
fn binary_native_control_snapshot()->BinarySnapshot{let input=binary_native_control_fixture();let count=usize::try_from(input["workItems"].as_u64().unwrap()).unwrap();let pattern=input["bytePattern"].as_array().unwrap().iter().map(|v|u8::try_from(v.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();BinarySnapshot{schema:input["ownedSchema"].as_str().unwrap().into(),bytes:(0..count).map(|i|pattern[i%4]).collect()}}
fn binary_native_payload(snapshot:&BinarySnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io_schema::IoPayload{match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(<BinarySnapshot as store::ArtifactPack>::encode_pack(snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(<BinarySnapshot as store::ArtifactDsl>::print_dsl(snapshot))}}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_binary_actual_factory_canonical_external_carrier(){
 use semio_framework_os_kernel::io::ArtifactDialect;
 use store::sqlite_snapshot::SnapshotEncoding;
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Binary full owned SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let codec=store::document_codec(crate::STDIO_BINARY_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().expect("actual owning declaration publishes semantic SQLite");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<BinarySnapshot>()));
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.binary".into(),standard:"raw".into(),subset:"*".into()};let snapshot=binary_canonical_carrier_snapshot();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=binary_native_payload(&snapshot,encoding);let database=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;assert_eq!(database,expected);let payload=(provider.import)(&codec.schema,&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;let restored=<BinarySnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(restored,snapshot);}
}
#[test]
fn sqlite_snapshot_binary_native_input_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=binary_native_control_fixture();let snapshot=binary_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=binary_native_payload(&snapshot,encoding);assert_eq!(<BinarySnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
  let mut observed=false;let result=<BinarySnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native decode cancellation required");

 }
}
#[test]
fn sqlite_snapshot_binary_long_schema_projection_and_reconstruction_are_interior_controlled(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let input=binary_native_control_fixture();let mut snapshot=binary_native_control_snapshot();snapshot.schema="x".repeat(usize::try_from(input["copyBytes"].as_u64().unwrap()).unwrap());let after=usize::try_from(input["copyCancelAfter"].as_u64().unwrap()).unwrap();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for phase in [SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut observed=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=after&&event.completed<event.total{observed=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let result=if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).map(|_|())}else{BinarySnapshot::from_sqlite_database(&database,&mut control).map(|_|())};assert!(result.is_err(),"long owned schema copied without interior cancellation");assert!(observed);}
}

#[test]
fn sqlite_snapshot_binary_native_output_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=binary_native_control_fixture();let snapshot=binary_native_control_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut observed=false;let result=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=after&&event.completed<event.total{observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native encode cancellation required");
  let rows=SqliteDatabaseLimits{max_rows:usize::try_from(input["lowRows"].as_u64().unwrap()).unwrap(),..Default::default()};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,rows)).is_err());assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
 }
}

fn binary_canonical_carrier_snapshot()->BinarySnapshot{let mut snapshot=binary_native_control_snapshot();snapshot.schema=crate::STDIO_BINARY_DOCUMENT_SCHEMA.into();snapshot}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_binary_actual_typed_file_preserves_complete_literal_owned_fields(){
 use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Binary typed SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.binary".into(),standard:"raw".into(),subset:"*".into()};
 let snapshot=binary_native_control_snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 assert_eq!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>(),usize::try_from(binary_native_control_fixture()["expectedRows"].as_u64().unwrap()).unwrap());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(&file[..16],b"SQLite format 3\0");let restored=io_import_sqlite_snapshot::<BinarySnapshot>(&dialect,&file,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(restored,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::DecodeNative|SqliteSnapshotPhase::EncodeNative)));}
 println!("[DEBUG] Binary actual declaration typed SQLite files preserved complete literal fields in both metadata encodings without native lowering");
}
#[test]
fn sqlite_snapshot_binary_external_carrier_normalizes_unrepresented_schema(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let snapshot=binary_native_control_snapshot();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=binary_native_payload(&snapshot,encoding);let normal:BinarySnapshot=match &input{store::io_schema::IoPayload::Binary(bytes)=><BinarySnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(),store::io_schema::IoPayload::Text(text)=><BinarySnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap()};assert_eq!(normal.schema,crate::STDIO_BINARY_DOCUMENT_SCHEMA);assert_ne!(normal.schema,snapshot.schema);assert_eq!(binary_native_payload(&normal,encoding),input);}
}
