//! ⏸️ The facade owns the park laws because the park extension trait lives above the context crate.
use semio_framework_job::*;
use semio_framework_job_context::*;
use semio_framework_job_context::testkit;
use semio_framework_value_testing_allocator as _;
#[test]
fn genuine_owned_park_keeps_alias_on_post_refusal_and_never_renews_its_seven_fuel_donor(){
 let row:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let qrow:serde_json::Value=serde_json::from_str(include_str!("../../🎟️source/🎟️loan/📐️.json")).unwrap();let s=&qrow["stock"];let bank=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let quantum=RetainedCloneGrant{maximum_items:1,..bank};
 for refuse in [false,true]{
  let mut caller=semio_framework_job_testing_caller::OriginalJobFixtureCaller::admit(OperationId(17),Generation(9),bank,||Some(1000),12345,s["fuel"].as_u64().unwrap());
  caller.turn(bank,|cx|{testkit::admit_return_origin(cx)?;Ok(())}).unwrap();let mut output=None;
  caller.turn(bank,|cx|{
   cx.bind_original_source_epoch(19)?;let view=cx.retained_grant();cx.narrow_original_work_budget(StepBudget::new(row["initialFuel"].as_u64().unwrap(),12345,view))?;
   let original=cx.fuel_remaining();let actual=cx.original_source_parent()?;let result=cx.with_retained_loan(quantum,|cx|cx.with_original_work_loan(1,12345,|cx|{let maximum=testkit::retained_grant_mut(cx).maximum_items;testkit::set_loan_receipt_refusal(refuse);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actual.park_owned_turn(&mut output,cx));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));testkit::retained_grant_mut(cx).maximum_items=maximum;if refuse{let failure=result.unwrap_err();assert_eq!(failure.retained_progress().copied_items,1);}else{assert!(!result?);}Ok(())}));
   result?;assert!(output.is_some());assert!(!output.as_ref().unwrap().admission_is_ready());assert!(!actual.original_loans_are_returned());assert_eq!(cx.fuel_remaining(),original-1);
   let before=cx.retained_progress();let work=cx.original_work_units_spent();cx.with_retained_loan(quantum,|cx|cx.with_original_work_loan(1,12345,|cx|{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actual.park_owned_turn(&mut output,cx));assert!(!result?);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())}))?;assert_eq!(cx.retained_progress(),before);assert_eq!(cx.original_work_units_spent(),work);assert_eq!(cx.fuel_remaining(),original-1);
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actual.park_owned_turn(&mut output,cx));assert!(result?);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let owned=output.as_ref().unwrap();assert_eq!(owned.original_fuel(),row["ownedFuel"].as_u64().unwrap());assert_eq!(cx.fuel_remaining(),0);Ok(())
  }).unwrap();
  caller.cancel().cancel_now();for _ in 0..s["maximumTurns"].as_u64().unwrap(){if output.as_ref().unwrap().admission_is_empty(){break}caller.turn(quantum,|cx|output.as_mut().unwrap().close_admission_step(cx.original_source_parent()?,cx)).unwrap();}assert!(output.as_ref().unwrap().admission_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(output.take()));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(bank);
 }
}
