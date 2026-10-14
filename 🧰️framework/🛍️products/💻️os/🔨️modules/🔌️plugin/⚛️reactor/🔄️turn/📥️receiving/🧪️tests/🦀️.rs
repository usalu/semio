//! 📥️ Actual cancellation retains captured original inputs until their paid source return.
use super::*;

#[test]
fn original_receiving_cancelled_before_first_loan_retires_same_inputs_and_source(){
 for admitted in [false,true]{
  let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();
  let runtime=crate::plugin_runtime::PluginRuntime::<crate::plugin_app::NoPluginApp>::new(crate::app_testing::artifact_app_laws::fixture_mounted_policy()).unwrap();
  let mut cx=caller.context();let recipient=cx.original_recipient_identity();
  let mut owner=RetainedResponse{instance:971,key:instance_lifetime::NativeCloseKey::fixture(971,1),witness:ReceivingSource::new(&cx).unwrap(),fields:ManuallyDrop::new(Some((Some(String::new()),Some(semio_framework_value::DslValue::Null),None))),closing:ManuallyDrop::new(None),output:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),finished:false};
  if admitted{assert!(!owner.witness.admit(&mut cx).unwrap());assert!(owner.witness.loan.is_some());}
  ::semio_framework_async::poll::resolve_ready(cx.original_cancel_token().cancel());
  let mut identity=crate::app_testing::artifact_app_laws::fixture_identity();
  let fields=owner.fields.as_ref().unwrap()as*const _;let before=cx.retained_progress();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();
  let(zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,deadline,|same|{let result=::semio_framework_async::poll::resolve_ready(owner.step(&runtime,&mut identity,same))?;assert!(result.is_none());Ok(())}));
  zero.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(owner.fields.as_ref().unwrap()as*const _,fields);assert_eq!(owner.witness.loan.is_some(),admitted);
  for turn in 0..crate::app_testing::OriginalFixtureCaller::maximum_turns(){
   if owner.terminal(){break}
   let before=cx.retained_progress();
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||::semio_framework_async::poll::resolve_ready(owner.step(&runtime,&mut identity,&mut cx)));
   assert!(result.unwrap().is_none());let after=cx.retained_progress();
   assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));
   assert_eq!(cx.original_recipient_identity(),recipient);assert!(turn+1<crate::app_testing::OriginalFixtureCaller::maximum_turns());
  }
  assert!(owner.terminal());assert!(owner.witness.loan.is_none());
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
  eprintln!("[DEBUG] originalReceivingCancellation admittedLoan={admitted} zeroWorkSystem0=true exactCapturedInputs=true sameRecipient=true paidInputAndSourceClose=true terminalDrop0=true");
 }
}
