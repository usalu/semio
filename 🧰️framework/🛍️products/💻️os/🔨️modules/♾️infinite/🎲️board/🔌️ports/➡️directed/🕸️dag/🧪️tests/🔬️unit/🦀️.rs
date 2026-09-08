
use super::*;

fn cursor_grant() -> DagCursorGrant {
    DagCursorGrant { fuel: 1, now_milliseconds: 1, deadline_milliseconds: 8, cancelled: false, interrupted: false }
}

#[test]
fn selected_nodes_cursor_censuses_and_emits_one_byte_per_grant() {
    let fixture =
        DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![DagNodeSpec { id: "a\"\\\n".into(), ..Default::default() }, DagNodeSpec { id: "β".into(), ..Default::default() }], edges: vec![] };
    let mut host = DagHost::from_fixture_without_layout(fixture);
    host.set_selection(&["a\"\\\n".into(), "β".into()]);
    let expected = dsl::os_pack::json::to_json_string(&host.selected_node_ids()).into_bytes();
    let mut cursor = DagSelectedNodesJsonCursor::default();
    let mut rejected = cursor_grant();
    rejected.fuel = 0;
    assert_eq!(cursor.step(&host, rejected), Err(DagCursorFault::NoFuel));
    let mut output = Vec::new();
    let mut census = None;
    loop {
        match cursor.step(&host, cursor_grant()).unwrap() {
            DagCursorStep::Census { bytes } => census = Some(bytes),
            DagCursorStep::Byte(byte) => output.push(byte),
            DagCursorStep::Complete => break,
            DagCursorStep::Progress { .. } => {}
        }
    }
    assert_eq!(census, Some(expected.len()));
    assert_eq!(output, expected);
    assert_eq!(cursor.step(&host, cursor_grant()), Ok(DagCursorStep::Complete));
}

#[test]
fn selected_edges_cursor_matches_direct_encode() {
    let mut host = DagHost::default_demo();
    let edge = host.fixture.edges.first().expect("demo edge").id.clone();
    host.set_selection_domains_json(&format!("{{\"nodes\":[],\"edges\":[{edge:?}],\"handles\":[]}}"));
    let expected = dsl::os_pack::json::to_json_string(&host.selected_edge_ids()).into_bytes();
    let mut cursor = DagSelectedNodesJsonCursor::edges();
    let mut output = Vec::new();
    let mut census = None;
    loop {
        match cursor.step(&host, cursor_grant()).unwrap() {
            DagCursorStep::Census { bytes } => census = Some(bytes),
            DagCursorStep::Byte(byte) => output.push(byte),
            DagCursorStep::Complete => break,
            DagCursorStep::Progress { .. } => {}
        }
    }
    assert_eq!(census, Some(expected.len()));
    assert_eq!(output, expected);
}

#[test]
fn bounded_interaction_projection_rejects_node_and_identifier_overflow() {
    let mut fixture = DagFixture::default();
    fixture.nodes = (0..=DAG_INTERACTION_NODE_CAPACITY).map(|index| DagNodeSpec { id: format!("node-{index}"), ..Default::default() }).collect();
    let host = DagHost::from_fixture_without_layout(fixture);
    assert_eq!(host.bounded_interaction_projection(0).unwrap_err(), DagInteractionPlanFault::NodeCredits);

    let mut fixture = DagFixture::default();
    fixture.nodes = vec![DagNodeSpec { id: "x".repeat(16 * 1024 + 1), ..Default::default() }];
    let host = DagHost::from_fixture_without_layout(fixture);
    assert_eq!(host.bounded_interaction_projection(0).unwrap_err(), DagInteractionPlanFault::StringCredits);
}

#[test]
fn io_node_handle_angles_left_right() {
    let (in_a, out_a) = io_node_handle_angles(0, 2, 0, 1);
    assert!(in_a > std::f64::consts::FRAC_PI_2);
    assert!(out_a.abs() < std::f64::consts::FRAC_PI_2);
}

#[test]
fn app_instance_node_serializes_and_sizes_n_ports() {
    let node = DagNodeSpec {
        id: "node-a".into(),
        name: "Draw".into(),
        abbreviation: "drw".into(),
        icon: "emoji:draw".into(),
        x: 100.0,
        y: 80.0,
        width: 180.0,
        height: 92.0,
        kind: DagNodeKind::AppInstance {
            instance_id: "app-1".into(),
            plugin_id: "draw".into(),
            app_id: "draw".into(),
            icon: "emoji:draw".into(),
            inputs: vec![IoPortSpec::simple("in-a", "In")],
            outputs: vec![IoPortSpec::simple("out-a", "Out"), IoPortSpec::simple("out-b", "Mesh")],
        },
        ..Default::default()
    };
    assert_eq!(dag_node_kind_tag(&node.kind), "appInstance");
    assert_eq!(node.inputs().len(), 1);
    assert_eq!(node.outputs().len(), 2);
    let json = dsl::os_pack::json::to_json_string(&node);
    assert!(json.contains("appInstance"));
    assert!(json.contains("instanceId"));
    let mut sized = node.clone();
    fit_node_size(&mut sized);
    assert!(sized.height >= 56.0);
}

#[test]
fn dag_selection_hover_and_dimmed_map_widget_ids() {
    let fixture = DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 0.0, 0.0, 160.0, 24.0),
            DagNodeSpec::computation(
                "b".into(),
                "B",
                "B",
                "emoji:🔷️".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                false,
                false,
                200.0,
                0.0,
                160.0,
                24.0,
            ),
        ],
        edges: vec![],
    };
    let mut host = DagHost::from_fixture(fixture);
    host.set_selection(&["a".to_string()]);
    assert_eq!(host.selected_node_ids(), vec!["a"]);
    host.set_hover(Some("b"));
    assert_eq!(host.hovered_node_id().as_deref(), Some("b"));
    host.set_dimmed(&["a".to_string()]);
    assert_eq!(host.dimmed_node_ids(), vec!["a"]);
}

#[test]
fn cycle_detection_blocks_back_edge() {
    let edges = vec![("a".into(), "b".into()), ("b".into(), "c".into())];
    assert!(would_create_cycle(&edges, "c", "a"));
    assert!(!would_create_cycle(&edges, "a", "c"));
}

/// 🧪️ Two-node/one-edge `dag.fixture` literal shared by the layout tests below.
fn ab_edge_layout_fixture() -> Value {
    let node = |id: &str| dsl::os_pack::json::object([("id".to_string(), Value::from(id)), ("x".to_string(), Value::from(0)), ("y".to_string(), Value::from(0)), ("handles".to_string(), Value::Array(vec![]))]);
    dsl::os_pack::json::object([
        ("schema".to_string(), Value::from("dag.fixture")),
        ("nodes".to_string(), Value::Array(vec![node("a"), node("b")])),
        ("edges".to_string(), Value::Array(vec![dsl::os_pack::json::object([("id".to_string(), Value::from("e1")), ("source".to_string(), Value::from("a")), ("target".to_string(), Value::from("b"))])])),
    ])
}

#[test]
fn dag_layout_left_right_orders_depth_on_x() {
    let mut fixture: Value = ab_edge_layout_fixture();
    apply_dag_layout_to_fixture_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();
    let a_x = fixture["nodes"][0]["x"].as_f64().unwrap();
    let b_x = fixture["nodes"][1]["x"].as_f64().unwrap();
    assert!(b_x > a_x + 1.0);
}

#[test]
fn dag_layout_top_bottom_orders_depth_on_y() {
    let mut fixture: Value = ab_edge_layout_fixture();
    let opts = DagLayoutOptions { orientation: DagLayoutOrientation::TopBottom, ..DagLayoutOptions::default() };
    apply_dag_layout_to_fixture_v1_value(&mut fixture, &opts).unwrap();
    let a_y = fixture["nodes"][0]["y"].as_f64().unwrap();
    let b_y = fixture["nodes"][1]["y"].as_f64().unwrap();
    assert!(b_y > a_y + 1.0);
}

#[test]
fn dag_layout_spacing_scales_coordinates() {
    let mut fixture: Value = ab_edge_layout_fixture();
    apply_dag_layout_to_fixture_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();
    let default_gap = (fixture["nodes"][1]["x"].as_f64().unwrap() - fixture["nodes"][0]["x"].as_f64().unwrap()).abs();
    let mut wide: Value = fixture.clone();
    apply_dag_layout_to_fixture_v1_value(&mut wide, &DagLayoutOptions { layer_spacing: 240.0, sibling_gap: 80.0, ..DagLayoutOptions::default() }).unwrap();
    let wide_gap = (wide["nodes"][1]["x"].as_f64().unwrap() - wide["nodes"][0]["x"].as_f64().unwrap()).abs();
    assert!(wide_gap > default_gap * 1.5);
}

#[test]
fn dag_node_spec_serde_round_trip_kinds() {
    let nodes = vec![
        DagNodeSpec::computation(
            "c".into(),
            "C",
            "C",
            "emoji:🔷️".into(),
            vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
            vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
            false,
            false,
            0.0,
            0.0,
            160.0,
            56.0,
        ),
        DagNodeSpec {
            id: "s".into(),
            name: "S".into(),
            abbreviation: "S".into(),
            icon: "emoji:🎚️".into(),
            x: 1.0,
            y: 2.0,
            width: 180.0,
            height: 80.0,
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
            ..Default::default()
        },
        DagNodeSpec {
            id: "m".into(),
            name: "M".into(),
            abbreviation: "M".into(),
            icon: "emoji:📋️".into(),
            x: 0.0,
            y: 0.0,
            width: 180.0,
            height: 80.0,
            kind: DagNodeKind::Select { options: vec!["A".into(), "B".into()], selected: 1, output: IoPortSpec { id: "out".into(), label: "mode".into(), ..Default::default() } },
            ..Default::default()
        },
        DagNodeSpec {
            id: "p".into(),
            name: "P".into(),
            abbreviation: "P".into(),
            icon: "emoji:🖥️".into(),
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 140.0,
            kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }), input: IoPortSpec { id: "in".into(), label: "result".into(), ..Default::default() } },
            ..Default::default()
        },
    ];
    for node in nodes {
        let json = dsl::os_pack::json::to_json_string(&node);
        let back: DagNodeSpec = dsl::os_pack::json::from_json_str(&json).unwrap();
        assert_eq!(node, back);
    }
}

#[test]
fn handle_hover_does_not_hover_parent_node() {
    let fixture = DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec::computation(
            "merge".into(),
            "Merge",
            "M",
            "emoji:🔀️".into(),
            vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }],
            vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
            false,
            false,
            0.0,
            0.0,
            160.0,
            48.0,
        )],
        edges: vec![],
    };
    let mut host = DagHost::from_fixture(fixture);
    host.set_viewport(800, 600, 1.0);
    let (handle_center, node_id) = {
        let snap = host.engine.render_snapshot();
        let (hid, center, _) = snap.handles.first().expect("input handle");
        let node_id = host.engine.handles.get(hid).expect("handle").node_id;
        (*center, node_id)
    };
    let (sx, sy) = world_to_screen_px(&host, handle_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    let hover = host.engine.hover.expect("handle hover");
    assert!(host.engine.handles.contains_key(&hover));
    assert!(host.hovered_node_id().is_none());
    assert!(!host.is_node_hovered(node_id));
    host.pointer_move_screen(0.0, 0.0, false, false, false);
    assert!(host.engine.hover.is_none());
}

#[test]
fn idle_pointer_move_updates_hover() {
    use canvas::Point;
    use canvas::camera::{Camera, Viewport, world_to_screen};

    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    let camera = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let slider = world_to_screen(&camera, &viewport, Point::new(-400.0, -40.0));
    host.pointer_move_screen(slider.x, slider.y, false, false, false);
    assert_eq!(host.hovered_node_id().as_deref(), Some("slider"));
    host.pointer_move_screen(8.0, 8.0, false, false, false);
    assert!(host.hovered_node_id().is_none());
}

#[test]
fn dag_node_spec_port_accessors_per_kind() {
    let slider = DagNodeSpec {
        id: "s".into(),
        name: "S".into(),
        abbreviation: "S".into(),
        icon: "emoji:🎚️".into(),
        x: 0.0,
        y: 0.0,
        width: 180.0,
        height: 80.0,
        kind: DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
        ..Default::default()
    };
    assert!(slider.inputs().is_empty());
    assert_eq!(slider.outputs().len(), 1);
    let screen = DagNodeSpec {
        id: "p".into(),
        name: "P".into(),
        abbreviation: "P".into(),
        icon: "emoji:🖥️".into(),
        x: 0.0,
        y: 0.0,
        width: 200.0,
        height: 140.0,
        kind: DagNodeKind::Screen { media: None, input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
        ..Default::default()
    };
    assert_eq!(screen.inputs().len(), 1);
    assert!(screen.outputs().is_empty());
}

#[test]
fn dag_host_delete_selected_preserves_remaining_positions() {
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
            DagNodeSpec::computation(
                "b".into(),
                "B",
                "B",
                "emoji:🔷️".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                false,
                false,
                400.0,
                500.0,
                160.0,
                56.0,
            ),
            DagNodeSpec::computation("c".into(), "C", "C", "emoji:🔷️".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
        ],
        edges: vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagFixtureEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
    });
    host.set_selection(&["b".to_string()]);
    host.delete_selected();
    let a = host.fixture.nodes.iter().find(|n| n.id == "a").expect("a");
    let c = host.fixture.nodes.iter().find(|n| n.id == "c").expect("c");
    assert!((a.x - 100.0).abs() < 0.01);
    assert!((a.y - 200.0).abs() < 0.01);
    assert!((c.x - 700.0).abs() < 0.01);
    assert!((c.y - 300.0).abs() < 0.01);
    assert!(host.fixture.nodes.iter().all(|n| n.id != "b"));
}

#[test]
fn dag_host_delete_selected_removes_edge_only_selection() {
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
            DagNodeSpec::computation(
                "b".into(),
                "B",
                "B",
                "emoji:🔷️".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                false,
                false,
                400.0,
                500.0,
                160.0,
                56.0,
            ),
            DagNodeSpec::computation("c".into(), "C", "C", "emoji:🔷️".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
        ],
        edges: vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagFixtureEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
    });
    let edge_id = *host.engine.edges.keys().next().expect("edge");
    host.engine.selection.edge_ids.insert(edge_id);
    assert!(host.has_selection());
    assert_eq!(host.fixture.edges.len(), 2);
    host.delete_selected();
    assert_eq!(host.fixture.edges.len(), 1);
    assert_eq!(host.fixture.nodes.len(), 3);
    assert!(!host.has_selection());
}

#[test]
fn dag_host_reorganize_updates_engine_positions() {
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 500.0, 500.0, 160.0, 56.0),
            DagNodeSpec::computation("b".into(), "B", "B", "emoji:🔷️".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 500.0, 500.0, 160.0, 56.0),
        ],
        edges: vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }],
    });
    host.reorganize(&DagLayoutOptions::default()).unwrap();
    let a = host.fixture.nodes.iter().find(|n| n.id == "a").expect("a");
    let b = host.fixture.nodes.iter().find(|n| n.id == "b").expect("b");
    assert!(b.x > a.x);
}

#[test]
fn dag_host_loads_demo_fixture() {
    let host = DagHost::default_demo();
    assert_eq!(host.fixture.schema, "dag.fixture");
    assert_eq!(host.fixture.nodes.len(), 5);
    assert_eq!(host.fixture.edges.len(), 4);
    assert!(!host.engine.render_snapshot().edges.is_empty());
}

#[test]
fn slider_track_bounds_stay_inside_node_rect() {
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let node = DagNodeSpec {
        id: "slider".into(),
        name: "Amount".into(),
        abbreviation: "Amount".into(),
        icon: "emoji:🎚️".into(),
        x: 100.0,
        y: 50.0,
        width: slider_widget_width("Amount", &output),
        height: slider_widget_height(),
        kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
        ..Default::default()
    };
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let (left, top, right, bottom) = slider_track_bounds(&node);
    assert!(left >= node.x - hw);
    assert!(right <= node.x + hw);
    assert!((top + bottom) * 0.5 - node.y < 1e-6);
    assert!(top >= node.y - hh);
    assert!(bottom <= node.y + hh);
}

#[test]
fn dag_host_slider_drag_mutates_value() {
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "slider".into(),
            name: "Amount".into(),
            abbreviation: "Amount".into(),
            icon: "emoji:🎚️".into(),
            x: 0.0,
            y: 0.0,
            width: slider_widget_width("Amount", &output),
            height: slider_widget_height(),
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_viewport(800, 600, 1.0);
    let (x0, y0, x1, y1) = slider_track_bounds(&host.fixture.nodes[0]);
    let mid_y = (y0 + y1) * 0.5;
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new((x0 + x1) * 0.5, mid_y));
    host.pointer_down(sx, sy, false);
    host.pointer_up(sx, sy);
    let DagNodeKind::Slider { value, .. } = host.fixture.nodes[0].kind else {
        panic!("expected slider");
    };
    assert!((value - 2.0).abs() > 0.1);
}

#[test]
fn dag_host_slider_drag_ignored_when_controls_hidden() {
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "slider".into(),
            name: "Amount".into(),
            abbreviation: "Amount".into(),
            icon: "emoji:🎚️".into(),
            x: 0.0,
            y: 0.0,
            width: slider_widget_width("Amount", &output),
            height: slider_widget_height(),
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("minimap");
    host.set_viewport(800, 600, 1.0);
    let (x0, y0, x1, y1) = slider_track_bounds(&host.fixture.nodes[0]);
    let mid_y = (y0 + y1) * 0.5;
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new((x0 + x1) * 0.5, mid_y));
    host.pointer_down(sx, sy, false);
    host.pointer_up(sx, sy);
    let DagNodeKind::Slider { value, .. } = host.fixture.nodes[0].kind else {
        panic!("expected slider");
    };
    assert!((value - 2.0).abs() < 1e-6, "minimap LOD should only move the node rectangle, not adjust the value");
}

#[test]
fn dag_host_select_click_advances_option() {
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "mode".into(),
            name: "Mode".into(),
            abbreviation: "Mode".into(),
            icon: "emoji:📋️".into(),
            x: 0.0,
            y: 0.0,
            width: 180.0,
            height: 80.0,
            kind: DagNodeKind::Select { options: vec!["Add".into(), "Multiply".into()], selected: 0, output: IoPortSpec { id: "out".into(), label: "mode".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_viewport(800, 600, 1.0);
    let (x0, y0, x1, y1) = select_control_bounds(&host.fixture.nodes[0]);
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
    host.pointer_down(sx, sy, false);
    let DagNodeKind::Select { selected, .. } = host.fixture.nodes[0].kind else {
        panic!("expected select");
    };
    assert_eq!(selected, 1);
}

#[test]
fn dag_host_label_overlay_paint_state_json_includes_compact_labels() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("compact");
    let raw: Value = dsl::os_pack::json::parse(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    assert!(!labels.is_empty());
    assert!(labels.iter().all(|row| row["layout"] == "horizontal"));
    assert!(!labels[0]["text"].as_str().unwrap_or("").is_empty());
}

#[test]
fn dag_host_label_overlay_paint_state_json_includes_slider_name() {
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec {
            id: "slider".into(),
            name: "Radius".into(),
            abbreviation: "Radius".into(),
            icon: "emoji:🎚️".into(),
            x: 0.0,
            y: 0.0,
            width: 120.0,
            height: 32.0,
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("micro");
    let raw: Value = dsl::os_pack::json::parse(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    assert!(labels.iter().any(|row| row["text"] == "Radius" && row["layout"] == "horizontal"));
}

#[test]
fn dag_host_slider_overlay_preserves_language_neutral_field_labels() {
    let fixture: Value = dsl::os_pack::json::parse(include_str!("../../🧪️fixtures/🎚️slider-overlay.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let row = &case["row"];
        let host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: row["widgetId"].as_str().unwrap().into(),
                name: row["label"].as_str().unwrap().into(),
                abbreviation: "short".into(),
                width: 120.0,
                height: 32.0,
                kind: DagNodeKind::Slider {
                    min: row["min"].as_f64().unwrap(),
                    max: row["max"].as_f64().unwrap(),
                    step: row["step"].as_f64().unwrap(),
                    value: row["value"].as_f64().unwrap(),
                    output: IoPortSpec { id: "out".into(), label: "internal-output".into(), ..Default::default() },
                },
                ..Default::default()
            }],
            edges: vec![],
        });
        let actual: Value = dsl::os_pack::json::parse(&host.slider_overlay_state_json().unwrap()).unwrap();
        for key in ["widgetId", "label", "value", "min", "max", "step"] {
            assert_eq!(actual["sliders"][0][key], row[key], "{key}");
        }
    }
}

#[test]
fn dag_host_slider_overlay_state_json_includes_slider_track() {
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec {
            id: "slider".into(),
            name: "Radius".into(),
            abbreviation: "Radius".into(),
            icon: "emoji:🎚️".into(),
            x: 0.0,
            y: 0.0,
            width: 120.0,
            height: 32.0,
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_viewport(1280, 800, 1.0);
    let raw: Value = dsl::os_pack::json::parse(&host.slider_overlay_state_json().unwrap()).unwrap();
    let sliders = raw["sliders"].as_array().expect("sliders");
    assert_eq!(sliders.len(), 1);
    assert_eq!(sliders[0]["widgetId"], "slider");
    assert_eq!(sliders[0]["label"], "Radius");
    assert_eq!(sliders[0]["value"], 3.0);
    assert_eq!(sliders[0]["min"], 0.0);
    assert_eq!(sliders[0]["max"], 10.0);
}

#[test]
fn label_overlay_port_rows_are_not_duplicated_in_json() {
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec {
            id: "combine".into(),
            name: "Combine".into(),
            abbreviation: "Combine".into(),
            icon: "emoji:🔀️".into(),
            x: 0.0,
            y: 0.0,
            width: 104.0,
            height: 28.0,
            kind: DagNodeKind::Computation {
                inputs: vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }],
                outputs: vec![IoPortSpec { id: "out".into(), label: "merged".into(), ..Default::default() }],
                variadic_inputs: false,
                variadic_outputs: false,
            },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("normal");
    let raw: Value = dsl::os_pack::json::parse(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    let port_rows: Vec<_> = labels.iter().filter(|row| row["kind"].as_str() == Some("port")).map(|row| (row["text"].as_str().unwrap_or(""), row["align"].as_str().unwrap_or(""))).collect();
    assert_eq!(port_rows.len(), 3);
    assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! a").count(), 1);
    assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! b").count(), 1);
    assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! merged").count(), 1);
}

#[test]
fn io_port_label_with_cardinality_prefixes_symbol() {
    let mut port = IoPortSpec::named("S", "Sld", "solid", "ExtrudedSolid");
    port.cardinality = "*".into();
    assert_eq!(port.label_with_cardinality(DagDrawLod::Normal), "* Sld");
}

#[test]
fn io_port_label_with_cardinality_marks_unresolved_blocked_inputs() {
    let mut port = IoPortSpec::named("R", "Rad", "radius", "Radius");
    port.resolved = Some(false);
    assert_eq!(port.label_with_cardinality(DagDrawLod::Normal), "? Rad");
    assert_eq!(port.label_with_cardinality(DagDrawLod::Detail), "? radius");
}

#[test]
fn io_port_display_label_follows_draw_lod() {
    let port = IoPortSpec::named("S", "Sld", "solid", "ExtrudedSolid");
    assert_eq!(port.display_label(DagDrawLod::Normal), "Sld");
    assert_eq!(port.display_label(DagDrawLod::Detail), "solid");
    assert_eq!(port.display_label(DagDrawLod::Micro), "ExtrudedSolid");
}

#[test]
fn dag_host_label_overlay_port_text_follows_draw_lod() {
    let node = DagNodeSpec {
        id: "box".into(),
        name: "Box".into(),
        abbreviation: "Box".into(),
        icon: "emoji:📦️".into(),
        x: 0.0,
        y: 0.0,
        width: 96.0,
        height: 42.0,
        kind: DagNodeKind::Computation { inputs: vec![IoPortSpec::named("W", "Wid", "width", "BoxWidth")], outputs: vec![IoPortSpec::named("S", "Sld", "solid", "BoxSolid")], variadic_inputs: false, variadic_outputs: false },
        ..Default::default()
    };
    let port_texts = |lod: &str| -> Vec<String> {
        let mut host = DagHost::from_fixture_without_layout(DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 }, nodes: vec![node.clone()], edges: vec![] });
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label(lod);
        let raw: Value = dsl::os_pack::json::parse(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        raw["labels"].as_array().expect("labels").iter().filter(|row| row["align"].as_str().is_some()).filter_map(|row| row["text"].as_str().map(str::to_string)).collect()
    };
    assert!(port_texts("normal").contains(&"! Wid".into()));
    assert!(port_texts("normal").contains(&"! Sld".into()));
    assert!(port_texts("detail").contains(&"! width".into()));
    assert!(port_texts("detail").contains(&"! solid".into()));
    assert!(port_texts("micro").contains(&"! BoxWidth".into()));
    assert!(port_texts("micro").contains(&"! BoxSolid".into()));
}

#[test]
fn dag_host_exports_screen_overlay_rect() {
    let host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "screen".into(),
            name: "Preview".into(),
            abbreviation: "Preview".into(),
            icon: "emoji:🖥️".into(),
            x: 100.0,
            y: 50.0,
            width: 200.0,
            height: 140.0,
            kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }), input: IoPortSpec { id: "in".into(), label: "result".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    let mut host = host;
    host.set_viewport(1280, 800, 1.0);
    let json = host.node_overlays_json().unwrap();
    let overlays: Vec<Value> = dsl::os_pack::json::parse(&json).unwrap().as_array().cloned().unwrap_or_default();
    assert_eq!(overlays.len(), 1);
    assert_eq!(overlays[0]["id"], "screen");
    assert_eq!(overlays[0]["mediaKind"], "svg");
    assert!(overlays[0]["rect"]["w"].as_f64().unwrap_or(0.0) > 10.0);
}

fn handle_world(host: &DagHost, port_key: &str) -> canvas::Point {
    let hid = host.handle_key_map.iter().find(|(_, key)| key.as_str() == port_key).map(|(id, _)| *id).expect("handle");
    host.engine.render_snapshot().handles.iter().find(|(id, _, _)| *id == hid).map(|(_, p, _)| *p).expect("handle pos")
}

fn world_to_screen_px(host: &DagHost, p: canvas::Point) -> (f64, f64) {
    (p.x + host.width as f64 * 0.5, p.y + host.height as f64 * 0.5)
}

#[test]
fn dag_host_area_select_previews_preselect_before_commit() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    let start_sx = 24.0;
    let start_sy = 24.0;
    let end_sx = 1100.0;
    let end_sy = 700.0;
    host.pointer_down_screen(start_sx, start_sy, 0, false, false, false, false);
    host.pointer_move_screen(end_sx, end_sy, false, false, false);
    assert!(matches!(host.engine.interaction, InteractionMode::AreaSelect { .. }), "expected area-select after marquee threshold");
    let preselect = host.preselect_widget_ids();
    assert!(!preselect.is_empty(), "marquee drag should preview widget ids before commit");
    let preview_points: Vec<[f64; 2]> = dsl::os_pack::json::from_json_str(&host.selection_preview_points_json()).unwrap();
    assert!(preview_points.len() >= 2, "marquee overlay points should be published during drag");
    host.pointer_up_screen(end_sx, end_sy, false, false, false);
    assert!(!host.selected_node_ids().is_empty(), "marquee drag should commit selection on release");
    assert!(host.preselect_widget_ids().is_empty(), "preselect should clear after commit");
}

#[test]
fn dag_host_align_selection_horizontal_and_vertical_center() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    host.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
    host.align_selection("alignHorizontal").unwrap();
    let xs: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.x).collect();
    assert!(xs.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
    host.align_selection("alignVertical").unwrap();
    let ys: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.y).collect();
    assert!(ys.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
}

#[test]
fn dag_host_align_selection_left_and_distribute_horizontal() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    host.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
    host.align_selection("alignLeft").unwrap();
    let left_edges: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.x - node.width * 0.5).collect();
    assert!(left_edges.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
    host.align_selection("distributeHorizontal").unwrap();
    let mut xs: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.x).collect();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(xs.windows(2).all(|pair| pair[1] > pair[0]));
}

#[test]
fn dag_host_selection_union_bounds_screen_json_nonempty_for_selection() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    host.set_selection(&["scale".into(), "combine".into()]);
    let json = host.selection_union_bounds_screen_json();
    assert_ne!(json, "null");
    let parsed: Value = dsl::os_pack::json::parse(&json).unwrap();
    assert!(parsed["width"].as_f64().unwrap_or(0.0) > 1.0);
    assert!(parsed["height"].as_f64().unwrap_or(0.0) > 1.0);
}

#[test]
fn dag_host_entity_screen_json_resolves_node_by_id_and_wildcard() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    let json = host.entity_screen_json("node", "scale");
    let parsed: Value = dsl::os_pack::json::parse(&json).unwrap();
    assert_eq!(parsed["visible"], true);
    assert!(parsed["x"].as_number().is_some());
    assert!(parsed["rect"].as_array().is_some());

    let wildcard: Value = dsl::os_pack::json::parse(&host.entity_screen_json("node", "*")).unwrap();
    assert_eq!(wildcard["visible"], true);
}

#[test]
fn dag_host_entity_screen_json_resolves_handle_by_widget_and_port() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    let input_json: Value = dsl::os_pack::json::parse(&host.entity_screen_json("handle", "scale@in")).unwrap();
    assert_eq!(input_json["visible"], true);
    let output_json: Value = dsl::os_pack::json::parse(&host.entity_screen_json("handle", "combine@b")).unwrap();
    assert_eq!(output_json["visible"], true);
    // 🐢️ A malformed id (no "@port") or a port that doesn't exist on the node must degrade to
    // unresolved, never panic.
    let malformed: Value = dsl::os_pack::json::parse(&host.entity_screen_json("handle", "scale")).unwrap();
    assert_eq!(malformed["visible"], false);
    let missing_port: Value = dsl::os_pack::json::parse(&host.entity_screen_json("handle", "scale@nope")).unwrap();
    assert_eq!(missing_port["visible"], false);
}

#[test]
fn dag_host_entity_screen_json_resolves_edge_with_a_two_point_polyline() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    let json: Value = dsl::os_pack::json::parse(&host.entity_screen_json("edge", "e1")).unwrap();
    assert_eq!(json["visible"], true);
    let polyline = json["polyline"].as_array().expect("edge geometry carries a polyline");
    assert_eq!(polyline.len(), 2);
}

#[test]
fn dag_host_entity_screen_json_unresolved_domain_or_id_never_panics() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    for (domain, id) in [("node", "nonexistent"), ("handle", "*"), ("edge", "nonexistent"), ("bogus-domain", "*")] {
        let json: Value = dsl::os_pack::json::parse(&host.entity_screen_json(domain, id)).unwrap();
        if json["visible"] == true {
            continue; // "handle":"*" may legitimately resolve to the demo fixture's first port.
        }
        assert_eq!(json["visible"], false, "domain={domain} id={id}");
    }
    let empty_fixture = DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] };
    let empty: Value = dsl::os_pack::json::parse(&DagHost::from_fixture(empty_fixture).entity_screen_json("node", "*")).unwrap();
    assert_eq!(empty["visible"], false);
}

#[test]
fn dag_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
    let mut host = DagHost::default_demo();
    host.set_viewport(800, 600, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("minimap");
    host.set_camera(0.0, 0.0, 0.1);
    host.set_selection(&["scale".into(), "combine".into()]);
    let scale_before = host.fixture.nodes.iter().find(|n| n.id == "scale").expect("scale").clone();
    let combine_before = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let gap = canvas::Point::new(0.0, 0.0);
    use canvas::camera::{Camera as CanvasCamera, Viewport, world_to_screen};
    let cam = CanvasCamera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let start = world_to_screen(&cam, &viewport, gap);
    host.pointer_down_screen(start.x, start.y, 0, false, false, false, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DragNodes { .. }), "expected bounded drag inside selection union at minimap LOD");
    host.pointer_move_screen(start.x + 50.0, start.y + 30.0, false, false, false);
    host.pointer_up_screen(start.x + 50.0, start.y + 30.0, false, false, false);
    let zoom = host.fixture.camera.zoom;
    let dx = 50.0 / zoom;
    let dy = 30.0 / zoom;
    let scale_after = host.fixture.nodes.iter().find(|n| n.id == "scale").expect("scale");
    let combine_after = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine");
    assert!((scale_after.x - (scale_before.x + dx)).abs() < 1e-3 && (scale_after.y - (scale_before.y + dy)).abs() < 1e-3);
    assert!((combine_after.x - (combine_before.x + dx)).abs() < 1e-3 && (combine_after.y - (combine_before.y + dy)).abs() < 1e-3);
}

#[test]
fn dag_host_drags_node_in_world_space() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    let mut dragged = false;
    for (nid, node) in host.engine.nodes.clone() {
        let grab = canvas::Point::new(node.center.x - node.width * 0.4, node.center.y);
        let (sx, sy) = world_to_screen_px(&host, grab);
        host.pointer_down(sx, sy, false);
        if !matches!(host.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
            host.pointer_up(sx, sy);
            continue;
        }
        let before = host.engine.nodes.get(&nid).expect("node").center;
        host.pointer_move(sx + 40.0, sy + 30.0);
        let idx = *host.node_id_map.get(&nid).expect("fixture index");
        let fixture = &host.fixture.nodes[idx];
        let engine = host.engine.nodes.get(&nid).expect("node").center;
        assert!((fixture.x - engine.x).abs() < 1e-6, "fixture x should track engine during drag");
        assert!((fixture.y - engine.y).abs() < 1e-6, "fixture y should track engine during drag");
        host.pointer_up(sx + 40.0, sy + 30.0);
        let after = host.engine.nodes.get(&nid).expect("node").center;
        assert!((after.x - before.x).abs() > 1.0);
        assert!((after.y - before.y).abs() > 1.0);
        dragged = true;
        break;
    }
    assert!(dragged, "expected at least one draggable node hit via screen coordinates");
}

#[test]
fn dag_host_grid_snap_aligns_dragged_node() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("normal");
    host.set_grid_snap_enabled(true);
    host.set_grid_factor(10.0).expect("grid factor");
    let mut dragged = false;
    for (nid, node) in host.engine.nodes.clone() {
        let grab = canvas::Point::new(node.center.x - node.width * 0.4, node.center.y);
        let (sx, sy) = world_to_screen_px(&host, grab);
        host.pointer_down(sx, sy, false);
        if !matches!(host.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
            host.pointer_up(sx, sy);
            continue;
        }
        host.pointer_move(sx + 37.0, sy + 23.0);
        host.pointer_up(sx + 37.0, sy + 23.0);
        let idx = *host.node_id_map.get(&nid).expect("fixture index");
        let fixture = &host.fixture.nodes[idx];
        let step = GRID_WORLD_MEDIUM * 10.0;
        assert!(((fixture.x / step).round() * step - fixture.x).abs() < 1e-6);
        assert!(((fixture.y / step).round() * step - fixture.y).abs() < 1e-6);
        dragged = true;
        break;
    }
    assert!(dragged, "expected draggable node for grid snap test");
}

#[test]
fn dag_host_focus_selection_camera_frames_selection() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    let ids: Vec<String> = host.fixture.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    host.set_selection(&ids);
    let camera = host.focus_selection_camera(1.2).expect("camera");
    assert!(camera.zoom > 0.0);
    assert!(camera.x.is_finite() && camera.y.is_finite());
}

#[test]
fn dag_host_reconnects_edge_endpoint() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    let in_w = handle_world(&host, "combine@b");
    let out_w = handle_world(&host, "scale@out");
    let (in_sx, in_sy) = world_to_screen_px(&host, in_w);
    let (out_sx, out_sy) = world_to_screen_px(&host, out_w);
    host.pointer_down(in_sx, in_sy, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }));
    host.pointer_move(out_sx, out_sy);
    host.pointer_up(out_sx, out_sy);
    let e3 = host.fixture.edges.iter().find(|e| e.id == "e3").expect("e3");
    assert_eq!(e3.source, "scale@out");
}

#[test]
fn dag_host_node_drag_proximity_preview_and_connects() {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let tgt_h = computation_node_height(1, 1, false, false);
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("src".into(), "Src", "Src", "emoji:🔢️".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
            DagNodeSpec::computation("tgt".into(), "Tgt", "Tgt", "emoji:🔢️".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
        ],
        edges: vec![],
    });
    host.set_viewport(1280, 800, 1.0);
    host.set_proximity_distance(120.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("normal");
    let src_center = canvas::Point::new(0.0, 0.0);
    let (sx, sy) = world_to_screen_px(&host, src_center);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 200.0, sy, false, false, false);
    assert!(host.engine.render_snapshot().pending_edge.is_some(), "proximity drag should preview edge");
    host.pointer_up_screen(sx + 200.0, sy, false, false, false);
    assert!(host.fixture.edges.iter().any(|edge| edge.source == "src@out" && edge.target == "tgt@in"), "proximity drag should commit edge");
}

#[test]
fn dag_host_node_drag_skips_wired_cut_inputs() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let cut_w = computation_node_width("Cut", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let cut_h = computation_node_height(2, 1, false, false);
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("sphere".into(), "Sphere", "Sphere", "emoji:🔵️".into(), vec![], outputs.clone(), false, false, 0.0, -60.0, src_w, src_h),
            DagNodeSpec::computation("torus".into(), "Torus", "Torus", "emoji:🍩️".into(), vec![], outputs.clone(), false, false, 0.0, 60.0, src_w, src_h),
            DagNodeSpec::computation("cut".into(), "Cut", "Cut", "emoji:✂️".into(), inputs, outputs, false, false, 240.0, 0.0, cut_w, cut_h),
        ],
        edges: vec![DagFixtureEdge { id: "e1".into(), source: "sphere@out".into(), target: "cut@a".into(), ..Default::default() }, DagFixtureEdge { id: "e2".into(), source: "torus@out".into(), target: "cut@b".into(), ..Default::default() }],
    });
    assert_eq!(host.engine.edges.len(), 2, "fixture edges should load into engine");
    host.set_viewport(1280, 800, 1.0);
    host.set_proximity_distance(160.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("normal");
    let cut_center = canvas::Point::new(240.0, 0.0);
    let (sx, sy) = world_to_screen_px(&host, cut_center);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx - 180.0, sy, false, false, false);
    assert!(host.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges to occupied inputs");
    host.pointer_up_screen(sx - 180.0, sy, false, false, false);
    assert_eq!(host.engine.edges.len(), 2);
    assert_eq!(host.fixture.edges.len(), 2);
}

#[test]
fn dag_host_keeps_same_named_input_and_output_handles_distinct() {
    let solid = vec![IoPortSpec { id: "solid".into(), label: "solid".into(), ..Default::default() }];
    let brep = vec![IoPortSpec { id: "brep".into(), label: "brep".into(), ..Default::default() }];
    let list = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }];
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("extrude".into(), "Extrude", "Extrude", "emoji:⬆️".into(), vec![], solid.clone(), false, false, 0.0, 0.0, computation_node_width("Extrude", &[], &solid), computation_node_height(0, 1, false, false)),
            DagNodeSpec::computation("brep".into(), "Brep", "Brep", "emoji:🧊️".into(), brep.clone(), brep.clone(), false, false, 200.0, 0.0, computation_node_width("Brep", &brep, &brep), computation_node_height(1, 1, false, false)),
            DagNodeSpec::computation(
                "get".into(),
                "Get",
                "Get",
                "emoji:📋️".into(),
                list,
                vec![],
                false,
                false,
                400.0,
                0.0,
                computation_node_width("Get", &[IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }], &[]),
                computation_node_height(1, 0, false, false),
            ),
        ],
        edges: vec![
            DagFixtureEdge { id: "e100".into(), source: "extrude@solid".into(), target: "brep@brep".into(), ..Default::default() },
            DagFixtureEdge { id: "e101".into(), source: "brep@brep".into(), target: "get@list".into(), ..Default::default() },
        ],
    });
    host.sync_edges_from_engine();
    let incoming = host.engine.edges.get(&100).expect("incoming brep edge");
    let outgoing = host.engine.edges.get(&101).expect("outgoing brep edge");
    let incoming_target = host.engine.handles.get(&incoming.target).expect("incoming target handle");
    let outgoing_source = host.engine.handles.get(&outgoing.source).expect("outgoing source handle");
    assert_eq!(incoming_target.role, HandleRole::Target);
    assert_eq!(outgoing_source.role, HandleRole::Source);
    assert_eq!(host.fixture.edges.iter().find(|edge| edge.id == "e100").map(|edge| edge.target.as_str()), Some("brep@brep"));
    assert_eq!(host.fixture.edges.iter().find(|edge| edge.id == "e101").map(|edge| edge.source.as_str()), Some("brep@brep"));
}

#[test]
fn dag_host_proximity_zero_disables_node_drag_connect() {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let tgt_h = computation_node_height(1, 1, false, false);
    let mut host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("src".into(), "Src", "Src", "emoji:🔢️".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
            DagNodeSpec::computation("tgt".into(), "Tgt", "Tgt", "emoji:🔢️".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
        ],
        edges: vec![],
    });
    host.set_viewport(1280, 800, 1.0);
    host.set_proximity_distance(0.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("normal");
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new(0.0, 0.0));
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 200.0, sy, false, false, false);
    assert!(host.engine.render_snapshot().pending_edge.is_none());
    host.pointer_up_screen(sx + 200.0, sy, false, false, false);
    assert!(host.fixture.edges.is_empty());
}

#[test]
fn hidden_lod_connection_hit_picking_disabled() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine");
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "combine@b");
    for lod in ["minimap", "overview", "compact"] {
        host.set_forced_draw_lod_label(lod);
        assert!(!host.draw_lod_for_frame().allows_connection_hit_picking(), "{lod}");
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_down(sx, sy, false);
        assert!(!matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), "{lod} input row should not start edge draw");
        host.pointer_up(sx, sy);
        let (hsx, hsy) = world_to_screen_px(&host, handle);
        host.pointer_down(hsx, hsy, false);
        assert!(!matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), "{lod} handle anchor should not start edge draw");
        host.pointer_up(hsx, hsy);
    }
}

#[test]
fn normal_lod_input_row_drags_node_handle_anchor_starts_edge_draw() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("normal");
    let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine");
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "combine@b");
    assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DragNode { .. }), "interior rectangle drag should move the node");
    host.pointer_up(sx, sy);
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.pointer_down(hsx, hsy, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }));
}

#[test]
fn set_hover_channel_targets_port_handle_at_detail_lod() {
    let mut host = DagHost::default_demo();
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("detail");
    host.set_hover_channel(Some("combine"), Some("b"));
    assert_eq!(host.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: "in".into() }));
    host.set_hover_channel(None, None);
    assert!(host.hovered_channel().is_none());
}

#[test]
fn set_hover_channel_falls_back_to_node_at_compact_lod() {
    let mut host = DagHost::default_demo();
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("compact");
    host.set_hover_channel(Some("combine"), Some("b"));
    assert_eq!(host.hovered_node_id().as_deref(), Some("combine"));
    assert!(host.hovered_channel().is_none());
}

#[test]
fn hovered_channel_decodes_port_handle_at_detail_lod() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("detail");
    let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    assert_eq!(host.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: "in".into() }));
}

#[test]
fn detail_lod_non_channel_body_hovers_and_selects_node() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("detail");
    let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    assert!(host.hovered_node_id().as_deref() == Some("combine"));
    assert!(host.engine.hover.is_some());
    assert!(!host.engine.selection.node_ids.contains(&host.node_id_for_widget_id("combine").expect("combine node id")));
    let divider_x = computation_column_divider_x(&combine).expect("divider");
    let (_, header_top, _, header_bottom) = channel_row_bounds(&combine, 0);
    let title_probe = canvas::Point::new(divider_x, (header_top + header_bottom) * 0.5);
    let (body_sx, body_sy) = world_to_screen_px(&host, title_probe);
    host.pointer_move_screen(body_sx, body_sy, false, false, false);
    assert!(host.hovered_node_id().as_deref() == Some("combine"));
    assert!(host.engine.hover.is_some());
    host.pointer_down(body_sx, body_sy, false);
    assert!(host.selected_node_ids().contains(&"combine".to_string()));
    host.pointer_up(body_sx, body_sy);
}

#[test]
fn visible_handle_lod_row_center_does_not_start_edge_draw() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("detail");
    let scale = host.fixture.nodes.iter().find(|n| n.id == "scale").expect("scale");
    let port_idx = scale.outputs().iter().position(|p| p.id == "out").expect("port out");
    let (x0, y0, x1, y1) = output_port_row_hit_bounds(scale, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "scale@out");
    assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(!matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), "visible handles require anchor hit for wire draw");
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.pointer_down(hsx, hsy, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }));
}

#[test]
fn detail_lod_channel_row_drags_node_without_prior_selection() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("micro");
    let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "combine@b");
    assert!((row_center.x - handle.x).abs() > 4.0);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DragNode { .. }));
    assert!(host.selected_node_ids().contains(&"combine".to_string()));
}

#[test]
fn detail_lod_title_row_drags_node() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("detail");
    let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "a").expect("port a");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let title_probe = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, title_probe);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.engine.interaction, InteractionMode::DragNode { .. }));
}

#[test]
fn input_port_row_hit_bounds_span_input_channel() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("Node", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, false);
    let node = DagNodeSpec::computation("n".into(), "Node", "Node", "emoji:🔢️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let hw = width * 0.5;
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let (x0, _, x1, _) = input_port_row_hit_bounds(&node, 1).expect("row");
    assert!((x0 - (node.x - hw)).abs() < 1e-9);
    assert!((x1 - divider_x).abs() < 1e-9);
}

#[test]
fn output_port_row_hit_bounds_span_output_channel() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "x".into(), label: "x".into(), ..Default::default() }, IoPortSpec { id: "y".into(), label: "y".into(), ..Default::default() }];
    let width = computation_node_width("Node", &inputs, &outputs);
    let height = computation_node_height(1, 2, false, false);
    let node = DagNodeSpec::computation("n".into(), "Node", "Node", "emoji:🔢️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let hw = width * 0.5;
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let (x0, _, x1, _) = output_port_row_hit_bounds(&node, 1).expect("row");
    assert!((x0 - divider_x).abs() < 1e-9);
    assert!((x1 - (node.x + hw)).abs() < 1e-9);
}

#[test]
fn variadic_plus_hit_maps_insert_index() {
    let inputs = vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }, IoPortSpec { id: "1".into(), label: "1".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("dictionary.merge", &inputs, &outputs);
    let height = computation_node_height(2, 1, true, false);
    let host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec::computation("merge".into(), "Merge", "Merge", "emoji:🔀️".into(), inputs, outputs, true, false, 0.0, 0.0, width, height)],
        edges: vec![],
    });
    let positions = variadic_input_insert_positions(&host.fixture.nodes[0]);
    assert_eq!(positions.len(), 1);
    let (_, px, py) = positions[0];
    let hit = host.port_insert_hit(px, py, 2.0).expect("hit");
    assert_eq!(hit.0, DagPortSide::Input);
    assert_eq!(hit.1, "merge");
    assert_eq!(hit.2, 2);
    assert!(host.port_insert_hit(px, py, 1.0).is_none());
}

#[test]
fn variadic_output_plus_hit_maps_insert_index() {
    let outputs = vec![IoPortSpec { id: "0".into(), label: "i".into(), ..Default::default() }];
    let inputs = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }, IoPortSpec { id: "index".into(), label: "index".into(), ..Default::default() }];
    let width = computation_node_width("list.get", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, true);
    let host = DagHost::from_fixture_without_layout(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec::computation("get".into(), "Get", "Get", "emoji:📋️".into(), inputs, outputs, false, true, 0.0, 0.0, width, height)],
        edges: vec![],
    });
    let positions = variadic_output_insert_positions(&host.fixture.nodes[0]);
    assert_eq!(positions.len(), 1);
    let (_, px, py) = positions[0];
    let hit = host.port_insert_hit(px, py, 2.0).expect("hit");
    assert_eq!(hit.0, DagPortSide::Output);
    assert_eq!(hit.1, "get");
    assert_eq!(hit.2, 1);
}

#[test]
fn component_width_is_twice_channel_width() {
    assert_eq!(DAG_COMPONENT_WIDTH, DAG_IO_COLUMN_WIDTH * 2.0);
}

#[test]
fn computation_node_width_is_uniform_for_all_components() {
    let inputs = vec![
        IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() },
        IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() },
        IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
    ];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    assert_eq!(width, DAG_COMPONENT_WIDTH);
}

#[test]
fn computation_io_columns_use_uniform_channel_width() {
    let inputs_short = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
    let inputs_long = vec![IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() }];
    let outputs_short = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let outputs_long = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let short = computation_node_width("n", &inputs_short, &outputs_short);
    let long = computation_node_width("n", &inputs_long, &outputs_long);
    assert_eq!(short, DAG_COMPONENT_WIDTH);
    assert_eq!(long, DAG_COMPONENT_WIDTH);
    assert_eq!(io_port_column_width(&inputs_long, DAG_LABEL_COMPACT_SCREEN_PX), DAG_IO_COLUMN_WIDTH);
    assert_eq!(io_port_column_width(&outputs_long, DAG_LABEL_COMPACT_SCREEN_PX), DAG_IO_COLUMN_WIDTH);
}

#[test]
fn computation_column_divider_splits_io_columns() {
    let inputs = vec![IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, false);
    let node = DagNodeSpec::computation("box".into(), "Box", "Box", "emoji:📦️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let hw = width * 0.5;
    assert!(divider_x > node.x - hw + 1.0);
    assert!(divider_x < node.x + hw - 1.0);
}

#[test]
fn computation_name_sits_above_rectangle_centered() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    let height = computation_node_height(1, 1, false, false);
    let node = DagNodeSpec::computation("box".into(), "Box", "Box", "emoji:📦️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let paint_px = dag_label_paint_px(1.0, 3);
    let (label_x, label_y) = computation_name_world_center(&node, "Box", paint_px, 1.0);
    assert!((label_x - node.x).abs() < 1e-6);
    let top = node.y - height * 0.5;
    assert!(label_y < top);
    let world_offset = top - label_y;
    let (_, label_h) = canvas::text::label_extent("Box", paint_px);
    let screen_offset = world_offset * 1.0;
    assert!((screen_offset - (DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_COMPACT_RATIO + label_h * 0.5)).abs() < 1e-6);
}

#[test]
fn io_widget_width_matches_component_width() {
    let width = io_widget_width("Amount");
    let height = io_widget_height("Amount");
    assert_eq!(width, DAG_COMPONENT_WIDTH);
    assert!(height >= 40.0);
}

#[test]
fn slider_widget_size_matches_function_row_metrics() {
    let input = IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() };
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let width = slider_widget_width("Amount", &output);
    let height = slider_widget_height();
    assert_eq!(width, DAG_COMPONENT_WIDTH);
    assert_eq!(width, computation_node_width("Amount", &[input], &[output]));
    assert_eq!(height, DAG_CHANNEL_ROW_HEIGHT);
    assert!(width > height, "slider track should be wider than tall");
}

#[test]
fn computation_channel_row_count_matches_io_rows() {
    let node = DagNodeSpec::computation(
        "box".into(),
        "Box",
        "Box",
        "emoji:📦️".into(),
        vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() },
            IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
        ],
        vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }],
        false,
        false,
        0.0,
        0.0,
        120.0,
        42.0,
    );
    assert_eq!(computation_channel_row_count(&node), 3);
}

#[test]
fn computation_channel_row_dividers_stop_at_last_port_on_shorter_side() {
    let three_inputs =
        vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "cornerB".into(), ..Default::default() }, IoPortSpec { id: "c".into(), label: "height".into(), ..Default::default() }];
    let one_output = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let three_outputs = vec![
        IoPortSpec { id: "outA".into(), label: "geometry".into(), ..Default::default() },
        IoPortSpec { id: "outB".into(), label: "mesh".into(), ..Default::default() },
        IoPortSpec { id: "outC".into(), label: "curve".into(), ..Default::default() },
    ];
    let one_input = vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }];
    let more_inputs = DagNodeSpec::computation(
        "more-in".into(),
        "Box",
        "Box",
        "emoji:📦️".into(),
        three_inputs.clone(),
        one_output.clone(),
        false,
        false,
        0.0,
        0.0,
        computation_node_width("Box", &three_inputs, &one_output),
        computation_node_height(3, 1, false, false),
    );
    let more_outputs = DagNodeSpec::computation(
        "more-out".into(),
        "Box",
        "Box",
        "emoji:📦️".into(),
        one_input.clone(),
        three_outputs.clone(),
        false,
        false,
        0.0,
        0.0,
        computation_node_width("Box", &one_input, &three_outputs),
        computation_node_height(1, 3, false, false),
    );
    let grid = computation_channel_row_count(&more_inputs);
    assert_eq!(grid, 3);
    assert_eq!(computation_io_side_row_counts(&more_inputs), (3, 1));
    assert_eq!(computation_io_side_row_divider_indices(3, grid).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(computation_io_side_row_divider_indices(1, grid).collect::<Vec<_>>(), vec![1]);
    assert_eq!(computation_io_side_row_counts(&more_outputs), (1, 3));
    assert_eq!(computation_io_side_row_divider_indices(1, grid).collect::<Vec<_>>(), vec![1]);
    assert_eq!(computation_io_side_row_divider_indices(3, grid).collect::<Vec<_>>(), vec![1, 2]);
}

#[test]
fn computation_channel_row_dividers_align_with_row_bounds() {
    let inputs =
        vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "cornerB".into(), ..Default::default() }, IoPortSpec { id: "c".into(), label: "height".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "outA".into(), label: "geometry".into(), ..Default::default() }, IoPortSpec { id: "outB".into(), label: "mesh".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    let height = computation_node_height(3, 2, false, false);
    let node = DagNodeSpec::computation("box".into(), "Box", "Box", "emoji:📦️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let (input_rows, output_rows) = computation_io_side_row_counts(&node);
    assert_eq!(input_rows, 3);
    assert_eq!(output_rows, 2);
    let (input_left, input_right) = computation_input_column_x_bounds(&node).expect("input column");
    let (output_left, output_right) = computation_output_column_x_bounds(&node).expect("output column");
    assert!(input_left < input_right);
    assert!(output_left < output_right);
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let (input_span_left, input_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Input);
    let (output_span_left, output_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Output);
    assert!((input_span_left - (node.x - width * 0.5)).abs() < 1e-6);
    assert!((input_span_right - divider_x).abs() < 1e-6);
    assert!((output_span_left - divider_x).abs() < 1e-6);
    assert!((output_span_right - (node.x + width * 0.5)).abs() < 1e-6);
    assert!((divider_x - node.x).abs() < 1e-6, "2× channel width nodes split IO at center");
    let divider_y = channel_row_divider_y(node.y, node.height, 1);
    let (_, _row0_top, _, row0_bottom) = channel_row_bounds(&node, 0);
    let (_, row1_top, _, _row1_bottom) = channel_row_bounds(&node, 1);
    assert!((divider_y - row0_bottom).abs() < 1e-6);
    assert!((divider_y - row1_top).abs() < 1e-6);
}

#[test]
fn computation_node_size_fits_io_labels() {
    let inputs = vec![
        IoPortSpec { id: "width".into(), label: "width".into(), ..Default::default() },
        IoPortSpec { id: "depth".into(), label: "depth".into(), ..Default::default() },
        IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
    ];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let width = computation_node_width("brep.prim3d.box", &inputs, &outputs);
    let height = computation_node_height(3, 1, false, false);
    assert!(height <= 42.0, "expected compact height, got {height}");
    assert!(height < 96.0, "expected shorter than legacy 4-row layout");
    assert_eq!(width, DAG_COMPONENT_WIDTH);
}

#[test]
fn io_node_rect_port_angles_on_edges() {
    use super::graph::handle_position_on_rectangle;
    use canvas::Point;
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("node", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, false);
    let hw = width * 0.5;
    let left = io_node_rect_port_angle(0.0, 0.0, width, height, 0, 2, true);
    let right = io_node_rect_port_angle(0.0, 0.0, width, height, 0, 1, false);
    let left_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), width, height, left);
    let right_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), width, height, right);
    assert!(left_pos.x < -hw + 1.0);
    assert!(right_pos.x > hw - 1.0);
    assert!(left_pos.y < 0.0);
    assert!(right_pos.y < 0.0);
}

#[test]
fn computation_port_handle_caps_bulge_outward() {
    use super::graph::{NodeShape, handle_exterior_cap_fill_path, handle_outward_at_node_rim, handle_position_on_rectangle};
    use canvas::Point;
    let inputs = vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }, IoPortSpec { id: "1".into(), label: "1".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "dictionary".into(), ..Default::default() }];
    let width = computation_node_width("Merge", &inputs, &outputs);
    let height = computation_node_height(2, 1, true, false);
    let center = Point::new(100.0, 50.0);
    let left_angle = io_node_rect_port_angle(center.x, center.y, width, height, 0, 2, true);
    let right_angle = io_node_rect_port_angle(center.x, center.y, width, height, 0, 1, false);
    let left_pos = handle_position_on_rectangle(center, width, height, left_angle);
    let right_pos = handle_position_on_rectangle(center, width, height, right_angle);
    let left_out = handle_outward_at_node_rim(left_pos, center, NodeShape::Rectangle, 0.0, width, height).expect("left outward");
    let right_out = handle_outward_at_node_rim(right_pos, center, NodeShape::Rectangle, 0.0, width, height).expect("right outward");
    assert!((left_out.x + 1.0).abs() < 1e-9 && left_out.y.abs() < 1e-9);
    assert!((right_out.x - 1.0).abs() < 1e-9 && right_out.y.abs() < 1e-9);
    let left_cap = handle_exterior_cap_fill_path(left_pos, left_out, DAG_HANDLE_WORLD_RADIUS);
    let right_cap = handle_exterior_cap_fill_path(right_pos, right_out, DAG_HANDLE_WORLD_RADIUS);
    assert!(left_cap.bounding_box().x0() < left_pos.x - 1.0, "left input cap must bulge outside the west edge");
    assert!(right_cap.bounding_box().x1() > right_pos.x + 1.0, "right output cap must bulge outside the east edge");
}

#[test]
fn dag_draw_lod_maps_zoom_to_puzzle2d_bands() {
    assert_eq!(dag_draw_lod(0.1), DagDrawLod::Minimap);
    assert_eq!(dag_draw_lod(0.45), DagDrawLod::Overview);
    assert_eq!(dag_draw_lod(0.65), DagDrawLod::Compact);
    assert_eq!(dag_draw_lod(1.2), DagDrawLod::Normal);
    assert_eq!(dag_draw_lod(2.5), DagDrawLod::Detail);
    assert_eq!(dag_draw_lod(3.0), DagDrawLod::Micro);
    assert_eq!(dag_draw_lod(5.0), DagDrawLod::Micro);
}

#[test]
fn dag_draw_lod_progressive_disclosure_gates() {
    assert_eq!(DagDrawLod::Normal.node_label(), DagNodeLabel::Name);
    assert_eq!(DagDrawLod::Detail.node_label(), DagNodeLabel::Abbreviation);
    assert!(DagDrawLod::Normal.shows_computation_layout());
    assert!(!DagDrawLod::Compact.shows_computation_layout());
    assert!(!DagDrawLod::Normal.shows_handles());
    assert!(DagDrawLod::Detail.shows_handles());
    assert!(DagDrawLod::Normal.uses_input_row_connection_hitbox());
    assert!(!DagDrawLod::Detail.uses_input_row_connection_hitbox());
    assert!(DagDrawLod::Detail.uses_channel_row_pick());
    assert!(DagDrawLod::Micro.uses_channel_row_pick());
    assert!(!DagDrawLod::Normal.uses_channel_row_pick());
    assert!(!DagDrawLod::Minimap.allows_connection_hit_picking());
    assert!(!DagDrawLod::Overview.allows_connection_hit_picking());
    assert!(!DagDrawLod::Compact.allows_connection_hit_picking());
    assert!(DagDrawLod::Normal.allows_connection_hit_picking());
    assert!(DagDrawLod::Detail.allows_connection_hit_picking());
    assert!(DagDrawLod::Micro.allows_connection_hit_picking());
    assert!(DagDrawLod::Normal.shows_port_labels());
    assert!(DagDrawLod::Detail.shows_port_labels());
    assert!(DagDrawLod::Micro.shows_port_labels());
    assert_eq!(DagDrawLod::Minimap.edge_stroke_screen_px(), DAG_EDGE_STROKE_MINIMAP_SCREEN_PX);
    assert_eq!(DagDrawLod::Normal.edge_stroke_screen_px(), DAG_EDGE_STROKE_SCREEN_PX);
}

#[test]
fn wheel_zoom_pins_draw_lod_until_gesture_ends() {
    let mut host = DagHost::default_demo();
    host.fixture.camera.zoom = 1.2;
    assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Normal);
    host.set_wheel_zoom_active(true);
    host.fixture.camera.zoom = 0.45;
    assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Normal);
    host.set_wheel_zoom_active(false);
    assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Overview);
}

#[test]
fn pointer_pan_gesture_moves_camera() {
    let mut host = DagHost::default_demo();
    host.set_camera(0.0, 0.0, 2.0);
    host.pointer_down_screen(100.0, 100.0, 1, false, false, false, true);
    host.pointer_move_screen(150.0, 100.0, false, false, false);
    assert!((host.fixture.camera.x - -25.0).abs() < 1e-6);
}

#[test]
fn minimap_widget_label_overlay_includes_rect_when_visible() {
    let mut host = DagHost::default_demo();
    host.set_minimap_widget_visible(true);
    host.set_viewport(1280, 800, 1.0);
    host.set_camera(500.0, 400.0, 3.0);
    let raw: Value = dsl::os_pack::json::parse(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let minimap = raw.get("minimapWidget").and_then(|v| v.as_object()).expect("minimap widget json");
    assert!(minimap.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) > 0.0);
}

#[test]
fn minimap_widget_click_repositions_camera() {
    let mut host = DagHost::default_demo();
    host.set_minimap_widget_visible(true);
    host.set_viewport(1280, 800, 1.0);
    host.set_camera(500.0, 400.0, 3.0);
    let raw: Value = dsl::os_pack::json::parse(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let minimap = raw.get("minimapWidget").expect("minimap");
    let x = minimap["x"].as_f64().unwrap() + minimap["width"].as_f64().unwrap() * 0.5;
    let y = minimap["y"].as_f64().unwrap() + minimap["height"].as_f64().unwrap() * 0.5;
    let before_x = host.fixture.camera.x;
    host.pointer_down_screen(x, y, 0, false, false, false, false);
    host.pointer_up_screen(x, y, false, false, false);
    assert!((host.fixture.camera.x - before_x).abs() > 1.0);
}

#[test]
fn minimap_widget_paints_without_panic() {
    let mut host = DagHost::default_demo();
    host.set_minimap_widget_visible(true);
    host.set_viewport(1280, 800, 1.0);
    host.set_camera(120.0, 80.0, 0.75);
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
}

#[test]
fn dag_handle_world_radius_is_zoom_invariant() {
    assert_eq!(DAG_HANDLE_WORLD_RADIUS, 5.0);
}

#[test]
fn dag_label_paint_px_scales_with_zoom_inside_lod_band() {
    assert_eq!(dag_label_layout_px(), DAG_LABEL_SCREEN_PX);
    let normal = 3usize;
    let normal_floor = dag_lod_band_floor_zoom(normal);
    assert!((dag_label_paint_px(normal_floor, normal) - DAG_LABEL_SCREEN_PX).abs() < 1e-9);
    assert!((dag_label_paint_px(1.1, normal) - DAG_LABEL_SCREEN_PX * 1.1 / normal_floor).abs() < 1e-9);
    assert!((dag_label_paint_px(normal_floor, normal) - dag_label_paint_px(1.1, normal) * normal_floor / 1.1).abs() < 1e-9);
    assert!((dag_label_compact_paint_px(1.1, normal) - DAG_LABEL_COMPACT_SCREEN_PX * 1.1 / normal_floor).abs() < 1e-9);
}

#[test]
fn dag_paint_scene_keeps_labels_when_lod_forced_at_low_zoom() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("compact");
    host.fixture.camera.zoom = 0.25;
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    assert!(scene.path_count() > 12, "compact LOD at low zoom should still paint abbreviation labels");
}

#[test]
fn dag_label_colors_use_theme_label_fields() {
    use canvas::Color;
    let theme = CanvasPalette { label_fill: Color::from_rgba8(240, 241, 245, 255), label_halo: Color::from_rgba8(10, 12, 16, 180), node_stroke: Color::from_rgba8(90, 100, 110, 255), ..CanvasPalette::default() };
    assert_ne!(theme.label_fill.to_rgba8(), theme.node_stroke.to_rgba8());
}

#[test]
fn dag_node_paint_fill_matches_puzzle2d_lod_chrome() {
    use canvas::Color;
    let theme = CanvasPalette {
        node_fill: Color::from_rgba8(10, 20, 30, 255),
        node_stroke: Color::from_rgba8(200, 210, 220, 255),
        node_fill_hovered: Color::from_rgba8(40, 50, 60, 255),
        node_stroke_hovered: Color::from_rgba8(90, 100, 110, 255),
        node_fill_selected: Color::from_rgba8(70, 80, 90, 255),
        node_stroke_selected: Color::from_rgba8(120, 130, 140, 255),
        node_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
        node_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
        ..CanvasPalette::default()
    };
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, false).expect("minimap neutral").to_rgba8(), theme.node_stroke.to_rgba8());
    assert!(dag_node_paint_fill(DagDrawLod::Overview, &theme, false, false, false, false).is_none());
    assert!(dag_node_paint_fill(DagDrawLod::Normal, &theme, false, false, false, false).is_none());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Normal, &theme, false, true, false, false).expect("selected").to_rgba8(), theme.node_fill_selected.to_rgba8());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true).expect("minimap hovered").to_rgba8(), theme.node_stroke_hovered.to_rgba8());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, true, false).expect("minimap highlighted").to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
    assert_eq!(dag_node_body_stroke(&theme, false, true, false, true).to_rgba8(), theme.node_stroke_selected.to_rgba8());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, true, false, true).expect("minimap selected").to_rgba8(), theme.node_stroke_selected.to_rgba8());
    assert_ne!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true).expect("minimap hovered").to_rgba8(), theme.node_fill_hovered.to_rgba8());
    assert_eq!(dag_node_body_stroke(&theme, false, false, false, true).to_rgba8(), theme.node_stroke_hovered.to_rgba8());
    assert_eq!(dag_node_body_stroke(&theme, false, false, true, false).to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, false, false, true).to_rgba8(), theme.label_fill_hovered.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, false, true, false).to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, false, false, false).to_rgba8(), theme.label_fill.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, true, false, false).to_rgba8(), theme.label_fill_hovered.to_rgba8());
    let body = dag_node_body_stroke(&theme, false, false, false, false);
    let label = dag_node_label_fill(&theme, false, false, false, true);
    assert_eq!(dag_node_internal_chrome_stroke(body, label, true).to_rgba8(), label.to_rgba8());
    assert_eq!(dag_node_internal_chrome_stroke(body, label, false).to_rgba8(), body.to_rgba8());
    let body_selected = dag_node_body_stroke(&theme, false, true, false, false);
    let label_selected = dag_node_label_fill(&theme, false, true, false, false);
    assert_eq!(dag_node_internal_chrome_stroke(body_selected, label_selected, true).to_rgba8(), label_selected.to_rgba8());
    assert_eq!(dag_node_internal_chrome_stroke(body_selected, label_selected, false).to_rgba8(), body_selected.to_rgba8());
}

#[test]
fn dag_handle_and_edge_stroke_use_theme_defaults() {
    use canvas::Color;
    let theme = CanvasPalette {
        edge_stroke: Color::from_rgba8(100, 110, 120, 255),
        edge_stroke_hovered: Color::from_rgba8(10, 20, 30, 255),
        edge_stroke_selected: Color::from_rgba8(40, 50, 60, 255),
        handle_stroke: Color::from_rgba8(130, 140, 150, 255),
        handle_stroke_hovered: Color::from_rgba8(20, 30, 40, 255),
        handle_stroke_selected: Color::from_rgba8(50, 60, 70, 255),
        ..CanvasPalette::default()
    };
    assert_eq!(dag_edge_body_stroke(&theme, false, false, false, false).to_rgba8(), theme.edge_stroke.to_rgba8());
    assert_eq!(dag_handle_body_stroke(&theme, false, false, false, false).to_rgba8(), theme.handle_stroke.to_rgba8());
}

#[test]
fn manual_lod_pins_draw_tier_until_automatic_restored() {
    let mut host = DagHost::default_demo();
    host.fixture.camera.zoom = 1.0;
    assert_eq!(host.draw_lod_label(), "normal");
    host.set_automatic_lod(false);
    host.set_forced_draw_lod_label("minimap");
    assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Minimap);
    host.fixture.camera.zoom = 5.0;
    assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Minimap);
    host.set_automatic_lod(true);
    assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Micro);
}

#[test]
fn note_widget_size_uses_uniform_component_width() {
    let short = note_widget_size("hi");
    let long = note_widget_size("some longer note text");
    assert_eq!(short.0, DAG_COMPONENT_WIDTH);
    assert_eq!(long.0, DAG_COMPONENT_WIDTH);
    assert_eq!(short.1, DAG_CHANNEL_ROW_HEIGHT);
    assert_eq!(long.1, DAG_CHANNEL_ROW_HEIGHT);
}

#[test]
fn fit_note_sizes_keeps_slider_height() {
    let mut host = DagHost::from_fixture(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            abbreviation: "Note".into(),
            icon: "emoji:📝️".into(),
            x: 0.0,
            y: 0.0,
            width: 80.0,
            height: 80.0,
            kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.fit_note_sizes();
    assert_eq!(host.fixture.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
    let DagNodeKind::Note { text, .. } = &mut host.fixture.nodes[0].kind else {
        panic!("expected note");
    };
    *text = "a much longer note body".into();
    host.fit_note_sizes();
    assert_eq!(host.fixture.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
    assert_eq!(host.fixture.nodes[0].width, DAG_COMPONENT_WIDTH);
}

#[test]
fn truncate_label_to_fit_width_adds_ellipsis() {
    let px = 12.0;
    let max_w = 40.0;
    let truncated = truncate_label_to_fit_width("alpha beta gamma delta", max_w, px);
    assert!(truncated.ends_with('…'));
    assert!(truncated.len() < "alpha beta gamma delta".len());
}

#[test]
fn begin_note_edit_inserts_and_backspaces_text() {
    let mut host = DagHost::from_fixture(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            abbreviation: "Note".into(),
            icon: "emoji:📝️".into(),
            x: 0.0,
            y: 0.0,
            width: note_widget_size("hi").0,
            height: note_widget_size("hi").1,
            kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    let origin_x = note_text_origin_x(&host.fixture.nodes[0]);
    assert!(host.begin_note_edit("note", origin_x + 100.0, 0.0));
    assert_eq!(host.editing_note_id(), Some("note"));
    assert!(host.note_insert_text("!"));
    {
        let DagNodeKind::Note { text, .. } = &host.fixture.nodes[0].kind else {
            panic!("expected note");
        };
        assert_eq!(text, "hi!");
    }
    assert!(host.note_backspace());
    {
        let DagNodeKind::Note { text, .. } = &host.fixture.nodes[0].kind else {
            panic!("expected note");
        };
        assert_eq!(text, "hi");
    }
    host.note_commit_edit();
    assert_eq!(host.editing_note_id(), None);
}

#[test]
fn note_label_overlay_skips_title_and_ports() {
    let mut host = DagHost::from_fixture(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            abbreviation: "Note".into(),
            icon: "emoji:📝️".into(),
            x: 0.0,
            y: 0.0,
            width: note_widget_size("hello").0,
            height: note_widget_size("hello").1,
            kind: DagNodeKind::Note { text: "hello".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_viewport(800, 600, 1.0);
    let raw: Value = dsl::os_pack::json::parse(&host.label_overlay_paint_state_json().unwrap()).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    assert!(labels.iter().all(|row| row["text"] != "Note" && row["text"] != "out"));
}

#[test]
fn note_preview_action_port_accessors() {
    let note = DagNodeSpec {
        id: "note".into(),
        name: "Note".into(),
        abbreviation: "Note".into(),
        icon: "emoji:📝️".into(),
        x: 0.0,
        y: 0.0,
        width: note_widget_size("hi").0,
        height: note_widget_size("hi").1,
        kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
        ..Default::default()
    };
    assert!(note.inputs().is_empty());
    assert_eq!(note.outputs().len(), 1);
    let preview = DagNodeSpec {
        id: "preview".into(),
        name: "Preview".into(),
        abbreviation: "Preview".into(),
        icon: "emoji:👁️".into(),
        x: 0.0,
        y: 0.0,
        width: 120.0,
        height: 48.0,
        kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: "3".into() }, expanded: BTreeSet::new(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
        ..Default::default()
    };
    assert_eq!(preview.inputs().len(), 1);
    assert!(preview.outputs().is_empty());
    let export = DagNodeSpec {
        id: "export".into(),
        name: "Export SVG".into(),
        abbreviation: "SVG".into(),
        icon: "emoji:📤️".into(),
        x: 0.0,
        y: 0.0,
        width: 120.0,
        height: 48.0,
        kind: DagNodeKind::Export { label: "SVG".into(), format: "svg".into(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
        ..Default::default()
    };
    assert_eq!(export.inputs().len(), 1);
    assert!(export.outputs().is_empty());
}

#[test]
fn to_pascal_case_normalizes_spaced_labels() {
    assert_eq!(to_pascal_case("pass through"), "PassThrough");
    assert_eq!(to_pascal_case("Draw Rectangle"), "DrawRectangle");
}

#[test]
fn dag_draw_lod_node_content_matrix() {
    assert!(!DagDrawLod::Minimap.node_icon_visible());
    assert_eq!(DagDrawLod::Minimap.node_label(), DagNodeLabel::None);
    assert!(DagDrawLod::Overview.node_icon_visible());
    assert_eq!(DagDrawLod::Overview.node_label(), DagNodeLabel::None);
    assert!(!DagDrawLod::Compact.node_icon_visible());
    assert_eq!(DagDrawLod::Compact.node_label(), DagNodeLabel::Abbreviation);
    assert!(!DagDrawLod::Normal.node_icon_visible());
    assert_eq!(DagDrawLod::Normal.node_label(), DagNodeLabel::Name);
    assert!(!DagDrawLod::Detail.node_icon_visible());
    assert_eq!(DagDrawLod::Detail.node_label(), DagNodeLabel::Abbreviation);
    assert!(!DagDrawLod::Micro.node_icon_visible());
    assert_eq!(DagDrawLod::Micro.node_label(), DagNodeLabel::Name);
    let computation = DagNodeSpec::computation(
        "add".into(),
        "Add",
        "Add",
        "emoji:➕️".into(),
        vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }],
        vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
        false,
        false,
        0.0,
        0.0,
        120.0,
        56.0,
    );
    assert!(DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Overview));
    assert!(!DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Detail));
    assert!(!DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Micro));
}

#[test]
fn dag_node_spec_round_trips_display_fields() {
    let node = DagNodeSpec::computation("n".into(), "pass through", "pass", "emoji:➡️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 0.0, 0.0, 80.0, 24.0);
    let json = dsl::os_pack::json::to_json_string(&node);
    let back: DagNodeSpec = dsl::os_pack::json::from_json_str(&json).unwrap();
    assert_eq!(back.name, "PassThrough");
    assert_eq!(back.abbreviation, "Pass");
    assert_eq!(back.icon, "emoji:➡️");
}

#[test]
fn preview_tree_toggle_expands_and_resizes() {
    let json = dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::object([("alpha".to_string(), dsl::os_pack::json::object([("beta".to_string(), Value::from(1))])), ("gamma".to_string(), Value::from("x"))]));
    let mut host = DagHost::from_fixture(DagFixture {
        schema: "dag.fixture".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "preview".into(),
            name: "Preview".into(),
            abbreviation: "Preview".into(),
            icon: "emoji:👁️".into(),
            x: 0.0,
            y: 0.0,
            width: 80.0,
            height: 80.0,
            kind: DagNodeKind::Preview { content: DagPreviewContent::Tree { json: json.clone() }, expanded: BTreeSet::new(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.set_viewport(800, 600, 1.0);
    let collapsed_h = host.fixture.nodes[0].height;
    let layouts = preview_tree_row_layouts(&host.fixture.nodes[0], &json, &BTreeSet::new());
    let row = layouts.iter().find(|entry| entry.path == "alpha").expect("alpha row");
    let (x0, y0, x1, y1) = row.row_rect;
    let world_x = x0 + (x1 - x0) * 0.75;
    let world_y = (y0 + y1) * 0.5;
    use canvas::Point;
    use canvas::camera::{Camera as CanvasCamera, Viewport, world_to_screen};
    let cam = CanvasCamera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let screen = world_to_screen(&cam, &viewport, Point::new(world_x, world_y));
    host.pointer_down(screen.x, screen.y, false);
    let expanded_h = host.fixture.nodes[0].height;
    assert!(expanded_h > collapsed_h);
    let DagNodeKind::Preview { expanded, .. } = &host.fixture.nodes[0].kind else {
        panic!("preview kind");
    };
    assert!(expanded.contains("alpha"));
}

#[test]
fn dag_paint_scene_smoke_at_overview_and_micro_zoom() {
    let mut host = DagHost::default_demo();
    host.set_viewport(1280, 800, 1.0);
    let mut scene = canvas::Scene::new();
    host.fixture.camera.zoom = 0.3;
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    host.fixture.camera.zoom = 5.0;
    host.paint_scene(&mut scene, 1280, 800, 1.0);
}

#[test]
fn cluster_node_round_trips_serde() {
    let inputs = vec![IoPortSpec::simple("a", "a")];
    let outputs = vec![IoPortSpec::simple("out", "out")];
    let node = DagNodeSpec::cluster("cluster".into(), "Cluster", "Cluster", "emoji:🧩️".into(), inputs, outputs, 10.0, 20.0, 120.0, 80.0);
    let json = dsl::os_pack::json::to_json_string(&node);
    let back: DagNodeSpec = dsl::os_pack::json::from_json_str(&json).unwrap();
    assert!(matches!(back.kind, DagNodeKind::Cluster { .. }));
}

#[test]
fn cluster_explode_hit_rect_detects_top_right_affordance() {
    let inputs = vec![IoPortSpec::simple("a", "a")];
    let outputs = vec![IoPortSpec::simple("out", "out")];
    let node = DagNodeSpec::cluster("cluster".into(), "Cluster", "Cluster", "emoji:🧩️".into(), inputs, outputs, 0.0, 0.0, 120.0, 80.0);
    let (x0, y0, x1, y1) = cluster_explode_hit_rect(&node).expect("rect");
    assert!(cluster_explode_hit(&node, (x0 + x1) * 0.5, (y0 + y1) * 0.5));
    assert!(!cluster_explode_hit(&node, node.x - 50.0, node.y - 50.0));
}
