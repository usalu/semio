/// 🌊️ Evaluation and baseline laws: the fixture to native DAG kinds, evaluation and unchanged tree skipping, pending ids, computing chrome, baseline outputs and the retained session baseline and cache.

#[test]
fn default_fixture_maps_widgets_to_native_dag_kinds() {
    let host = host_with_test_fixture();
    let slider = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == "slider").expect("slider");
    assert!(matches!(slider.kind, DagNodeKind::Slider { .. }));
    assert_eq!(slider.height, slider_widget_height());
    let add = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == "add").expect("add");
    assert!(matches!(add.kind, DagNodeKind::Computation { .. }));
    assert_eq!(slider.width, add.width, "all components should share one width");
    assert_eq!(slider.width, computation_node_width(&slider.name, &[], &[]));
    let preview = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == "preview").expect("preview");
    assert!(matches!(preview.kind, DagNodeKind::Preview { .. }));
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn default_fixture_evaluates_add_preview() {
    let host = host_with_test_bridge();
    assert_eq!(host.preview_text(), "3");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn slider_updates_preview() {
    let mut host = host_with_test_bridge();
    host.set_slider_value("slider", 5.0);
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "5");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn evaluate_skips_unchanged_tree_after_move_widget() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_for_bridge = calls.clone();
    let mut host = FlowHost::default();
    let _dispatch_observation=FixtureDispatchObservation::counter(calls_for_bridge);
    let mut host=FixtureRegistryHost::new(host,FixtureOperatorFamily::Math);
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    host.evaluate_internal();
    let baseline = calls.load(Ordering::Relaxed);
    host.move_widget("slider", -120.0, 20.0).unwrap();
    host.evaluate_internal();
    assert_eq!(calls.load(Ordering::Relaxed), baseline);
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn pending_eval_widget_ids_reports_without_computing() {
    let mut host = host_with_test_bridge();
    let before = host.preview_text();
    host.set_slider_value("slider", 9.0);
    let pending = host.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()), "the widget downstream of the changed slider is pending");
    assert!(!pending.contains(&"slider".to_string()), "the seed slider itself is not a pending neuron");
    assert_eq!(host.preview_text(), before, "a probe must never actually compute anything");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn set_slider_value_marks_downstream_computing_chrome() {
    let mut host = host_with_test_bridge();
    host.set_slider_value("slider", 7.0);
    let pending = host.pending_eval_widget_ids();
    assert!(!pending.is_empty(), "slider change must flag downstream nodes as pending");
    host.refresh_computing_chrome_from_pending();
    let remaining = host.pending_eval_widget_ids();
    assert_eq!(remaining.first().map(String::as_str), pending.first().map(String::as_str));
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn apply_eval_outputs_json_establishes_baseline_for_dirty_probe() {
    let host = host_with_test_bridge();
    let eval_json = host.last_eval_json.clone();
    let mut fresh = FlowHost::default();
    let mut fresh=FixtureRegistryHost::new(fresh,FixtureOperatorFamily::Math);
    fresh.set_neuron_kind_infos_json(&test_kind_infos_json());
    fresh.apply_eval_outputs_json(&eval_json);
    fresh.set_slider_value("slider", 4.0);
    let pending = fresh.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()));
    assert!(!pending.contains(&"slider".to_string()));
    fresh.retire_cold();
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn apply_eval_outputs_json_skips_baseline_when_outputs_stale_for_seeds() {
    let mut host = host_with_test_bridge();
    let stale_eval_json = host.last_eval_json.clone();
    host.set_slider_value("slider", 9.0);
    host.apply_eval_outputs_json(&stale_eval_json);
    let pending = host.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()), "stale channel outputs must not converge the baseline after a seed change");
    assert!(!pending.contains(&"slider".to_string()));
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "9");
    let fresh_eval_json = host.last_eval_json.clone();
    host.apply_eval_outputs_json(&fresh_eval_json);
    assert!(host.pending_eval_widget_ids().is_empty(), "fresh eval for the current seeds must establish a converged baseline");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn flow_eval_session_retains_baseline_across_ephemeral_hosts() {
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    let mut host = host_with_test_bridge();
    capture_fixture_baseline(&mut session,&host);
    host.set_slider_value("slider", 8.0);
    let mut replay = FlowHost::default();
    let mut replay=FixtureRegistryHost::new(replay,FixtureOperatorFamily::Math);
    replay.set_neuron_kind_infos_json(&test_kind_infos_json());
    replay.replace_host_snapshot(host.host_snapshot.clone());
    session.install_baseline_into(&mut replay,FIXTURE_SESSION_GRANT).unwrap();
    let pending = replay.pending_eval_widget_ids();
    assert!(pending.contains(&"add".to_string()));
    assert!(!pending.contains(&"slider".to_string()));
    session.retire_cold();
    replay.retire_cold();
    host.retire_cold();
}

#[test]
fn flow_eval_session_seeds_its_retained_neural_cache() {
    let session = FlowEvalSession::new();
    let expected = Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(42.0)));
    session.seed_node_cache(17, expected.clone());
    let seeded = session.neural_cache().get(17);
    assert_eq!(seeded.as_ref(), Some(&expected));
    seeded.retire_cold();
    expected.retire_cold();
    session.retire_cold();
}

/// 🧵️ Builds a two-computable-node chain (`add` -> `pass`, replacing `add`'s direct link to
/// `preview`) on top of the default fixture, for tests that need more than one node to step
/// through with a budgeted `evaluate_step`.
#[cfg(not(target_arch = "wasm32"))]
fn host_with_two_node_chain() -> (FixtureRegistryHost, String) {
    let mut host = host_with_test_bridge();
    let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 240.0, 0.0).unwrap();
    host.connect_ports("add", "sum", &pass_id, "number").unwrap();
    host.connect_ports(&pass_id, "number", "preview", "").unwrap();
    let stale_link = host.host_snapshot.synapses.iter().find(|s| s.from == "add" && s.to == "preview").map(|s| s.id.clone());
    if let Some(id) = stale_link {
        host.disconnect(&id).unwrap();
    }
    host.evaluate_internal();
    (host, pass_id)
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn evaluate_step_budget_one_converges_over_multiple_calls() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    assert_eq!(host.preview_text(), "3", "chain settles to the same value as the direct add->preview link");
    host.set_slider_value("slider", 6.0);
    assert_eq!(host.preview_text(), "3");
    let remaining_after_tick1 = host.evaluate_cold_step(EvalStepBudget::dispatches(1),&|_|true).unwrap();
    assert_eq!(remaining_after_tick1.first(), Some(&"pass".to_string()), "pass is the next node blocking completion");
    assert_eq!(host.preview_text(), "3", "the chain hasn't reached \"pass\" (and thus \"preview\") yet");
    let remaining_after_tick2 = host.evaluate_cold_step(EvalStepBudget::dispatches(1),&|_|true).unwrap();
    assert!(remaining_after_tick2.is_empty(), "the walk reached the end of the topo order");
    assert_eq!(host.preview_text(), "6", "converged to the dragged value after both ticks");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn flow_eval_session_sync_and_tick_state_machine() {
    let (mut host, _pass_id) = host_with_two_node_chain();
    let mut session = FlowEvalSession::new();
    prepare_fixture_window(&mut session);
    capture_fixture_baseline(&mut session,&host);
    assert!(!session.pending());
    assert!(!session.sync(&host));
    assert!(!session.pending());
    host.set_slider_value("slider", 12.0);
    assert!(session.sync(&host), "a changed slider arms the chain");
    assert!(session.pending());
    assert!(session.status_json().contains("computing"), "the immediate dependent is reported as computing");
    assert!(!session.sync(&host));
    while session.tick_cold(&mut host, None) {}
    assert!(!session.pending());
    assert_eq!(host.preview_text(), "12");
    host.set_slider_value("slider", 20.0);
    assert!(session.sync(&host));
    assert!(session.pending());
    host.set_slider_value("slider", 30.0);
    assert!(!session.sync(&host), "a chain is already scheduled — sync must not arm a redundant second one");
    assert!(session.pending(), "the in-flight chain is still the one that will pick up 30");
    while session.tick_cold(&mut host, None) {}
    assert_eq!(host.preview_text(), "30", "converges on the latest value, not the superseded intermediate one");
    session.retire_cold();
    host.retire_cold();
}
