//! 📥️ Actual cancellation retains captured original inputs until their paid source return.
use super::*;

#[test]
fn original_receiving_cancelled_before_first_loan_retires_same_inputs_and_source(){
 for admitted in [false,true]{
  let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();
  let runtime=crate::plugin_runtime::PluginRuntime::<crate::plugin_app::NoPluginApp>::new(crate::app_testing::artifact_app_laws::fixture_mounted_policy()).unwrap();
  let mut cx=caller.context();let recipient=cx.original_recipient_identity();
  let mut owner=RetainedResponse{instance:Some(971),key:Some(instance_lifetime::NativeCloseKey::fixture(971,1)),witness:ReceivingSource::new(&cx).unwrap(),request:None,input:ManuallyDrop::new(None),fields:ManuallyDrop::new(Some((Some(String::new()),Some(semio_framework_value::DslValue::Null),None))),request_result:ManuallyDrop::new(None),request_closing:ManuallyDrop::new(None),closing:ManuallyDrop::new(None),output:ManuallyDrop::new(None),failure:ManuallyDrop::new(None),finished:false};
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

/// 📨️ A stale completion keeps its actual source in the donor through every unpaid frontier.
#[test]
fn original_completed_stale_donor_zero_work_preserved_and_actual_source_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📦️completed/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let bytes=serde_json::to_vec(&law["fault"]).unwrap();let pointer=bytes.as_ptr();let req=semio_framework::kernel::RequestId(u64::MAX);let mut donor=Some(Event::Completed{req,result:semio_framework::kernel::RequestOutcome::Err(bytes)});
 let runtime=crate::plugin_runtime::PluginRuntime::<crate::plugin_app::NoPluginApp>::new(crate::app_testing::artifact_app_laws::fixture_mounted_policy()).unwrap();let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let before=cx.retained_progress();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,deadline,|same|retain_completed(&runtime,&mut donor,same).map(|_|())));result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);let Some(Event::Completed{result:semio_framework::kernel::RequestOutcome::Err(bytes),..})=donor.as_ref()else{panic!()};assert_eq!(bytes.as_ptr(),pointer);assert!(!response_pending());
 for turn in 0..crate::app_testing::OriginalFixtureCaller::maximum_turns(){let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retain_completed(&runtime,&mut donor,&mut cx));result.unwrap();let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));if donor.is_none(){break}let Some(Event::Completed{result:semio_framework::kernel::RequestOutcome::Err(bytes),..})=donor.as_ref()else{panic!()};assert_eq!(bytes.as_ptr(),pointer);assert!(turn+1<crate::app_testing::OriginalFixtureCaller::maximum_turns())}assert!(donor.is_none());
 let mut identity=crate::app_testing::artifact_app_laws::fixture_identity();let mut effects=Vec::new();for turn in 0..crate::app_testing::OriginalFixtureCaller::maximum_turns(){if !response_pending(){break}let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||::semio_framework_async::poll::resolve_ready(advance_responses(&runtime,&mut identity,&mut effects,&mut cx)));result.unwrap();let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));assert!(turn+1<crate::app_testing::OriginalFixtureCaller::maximum_turns())}assert!(!response_pending());assert!(effects.is_empty());let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();eprintln!("[DEBUG] originalCompletedStale sameRawDonorUntilAdmission=true zeroWorkSystem0=true noInventedLifetime=true paidDecoderAndSourceClose=true terminalSlotReturned=true");
}
