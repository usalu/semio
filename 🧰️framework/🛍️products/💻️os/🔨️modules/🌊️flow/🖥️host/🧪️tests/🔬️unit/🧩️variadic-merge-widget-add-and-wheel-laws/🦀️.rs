/// 🧩️ Variadic merge, widget add and wheel laws: the dictionary merge bridge, the variadic merge, widget display meta, the slider and note widget adds and the wheel zoom and plan.

fn test_dictionary_merge_bridge(kind: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
    if kind == "core.number" {
        let value = input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
        return Ok(Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(value))));
    }
    if kind != "dictionary.merge" {
        return Err(EvalError::UnknownKind(kind.into()));
    }
    let items = input.get("items").and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput("items".into()))?;
    let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    if indices.len() < 2 {
        return Err(EvalError::MissingInput("items".into()));
    }
    let mut merged = Dictionary::with_schema("dictionary");
    for index in indices {
        let slot = items.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
        let next = merged.merge(slot);
        std::mem::replace(&mut merged, next).retire_cold();
    }
    Ok(channel_output("dictionary", merged))
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn variadic_merge_evaluates_port_routed_inputs() {
    let mut host = FlowHost::from_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "a".into(), label: "A".into(), value: 1.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
            Widget::InputSlider { id: "b".into(), label: "B".into(), value: 2.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
            Widget::Neuron { id: "merge".into(), neuron_kind: "dictionary.merge".into(), params: Dictionary::new(), input_ports: vec!["0".into(), "1".into()], output_ports: vec![], preview: true },
            Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: semio_framework_value_ordered::ordered::OrderedSet::new() },
        ],
        synapses: vec![
            SynapseSpec { id: "s1".into(), from: "a".into(), to: "merge".into(), from_port: "number".into(), to_port: "0".into() },
            SynapseSpec { id: "s2".into(), from: "b".into(), to: "merge".into(), from_port: "number".into(), to_port: "1".into() },
            SynapseSpec { id: "s3".into(), from: "merge".into(), to: "preview".into(), from_port: "dictionary".into(), to_port: String::new() },
        ],
        layout: semio_framework_value_ordered::ordered::OrderedMap::new(),
    });
    let mut host=FixtureRegistryHost::new(host,FixtureOperatorFamily::Merge);
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "dictionary.merge".into(),
        extension: "dictionary".into(),
        name: "Merge".into(),
        abbreviation: "Merge".into(),
        icon: "emoji:🔀️".into(),
        summary: "Merge".into(),
        inputs: vec![],
        outputs: vec![InputSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
        variadic_input: Some(neural::VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
        ..Default::default()
    }]));
    host.previous_snapshot = None;
    if let Some(channels)=host.current_channels.take().and_then(std::sync::Arc::into_inner){neural::ColdOwner::new(channels);}
    host.evaluate_internal();
    let preview = host
        .host_snapshot
        .widgets
        .iter()
        .find_map(|widget| match widget {
            Widget::OutputPreview { preview, .. } => Some(preview),
            _ => None,
        })
        .expect("preview");
    assert_eq!(preview.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(2.0));
    host.retire_cold();
}

#[test]
fn widget_to_dag_node_carries_display_meta() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 0.0, 0.0).unwrap();
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == id).expect("node");
    assert_eq!(node.name, "Add");
    assert_eq!(node.abbreviation, "Add");
    assert_eq!(node.icon, "emoji:➕️");
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_explicit_range() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":10.2,"min":10.2,"max":15.0,"step":0.1}"#, 0.0, 0.0).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 10.2).abs() < 1e-6);
    assert!((min - 10.2).abs() < 1e-6);
    assert!((max - 15.0).abs() < 1e-6);
    assert!((step - 0.1).abs() < 1e-6);
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
        panic!("expected slider node");
    };
    assert!((dag_min - 10.2).abs() < 1e-6);
    assert!((dag_max - 15.0).abs() < 1e-6);
    assert!((dag_step - 0.1).abs() < 1e-6);
    assert!((dag_value - 10.2).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn add_note_widget_with_text() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"some text"}"#, 0.0, 0.0).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputNote { text, .. } = widget else {
        panic!("expected note widget");
    };
    assert_eq!(text, "some text");
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Note { text: dag_text, .. } = &node.kind else {
        panic!("expected note node");
    };
    assert_eq!(dag_text, "some text");
    assert!(node.width >= 40.0);
    assert_eq!(node.height, semio_framework_os_infinite_directed_presentation::DAG_CHANNEL_ROW_HEIGHT);
    host.retire_cold();
}

#[test]
fn begin_note_edit_groups_undo_into_single_gesture() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
    let origin_x = node.x - node.width * 0.5 + 4.0;
    host.begin_note_edit(&id, origin_x + 40.0, node.y);
    host.note_insert_text("!");
    host.note_commit_edit();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputNote { text, .. } = widget else {
        panic!("expected note widget");
    };
    assert_eq!(text, "hi!");
    assert!(host.undo());
    let Widget::InputNote { text: restored, .. } = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget") else {
        panic!("expected note widget");
    };
    assert_eq!(restored, "hi");
    host.retire_cold();
}

#[test]
fn wheel_screen_zoom_gesture_changes_zoom() {
    let mut host = host_with_test_fixture();
    let z0 = host.host_snapshot.camera.zoom;
    host.wheel_screen(400.0, 300.0, 0.0, -10.0, true);
    assert_ne!(host.host_snapshot.camera.zoom, z0);
    host.retire_cold();
}

#[test]
fn wheel_plan_matches_direct_and_rejects_stale_revision() {
    let mut direct = host_with_test_fixture();
    let mut planned = host_with_test_fixture();
    direct.set_viewport(800, 600, 1.0);
    planned.set_viewport(800, 600, 1.0);
    direct.wheel_screen(320.0, 240.0, 0.0, -10.0, true);
    let plan = planned.plan_wheel(320.0, 240.0, 0.0, -10.0, true);
    assert!(planned.commit_wheel(plan));
    assert_eq!(direct.host_snapshot.camera, planned.host_snapshot.camera);

    let stale = planned.plan_wheel(320.0, 240.0, 0.0, -10.0, true);
    planned.pointer_down_screen(10.0, 10.0, 0, false, false, false, true);
    let replacement = planned.host_snapshot.camera.clone();
    assert!(!planned.commit_wheel(stale));
    assert_eq!(planned.host_snapshot.camera, replacement);
    planned.retire_cold();
    direct.retire_cold();
}

#[test]
fn set_note_text_keeps_uniform_component_width() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 0.0).unwrap();
    let short_w = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node").width;
    host.set_note_text(&id, "a much longer note string");
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Note { text, .. } = &node.kind else {
        panic!("expected note node");
    };
    assert_eq!(text, "a much longer note string");
    assert_eq!(node.width, short_w);
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_single_value_uses_sensible_range() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":5.0}"#, 0.0, 0.0).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 5.0).abs() < 1e-6);
    assert!((min - 0.0).abs() < 1e-6);
    assert!((max - 10.0).abs() < 1e-6);
    assert!((step - 1.0).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_decimal_value_uses_matching_step() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.3}"#, 0.0, 0.0).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 1.3).abs() < 1e-6);
    assert!((min - 0.0).abs() < 1e-6);
    assert!((max - 10.0).abs() < 1e-6);
    assert!((step - 0.1).abs() < 1e-6);
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == id).expect("node");
    let DagNodeKind::Slider { min: dag_min, max: dag_max, step: dag_step, value: dag_value, .. } = &node.kind else {
        panic!("expected slider node");
    };
    assert!((dag_min - 0.0).abs() < 1e-6);
    assert!((dag_max - 10.0).abs() < 1e-6);
    assert!((dag_step - 0.1).abs() < 1e-6);
    assert!((dag_value - 1.3).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn add_slider_widget_with_two_decimal_places_uses_finer_step() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":1.25}"#, 0.0, 0.0).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { step, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((step - 0.01).abs() < 1e-6);
    host.retire_cold();
}

#[test]
fn set_slider_value_expands_bounds_when_out_of_range() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":3.0,"min":0.0,"max":10.0,"step":1.0}"#, 0.0, 0.0).unwrap();
    host.set_slider_value(&id, 12.0);
    let widget = host.host_snapshot.widgets.iter().find(|w| widget_id_for(w) == id).expect("widget");
    let Widget::InputSlider { value, min, max, .. } = widget else {
        panic!("expected slider widget");
    };
    assert!((value - 12.0).abs() < 1e-6);
    assert!((min - 0.0).abs() < 1e-6);
    assert!((max - 20.0).abs() < 1e-6);
    host.retire_cold();
}
