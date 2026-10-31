//! 🪪️ Genuine active and physically retired parents lend exact zero-effect qualification refusals.
use super::*;
use crate::original_failure_test_support::{projected,wire_value,wire_progress};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use serde_json::{Value,json};
fn clock()->Option<u64>{Some(0)}
#[test]
fn readonly_source_qualification_keeps_both_exact_native_value_refusals(){
 let law:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy:Value=serde_json::from_str(include_str!("../../../../../⚠️failure/🎟️work/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let stock:RetainedCloneGrant=serde_json::from_value(policy["stock"].clone()).unwrap();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(law["operation"].as_u64().unwrap()),Generation(law["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());
 let mut other=OriginalJobFixtureCaller::admit(OperationId(law["foreignOperation"].as_u64().unwrap()),Generation(law["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());
 for _ in 0..64{if other.parent.as_ref().unwrap().source_identity().is_err(){break}other.retire_step(stock).unwrap();}
 assert!(other.parent.as_ref().unwrap().source_identity().is_err());
 caller.turn(stock,|cx|{
  let before=cx.retained_progress();let fuel=cx.fuel_remaining();let identity=(cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity());let parent=cx.original_source_parent()?;
  let(result,heap)=observe(||parent.validate_borrowed_context(cx));result?;assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  cx.operation=OperationId(law["foreignOperation"].as_u64().unwrap());let(result,heap)=observe(||parent.validate_borrowed_context(cx));cx.operation=identity.0;let foreign=result.unwrap_err();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let(result,heap)=observe(||other.parent.as_ref().unwrap().validate_borrowed_context(cx));let retiring=result.unwrap_err();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for(name,error)in[("foreign",foreign),("retiring",retiring)]{let expected=&law["expected"][name];assert_eq!(error.kind.as_str(),expected["kind"].as_str().unwrap());assert_eq!(error.message,expected["message"].as_str().unwrap());assert_eq!(error.retained_progress(),Default::default());let native=OriginalTurnFailure::from(error);assert_eq!(projected(&native,&mut Vec::new()),json!({"primary":{"kind":"value","original":wire_value(expected["kind"].as_str().unwrap(),expected["message"].as_str().unwrap(),Default::default())},"effects":wire_progress(Default::default())}));}
  assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!((cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity()),identity);Ok(())
 }).unwrap();
 other.retire(stock);caller.retire(stock);
}
