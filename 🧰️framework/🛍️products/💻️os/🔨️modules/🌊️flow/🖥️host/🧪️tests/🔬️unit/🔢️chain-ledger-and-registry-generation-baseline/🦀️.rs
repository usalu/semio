/// 🔢️ Chain ledger and registry generation baseline: the converging step budget, the session tick state machine, the chain ledger census, the registry generation invalidation and the re-dispatching baselines.

/// ⚖️ LAW: the CHAIN ledger stays live for the whole of an evaluation, including at the hop
/// boundaries where both finer ledgers are structurally empty, and its node census only ever grows.
///
/// 🩸️ This is the law the whole progress defect needed. A window parked on an extension answer is
/// marked `owed`, not `armed`, and `tick_scheduled` is false — so `FlowEvalSession::pending`, the
/// tessellation ledger and the budgeted-eval ledger ALL report nothing, and the status a preview
/// published at that moment said `phase: "idle", inFlight: 0, ratio: 1.0` while the kernel was busy.
/// Measured on 6118 as 54 byte-identical publications across one 23 s evaluation
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md`).
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_chain_ledger_is_live_at_a_hop_boundary_and_its_census_only_grows() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    capture_fixture_baseline(&mut session,&host);
    let settled = session.preview_chain_status();
    assert!(!settled.working && settled.in_flight == 0, "a quiesced session owes nothing");
    assert!((settled.ratio() - 1.0).abs() < 1e-9, "a quiesced session is complete, not at zero");

    host.set_slider_value("slider", 12.0);
    assert!(session.sync(&host), "a changed slider arms the chain");
    let armed = session.preview_chain_status();
    assert!(armed.working && armed.nodes_total > 0, "an armed chain publishes a census to measure against");
    assert!(armed.nodes_done < armed.nodes_total, "an armed chain has not settled every node");

    // ⏳️ THE HOP BOUNDARY: the window is waiting on an extension answer, so nothing is armed and no
    // finer ledger holds a row. Only the latch knows, and the chain ledger is what reads it.
    session.arm_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    session.begin_window_tick("preview-1",FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_extensions_in_flight("preview-1", 1,FIXTURE_SESSION_GRANT).unwrap();
    session.note_window_tick_outcome("preview-1", true,FIXTURE_SESSION_GRANT).unwrap();
    let parked = session.preview_chain_status();
    assert!(!session.window_tick_is_armed("preview-1"), "a parked window is owed, never armed — the state that made every ledger read idle");
    assert_eq!(session.preview_tessellate_status().in_flight, 0, "the tessellation ledger is empty at a hop boundary");
    assert_eq!(session.preview_eval_status().in_flight, 0, "the budgeted-eval ledger is empty at a hop boundary");
    assert!(parked.working, "the chain ledger still reports the work the other two cannot see");
    assert_eq!(parked.in_flight, 1, "one extension answer is outstanding");

    let mut ratios = vec![parked.ratio()];
    while session.tick_cold(&mut host, None) {
        ratios.push(session.preview_chain_status().ratio());
    }
    session.settle_window_extension("preview-1",FIXTURE_SESSION_GRANT).unwrap().0;
    session.note_window_tick_outcome("preview-1", false,FIXTURE_SESSION_GRANT).unwrap();
    ratios.push(session.preview_chain_status().ratio());
    for pair in ratios.windows(2) {
        assert!(pair[1] >= pair[0] - 1e-9, "the census ratio went backwards: {ratios:?}");
    }
    let done = session.preview_chain_status();
    assert!(!done.working && done.in_flight == 0, "a settled chain owes nothing again");
    assert!((done.ratio() - 1.0).abs() < 1e-9, "a settled chain reports complete");
    session.retire_cold();
    host.retire_cold();
}

/// ⚖️ LAW: the flow extension registry GENERATION is a session's invalidation key.
///
/// A node whose operator no plugin has contributed yet does not merely stall — its miss is cached in
/// the session's neural cache, in its incremental baseline and in its published
/// `eval_json`/`status_json`, and the chain that hit it stops. Installing the contribution afterwards
/// changes a process-wide registry and publishes nothing, so a session has to be TOLD its results
/// predate the registry it can now address. Keying that on the generation is what stops a re-push of
/// an unchanged closure from restarting a settled evaluation
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🧪️ Built on a bare `FlowHost::default()` and a seeded cache rather than on
/// `host_with_two_node_chain`, whose `test_kind_infos_json` helper currently trips the neural
/// engine's `final Dictionary ownership` gate on its own (every session law in this file is red for
/// that reason) — the claim here is about the SESSION, and it is stated without inheriting an
/// unrelated red.
#[test]
fn flow_eval_session_invalidates_only_when_the_flow_extension_registry_generation_moves() {
    let host = FlowHost::default();
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    let generation = session.flow_extension_generation();
    assert_eq!(generation, semio_framework_os_flow_operators::flow_extension_registry_generation(), "a fresh session is current with the registry it will evaluate against");
    let cached = Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(42.0)));
    session.seed_node_cache(17, cached);
    assert!(session.sync(&host), "the default demo graph has pending nodes, so a chain is armed");
    assert!(session.pending());

    assert!(!session.invalidate_for_flow_extension_registry(generation,FIXTURE_SESSION_GRANT).unwrap().0, "an unmoved generation invalidates nothing");
    assert!(session.pending(), "and therefore releases nothing");
    assert!(session.neural_cache().contains(17));

    assert!(session.invalidate_for_flow_extension_registry(generation + 1,FIXTURE_SESSION_GRANT).unwrap().0, "a moved generation invalidates");
    assert_eq!(session.flow_extension_generation(), generation + 1);
    assert_eq!(session.status_json(), "{}", "the per-node status computed against the old registry is released");
    assert!(session.eval_json().is_empty(), "the published evaluation is released");
    assert!(!session.pending(), "a released session admits a new chain rather than waiting on the one that gave up");
    assert!(!session.neural_cache().contains(17), "every node output computed against the old registry is evicted");
    assert!(!session.invalidate_for_flow_extension_registry(generation + 1,FIXTURE_SESSION_GRANT).unwrap().0, "the same generation twice invalidates once");

    session.begin_close();
    for _ in 0..1_000_000 {
        if session.terminal_is_empty() {
            break;
        }
        let _ = session.close_step(FIXTURE_SESSION_GRANT);
    }
    assert!(session.terminal_is_empty(), "an invalidated session still reaches terminal-empty through its own close ladder");
    host.retire_cold();
}

//#region 🔢️RegistryGenerationBaseline
/// 🧵️ Rebuilds the ephemeral host a durable driver would hand a session's baseline to — the same
/// three lines `flow_host_with_session` runs in production, over the test bridge.
#[cfg(not(target_arch = "wasm32"))]
fn replay_host_of(host: &FlowHost) -> FixtureRegistryHost {
    let mut replay = FlowHost::default();
    let mut replay=FixtureRegistryHost::new(replay,FixtureOperatorFamily::Math);
    replay.set_neuron_kind_infos_json(&test_kind_infos_json());
    replay.replace_host_snapshot(host.host_snapshot.clone());
    replay
}

/// ⚖️ LAW: a registry-generation invalidation clears the session's incremental BASELINE, so the
/// next ephemeral host re-dispatches the very same tree instead of skipping it.
///
/// `invalidate_for_flow_extension_registry` releases `eval_json`/`status_json` and sweeps the
/// neural cache — but the incremental baseline is the third place a pre-contribution miss is
/// retained, and it is the one that decides whether `evaluate_step` runs at all. A baseline that
/// outlived the invalidation would let `compute_dirty_set` answer "nothing changed" for a tree
/// that never did change, and the surface would keep its `unknown kind` fault forever with no user
/// action able to clear it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️audit-unknown-kind-2026-09-12.md` §3).
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn an_invalidated_session_hands_an_ephemeral_host_a_re_dispatching_baseline() {
    let host = host_with_test_bridge();
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    capture_fixture_baseline(&mut session,&host);
    let mut settled = replay_host_of(&host);
    session.install_baseline_into(&mut settled,FIXTURE_SESSION_GRANT).unwrap();
    assert!(settled.pending_eval_widget_ids().is_empty(), "control: an unchanged tree under an unchanged registry owes no dispatch");
    let generation = session.flow_extension_generation();
    assert!(session.invalidate_for_flow_extension_registry(generation + 1,FIXTURE_SESSION_GRANT).unwrap().0, "a moved generation invalidates");
    let mut rearmed = replay_host_of(&host);
    session.install_baseline_into(&mut rearmed,FIXTURE_SESSION_GRANT).unwrap();
    assert_eq!(rearmed.eval_baseline_registry_generation(), generation + 1, "the baseline carries the generation it was computed against");
    assert!(!rearmed.pending_eval_widget_ids().is_empty(), "after a registry-generation invalidation the SAME tree must be re-dispatched");
    session.begin_close();
    for _ in 0..1_000_000 {
        if session.terminal_is_empty() { break; }
        let _ = session.close_step(FIXTURE_SESSION_GRANT);
    }
    assert!(session.terminal_is_empty());
    settled.retire_cold();
    rearmed.retire_cold();
    host.retire_cold();
}

/// 🧵️ One ephemeral host of the shared fixture, with its OWN neural cache and a bridge that
/// counts every dispatch — the shape `flow_host_with_session` rebuilds per tick.
#[cfg(not(target_arch = "wasm32"))]
fn counting_replay_host_of(host: &FlowHost, dispatches: &std::sync::Arc<std::sync::atomic::AtomicUsize>) -> FixtureRegistryHost {
    let counter = dispatches.clone();
    let mut replay = FlowHost::default();
    let mut replay=FixtureRegistryHost::new(replay,FixtureOperatorFamily::Math);
    replay.observation=Some(FixtureDispatchObservation::counter(counter));
    replay.set_neuron_kind_infos_json(&test_kind_infos_json());
    replay.replace_host_snapshot(host.host_snapshot.clone());
    replay
}

/// ⚖️ LAW: a baseline stamped with a SUPERSEDED flow extension registry replacement re-dispatches
/// the whole tree; only a baseline stamped with the installed one may be skipped.
///
/// An unchanged tree is not an unchanged evaluation. `compute_dirty_set` diffs the TREE, and a
/// contributed operator arriving in a registry replacement changes no tree at all — the operator
/// table it is dispatched through is process-wide state swapped out underneath a host that has
/// already settled. Refusing only the skip is not enough either: a refused skip that still diffs
/// against the superseded snapshot computes an EMPTY dirty set and dispatches nothing, which is
/// the same stale answer by a longer road. The baseline must be dropped whole
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🧪️ Both halves run on an ephemeral host with its OWN neural cache, exactly as
/// `flow_host_with_session` rebuilds one per tick: the retained cache is the session's to sweep
/// (`invalidate_for_flow_extension_registry`), and a cache that still answers would mask the very
/// dispatch this law counts.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn a_superseded_registry_generation_re_dispatches_an_unchanged_tree() {
    let _serialized = semio_framework_os_flow_operators::registry::lock_flow_extension_registry_for_test();
    semio_framework_os_flow_operators::registry::drain_flow_extension_registry_retirements();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📔️registry/🧪️tests/🧫️fixtures/🔣️manifest-admission.json")).unwrap();
    let bump = &fixture["generationBump"];
    let source = host_with_test_bridge();
    let generation = source.eval_baseline_registry_generation();
    assert_eq!(generation, semio_framework_os_flow_operators::flow_extension_registry_generation(), "a settled host is stamped with the registry it dispatched through");
    let dispatches = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut settled = counting_replay_host_of(&source, &dispatches);
    let (snapshot, channels) = source.eval_baseline();
    settled.install_eval_baseline(snapshot, channels, generation, FIXTURE_SESSION_GRANT).map_err(|(error, _, _)| error).unwrap();
    settled.evaluate_internal();
    assert_eq!(dispatches.load(std::sync::atomic::Ordering::Relaxed), 0, "control: a current baseline over an unchanged tree dispatches nothing");
    assert!(settled.pending_eval_widget_ids().is_empty(), "control: and owes no further work");

    semio_framework_os_flow_operators::install_flow_extension_manifest(bump["pluginId"].as_str().unwrap(), bump["manifestJson"].as_str().unwrap()).expect("the fixture manifest installs");
    assert_ne!(semio_framework_os_flow_operators::flow_extension_registry_generation(), generation, "installing a manifest replaces the registry");
    let mut rearmed = counting_replay_host_of(&source, &dispatches);
    let (snapshot, channels) = source.eval_baseline();
    rearmed.install_eval_baseline(snapshot, channels, generation, FIXTURE_SESSION_GRANT).map_err(|(error, _, _)| error).unwrap();
    assert!(!rearmed.pending_eval_widget_ids().is_empty(), "a superseded baseline owes the whole tree again");
    rearmed.evaluate_internal();
    assert!(dispatches.load(std::sync::atomic::Ordering::Relaxed) > 0, "a superseded registry generation must re-dispatch the unchanged tree");
    assert_eq!(rearmed.preview_text(), source.preview_text(), "re-dispatching an unchanged tree reaches the same answer");

    semio_framework_os_flow_operators::uninstall_flow_extension(bump["extensionId"].as_str().unwrap()).expect("the law leaves the process-wide registry as it found it");
    for _ in 0..100_000 {
        if matches!(semio_framework_os_flow_operators::test_flow_registry_retirement_step(1,64),Ok(neural::RetainedCloneStep::Complete(_))) { break; }
    }
    settled.retire_cold();
    rearmed.retire_cold();
    source.retire_cold();
}
//#endregion 🔢️RegistryGenerationBaseline
