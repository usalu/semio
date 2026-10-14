#[global_allocator]
static ORIGINAL_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_ui::wgpu::PreparedRasterReservation;
use semio_framework_value::RetainedCloneGrant;

#[test]
fn original_prepared_raster_refusal_preserves_unfunded_backings_and_exact_release(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
 let source=vec![3;law["sourceBytes"].as_u64().unwrap()as usize];
 let retained=vec![7;law["retainedBytes"].as_u64().unwrap()as usize];
 let key=law["key"].as_str().unwrap().to_owned();
 let expected=source.capacity()+retained.capacity()+key.capacity();
 assert_eq!(expected,law["expected"]["releasedBytes"].as_u64().unwrap()as usize);
 let reservation=PreparedRasterReservation::try_reserve_source(key,source.len()+retained.len()).unwrap();
 let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reservation.reject_with_retained("original fixture raster refusal",source,retained));
 assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let mut released=0;
 for _ in 0..100000{
  if owner.terminal_is_empty(){break}
  let demand=owner.next_close_demands(grant.maximum_copy_bytes).unwrap();
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());
  assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  if demand.maximum_release_bytes!=0{
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(RetainedCloneGrant{maximum_release_bytes:demand.maximum_release_bytes-1,..grant}).unwrap());
   assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());
  let progress=step.progress();assert!(progress.fits(grant));
  assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));released+=heap.released_bytes;
 }
 assert!(owner.terminal_is_empty());assert_eq!(released,expected);
 let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 eprintln!("[DEBUG] Original raster refusal admission0/unfunded0/whole paid release={released}/terminalDrop0");
}
