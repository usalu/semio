use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue}, ArtifactSqliteSnapshot};

fn fixture() -> DeflateSnapshot { semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() }

#[test]
fn sqlite_snapshot_deflate_header_dictionary_and_decoded_payload_roundtrip() {
    let snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(database.table("deflate_payload_byte").unwrap().rows.len(), snapshot.payload.len());
    assert!(database.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).all(|value| !matches!(value, SqliteValue::Blob(_))));
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let restored = DeflateSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    assert_eq!(restored, snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&restored))).unwrap();
    assert_eq!(oracle, serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());
    let limits = SqliteDatabaseLimits { max_value_bytes: 0, ..SqliteDatabaseLimits::default() };
    assert!(DeflateSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
    for hint in 0..4 { let mut value = snapshot.clone(); value.compression_level_hint = DeflateLevelHint::from_bits(hint); value.compression_method = 15; value.window_bits = 15; value.dict_id = None; let database = value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(); assert_eq!(DeflateSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), value); }
}


#[test]
fn sqlite_snapshot_deflate_borrowed_native_preflight_is_bounded_cancellable_and_unowned(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase,ValueRefusalKind};
 let fixture=deflate_native_control_fixture();let policy=&fixture["preflight"];assert_eq!(policy["ownershipBytes"],0);assert_eq!(policy["retirementRefund"],false);
 let snapshot=deflate_canonical_carrier_snapshot();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=deflate_native_payload(&snapshot,encoding);let length=match &payload{store::io_schema::IoPayload::Binary(bytes)=>bytes.len(),store::io_schema::IoPayload::Text(text)=>text.len()};assert!(length>0);
  let raw_bound=snapshot.payload.len().checked_mul(9).and_then(|bits|bits.checked_add(17)).unwrap()/8;let zlib_bound=raw_bound.checked_add(6).unwrap();let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};let prefix=store::semio_format::declared_envelope_prefix_len("stdio.deflate",component,1).unwrap();let boundary=match encoding{SnapshotEncoding::Binary=>zlib_bound,SnapshotEncoding::Text=>zlib_bound.checked_mul(2).unwrap()}.checked_add(prefix).unwrap();assert!(boundary>=length,"authored fixed-Huffman bound must cover actual literal native output");
  let exact=SqliteDatabaseLimits{max_file_bytes:boundary,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,exact);
  snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).expect("actual owner preflight must admit its exact literal output with zero owned admission");assert_eq!(control.allocation_remaining_bytes(),0);
  let short=SqliteDatabaseLimits{max_file_bytes:boundary-1,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,short);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let rows=SqliteDatabaseLimits{max_rows:0,max_allocation_bytes:0,..Default::default()};let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,rows);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!(control.allocation_remaining_bytes(),0);
  let initial=usize::try_from(fixture["maxAllocationBytes"].as_u64().unwrap()).unwrap();let limits=SqliteDatabaseLimits{max_allocation_bytes:initial,..Default::default()};let mut reached=false;
  let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::EncodeNative{reached=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,limits);
  let error=snapshot.preflight_sqlite_snapshot_encoding(encoding,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!(control.allocation_remaining_bytes(),initial);drop(control);assert!(reached,"actual borrowed preflight cancellation checkpoint required");
 }
}

#[test]
fn sqlite_snapshot_deflate_independent_decoded_payload_edit() {
    use std::{io::Write, process::{Command, Stdio}};
    let mut snapshot = fixture();
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('SELECT d.dictionary_adler32,b.value FROM deflate_document d JOIN deflate_payload_byte b ON b.document_id=d.id WHERE b.ordinal=2').get();if(row.dictionary_adler32!==4294967295||row.value!==255)throw Error('payload');db.query('UPDATE deflate_payload_byte SET value=42 WHERE ordinal=2').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    snapshot.payload[2] = 42;
    assert_eq!(DeflateSnapshot::from_sqlite_database(&edited, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), snapshot);
}

fn deflate_native_control_fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🛬️native-control/🔣️.json")).unwrap()}
fn deflate_native_control_snapshot()->DeflateSnapshot{let input=deflate_native_control_fixture();let count=usize::try_from(input["workItems"].as_u64().unwrap()).unwrap();let pattern=input["bytePattern"].as_array().unwrap().iter().map(|v|u8::try_from(v.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();DeflateSnapshot{schema:input["ownedSchema"].as_str().unwrap().into(),compression_method:u8::try_from(input["compressionMethod"].as_u64().unwrap()).unwrap(),window_bits:u8::try_from(input["windowBits"].as_u64().unwrap()).unwrap(),compression_level_hint:DeflateLevelHint::Maximum,dict_id:Some(u32::try_from(input["dictionaryAdler32"].as_u64().unwrap()).unwrap()),payload:(0..count).map(|i|pattern[i%4]).collect()}}
fn deflate_native_payload(snapshot:&DeflateSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io_schema::IoPayload{match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(<DeflateSnapshot as store::ArtifactPack>::encode_pack(snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(<DeflateSnapshot as store::ArtifactDsl>::print_dsl(snapshot))}}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_deflate_actual_factory_canonical_external_carrier(){
 use semio_framework_os_kernel::io::ArtifactDialect;
 use store::sqlite_snapshot::SnapshotEncoding;
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Deflate full owned SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let codec=store::document_codec(crate::STDIO_DEFLATE_DOCUMENT_SCHEMA).await.unwrap().unwrap();let provider=codec.snapshot_sqlite.as_ref().expect("actual owning declaration publishes semantic SQLite");assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<DeflateSnapshot>()));
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.deflate".into(),standard:"rfc1950".into(),subset:"*".into()};let snapshot=deflate_canonical_carrier_snapshot();let expected=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=deflate_native_payload(&snapshot,encoding);let database=(provider.export)(&codec.schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;assert_eq!(database,expected);let payload=(provider.import)(&codec.schema,&dialect,database,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;let restored=<DeflateSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(restored,snapshot);}
}
#[test]
fn sqlite_snapshot_deflate_native_input_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=deflate_native_control_fixture();let mut snapshot=deflate_native_control_snapshot();snapshot.compression_method=u8::try_from(input["nativeInput"]["compressionMethod"].as_u64().unwrap()).unwrap();snapshot.window_bits=u8::try_from(input["nativeInput"]["windowBits"].as_u64().unwrap()).unwrap();assert_eq!(input["nativeInput"]["dictionaryPresent"],false);let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let dictionary_payload=deflate_native_payload(&snapshot,encoding);assert_eq!(<DeflateSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&dictionary_payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::UnsupportedOwner);if let store::io_schema::IoPayload::Binary(bytes)=dictionary_payload{let(_,framed)=store::semio_format::unwrap_binary(&bytes).unwrap();assert_ne!(framed[1]&32,0);assert_eq!(u32::from_be_bytes(framed[2..6].try_into().unwrap()),u32::MAX);}}
 snapshot.dict_id=None;
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=deflate_native_payload(&snapshot,encoding);if let store::io_schema::IoPayload::Binary(bytes)=&payload{let(_,framed)=store::semio_format::unwrap_binary(bytes).unwrap();assert_eq!(framed[0]&15,8);assert_eq!(framed[0]>>4,7);assert_eq!(framed[1]&32,0);assert_eq!(miniz_oxide::inflate::decompress_to_vec_zlib(&framed).unwrap(),snapshot.payload);}assert_eq!(<DeflateSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
  let mut observed=false;let result=<DeflateSnapshot as ArtifactSqliteSnapshot>::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=after&&(event.total==0||event.completed<event.total){observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native decode cancellation required");

 }
}
#[test]
fn sqlite_snapshot_deflate_long_schema_projection_and_reconstruction_are_interior_controlled(){
 use store::sqlite_snapshot::SqliteSnapshotPhase;
 let input=deflate_native_control_fixture();let mut snapshot=deflate_native_control_snapshot();snapshot.schema="x".repeat(usize::try_from(input["copyBytes"].as_u64().unwrap()).unwrap());let after=usize::try_from(input["copyCancelAfter"].as_u64().unwrap()).unwrap();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 for phase in [SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot]{let mut observed=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.phase==phase&&event.completed>=after&&(event.total==0||event.completed<event.total){observed=true;return false}true};let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());let result=if phase==SqliteSnapshotPhase::ProjectSnapshot{snapshot.to_sqlite_database(&mut control).map(|_|())}else{DeflateSnapshot::from_sqlite_database(&database,&mut control).map(|_|())};assert!(result.is_err(),"long owned schema copied without interior cancellation");assert!(observed);}
}

#[test]
fn sqlite_snapshot_deflate_native_output_interior_and_caller_ceilings(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let input=deflate_native_control_fixture();let snapshot=deflate_canonical_carrier_snapshot();let after=usize::try_from(input["cancelAfter"].as_u64().unwrap()).unwrap();let low=SqliteDatabaseLimits{max_allocation_bytes:usize::try_from(input["maxAllocationBytes"].as_u64().unwrap()).unwrap(),..Default::default()};
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let actual=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(actual,deflate_native_payload(&snapshot,encoding),"controlled RFC1950 bytes must match the actual owning codec");
  if let store::io_schema::IoPayload::Binary(bytes)=&actual{let (_,zlib)=store::semio_format::unwrap_binary(bytes).unwrap();assert_eq!(miniz_oxide::inflate::decompress_to_vec_zlib(&zlib).unwrap(),snapshot.payload);}
  let mut observed=false;let result=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::EncodeNative&&event.completed>=after&&(event.total==0||event.completed<event.total){observed=true;return false}true},Default::default()));assert!(result.is_err());assert!(observed,"actual interior native encode cancellation required");
  let rows=SqliteDatabaseLimits{max_rows:usize::try_from(input["lowRows"].as_u64().unwrap()).unwrap(),..Default::default()};assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,rows)).is_err());assert_eq!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,low)).unwrap_err().kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit);
 }
}

fn deflate_canonical_carrier_snapshot()->DeflateSnapshot{let mut snapshot=deflate_native_control_snapshot();snapshot.schema=crate::STDIO_DEFLATE_DOCUMENT_SCHEMA.into();snapshot.compression_method=8;snapshot.window_bits=7;snapshot.dict_id=None;snapshot}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_deflate_actual_typed_file_preserves_complete_literal_owned_fields(){
 use semio_framework_os_kernel::io::{ArtifactDialect,io_mechanism::{io_export_sqlite_snapshot,io_import_sqlite_snapshot}};
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Deflate typed SQLite").version("0.0.1").package_id("semio:stdio").artifact(crate::declaration(crate::definition().unwrap()).unwrap()).try_build().unwrap();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.deflate".into(),standard:"rfc1950".into(),subset:"*".into()};
 let snapshot=deflate_native_control_snapshot();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 assert_eq!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>(),usize::try_from(deflate_native_control_fixture()["expectedRows"].as_u64().unwrap()).unwrap());
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,encoding,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(&file[..16],b"SQLite format 3\0");let restored=io_import_sqlite_snapshot::<DeflateSnapshot>(&dialect,&file,Default::default(),&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(restored,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::DecodeNative|SqliteSnapshotPhase::EncodeNative)));}
 println!("[DEBUG] Deflate actual declaration typed SQLite files preserved complete literal fields in both metadata encodings without native lowering");
}
#[test]
fn sqlite_snapshot_deflate_external_carrier_normalizes_unrepresented_schema(){
 use store::sqlite_snapshot::SnapshotEncoding;
 let mut snapshot=deflate_native_control_snapshot();snapshot.compression_method=8;snapshot.window_bits=7;
 for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=deflate_native_payload(&snapshot,encoding);let normal:DeflateSnapshot=match &input{store::io_schema::IoPayload::Binary(bytes)=><DeflateSnapshot as store::ArtifactPack>::decode_pack(bytes).unwrap(),store::io_schema::IoPayload::Text(text)=><DeflateSnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap()};assert_eq!(normal.schema,crate::STDIO_DEFLATE_DOCUMENT_SCHEMA);assert_ne!(normal.schema,snapshot.schema);assert_eq!(deflate_native_payload(&normal,encoding),input);}
}
