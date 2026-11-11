//! 🪪️ Genuine active and physically retired parents lend exact zero-effect qualification refusals.
use semio_framework_job_context::*;
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use crate::original_failure_test_support::{projected,wire_value,wire_progress};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use serde_json::{Value,json};
fn clock()->Option<u64>{Some(0)}
fn assert_refusal(law:&Value,name:&str,result:Result<(),semio_framework_value::ValueError>,heap:semio_framework_trace::HeapAllocationObservation){
 assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let error=result.unwrap_err();let expected=&law["expected"][name];assert_eq!(error.kind.as_str(),expected["kind"].as_str().unwrap());assert_eq!(error.message,expected["message"].as_str().unwrap());assert_eq!(error.retained_progress(),Default::default());
 let native=OriginalTurnFailure::from(error);assert_eq!(projected(&native,&mut Vec::new()),json!({"primary":{"kind":"value","original":wire_value(expected["kind"].as_str().unwrap(),expected["message"].as_str().unwrap(),Default::default())},"effects":wire_progress(Default::default())}));
}
#[test]
fn readonly_source_qualification_keeps_both_exact_native_value_refusals(){
 let law:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy:Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let stock:RetainedCloneGrant=serde_json::from_value(policy["stock"].clone()).unwrap();
 let admit=|operation:&str|OriginalJobFixtureCaller::admit(OperationId(law[operation].as_u64().unwrap()),Generation(law["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());
 let mut caller=admit("operation");let mut foreign=admit("foreignOperation");let mut other=admit("foreignOperation");
 for _ in 0..64{if other.source_parent().source_identity().is_err(){break}other.retire_step(stock).unwrap();}
 assert!(other.source_parent().source_identity().is_err());
 foreign.bare_turn(stock,|foreign_cx|{
  let before=foreign_cx.retained_progress();let fuel=foreign_cx.fuel_remaining();
  let(result,heap)=observe(||caller.source_parent().validate_borrowed_context(foreign_cx));assert_refusal(&law,"foreign",result,heap);
  assert_eq!(foreign_cx.retained_progress(),before);assert_eq!(foreign_cx.fuel_remaining(),fuel);Ok(())
 }).unwrap();
 caller.bare_turn(stock,|cx|{
  let before=cx.retained_progress();let fuel=cx.fuel_remaining();let identity=(cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity());let parent=cx.original_source_parent()?;
  let(result,heap)=observe(||parent.validate_borrowed_context(cx));result?;assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let(result,heap)=observe(||other.source_parent().validate_borrowed_context(cx));assert_refusal(&law,"retiring",result,heap);
  assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!((cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity()),identity);Ok(())
 }).unwrap();
 other.retire(stock);foreign.retire(stock);caller.retire(stock);
}
