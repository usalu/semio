//! 🫴️ Original Probe IO laws retain refused cursors under independently authored grants.
use super::*;
use std::cell::Cell;
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress};
use store::{ArtifactSqliteSnapshot as _,ArtifactDsl as _};
fn contract()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn grant(value:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap()as usize}}
fn close_decode(native:&mut NativeDecodeControl<'_>,law:&serde_json::Value){let closing=grant(&law["closeGrant"]);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){return}let step=native.close_retirement_recipient(closing).unwrap();assert!(step.progress().fits(closing));}panic!("Probe original decode recipient did not reach terminal")}
fn close_encode(native:&mut NativeEncodeControl<'_>,law:&serde_json::Value){let closing=grant(&law["closeGrant"]);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){return}let step=native.close_retirement_recipient(closing).unwrap();assert!(step.progress().fits(closing));}panic!("Probe original encode recipient did not reach terminal")}

#[test]
fn probe_sqlite_original_native_io_roundtrips_both_encodings(){
 let law=contract();let incoming=grant(&law["bodyGrant"]);let maximum=law["nativeMaximumBytes"].as_u64().unwrap()as usize;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for sample in cases["cases"].as_array().unwrap(){for encoding in [SnapshotEncoding::Text,SnapshotEncoding::Binary]{
  let wire=sample["wire"].as_str().unwrap();let oracle:serde_json::Value=serde_json::from_str(wire).unwrap();
  let payload=match encoding{SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(wire.to_string()),SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(wire.as_bytes().to_vec())};
  let original=match &payload{store::io_schema::IoPayload::Text(text)=>text.as_ptr(),store::io_schema::IoPayload::Binary(bytes)=>bytes.as_ptr()};
  let mut accepted=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(maximum,&mut accepted);native.install_retirement_recipient(&mut recipient).unwrap();
  let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());
  let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,incoming);let decoded=super::super::ProbeSnapshot::decode_sqlite_snapshot_native(&payload,&mut control,&mut owner).unwrap();
  assert!(owner.progress().fits(incoming));assert!(owner.progress().copied_bytes>0);assert!(owner.progress().retained_capacity_bytes>0);drop(owner);assert!(!native.has_retirement_owner());
  assert_eq!(match &payload{store::io_schema::IoPayload::Text(text)=>text.as_ptr(),store::io_schema::IoPayload::Binary(bytes)=>bytes.as_ptr()},original);
  assert_eq!(serde_json::Value::from(decoded.0.clone()),oracle);
  let mut accepted=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(maximum,&mut accepted);native.install_retirement_recipient(&mut recipient).unwrap();
  let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());
  let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,incoming);let output=decoded.encode_sqlite_snapshot_native(encoding,&mut control,&mut owner).unwrap();assert!(owner.progress().fits(incoming));assert!(owner.progress().copied_bytes>0);drop(owner);assert!(!native.has_retirement_owner());
  let text=match output{store::io_schema::IoPayload::Text(text)=>text,store::io_schema::IoPayload::Binary(bytes)=>String::from_utf8(bytes).unwrap()};assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(),oracle);
 }}
 eprintln!("[DEBUG] Probe original Native Decode and Encode preserve ordered first-party semantics, borrowed source backing and exact original physical receipts in Text and Binary");
}

#[test]
fn probe_sqlite_original_native_refusals_keep_recipient_and_actual_receipts(){
 let law=contract();let full=grant(&law["bodyGrant"]);let maximum=law["nativeMaximumBytes"].as_u64().unwrap()as usize;
 for row in law["refusals"].as_array().unwrap(){
  let mut incoming=full;let value=row["value"].as_u64().unwrap()as usize;match row["axis"].as_str().unwrap(){"work"=>incoming.maximum_items=value,"copy"=>incoming.maximum_copy_bytes=value,"capacity"=>incoming.maximum_capacity_bytes=value,"release"=>incoming.maximum_release_bytes=value,"depth"=>incoming.maximum_depth=value,_=>unreachable!()}
  let mut accepted=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(maximum,&mut accepted);native.install_retirement_recipient(&mut recipient).unwrap();
  let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,incoming);let error=super::super::probe_json::read("\"λ🙂\"",&mut owner).unwrap_err();let performed=owner.progress();assert!(performed.fits(incoming));assert_eq!(error.kind.as_str(),row["kind"].as_str().unwrap());assert_eq!(error.retained_progress(),performed);drop(owner);
  let before=native.owned_bytes();let held=native.has_retirement_owner();let step=native.close_retirement_recipient(grant(&law["deniedCloseGrant"])).unwrap();assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(native.owned_bytes(),before);assert_eq!(native.has_retirement_owner(),held);close_decode(&mut native,&law);
 }
 for entry in [&law["cancellation"],&law["duplicate"]]{
  let cancellation=entry.get("afterCheckpoints").is_some();let armed=Cell::new(cancellation);let calls=Cell::new(0usize);let stop=entry["afterCheckpoints"].as_u64().unwrap_or(u64::MAX)as usize;
  let mut callback=|_|{calls.set(calls.get()+1);!armed.get()||calls.get()<stop};let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(maximum,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();
  let source=entry["source"].as_str().unwrap();let original=source.as_ptr();let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,full);let error=super::super::probe_json::read(source,&mut owner).unwrap_err();let performed=owner.progress();assert_eq!(error.kind.as_str(),entry["kind"].as_str().unwrap());assert_eq!(error.retained_progress(),performed);assert!(performed.fits(full));assert!(performed.retained_capacity_bytes>0);drop(owner);assert!(native.has_retirement_owner());assert_eq!(source.as_ptr(),original);armed.set(false);
  let before=native.owned_bytes();let step=native.close_retirement_recipient(grant(&law["deniedCloseGrant"])).unwrap();assert_eq!(step.progress(),RetainedCloneProgress::default());assert!(native.has_retirement_owner());assert_eq!(native.owned_bytes(),before);close_decode(&mut native,&law);assert!(!native.has_retirement_owner());
 }
 eprintln!("[DEBUG] Probe original five-axis refusals, canceled partial tree and original duplicate diagnostic retain exact receipts and unchanged denied-close recipient until funded terminal");
}

#[test]
fn probe_sqlite_original_file_ceilings_preserve_original_refusal_custody(){
 let law=contract();let incoming=grant(&law["bodyGrant"]);let maximum=law["nativeMaximumBytes"].as_u64().unwrap()as usize;let limit=law["maximumFileBytes"].as_u64().unwrap()as usize;
 let snapshot=super::super::ProbeSnapshot::parse_dsl("\"λ🙂\"").unwrap();let payload=store::io_schema::IoPayload::Text("\"λ🙂\"".to_string());let limits=store::sqlite_snapshot::SqliteDatabaseLimits{max_file_bytes:limit,..Default::default()};
 let mut callback=|_|true;let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(maximum,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut accepted=|_|true;let mut control=SqliteSnapshotControl::new(&mut accepted,limits);
 let mut owner=store::NativeSnapshotDecodeOwner::new(&mut native,incoming);let error=super::super::ProbeSnapshot::decode_sqlite_snapshot_native(&payload,&mut control,&mut owner).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(owner.progress(),Default::default());drop(owner);assert_eq!(native.owned_bytes(),0);assert!(!native.has_retirement_owner());
 let mut callback=|_|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(maximum,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut accepted=|_|true;let mut control=SqliteSnapshotControl::new(&mut accepted,limits);
 let mut owner=store::NativeSnapshotEncodeOwner::new(&mut native,incoming);let error=snapshot.encode_sqlite_snapshot_native(SnapshotEncoding::Binary,&mut control,&mut owner).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(error.retained_progress(),owner.progress());assert!(owner.progress().fits(incoming));drop(owner);assert!(native.has_retirement_owner());let before=native.owned_bytes();let denied=native.close_retirement_recipient(grant(&law["deniedCloseGrant"])).unwrap();assert_eq!(denied.progress(),Default::default());assert_eq!(native.owned_bytes(),before);assert!(native.has_retirement_owner());close_encode(&mut native,&law);
 eprintln!("[DEBUG] Probe original file ceiling refuses oversized decode before birth and preserves the genuine refused encoding cursor until funded retirement");
}
