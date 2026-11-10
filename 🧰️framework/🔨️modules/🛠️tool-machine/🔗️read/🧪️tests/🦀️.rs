//! 🧪️ Native ledger leases return the same authored rows and factory after exact funded closure.
use super::*;
use semio_framework_job::*;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,OwnedValueRetirementFactory,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,retirement::RetireOwned};
use std::sync::Arc;
#[path="../../../⏯️tool-run/🧪️tests/⏱️effect/🦀️.rs"]
mod effect_harness;
const TURN:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
fn original()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(8610),Generation(51),RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096},default_now_us,u64::MAX,1<<24)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(g,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
fn turn_native<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
/// 📏️ Funds one test turn with the exact quote on top of the standing turn grant.
fn funded(d:RetirementDemand)->RetainedCloneGrant{let g=effect_harness::grant(d);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:g.maximum_copy_bytes.max(TURN.maximum_copy_bytes),maximum_capacity_bytes:g.maximum_capacity_bytes.max(TURN.maximum_capacity_bytes),maximum_release_bytes:g.maximum_release_bytes.max(TURN.maximum_release_bytes),maximum_depth:g.maximum_depth.max(TURN.maximum_depth)}}
fn denied(d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut values=vec![RetainedCloneGrant{maximum_items:0,..TURN}];if d.copy_bytes>0{values.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..TURN})}if d.capacity_bytes>0{values.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..TURN})}if d.release_bytes>0{values.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..TURN})}if d.depth>0{values.push(RetainedCloneGrant{maximum_depth:d.depth-1,..TURN})}values}
macro_rules! read_law{($ledger:expr,$state:ty,$field:ident,$blocked:expr)=>{{for cut in 0..4{let mut ledger=$ledger;let mut bank=effect_harness::bank();let key=ledger.retained_read_key("source").unwrap();let revision=ledger.source_revision();let pointer=ledger.open("source").unwrap().$field.as_ptr();let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<$state>>=Arc::new(OwnedValueRetirementFactory::<$state>::default());let mut ready=false;
for _ in 0..cut{let d=ledger.retained_read_demands("source",key,4096).unwrap();for g in denied(d){assert_eq!(effect_harness::paid(&mut bank,g,|e,r,recover|if recover{ledger.recover_advance_retained_read_into("source",key,&factory,e,r)}else{ledger.advance_retained_read_into("source",key,&factory,e,r)}).progress,Default::default());}effect_harness::paid(&mut bank,TURN,|e,r,recover|if recover{ledger.recover_advance_retained_read_into("source",key,&factory,e,r)}else{ledger.advance_retained_read_into("source",key,&factory,e,r)});ready=ledger.retained_read("source",key).is_ok();if ready{break}}
let mut lease=semio_framework_value::retained_clone::RetainedOwnedProjectionAdmission::<$state>::default();if ready{let d=ledger.retain_read_demand("source",key,&lease).unwrap();effect_harness::paid(&mut bank,TURN,|e,r,_|{let mut result=None;drop(ledger.retain_read_into("source",key,&mut lease,e,semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut result)?)?);Ok(r.complete(result.ok_or_else(||ValueError::invariant("retain suspended"))?.map_err(Into::into)))});assert!(d.copy_bytes<=TURN.maximum_copy_bytes);assert_eq!(lease.borrow().unwrap().get().$field.as_ptr(),pointer);assert!($blocked(&mut ledger));assert_eq!(ledger.source_revision(),revision);}
for index in 0..65536{if lease.terminal_is_empty(){break}let d=lease.close_demand(4096).unwrap();for g in denied(d){assert_eq!(effect_harness::paid(&mut bank,g,|e,r,recover|if recover{lease.recover_close_into(4096,e,r)}else{lease.close_into(4096,e,r)}).progress,Default::default());}effect_harness::paid(&mut bank,TURN,|e,r,recover|if recover{lease.recover_close_into(4096,e,r)}else{lease.close_into(4096,e,r)});assert!(index<65535)}drop(lease);
let mut returned=None;for index in 0..65536{if !ledger.retained_read_pending("source"){break}let d=ledger.retained_read_return_demands("source",key,4096).unwrap();for g in denied(d){assert_eq!(effect_harness::paid(&mut bank,g,|e,r,recover|if recover{ledger.recover_return_retained_read_into("source",key,&mut returned,e,r)}else{ledger.return_retained_read_into("source",key,&mut returned,e,r)}).progress,Default::default());}effect_harness::paid(&mut bank,TURN,|e,r,recover|if recover{ledger.recover_return_retained_read_into("source",key,&mut returned,e,r)}else{ledger.return_retained_read_into("source",key,&mut returned,e,r)});assert!(index<65535)}assert_eq!(ledger.source_revision(),revision);assert_eq!(ledger.open("source").unwrap().$field.as_ptr(),pointer);if let Some(returned)=returned{assert!(Arc::ptr_eq(&factory,&returned));}}}}}
fn scrub()->ScrubLedger<u64>{let mut ledger=ScrubLedger::default();let mut input=Some(ScrubInput::Tick{gesture:"press".into(),leaves:(0..8197).collect()});ledger.send("source","test#press",&ActorId("native-source".into()),"base",&mut input,HybridLogicalTimestamp{actor:0,physical_ms:10,logical:0}).unwrap();ledger}
fn typing()->TypingLedger<u64>{let mut ledger=TypingLedger::default();ledger.send("source","test#typing",&ActorId("native-source".into()),TypingInput::Edit{buffer:"text".into(),leaves:(0..8197).collect()},|_,next|TypingFold::Net(next.to_vec()),HybridLogicalTimestamp{actor:0,physical_ms:10,logical:0}).unwrap();ledger}
fn gesture()->GestureLedger<u64>{let mut ledger=GestureLedger::default();ledger.settle("source",Some(GestureState{states:vec!["dragging".into()],verb:"test#gesture".into(),press:"press".into(),authoring_seed:"native-source".into(),base_revision:"base".into(),transaction:TransactionRef{id:"tx-0000000000000001".into(),tool:"test#gesture".into()},entries:(0..8197).map(|value|(value.to_string(),value)).collect(),context:protocol::DslValue::Null})).unwrap();ledger}
#[test]fn original_scrub_read_preserves_original_rows_and_issuer(){read_law!(scrub(),ScrubState<u64>,entries,|ledger:&mut ScrubLedger<u64>|{let mut input=Some(ScrubInput::Commit{gesture:"press".into(),leaves:vec![]});ledger.send("source","test#press",&ActorId("native-source".into()),"base",&mut input,HybridLogicalTimestamp{actor:0,physical_ms:10,logical:1}).is_err()&&input.is_some()});}
#[test]fn original_typing_read_preserves_original_rows_and_issuer(){read_law!(typing(),TypingState<u64>,net,|ledger:&mut TypingLedger<u64>|ledger.commit("source",TypingCommit::Enter,HybridLogicalTimestamp{actor:0,physical_ms:10,logical:1}).is_err());}
#[test]fn original_gesture_read_preserves_original_rows_and_issuer(){read_law!(gesture(),GestureState<u64>,entries,|ledger:&mut GestureLedger<u64>|ledger.settle("source",None).is_err());}

macro_rules! transfer_law{($ledger:expr,$ledger_type:ty,$field:ident)=>{{
 let mut ledger=$ledger;let mut bank=effect_harness::bank();let key=ledger.resident_key("source").unwrap();let pointer=ledger.open("source").unwrap().$field.as_ptr();let revision=ledger.source_revision();let mut output=None;let d=<$ledger_type>::original_transfer_demands();
 for denied in denied(d){let(result,heap)=effect_harness::primitive(&mut bank,denied,|e|ledger.withdraw_original("source",key,&mut output,e.grant()));assert!(result.unwrap().is_none());assert_eq!(heap,(0,0));assert!(output.is_none());assert_eq!(ledger.source_revision(),revision);assert_eq!(ledger.open("source").unwrap().$field.as_ptr(),pointer);}
 let(result,_)=effect_harness::primitive(&mut bank,TURN,|e|ledger.withdraw_original("source",key,&mut output,e.grant()));assert!(result.unwrap().is_some());assert!(ledger.is_empty());assert!(ledger.open("source").is_none());assert_eq!(ledger.provisional().count(),0);assert_eq!(output.as_ref().unwrap().$field.as_ptr(),pointer);assert_ne!(ledger.source_revision(),revision);let vacant=ledger.resident_key("source").unwrap();assert_ne!(vacant,key);
 for denied in denied(d){let(result,heap)=effect_harness::primitive(&mut bank,denied,|e|ledger.install_resident_original("source",vacant,&mut output,e.grant()));assert!(result.unwrap().is_none());assert_eq!(heap,(0,0));assert!(ledger.is_empty());assert_eq!(output.as_ref().unwrap().$field.as_ptr(),pointer);}
 let(result,_)=effect_harness::primitive(&mut bank,TURN,|e|ledger.install_resident_original("source",vacant,&mut output,e.grant()));assert!(result.unwrap().is_some());assert!(output.is_none());assert!(!ledger.is_empty());assert_eq!(ledger.open("source").unwrap().$field.as_ptr(),pointer);
 let key=ledger.resident_key("source").unwrap();let(result,_)=effect_harness::primitive(&mut bank,TURN,|e|ledger.withdraw_original("source",key,&mut output,e.grant()));assert!(result.unwrap().is_some());effect_harness::retire(&mut bank,output.take().unwrap());assert!(ledger.is_empty());
}}}
#[test]fn original_scrub_transfer_preserves_resident_and_entries(){transfer_law!(scrub(),ScrubLedger<u64>,entries);}
#[test]fn original_gesture_transfer_preserves_resident_and_entries(){transfer_law!(gesture(),GestureLedger<u64>,entries);}
#[test]fn original_typing_transfer_preserves_resident_and_entries(){transfer_law!(typing(),TypingLedger<u64>,net);}

#[path="🧵️scrub/🦀️.rs"]
mod original_scrub_write_laws;

/// 🪟️ Issued native readers retain the exact source while sorted prefix windows force resident backing growth.
#[test]
fn typing_reader_survives_resident_growth_and_prefix_insertion(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let name=law["readerWindow"].as_str().unwrap();let length=law["readerLeaves"].as_u64().unwrap();let maximum=law["maximumTurns"].as_u64().unwrap();
 let mut ledger=TypingLedger::default();
 ledger.send(name,"test#typing",&ActorId("resident-source".into()),TypingInput::Edit{buffer:"text".into(),leaves:(0..length).collect()},|_,next|TypingFold::Net(next.to_vec()),HybridLogicalTimestamp{actor:0,physical_ms:10,logical:0}).unwrap();
 let pointer=ledger.open(name).unwrap().net.as_ptr();let capacity=ledger.windows.capacity();let key=ledger.retained_read_key(name).unwrap();let mut bank=effect_harness::bank();
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<TypingState<u64>>>=Arc::new(OwnedValueRetirementFactory::<TypingState<u64>>::default());
 let mut ready=false;let mut stalls=0;
 for step in 0..maximum{let g=funded(ledger.retained_read_demands(name,key,4096).unwrap());let o=effect_harness::paid(&mut bank,g,|e,r,recover|if recover{ledger.recover_advance_retained_read_into(name,key,&factory,e,r)}else{ledger.advance_retained_read_into(name,key,&factory,e,r)});effect_harness::expect_progress(&o,&mut stalls,"typing reader capture");ready=ledger.retained_read(name,key).is_ok();if ready{break}assert!(step+1<maximum);}
 assert!(ready);
 let mut projection=semio_framework_value::retained_clone::RetainedOwnedProjectionAdmission::<TypingState<u64>>::default();
 effect_harness::paid(&mut bank,funded(ledger.retain_read_demand(name,key,&projection).unwrap()),|e,r,_|{let mut result=None;drop(ledger.retain_read_into(name,key,&mut projection,e,semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut result)?)?);Ok(r.complete(result.ok_or_else(||ValueError::invariant("retain suspended"))?.map_err(Into::into)))});
 for index in 0..law["prefixWindows"].as_u64().unwrap(){
  let window=format!("a-{index:05}");
  ledger.send(&window,"test#typing",&ActorId("prefix-source".into()),TypingInput::Edit{buffer:"text".into(),leaves:vec![index]},|_,next|TypingFold::Net(next.to_vec()),HybridLogicalTimestamp{actor:0,physical_ms:11+index,logical:0}).unwrap();
  assert_eq!(ledger.retained_read_key(name),Some(key));assert_eq!(ledger.open(name).unwrap().net.as_ptr(),pointer);
  assert_eq!(projection.borrow().unwrap().get().net.as_ptr(),pointer);
  assert_eq!(projection.borrow().unwrap().get().net.len(),length as usize);
 }
 assert!(ledger.windows.capacity()>capacity);assert_eq!(ledger.windows().last(),Some(name));
 let mut stalls=0;
 for step in 0..maximum{if projection.terminal_is_empty(){break}let g=funded(projection.close_demand(4096).unwrap());let o=effect_harness::paid(&mut bank,g,|e,r,recover|if recover{projection.recover_close_into(4096,e,r)}else{projection.close_into(4096,e,r)});effect_harness::expect_progress(&o,&mut stalls,"typing reader close");assert!(step+1<maximum);}
 assert!(projection.terminal_is_empty());
 let mut returned=None;let mut stalls=0;
 for step in 0..maximum{if !ledger.retained_read_pending(name){break}let g=funded(ledger.retained_read_return_demands(name,key,4096).unwrap());let o=effect_harness::paid(&mut bank,g,|e,r,recover|if recover{ledger.recover_return_retained_read_into(name,key,&mut returned,e,r)}else{ledger.return_retained_read_into(name,key,&mut returned,e,r)});effect_harness::expect_progress(&o,&mut stalls,"typing reader return");assert!(step+1<maximum);}
 assert!(!ledger.retained_read_pending(name));assert_eq!(ledger.open(name).unwrap().net.as_ptr(),pointer);
 assert!(Arc::ptr_eq(&factory,&returned.unwrap()));
}
