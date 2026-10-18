/// 🕸️ Node graph surface and DAG edges: the fixed admission surface bound, the contributed extension manifest, the synapse to DAG edge and port laws, the slider ghost and the widget catalogue payload.

/// 🛍️ The operator catalogue is APP-STATIC — it leaves the per-scene extras entirely and rides
/// `FlowAppCatalogue` on the reserved `framework.section.catalogue` surface instead
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).

/// 🛡️ THE surface bound law. A node-graph surface is admitted against `ui_contract::UI_FIXED_BYTES`
/// (32 KiB, a preallocated per-surface capacity), and before this ticket every flow-backed window
/// embedded the whole registered operator catalogue in its scene: with the real `brep`/`math` sets
/// installed that surface measured 111 031 B and the window could not render at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). The catalogue is app-static, so it moved to the
/// reserved `framework.section.catalogue` retained surface — and the scene now names operators by kind
/// id only. This law installs FIVE HUNDRED operators, an order of magnitude past the real sets, and
/// asserts the surface is unmoved by them.
#[test]
fn a_node_graph_surface_stays_under_the_fixed_admission_with_five_hundred_operators() {
    let _serialized = semio_framework_os_flow_operators::registry::lock_flow_extension_registry_for_test();
    const OPERATORS: usize = 500;
    let operators = (0..OPERATORS)
        .map(|index| format!(r#"{{"id":"bulk.op{index}","extension":"bulk","name":"Bulk Operator {index}","abbreviation":"B{index}","icon":"box","summary":"Bulk catalogue operator {index} with a deliberately verbose summary line","inputs":[],"outputs":[]}}"#))
        .collect::<Vec<_>>()
        .join(",");
    let manifest = format!(r#"{{"schema":"flow.extension","id":"bulk","name":"Bulk","version":"0.0.1","activationEvents":[],"contributes":{{"schemas":[],"operators":[{operators}],"widgets":[],"commands":[],"settings":[]}}}}"#);
    install_flow_extension_manifest("bulk-plugin", &manifest).expect("bulk extension admission");

    let catalogue_bytes = flow_app_catalogue_json().len();
    let registered = flow_app_catalogue().operators.len();
    assert!(registered >= OPERATORS, "the registry must actually hold the bulk operators, holds {registered}");

    let fixture = FlowHostSnapshot::default();
    let extras = flow_backed_node_graph_extras(&fixture, FLOW_LOD_MODE_AUTOMATIC, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, None);
    let scene = ui_wgpu::wgpu::NodeGraphScene {
        editable: Some(true),
        capabilities_json: extras.capabilities_json,
        lod_json: extras.lod_json,
        host_snapshot_json: extras.host_snapshot_json,
        eval_json: extras.eval_json,
        status_json: extras.status_json,
        ..ui_wgpu::wgpu::NodeGraphScene::base(Vec::new(), Vec::new(), semio_framework_os_kernel::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 })
    };
    let props = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::NodeGraph, &scene).expect("a node-graph surface must fit the fixed admission with 500 operators registered");
    let surface_bytes = props.doc.bytes.as_slice().len();
    println!("[STATS] node-graph surface with {registered} registered operators: surface={surface_bytes} B of {} B, app catalogue={catalogue_bytes} B", ui_contract::UI_FIXED_BYTES);
    assert!(surface_bytes < ui_contract::UI_FIXED_BYTES, "surface is {surface_bytes} B, over the {} B fixed admission", ui_contract::UI_FIXED_BYTES);
    assert!(catalogue_bytes > ui_contract::UI_FIXED_BYTES, "the catalogue must be the thing that would not have fit: {catalogue_bytes} B");

    fixture.retire_cold();
    uninstall_flow_extension("bulk").expect("bulk extension uninstall admission");
}

#[test]
fn contributed_extension_manifest_installs_catalogue_operator() {
    let _serialized = semio_framework_os_flow_operators::registry::lock_flow_extension_registry_for_test();
    let manifest = r#"{"schema":"flow.extension","id":"stubext","name":"Stub","version":"0.0.1","activationEvents":[],"contributes":{"schemas":[],"operators":[{"id":"stubext.echo","extension":"stubext","name":"Echo","abbreviation":"Echo","icon":"emoji:📣️","summary":"Echo","inputs":[],"outputs":[]}],"widgets":[],"commands":[],"settings":[]}}"#;
    install_flow_extension_manifest("stub-plugin", manifest).expect("stub extension admission");
    assert!(flow_extension_registry().operator_info("stubext.echo").is_some());
    let sections = flow_catalogue_sections();
    assert!(sections.iter().any(|section| section.id == "stubext"));
    uninstall_flow_extension("stubext").expect("stub extension uninstall admission");
}

#[test]
fn flow_fixture_with_synapses_builds_dag_edges_and_ports() {
    let mut host = host_with_test_fixture();
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    host.replace_host_snapshot(<FlowHostSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(include_str!("../../../../📚️examples/🗣️.dsl.semio")).expect("fixture"));
    assert!(!host.dag.graph.host_snapshot.edges.is_empty(), "synapses should become dag edges");
    let add = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add node");
    assert_eq!(add.inputs().len(), 2);
    assert_eq!(add.outputs().len(), 1);
    let mut scene = canvas::Scene::new();
    host.set_canvas_theme_dark(true);
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    assert!(scene.path_count() > 8, "rich flow graph should paint edges and handles");
    host.retire_cold();

    let wired = FlowHost::from_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "height".into(), label: "Height".into(), value: 3.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
            Widget::OutputPreview { id: "result".into(), preview: Dictionary::new(), expanded: protocol::value::ordered::OrderedSet::new() },
        ],
        synapses: vec![SynapseSpec { id: "wire-height".into(), from: "height".into(), to: "result".into(), from_port: "out".into(), to_port: "in".into() }],
        layout: protocol::value::ordered::OrderedMap::new(),
    });
    assert_eq!(wired.dag.graph.host_snapshot.edges.len(), 1, "legacy out/in synapse becomes one workflow edge");
    assert_eq!(wired.dag.graph.host_snapshot.edges[0].source, "height@number");
    assert_eq!(wired.dag.graph.host_snapshot.edges[0].target, "result@");
    assert_eq!(wired.dag.graph.engine.edges.len(), 1);
    wired.retire_cold();

    let refused = FlowHost::from_host_snapshot(FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputSlider { id: "height".into(), label: "Height".into(), value: 3.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
            Widget::OutputPreview { id: "result".into(), preview: Dictionary::new(), expanded: protocol::value::ordered::OrderedSet::new() },
        ],
        synapses: vec![SynapseSpec { id: "dangling".into(), from: "height".into(), to: "result".into(), from_port: "out".into(), to_port: "missing".into() }],
        layout: protocol::value::ordered::OrderedMap::new(),
    });
    assert_eq!(refused.dag.graph.host_snapshot.edges.iter().map(|edge| edge.target.as_str()).collect::<Vec<_>>(), ["result@missing"], "an unresolved port stays in the document, unjoined, until it can be joined");
    assert_eq!(refused.dag.graph.engine.edges.len(), 0, "a synapse to a missing port is not a workflow edge");
    refused.retire_cold();
}

#[test]
fn legacy_out_in_synapse_joins_canonical_ports_and_paints_an_engine_edge() {
    let mut host = host_with_test_fixture();
    let mut snapshot = host.host_snapshot.clone();
    snapshot.synapses.clear();
    snapshot.synapses.push(SynapseSpec { id: "legacy-wire".into(), from: "slider".into(), to: "add".into(), from_port: "out".into(), to_port: "in".into() });
    host.replace_host_snapshot(snapshot);
    let edge = host.dag.graph.host_snapshot.edges.iter().find(|edge| edge.id == "legacy-wire").expect("legacy synapse");
    assert_eq!(edge.source, "slider@number");
    assert_eq!(edge.target, "add@a");
    assert_eq!(host.dag.graph.engine.edges.len(), 1);
    host.retire_cold();
}

#[test]
fn synapse_to_a_missing_port_does_not_create_an_engine_edge() {
    let mut host = host_with_test_fixture();
    let mut snapshot = host.host_snapshot.clone();
    snapshot.synapses.clear();
    snapshot.synapses.push(SynapseSpec { id: "dangling".into(), from: "slider".into(), to: "add".into(), from_port: "out".into(), to_port: "missing".into() });
    host.replace_host_snapshot(snapshot);
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.id == "dangling"), "an unresolved port stays in the document until it can be joined");
    assert!(host.dag.graph.engine.edges.is_empty());
    host.retire_cold();
}


#[test]
fn slider_ghost_descriptor_requires_authored_label() {
    let missing = r#"{"kind":"inputSlider"}"#;
    assert!(semio_framework_pack_json::from_json_str::<WidgetDescriptor>(missing, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(serde_json::from_str::<WidgetDescriptor>(missing).is_err());
    let descriptor: WidgetDescriptor = semio_framework_pack_json::from_json_str(r#"{"kind":"inputSlider","label":""}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored empty label");
    let WidgetDescriptor::InputSlider { label, .. } = descriptor else { panic!("expected a slider"); };
    assert!(label.is_empty());
}


#[test]
#[cfg(not(target_arch = "wasm32"))]
fn add_widget_and_connect() {
    let mut host = host_with_test_bridge();
    let id = host.add_widget(r#"{"kind":"neuron","neuronKind":"math.passThrough"}"#, 100.0, 50.0).unwrap();
    host.connect_ports("slider", "number", &id, "number").unwrap();
    host.connect_ports(&id, "number", "preview", "").unwrap();
    host.set_slider_value("slider", 4.0);
    host.evaluate_internal();
    assert_eq!(host.preview_text(), "4");
    host.retire_cold();
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn output_export_widget_catalogue_descriptor_and_payload() {
    let mut host = host_with_test_bridge();
    let sections = merge_catalogue_sections("").unwrap();
    let exports: Vec<_> = sections.iter().flat_map(|section| section.items.iter()).filter(|item| item.kind == "outputExport").collect();
    assert_eq!(exports.len(), 4);
    assert!(exports.iter().any(|item| item.format.as_deref() == Some("svg")));
    let id = host.add_widget(r#"{"kind":"outputExport","format":"png"}"#, 120.0, 80.0).unwrap();
    host.connect_ports("add", "sum", &id, "").unwrap();
    host.set_slider_value("slider", 4.0);
    host.evaluate_internal();
    let payload_json = host.export_payload_json(&id).expect("export payload");
    assert_ne!(payload_json, "{}");
    assert!(payload_json.contains("4") || payload_json.contains("value") || payload_json.contains("sum"));
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == id).expect("export node");
    assert!(matches!(node.kind, DagNodeKind::Export { .. }));
    host.retire_cold();
}
