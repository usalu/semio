//! 🧪️ Actual staged Pack receiving preserves exact private request owners on the original caller.
use semio_framework_plugin_reactor_turn::original_receiving::staged::StagedRequestInput;
use semio_framework::action_staging::StagedFormLimits;
use semio_framework_value::{DslValue,retirement::controlled::ControlledRetirement};
thread_local!{static STAGED_CLOCK:std::cell::Cell<u64>=const{std::cell::Cell::new(1000)};}
fn staged_clock()->Option<u64>{Some(STAGED_CLOCK.get())}

#[test]
fn staged_input_expired_deadline_closes_only_under_original_retirement(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/receiving-cases.json")).unwrap();
 let stock:semio_framework_value::RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
 STAGED_CLOCK.set(fixture["clockUs"].as_u64().unwrap());
 let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(97531),semio_framework_job::Generation(17),stock,staged_clock,fixture["deadlineUs"].as_u64().unwrap(),fixture["fuel"].as_u64().unwrap());
 let row=serde_json::json!({"token":{"action":"ApplyTemplate","generation":"1","revision":"r1","window":"w1"},"args":{"rows":["a","b","c"]},"locale":"en"});
 let mut donor=Some(semio_framework_os_kernel::os_pack::pack_rt::encode_wire_value(&DslValue::from(&row)));
 let mut input=caller.turn(stock,|cx|StagedRequestInput::admit(&mut donor,StagedFormLimits::FRAMEWORK,1<<20,cx)).unwrap().unwrap();
 for _ in 0..fixture["prefix"].as_u64().unwrap(){assert!(caller.turn(stock,|cx|input.step(cx)).unwrap().is_none());}
 STAGED_CLOCK.set(fixture["expiredClockUs"].as_u64().unwrap());
 let before=caller.cumulative_receipt();assert!(!caller.turn(stock,|cx|input.close_step(cx)).unwrap());assert_eq!(caller.cumulative_receipt(),before);
 for turn in 0..fixture["maximumTurns"].as_u64().unwrap(){
  let before=caller.cumulative_receipt();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|cx.with_original_retirement(|same|input.close_step(same))));
  let after=caller.cumulative_receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));
  if result.unwrap(){break}assert!(turn+1<fixture["maximumTurns"].as_u64().unwrap());
 }
 assert!(input.terminal_is_empty());drop(input);caller.retire(stock);STAGED_CLOCK.set(1000);
}

#[test]
fn staged_request_original_pack_census_move_and_close_keep_exact_receipts(){
 let row=serde_json::json!({"token":{"action":"ApplyTemplate","generation":"9007199254740993","revision":"r1","window":"w1"},"args":{"id":"18446744073709551615","bits":"8000000000000000"},"locale":"de","page":{"fieldOffset":"16","diagnosticOffset":"0","choice":{"path":["entries","0","value"],"offset":"32"}}});
 let bytes=semio_framework_os_kernel::os_pack::pack_rt::encode_wire_value(&DslValue::from(&row));let pointer=bytes.as_ptr();let mut donor=Some(bytes);
 let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let recipient=cx.original_recipient_identity();
 let mut input=StagedRequestInput::admit(&mut donor,StagedFormLimits::FRAMEWORK,1<<20,&mut cx).unwrap().unwrap();assert!(donor.is_none());
 let before=cx.retained_progress();let deadline=cx.deadline_us();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,deadline,|same|input.step(same).map(|_|())));result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);
 let mut output=None;
 for turn in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){
  assert_eq!(input.source_bytes().unwrap().as_ptr(),pointer);assert_eq!(cx.original_recipient_identity(),recipient);let before=cx.retained_progress();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.step(&mut cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));
  output=result.unwrap();if output.is_some(){break}assert!(turn+1<semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns());
 }
 assert_eq!(serde_json::to_value(output.as_ref().unwrap()).unwrap(),row);
 let mut output=ControlledRetirement::new(output).map_err(|_|()).unwrap();
 for _ in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){if output.terminal_is_empty(){break}semio_framework_job::close_original_retained_step(&mut cx,output.selected_demand(32768).unwrap(),|grant|output.step(grant)).unwrap();}
 assert!(output.terminal_is_empty());drop(output);
 for turn in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){
  let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.close_step(&mut cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));
  if result.unwrap(){break}assert!(turn+1<semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns());
 }
 assert!(input.terminal_is_empty());drop(input);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
}

#[test]
fn staged_request_cancelled_prefixes_retire_on_the_same_original_source(){
 for cut in [0,1,2,8,32,128,512,1024]{
  let row=serde_json::json!({"token":{"action":"ApplyTemplate","generation":"1","revision":"r1","window":"w1"},"args":{"rows":["a","b","c"]},"locale":"en","page":{"fieldOffset":"0","diagnosticOffset":"0","choice":{"path":["entries","0","value"],"offset":"32"}}});
  let mut donor=Some(semio_framework_os_kernel::os_pack::pack_rt::encode_wire_value(&DslValue::from(&row)));
  let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let mut input=StagedRequestInput::admit(&mut donor,StagedFormLimits::FRAMEWORK,1<<20,&mut cx).unwrap().unwrap();let mut output=None;
  for _ in 0..cut{output=input.step(&mut cx).unwrap();if output.is_some(){break}}
  cx.original_cancel_token().cancel_now();let before=cx.retained_progress();assert!(input.step(&mut cx).unwrap().is_none());assert_eq!(cx.retained_progress(),before);
  let mut output=ControlledRetirement::new(output).map_err(|_|()).unwrap();
  for _ in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){if output.terminal_is_empty(){break}semio_framework_job::close_original_retained_step(&mut cx,output.selected_demand(32768).unwrap(),|grant|output.step(grant)).unwrap();}
  assert!(output.terminal_is_empty());drop(output);
  for turn in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){
   let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.close_step(&mut cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));
   if result.unwrap(){break}assert!(turn+1<semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns());
  }
  assert!(input.terminal_is_empty());assert!(input.step(&mut cx).is_err());drop(input);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
 }
}

#[test]
fn staged_request_admission_keeps_denied_donors_and_rejects_substitute_callers(){
 let row=serde_json::json!({"token":{"action":"ApplyTemplate","generation":"1","revision":"r1","window":"w1"},"args":{},"locale":"en"});
 let mut donor=Some(semio_framework_os_kernel::os_pack::pack_rt::encode_wire_value(&DslValue::from(&row)));let pointer=donor.as_ref().unwrap().as_ptr();
 let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let grant=cx.retained_grant();
 for denied in [semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:0,..grant},semio_framework_value::RetainedCloneGrant{maximum_depth:0,..grant}]{
  let before=cx.retained_progress();cx.with_retained_loan(denied,|same|{assert!(StagedRequestInput::admit(&mut donor,StagedFormLimits::FRAMEWORK,1<<20,same)?.is_none());Ok(())}).unwrap();assert_eq!(cx.retained_progress(),before);assert_eq!(donor.as_ref().unwrap().as_ptr(),pointer);
 }
 let mut input=StagedRequestInput::admit(&mut donor,StagedFormLimits::FRAMEWORK,1<<20,&mut cx).unwrap().unwrap();
 let mut foreign=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut other=foreign.context();let before=other.retained_progress();assert!(input.step(&mut other).is_err());assert_eq!(other.retained_progress(),before);let fuel=other.fuel_remaining();drop(other);foreign.retire(fuel).unwrap();
 assert_eq!(input.source_bytes().unwrap().as_ptr(),pointer);
 for turn in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){if input.close_step(&mut cx).unwrap(){break}assert!(turn+1<semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns());}
 assert!(input.terminal_is_empty());drop(input);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
}
