//! 🪆️ Nested IO loans retain original pending frames and settle accepted native spans exactly once.
use super::*;
use semio_framework_value::{native_decoding::NativeDecodeRetirementRecipient,native_encoding::NativeEncodeRetirementRecipient};
use std::cell::Cell;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("nested-io.json")).unwrap()}
#[test]
fn original_receiving_nested_io_decode_retains_child_ticket_and_same_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut source=Some(f["original"].as_str().unwrap().to_owned());let backing=source.as_ref().unwrap().capacity();let live=Cell::new(true);let mut observer=|_|live.get();let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(f["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();native.charge(backing).unwrap();let mut run=IoRunControl::decoding(&mut native,grant);
 let(result,born,released)=crate::test_allocation::observe_backing(||run.receive_nested::<(),()>(|_,run|run.receive_nested::<String,()>(|slot,child|{*slot=source.take();child.decode()?.checkpoint()?;live.set(false);Ok(())})));
 assert_eq!(result.err().unwrap().kind,ValueRefusalKind::Canceled);assert!(source.is_none());let progress=run.progress();assert_eq!(progress.copied_items,f["expected"]["nestedFrames"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,born);assert_eq!(released,0);drop(run);assert!(native.has_retirement_owner());live.set(true);let(step,allocated,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(Default::default()).unwrap());assert_eq!((step.progress(),allocated,released),(Default::default(),0,0));let mut total=0;let mut extra=0;for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,allocated,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(grant).unwrap());assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));extra+=allocated;total+=released;}assert!(!native.has_retirement_owner());assert_eq!(total,backing+born+extra);eprintln!("[DEBUG] Nested decoding IO same pending child and cumulative wallet: two real frames, cancellation retained, zero close unchanged, exact physical conservation");
}
#[test]
fn original_receiving_nested_io_encode_retains_child_ticket_and_same_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut source=Some(f["original"].as_str().unwrap().to_owned());let backing=source.as_ref().unwrap().capacity();let live=Cell::new(true);let mut observer=|_|live.get();let mut recipient=NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(f["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();native.charge(backing).unwrap();let mut run=IoRunControl::encoding(&mut native,grant);
 let(result,born,released)=crate::test_allocation::observe_backing(||run.receive_nested::<(),()>(|_,run|run.receive_nested::<String,()>(|slot,child|{*slot=source.take();child.encode()?.checkpoint()?;live.set(false);Ok(())})));
 assert_eq!(result.err().unwrap().kind,ValueRefusalKind::Canceled);assert!(source.is_none());let progress=run.progress();assert_eq!(progress.copied_items,f["expected"]["nestedFrames"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,born);assert_eq!(released,0);drop(run);assert!(native.has_retirement_owner());live.set(true);let(step,allocated,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(Default::default()).unwrap());assert_eq!((step.progress(),allocated,released),(Default::default(),0,0));let mut total=0;let mut extra=0;for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,allocated,released)=crate::test_allocation::observe_backing(||native.close_retirement_recipient(grant).unwrap());assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));extra+=allocated;total+=released;}assert!(!native.has_retirement_owner());assert_eq!(total,backing+born+extra);eprintln!("[DEBUG] Nested encoding IO same pending child and cumulative wallet: two real frames, cancellation retained, zero close unchanged, exact physical conservation");
}

#[test]
fn original_receiving_nested_io_dual_control_phase_change_refuses_without_foreign_ledger_work(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();
 for direction in [IoNativeDirection::Decode,IoNativeDirection::Encode]{
  let mut decode_observer=|_|true;let mut encode_observer=|_|true;
  let mut decode_recipient=NativeDecodeRetirementRecipient::new();let mut encode_recipient=NativeEncodeRetirementRecipient::new();
  let mut decode=NativeDecodeControl::new(1048576,&mut decode_observer);let mut encode=NativeEncodeControl::new(1048576,&mut encode_observer);
  decode.install_retirement_recipient(&mut decode_recipient).unwrap();encode.install_retirement_recipient(&mut encode_recipient).unwrap();
  let mut run=IoRunControl::new(&mut decode,&mut encode,grant);run.select(direction).unwrap();
  let (result,born,released)=crate::test_allocation::observe_backing(||run.receive_nested::<(),()>(|_,child|{
   let opposite=if direction==IoNativeDirection::Decode{IoNativeDirection::Encode}else{IoNativeDirection::Decode};
   let before=child.progress();let(refused,birth,release)=crate::test_allocation::observe_backing(||child.select(opposite));
   assert_eq!(refused.unwrap_err().kind,ValueRefusalKind::UnsupportedOwner);assert_eq!((birth,release),(0,0));assert_eq!(child.direction(),direction);assert_eq!(child.progress(),before);Ok(())
  }));result.unwrap();assert_eq!(born,run.progress().retained_capacity_bytes);assert_eq!(released,run.progress().released_bytes);assert_eq!(born,released);drop(run);
  assert!(!decode.has_retirement_owner());assert!(!encode.has_retirement_owner());assert_eq!(decode.owned_bytes(),if direction==IoNativeDirection::Decode{born}else{0});assert_eq!(encode.owned_bytes(),if direction==IoNativeDirection::Encode{born}else{0});
 }
 eprintln!("[DEBUG] Genuine dual native IO nested admission refuses opposite phase with heap0 unchanged direction/wallet and original ledger physical frame conservation");
}


#[test]
fn original_snapshot_wallet_retains_actual_faulty_child_backing_before_refusal(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../🛫️snapshot/🧪️tests/🧫️fixtures/🧾️overdraw.json")).unwrap();
 let policy:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();
 let(bytes,born,released)=crate::test_allocation::observe_backing(||Vec::<u8>::with_capacity(policy.maximum_capacity_bytes*2+1));
 assert_eq!(released,0);assert_eq!(born,bytes.capacity());let pointer=bytes.as_ptr();
 let actual=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:born,..Default::default()};assert!(!actual.fits(policy));
 let mut body=NativeSnapshotBodyWallet::new(policy);let error=body.record_progress(actual).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(error.retained_progress(),actual);assert_eq!(body.progress(),actual);assert_eq!(body.remaining_grant().maximum_capacity_bytes,0);assert_eq!(body.remaining_grant().maximum_copy_bytes,policy.maximum_copy_bytes);
 let mut original=|_|true;let mut native=NativeDecodeControl::new(1048576,&mut original);let mut owner=NativeSnapshotDecodeOwner::new(&mut native,policy);let error=owner.record_progress(actual).unwrap_err();assert_eq!(error.retained_progress(),actual);assert_eq!(owner.progress(),actual);assert_eq!(owner.grant(),policy);assert_eq!(owner.remaining_grant().maximum_capacity_bytes,0);assert!(owner.record_progress(Default::default()).is_err());assert_eq!(owner.progress(),actual);drop(owner);
 let mut original=|_|true;let mut native=NativeEncodeControl::new(1048576,&mut original);let mut owner=NativeSnapshotEncodeOwner::new(&mut native,policy);let error=owner.record_progress(actual).unwrap_err();assert_eq!(error.retained_progress(),actual);assert_eq!(owner.progress(),actual);assert_eq!(owner.grant(),policy);assert_eq!(owner.remaining_grant().maximum_capacity_bytes,0);drop(owner);
 assert_eq!((bytes.as_ptr(),bytes.len(),bytes.capacity()),(pointer,0,born));
 let close=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
 let mut retained=semio_framework_value::retirement::controlled::ControlledRetirement::new(bytes).ok().unwrap();let mut released=0;let mut allocated=0;
 for _ in 0..4096{if retained.terminal_is_empty(){break}let(step,born,freed)=crate::test_allocation::observe_backing(||retained.step(close).unwrap());assert!(step.progress().fits(close));assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));allocated+=born;released+=freed;}
 assert!(retained.terminal_is_empty());assert_eq!(released,born+allocated);
 eprintln!("[DEBUG] Actual faulty child System backing remains at same pointer while body/decode/encode wallets refuse original authority, retain exact performed receipt and exhaust only spent capacity; independent fixed close conserved backing");
}
