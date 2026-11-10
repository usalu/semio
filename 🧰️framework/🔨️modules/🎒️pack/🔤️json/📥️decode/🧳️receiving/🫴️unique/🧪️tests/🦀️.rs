//! 🧪️ Real Original Source loans fund every output prefix, foreign-borrow refusal, typed capture and physical cleanup.
use super::*;
use semio_framework_job::{Generation,OperationId,OriginalTurnFailure,StepContext};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_job_native_decode::OriginalNativeDecodePolicy;
use semio_framework_value::{retirement::controlled::ControlledRetirement};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[derive(Debug,PartialEq,serde::Serialize,serde::Deserialize,semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="::semio_framework_value",rename_all="camelCase",deny_unknown_fields)]
struct Projection{message:String,values:Vec<u8>}
fn drain(caller:&mut OriginalJobFixtureCaller,g:RetainedCloneGrant,state:&mut OriginalJsonOutputSource<Projection>,closes:&mut Vec<ControlledRetirement<OriginalJsonOutputOutcome>>){let mut spins=0usize;while let Some(close)=closes.last_mut(){spins+=1;assert!(spins<20000,"deferred outcome cleanup must reach its terminal empty state");if close.terminal_is_empty(){closes.pop();continue}match funded(caller,g,|cx|state.close_owned_step(close,cx)){Some(receipt)=>{assert!(receipt.receiving_refusal.is_none());receipt.value.unwrap();},None=>break}}}
fn clock()->Option<u64>{Some(1)}
fn funded<T>(caller:&mut OriginalJobFixtureCaller,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=observe(||caller.turn(g,run));let p=caller.receipt();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
#[test]
fn unique_json_output_keeps_original_source_and_every_cancelled_typed_prefix(){
 let c:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();let stock=&c["stock"];let g=RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize};let limits=JsonReadLimits{maximum_bytes:2048,maximum_allocation_bytes:g.maximum_capacity_bytes,maximum_depth:16,maximum_items:10000};
 for row in c["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(c["cancelPrefixes"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize))){let wire=row["wire"].as_str().unwrap();let accepted=row["accepted"].as_bool().unwrap();let oracle=serde_json::from_str::<Projection>(wire);assert_eq!(oracle.is_ok(),accepted);let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(1),g,clock,100000000,stock["fuel"].as_u64().unwrap());caller.retain_dispatch().unwrap();let mut state=OriginalJsonOutputSource::<Projection>::new(OriginalNativeDecodePolicy{maximum_owned_bytes:g.maximum_capacity_bytes},limits);let mut captured=None;let mut failed=false;let mut checked_foreign=false;let mut closes=Vec::new();
  for at in 0..100000{if cut==Some(at){break}let outcome=funded(&mut caller,g,|cx|if state.ready(){state.take_step(&mut captured,cx)}else{state.receive_step(wire.as_bytes(),cx)});
   if let Some(outcome)=outcome{assert!(outcome.receiving_refusal.is_none());failed=outcome.value.is_err();closes.push(ControlledRetirement::retain(outcome));drain(&mut caller,g,&mut state,&mut closes);if failed{break}}
   if !checked_foreign&&state.receiver().is_some(){let foreign=wire.as_bytes().to_vec();let result=funded(&mut caller,g,|cx|state.receive_step(&foreign,cx)).unwrap();assert!(result.receiving_refusal.is_none());assert!(result.value.is_err());assert_eq!(result.progress,Default::default());assert_eq!(result.work,0);closes.push(ControlledRetirement::retain(result));drain(&mut caller,g,&mut state,&mut closes);checked_foreign=true;}
   if captured.is_some(){break}assert!(at<99999);
  }
  if cut.is_none(){assert_eq!(failed,!accepted);assert_eq!(captured.is_some(),accepted);if let Some(value)=captured.as_ref(){assert_eq!(value,&oracle.unwrap());}}
  caller.cancel().cancel_now();drain(&mut caller,g,&mut state,&mut closes);
  if let Some(value)=captured{let(mut close,heap)=observe(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("unique captured output supports controlled retirement")));assert_eq!(heap,(0,0));for at in 0..100000{if close.terminal_is_empty(){break}if let Some(outcome)=funded(&mut caller,g,|cx|state.close_owned_step(&mut close,cx)){assert!(outcome.receiving_refusal.is_none());outcome.value.unwrap();}assert!(at<99999)}assert!(close.terminal_is_empty());}
  for at in 0..100000{if state.terminal_is_empty(){break}drain(&mut caller,g,&mut state,&mut closes);match funded(&mut caller,g,|cx|state.close_step(cx)){OriginalJsonOutputClose::Producer(Some(outcome))=>{assert!(outcome.receiving_refusal.is_none());outcome.value.unwrap();},_=>{}}assert!(at<99999)}assert!(closes.is_empty());assert!(state.terminal_is_empty());assert!(caller.fuel_remaining()>0);caller.retire(g);
 }}
}

#[test]
fn source_owned_unique_output_returns_real_typed_capture_before_decoder_retirement(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();let g=RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:64};
 let mut worst=0usize;for row in corpus["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(corpus["cancelPrefixes"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize))){let wire=row["wire"].as_str().unwrap();let oracle=serde_json::from_str::<Projection>(wire);let mut caller=OriginalJobFixtureCaller::admit(OperationId(942),Generation(1),g,clock,100000000,1000000);caller.retain_dispatch().unwrap();let mut state=OriginalJsonOutputSource::<Projection>::new(OriginalNativeDecodePolicy{maximum_owned_bytes:g.maximum_capacity_bytes},JsonReadLimits{maximum_bytes:2048,maximum_allocation_bytes:g.maximum_capacity_bytes,maximum_depth:16,maximum_items:10000});let mut captured=None;let mut refused=false;let mut used=0usize;let mut closes=Vec::new();
  for at in 0..100000{if cut==Some(at){break}let outcome=funded(&mut caller,g,|cx|if state.ready(){state.take_step(&mut captured,cx)}else{state.receive_step(wire.as_bytes(),cx)});if let Some(outcome)=outcome{assert!(outcome.receiving_refusal.is_none());let failed=outcome.value.is_err();closes.push(ControlledRetirement::retain(outcome));drain(&mut caller,g,&mut state,&mut closes);if failed{refused=true;used=at+1;break}}if captured.is_some(){used=at+1;break}assert!(at<99999)}if cut.is_none(){worst=worst.max(used);assert_eq!(refused,oracle.is_err());assert_eq!(captured.as_ref(),oracle.as_ref().ok());}
  caller.cancel().cancel_now();drain(&mut caller,g,&mut state,&mut closes);if let Some(value)=captured{let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("captured output supports controlled retirement"));for at in 0..100000{if close.terminal_is_empty(){break}if let Some(outcome)=funded(&mut caller,g,|cx|state.close_owned_step(&mut close,cx)){assert!(outcome.receiving_refusal.is_none());outcome.value.unwrap();}assert!(at<99999)}}for at in 0..100000{if state.terminal_is_empty(){break}drain(&mut caller,g,&mut state,&mut closes);match funded(&mut caller,g,|cx|state.close_step(cx)){OriginalJsonOutputClose::Producer(Some(outcome))=>{assert!(outcome.receiving_refusal.is_none());outcome.value.unwrap();},_=>{}}assert!(at<99999)}assert!(closes.is_empty());assert!(state.terminal_is_empty());assert!(caller.fuel_remaining()>0);caller.retire(g);
 }}
 assert!(worst<=12,"a 39-byte wire completes within a bounded number of receive turns: {worst}");
}

#[test]
fn per_turn_outcome_costs_what_it_holds_and_failures_stay_out_of_line(){
 use std::mem::size_of;
 let(outcome,ticket,turn,primary)=(size_of::<OriginalJsonOutputOutcome>(),size_of::<OriginalJsonOutputFailure>(),size_of::<OriginalTurnFailure>(),size_of::<semio_framework_value_native_owned::OriginalReceivingPrimary>());
 assert!(ticket<=128,"failure ticket {ticket} bytes");
 assert!(outcome<=256,"per-turn outcome {outcome} bytes must not hold the {turn} byte receiving refusal inline");
 assert!(primary<=turn);
}
