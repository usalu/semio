use super::*;
#[test]
fn original_request_shared_field_returns_its_alias_without_waiting_for_live_store(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["owned"].as_array().unwrap().len(),19);
 let source=Arc::new(vec![1u8,3,7,11]);let same=Arc::clone(&source);let owner=OriginalRequestLease(same);let quote=owner.retirement_birth_bytes().unwrap();
 let(mut cursor,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.retirement());assert_eq!((birth.requested_bytes,birth.released_bytes),(quote,0));
 let grant=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
 let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(grant));let RetirementStep::Progress(progress)=step else{panic!("original shared alias must make paid custody progress")};assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(progress.copied_items,1);assert_eq!(Arc::strong_count(&source),1);assert!(cursor.terminal_is_empty());assert_eq!(source.as_slice(),&[1,3,7,11]);
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,quote));eprintln!("[DEBUG] original request snapshot/config/history/interaction leases return same aliases while live store remains, constructor/terminal backing match heap witness");
}
