//! 📨️ Drives donated preview replies through the same retained controller as the app.
use semio_framework_job::{Generation, OriginalJobFixtureCaller, OriginalTurnFailure, allocate_operation_id};
use semio_framework_value::{DslValue, Number, RetainedCloneGrant, RetirementDemand, ToValue};
use semio_os_flow::FlowEvalSession;
use semio_s_artifact_procedural_3d_preview_eval::{FlowEvalResolve, FlowTessellateResolve, PreviewEvalReceive, PreviewTessellateReceive};

pub const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 256 << 20,
    maximum_release_bytes: 512 << 20, maximum_depth: 4096,
};
pub const TURN_LIMIT: usize = 250_000;
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 4 * TURN_LIMIT, maximum_copy_bytes: 1 << 30, ..TURN
};

pub fn lod_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).expect("canonical preview fixture")
}

pub fn evaluation_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("📨️evaluation.json")).expect("typed evaluation fixture")
}

pub fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(allocate_operation_id(), Generation(1), STOCK, || Some(1000), 1500, TURN_LIMIT as u64)
}

fn quoted(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_copy_bytes: demand.copy_bytes.max(TURN.maximum_copy_bytes),
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth.max(1),
        ..TURN
    }
}

fn retire_payload(payload: impl semio_framework_value::RetireOwned, caller: &mut OriginalJobFixtureCaller) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(payload)
        .unwrap_or_else(|_| panic!("preview reply payload retirement"));
    for _ in 0..TURN_LIMIT {
        if owner.terminal_is_empty() { return; }
        let demand = owner.selected_demand(TURN.maximum_copy_bytes).expect("reply close demand");
        caller.turn(quoted(demand), |cx| semio_framework_job::close_original_effect_step(cx, demand, |effect| {
            owner.step(TURN.maximum_copy_bytes, effect)
        })).expect("reply payload closes");
    }
    panic!("reply payload did not retire");
}

pub fn receive(session: &mut FlowEvalSession, mut payload: FlowTessellateResolve) -> Result<(), OriginalTurnFailure> {
    if session.preview_tessellate_is_pending(payload.node_hash) {
        session.admit_window_tick_latch(&payload.window_id, TURN)?;
    }
    let mut caller = caller();
    let mut controller = PreviewTessellateReceive::new();
    let result = (|| {
        assert!(caller.turn(TURN, |cx| controller.receive(&mut payload, cx))?);
        for _ in 0..TURN_LIMIT {
            let grant = quoted(controller.demands(&payload, session, TURN.maximum_copy_bytes)?);
            if caller.turn(grant, |cx| controller.step(&payload, session, cx))? { return Ok(()); }
        }
        panic!("original tessellation receipt did not reach its ledger")
    })();
    controller.begin_close();
    for _ in 0..TURN_LIMIT {
        if controller.terminal_is_empty() { break; }
        let grant = quoted(controller.close_demands(TURN.maximum_copy_bytes).expect("preview receiver close demand"));
        caller.turn(grant, |cx| controller.close_step(cx)).expect("preview receiver closes");
    }
    assert!(controller.terminal_is_empty());
    retire_payload(payload, &mut caller);
    let stock = caller.stock();
    caller.retire(stock);
    result
}

pub fn payload(window: (&str, &str), handle: &str, tolerance: f64, output: DslValue) -> FlowTessellateResolve {
    FlowTessellateResolve {
        window_id: window.0.into(), window_kind_id: window.1.into(), handle: handle.into(),
        node_hash: semio_os_flow::preview_tessellate_node_hash(handle, tolerance.to_bits()),
        tolerance, output, fault: None, original_input: None, input_valid: true,
    }
}

pub fn response(window: (&str, &str), handle: &str, tolerance: f64, phase: &str, units: (u64, u64), mesh: Option<&semio_framework::MeshData>) -> FlowTessellateResolve {
    let hash = semio_os_flow::preview_tessellate_node_hash(handle, tolerance.to_bits());
    let mut fields = vec![
        ("done".into(), DslValue::Bool(mesh.is_some())), ("cancellable".into(), DslValue::Bool(mesh.is_none())),
        ("phase".into(), DslValue::String(phase.into())),
        ("unitsDone".into(), DslValue::Number(Number::UInt(units.0))),
        ("unitsTotal".into(), DslValue::Number(Number::UInt(units.1))),
        ("nodeHash".into(), DslValue::Number(Number::UInt(hash))),
        ("windowId".into(), DslValue::String(window.0.into())),
        ("windowKindId".into(), DslValue::String(window.1.into())),
        ("handle".into(), DslValue::String(handle.into())),
        ("tolerance".into(), DslValue::Number(Number::Float(tolerance))),
    ];
    if let Some(mesh) = mesh { fields.push(("mesh".into(), mesh.to_value())); }
    payload(window, handle, tolerance, DslValue::Object(fields))
}

pub fn dto(value: serde_json::Value) -> DslValue {
    match value {
        serde_json::Value::Null => DslValue::Null,
        serde_json::Value::Bool(value) => DslValue::Bool(value),
        serde_json::Value::Number(value) => DslValue::Number(value.as_u64().map(Number::UInt)
            .unwrap_or_else(|| Number::Float(value.as_f64().unwrap()))),
        serde_json::Value::String(value) => DslValue::String(value),
        serde_json::Value::Array(values) => DslValue::Array(values.into_iter().map(dto).collect()),
        serde_json::Value::Object(values) => DslValue::Object(values.into_iter().map(|(key, value)| (key, dto(value))).collect()),
    }
}

pub fn invoke(
    owner: &semio_s_plugin_flow_brep::geometry_inference::GeometryInferenceContext,
    request: &[u8], observe: impl FnMut(u64),
) -> Result<Vec<u8>, OriginalTurnFailure> {
    invoke_original(|cx| owner.tessellate_raw(request, cx), observe)
}

pub fn invoke_original(
    mut advance: impl FnMut(&mut semio_framework_job::StepContext<'_>) -> Result<semio_plugin::ExtensionInvokeStep, semio_framework::Fault>,
    mut observe: impl FnMut(u64),
) -> Result<Vec<u8>, OriginalTurnFailure> {
    let mut caller = caller();
    let result = (|| {
        for _ in 0..TURN_LIMIT {
            let started = std::time::Instant::now();
            let step = caller.refreshed_turn(TURN, |cx| advance(cx).map_err(Into::into))?;
            observe(started.elapsed().as_micros() as u64);
            if let Some(failure) = step.refusal { return Err(failure); }
            if let Some(bytes) = step.payload { return Ok(bytes); }
        }
        panic!("native preview extension did not publish a canonical answer")
    })();
    let stock = caller.stock();
    caller.retire(stock);
    result
}

pub fn deliver(
    owner: &semio_s_plugin_flow_brep::geometry_inference::GeometryInferenceContext,
    session: &mut FlowEvalSession, window: (&str, &str), handle: &str, tolerance: f64, observe: impl FnMut(u64),
) -> Result<usize, OriginalTurnFailure> {
    let hash = semio_os_flow::preview_tessellate_node_hash(handle, tolerance.to_bits());
    assert!(session.note_pending_tessellate(hash, handle.to_owned(), tolerance));
    let request = serde_json::to_vec(&serde_json::json!({
        "windowId":window.0, "windowKindId":window.1, "nodeHash":hash, "handle":handle, "tolerance":tolerance
    })).expect("valid preview request");
    let bytes = invoke(owner, &request, observe)?;
    let output = dto(serde_json::from_slice(&bytes).expect("canonical preview response"));
    receive(session, payload(window, handle, tolerance, output))?;
    assert!(session.preview_mesh_for_tolerance(handle, tolerance).is_some());
    Ok(bytes.len())
}

pub fn standalone_mesh(
    owner: &semio_s_plugin_flow_brep::geometry_inference::GeometryInferenceContext,
    handle: &str, tolerance: f64,
) -> Result<semio_framework::MeshData, OriginalTurnFailure> {
    let mut session = FlowEvalSession::new();
    let result = deliver(owner, &mut session, ("composition-preview", "preview"), handle, tolerance, |_| {})
        .map(|_| session.preview_mesh_for_tolerance(handle, tolerance).expect("completed canonical mesh").clone());
    session.retire_cold();
    result
}

pub fn cancel(session: &mut FlowEvalSession, window: &str) {
    session.cancel_preview_evaluation(window, TURN).expect("preview cancellation admitted");
    let mut caller = caller();
    for _ in 0..TURN_LIMIT {
        if session.preview_cancellation_terminal_is_empty() { break; }
        let demand = session.next_preview_cancellation_demands(TURN.maximum_copy_bytes).unwrap();
        caller.turn(quoted(demand), |cx| semio_framework_job::close_original_retained_effect_step(
            cx, demand, |effect| session.preview_cancellation_step(effect),
        )).expect("preview cancellation advances");
    }
    assert!(session.preview_cancellation_terminal_is_empty());
    let stock = caller.stock();
    caller.retire(stock);
}

pub fn retain(session: &mut FlowEvalSession, handles: Vec<String>) {
    session.begin_retain_preview_meshes(handles).expect("preview retention admitted");
    let mut caller = caller();
    for _ in 0..TURN_LIMIT {
        if session.preview_retention_terminal_is_empty() { break; }
        let demand = session.next_preview_retention_demands(TURN.maximum_copy_bytes).unwrap();
        caller.turn(quoted(demand), |cx| semio_framework_job::close_original_retained_effect_step(
            cx, demand, |effect| session.retain_preview_meshes_step(effect),
        )).expect("preview retention advances");
    }
    assert!(session.preview_retention_terminal_is_empty());
    let stock = caller.stock();
    caller.retire(stock);
}

pub fn record_evaluate_fault(session: &mut FlowEvalSession, fault: semio_os_flow::ExtensionEvaluateFault) {
    let mut fault = Some(fault);
    let mut caller = caller();
    let demand = RetirementDemand { depth: 1, ..Default::default() };
    caller.turn(TURN, |cx| semio_framework_job::close_original_retained_effect_step(cx, demand, |effect| {
        session.note_extension_evaluate_fault(&mut fault, effect)
            .map(|receipt| semio_framework_value::RetainedCloneStep::Progress(receipt.unwrap_or_default()))
    })).expect("original evaluation fault folds");
    assert!(fault.is_none());
    let stock = caller.stock();
    caller.retire(stock);
}

pub fn evaluation_payload(window: (&str, &str), node_hash: u64, extension: &str, output: DslValue) -> FlowEvalResolve {
    FlowEvalResolve {
        window_id: window.0.into(), window_kind_id: window.1.into(), node_hash,
        output, fault: None, extension_id: extension.into(), ok: true, original_input: None, input_valid: true,
    }
}

pub fn receive_evaluation(session: &mut FlowEvalSession, mut payload: FlowEvalResolve) -> Result<(), OriginalTurnFailure> {
    let mut caller = caller();
    let mut controller = PreviewEvalReceive::new();
    let result = (|| {
        assert!(caller.turn(TURN, |cx| controller.receive(&mut payload, cx))?);
        for _ in 0..TURN_LIMIT {
            let grant = quoted(controller.demands(&payload, session, TURN.maximum_copy_bytes)?);
            if caller.turn(grant, |cx| controller.step(&payload, session, cx))? { return Ok(()); }
        }
        panic!("original evaluation reply did not settle")
    })();
    controller.begin_close();
    for _ in 0..TURN_LIMIT {
        if controller.terminal_is_empty() { break; }
        let grant = quoted(controller.close_demands(TURN.maximum_copy_bytes).expect("evaluation receiver close demand"));
        caller.turn(grant, |cx| controller.close_step(cx)).expect("evaluation receiver closes");
    }
    assert!(controller.terminal_is_empty());
    retire_payload(payload, &mut caller);
    let stock = caller.stock();
    caller.retire(stock);
    result
}

pub fn evaluation_response(done: bool, phase: &str, units: (u64, u64), output: serde_json::Value) -> DslValue {
    dto(serde_json::json!({
        "done": done, "cancellable": !done, "phase": phase, "unitsDone": units.0, "unitsTotal": units.1, "output": output
    }))
}

pub fn clear_evaluate_fault(session: &mut FlowEvalSession) {
    let mut caller = caller();
    let demand = RetirementDemand { depth: 1, ..Default::default() };
    caller.turn(TURN, |cx| semio_framework_job::close_original_retained_effect_step(cx, demand, |effect| {
        let receipt = session.clear_extension_evaluate_fault(effect)?
            .ok_or_else(|| semio_framework_value::ValueError::invariant("original evaluation fault could not clear"))?;
        Ok::<_, semio_framework_value::ValueError>(semio_framework_value::RetainedCloneStep::Progress(receipt))
    })).expect("original evaluation fault clears");
    let stock = caller.stock();
    caller.retire(stock);
}

pub fn advance_registry_generation(session: &mut FlowEvalSession) {
    let mut caller = caller();
    let demand = RetirementDemand { depth: 1, ..Default::default() };
    let generation = session.flow_extension_generation() + 1;
    caller.turn(TURN, |cx| semio_framework_job::close_original_retained_effect_step(cx, demand, |effect| {
        let (changed, receipt) = session.invalidate_for_flow_extension_registry(generation, effect)?
            .ok_or_else(|| semio_framework_value::ValueError::invariant("original evaluation registry invalidation did not advance"))?;
        assert!(changed);
        Ok::<_, semio_framework_value::ValueError>(semio_framework_value::RetainedCloneStep::Progress(receipt))
    })).expect("moving the registry invalidates the evaluation");
    assert_eq!(session.flow_extension_generation(), generation);
    let stock = caller.stock();
    caller.retire(stock);
}
