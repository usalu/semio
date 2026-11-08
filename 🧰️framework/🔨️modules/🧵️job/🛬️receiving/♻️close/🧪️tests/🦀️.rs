//! 🧪️ Actual decoder retirement returns its finite Source alias after pure recipient and issuer closure.
use semio_framework_job_context::*;
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
#[global_allocator]
static CLOSE_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[test]
fn actual_original_decoder_receipt_closes_only_on_distinct_paid_metadata_turns(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let s=&row["stock"];let stock=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let q=RetainedCloneGrant{maximum_items:1,..stock};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(row["operation"].as_u64().unwrap()),Generation(row["generation"].as_u64().unwrap()),stock,||Some(1),u64::MAX,s["fuel"].as_u64().unwrap());let mut owner=OriginalJobDecodeOwner::empty();for _ in 0..s["maximumTurns"].as_u64().unwrap(){if caller.turn(q,|cx|owner.admit_step(cx)).unwrap(){break}}assert!(owner.source_loan_admitted()&&owner.work_issuer_admitted());
 let value=caller.turn_with_decoder(q,&mut owner,|cx|{let mut out=None;let mut produced=None;cx.with_native_receiving(&mut out,|_,_|{produced=Some(17u64);Ok::<_,semio_framework_value::ValueError>(())})?;cx.consume_fuel(1);Ok(produced)}).unwrap();assert_eq!(serde_json::to_value(value).unwrap(),row["value"]);let identity=owner.receiving_identity();assert!(identity.is_some());assert!(!owner.is_closing());
 let before=caller.fuel_remaining();let done=caller.turn(RetainedCloneGrant{maximum_items:0,..q},|cx|owner.close_on_source(cx)).unwrap();assert!(!done);assert_eq!(before,caller.fuel_remaining());assert_eq!(owner.receiving_identity(),identity);assert!(!owner.is_closing());let mut work=0u64;
 for _ in 0..s["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let before=caller.fuel_remaining();let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(q,|cx|owner.close_on_source(cx)).unwrap());assert_eq!(heap.requested_bytes,caller.receipt().retained_capacity_bytes);assert_eq!(heap.released_bytes,caller.receipt().released_bytes);let actual=before-caller.fuel_remaining();assert!(actual<=1);work+=actual;if done{break}}
 assert!(owner.terminal_is_empty());assert!(work>=row["minimumWork"].as_u64().unwrap());let before=caller.fuel_remaining();assert!(caller.turn(q,|cx|owner.close_on_source(cx)).unwrap());assert_eq!(before,caller.fuel_remaining());drop(owner);caller.retire(stock);
}
