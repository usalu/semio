//! 🧪️ Actual window wrapper backing stays in custody until independently funded retirement.
use semio_framework_value::{RetainedCloneGrant,RetireOwned,retirement::{controlled::ControlledRetirement,OwnedValueRetirementFactory}};
use std::sync::Arc;
fn run<T:RetireOwned+Send+'static>(construct:impl FnOnce()->T,body:usize){
 let(original,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(construct);let held=heap.requested_bytes-heap.released_bytes;let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(original).map_err(|(error,_)|error).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(mut born,mut released)=(0,0);
 for turn in 0..1000000{if owner.terminal_is_empty(){break;}let demanded_copy=owner.next_copy_byte_demand().unwrap();let copy=demanded_copy.max(body);let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};
  let denied=[Some(RetainedCloneGrant{maximum_items:0,..grant}),(demanded_copy>0).then_some(RetainedCloneGrant{maximum_copy_bytes:0,..grant}),(grant.maximum_capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}),(grant.maximum_release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant}),(grant.maximum_depth>0).then_some(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant})];
  for under in denied.into_iter().flatten(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(under));if let Ok(step)=step{assert_eq!(step.progress(),Default::default());}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<999999);
 }
 assert!(owner.terminal_is_empty());assert_eq!(released,held+born);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Actual window wrapper original={held} born={born} released={released} terminalDrop=0 copy={body}");
}
#[test]
fn window_original_mutation_wrappers_have_exact_physical_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for body in law["copyGrants"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;for kind in ["canvas","layers"]{let text=law["payload"].as_str().unwrap();let capacity=law["capacity"].as_u64().unwrap()as usize;let window=law["windowId"].as_str().unwrap();
 run(||{let mut payload=String::with_capacity(capacity);payload.push_str(text);crate::window_config::WindowConfigMutation::from_issued(window,kind,payload,Arc::new(OwnedValueRetirementFactory::<String>::default()))},body);
 run(||{let mut payload=String::with_capacity(capacity);payload.push_str(text);crate::window_transient::WindowTransientMutation::from_issued(window,kind,payload,Arc::new(OwnedValueRetirementFactory::<String>::default()))},body);
 }}
}