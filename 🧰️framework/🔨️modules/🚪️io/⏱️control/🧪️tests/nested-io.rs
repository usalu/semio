//! 🪆️ Nested IO loans retain original pending frames and settle accepted native spans exactly once.
use super::*;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use std::cell::Cell;
#[path="../🛫️snapshot/🧪️tests/authority.rs"]
mod authority;
use authority::{Physical,Work,with_decode,with_encode,with_natives,decode_close,decode_close_installed,encode_close,controlled_step,funded,settle};
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
 assert_eq!(pending.as_ref().unwrap().as_ref()as*const _ as*const()as usize,pointer);while pending.is_some(){let(turn,_,_)=funded(grant,|effect|{let body=effect.grant().maximum_copy_bytes;settle(semio_framework_value::close_factory_ticket(&mut pending,body,effect))});turn.result.unwrap();}
}
#[test]
fn actual_nested_receiving_frame_can_finish_decode_and_select_original_encoder(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let encoded=Cell::new(false);let mut receive=|_|true;let mut publish=|_|{encoded.set(true);true};
 let turn=with_natives(&mut receive,&mut publish,grant,|decode,encode|{let turn={let mut run=IoRunControl::new(decode,encode,grant);let mut output=None;run.receive_nested::<(),(),_>(&mut output,&mut |_,out,_,_,child|super::native_snapshot_turn(|work|{{work.performed();child.decode()?.checkpoint()}?;{work.performed();child.encode()?.checkpoint()}?;*out=Some(());Ok(())}))};for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !decode.has_retirement_owner(){break}decode_close_installed(decode,grant).0.unwrap();}assert!(!decode.has_retirement_owner());for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !encode.has_retirement_owner(){break}encode_close(encode,grant).0.unwrap();}assert!(!encode.has_retirement_owner());turn});turn.result.unwrap();assert!(encoded.get());
}
#[test]
fn original_receiving_nested_io_decode_retains_child_ticket_and_same_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut source=Some(f["original"].as_str().unwrap().to_owned());let backing=source.as_ref().unwrap().capacity();let live=Cell::new(true);let mut observer=|_|live.get();let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(f["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();native.charge(backing).unwrap();
 with_decode(&mut native,Work(true),Physical::new(grant),|native|{
  let((result,born,released),progress)={let mut run=IoRunControl::decoding(&mut *native,grant);let mut output=None;let turn=semio_framework_trace_memory_testing::observe_backing(||run.receive_nested::<(),(),_>(&mut output,&mut |_,_,_,_,run|{let mut inner=None;run.receive_nested::<String,(),_>(&mut inner,&mut |slot,_,_,_,child|super::native_snapshot_turn(|work|{work.performed();*slot=source.take();{work.performed();child.decode()?.checkpoint()}?;live.set(false);Ok(())}))}));(turn,run.progress())};
  assert!(result.work>=2);assert_eq!(result.result.err().unwrap().kind(),ValueRefusalKind::Canceled);assert!(source.is_none());assert_eq!(progress.copied_items,f["expected"]["nestedFrames"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,born);assert_eq!(released,0);assert!(native.has_retirement_owner());live.set(true);
  let(step,allocated,released)=decode_close_installed(native,Default::default());let step=step.unwrap_or_else(|error|semio_framework_value::retained_clone::RetainedCloneStep::Progress(error.retained_progress()));assert_eq!((step.progress(),allocated,released),(Default::default(),0,0));let mut total=0;let mut extra=0;
  for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,allocated,released)=decode_close_installed(native,grant);let step=step.unwrap();assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));extra+=allocated;total+=released;}assert!(!native.has_retirement_owner());assert_eq!(total,backing+born+extra);
 });
}
#[test]
fn original_receiving_nested_io_encode_retains_child_ticket_and_same_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();let mut source=Some(f["original"].as_str().unwrap().to_owned());let backing=source.as_ref().unwrap().capacity();let live=Cell::new(true);let mut observer=|_|live.get();let mut recipient=NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(f["nativeMaximumBytes"].as_u64().unwrap()as usize,&mut observer);native.install_retirement_recipient(&mut recipient).unwrap();native.charge(backing).unwrap();
 let(((result,born,released),progress),_)=with_encode(&mut native,Work(true),Physical::new(grant),|native|{let mut run=IoRunControl::encoding(native,grant);let mut output=None;let turn=semio_framework_trace_memory_testing::observe_backing(||run.receive_nested::<(),(),_>(&mut output,&mut |_,_,_,_,run|{let mut inner=None;run.receive_nested::<String,(),_>(&mut inner,&mut |slot,_,_,_,child|super::native_snapshot_turn(|work|{work.performed();*slot=source.take();{work.performed();child.encode()?.checkpoint()}?;live.set(false);Ok(())}))}));(turn,run.progress())});
 assert!(result.work>=2);assert_eq!(result.result.err().unwrap().kind(),ValueRefusalKind::Canceled);assert!(source.is_none());assert_eq!(progress.copied_items,f["expected"]["nestedFrames"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,born);assert_eq!(released,0);assert!(native.has_retirement_owner());live.set(true);let(step,allocated,released)=encode_close(&mut native,Default::default());let step=step.unwrap_or_else(|error|semio_framework_value::retained_clone::RetainedCloneStep::Progress(error.retained_progress()));assert_eq!((step.progress(),allocated,released),(Default::default(),0,0));let mut total=0;let mut extra=0;
 for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,allocated,released)=encode_close(&mut native,grant);let step=step.unwrap();assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));extra+=allocated;total+=released;}assert!(!native.has_retirement_owner());assert_eq!(total,backing+born+extra);
}

#[test]
fn original_receiving_nested_io_dual_control_empty_phase_change_keeps_original_wallet(){
 let f=fixture();let grant:RetainedCloneGrant=serde_json::from_value(f["grant"].clone()).unwrap();
 for direction in [IoNativeDirection::Decode,IoNativeDirection::Encode]{
  let mut decode_observer=|_|true;let mut encode_observer=|_|true;
  with_natives(&mut decode_observer,&mut encode_observer,grant,|decode,encode|{
   let(result,born,released,progress)={let mut run=IoRunControl::new(decode,encode,grant);run.select(direction).unwrap();let mut output=None;
    let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||run.receive_nested::<(),(),_>(&mut output,&mut |_,out,_,_,child|super::native_snapshot_turn(|work|{
     let opposite=if direction==IoNativeDirection::Decode{IoNativeDirection::Encode}else{IoNativeDirection::Decode};
     let before=child.progress();let(refused,birth,release)=semio_framework_trace_memory_testing::observe_backing(||child.select(opposite));
     work.performed();refused.unwrap();assert_eq!((birth,release),(0,0));assert_eq!(child.direction(),opposite);assert_eq!(child.progress(),before);*out=Some(());Ok(())
    })));let progress=run.progress();(result,born,released,progress)};
   assert!(result.work>=1);result.result.unwrap();assert_eq!(born,progress.retained_capacity_bytes);assert_eq!(released,progress.released_bytes);
   assert_eq!(decode.has_retirement_owner(),direction==IoNativeDirection::Decode);assert_eq!(encode.has_retirement_owner(),direction==IoNativeDirection::Encode);
   let(mut closed_born,mut closed_released)=(0,0);
   for _ in 0..f["maximumTurns"].as_u64().unwrap(){if !decode.has_retirement_owner()&&!encode.has_retirement_owner(){break}
    if decode.has_retirement_owner(){let(step,allocated,freed)=decode_close_installed(decode,grant);let step=step.unwrap();assert_eq!((allocated,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));closed_born+=allocated;closed_released+=freed;}
    if encode.has_retirement_owner(){let(step,allocated,freed)=encode_close(encode,grant);let step=step.unwrap();assert_eq!((allocated,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));closed_born+=allocated;closed_released+=freed;}
   }
   assert!(!decode.has_retirement_owner());assert!(!encode.has_retirement_owner());assert_eq!(born+closed_born,released+closed_released);if direction==IoNativeDirection::Decode{assert!(decode.owned_bytes()>=born+closed_born)}else{assert_eq!(decode.owned_bytes(),0)}if direction==IoNativeDirection::Encode{assert!(encode.owned_bytes()>=born+closed_born)}else{assert_eq!(encode.owned_bytes(),0)}
  });
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
 for _ in 0..4096{if retained.terminal_is_empty(){break}let(step,born,freed)=controlled_step(&mut retained,close);let step=step.unwrap();assert!(step.progress().fits(close));assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));allocated+=born;released+=freed;}
 assert!(retained.terminal_is_empty());assert_eq!(released,born+allocated);
}
