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
  let admitted=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  for denied in[RetainedCloneGrant{maximum_items:0,..admitted},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..admitted},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..admitted},RetainedCloneGrant{maximum_depth:0,..admitted}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_ledger_backing(&mut owner,denied).unwrap().unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.as_ptr(),address);assert_eq!(owner.capacity(),original_capacity);}
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_ledger_backing(&mut owner,admitted).unwrap().unwrap());assert!(step.progress().fits(admitted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,demand.release_bytes));assert!(ledger_backing_is_empty(&owner));let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
 let mut original=vec![String::from("held original🧩")];let vector_pointer=original.as_ptr();let body_pointer=original[0].as_ptr();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_ledger_backing(&mut original,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:usize::MAX,maximum_depth:64}).unwrap());assert!(step.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.as_ptr(),vector_pointer);assert_eq!(original[0].as_ptr(),body_pointer);assert_eq!(original[0],"held original🧩");
 println!("[DEBUG] tool ledger original empty0/1/8192 capacities demand0heap, zero/onebelowcopy/release/depth preserve pointers, exact physical backing release, terminalDrop0; nonempty original string and vector pointers remain unchanged");
}
