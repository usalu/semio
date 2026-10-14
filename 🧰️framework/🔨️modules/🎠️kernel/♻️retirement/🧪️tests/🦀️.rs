use semio_framework::ui::kernel::*;
use semio_framework_actor::cold_pair::ColdDocumentPairFrontier;
use semio_framework_value::{RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_job::{StepContext,StepBudget,OperationId,Generation,close_original_retained_step,CancelToken};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observed;
#[global_allocator]
static ORIGINAL_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn drain<T:RetireOwned>(source:T,original_bytes:usize,grant:RetainedCloneGrant,root:&CancelToken,sequence:&mut u64){
 let(mut owner,heap)=observed(||ControlledRetirement::new(source).unwrap_or_else(|_|panic!("original typed kernel custody admitted")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut born=original_bytes;let mut released=0;
 for turn in 0..100000{if owner.terminal_is_empty(){break}let demand=owner.selected_demand(grant.maximum_copy_bytes).unwrap();
  let mut receipt=Default::default();let mut denied=StepContext::new(OperationId(71001),Generation(3),StepBudget::new(0,12345,grant),root.clone(),||Some(1),sequence,&mut receipt);
  let(step,heap)=observed(||close_original_retained_step(&mut denied,demand,|grant|owner.step(grant)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));drop(denied);
  for denied_grant in [if demand.capacity_bytes!=0{RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant}}else{RetainedCloneGrant{maximum_items:0,..grant}},if demand.release_bytes!=0{RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}}else{RetainedCloneGrant{maximum_items:0,..grant}}]{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(71001),Generation(3),StepBudget::new(1,12345,denied_grant),root.clone(),||Some(1),sequence,&mut receipt);let(step,heap)=observed(||close_original_retained_step(&mut cx,demand,|grant|owner.step(grant)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(71001),Generation(3),StepBudget::new(1,12345,grant),root.clone(),||Some(1),sequence,&mut receipt);
  let(step,heap)=observed(||close_original_retained_step(&mut cx,demand,|grant|owner.step(grant)).unwrap());assert_eq!(step.progress(),cx.retained_progress());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<99999);
 }assert!(owner.terminal_is_empty());assert_eq!(born,released);let(_,heap)=observed(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
#[test]
fn original_kernel_wire_owners_retire_every_owned_field_through_the_original_context(){
 use base64::Engine;
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let(root,root_birth)=CancelToken::admit_root(grant).unwrap().unwrap();let mut sequence=0;
 let lifetime=ActorInstanceLifetime{activation_generation:1,instance_id:2,guest_lifetime:3};
 drain(CapabilityToken(9),0,grant,&root,&mut sequence);drain(ActorInstanceOpenRequest{activation_generation:1,instance_id:2,request_sequence:3},0,grant,&root,&mut sequence);drain(ActorInstanceCloseRequest{lifetime,request_sequence:4},0,grant,&root,&mut sequence);drain(ActorInstanceLifecycleAck{receipt:ActorInstanceLifecycleReceipt::Captured{lifetime,request_sequence:5}},0,grant,&root,&mut sequence);drain(ActorUiPatchReceipt{lifetime,patch_sequence:6},0,grant,&root,&mut sequence);
 let(id,heap)=observed(||AppInstanceId(law["appId"].as_str().unwrap().to_owned()));drain(id,heap.requested_bytes-heap.released_bytes,grant,&root,&mut sequence);
 let(page,heap)=observed(||FixedCommandPage::try_copy_from(b"original").unwrap());assert_eq!(heap.requested_bytes,law["expected"]["pageBacking"].as_u64().unwrap()as usize);assert_eq!(base64::engine::general_purpose::STANDARD.encode(page.as_slice()),law["expected"]["pageBase64"].as_str().unwrap());drain(page,heap.requested_bytes-heap.released_bytes,grant,&root,&mut sequence);
 drain(CommandPageCursor{owner:1,generation:1,command_index:0,command_count:1,instance:2,seq:3,kind:0,page_index:0,page_count:1,item_count:1,metadata:0},0,grant,&root,&mut sequence);
 let(page,heap)=observed(||ColdDocumentPairPage{header:ColdDocumentPairHeader{lifetime,transfer_generation:1,descriptor_sha256:[1;32],baseline_frontier:ColdDocumentPairFrontier{document_id:law["documentId"].as_str().unwrap().to_owned(),head_edit_ordinal:1,head_edit_id:law["editId"].as_str().unwrap().to_owned(),last_commit_seq:1,chain_sha256:[2;32]},pack_sha256:[3;32],spr_sha256:[4;32],aggregate_sha256:[5;32],pack_length:4,spr_length:4,page_count:1},page_index:0,bytes:b"original".to_vec()});drain(page,heap.requested_bytes-heap.released_bytes,grant,&root,&mut sequence);
 let mut root=semio_framework_async::CancelTokenRetirement::from_token(root);let mut released=0;while !root.terminal_is_empty(){let(step,heap)=observed(||root.close_step(grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));released+=heap.released_bytes;}assert_eq!(released,root_birth.retained_capacity_bytes);assert_eq!(observed(||drop(root)).1.released_bytes,0);eprintln!("[DEBUG] Original kernel wire owners preserve denied originals, pay all concrete Strings/page4096/frontier/Vec fields once, same context work+retained receipts, terminalDrop0");
}
