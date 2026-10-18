/// 🔁️ Extension bridge and chain census laws: the extension bridge fixtures, the sibling extension hosts, the node census, the coalesced wave, the inline continuation admission and the wall deadline.

/// 🔌️ A bridge whose `plugin.*` kinds live in a PLUGIN, not in this process — the exact shape a
/// contributed brep operator has from the guest's side.
fn test_extension_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
    if let Some(operator_id) = kind.strip_prefix("plugin.") {
        return Err(EvalError::PendingExtension { extension_id: "geometry".into(), operator_id: operator_id.into(), node_hash: neural::node_hash(kind, input) });
    }
    test_math_bridge(kind, input)
}

/// 🔌️ The two-operator catalogue plus the two CONTRIBUTED kinds a wave law needs. Declared here
/// rather than folded into `test_kind_infos_json` so every other host law keeps indexing exactly the
/// catalogue it was written against.
fn test_extension_kind_infos_json() -> String {
    kind_infos_json(vec![
        NeuronKindInfo {
            id: "math.add".into(),
            extension: "math".into(),
            name: "Add".into(),
            abbreviation: "Add".into(),
            icon: "emoji:➕️".into(),
            summary: "Sums two numbers".into(),
            inputs: vec![InputSpec::number("a", NUMBER_OPS), InputSpec::number_default("b", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("S", "Sum", "sum", "Sum")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "plugin.left".into(),
            extension: "geometry".into(),
            name: "Left".into(),
            abbreviation: "Left".into(),
            icon: "emoji:⬅️".into(),
            summary: "Contributed left branch".into(),
            inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("O", "Out", "out", "Number")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "plugin.right".into(),
            extension: "geometry".into(),
            name: "Right".into(),
            abbreviation: "Right".into(),
            icon: "emoji:➡️".into(),
            summary: "Contributed right branch".into(),
            inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("O", "Out", "out", "Number")],
            ..Default::default()
        },
    ])
}

/// 🌊️ The default fixture plus two INDEPENDENT contributed nodes hanging off `add` — the flow-host
/// shape of one topological wave.
#[cfg(not(target_arch = "wasm32"))]
fn host_with_two_extension_siblings() -> FixtureRegistryHost {
    let mut host = FlowHost::default();
    let mut host=FixtureRegistryHost::new(host,FixtureOperatorFamily::Extension);
    host.set_neuron_kind_infos_json(&test_extension_kind_infos_json());
    let left = host.add_widget(r#"{"kind":"neuron","id":"left","neuronKind":"plugin.left","params":{},"input_ports":[],"preview":false}"#, 240.0, 0.0).unwrap();
    let right = host.add_widget(r#"{"kind":"neuron","id":"right","neuronKind":"plugin.right","params":{},"input_ports":[],"preview":false}"#, 240.0, 80.0).unwrap();
    host.connect_ports("add", "sum", &left, "number").unwrap();
    host.connect_ports("add", "sum", &right, "number").unwrap();
    host
}

/// 📊️ The census entries of one status object, as `(widget id, status tag)` pairs.
fn census_entries(status_json: &str) -> Vec<(String, String)> {
    let value = semio_framework_pack_json::parse(status_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("census json");
    let object = value.as_object().expect("census object").clone();
    let mut entries: Vec<(String, String)> = object.iter().map(|(id, entry)| (id.to_string(), entry.get("status").and_then(semio_framework_pack_json::Value::as_str).unwrap_or_default().to_string())).collect();
    entries.sort();
    entries
}

/// 📈 How many nodes of a census have SETTLED — the numerator [`FlowEvalSession::preview_chain_status`]
/// derives, stated here so a law can watch it move.
fn census_nodes_done(status_json: &str) -> usize {
    census_entries(status_json).into_iter().filter(|(_, status)| !matches!(status.as_str(), "queued" | "computing" | "stale")).count()
}

/// ⚖️ LAW: the node census ADVANCES as a chain walks — a node this evaluation has already
/// recomputed is `ok`, never `stale`.
///
/// 🩸️ `dirty` is measured against the chain's FROZEN baseline, which only advances when the chain
/// COMPLETES, so every node an evaluation touches stays dirty until the very last hop. Calling those
/// nodes `stale` made `nodes_done` the count of nodes the evaluation never touched — constant from
/// the first hop to the last. The published ratio was monotone and completely flat, then jumped to
/// `1.0`: a progress bar that only ever reads "nothing yet" and then "done"
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_node_census_advances_as_a_chain_walks_and_never_calls_a_recomputed_node_stale() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    host.set_slider_value("slider", 12.0);
    let armed = build_flow_status_json(&host, &host.pending_eval_widget_ids());
    assert_eq!(census_entries(&armed), [("add".to_string(), "computing".to_string()), ("pass".to_string(), "queued".to_string()), ("preview".to_string(), "ok".to_string()), ("slider".to_string(), "ok".to_string())]);

    let after_first = host.evaluate_cold_step(EvalStepBudget::dispatches(1),&|_|true).unwrap();
    let hop1 = build_flow_status_json(&host, &after_first);
    assert_eq!(census_entries(&hop1), [("add".to_string(), "ok".to_string()), ("pass".to_string(), "computing".to_string()), ("preview".to_string(), "ok".to_string()), ("slider".to_string(), "ok".to_string())], "the node this hop recomputed has SETTLED, whatever the frozen baseline still calls dirty");

    let after_second = host.evaluate_cold_step(EvalStepBudget::dispatches(1),&|_|true).unwrap();
    assert!(after_second.is_empty(), "two budget-one hops converge this chain");
    let hop2 = build_flow_status_json(&host, &after_second);
    let census = [census_nodes_done(&armed), census_nodes_done(&hop1), census_nodes_done(&hop2)];
    assert!(census[0] < census[1] && census[1] < census[2], "the census must GROW every hop, not merely refuse to shrink: {census:?}");
    host.retire_cold();
}

/// ⚖️ LAW: a coalesced tick parks a whole wave AND paints every member of it `computing` — the
/// census names what is outstanding at a plugin right now, never just the head of the remaining list.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn a_coalesced_tick_parks_a_whole_wave_and_paints_every_member_computing() {
    let mut host = host_with_two_extension_siblings();
    let remaining = host.evaluate_cold_step(flow_eval_tick_budget(None),&|_|true).unwrap();
    let parked: Vec<&str> = host.pending_extension_evals.iter().map(|pending| pending.neuron_id.as_str()).collect();
    assert_eq!(parked, ["left", "right"], "both ready contributed nodes park on the SAME hop");
    let census = census_entries(&build_flow_status_json(&host, &remaining));
    assert_eq!(census, [("add".to_string(), "ok".to_string()), ("left".to_string(), "computing".to_string()), ("preview".to_string(), "ok".to_string()), ("right".to_string(), "computing".to_string()), ("slider".to_string(), "ok".to_string())]);
    host.retire_cold();
}

/// ⚖️ LAW: cancellation is WAVE-SIZED — one gesture retires a whole coalesced wave, and every answer
/// that was already crossing when it landed arms nothing.
///
/// 🚨️ The cancel affordance must not get weaker as a hop carries more work. A wave of N parked
/// answers leaves N settles still to come; if any of them could re-arm the chain, a user's cancel
/// would be silently undone by the network (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn cancelling_a_coalesced_wave_retires_every_parked_answer_and_late_settles_arm_nothing() {
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    session.arm_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    session.begin_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_extensions_in_flight("preview-1", 3,FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_tick_outcome("preview-1", true,FIXTURE_SESSION_GRANT).unwrap();
    assert_eq!(session.window_extensions_in_flight("preview-1"), 3, "a three-wide wave is three answers outstanding");
    assert!(session.preview_chain_status().working, "a parked wave is live work");

    let retired = session.cancel_preview_evaluation("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    drain_fixture_cancellation(&mut session);
    assert_eq!(session.window_extensions_in_flight("preview-1"), 0, "one gesture retires the WHOLE wave, not one answer of it");
    assert!(session.preview_cancelled(), "the surface may publish the cancelled banner");
    assert!(!session.preview_chain_status().working, "a cancelled chain owes nothing");

    let rearms = (0..3).filter(|_| session.settle_window_extension("preview-1",FIXTURE_SESSION_GRANT).unwrap().0).count();
    assert_eq!(rearms, 0, "every answer still crossing when the cancel landed arms nothing");
    assert!(!session.window_tick_is_armed("preview-1"), "a cancelled window stays unarmed");
    eprintln!("wave cancel: retiredTessellations={retired} rearms={rearms}");
    session.retire_cold();
}

/// ⚖️ LAW: a fold may only take over a hop the SCHEDULER would have dispatched — never invent one,
/// never resume a cancelled chain, and never start a walk a turn has no wall left for.
///
/// 🔁️ The inline continuation is the removal of the `flowEvalResolve` → `flowEvalTick` pair each
/// dependency level used to cost (`📓️flow-tick-coalescing-2026-09-14.md` §7 item 4). Its whole
/// safety argument is that its admission question IS the run job's own scheduling question
/// (`window_tick_owed`), so the chain's SHAPE cannot change — only who runs the hop.
#[test]
fn an_inline_continuation_is_admitted_exactly_where_the_run_job_would_have_dispatched_a_hop() {
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    assert!(session.inline_continuation_admitted("preview-1", None, None), "a window that never ticked owes its first hop, so a fold may run it");

    session.arm_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "a hop is already armed — a fold must not run a second one");

    session.begin_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_extensions_in_flight("preview-1", 2,FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_tick_outcome("preview-1", true,FIXTURE_SESSION_GRANT).unwrap();
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "the FIRST answer of a two-wide wave leaves a sibling in flight — a fold must not run the next wave yet");

    assert!(!session.settle_window_extension("preview-1",FIXTURE_SESSION_GRANT).unwrap().0, "the first settle of a fan-out arms nothing");
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "one answer is still outstanding");
    assert!(!session.settle_window_extension("preview-1",FIXTURE_SESSION_GRANT).unwrap().0, "no source had asked for a re-arm while the wave was crossing");
    assert!(session.inline_continuation_admitted("preview-1", None, None), "the LAST answer of a wave holds a window that owes a hop nothing is chasing — exactly the run job's Dispatch branch");

    let armed = session.arm_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    assert!(armed, "the continuation CLAIMS the hop it takes over");
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "a claimed hop is not offered twice");

    session.begin_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_tick_outcome("preview-1", false,FIXTURE_SESSION_GRANT).unwrap();
    assert!(!session.inline_continuation_admitted("preview-1", None, None), "a finished window owes nothing to continue");
    session.retire_cold();
}

/// ⚖️ LAW: a CANCELLED chain is never continued inline.
///
/// 🛑 `begin_window_tick` retires the `cancelled` banner, because work resuming is the one thing that
/// may. An inline continuation on a cancelled session would therefore not merely compute one wave too
/// many — it would UN-CANCEL the run the user stopped, from inside an answer that was already
/// crossing when they stopped it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn a_cancelled_chain_is_never_continued_inline_by_an_answer_that_was_already_crossing() {
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    session.arm_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    session.begin_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_extensions_in_flight("preview-1", 2,FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_tick_outcome("preview-1", true,FIXTURE_SESSION_GRANT).unwrap();

    session.cancel_preview_evaluation("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    drain_fixture_cancellation(&mut session);
    assert!(session.preview_cancelled(), "the gesture landed");
    for _ in 0..2 {
        session.settle_window_extension("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
        assert!(!session.inline_continuation_admitted("preview-1", None, None), "an answer that was already crossing may not continue a cancelled chain");
    }
    assert!(!session.window_tick_is_armed("preview-1"), "and it armed nothing either");
    session.retire_cold();
}

/// ⚖️ LAW: the wall a continuation runs under belongs to the TURN, not to the walk — a turn with less
/// than [`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`] left parks, and a continuation that is admitted
/// inherits the turn's own deadline instead of opening a fresh allowance.
///
/// 🚨️ Without both halves an inline chain stretches the interactive hold by one whole
/// [`FLOW_EVAL_TICK_ELAPSED_CEILING_US`] per wave — and `EvalStepBudget` always dispatches at least
/// one node before a deadline can stop it, so an admission with a sliver left overruns by a whole
/// operator rather than yielding.
#[test]
fn every_dag_walk_of_one_guest_turn_shares_one_wall_deadline_and_a_spent_turn_parks() {
    let turn_started_us = 1_000_000_u64;
    assert!(flow_eval_inline_continuation_fits(Some(turn_started_us), Some(turn_started_us)), "a turn that has spent nothing admits a continuation");
    let last_admitted_us = turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US - FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US;
    assert!(flow_eval_inline_continuation_fits(Some(turn_started_us), Some(last_admitted_us)), "a turn with exactly the reserve left still admits one");
    assert!(!flow_eval_inline_continuation_fits(Some(turn_started_us), Some(last_admitted_us + 1)), "one microsecond past the reserve parks");
    assert!(!flow_eval_inline_continuation_fits(Some(turn_started_us), Some(turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US * 4)), "a turn that already overran parks");
    assert!(flow_eval_inline_continuation_fits(None, Some(last_admitted_us)), "a walk that OPENS its own turn is not measured against anyone else's");
    assert!(flow_eval_inline_continuation_fits(Some(turn_started_us), None), "an uninstrumented process falls back to the node budget, exactly as flow_eval_tick_budget does");

    assert_eq!(flow_eval_tick_budget(Some(turn_started_us)).deadline_us(), Some(turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US), "an inline walk inherits the TURN's deadline");
    let own_turn = flow_eval_tick_budget(None).deadline_us();
    assert_ne!(own_turn, Some(turn_started_us + FLOW_EVAL_TICK_ELAPSED_CEILING_US), "a walk that opens its own turn gets its own allowance");
    assert!(FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US > 0 && FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US < FLOW_EVAL_TICK_ELAPSED_CEILING_US, "the reserve is a slice of the allowance, not all of it and not none of it");
}

/// ⚖️ LAW: a chain that has settled its own census says so, and a chain that has published no census
/// yet does NOT — the two windows in which the run view legitimately knows more, and less, than the
/// evaluation it is pacing.
///
/// 🩸️ `computing` and the `toolRunAbort` affordance were ORed straight off `ToolRunView`, which is a
/// host artifact that reaches a surface on a render and therefore lags its own job. Once the inline
/// continuation let a chain finish inside an extension answer's turn, the guest went quiet with the
/// run's terminal state still unrendered — and the preview published `phase: "idle", ratio: 1.0,
/// nodesDone 7/7` beside `computing: true, cancellable: true`, for good: a spinner that outlived its
/// work and a Cancel for an evaluation with nothing left to cancel. Measured on 6024 in two picks
/// (`📓️flow-inline-continuation-2026-09-14.md`).
#[test]
fn a_settled_chain_census_outranks_a_lagging_run_view_and_an_empty_one_does_not() {
    let working = PreviewChainStatus { nodes_done: 3, nodes_total: 7, in_flight: 1, working: true };
    assert!(!working.settled(), "a chain with work in flight has not settled");

    let half = PreviewChainStatus { nodes_done: 3, nodes_total: 7, in_flight: 0, working: false };
    assert!(!half.settled(), "a quiesced chain whose census is incomplete has not settled either");

    let census_free = PreviewChainStatus { nodes_done: 0, nodes_total: 0, in_flight: 0, working: false };
    assert!(!census_free.settled(), "a chain that has published NO census may not silence the run: this is the window a gesture's own start lives in, before its first hop ran");

    let done = PreviewChainStatus { nodes_done: 7, nodes_total: 7, in_flight: 0, working: false };
    assert!(done.settled(), "nothing owed, nothing in flight, every published node accounted for");
    assert_eq!(done.units(), (7, 7), "and its published fraction is complete");

    let overshoot = PreviewChainStatus { nodes_done: 9, nodes_total: 7, in_flight: 0, working: false };
    assert!(overshoot.settled(), "a census that counted more than it declared is settled, never unsettled");
}
