//! 🔁️ The published Math progress transfers the Edit preview latch and emits its exact compact resume.

use semio_framework_job::{close_original_effect_step, Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::{DslValue, Number, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, retained_clone::RetainedCloneGrant};
use semio_os_flow_host_session::{FlowEvalSession, FlowEvaluationReplyAddress, FlowEvaluationReplyReceiver, FlowInvocationOrigin, invocation_publication::FlowInvocationDispatchWork};

fn fixed_clock() -> Option<u64> {
    Some(1)
}

fn bounded_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth.max(1),
    }
}

#[test]
fn captured_math_progress_rearms_the_edit_preview_and_dispatches_a_compact_resume() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔁️math.json")).unwrap();
    let request = &fixture["request"];
    let text = |key: &str| request[key].as_str().unwrap().to_owned();
    let origin = FlowInvocationOrigin {
        window_id: text("windowId"),
        window_kind_id: text("windowKindId"),
        neuron_id: text("neuronId"),
        extension_id: text("extensionId"),
        operator_id: text("operatorId"),
        node_hash: request["nodeHash"].as_u64().unwrap(),
        input_json: text("inputJson"),
        dependency_json: text("dependencyJson"),
        operator_version: text("operatorVersion"),
        outer_node_hash: None,
        preview_source: None,
    };
    let address = FlowEvaluationReplyAddress { node_hash: origin.node_hash, window_id: &origin.window_id, window_kind_id: &origin.window_kind_id };
    let reply = &fixture["reply"];
    let expected_reply = [
        ("cancellable", DslValue::Bool(reply["cancellable"].as_bool().unwrap())),
        ("done", DslValue::Bool(reply["done"].as_bool().unwrap())),
        ("output", DslValue::Null),
        ("phase", DslValue::String(reply["phase"].as_str().unwrap().into())),
        ("unitsDone", DslValue::Number(Number::UInt(reply["unitsDone"].as_u64().unwrap()))),
        ("unitsTotal", DslValue::Number(Number::UInt(reply["unitsTotal"].as_u64().unwrap()))),
    ];
    let hex = fixture["wireHex"].as_str().unwrap();
    let wire: Vec<u8> = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap()).collect();
    let source = semio_os_kernel::os_pack::pack_rt::decode_wire_value(&wire).unwrap();
    assert!(matches!(&source, DslValue::Object(rows) if rows.len() == expected_reply.len()));
    for (key, value) in expected_reply {
        assert_eq!(source.get(key), Some(&value));
    }
    let mut session = FlowEvalSession::new();
    let stock = RetainedCloneGrant {
        maximum_items: 65_536,
        maximum_copy_bytes: 128 << 20,
        maximum_capacity_bytes: 256 << 20,
        maximum_release_bytes: 512 << 20,
        maximum_depth: 4096,
    };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(706), Generation(9), stock, fixed_clock, 1500, 65_536);
    let admission = RetainedCloneGrant { maximum_items: 16, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1 << 16, maximum_release_bytes: 1 << 16, maximum_depth: 32 };
    session.arm_window_tick(address.window_id, admission).unwrap();
    session.begin_window_tick(address.window_id, admission).unwrap();
    session.note_window_tick_outcome(address.window_id, true, admission).unwrap();
    session.note_window_extensions_in_flight(address.window_id, 1, admission).unwrap();
    assert!(!session.window_tick_owed(address.window_id));

    let mut receiver = {
        let grant = bounded_grant(FlowEvaluationReplyReceiver::native_intake_demands());
        caller.turn(grant, |cx| {
            Ok(FlowEvaluationReplyReceiver::new(source, cx).unwrap_or_else(|_| panic!("captured reply intake")))
        }).unwrap()
    };
    for _ in 0..256 {
        if receiver.terminal_is_empty() { break }
        let grant = bounded_grant(receiver.next_demands(&session, 64, address.window_id).unwrap());
        caller.turn(grant, |cx| receiver.step(&mut session, &address, cx)).unwrap();
        assert!(caller.receipt().fits(grant));
    }
    assert!(receiver.terminal_is_empty());
    assert_eq!(session.window_extensions_in_flight(address.window_id), 0);
    assert!(session.window_tick_owed(address.window_id));
    assert!(session.arm_owed_window_tick(address.window_id, admission).unwrap().0);
    assert!(!session.arm_owed_window_tick(address.window_id, admission).unwrap().0);
    session.begin_window_tick(address.window_id, admission).unwrap();

    let mut dispatch = FlowInvocationDispatchWork::new(session.has_evaluation_progress(origin.node_hash));
    let resumed = (0..2048).find_map(|_| {
        let grant = bounded_grant(dispatch.next_demands(&origin, 64).unwrap());
        caller.turn(grant, |cx| dispatch.step(&origin, cx)).unwrap()
    }).expect("the retained request producer reaches publication");
    assert_eq!((resumed.extension_id.as_str(), resumed.capability.as_str(), resumed.response_action.as_str()),
               ("flow-extension-math", "evaluate", "flowEvalResolve"));
    let mut expected = request.clone();
    expected["resume"] = true.into();
    expected["inputJson"] = "".into();
    expected["dependencyJson"] = "".into();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&resumed.request_json).unwrap(), expected);

    dispatch.restore_output(resumed);
    dispatch.begin_close();
    for _ in 0..2048 {
        if dispatch.terminal_is_empty() { break }
        let demand = dispatch.next_close_demands(64).unwrap();
        let grant = bounded_grant(demand);
        caller.retirement_turn(grant, |cx| {
            close_original_effect_step(cx, demand, |effect| RetainedWorkPoll::Complete(RetainedWorkOutcome {
                result: dispatch.close_step(grant, effect),
                work: 1,
            }))
        }).unwrap();
    }
    assert!(dispatch.terminal_is_empty());
    session.begin_close();
    for _ in 0..2048 {
        if session.terminal_is_empty() { break }
        let demand = RetirementDemand {
            copy_bytes: session.next_close_copy_byte_demand().unwrap(),
            capacity_bytes: session.next_close_capacity_byte_demand(64).unwrap(),
            release_bytes: session.next_close_release_byte_demand().unwrap(),
            depth: session.next_close_depth_demand().unwrap(),
        };
        caller.retirement_turn(bounded_grant(demand), |cx| {
            close_original_effect_step(cx, demand, |effect| RetainedWorkPoll::Complete(session.retire_source_step(effect)))
        }).unwrap();
    }
    assert!(session.terminal_is_empty());
    caller.retire(stock);
}
