//! 🧰️ Empty original ledger backing requires exact physical admission; held elements are never erased.
use semio_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_plugin_store_lifecycle::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_ui_scene::SceneDoc;
use crate::{ToolRunMemberEmissionOwner, ToolRunMemberOwners, close_ledger_backing, ledger_backing_is_empty, ledger_empty_backing_demand};
use semio_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_plugin_window_config::WindowConfigOwner;
use semio_ui_contract::*;
#[test]
fn tool_ledger_empty_backing_preserves_undergrant_and_exact_original_release(){
 let vectors:serde_json::Value=serde_json::from_str(include_str!("../../../🛠️tool-machine/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let capacity=vectors["cause"]["capacity"].as_u64().unwrap()as usize;
 for capacity in[0,1,capacity]{
  let mut owner=Vec::<u64>::with_capacity(capacity);let address=owner.as_ptr();let original_capacity=owner.capacity();
  if capacity==0{assert!(ledger_backing_is_empty(&owner));continue;}
  let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ledger_empty_backing_demand(&owner).unwrap().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(demand.release_bytes,original_capacity*std::mem::size_of::<u64>());
  let admitted=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
  for denied in[RetainedCloneGrant{maximum_items:0,..admitted},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..admitted},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..admitted},RetainedCloneGrant{maximum_depth:0,..admitted}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_ledger_backing(&mut owner,denied).unwrap().unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.as_ptr(),address);assert_eq!(owner.capacity(),original_capacity);}
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_ledger_backing(&mut owner,admitted).unwrap().unwrap());assert!(step.progress().fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,demand.release_bytes));assert!(ledger_backing_is_empty(&owner));let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
 let mut original=vec![String::from("held original🧩")];let vector_pointer=original.as_ptr();let body_pointer=original[0].as_ptr();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_ledger_backing(&mut original,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:64}).unwrap());assert!(step.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.as_ptr(),vector_pointer);assert_eq!(original[0].as_ptr(),body_pointer);assert_eq!(original[0],"held original🧩");
}

#[test]
fn tool_job_outcome_original_pages_close_under_independent_common_grants(){
 use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, StepOutcome, receive_original_retained_work};
 use semio_framework_job_payload::{JobPayloadStream, RetainedPayloadBuilder};
 for pages in [0,1,3] {
  let stock=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
  let ((mut original,caller),setup)=semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(95101),Generation(8),stock,||Some(1),u64::MAX,100000);
   let mut builder=RetainedPayloadBuilder::new(JobPayloadStream::Preview);
   let bytes=&b"abc"[..pages];
   let mut cursor=0;
   let mut payload=None;
   for _ in 0..64 {
    caller.turn(RetainedCloneGrant{maximum_capacity_bytes:1,..stock},|cx| {
     let turn=if !builder.is_initialized(){builder.advance_initialization(cx)}
      else if cursor<bytes.len(){builder.append_original(cx,bytes,&mut cursor)}
      else if builder.published().is_none(){builder.seal(cx)}
      else{builder.take_finished(&mut payload,cx.retained_grant())};
     receive_original_retained_work(cx,turn)?;
     Ok(())
    }).unwrap();
    if payload.is_some(){break}
   }
   let payload=payload.expect("preview payload was published");
   assert_eq!(payload.page_count(),pages);
   (semio_framework_job::JobOutcomeSlot::from_outcome(StepOutcome::PreviewReady(payload)),caller)
  });
  let mut bank=crate::test_effect::bank();let mut physical=0;let mut turns=0;
  while !original.is_empty(){
   let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::step_outcome_slot_retirement_demands(&original).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes+demand.capacity_bytes<=4096);
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
   let address=original.original().map(|value|value as*const StepOutcome);let pointer=match original.original().unwrap(){StepOutcome::PreviewReady(payload)=>payload.page(0).map(|page|page.as_ptr()),_=>None};
   let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}
   for denied in denied{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::test_effect::settled(&mut bank,denied,|effect|semio_framework_value::RetainedWorkPoll::Complete(semio_framework_job::close_step_outcome_slot(&mut original,effect))).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.original().map(|value|value as*const StepOutcome),address);assert_eq!(match original.original().unwrap(){StepOutcome::PreviewReady(payload)=>payload.page(0).map(|page|page.as_ptr()),_=>None},pointer);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::test_effect::settled(&mut bank,grant,|effect|semio_framework_value::RetainedWorkPoll::Complete(semio_framework_job::close_step_outcome_slot(&mut original,effect))).result.unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));physical+=heap.released_bytes;turns+=1;assert!(turns<100000);
  }
  let (_,root_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.retire(stock));
  assert_eq!(physical+root_heap.released_bytes+setup.released_bytes,setup.requested_bytes+root_heap.requested_bytes);
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
}
