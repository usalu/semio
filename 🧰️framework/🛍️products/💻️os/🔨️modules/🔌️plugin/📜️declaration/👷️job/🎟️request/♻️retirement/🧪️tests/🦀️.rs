use super::*;
use semio_framework_value::{RetainedWorkPoll,RetainedWorkOutcome};
#[test]
fn original_request_shared_field_returns_its_alias_without_waiting_for_live_store(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["owned"].as_array().unwrap().len(),19);
 let source=Arc::new(vec![1u8,3,7,11]);let same=Arc::clone(&source);let owner=OriginalRequestLease(same);let quote=owner.retirement_birth_bytes().unwrap();
 let(mut cursor,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.retirement_seed());assert_eq!(quote,size_of::<ArcLeaseRetirement<Vec<u8>>>());assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));
 let grant=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
 let mut bank=semio_framework_value_testing_effect_bank::EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::new(grant,1));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,1,|effect|RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(cursor.close_step(0,effect)),work:1})).0);let RetainedWorkPoll::Complete(step)=step else{panic!("original shared alias closes in one effect")};let step=step.result.unwrap();let RetirementStep::Progress(progress,_)=step else{panic!("original shared alias must make paid custody progress")};assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(progress.copied_items,1);assert_eq!(Arc::strong_count(&source),1);assert!(cursor.terminal_is_empty());assert_eq!(source.as_slice(),&[1,3,7,11]);
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
