use semio_framework::kernel::*;
use semio_framework_actor::cold_pair::ColdDocumentPairFrontier;
use semio_framework_value::{RetainedCloneGrant,RetirementDemand,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_job::{OperationId,Generation,InteractiveJobCloseStep,OriginalJobFixtureCaller,close_original_effect_step};
use semio_framework_trace::{HeapAllocationObservation,observe_heap_allocations_on_this_thread as observed};
const OPERATION:OperationId=OperationId(71001);
fn clock()->Option<u64>{Some(1)}
fn fixture(stock:RetainedCloneGrant,fuel:u64)->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OPERATION,Generation(3),stock,clock,u64::MAX,fuel)}
fn turn<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,owner:&mut ControlledRetirement<T>,demand:RetirementDemand,grant:RetainedCloneGrant)->(InteractiveJobCloseStep,HeapAllocationObservation){
 let(result,heap)=observed(||caller.bare_turn(grant,|cx|close_original_effect_step(cx,demand,|effect|owner.step(effect.grant().maximum_copy_bytes,effect))));(result.unwrap(),heap)
}
fn drain<T:RetireOwned>(source:T,original_bytes:usize,grant:RetainedCloneGrant,caller:&mut OriginalJobFixtureCaller,starved:&mut OriginalJobFixtureCaller){
 let(mut owner,heap)=observed(||ControlledRetirement::new(source).unwrap_or_else(|_|panic!("original typed kernel custody admitted")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut born=original_bytes;let mut released=0;
 for iteration in 0..100000{if owner.terminal_is_empty(){break}let demand=owner.selected_demand(grant.maximum_copy_bytes).unwrap();
  let before=starved.cumulative_receipt();let(step,heap)=turn(starved,&mut owner,demand,grant);assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(starved.cumulative_receipt(),before);
  for denied in [if demand.capacity_bytes!=0{RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant}}else{RetainedCloneGrant{maximum_items:0,..grant}},if demand.release_bytes!=0{RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}}else{RetainedCloneGrant{maximum_items:0,..grant}}]{let before=caller.cumulative_receipt();let(step,heap)=turn(caller,&mut owner,demand,denied);assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.cumulative_receipt(),before);}
  let(step,heap)=turn(caller,&mut owner,demand,grant);assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert!(iteration<99999);
 }assert!(owner.terminal_is_empty());assert_eq!(born,released);let(_,heap)=observed(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
#[test]
fn original_kernel_wire_owners_retire_every_owned_field_through_the_original_context(){
 use base64::Engine;
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let stock=RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:128};let mut caller=fixture(stock,1<<40);let mut starved=fixture(stock,0);
 let lifetime=ActorInstanceLifetime{activation_generation:1,instance_id:2,guest_lifetime:3};
 drain(CapabilityToken(9),0,grant,&mut caller,&mut starved);drain(ActorInstanceOpenRequest{activation_generation:1,instance_id:2,request_sequence:3},0,grant,&mut caller,&mut starved);drain(ActorInstanceCloseRequest{lifetime,request_sequence:4},0,grant,&mut caller,&mut starved);drain(ActorInstanceLifecycleAck{receipt:ActorInstanceLifecycleReceipt::Captured{lifetime,request_sequence:5}},0,grant,&mut caller,&mut starved);drain(ActorUiPatchReceipt{lifetime,patch_sequence:6},0,grant,&mut caller,&mut starved);
 let(id,heap)=observed(||AppInstanceId(law["appId"].as_str().unwrap().to_owned()));drain(id,heap.requested_bytes-heap.released_bytes,grant,&mut caller,&mut starved);
 let(page,heap)=observed(||FixedCommandPage::try_copy_from(b"original").unwrap());assert_eq!(heap.requested_bytes,law["expected"]["pageBacking"].as_u64().unwrap()as usize);assert_eq!(base64::engine::general_purpose::STANDARD.encode(page.as_slice()),law["expected"]["pageBase64"].as_str().unwrap());drain(page,heap.requested_bytes-heap.released_bytes,grant,&mut caller,&mut starved);
 drain(CommandPageCursor{owner:1,generation:1,command_index:0,command_count:1,instance:2,seq:3,kind:0,page_index:0,page_count:1,item_count:1,metadata:0},0,grant,&mut caller,&mut starved);
 let(page,heap)=observed(||ColdDocumentPairPage{header:ColdDocumentPairHeader{lifetime,transfer_generation:1,descriptor_sha256:[1;32],baseline_frontier:ColdDocumentPairFrontier{document_id:law["documentId"].as_str().unwrap().to_owned(),head_edit_ordinal:1,head_edit_id:law["editId"].as_str().unwrap().to_owned(),last_commit_seq:1,chain_sha256:[2;32]},pack_sha256:[3;32],spr_sha256:[4;32],aggregate_sha256:[5;32],pack_length:4,spr_length:4,page_count:1},page_index:0,bytes:b"original".to_vec()});drain(page,heap.requested_bytes-heap.released_bytes,grant,&mut caller,&mut starved);
 caller.retire(stock);starved.retire(stock);
}
