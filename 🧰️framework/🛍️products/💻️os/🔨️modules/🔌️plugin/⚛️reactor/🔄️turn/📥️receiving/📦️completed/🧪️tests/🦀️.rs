//! 🧪️ The native production receiver preserves its original byte owners and complete Fault through actual finite caller close.
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework::kernel::Event;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_plugin_actor_host::requests;
use semio_plugin_reactor_turn as turn;
use semio_framework_value::ToValue;
use semio_plugin_reactor_receiving_completed::CompletedInput;
use semio_plugin_reactor_receiving_completed::CompletedResult;

#[test]
fn original_completed_accepts_the_extension_pack_wire_body() {
    use semio_framework::kernel::{RequestId, RequestOutcome};

    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let wire_hex = law["replyWireHex"].as_str().unwrap();
    let wire: Vec<u8> = (0..wire_hex.len()).step_by(2).map(|offset| u8::from_str_radix(&wire_hex[offset..offset + 2], 16).unwrap()).collect();
    let mut expected = law["expected"].clone();
    expected["ok"] = serde_json::Value::Bool(true);
    expected["output"] = law["reply"].clone();
    let page_size = law["maximumPageBytes"].as_u64().unwrap() as usize;
    assert!(wire.len() > page_size);
    for split in [0, page_size] {
        let continuation = requests::ExtensionContinuation {
            instance: 971,
            response_action: "flowEvalResolve".into(),
            request_json: law["request"].to_string(),
            partial: Ok(wire[..split].to_vec()),
        };
        let event = Event::Completed { req: RequestId(71), result: RequestOutcome::Ok(wire[split..].to_vec()) };
        let mut input = CompletedInput::new(event, continuation);
        let mut caller = semio_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();
        let mut cx = caller.context();
        let maximum_turns = semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns();
        let result = (0..maximum_turns).find_map(|_| {
            if input.phase != 2 {
                return input.step(&mut cx).unwrap();
            }
            let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..cx.retained_grant() };
            cx.with_retained_loan(grant, |turn| {
                assert!(input.step(turn)?.is_none());
                Ok(())
            }).unwrap();
            None
        }).expect("extension wire answer reached its response action");
        let CompletedResult::Response(fields) = result else { panic!("expected response action"); };
        assert_eq!(fields.0.as_deref(), Some("flowEvalResolve"));
        assert_eq!(serde_json::Value::from(fields.1.as_ref().unwrap().clone()), expected);
        assert!(fields.2.is_none());

        let mut fields = ControlledRetirement::new(fields).map_err(|_| ()).unwrap();
        for _ in 0..maximum_turns {
            if fields.terminal_is_empty() { break; }
            semio_framework_job::close_original_effect_step(&mut cx, fields.selected_demand(64).unwrap(), |effect| fields.step(64, effect)).unwrap();
        }
        assert!(fields.terminal_is_empty());
        assert!((0..maximum_turns).any(|_| input.close_step(&mut cx).unwrap()));
        assert!(input.closed);
        let fuel = cx.fuel_remaining();
        drop(cx);
        caller.retire(fuel).unwrap();
    }
}


fn fault_wire(law:&serde_json::Value)->Vec<u8>{
 let hex=law["faultWireHex"].as_str().unwrap();
 (0..hex.len()).step_by(2).map(|offset|u8::from_str_radix(&hex[offset..offset+2],16).unwrap()).collect()
}

#[test]
fn original_completed_full_fault_same_input_zero_work_and_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let request=serde_json::to_string(&law["request"]).unwrap();let bytes=fault_wire(&law);let pointer=bytes.as_ptr();let request_pointer=request.as_ptr();
 let continuation=requests::ExtensionContinuation{instance:971,response_action:"receive-original".into(),request_json:request,partial:Ok(Vec::new())};
 let mut input=CompletedInput::new(Event::Completed{req:semio_framework::kernel::RequestId(71),result:semio_framework::kernel::RequestOutcome::Err(bytes)},continuation);
 let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let recipient=cx.original_recipient_identity();let before=cx.retained_progress();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,deadline,|same|input.step(same).map(|_|())));result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
 let mut fields=None;for turn in 0..semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.step(&mut cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));fields=result.unwrap();assert_eq!(cx.original_recipient_identity(),recipient);let sources=input.inputs.as_ref().unwrap();assert_eq!(sources.continuation.as_ref().unwrap().request_json.as_ptr(),request_pointer);let Event::Completed{result:semio_framework::kernel::RequestOutcome::Err(bytes),..}=sources.event.as_ref().unwrap()else{panic!()};assert_eq!(input.pack.as_ref().and_then(|owner|owner.input()).unwrap_or(bytes).as_ptr(),pointer);if fields.is_some(){break}assert!(turn+1<semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns())}
 let CompletedResult::Response(fields)=fields.unwrap()else{panic!()};assert_eq!(fields.0.as_deref(),Some("receive-original"));assert_eq!(serde_json::Value::from(fields.1.as_ref().unwrap().clone()),law["expected"]);assert_eq!(serde_json::Value::from(fields.2.as_ref().unwrap().to_value()),law["fault"]);
 let mut fields=ControlledRetirement::new(fields).map_err(|_|()).unwrap();for turn in 0..semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){if fields.terminal_is_empty(){break}let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::close_original_effect_step(&mut cx,fields.selected_demand(64).unwrap(),|effect|fields.step(64,effect)));result.unwrap();let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));assert!(turn+1<semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns())}assert!(fields.terminal_is_empty());drop(fields);
 for turn in 0..semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.close_step(&mut cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));if result.unwrap(){break}assert!(turn+1<semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns())}assert!(input.closed);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(input));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
}

#[test]
fn original_completed_parked_future_preserves_pages_and_same_source_until_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for failed in [false,true]{
  let partial=law["parked"]["partial"].as_str().unwrap().as_bytes().to_vec();let pointer=partial.as_ptr();let tail=if failed{fault_wire(&law)}else{law["parked"]["terminal"].as_str().unwrap().as_bytes().to_vec()};let tail_pointer=tail.as_ptr();let result=if failed{semio_framework::kernel::RequestOutcome::Err(tail)}else{semio_framework::kernel::RequestOutcome::Ok(tail)};
  let mut input=CompletedInput::new_request(Event::Completed{req:semio_framework::kernel::RequestId(72),result},partial);let mut caller=semio_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let before=cx.retained_progress();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();let(zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,deadline,|same|input.step(same).map(|_|())));zero.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
  let mut received=None;for turn in 0..semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){let sources=input.inputs.as_ref().unwrap();assert_eq!(sources.partial.as_ptr(),pointer);let Event::Completed{result,..}=sources.event.as_ref().unwrap()else{panic!()};let bytes=match result{semio_framework::kernel::RequestOutcome::Ok(bytes)|semio_framework::kernel::RequestOutcome::Err(bytes)=>bytes};assert_eq!(input.pack.as_ref().and_then(|owner|owner.input()).unwrap_or(bytes).as_ptr(),tail_pointer);let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.step(&mut cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));received=result.unwrap();if received.is_some(){break}assert!(turn+1<semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns())}
  let CompletedResult::Request(result)=received.unwrap()else{panic!()};if failed{assert_eq!(serde_json::Value::from(result.as_ref().unwrap_err().to_value()),law["fault"])}else{assert_eq!(result.as_ref().unwrap(),law["parked"]["expected"].as_str().unwrap().as_bytes())}
  let mut result=ControlledRetirement::new(result).map_err(|_|()).unwrap();for turn in 0..semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){if result.terminal_is_empty(){break}semio_framework_job::close_original_effect_step(&mut cx,result.selected_demand(64).unwrap(),|effect|result.step(64,effect)).unwrap();assert!(turn+1<semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns())}assert!(result.terminal_is_empty());drop(result);
  for turn in 0..semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||input.close_step(&mut cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));if result.unwrap(){break}assert!(turn+1<semio_plugin_testing_laws::OriginalFixtureCaller::maximum_turns())}assert!(input.closed);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(input));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
 }
}
