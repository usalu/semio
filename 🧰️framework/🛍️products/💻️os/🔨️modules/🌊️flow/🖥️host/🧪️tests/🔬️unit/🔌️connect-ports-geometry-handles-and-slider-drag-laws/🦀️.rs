/// 🔌️ Connect ports, geometry handles and slider drag laws: the declared endpoint admission, fan out and replacement, dirty propagation, the neural cache, live geometry and drawing handles, previews and slider drags.

#[test]
fn connect_ports_admits_only_declared_neutral_endpoints() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/declared-endpoints/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut host = host_with_test_fixture();
        let before = host.host_snapshot.synapses.len();
        let result = host.connect_ports(case["source"].as_str().unwrap(), case["sourcePort"].as_str().unwrap(), case["target"].as_str().unwrap(), case["targetPort"].as_str().unwrap());
        match case["expected"].as_str().unwrap() {
            "accepted" => {
                let id = result.unwrap();
                let edge = host.host_snapshot.synapses.iter().find(|edge| edge.id == id).unwrap();
                let oracle: serde_json::Value = serde_json::from_slice(&serde_json::to_vec(edge).unwrap()).unwrap();
                assert_eq!(oracle["from"], case["source"]);
                assert_eq!(oracle["to"], case["target"]);
                assert_eq!(oracle["fromPort"], case["sourcePort"]);
                assert_eq!(oracle["toPort"], case["targetPort"]);
            },
            "unknownOutput" => { assert!(matches!(result, Err(FlowCoreError::UnknownOutputPort(_)))); assert_eq!(host.host_snapshot.synapses.len(), before); },
            "unknownInput" => { assert!(matches!(result, Err(FlowCoreError::UnknownInputPort(_)))); assert_eq!(host.host_snapshot.synapses.len(), before); },
            _ => panic!("neutral endpoint verdict is supported"),
        }
        host.retire_cold();
    }
}

#[test]
fn connect_ports_allows_fan_out_from_same_output() {
    let mut host = host_with_test_fixture();
    let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 120.0, 120.0).unwrap();
    host.connect_ports("add", "sum", &pass_id, "number").unwrap();
    let fan_out: Vec<_> = host.host_snapshot.synapses.iter().filter(|s| s.from == "add" && s.from_port == "sum").collect();
    assert_eq!(fan_out.len(), 2);
    assert!(fan_out.iter().any(|s| s.to == "preview"));
    assert!(fan_out.iter().any(|s| s.to == pass_id));
    host.retire_cold();
}

#[test]
fn connect_ports_replaces_existing_incoming_on_same_input() {
    let mut host = host_with_test_fixture();
    assert!(host.host_snapshot.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
    let replacement_id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":2.0}"#, -120.0, 0.0).unwrap();
    host.connect_ports(&replacement_id, "number", "add", "a").unwrap();
    let incoming_a: Vec<_> = host.host_snapshot.synapses.iter().filter(|s| s.to == "add" && s.to_port == "a").collect();
    assert_eq!(incoming_a.len(), 1);
    assert_eq!(incoming_a[0].from, replacement_id);
    assert!(!host.host_snapshot.synapses.iter().any(|s| s.from == "slider" && s.to == "add" && s.to_port == "a"));
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn evaluate_runs_after_tree_change() {
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
    host.set_slider_value("slider", 5.0);
    host.evaluate_internal();
    let after_slider = calls.load(Ordering::Relaxed);
    assert!(after_slider > baseline);
    host.disconnect("s1").unwrap();
    host.connect_ports("slider", "number", "add", "b").unwrap();
    host.evaluate_internal();
    assert!(calls.load(Ordering::Relaxed) > after_slider);
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn dirty_propagation_only_dispatches_affected_branch() {
    use std::sync::Arc;
    use std::sync::Mutex as StdMutex;
    // Branch A (default fixture): slider -> add -> preview. Branch B (added here): a second,
    // disconnected slider -> passThrough, sharing no synapse with branch A.
    let calls: Arc<StdMutex<Vec<String>>> = Arc::new(StdMutex::new(Vec::new()));
    let calls_for_bridge = calls.clone();
    let mut host = FlowHost::default();
    let _dispatch_observation=FixtureDispatchObservation::calls(calls_for_bridge);
    let mut host=FixtureRegistryHost::new(host,FixtureOperatorFamily::Math);
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    let slider_b_id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.0}"#, 400.0, 0.0).unwrap();
    let pass_id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough","params":{},"input_ports":[],"preview":false}"#, 600.0, 0.0).unwrap();
    host.connect_ports(&slider_b_id, "number", &pass_id, "number").unwrap();
    host.evaluate_internal();
    calls.lock().unwrap().clear();

    host.set_slider_value("slider", 5.0);
    host.evaluate_internal();

    let dispatched = calls.lock().unwrap().clone();
    assert!(dispatched.iter().any(|kind| kind == "math.add"), "branch A (add) should re-dispatch after its slider changed");
    assert!(!dispatched.iter().any(|kind| kind == "math.passThrough"), "branch B (pass) must stay clean when only branch A changed");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn neural_cache_persists_across_evaluations() {
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
    assert!(baseline > 0, "first evaluation is a cache miss and must dispatch to the bridge");
    host.evaluate_internal();
    assert_eq!(calls.load(Ordering::Relaxed), baseline, "an unchanged tree must be served entirely from the cache");
    host.set_slider_value("slider", 4.0);
    host.evaluate_internal();
    assert_eq!(calls.load(Ordering::Relaxed), baseline + 1, "only the node downstream of the changed slider should re-dispatch");
    host.retire_cold();
}

#[test]
fn collect_live_geometry_handles_includes_input_channels() {
    let mut outputs = HistoryFoldIndex::new();
    outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
    outputs.insert("volume".into(), Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(12.0))));
    let mut inputs = HistoryFoldIndex::new();
    inputs.insert("volume".into(), Dictionary::new().insert("geometry", NeuralValue::Dictionary(Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-box".into()))))));
    let channels = EvalChannels { outputs, inputs };
    let handles = collect_live_geometry_handles_from_channels(&channels);
    assert_eq!(handles, vec![String::from("solid-box")]);
    channels.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn apply_eval_outputs_json_preserves_state_on_global_error() {
    let mut host = host_with_test_bridge();
    let good = host.last_eval_json.clone();
    host.apply_eval_outputs_json(r#"{"error":"missing input: geometry"}"#);
    assert_eq!(host.last_eval_json, good);
    assert!(host.current_channels.as_ref().is_some_and(|channels|!channels.outputs.is_empty()));
    host.retire_cold();
}

fn collect_live_geometry_handles(outputs: &HistoryFoldIndex<String, Dictionary>) -> Vec<String> {
    let mut handles = Vec::new();
    for dict in outputs.values() {
        collect_geometry_handles_from_dictionary(dict, &mut handles);
    }
    handles.sort();
    handles.dedup();
    handles
}

#[test]
fn collect_live_geometry_handles_traverses_nested_dictionaries() {
    let mut outputs = HistoryFoldIndex::new();
    outputs.insert("box".into(), Dictionary::with_schema("geometry").insert("handle", NeuralValue::Atom(Atom::String("solid-1".into()))).insert("kind", NeuralValue::Atom(Atom::String("solid".into()))));
    outputs.insert("nested".into(), Dictionary::new().insert("child", NeuralValue::Dictionary(Dictionary::with_schema("face").insert("handle", NeuralValue::Atom(Atom::String("face-2".into()))))));
    let handles = collect_live_geometry_handles(&outputs);
    assert_eq!(handles, vec![String::from("face-2"), String::from("solid-1")]);
    outputs.retire_cold();
}

#[test]
fn collect_live_drawing_handles_traverses_list_values() {
    let mut outputs = HistoryFoldIndex::new();
    outputs.insert(
        "get".into(),
        Dictionary::new().insert("value", NeuralValue::Dictionary(Dictionary::with_schema("list").insert("0", NeuralValue::Dictionary(Dictionary::with_schema("draw.drawing").insert("handle", NeuralValue::Atom(Atom::String("drawing-2".into()))))))),
    );
    let channels = EvalChannels { outputs, inputs: HistoryFoldIndex::new() };
    assert_eq!(collect_live_drawing_handles_from_channels(&channels), vec![String::from("drawing-2")]);
    channels.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn evaluate_emits_channel_structured_json() {
    let host = host_with_test_bridge();
    let parsed: serde_json::Value = serde_json::from_str(&host.last_eval_json).expect("json");
    let add = parsed.get("add").and_then(|value| value.as_object()).expect("add channels");
    assert!(add.get("in").and_then(|value| value.as_object()).is_some());
    let out = add.get("out").and_then(|value| value.as_object()).expect("add out");
    assert!(out.get("sum").is_some());
    host.retire_cold();
}

#[test]
fn preview_text_formats_geometry_as_tree_summary() {
    let dict = Dictionary::new().insert("geometry", NeuralValue::Atom(Atom::String("solid-3".into())));
    let content = dag_preview_content_from_dict(&dict);
    assert!(matches!(content, DagPreviewContent::Tree { .. }));
    assert_eq!(preview_content_summary(&content), "{1 keys}");
    dict.retire_cold();
}

#[test]
fn preview_scalar_content_from_number_dict() {
    let dict = Dictionary::new().insert("number", NeuralValue::Atom(Atom::Decimal(3.0)));
    assert!(matches!(
        dag_preview_content_from_dict(&dict),
        DagPreviewContent::Scalar { text } if text == "3"
    ));
    dict.retire_cold();
}

#[test]
fn image_input_seed_and_preview_content() {
    let mut host = host_with_test_fixture();
    let png = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
    host.host_snapshot.widgets.push(Widget::InputImage { id: "image".into(), src: png.into() });
    host.rebuild_dag();
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == "image").expect("image node");
    assert!(matches!(node.kind, DagNodeKind::Image { .. }));
    let seeds = host.build_seeds();
    assert_eq!(seeds.get("image").and_then(|d| d.get("image")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("dataUrl")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some(png));
    seeds.retire_cold();
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn slider_drag_does_not_evaluate_until_explicit_evaluate() {
    // 🧵️ A live drag firing many pointer-move ticks used to re-evaluate the whole graph on every
    // one of them (fine for cheap graphs, a repeated multi-second stall for a heavy one, e.g. a
    // brep boolean). Dragging alone must never evaluate now — the off-main-thread ticker (outside
    // `flow`) picks up the changed slider value at its own pace; an explicit `evaluate`
    // (simulated here) still updates the preview once it runs.
    let mut host = host_with_test_bridge();
    host.set_viewport(800, 600, 1.0);
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    assert_eq!(host.preview_text(), "3");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    assert_eq!(host.preview_text(), "3", "a live drag must not synchronously re-evaluate the graph");
    host.evaluate_internal();
    assert_ne!(host.preview_text(), "3", "an explicit evaluate still picks up the dragged value");
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    host.retire_cold();
}

#[test]
fn dag_slider_drag_syncs_fixture_value() {
    let mut host = host_with_test_fixture();
    host.set_viewport(800, 600, 1.0);
    let slider_node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == "slider").expect("slider").clone();
    let DagNodeKind::Slider { .. } = slider_node.kind else {
        panic!("expected slider kind");
    };
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    let value = host
        .host_snapshot
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id.as_str() == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(value > 3.0);
    host.retire_cold();
}
