/// 🧲️ Port insertion, cluster and collapse laws: the edge deletion and alignment, the variadic port slots, insert between, make space, the neuron params, cluster ports and collapse and explode.

#[test]
fn delete_selection_removes_selected_edge_from_host_snapshot() {
    let mut host = host_with_test_fixture();
    let synapse_count_before = host.host_snapshot.synapses.len();
    assert!(synapse_count_before > 0);
    let edge_id = *host.dag.graph.engine.edges.keys().next().expect("edge");
    host.dag.graph.engine.selection.edge_ids.insert(edge_id);
    assert!(host.has_selection());
    host.delete_selection().unwrap();
    assert!(host.host_snapshot.synapses.len() < synapse_count_before);
    assert!(!host.has_selection());
    host.retire_cold();
}

#[test]
fn delete_selection_removes_edge_selected_by_synapse_id_domain() {
    let mut host = host_with_test_fixture();
    let before = host.host_snapshot.synapses.len();
    host.dag.graph.set_selection_domains(&DagSelectionDomains { edges: vec!["s1".into()], ..Default::default() });
    assert!(host.has_selection(), "synapse id s1 must map into engine edge selection");
    host.delete_selection().unwrap();
    assert!(host.host_snapshot.synapses.len() < before);
    assert!(!host.host_snapshot.synapses.iter().any(|synapse| synapse.id == "s1"));
    host.retire_cold();
}

#[test]
fn align_selection_left_aligns_selected_widget_layout() {
    let mut host = host_with_test_fixture();
    host.move_widget("slider", -120.0, 20.0).unwrap();
    host.move_widget("add", 180.0, -40.0).unwrap();
    host.dag.graph.set_selection(&["slider".into(), "add".into()]);
    host.align_selection("alignLeft").unwrap();
    let slider = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == "slider").expect("slider");
    let add = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add");
    let slider_left = slider.x - slider.width * 0.5;
    let add_left = add.x - add.width * 0.5;
    assert!((slider_left - add_left).abs() < 1e-6, "left edges should match after alignLeft");
    assert!(host.host_snapshot.layout.contains_key("slider"));
    assert!(host.host_snapshot.layout.contains_key("add"));
    host.retire_cold();
}

#[test]
fn add_input_port_inserts_variadic_slot() {
    let mut host = host_with_test_fixture();
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
    let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 0.0, 0.0).unwrap();
    host.add_input_port(&merge_id, 1).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == merge_id).expect("merge");
    let Widget::Neuron { input_ports, .. } = widget else { panic!("neuron") };
    assert_eq!(input_ports.len(), 3);
    host.retire_cold();
}

#[test]
fn add_output_port_inserts_variadic_get_slot() {
    let mut host = host_with_test_fixture();
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "list.get".into(),
        extension: "list".into(),
        name: "Get".into(),
        abbreviation: "Get".into(),
        icon: "emoji:📋️".into(),
        summary: "Reads consecutive values by index".into(),
        inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
        outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
        variadic_output: Some(neural::VariadicSpec { slot_key: "value".into(), min: 1, max: None }),
        ..Default::default()
    }]));
    let get_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"list.get"}"#, 0.0, 0.0).unwrap();
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == get_id).expect("get");
    let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
    assert_eq!(labels, vec!["i"]);
    host.add_output_port(&get_id, 1).unwrap();
    let widget = host.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == get_id).expect("get");
    let Widget::Neuron { output_ports, .. } = widget else { panic!("neuron") };
    assert_eq!(output_ports.len(), 2);
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == get_id).expect("get");
    let labels: Vec<&str> = node.outputs().iter().map(|port| port.label.as_str()).collect();
    assert_eq!(labels, vec!["i", "i+1"]);
    host.retire_cold();
}

#[test]
fn add_widget_with_explicit_id() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","id":"custom_slider","value":2.0}"#, 0.0, 0.0).unwrap();
    assert_eq!(id, "custom_slider");
    host.retire_cold();
}

#[test]
fn insert_between_rewires_downstream_and_connects_anchor() {
    let mut host = host_with_test_fixture();
    let mid = host.add_widget(r#"{"kind":"neuron","id":"mid","neuronKind":"math.passThrough"}"#, 120.0, 0.0).unwrap();
    host.insert_between("slider", "number", &mid, "number", "number").unwrap();
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "mid"));
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "mid" && synapse.to == "add"));
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "add" && synapse.to == "preview"));
    assert!(!host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "add"));
    host.retire_cold();
}

#[test]
fn insert_between_allocates_unique_synapses_after_host_reconstruction() {
    let mut host = host_with_test_fixture();
    host.host_snapshot.synapses[0].id = "s101".into();
    host.host_snapshot.synapses[1].id = "s102".into();
    let mid = host.add_widget(r#"{"kind":"neuron","id":"mid","neuronKind":"math.passThrough"}"#, 120.0, 0.0).unwrap();
    host.insert_between("slider", "number", &mid, "number", "number").unwrap();
    let ids: std::collections::BTreeSet<_> = host.host_snapshot.synapses.iter().map(|synapse| &synapse.id).collect();
    assert_eq!(ids.len(), host.host_snapshot.synapses.len());
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.id == "s101" && synapse.from == "mid" && synapse.to == "add"));
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.id == "s103" && synapse.from == "slider" && synapse.to == "mid"));
    host.retire_cold();
}

#[test]
fn insert_between_preserves_existing_mid_inputs() {
    let mut host = host_with_test_fixture();
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 120.0, 0.0).unwrap();
    host.connect_ports("slider", "number", &variable_id, "width").unwrap();
    host.insert_between("slider", "number", &variable_id, "width", "width").unwrap();
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == variable_id && synapse.to_port == "width"));
    assert!(!host.host_snapshot.synapses.iter().any(|synapse| synapse.from == variable_id && synapse.to == variable_id));
    host.retire_cold();
}

#[test]
fn make_space_shifts_widgets_right_of_anchor() {
    let mut host = host_with_test_fixture();
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.host_snapshot.layout.insert("preview".into(), WidgetLayout { x: 400.0, y: 0.0 });
    host.rebuild_dag();
    host.make_space("slider", 100.0, 0.0).unwrap();
    assert!((host.host_snapshot.layout.get("slider").expect("slider").x - 0.0).abs() < 1e-6);
    assert!((host.host_snapshot.layout.get("add").expect("add").x - 300.0).abs() < 1e-6);
    assert!((host.host_snapshot.layout.get("preview").expect("preview").x - 500.0).abs() < 1e-6);
    host.retire_cold();
}

/// ⚖️ LAW (design §20.9): an inserted operator records every declared input's default literal in its `params` — the record
/// is self-describing, so a fold decides an input's existence and literal type without any registry — while an input
/// without a default (a data port) stays absent.
#[test]
fn an_inserted_operator_records_every_declared_default_input() {
    let mut host = host_with_test_fixture();
    let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 0.0, 0.0).unwrap();
    let recorded = |key: &str| {
        host.host_snapshot.widgets.iter().find_map(|widget| match widget {
            Widget::Neuron { id: candidate, params, .. } if *candidate == id => Some(params.get(key).map(|value| value.as_dictionary().and_then(|literal| literal.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()))),
            _ => None,
        })
    };
    assert_eq!(recorded("b"), Some(Some(Some(0.0))), "the defaulted input is recorded with its default literal");
    assert_eq!(recorded("a"), Some(None), "an input without a default stays absent");
    host.retire_cold();
}

/// ⚖️ LAW (design §20.9): a wire into an input shadows the literal its operator records for that input — an operator
/// inserted with its declared defaults and then wired evaluates the wire, never the recorded default.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn an_inserted_operator_wired_into_a_defaulted_input_evaluates_its_wire() {
    let (host, pass_id) = host_with_two_node_chain();
    assert!(host.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, params, .. } if *id == pass_id && params.get("number").is_some())), "the inserted operator recorded its declared default");
    assert_eq!(host.preview_text(), "3", "the wire from `add` shadows the recorded default 0");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn set_neuron_params_merges_into_eval_input() {
    let mut host = host_with_test_bridge();
    let preview_synapse = host.host_snapshot.synapses.iter().find(|synapse| synapse.from == "add" && synapse.to == "preview").map(|synapse| synapse.id.clone()).expect("preview synapse");
    host.disconnect(&preview_synapse).unwrap();
    let id = host.add_widget(r#"{"kind":"neuron","id":"pass","neuronKind":"math.passThrough"}"#, 100.0, 0.0).unwrap();
    host.connect_ports(&id, "number", "preview", "").unwrap();
    host.set_neuron_params(&id, r#"{"number":{"$schema":"number","value":7.5}}"#).unwrap();
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "7.5");
    host.retire_cold();
}

#[test]
fn cluster_ports_from_contract() {
    let inner = Tree {
        neurons: vec![
            Neuron::with_kind("in_a", INPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("a".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
            Neuron::with_kind("out_sum", OUTPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("sum".into()))).insert("operators", NeuralValue::Atom(Atom::String("core.number".into())))),
        ],
        synapses: vec![],
    };
    let widget = Widget::Cluster { id: "cluster".into(), name: "Add cluster".into(), tree: inner, flow: FlowGui::default() };
    let (inputs, outputs, _, _) = widget_io_ports(&widget, &[], &HistoryFoldIndex::new());
    assert_eq!(inputs.len(), 1);
    assert_eq!(outputs.len(), 1);
    assert_eq!(inputs[0].id, "a");
    assert_eq!(outputs[0].id, "sum");
    widget.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn variable_relay_evaluates_through_flow_host() {
    let mut host = host_with_test_bridge();
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 0.0, 0.0).unwrap();
    let slider_id = host.add_widget(r#"{"kind":"inputSlider","label":"Number","value":4.0}"#, -200.0, 0.0).unwrap();
    host.connect_ports(&slider_id, "number", &variable_id, "width").unwrap();
    let eval_json = host.evaluate().expect("evaluate");
    let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
    let width = parsed.get(&variable_id).and_then(|entry| entry.get("out")).and_then(|out| out.get("width")).expect("variable width output");
    assert_eq!(width.get("$schema").and_then(|value| value.as_str()), Some("number"));
    host.retire_cold();
}

#[test]
fn collapse_uses_variable_name_as_cluster_input_port() {
    let mut host = host_with_test_fixture();
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    let variable_id = host.add_widget(r#"{"kind":"variable","name":"width","schema":"number"}"#, 100.0, 0.0).unwrap();
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.host_snapshot.synapses.retain(|synapse| synapse.from != "slider" || synapse.to != "add");
    host.connect_ports("slider", "number", &variable_id, "width").unwrap();
    host.connect_ports(&variable_id, "width", "add", "a").unwrap();
    host.rebuild_dag();
    let cluster_id = host.collapse_selection(&[variable_id.clone(), "add".into()]).unwrap();
    let cluster = host
        .host_snapshot
        .widgets
        .iter()
        .find_map(|widget| match widget {
            Widget::Cluster { id, tree, .. } if id == &cluster_id => Some(tree.clone()),
            _ => None,
        })
        .expect("cluster");
    let (inputs, _) = cluster.contract();
    assert!(inputs.iter().any(|port| port.name == "width"));
    host.explode_cluster(&cluster_id).unwrap();
    assert!(host.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Variable { name, .. } if name == "width")));
    cluster.retire_cold();
    host.retire_cold();
}

#[test]
fn collapse_then_explode_round_trips() {
    let mut host = host_with_test_fixture();
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.rebuild_dag();
    let cluster_id = host.collapse_selection(&["slider".into(), "add".into()]).unwrap();
    assert!(host.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { id, .. } if id == &cluster_id)));
    host.explode_cluster(&cluster_id).unwrap();
    assert!(host.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget).starts_with(&format!("{cluster_id}/"))));
    assert!(!host.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Cluster { .. })));
    host.retire_cold();
}




#[test]
fn compiled_wire_literal_includes_operator_kinds() {
    let host = host_with_test_fixture();
    let text = host.compiled_wire_literal();
    assert!(text.contains("core.number"));
    assert!(text.contains("math.add"));
    host.retire_cold();
}

#[test]
fn apply_generation_values_to_host_snapshot_patches_slider_value() {
    use self::forms_bridge::apply_generation_values_to_host_snapshot;
    let fixture = FlowHostSnapshot::default();
    let slider_id = fixture.widgets.iter().find_map(|widget|match widget {Widget::InputSlider{id,..}=>Some(id.clone()),_=>None}).expect("original slider source");
    let snapshot_json = semio_framework_pack_json::to_json_string(&fixture);
    let mut values = semio_framework_pack_json::Object::new();
    values.insert(slider_id.clone(), semio_framework_pack_json::Value::Number(8.0.into()));
    let patched = apply_generation_values_to_host_snapshot(&snapshot_json, &values);
    let reparsed = semio_framework_pack_json::parse(&patched, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("patched json");
    let slider = reparsed.get("widgets").and_then(|widgets| widgets.as_array()).and_then(|widgets| widgets.iter().find(|widget| widget.get("id").and_then(|id| id.as_str()) == Some(slider_id.as_str()))).expect("slider widget");
    assert_eq!(slider.get("value").and_then(|value| value.as_f64()), Some(8.0));
}
