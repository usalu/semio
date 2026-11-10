//! 🧪️ Pure receiving headers and denied close start preserve authentic empty ownership.
use super::*;
fn denied_begin<R:NativeReceiver<crate::DslValue,T>,T>(receiver:&mut R){
    let denied=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
    let turn=crate::effect_fixture::settled(crate::effect_fixture::with_effect(denied,1<<20,&mut None,|effect|{let mut slot=None;let reply=crate::retained_clone::OriginalEffectReply::borrow(&mut slot)?;receiver.begin_close_into(effect,reply)?;slot.take().ok_or_else(||ValueError::invariant("denied close start stayed pending"))}));
    assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(!receiver.is_closing());
}
#[test]
fn paged_receiver_headers_are_allocation_free_and_denied_close_start_is_zero_work(){
 let fixture:serde_json::Value=serde_json::from_str::<serde_json::Value>(include_str!("../../../🎮️append/🎟️work/📐️.json")).unwrap()["receiveLaw"].clone();
 assert_eq!(fixture["rules"]["ownershipCopyBytes"],0);
 let (mut text,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||<PagedUtf8<8> as NativeReceive<crate::DslValue>>::receiver());
 assert_eq!(heap,(0,0));assert!(!text.terminal_is_empty());denied_begin::<_,PagedUtf8<8>>(&mut text);
 let (mut map,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||<super::super::PagedMap<u64,8> as NativeReceive<crate::DslValue>>::receiver());
 assert_eq!(heap,(0,0));assert!(map.terminal_is_empty());denied_begin::<_,super::super::PagedMap<u64,8>>(&mut map);
}
