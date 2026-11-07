//! 🧪️ Pure receiving headers and denied cleanup preserve authentic empty ownership.
use super::*;
#[test]
fn paged_receiver_headers_are_allocation_free_and_denied_close_is_zero_work(){
 let fixture:serde_json::Value=serde_json::from_str::<serde_json::Value>(include_str!("../../../🎮️append/🎟️work/📐️.json")).unwrap()["receiveLaw"].clone();
 assert_eq!(fixture["rules"]["ownershipCopyBytes"],0);
 let (mut text,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||<PagedUtf8<8> as NativeReceive<crate::DslValue>>::receiver());
 assert_eq!(heap,(0,0));assert!(!text.terminal_is_empty());text.begin_close();
 let denied=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
 let turn=text.close_step(denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());
 let (mut map,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||<super::super::PagedMap<u64,8> as NativeReceive<crate::DslValue>>::receiver());
 assert_eq!(heap,(0,0));assert!(map.terminal_is_empty());map.begin_close();let turn=map.close_step(denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());
}
