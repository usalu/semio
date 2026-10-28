//! 🪆️ Nested IO loans retain original pending frames and settle accepted native spans exactly once.
use super::*;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use std::cell::Cell;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("nested-io.json")).unwrap()}
#[test]
fn empty_nested_pending_slot_preserves_both_original_phase_controls(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let decode_calls=Cell::new(0);let encode_calls=Cell::new(0);let mut receive=|_|{decode_calls.set(decode_calls.get()+1);true};let mut publish=|_|{encode_calls.set(encode_calls.get()+1);true};let mut decode=NativeDecodeControl::new(1048576,&mut receive);let ledger=decode.ledger_identity();let mut encode=NativeEncodeControl::new(1048576,&mut publish);let mut wallet=RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()};let initial=wallet;let mut pending=None;
 {let mut run=IoRunControl::borrowed_native(&mut decode,&mut encode,grant,&mut wallet,Some(&mut pending),IoNativeDirection::Decode);let remaining=run.remaining_grant();run.select(IoNativeDirection::Encode).unwrap();run.encode().unwrap().checkpoint().unwrap();run.select(IoNativeDirection::Decode).unwrap();assert_eq!(run.decode().unwrap().ledger_identity(),ledger);assert_eq!(run.remaining_grant(),remaining);}
 assert_eq!(wallet,initial);assert!(pending.is_none());assert!(decode_calls.get()>0&&encode_calls.get()>0);
}
#[test]
fn occupied_nested_pending_owner_blocks_phase_switch_without_replacing_it(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut receive=|_|true;let mut publish=|_|true;let mut decode=NativeDecodeControl::new(1048576,&mut receive);let mut encode=NativeEncodeControl::new(1048576,&mut publish);let mut wallet=RetainedCloneProgress::default();let mut pending:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>=Some(Box::new(semio_framework_value::retirement::controlled::ControlledRetirement::new(()).ok().unwrap()));let pointer=pending.as_ref().unwrap().as_ref()as*const _ as*const()as usize;
 {let mut run=IoRunControl::borrowed_native(&mut decode,&mut encode,grant,&mut wallet,Some(&mut pending),IoNativeDirection::Decode);assert_eq!(run.select(IoNativeDirection::Encode).unwrap_err().kind,ValueRefusalKind::UnsupportedOwner);assert_eq!(run.direction(),IoNativeDirection::Decode);}
 assert_eq!(pending.as_ref().unwrap().as_ref()as*const _ as*const()as usize,pointer);while pending.is_some(){semio_framework_value::close_factory_ticket(&mut pending,grant).unwrap();}
}
#[test]
fn actual_nested_receiving_frame_can_finish_decode_and_select_original_encoder(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut receive=|_|true;let encoded=Cell::new(false);let mut publish=|_|{encoded.set(true);true};let mut recipient=NativeDecodeRetirementRecipient::new();let mut decode=NativeDecodeControl::new(1048576,&mut receive);decode.install_retirement_recipient(&mut recipient).unwrap();let mut encode=NativeEncodeControl::new(1048576,&mut publish);let mut run=IoRunControl::new(&mut decode,&mut encode,grant);
 run.receive_nested::<(),()>(|_,child|{child.decode()?.checkpoint()?;child.encode()?.checkpoint()?;Ok(())}).unwrap();assert!(encoded.get());
}
#[test]
fn original_receiving_nested_io_decode_retains_child_ticket_and_same_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut source=Some(f["original"].as_str().unwrap().to_owned());let backing=source.as_ref().unwrap().capacity();let live=Cell::new(true);let mut observer=|_|live.get();let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(f["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();native.charge(backing).unwrap();let mut run=IoRunControl::decoding(&mut native,grant);
 let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||run.receive_nested::<(),()>(|_,run|run.receive_nested::<String,()>(|slot,child|{*slot=source.take();child.decode()?.checkpoint()?;live.set(false);Ok(())})));
 assert_eq!(result.err().unwrap().kind,ValueRefusalKind::Canceled);assert!(source.is_none());let progress=run.progress();assert_eq!(progress.copied_items,f["expected"]["nestedFrames"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,born);assert_eq!(released,0);drop(run);assert!(native.has_retirement_owner());live.set(true);let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||native.close_retirement_recipient(Default::default()).unwrap());assert_eq!((step.progress(),allocated,released),(Default::default(),0,0));let mut total=0;let mut extra=0;for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||native.close_retirement_recipient(grant).unwrap());assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));extra+=allocated;total+=released;}assert!(!native.has_retirement_owner());assert_eq!(total,backing+born+extra);
}
#[test]
fn original_receiving_nested_io_encode_retains_child_ticket_and_same_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut source=Some(f["original"].as_str().unwrap().to_owned());let backing=source.as_ref().unwrap().capacity();let live=Cell::new(true);let mut observer=|_|live.get();let mut recipient=NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(f["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();native.charge(backing).unwrap();let mut run=IoRunControl::encoding(&mut native,grant);
 let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||run.receive_nested::<(),()>(|_,run|run.receive_nested::<String,()>(|slot,child|{*slot=source.take();child.encode()?.checkpoint()?;live.set(false);Ok(())})));
 assert_eq!(result.err().unwrap().kind,ValueRefusalKind::Canceled);assert!(source.is_none());let progress=run.progress();assert_eq!(progress.copied_items,f["expected"]["nestedFrames"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,born);assert_eq!(released,0);drop(run);assert!(native.has_retirement_owner());live.set(true);let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||native.close_retirement_recipient(Default::default()).unwrap());assert_eq!((step.progress(),allocated,released),(Default::default(),0,0));let mut total=0;let mut extra=0;for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||native.close_retirement_recipient(grant).unwrap());assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));extra+=allocated;total+=released;}assert!(!native.has_retirement_owner());assert_eq!(total,backing+born+extra);
}

#[test]
fn original_receiving_nested_io_dual_control_empty_phase_change_keeps_original_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();
 for direction in [IoNativeDirection::Decode,IoNativeDirection::Encode]{
  let mut decode_observer=|_|true;let mut encode_observer=|_|true;
  let mut decode_recipient=NativeDecodeRetirementRecipient::new();let mut encode_recipient=NativeEncodeRetirementRecipient::new();
  let mut decode=NativeDecodeControl::new(1048576,&mut decode_observer);let mut encode=NativeEncodeControl::new(1048576,&mut encode_observer);
  decode.install_retirement_recipient(&mut decode_recipient).unwrap();encode.install_retirement_recipient(&mut encode_recipient).unwrap();
  let mut run=IoRunControl::new(&mut decode,&mut encode,grant);run.select(direction).unwrap();
  let (result,born,released)=semio_framework_trace_memory_testing::observe_backing(||run.receive_nested::<(),()>(|_,child|{
   let opposite=if direction==IoNativeDirection::Decode{IoNativeDirection::Encode}else{IoNativeDirection::Decode};
   let before=child.progress();let(refused,birth,release)=semio_framework_trace_memory_testing::observe_backing(||child.select(opposite));
   refused.unwrap();assert_eq!((birth,release),(0,0));assert_eq!(child.direction(),opposite);assert_eq!(child.progress(),before);Ok(())
  }));result.unwrap();assert_eq!(born,run.progress().retained_capacity_bytes);assert_eq!(released,run.progress().released_bytes);assert_eq!(born,released);drop(run);
  assert!(!decode.has_retirement_owner());assert!(!encode.has_retirement_owner());assert_eq!(decode.owned_bytes(),if direction==IoNativeDirection::Decode{born}else{0});assert_eq!(encode.owned_bytes(),if direction==IoNativeDirection::Encode{born}else{0});
 }
}


#[test]
fn original_snapshot_wallet_retains_actual_faulty_child_backing_before_refusal(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../🛫️snapshot/🧪️tests/🧫️fixtures/🧾️overdraw.json")).unwrap();
 let policy:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();
 let(bytes,born,released)=semio_framework_trace_memory_testing::observe_backing(||Vec::<u8>::with_capacity(policy.maximum_capacity_bytes*2+1));
 assert_eq!(released,0);assert_eq!(born,bytes.capacity());let pointer=bytes.as_ptr();
 let actual=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:born,..Default::default()};assert!(!actual.fits(policy));
 let mut body=NativeSnapshotBodyWallet::new(policy);let error=body.record_progress(actual).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(error.retained_progress(),actual);assert_eq!(body.progress(),actual);assert_eq!(body.remaining_grant().maximum_capacity_bytes,0);assert_eq!(body.remaining_grant().maximum_copy_bytes,policy.maximum_copy_bytes);
 let mut original=|_|true;let mut native=NativeDecodeControl::new(1048576,&mut original);let mut owner=NativeSnapshotDecodeOwner::new(&mut native,policy);let error=owner.record_progress(actual).unwrap_err();assert_eq!(error.retained_progress(),actual);assert_eq!(owner.progress(),actual);assert_eq!(owner.grant(),policy);assert_eq!(owner.remaining_grant().maximum_capacity_bytes,0);assert!(owner.record_progress(Default::default()).is_err());assert_eq!(owner.progress(),actual);drop(owner);
 let mut original=|_|true;let mut native=NativeEncodeControl::new(1048576,&mut original);let mut owner=NativeSnapshotEncodeOwner::new(&mut native,policy);let error=owner.record_progress(actual).unwrap_err();assert_eq!(error.retained_progress(),actual);assert_eq!(owner.progress(),actual);assert_eq!(owner.grant(),policy);assert_eq!(owner.remaining_grant().maximum_capacity_bytes,0);drop(owner);
 assert_eq!((bytes.as_ptr(),bytes.len(),bytes.capacity()),(pointer,0,born));
 let close=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
 let mut retained=semio_framework_value::retirement::controlled::ControlledRetirement::new(bytes).ok().unwrap();let mut released=0;let mut allocated=0;
 for _ in 0..4096{if retained.terminal_is_empty(){break}let(step,born,freed)=semio_framework_trace_memory_testing::observe_backing(||retained.step(close).unwrap());assert!(step.progress().fits(close));assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));allocated+=born;released+=freed;}
 assert!(retained.terminal_is_empty());assert_eq!(released,born+allocated);
}
