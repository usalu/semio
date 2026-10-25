use neural_engine::ColdRetire;
use super::*;

#[test]
fn flow_source_read_comparison_matches_original_serde_oracle_and_retains_frontier_on_denial(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["comparisons"].as_array().unwrap(){
  let right=row.get("right").unwrap_or(&row["left"]);
  let a:Tree=semio_framework_pack_json::from_json_str(&row["left"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let b:Tree=semio_framework_pack_json::from_json_str(&right.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let expected=row["left"]==*right;assert_eq!(expected,row["equal"].as_bool().unwrap());
  let mut compare=SourceCompare::new();let mut born=0;let mut freed=0;
  for _ in 0..100000{
   if compare.result().is_some(){break}let d=compare.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
   let frontier=compare.frames.len();let bytes=compare.frames.allocated_bytes();
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||compare.step(&a,&b,denied));assert_eq!(result.unwrap(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((compare.frames.len(),compare.frames.allocated_bytes()),(frontier,bytes));
   }
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||compare.step(&a,&b,grant));let p=result.unwrap();assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=p.retained_capacity_bytes;
  }
  assert_eq!(compare.result(),Some(expected),"{}",row["name"]);compare.begin_close();
  for _ in 0..100000{if compare.terminal_is_empty(){break}let d=compare.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
   if d.release_bytes>0{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||compare.close_step(RetainedCloneGrant{maximum_release_bytes:0,..grant}));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||compare.close_step(grant));let p=result.unwrap().progress();assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));freed+=p.released_bytes;
  }
  assert!(compare.terminal_is_empty());assert_eq!(born,freed);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(compare)).1.released_bytes,0);a.retire_cold();b.retire_cold();
 }
}
