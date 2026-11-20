//! 🧪️ Original completion aliases and vacant allocations obey the shared custody corpus.
use semio_framework_test_fixtures::fixture_text;

#[allow(unused_imports)] use semio_plugin::{ArtifactToolCompletionValue,ArtifactCompletionFault,ArtifactDownloadOutput,EditorApp,Emit,EphemeralEmit,NoConfigMutation,NoDraftMutation,VacantVec,VacantDeque,CompletionEffect};
#[allow(unused_imports)] use semio_framework_value::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::{ControlledRetirement,admit_typed_controlled_retirement}},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
#[allow(unused_imports)] use std::mem::ManuallyDrop;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_editor::completion::*;
use semio_plugin::{ArtifactToolCompletion,Effect};
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkPoll};

fn effect_turn<T>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let mut turn = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, || Some(0), grant).unwrap();
    turn.with_effect(run).unwrap().unwrap().0
}

fn progress(poll: OriginalRetirementPoll) -> RetainedCloneProgress {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.unwrap().progress(),
        RetainedWorkPoll::Pending { .. } => Default::default(),
    }
}

fn corpus()->serde_json::Value{serde_json::from_str(fixture_text!("../../../../📬️completion/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn settle(owner:&mut dyn ErasedSnapshotRetirement)->usize{
 let mut released=0;
 for _ in 0..100000{
  if owner.terminal_is_empty(){return released;}
  let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let body=copy.max(release);let capacity=owner.next_capacity_byte_demand(body).unwrap();let depth=owner.next_depth_demand().unwrap();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth.max(1)};
  let (progress,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||effect_turn(grant,|effect|progress(owner.close_step(body,effect))));
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
  for index in 0..count{let alias=aliases.pop().unwrap();let mut owner=ControlledRetirement::new(alias).unwrap_or_else(|_|panic!("completion authority birth"));let (zero,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||effect_turn(Default::default(),|effect|progress(owner.step(0,effect))));assert_eq!(zero,Default::default());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert!(!owner.terminal_is_empty());let released=settle(&mut owner);if index+1<count{assert!(released<fixture["bodyBytes"].as_u64().unwrap()as usize);}else{assert!(released>=fixture["bodyBytes"].as_u64().unwrap()as usize);}}
 }
}
#[test]
fn completion_factory_refusal_preserves_actual_original(){
 let mut value=Some(Value::Emit(Ok(Emit::effect(Effect::RequestSync)),EphemeralEmit::default()));assert!(demand(value.as_ref().unwrap()).is_err());
 let original=value.as_ref().unwrap()as *const Value;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:64};
 assert!(effect_turn(grant,|effect|admit(&mut value,effect)).result.is_err());assert_eq!(value.as_ref().unwrap()as *const Value,original);
 match value.take().unwrap(){Value::Emit(Ok(mut emit),_)=>{assert!(matches!(emit.effects.pop(),Some(Effect::RequestSync)));},_=>panic!("original publication replaced")}
}
#[test]
fn completion_vacant_backing_requires_exact_release_grant(){
 for capacity in corpus()["vacantCapacities"].as_array().unwrap(){let mut values=Vec::<u8>::with_capacity(capacity.as_u64().unwrap()as usize);let actual=values.capacity();let mut cursor=VacantVec(std::mem::take(&mut values)).retirement_seed();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:actual-1,..Default::default()};let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||effect_turn(grant,|effect|cursor.close_step(actual,effect)));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert!(!cursor.terminal_is_empty());let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||effect_turn(RetainedCloneGrant{maximum_release_bytes:actual,..grant},|effect|cursor.close_step(actual,effect)));assert!(matches!(step,RetirementStep::Progress(progress, _) if progress.released_bytes==actual));assert_eq!(event.released_bytes,actual);assert!(cursor.terminal_is_empty());}
}
