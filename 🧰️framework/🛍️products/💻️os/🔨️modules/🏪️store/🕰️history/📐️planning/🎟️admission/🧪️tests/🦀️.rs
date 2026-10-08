//! 🎟️ The real history frame constructor keeps original pointers until its exact admission.
use super::{admit_history_read_retirement,history_read_retirement_birth_bytes};
use super::super::{ArtifactDerivedHistoryPreview,HistoryReadPlan,ReplayProgress,SnapshotReadLeaseRegistry,artifact_retirement_box_close_step,artifact_retirement_box_demands,fixture_mutations::demo::DemoMutation,tests::DemoSnapshot};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,retirement::OwnedValueRetirementFactory,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},ValueRefusalKind};
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use std::{sync::Arc,marker::PhantomData};
#[test]
fn history_read_retirement_frame_preserves_original_preview_on_every_refusal() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let((mut original,snapshots,mutations),heap)=observe_heap_allocations_on_this_thread(||{
        let input=protocol::InputReplacement::Input {schema:String::with_capacity(law["unusedInputCapacity"].as_u64().unwrap()as usize),payload:Vec::with_capacity(law["unusedInputCapacity"].as_u64().unwrap()as usize)};
        let plan=HistoryReadPlan::new(Some(protocol::MutationId(law["originalTarget"].as_str().unwrap().into())),protocol::HistoryInputDrafts::from([(protocol::MutationId(law["originalTarget"].as_str().unwrap().into()),input)]),17,false);
        let original=Some(ArtifactDerivedHistoryPreview::<DemoSnapshot,DemoMutation> {plan:Some(plan),plan_retirement:None,drafts:protocol::HistoryInputDrafts::new(),target:None,cursor:0,operation:0,state:Some(Arc::new(DemoSnapshot {n:Some(law["snapshot"]["n"].as_i64().unwrap()as i32)})),progress:ReplayProgress::default(),registry:Arc::new(SnapshotReadLeaseRegistry::new()),generation:7,revision:[11;32],marker:PhantomData,finished:false});
        let snapshots:Arc<dyn ArtifactOwnedValueRetirementFactory<DemoSnapshot>>=Arc::new(OwnedValueRetirementFactory::<DemoSnapshot>::default());let mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<DemoMutation>>=Arc::new(OwnedValueRetirementFactory::<DemoMutation>::default());(original,snapshots,mutations)
    });let held=heap.requested_bytes-heap.released_bytes;
    let identity=Arc::as_ptr(original.as_ref().unwrap().state.as_ref().unwrap());let registry=Arc::as_ptr(&original.as_ref().unwrap().registry);let plan_target=original.as_ref().unwrap().plan.as_ref().unwrap().target.as_ref().unwrap().0.as_ptr();
    let(bytes,heap)=observe_heap_allocations_on_this_thread(||history_read_retirement_birth_bytes::<DemoSnapshot,DemoMutation>());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(bytes>0);
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1};let mut frame=None;
    for (index,case) in law["grantCases"].as_array().unwrap().iter().enumerate() {
        let grant=match case.as_str().unwrap() {"zeroItems"=>RetainedCloneGrant {maximum_items:0,..grant},"belowCapacity"=>RetainedCloneGrant {maximum_capacity_bytes:bytes-1,..grant},"zeroDepth"=>RetainedCloneGrant {maximum_depth:0,..grant},"full"=>grant,_=>unreachable!()};
        let(result,heap)=observe_heap_allocations_on_this_thread(||admit_history_read_retirement(&mut original,&snapshots,&mutations,grant,|owner,preview|*owner.preview=Some(preview)));
        assert_eq!(usize::from(original.is_none()),law["expectedTransfers"][index].as_u64().unwrap()as usize);
        match result {Err(error)=>{assert!(matches!(error.kind,ValueRefusalKind::WorkLimit|ValueRefusalKind::OwnershipLimit|ValueRefusalKind::DepthLimit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let original=original.as_ref().unwrap();assert_eq!(Arc::as_ptr(original.state.as_ref().unwrap()),identity);assert_eq!(Arc::as_ptr(&original.registry),registry);assert_eq!(original.plan.as_ref().unwrap().target.as_ref().unwrap().0.as_ptr(),plan_target);},Ok(Some((owner,progress)))=>{assert_eq!(progress,RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()});assert_eq!((heap.requested_bytes,heap.released_bytes),(bytes,0));frame=Some(owner);},Ok(None)=>panic!("original preview is present")}
    }
    let(_,heap)=observe_heap_allocations_on_this_thread(||drop((snapshots,mutations)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(mut allocated,mut released)=(bytes,0);
    for turn in 0..200000 {
        let(demand,heap)=observe_heap_allocations_on_this_thread(||artifact_retirement_box_demands(frame.as_ref().unwrap(),3).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:3.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let(step,heap)=observe_heap_allocations_on_this_thread(||artifact_retirement_box_close_step(&mut frame,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress().retained_capacity_bytes,heap.requested_bytes,"birth turn{turn}");assert_eq!(step.progress().released_bytes,heap.released_bytes,"release turn{turn}");allocated+=heap.requested_bytes;released+=heap.released_bytes;if frame.is_none(){break;}assert!(turn<199999,"admitted history frame stalled");
    }
    assert_eq!(released,held+allocated);assert!(frame.is_none());let(_,heap)=observe_heap_allocations_on_this_thread(||drop((frame,original)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    println!("[DEBUG] actual history frame constructor refusal preserves original snapshot/registry/target pointers and empty8192 input backing0heap; exact admitted Box birth; original factories retained; full nested cleanup and frame release exact; terminal Drop0heap");
}
