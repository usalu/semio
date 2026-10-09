//! 🧪️ Original completion aliases and vacant allocations obey the shared custody corpus.
use super::*;
use semio_framework_plugin::{ArtifactToolCompletion,Effect};
use semio_framework_value::retained_clone::RetainedCloneStep;

fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn settle(owner:&mut dyn ErasedSnapshotRetirement)->usize{
 let mut released=0;
 for _ in 0..100000{
  if owner.terminal_is_empty(){return released;}
  let copy=owner.next_copy_byte_demand().unwrap();let capacity=owner.next_capacity_byte_demand(copy).unwrap();let release=owner.next_release_byte_demand().unwrap();let depth=owner.next_depth_demand().unwrap();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth.max(1)};
  let (step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());
  let progress=match step{RetainedCloneStep::Progress(value)|RetainedCloneStep::Complete(value)=>value};
  assert!(progress.fits(grant));assert!(!event.overflowed);assert_eq!(event.released_bytes,progress.released_bytes);assert_eq!(event.requested_bytes,progress.retained_capacity_bytes);released+=progress.released_bytes;
 }
 panic!("completion owner failed to return its original fields");
}
#[test]
fn completion_original_alias_and_payload_physical_release(){
 let fixture=corpus();
 for count in fixture["aliasCounts"].as_array().unwrap(){
  let count=count.as_u64().unwrap()as usize;let original=ArtifactToolCompletion::<App>::test_new();let mut aliases=Vec::with_capacity(count);
  for _ in 1..count{aliases.push(original.clone());}aliases.push(original);
  let body="x".repeat(fixture["bodyBytes"].as_u64().unwrap()as usize);
  aliases[0].complete(Ok(Emit::effect(Effect::Notify{message:body})),EphemeralEmit::default()).unwrap();
  for index in 0..count{let alias=aliases.pop().unwrap();let mut owner=ControlledRetirement::new(alias).unwrap_or_else(|_|panic!("completion authority birth"));let (zero,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(Default::default()).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert!(!owner.terminal_is_empty());let released=settle(&mut owner);if index+1<count{assert!(released<fixture["bodyBytes"].as_u64().unwrap()as usize);}else{assert!(released>=fixture["bodyBytes"].as_u64().unwrap()as usize);}}
 }
}
#[test]
fn completion_factory_refusal_preserves_actual_original(){
 let mut value=Some(Value::Emit(Ok(Emit::effect(Effect::RequestSync)),EphemeralEmit::default()));assert!(birth(value.as_ref().unwrap()).is_none());
 let original=value.as_ref().unwrap()as *const Value;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:64};
 assert!(admit(&mut value,grant).is_err());assert_eq!(value.as_ref().unwrap()as *const Value,original);
 match value.take().unwrap(){Value::Emit(Ok(mut emit),_)=>{assert!(matches!(emit.effects.pop(),Some(Effect::RequestSync)));},_=>panic!("original publication replaced")}
}
#[test]
fn completion_vacant_backing_requires_exact_release_grant(){
 for capacity in corpus()["vacantCapacities"].as_array().unwrap(){let mut values=Vec::<u8>::with_capacity(capacity.as_u64().unwrap()as usize);let actual=values.capacity();let mut cursor=VacantVec(std::mem::take(&mut values)).retirement();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:actual-1,..Default::default()};let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(grant));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert!(!cursor.terminal_is_empty());let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_release_bytes:actual,..grant}));assert!(matches!(step,RetirementStep::Bytes(bytes)if bytes==actual));assert_eq!(event.released_bytes,actual);assert!(cursor.terminal_is_empty());}
}
