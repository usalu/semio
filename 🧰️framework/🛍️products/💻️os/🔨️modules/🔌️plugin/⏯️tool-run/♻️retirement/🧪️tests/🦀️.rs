//! 🧰️ Empty original ledger backing requires exact physical admission; held elements are never erased.
use super::*;
#[test]
fn tool_ledger_empty_backing_preserves_undergrant_and_exact_original_release(){
 let vectors:serde_json::Value=serde_json::from_str(include_str!("../../../🛠️tool-machine/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
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
 println!("[DEBUG] tool ledger original empty0/1/8192 capacities demand0heap, zero/onebelowcopy/release/depth preserve pointers, exact physical backing release, terminalDrop0; nonempty original string and vector pointers remain unchanged");
}

#[test]
fn tool_job_outcome_original_pages_close_under_independent_common_grants(){
 use semio_framework_job::{StepOutcome,RetainedJobPayloadWriter,JobPayloadStream,StepContext,OperationId,Generation,StepBudget};
 for pages in [0,1,3]{
  let(mut original,setup)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let mut sequence=0;let mut ownership=RetainedCloneProgress::default();let mut context=StepContext::new(OperationId(95101),Generation(8),StepBudget::new(16,u64::MAX,RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:0,maximum_depth:64}),semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut sequence,&mut ownership);let mut writer=RetainedJobPayloadWriter::new(JobPayloadStream::Preview);
   for index in 0..pages{let mut page=writer.admit_page(&mut context).unwrap();page.write(if index==0{b""}else{b"original\0"}).unwrap();page.commit();}
   semio_framework_job::JobOutcomeSlot::from_outcome(StepOutcome::PreviewReady(writer.finish().unwrap()))
  });
  let mut physical=0;let mut turns=0;
  while !original.is_empty(){
   let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||tool_job_outcome_demands(&original).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes+demand.capacity_bytes<=4096);
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
   let address=original.original().map(|value|value as*const StepOutcome);let pointer=match original.original().unwrap(){StepOutcome::PreviewReady(payload)=>payload.page(0).map(|page|page.as_ptr()),_=>None};
   let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}
   for denied in denied{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_tool_job_outcome(&mut original,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.original().map(|value|value as*const StepOutcome),address);assert_eq!(match original.original().unwrap(){StepOutcome::PreviewReady(payload)=>payload.page(0).map(|page|page.as_ptr()),_=>None},pointer);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_tool_job_outcome(&mut original,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));physical+=heap.released_bytes;turns+=1;assert!(turns<100000);
  }
  assert_eq!(physical,setup.requested_bytes-setup.released_bytes);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] ToolRun original outcome pages={pages} turns={turns} physical={physical}, exact zero/below copy/release/depth preserve original pointers, full common receipts and terminalDrop0");
 }
}
