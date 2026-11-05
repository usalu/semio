//! 🏡️ Real native Source custody precedes arena allocation and survives cancellation at every partial opening.
use super::*;
use semio_framework_job::{Generation,OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
static REGISTRY:UiArenaRegistry=UiArenaRegistry::new();
fn clock()->Option<u64>{Some(1)}
#[test]
fn ui_arena_lifetime_owns_every_partial_allocation_before_publication(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();let s=&corpus["stock"];let grant=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};
 for prefix in corpus["cancelPrefixes"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain([None]){
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(841),Generation(1),grant,clock,10000000,s["fuel"].as_u64().unwrap());let mut foreign=OriginalJobFixtureCaller::admit(OperationId(842),Generation(1),grant,clock,10000000,1000000);let (mut input,heap)=observe(||Some(UiArenaLifetime::new(&REGISTRY)));assert_eq!(heap,(0,0));let mut lease=None;let mut born=0;let mut freed=0;
  let zero=RetainedCloneGrant{maximum_capacity_bytes:0,..grant};let(result,heap)=observe(||caller.turn(zero,|cx|UiArenaLifetime::admit_source(&mut input,&mut lease,cx)));assert!(!result.unwrap());assert!(input.is_some()&&lease.is_none());assert_eq!(heap,(0,0));
  for _ in 0..100{let(result,heap)=observe(||caller.turn(grant,|cx|UiArenaLifetime::admit_source(&mut input,&mut lease,cx)));let receipt=caller.receipt();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;if result.unwrap(){break}}
  assert!(input.is_none()&&lease.is_some());assert!(REGISTRY.borrow().is_err());assert!(foreign.turn(grant,|cx|UiArenaLifetime::advance_on_source(lease.as_mut().unwrap(),cx)).is_err());
  for at in 0..100000{if prefix==Some(at){break}let(result,heap)=observe(||caller.turn(grant,|cx|UiArenaLifetime::advance_on_source(lease.as_mut().unwrap(),cx)));let receipt=caller.receipt();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;if result.unwrap(){assert!(prefix.is_none());assert!(REGISTRY.borrow().unwrap().is_some());break}assert!(at<99999)}
  caller.cancel().cancel_now();while !lease.as_ref().unwrap().terminal_is_empty(){caller.turn(grant,|cx|lease.as_mut().unwrap().return_step(cx)).unwrap();}
  for at in 0..100000{let(result,heap)=observe(||caller.turn(grant,|cx|cx.close_original_native_step()));let receipt=caller.receipt();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;if result.unwrap(){break}assert!(at<99999)}assert!(REGISTRY.borrow().is_err());assert_eq!(born,freed);drop(lease);caller.retire(grant);foreign.retire(grant);
 }
}
