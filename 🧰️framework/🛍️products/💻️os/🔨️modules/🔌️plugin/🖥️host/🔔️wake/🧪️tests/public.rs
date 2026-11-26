//! 🧪️ Genuine original Host currency cannot renew itself or admit a second owner before exact receipt collection.
use semio_plugin_host_wake::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
include!("🌱️turns.rs");
#[path="../📃️policy/📥️operation/🪙️guest/🧪️tests/core-width.rs"]
mod core_width;
#[path="../📃️policy/📥️operation/🧪️tests/reserve-runtime.rs"]
mod reserve_runtime;

#[test]
fn original_host_production_driver_keeps_same_finite_treasury_across_all_wake_returns(){
 let policy:GuestRelayDriverPolicy=semio_framework_pack_json::from_json_str(include_str!("../📃️policy/🧪️tests/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let original=GuestRelayWakeDriver::new(policy,semio_framework_job::default_now_us,Box::new(||false)).unwrap();let pointer=&original as*const _;let initial=original.original_remaining().unwrap();let source=[83u8;41];let mut destination=[0u8;41];let source_pointer=source.as_ptr();let destination_pointer=destination.as_ptr();
 for index in 0..initial.maximum_items{
  let(turn,heap)=observe(||original.checkout(7,3).unwrap().unwrap());assert_eq!(heap,(0,0));assert_eq!(turn.epoch,index as u64+1);assert_eq!(turn.grant.maximum_items,policy.wake.maximum_items.min(initial.maximum_items-index));let(second,heap)=observe(||original.checkout(9,5).unwrap());assert!(second.is_none());assert_eq!(heap,(0,0));let actual=RetainedCloneProgress{copied_items:1,copied_bytes:41,..Default::default()};let receipt=GuestRelayWakeReceipt{epoch:turn.epoch,slot:7,generation:3,grant:turn.grant,progress:actual};
  for denied in [GuestRelayWakeReceipt{epoch:turn.epoch+1,..receipt},GuestRelayWakeReceipt{slot:9,..receipt},GuestRelayWakeReceipt{generation:5,..receipt},GuestRelayWakeReceipt{grant:RetainedCloneGrant{maximum_items:turn.grant.maximum_items+1,..turn.grant},..receipt},GuestRelayWakeReceipt{grant:RetainedCloneGrant{maximum_copy_bytes:turn.grant.maximum_copy_bytes+1,..turn.grant},..receipt},GuestRelayWakeReceipt{grant:RetainedCloneGrant{maximum_capacity_bytes:turn.grant.maximum_capacity_bytes+1,..turn.grant},..receipt},GuestRelayWakeReceipt{grant:RetainedCloneGrant{maximum_release_bytes:turn.grant.maximum_release_bytes+1,..turn.grant},..receipt},GuestRelayWakeReceipt{grant:RetainedCloneGrant{maximum_depth:turn.grant.maximum_depth+1,..turn.grant},..receipt}]{let(result,heap)=observe(||original.collect(&denied));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(&original as*const _,pointer);}
  let(_,heap)=observe(||destination.copy_from_slice(&source));assert_eq!(heap,(0,0));let(collected,heap)=observe(||original.collect(&receipt).unwrap());assert!(collected);assert_eq!(heap,(0,0));assert_eq!(source.as_ptr(),source_pointer);assert_eq!(destination.as_ptr(),destination_pointer);assert_eq!(destination,source);let remaining=original.original_remaining().unwrap();assert_eq!(remaining.maximum_items,initial.maximum_items-index-1);assert_eq!(remaining.maximum_copy_bytes,initial.maximum_copy_bytes-41*(index+1));assert_eq!(remaining.maximum_capacity_bytes,initial.maximum_capacity_bytes);assert_eq!(remaining.maximum_release_bytes,initial.maximum_release_bytes);assert_eq!(remaining.maximum_depth,initial.maximum_depth);let(duplicate,heap)=observe(||original.collect(&receipt));assert!(duplicate.is_err());assert_eq!(heap,(0,0));assert_eq!(original.original_remaining().unwrap(),remaining);
 }
 let(turn,heap)=observe(||original.checkout(7,3).unwrap());assert!(turn.is_none());assert_eq!(heap,(0,0));let(_,heap)=observe(||drop(original));assert_eq!(heap,(0,0));
}
