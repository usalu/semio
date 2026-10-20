/// 👻️ Ghost, selection and drag laws: the ghost widget size, preview and label overlay, the selection and hover state, the merge drag, the node drag proximity and the same-named input and output ports.

#[test]
fn ghost_widget_matches_placed_neuron_size() {
    let mut host = host_with_test_fixture();
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "brep.sketch2d.circle".into(),
        extension: "brep".into(),
        name: "Sketch Circle".into(),
        abbreviation: "Circle".into(),
        icon: "emoji:⚪️".into(),
        summary: "Sketched circle profile".into(),
        inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
        outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
        ..Default::default()
    }]));
    let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
    host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
    let ghost_width = host.ghost_node.as_ref().expect("ghost").width;
    let placed_id = host.add_widget(descriptor, 80.0, 80.0).unwrap();
    let placed_width = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == placed_id).expect("placed").width;
    assert!((ghost_width - placed_width).abs() < 1e-6, "ghost width {ghost_width} != placed {placed_width}");
    host.retire_cold();
}

#[test]
fn ghost_widget_preview_and_clear() {
    let mut host = host_with_test_fixture();
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 42.0, 24.0).unwrap();
    let ghost = host.ghost_node.as_ref().expect("ghost");
    assert!((ghost.x - 42.0).abs() < 1e-6);
    assert!((ghost.y - 24.0).abs() < 1e-6);
    assert_eq!(ghost.name, "Add");
    assert_eq!(ghost.abbreviation, "Add");
    assert_eq!(ghost.icon, "emoji:➕️");
    host.clear_ghost_widget();
    assert!(host.ghost_node.is_none());
    host.retire_cold();
}

#[test]
fn ghost_widget_label_overlay_matches_placed_at_micro() {
    let mut host = host_with_test_fixture();
    host.set_viewport(1280, 800, 1.0);
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "brep.sketch2d.circle".into(),
        extension: "brep".into(),
        name: "Sketch Circle".into(),
        abbreviation: "Circle".into(),
        icon: "emoji:⚪️".into(),
        summary: "Sketched circle profile".into(),
        inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
        outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
        ..Default::default()
    }]));
    host.dag.viewport.set_automatic_lod(false);
    host.dag.viewport.set_forced_draw_lod_label("micro");
    let descriptor = r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#;
    host.set_ghost_widget(descriptor, 40.0, 40.0).unwrap();
    let ghost = host.dag.pointer.ghost_node().expect("ghost");
    assert_eq!(host.draw_lod_label(), "micro");
    assert_eq!(semio_framework_os_infinite_directed_presentation::DagDrawLod::Micro.node_label(), semio_framework_os_infinite_directed_presentation::DagNodeLabel::Name);
    assert!(semio_framework_os_infinite_directed_presentation::DagDrawLod::Micro.shows_port_labels());
    assert!(semio_framework_os_infinite_directed_presentation::DagDrawLod::Micro.shows_handles());
    let ghost_overlay_rows = label_overlay_rows_for_node_spec(&host.dag.viewport, &host.dag.graph, &host.dag.paint, ghost, true);
    assert_eq!(ghost_overlay_rows.len(), 3);
    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let overlay_ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
    assert_eq!(overlay_ghost_rows.len(), 3);
    let placed_node = {
        let widget = widget_from_descriptor(&serde_json::from_str::<WidgetDescriptor>(descriptor).unwrap(), "placed".into(), &host.kind_infos);
        let mut layout = protocol::value::ordered::OrderedMap::new();
        layout.insert("placed".into(), WidgetLayout { x: 80.0, y: 80.0 });
        let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &host.kind_infos, widget_node_size(&widget, &[], &host.kind_infos));
        widget.retire_cold();
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(layout));
        retirement.retire_cold();
        fit_node_size(&mut node);
        node
    };
    let placed_rows = label_overlay_rows_for_node_spec(&host.dag.viewport, &host.dag.graph, &host.dag.paint, &placed_node, false);
    assert_eq!(ghost_overlay_rows.len(), placed_rows.len());
    for (ghost_row, placed_row) in ghost_overlay_rows.iter().zip(placed_rows.iter()) {
        assert_eq!(ghost_row["text"], placed_row["text"]);
        assert_eq!(ghost_row["layout"], placed_row["layout"]);
        assert_eq!(ghost_row["align"], placed_row["align"]);
    }
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    host.retire_cold();
}

#[test]
fn rebuild_dag_preserves_ghost_overlay_at_micro() {
    let mut host = host_with_test_fixture();
    host.set_viewport(1280, 800, 1.0);
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![NeuronKindInfo {
        id: "brep.sketch2d.circle".into(),
        extension: "brep".into(),
        name: "Sketch Circle".into(),
        abbreviation: "Circle".into(),
        icon: "emoji:⚪️".into(),
        summary: "Sketched circle profile".into(),
        inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
        outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
        ..Default::default()
    }]));
    host.dag.viewport.set_automatic_lod(false);
    host.dag.viewport.set_forced_draw_lod_label("micro");
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"brep.sketch2d.circle"}"#, 12.0, 18.0).unwrap();
    host.rebuild_dag();
    assert!(host.dag.pointer.ghost_node().is_some());
    assert_eq!(host.draw_lod_label(), "micro");
    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let ghost_rows: Vec<_> = overlay["labels"].as_array().unwrap().iter().filter(|row| row["ghost"] == true).collect();
    assert_eq!(ghost_rows.len(), 3);
    host.retire_cold();
}

#[test]
fn ghost_widget_paint_scene_smoke() {
    let mut host = host_with_test_fixture();
    host.set_viewport(800, 600, 1.0);
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"math.add"}"#, 10.0, 20.0).unwrap();
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 800, 600, 1.0);
    host.retire_cold();
}

#[test]
fn selection_and_preview_state_round_trip() {
    let mut host = FlowHost::default();
    host.set_selection(&["slider".into(), "add".into()]);
    let selected=host.selected_widget_ids();
    assert_eq!(selected, vec!["slider", "add"]);
    host.set_hover(Some("add"));
    assert_eq!(host.hovered_widget_id().as_deref(), Some("add"));
    host.set_preview_off_json(r#"["add"]"#);
    assert_eq!(host.preview_off_widget_ids(), vec!["add"]);
    host.toggle_preview("add").unwrap();
    assert!(host.preview_off_widget_ids().is_empty());
    host.retire_cold();
}

#[test]
fn channel_hover_and_selection_round_trip_at_detail_lod() {
    let mut host = host_with_test_fixture();
    host.dag.viewport.set_automatic_lod(false);
    host.dag.viewport.set_forced_draw_lod_label("detail");
    host.set_hover_channel(Some("add"), Some("a"));
    let hovered=host.hover_facts().channel.unwrap();
    assert_eq!(hovered.widget_id, "add");
    assert_eq!(hovered.port, "a");
    assert_eq!(hovered.direction, dag::DagChannelDirection::In);
    host.set_selected_channels(&[semio_framework_os_infinite::board::schema::dag_input::DagChannelRef { widget_id: "add".into(), port: "a".into(), direction: semio_framework_os_infinite::board::schema::dag_input::DagChannelDirection::In }]);
    let selected = host.selected_channels();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].widget_id, "add");
    assert_eq!(selected[0].port, "a");
    host.retire_cold();
}

#[test]
fn drag_merge_node_preserves_single_fixture_widget() {
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
    let merge_id = host.add_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 120.0, 80.0).unwrap();
    host.set_viewport(800, 600, 1.0);
    let merge = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == merge_id).expect("merge").clone();
    let grab = Point::new(merge.x, merge.y);
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, grab);
    host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
    host.pointer_move_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
    host.pointer_up_screen(screen.x + 80.0, screen.y + 40.0, false, false, false);
    assert_eq!(host.host_snapshot.widgets.iter().filter(|w| widget_id_for(w) == merge_id).count(), 1);
    assert_eq!(host.dag.graph.host_snapshot.nodes.iter().filter(|n| n.id == merge_id).count(), 1);
    let moved = host.host_snapshot.layout.get(&merge_id).expect("merge layout");
    assert!((moved.x - merge.x).abs() > 1.0);
    host.retire_cold();
}

#[test]
fn ghost_widget_cleared_on_pointer_down_and_add_widget() {
    let mut host = host_with_test_fixture();
    host.set_ghost_widget(r#"{"kind":"neuron","neuronKind":"dictionary.merge"}"#, 12.0, 18.0).unwrap();
    host.set_viewport(800, 600, 1.0);
    host.pointer_down_screen(120.0, 120.0, 0, false, false, false, false);
    assert!(host.ghost_node.is_none());
    host.set_ghost_widget(r#"{"kind":"inputSlider","label":"Number"}"#, 0.0, 0.0).unwrap();
    let _ = host.add_widget(r#"{"kind":"inputSlider","label":"Number"}"#, 40.0, 40.0).unwrap();
    assert!(host.ghost_node.is_none());
    assert_eq!(host.host_snapshot.widgets.iter().filter(|w| widget_id_for(w).starts_with("slider")).count(), 2);
    assert_eq!(host.dag.graph.host_snapshot.nodes.iter().filter(|n| n.id == "slider").count(), 1);
    host.retire_cold();
}

#[test]
fn delete_selection_removes_widget_from_host_snapshot() {
    let mut host = host_with_test_fixture();
    host.dag.graph.set_selection(&["slider".into()]);
    host.delete_selection().unwrap();
    assert!(host.host_snapshot.widgets.iter().all(|w| widget_id_for(w) != "slider"));
    assert!(host.dag.graph.host_snapshot.nodes.iter().all(|n| n.id != "slider"));
    host.retire_cold();
}

#[test]
fn node_drag_proximity_skips_wired_cut_inputs_in_flow() {
    use canvas::camera::{world_to_screen, Camera, Viewport};
    use canvas::Point;
    let mut host = FlowHost::default();
    host.set_viewport(1280, 800, 1.0);
    host.host_snapshot.replace_widgets(vec![
        Widget::Neuron { id: "sphere".into(), neuron_kind: "brep.prim3d.sphere".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
        Widget::Neuron { id: "torus".into(), neuron_kind: "brep.prim3d.torus".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: false },
        Widget::Neuron { id: "cut".into(), neuron_kind: "brep.bool.cut".into(), params: Dictionary::new(), input_ports: vec!["a".into(), "b".into()], output_ports: vec![], preview: true },
    ]);
    host.host_snapshot.synapses = vec![
        SynapseSpec { id: "e1".into(), from: "sphere".into(), to: "cut".into(), from_port: "solid".into(), to_port: "a".into() },
        SynapseSpec { id: "e2".into(), from: "torus".into(), to: "cut".into(), from_port: "solid".into(), to_port: "b".into() },
    ];
    host.host_snapshot.layout.insert("sphere".into(), WidgetLayout { x: 0.0, y: -60.0 });
    host.host_snapshot.layout.insert("torus".into(), WidgetLayout { x: 0.0, y: 60.0 });
    host.host_snapshot.layout.insert("cut".into(), WidgetLayout { x: 240.0, y: 0.0 });
    let solid_out = vec![InputSpec::named("S", "Sld", "solid", "Solid")];
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![
        NeuronKindInfo {
            id: "brep.prim3d.sphere".into(),
            extension: "brep".into(),
            name: "Sphere".into(),
            abbreviation: "Sphere".into(),
            icon: "emoji:⚪️".into(),
            summary: "Sphere".into(),
            inputs: vec![InputSpec::number_default("radius", 1.0, NUMBER_OPS)],
            outputs: solid_out.clone(),
            ..Default::default()
        },
        NeuronKindInfo {
            id: "brep.prim3d.torus".into(),
            extension: "brep".into(),
            name: "Torus".into(),
            abbreviation: "Torus".into(),
            icon: "emoji:🛢️".into(),
            summary: "Torus".into(),
            inputs: vec![InputSpec::number_default("major", 2.0, NUMBER_OPS), InputSpec::number_default("minor", 0.5, NUMBER_OPS)],
            outputs: solid_out.clone(),
            ..Default::default()
        },
        NeuronKindInfo {
            id: "brep.bool.cut".into(),
            extension: "brep".into(),
            name: "Cut".into(),
            abbreviation: "Cut".into(),
            icon: "emoji:🔗️".into(),
            summary: "Cut".into(),
            inputs: vec![InputSpec::requires("a", &["geometry"]), InputSpec::requires("b", &["geometry"])],
            outputs: solid_out,
            ..Default::default()
        },
    ]));
    host.rebuild_dag();
    host.dag.graph.set_proximity_distance(160.0);
    host.dag.viewport.set_automatic_lod(false);
    host.dag.viewport.set_forced_draw_lod_label("normal");
    assert_eq!(host.dag.graph.engine.edges.len(), 2, "synapses should load as engine edges");
    let cut = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == "cut").expect("cut");
    let grab = Point::new(cut.x, cut.y);
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, grab);
    host.pointer_down_screen(screen.x, screen.y, 0, false, false, false, false);
    host.pointer_move_screen(screen.x - 180.0, screen.y, false, false, false);
    assert!(host.dag.graph.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges");
    host.pointer_up_screen(screen.x - 180.0, screen.y, false, false, false);
    assert_eq!(host.dag.graph.engine.edges.len(), 2);
    assert_eq!(host.host_snapshot.synapses.len(), 2);
    host.retire_cold();
}

#[test]
fn dag_bridge_keeps_same_named_brep_input_and_output_distinct() {
    let mut host = FlowHost::default();
    host.host_snapshot.replace_widgets(vec![
        Widget::Neuron { id: "extrude".into(), neuron_kind: "brep.solid.extrude".into(), params: Dictionary::new(), input_ports: vec!["wire".into(), "vector".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "brep".into(), neuron_kind: "brep.brep".into(), params: Dictionary::new(), input_ports: vec!["brep".into(), "vertex".into(), "edge".into(), "face".into()], output_ports: vec![], preview: true },
        Widget::Neuron { id: "get".into(), neuron_kind: "list.get".into(), params: Dictionary::new(), input_ports: vec!["list".into(), "index".into(), "wrap".into()], output_ports: vec!["0".into()], preview: true },
    ]);
    host.host_snapshot.synapses = vec![
        SynapseSpec { id: "e112".into(), from: "extrude".into(), to: "brep".into(), from_port: "solid".into(), to_port: "brep".into() },
        SynapseSpec { id: "e113".into(), from: "brep".into(), to: "get".into(), from_port: "brep".into(), to_port: "list".into() },
    ];
    host.host_snapshot.layout.insert("extrude".into(), WidgetLayout { x: 0.0, y: 0.0 });
    host.host_snapshot.layout.insert("brep".into(), WidgetLayout { x: 200.0, y: 0.0 });
    host.host_snapshot.layout.insert("get".into(), WidgetLayout { x: 400.0, y: 0.0 });
    host.set_neuron_kind_infos_json(&kind_infos_json(vec![
        NeuronKindInfo {
            id: "brep.solid.extrude".into(),
            extension: "brep".into(),
            name: "Extrude".into(),
            abbreviation: "Extr".into(),
            icon: "emoji:⬆️".into(),
            summary: "Extrude".into(),
            inputs: vec![InputSpec::requires("wire", &["geometry"]), InputSpec::requires("vector", &["vector"])],
            outputs: vec![InputSpec::named("S", "Sld", "solid", "Solid")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "brep.brep".into(),
            extension: "brep".into(),
            name: "Brep".into(),
            abbreviation: "Brep".into(),
            icon: "emoji:🧊️".into(),
            summary: "Brep".into(),
            inputs: vec![InputSpec::requires("brep", &["brep.brep"]), InputSpec::list("vertex", &["brep.brep"]), InputSpec::list("edge", &["brep.brep"]), InputSpec::list("face", &["brep.brep"])],
            outputs: vec![InputSpec::named("B", "Brp", "brep", "Brep")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "list.get".into(),
            extension: "list".into(),
            name: "Get".into(),
            abbreviation: "Get".into(),
            icon: "emoji:📋️".into(),
            summary: "Get".into(),
            inputs: vec![InputSpec::list("list", &["list.get"]), InputSpec::number_default("index", 0.0, &["list.get"]), InputSpec::boolean_default("wrap", false, &["list.get"])],
            outputs: vec![InputSpec::named("V", "Val", "value", "ListValue")],
            ..Default::default()
        },
    ]));
    host.rebuild_dag();
    let incoming = host.dag.graph.engine.edges.get(&112).expect("incoming brep edge");
    let outgoing = host.dag.graph.engine.edges.get(&113).expect("outgoing brep edge");
    let incoming_target = host.dag.graph.engine.handles.get(&incoming.target).expect("incoming target");
    let outgoing_source = host.dag.graph.engine.handles.get(&outgoing.source).expect("outgoing source");
    assert_eq!(incoming_target.role, HandleRole::Target);
    assert_eq!(outgoing_source.role, HandleRole::Source);
    host.retire_cold();
}
