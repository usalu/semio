//! 🧪️ Genuine Original turns preserve complete body and posting owners before paid erasure.
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use crate::{OriginalWorkFailure,OriginalDomainReservation,OriginalDomainView as V,OperationId,Generation,admit_original_domain_failure};
use semio_framework_diagnostic::FaultMessageOwner;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress as P};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use serde_json::{Value,json};
use crate::original_failure_test_support::*;
fn reference(row:&Value,ordinal:u64,progress:P)->Value{
 let mut result=serde_json::Map::new();
 if row["domain"].as_bool().unwrap(){result.insert("primary".into(),independent_body(ordinal,row["message"].as_str().unwrap()));}
 if row["receiptRefusal"].as_bool().unwrap(){result.insert("receiptRefusal".into(),wire_value("invariantViolated","job child receipt exceeds original remaining retained grant",progress));}
 if row["contextRefusal"].as_bool().unwrap()||row["beforeRefusal"].as_bool().unwrap(){result.insert("contextRefusal".into(),json!({"primary":{"kind":"value","original":wire_value("invariantViolated","owned original reply requires its genuine source receiver",P::default())},"effects":wire_progress(P::default())}));}
 result.insert("effects".into(),wire_progress(progress));Value::Object(result)
}
#[test]
fn actual_original_work_preserves_body_receipt_context_and_owned_success(){
 let law:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let stock:RetainedCloneGrant=serde_json::from_value(law["stock"].clone()).unwrap();let operation=OperationId(law["operation"].as_u64().unwrap());let foreign=OperationId(law["foreignOperation"].as_u64().unwrap());let generation=Generation(law["generation"].as_u64().unwrap());let bound=law["maximumCloseTurns"].as_u64().unwrap()as usize;
 for(ordinal,row)in law["cases"].as_array().unwrap().iter().enumerate(){
  let(mut caller,heap)=observe(||OriginalJobFixtureCaller::admit(operation,generation,stock,clock,law["deadlineUs"].as_u64().unwrap(),law["fuel"].as_u64().unwrap()));assert!(!heap.overflowed);
  let(mut other,heap)=observe(||OriginalJobFixtureCaller::admit(foreign,generation,stock,clock,law["deadlineUs"].as_u64().unwrap(),law["fuel"].as_u64().unwrap()));assert!(!heap.overflowed);other.turn(stock,|cx|{assert_eq!(cx.operation(),foreign);Ok(())}).unwrap();
  caller.turn(stock,|cx|{
   let identity=(cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity(),cx.original_clock_binding());let before=cx.retained_progress();let fuel=cx.fuel_remaining();
   if row["beforeRefusal"].as_bool().unwrap(){
    let(result,heap)=observe(||semio_framework_job_context::testkit::with_original_work_refusals(cx,Some(foreign),None,|cx|OriginalWorkFailure::<BodyFailure>::qualify(cx)));let failure=result.unwrap_err();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(projected(&failure,&mut Vec::new()),reference(row,ordinal as u64,P::default()));close(failure,cx,bound);return Ok(())
   }
   let mut reservation=OriginalDomainReservation::<OriginalWorkFailure<BodyFailure>>::new();assert!(reservation.prepare(cx)?);let before=cx.retained_progress();let fuel=cx.fuel_remaining();OriginalWorkFailure::<BodyFailure>::qualify(cx).unwrap();let mut output=None;let mut primary=None;let mut actual=P::default();let mut actual_cpu=0;let mut pointer=None;
   if row["execute"].as_bool().unwrap(){
    let(birth,heap)=observe(||FaultMessageOwner::construct(row["message"].as_str().unwrap(),cx.retained_grant()));actual_cpu=birth.work;let(owner,p)=birth.result.unwrap();assert!(!heap.overflowed);assert!(owner.ready());assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(p.copied_bytes,row["message"].as_str().unwrap().len()+"app.message".len());pointer=Some((owner.fault().unwrap().message.as_ptr()as usize,&*owner.fault().unwrap().scope as*const _ as usize));actual=p;let body=BodyFailure{ordinal:ordinal as u64,owner,progress:p};if row["domain"].as_bool().unwrap(){primary=Some(body)}else{output=Some(body)}
   }
   let capacity=row["receiptRefusal"].as_bool().unwrap().then_some(before.retained_capacity_bytes);let refused_operation=row["contextRefusal"].as_bool().unwrap().then_some(foreign);
   let cpu=actual_cpu;assert_eq!(cpu,u64::from(row["execute"].as_bool().unwrap()));let(result,heap)=observe(||semio_framework_job_context::testkit::with_original_work_refusals(cx,refused_operation,capacity,|cx|OriginalWorkFailure::fold(cx,primary,actual,cpu)));
   assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(delta(before,cx.retained_progress()),actual);assert_eq!(fuel-cx.fuel_remaining(),cpu);assert_eq!((cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity(),cx.original_clock_binding()),identity);
   let expects=row["domain"].as_bool().unwrap()||row["receiptRefusal"].as_bool().unwrap()||row["contextRefusal"].as_bool().unwrap();assert_eq!(result.is_err(),expects,"{}",row["name"]);
   if let Err(failure)=result{
    assert_eq!(failure.retained_progress(),actual);assert_eq!(projected(&failure,&mut Vec::new()),reference(row,ordinal as u64,actual));
    if let Some(body)=failure.primary(){assert_eq!((body.owner.fault().unwrap().message.as_ptr()as usize,&*body.owner.fault().unwrap().scope as*const _ as usize),pointer.unwrap());}
    let expected=projected(&failure,&mut Vec::new());assert!(reservation.with_body(cx,|_,slot|{*slot=Some(failure);Ok(())})?.is_some());let posted=cx.retained_progress();let admission_fuel=cx.fuel_remaining();let(result,heap)=observe(||admit_original_domain_failure(&mut reservation,cx));let admitted=result?.unwrap();assert!(reservation.terminal_is_empty());assert_eq!(admitted.retained_progress(),P::default());assert_eq!(admitted.primary.retained_progress(),P::default());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(delta(posted,cx.retained_progress()),P{copied_items:1,..Default::default()});assert_eq!(admission_fuel-cx.fuel_remaining(),1);let projection=admitted.primary_domain().unwrap().projection();assert_eq!(projected(projection,&mut Vec::new()),expected);if row["domain"].as_bool().unwrap(){assert!(matches!(projection.projection_view(&[0,1,3])?,V::IntrinsicText(text)if text.as_ptr()as usize==pointer.unwrap().0));}close(admitted,cx,bound);
   }
   if let Some(body)=output{assert_eq!((body.owner.fault().unwrap().message.as_ptr()as usize,&*body.owner.fault().unwrap().scope as*const _ as usize),pointer.unwrap());assert_eq!(projected(&body,&mut Vec::new()),independent_body(ordinal as u64,row["message"].as_str().unwrap()));close(body,cx,bound);}
   for _ in 0..bound{if reservation.terminal_is_empty(){break}let d=reservation.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;crate::close_original_retained_step(cx,d,|grant|reservation.close_step(grant))?;}assert!(reservation.terminal_is_empty());
   Ok(())
  }).unwrap();
  let(_,heap)=observe(||other.retire(stock));assert!(!heap.overflowed);let(_,heap)=observe(||caller.retire(stock));assert!(!heap.overflowed);
 }
}
