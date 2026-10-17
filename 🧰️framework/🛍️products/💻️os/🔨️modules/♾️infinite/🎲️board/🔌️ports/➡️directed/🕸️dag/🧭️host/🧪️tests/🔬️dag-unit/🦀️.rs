//! 🧪️ dag-host tests (🔬️dag-unit): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::{DagChannelDirection, DagChannelRef, DagLayoutOptions, DagSelectionDomains, DagSelectionJsonCursor, DagSelectionTextFault, DagSelectionTextGrant, DagSelectionTextStep, LayoutControl};
use semio_framework_os_infinite_board_engine::{Camera, GRID_WORLD_MEDIUM, InteractionMode};
use semio_framework_os_infinite_board_dag_model::{channel_row_bounds, computation_column_divider_x, computation_node_height, computation_node_width, DAG_COMPONENT_WIDTH, DAG_INTERACTION_NODE_CAPACITY, DagInteractionPlanFault, DagPortSide, input_port_row_hit_bounds, note_text_origin_x, note_widget_size, output_port_row_hit_bounds, preview_tree_row_layouts, select_control_bounds, slider_track_bounds, slider_widget_height, slider_widget_width, variadic_input_insert_positions, variadic_output_insert_positions};
use semio_framework_os_infinite_board_dag_graph::{dag_draw_lod_label, dag_node_overlays_json, dag_set_wheel_zoom_active, draw_lod_for_frame, entity_screen_json, focus_selection_camera, hovered_node_id, selection_union_bounds_screen_json, set_hover_channel, slider_overlay_state_json};
use semio_framework_os_infinite_board_dag_host::{begin_note_edit, DagHost, dimmed_node_ids, minimap_widget_layout, note_backspace, note_insert_text, set_dimmed};
use graph::HandleRole;
use semio_framework_pack_json::{JsonMemberPolicy, Value};
use semio_framework_canvas::{Point, Scene};
use semio_framework_canvas as canvas;
use semio_framework_os_infinite_directed_presentation::*;
use semio_framework_test_fixtures::fixture_text;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

fn cursor_grant() -> DagSelectionTextGrant {
    DagSelectionTextGrant { fuel: 1, now_milliseconds: 1, deadline_milliseconds: 8, cancelled: false, interrupted: false }
}

#[test]
fn selected_nodes_cursor_censuses_and_emits_one_byte_per_grant() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🎯️selected-json.json")).unwrap();
    for case in vectors["cases"].as_array().unwrap() {
        let ids: Vec<String> = case["ids"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().into()).collect();
        let fixture = DagHostSnapshot { schema: "dag.hostDocument".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: ids.iter().map(|id|DagNodeSpec { id: id.clone(), ..Default::default() }).collect(), edges: vec![] };
        let mut host = DagHost::from_host_snapshot_without_layout(fixture);
        host.graph.set_selection(&ids);
        assert_eq!(host.graph.selected_node_ids(), ids);
        let expected = semio_framework_pack_json::to_json_string(&host.graph.selected_node_ids()).into_bytes();
        assert_eq!(expected, serde_json::to_vec(&host.graph.selected_node_ids()).unwrap());
        let mut cursor = DagSelectionJsonCursor::default();
        let mut rejected = cursor_grant();
        rejected.fuel = 0;
        assert_eq!(cursor.step(&host, rejected), Err(DagSelectionTextFault::NoFuel));
        let mut output = Vec::new();
        let mut census = None;
        loop {
            match cursor.step(&host, cursor_grant()).unwrap() {
                DagSelectionTextStep::Census { bytes } => census = Some(bytes),
                DagSelectionTextStep::Byte(byte) => output.push(byte),
                DagSelectionTextStep::Complete => break,
                DagSelectionTextStep::Progress { .. } => {}
            }
        }
        assert_eq!(census, Some(expected.len()));
        assert_eq!(output, expected);
        assert_eq!(cursor.step(&host, cursor_grant()), Ok(DagSelectionTextStep::Complete));
    }
}

#[test]
fn selected_edges_cursor_matches_direct_encode() {
    let mut host = DagHost::default_demo();
    let edge = host.graph.host_snapshot.edges.first().expect("demo edge").id.clone();
    host.graph.set_selection_domains(&DagSelectionDomains { edges: vec![edge], ..Default::default() });
    let expected = semio_framework_pack_json::to_json_string(&host.graph.selected_edge_ids()).into_bytes();
    let mut cursor = DagSelectionJsonCursor::edges();
    let mut output = Vec::new();
    let mut census = None;
    loop {
        match cursor.step(&host, cursor_grant()).unwrap() {
            DagSelectionTextStep::Census { bytes } => census = Some(bytes),
            DagSelectionTextStep::Byte(byte) => output.push(byte),
            DagSelectionTextStep::Complete => break,
            DagSelectionTextStep::Progress { .. } => {}
        }
    }
    assert_eq!(census, Some(expected.len()));
    assert_eq!(output, expected);
}

#[test]
fn bounded_interaction_projection_rejects_node_and_identifier_overflow() {
    let mut fixture = DagHostSnapshot::default();
    fixture.nodes = (0..=DAG_INTERACTION_NODE_CAPACITY).map(|index| DagNodeSpec { id: format!("node-{index}"), ..Default::default() }).collect();
    let host = DagHost::from_host_snapshot_without_layout(fixture);
    assert_eq!(host.graph.bounded_interaction_projection(0).unwrap_err(), DagInteractionPlanFault::NodeCredits);

    let mut fixture = DagHostSnapshot::default();
    fixture.nodes = vec![DagNodeSpec { id: "x".repeat(16 * 1024 + 1), ..Default::default() }];
    let host = DagHost::from_host_snapshot_without_layout(fixture);
    assert_eq!(host.graph.bounded_interaction_projection(0).unwrap_err(), DagInteractionPlanFault::StringCredits);
}

#[test]
fn dag_selection_hover_and_dimmed_map_widget_ids() {
    let fixture = DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    let mut host = DagHost::from_host_snapshot(fixture);
    host.graph.set_selection(&["a".to_string()]);
    assert_eq!(host.graph.selected_node_ids(), vec!["a"]);
    host.graph.set_hover(Some("b"));
    assert_eq!(hovered_node_id(&host.viewport, &host.graph).as_deref(), Some("b"));
    set_dimmed(&mut host.graph, &mut host.paint, &["a".to_string()]);
    assert_eq!(dimmed_node_ids(&host.graph, &host.paint), vec!["a"]);
}

#[test]
fn handle_hover_does_not_hover_parent_node() {
    let fixture = DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    let mut host = DagHost::from_host_snapshot(fixture);
    host.viewport.set_viewport(800, 600, 1.0);
    let (handle_center, node_id) = {
        let snap = host.graph.engine.render_snapshot();
        let (hid, center, _) = snap.handles.first().expect("input handle");
        let node_id = host.graph.engine.handles.get(hid).expect("handle").node_id;
        (*center, node_id)
    };
    let (sx, sy) = world_to_screen_px(&host, handle_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    let hover = host.graph.engine.hover.expect("handle hover");
    assert!(host.graph.engine.handles.contains_key(&hover));
    assert!(hovered_node_id(&host.viewport, &host.graph).is_none());
    assert!(!host.graph.is_node_hovered(node_id));
    host.pointer_move_screen(0.0, 0.0, false, false, false);
    assert!(host.graph.engine.hover.is_none());
}

#[test]
fn idle_pointer_move_updates_hover() {
    use canvas::Point;
    use canvas::camera::{Camera, Viewport, world_to_screen};

    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let camera = Camera { x: host.graph.host_snapshot.camera.x, y: host.graph.host_snapshot.camera.y, zoom: host.graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let slider = world_to_screen(&camera, &viewport, Point::new(-400.0, -40.0));
    host.pointer_move_screen(slider.x, slider.y, false, false, false);
    assert_eq!(hovered_node_id(&host.viewport, &host.graph).as_deref(), Some("slider"));
    host.pointer_move_screen(8.0, 8.0, false, false, false);
    assert!(hovered_node_id(&host.viewport, &host.graph).is_none());
}

#[test]
fn dag_host_delete_selected_preserves_remaining_positions() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
        edges: vec![DagHostSnapshotEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagHostSnapshotEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
    });
    host.graph.set_selection(&["b".to_string()]);
    host.graph.delete_selected();
    let a = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "a").expect("a");
    let c = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "c").expect("c");
    assert!((a.x - 100.0).abs() < 0.01);
    assert!((a.y - 200.0).abs() < 0.01);
    assert!((c.x - 700.0).abs() < 0.01);
    assert!((c.y - 300.0).abs() < 0.01);
    assert!(host.graph.host_snapshot.nodes.iter().all(|n| n.id != "b"));
}

#[test]
fn dag_host_delete_selected_removes_edge_only_selection() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
        edges: vec![DagHostSnapshotEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagHostSnapshotEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
    });
    let edge_id = *host.graph.engine.edges.keys().next().expect("edge");
    host.graph.engine.selection.edge_ids.insert(edge_id);
    assert!(host.graph.has_selection());
    assert_eq!(host.graph.host_snapshot.edges.len(), 2);
    host.graph.delete_selected();
    assert_eq!(host.graph.host_snapshot.edges.len(), 1);
    assert_eq!(host.graph.host_snapshot.nodes.len(), 3);
    assert!(!host.graph.has_selection());
}

#[test]
fn dag_host_reorganize_updates_engine_positions() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 500.0, 500.0, 160.0, 56.0),
            DagNodeSpec::computation("b".into(), "B", "B", "emoji:🔷️".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 500.0, 500.0, 160.0, 56.0),
        ],
        edges: vec![DagHostSnapshotEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }],
    });
    let mut progress=|_|true;let mut control=LayoutControl::new(100_000_000,&mut progress);host.graph.reorganize(&DagLayoutOptions::default(),&mut control).unwrap();
    let a = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "a").expect("a");
    let b = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "b").expect("b");
    assert!(b.x > a.x);
}

#[test]
fn dag_host_loads_demo_fixture() {
    let host = DagHost::default_demo();
    assert_eq!(host.graph.host_snapshot.schema, "dag.hostDocument");
    assert_eq!(host.graph.host_snapshot.nodes.len(), 5);
    assert_eq!(host.graph.host_snapshot.edges.len(), 4);
    assert!(!host.graph.engine.render_snapshot().edges.is_empty());
}

#[test]
fn dag_host_slider_drag_mutates_value() {
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_viewport(800, 600, 1.0);
    let (x0, y0, x1, y1) = slider_track_bounds(&host.graph.host_snapshot.nodes[0]);
    let mid_y = (y0 + y1) * 0.5;
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new((x0 + x1) * 0.5, mid_y));
    host.pointer_down(sx, sy, false);
    host.pointer_up(sx, sy);
    let DagNodeKind::Slider { value, .. } = host.graph.host_snapshot.nodes[0].kind else {
        panic!("expected slider");
    };
    assert!((value - 2.0).abs() > 0.1);
}

#[test]
fn dag_host_slider_drag_ignored_when_controls_hidden() {
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("minimap");
    host.viewport.set_viewport(800, 600, 1.0);
    let (x0, y0, x1, y1) = slider_track_bounds(&host.graph.host_snapshot.nodes[0]);
    let mid_y = (y0 + y1) * 0.5;
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new((x0 + x1) * 0.5, mid_y));
    host.pointer_down(sx, sy, false);
    host.pointer_up(sx, sy);
    let DagNodeKind::Slider { value, .. } = host.graph.host_snapshot.nodes[0].kind else {
        panic!("expected slider");
    };
    assert!((value - 2.0).abs() < 1e-6, "minimap LOD should only move the node rectangle, not adjust the value");
}

#[test]
fn dag_host_select_click_advances_option() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_viewport(800, 600, 1.0);
    let (x0, y0, x1, y1) = select_control_bounds(&host.graph.host_snapshot.nodes[0]);
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
    host.pointer_down(sx, sy, false);
    let DagNodeKind::Select { selected, .. } = host.graph.host_snapshot.nodes[0].kind else {
        panic!("expected select");
    };
    assert_eq!(selected, 1);
}

#[test]
fn dag_host_label_overlay_paint_state_json_includes_compact_labels() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("compact");
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    assert!(!labels.is_empty());
    assert!(labels.iter().all(|row| row["layout"] == "horizontal"));
    assert!(!labels[0]["text"].as_str().unwrap_or("").is_empty());
}

#[test]
fn dag_host_label_overlay_paint_state_json_includes_slider_name() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("micro");
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    assert!(labels.iter().any(|row| row["text"] == "Radius" && row["layout"] == "horizontal"));
}

#[test]
fn dag_host_slider_overlay_preserves_language_neutral_field_labels() {
    let fixture: Value = semio_framework_pack_json::parse(fixture_text!("../../../🧪️tests/🧫️fixtures/🎚️slider-overlay.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let row = &case["row"];
        let host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
            schema: "dag.hostDocument".into(),
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
        let json = slider_overlay_state_json(&host.viewport, &host.graph).unwrap();
        let actual: Value = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let independent: serde_json::Value = serde_json::from_str(&json).unwrap();
        for key in ["widgetId", "label"] {
            assert_eq!(actual["sliders"][0][key].as_str(), row[key].as_str(), "{key}");
            assert_eq!(independent["sliders"][0][key].as_str(), row[key].as_str(), "independent {key}");
        }
        for key in ["value", "min", "max", "step"] {
            assert_eq!(actual["sliders"][0][key].as_f64(), row[key].as_f64(), "{key}");
            assert_eq!(independent["sliders"][0][key].as_f64(), row[key].as_f64(), "independent {key}");
        }
    }
}

#[test]
fn dag_host_slider_overlay_state_json_includes_slider_track() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_viewport(1280, 800, 1.0);
    let raw: Value = semio_framework_pack_json::parse(&slider_overlay_state_json(&host.viewport, &host.graph).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
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
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    let port_rows: Vec<_> = labels.iter().filter(|row| row["kind"].as_str() == Some("port")).map(|row| (row["text"].as_str().unwrap_or(""), row["align"].as_str().unwrap_or(""))).collect();
    assert_eq!(port_rows.len(), 3);
    assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! a").count(), 1);
    assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! b").count(), 1);
    assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! merged").count(), 1);
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
        let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot { schema: "dag.hostDocument".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 }, nodes: vec![node.clone()], edges: vec![] });
        host.viewport.set_viewport(1280, 800, 1.0);
        host.viewport.set_automatic_lod(false);
        host.viewport.set_forced_draw_lod_label(lod);
        let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
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
    let host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_viewport(1280, 800, 1.0);
    let json = dag_node_overlays_json(&host.viewport, &host.graph).unwrap();
    let overlays: Vec<Value> = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap().as_array().cloned().unwrap_or_default();
    assert_eq!(overlays.len(), 1);
    assert_eq!(overlays[0]["id"], "screen");
    assert_eq!(overlays[0]["mediaKind"], "svg");
    assert!(overlays[0]["rect"]["w"].as_f64().unwrap_or(0.0) > 10.0);
}

fn handle_world(host: &DagHost, port_key: &str) -> canvas::Point {
    let hid = host.graph.handle_key_map.iter().find(|(_, key)| key.as_str() == port_key).map(|(id, _)| *id).expect("handle");
    host.graph.engine.render_snapshot().handles.iter().find(|(id, _, _)| *id == hid).map(|(_, p, _)| *p).expect("handle pos")
}

fn world_to_screen_px(host: &DagHost, p: canvas::Point) -> (f64, f64) {
    (p.x + host.viewport.width as f64 * 0.5, p.y + host.viewport.height as f64 * 0.5)
}

#[test]
fn dag_host_area_select_previews_preselect_before_commit() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let start_sx = 24.0;
    let start_sy = 24.0;
    let end_sx = 1100.0;
    let end_sy = 700.0;
    host.pointer_down_screen(start_sx, start_sy, 0, false, false, false, false);
    host.pointer_move_screen(end_sx, end_sy, false, false, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::AreaSelect { .. }), "expected area-select after marquee threshold");
    let preselect = host.graph.preselect_widget_ids();
    assert!(!preselect.is_empty(), "marquee drag should preview widget ids before commit");
    let preview_points: Vec<[f64; 2]> = semio_framework_pack_json::from_json_str(&host.graph.selection_preview_points_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(preview_points.len() >= 2, "marquee overlay points should be published during drag");
    host.pointer_up_screen(end_sx, end_sy, false, false, false);
    assert!(!host.graph.selected_node_ids().is_empty(), "marquee drag should commit selection on release");
    assert!(host.graph.preselect_widget_ids().is_empty(), "preselect should clear after commit");
}

#[test]
fn dag_host_align_selection_horizontal_and_vertical_center() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.graph.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
    host.graph.align_selection("alignHorizontal").unwrap();
    let xs: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.x).collect();
    assert!(xs.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
    host.graph.align_selection("alignVertical").unwrap();
    let ys: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.y).collect();
    assert!(ys.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
}

#[test]
fn dag_host_align_selection_left_and_distribute_horizontal() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.graph.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
    host.graph.align_selection("alignLeft").unwrap();
    let left_edges: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.x - node.width * 0.5).collect();
    assert!(left_edges.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
    host.graph.align_selection("distributeHorizontal").unwrap();
    let mut xs: Vec<f64> = host.graph.selected_snapshot_nodes().into_iter().map(|(_, node)| node.x).collect();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(xs.windows(2).all(|pair| pair[1] > pair[0]));
}

#[test]
fn dag_host_selection_union_bounds_screen_json_nonempty_for_selection() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.graph.set_selection(&["scale".into(), "combine".into()]);
    let json = selection_union_bounds_screen_json(&host.viewport, &host.graph);
    assert_ne!(json, "null");
    let parsed: Value = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(parsed["width"].as_f64().unwrap_or(0.0) > 1.0);
    assert!(parsed["height"].as_f64().unwrap_or(0.0) > 1.0);
}

#[test]
fn dag_host_entity_screen_json_resolves_node_by_id_and_wildcard() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let json = entity_screen_json(&host.viewport, &host.graph, "node", "scale");
    let parsed: Value = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(parsed["visible"], true);
    assert!(parsed["x"].as_number().is_some());
    assert!(parsed["rect"].as_array().is_some());

    let wildcard: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "node", "*"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(wildcard["visible"], true);
}

/// 🐢️ A malformed id (no "@port") or a port that doesn't exist on the node must degrade to
/// unresolved, never panic.
#[test]
fn dag_host_entity_screen_json_resolves_handle_by_widget_and_port() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let input_json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "scale@in"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(input_json["visible"], true);
    let output_json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "combine@b"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(output_json["visible"], true);
    let malformed: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "scale"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(malformed["visible"], false);
    let missing_port: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "handle", "scale@nope"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(missing_port["visible"], false);
}

#[test]
fn dag_host_entity_screen_json_resolves_edge_with_a_two_point_polyline() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, "edge", "e1"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(json["visible"], true);
    let polyline = json["polyline"].as_array().expect("edge geometry carries a polyline");
    assert_eq!(polyline.len(), 2);
}

/// 🛟️ "handle":"*" may legitimately resolve to the demo fixture's first port.
#[test]
fn dag_host_entity_screen_json_unresolved_domain_or_id_never_panics() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    for (domain, id) in [("node", "nonexistent"), ("handle", "*"), ("edge", "nonexistent"), ("bogus-domain", "*")] {
        let json: Value = semio_framework_pack_json::parse(&entity_screen_json(&host.viewport, &host.graph, domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        if json["visible"] == true {
            continue;
        }
        assert_eq!(json["visible"], false, "domain={domain} id={id}");
    }
    let empty_fixture = DagHostSnapshot { schema: "dag.hostDocument".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] };
    let empty: Value = semio_framework_pack_json::parse(&{ let host = DagHost::from_host_snapshot(empty_fixture); entity_screen_json(&host.viewport, &host.graph, "node", "*") }, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(empty["visible"], false);
}

#[test]
fn dag_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("minimap");
    host.graph.set_camera(0.0, 0.0, 0.1);
    host.graph.set_selection(&["scale".into(), "combine".into()]);
    let scale_before = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "scale").expect("scale").clone();
    let combine_before = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let gap = canvas::Point::new(0.0, 0.0);
    use canvas::camera::{Camera as CanvasCamera, Viewport, world_to_screen};
    let cam = CanvasCamera { x: host.graph.host_snapshot.camera.x, y: host.graph.host_snapshot.camera.y, zoom: host.graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let start = world_to_screen(&cam, &viewport, gap);
    host.pointer_down_screen(start.x, start.y, 0, false, false, false, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNodes { .. }), "expected bounded drag inside selection union at minimap LOD");
    host.pointer_move_screen(start.x + 50.0, start.y + 30.0, false, false, false);
    host.pointer_up_screen(start.x + 50.0, start.y + 30.0, false, false, false);
    let zoom = host.graph.host_snapshot.camera.zoom;
    let dx = 50.0 / zoom;
    let dy = 30.0 / zoom;
    let scale_after = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "scale").expect("scale");
    let combine_after = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine");
    assert!((scale_after.x - (scale_before.x + dx)).abs() < 1e-3 && (scale_after.y - (scale_before.y + dy)).abs() < 1e-3);
    assert!((combine_after.x - (combine_before.x + dx)).abs() < 1e-3 && (combine_after.y - (combine_before.y + dy)).abs() < 1e-3);
}

#[test]
fn dag_host_drags_node_in_world_space() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let mut dragged = false;
    for (nid, node) in host.graph.engine.nodes.clone() {
        let grab = canvas::Point::new(node.center.x - node.width * 0.4, node.center.y);
        let (sx, sy) = world_to_screen_px(&host, grab);
        host.pointer_down(sx, sy, false);
        if !matches!(host.graph.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
            host.pointer_up(sx, sy);
            continue;
        }
        let before = host.graph.engine.nodes.get(&nid).expect("node").center;
        host.pointer_move(sx + 40.0, sy + 30.0);
        let idx = *host.graph.node_id_map.get(&nid).expect("fixture index");
        let fixture = &host.graph.host_snapshot.nodes[idx];
        let engine = host.graph.engine.nodes.get(&nid).expect("node").center;
        assert!((fixture.x - engine.x).abs() < 1e-6, "fixture x should track engine during drag");
        assert!((fixture.y - engine.y).abs() < 1e-6, "fixture y should track engine during drag");
        host.pointer_up(sx + 40.0, sy + 30.0);
        let after = host.graph.engine.nodes.get(&nid).expect("node").center;
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
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    host.viewport.set_grid_snap_enabled(true);
    host.viewport.set_grid_factor(10.0).expect("grid factor");
    let mut dragged = false;
    for (nid, node) in host.graph.engine.nodes.clone() {
        let grab = canvas::Point::new(node.center.x - node.width * 0.4, node.center.y);
        let (sx, sy) = world_to_screen_px(&host, grab);
        host.pointer_down(sx, sy, false);
        if !matches!(host.graph.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
            host.pointer_up(sx, sy);
            continue;
        }
        host.pointer_move(sx + 37.0, sy + 23.0);
        host.pointer_up(sx + 37.0, sy + 23.0);
        let idx = *host.graph.node_id_map.get(&nid).expect("fixture index");
        let fixture = &host.graph.host_snapshot.nodes[idx];
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
    host.viewport.set_viewport(1280, 800, 1.0);
    let ids: Vec<String> = host.graph.host_snapshot.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    host.graph.set_selection(&ids);
    let camera = focus_selection_camera(&host.viewport, &host.graph, 1.2).expect("camera");
    assert!(camera.zoom > 0.0);
    assert!(camera.x.is_finite() && camera.y.is_finite());
}

#[test]
fn dag_host_reconnects_edge_endpoint() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let in_w = handle_world(&host, "combine@b");
    let out_w = handle_world(&host, "scale@out");
    let (in_sx, in_sy) = world_to_screen_px(&host, in_w);
    let (out_sx, out_sy) = world_to_screen_px(&host, out_w);
    host.pointer_down(in_sx, in_sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }));
    host.pointer_move(out_sx, out_sy);
    host.pointer_up(out_sx, out_sy);
    let e3 = host.graph.host_snapshot.edges.iter().find(|e| e.id == "e3").expect("e3");
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
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("src".into(), "Src", "Src", "emoji:🔢️".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
            DagNodeSpec::computation("tgt".into(), "Tgt", "Tgt", "emoji:🔢️".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
        ],
        edges: vec![],
    });
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_proximity_distance(120.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let src_center = canvas::Point::new(0.0, 0.0);
    let (sx, sy) = world_to_screen_px(&host, src_center);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.engine.render_snapshot().pending_edge.is_some(), "proximity drag should preview edge");
    host.pointer_up_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.host_snapshot.edges.iter().any(|edge| edge.source == "src@out" && edge.target == "tgt@in"), "proximity drag should commit edge");
}

#[test]
fn dag_host_node_drag_skips_wired_cut_inputs() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let cut_w = computation_node_width("Cut", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let cut_h = computation_node_height(2, 1, false, false);
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("sphere".into(), "Sphere", "Sphere", "emoji:🔵️".into(), vec![], outputs.clone(), false, false, 0.0, -60.0, src_w, src_h),
            DagNodeSpec::computation("torus".into(), "Torus", "Torus", "emoji:🍩️".into(), vec![], outputs.clone(), false, false, 0.0, 60.0, src_w, src_h),
            DagNodeSpec::computation("cut".into(), "Cut", "Cut", "emoji:✂️".into(), inputs, outputs, false, false, 240.0, 0.0, cut_w, cut_h),
        ],
        edges: vec![DagHostSnapshotEdge { id: "e1".into(), source: "sphere@out".into(), target: "cut@a".into(), ..Default::default() }, DagHostSnapshotEdge { id: "e2".into(), source: "torus@out".into(), target: "cut@b".into(), ..Default::default() }],
    });
    assert_eq!(host.graph.engine.edges.len(), 2, "fixture edges should load into engine");
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_proximity_distance(160.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let cut_center = canvas::Point::new(240.0, 0.0);
    let (sx, sy) = world_to_screen_px(&host, cut_center);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx - 180.0, sy, false, false, false);
    assert!(host.graph.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges to occupied inputs");
    host.pointer_up_screen(sx - 180.0, sy, false, false, false);
    assert_eq!(host.graph.engine.edges.len(), 2);
    assert_eq!(host.graph.host_snapshot.edges.len(), 2);
}

#[test]
fn dag_host_keeps_same_named_input_and_output_handles_distinct() {
    let solid = vec![IoPortSpec { id: "solid".into(), label: "solid".into(), ..Default::default() }];
    let brep = vec![IoPortSpec { id: "brep".into(), label: "brep".into(), ..Default::default() }];
    let list = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }];
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
            DagHostSnapshotEdge { id: "e100".into(), source: "extrude@solid".into(), target: "brep@brep".into(), ..Default::default() },
            DagHostSnapshotEdge { id: "e101".into(), source: "brep@brep".into(), target: "get@list".into(), ..Default::default() },
        ],
    });
    host.graph.sync_edges_from_engine();
    let incoming = host.graph.engine.edges.get(&100).expect("incoming brep edge");
    let outgoing = host.graph.engine.edges.get(&101).expect("outgoing brep edge");
    let incoming_target = host.graph.engine.handles.get(&incoming.target).expect("incoming target handle");
    let outgoing_source = host.graph.engine.handles.get(&outgoing.source).expect("outgoing source handle");
    assert_eq!(incoming_target.role, HandleRole::Target);
    assert_eq!(outgoing_source.role, HandleRole::Source);
    assert_eq!(host.graph.host_snapshot.edges.iter().find(|edge| edge.id == "e100").map(|edge| edge.target.as_str()), Some("brep@brep"));
    assert_eq!(host.graph.host_snapshot.edges.iter().find(|edge| edge.id == "e101").map(|edge| edge.source.as_str()), Some("brep@brep"));
}

#[test]
fn dag_host_proximity_zero_disables_node_drag_connect() {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let src_w = computation_node_width("Src", &[], &outputs);
    let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
    let src_h = computation_node_height(0, 1, false, false);
    let tgt_h = computation_node_height(1, 1, false, false);
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("src".into(), "Src", "Src", "emoji:🔢️".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
            DagNodeSpec::computation("tgt".into(), "Tgt", "Tgt", "emoji:🔢️".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
        ],
        edges: vec![],
    });
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_proximity_distance(0.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let (sx, sy) = world_to_screen_px(&host, canvas::Point::new(0.0, 0.0));
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.engine.render_snapshot().pending_edge.is_none());
    host.pointer_up_screen(sx + 200.0, sy, false, false, false);
    assert!(host.graph.host_snapshot.edges.is_empty());
}

/// 🔌️ Only the whole-graph SILHOUETTE withholds ports: every tier that draws a node draws its port
/// rows and publishes their connector rects, so a press on one wires there. The minimap tier owns the
/// bounded selection-AABB drag instead.
#[test]
fn only_the_silhouette_lod_withholds_port_hit_picking() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    let handle = handle_world(&host, "combine@b");
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.viewport.set_forced_draw_lod_label("minimap");
    assert!(!draw_lod_for_frame(&host.viewport, &host.graph).allows_connection_hit_picking(), "minimap");
    host.pointer_down(hsx, hsy, false);
    assert!(!matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }), "minimap must not start an edge draw");
    host.pointer_up(hsx, hsy);
    for lod in ["overview", "compact", "normal", "detail", "micro"] {
        host.viewport.set_forced_draw_lod_label(lod);
        assert!(draw_lod_for_frame(&host.viewport, &host.graph).allows_connection_hit_picking(), "{lod}");
        host.pointer_down(hsx, hsy, false);
        assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }), "{lod} must wire from a port");
        host.pointer_up(hsx, hsy);
    }
}

/// 🔌️ The row splits in two: its outer CONNECTOR share wires, its interior drags the node. Both the
/// anchor the cap is painted at and the rect the host publishes fall in the connector share.
#[test]
fn normal_lod_input_row_drags_node_handle_anchor_starts_edge_draw() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("normal");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine");
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "combine@b");
    assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNode { .. }), "interior rectangle drag should move the node");
    host.pointer_up(sx, sy);
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.pointer_down(hsx, hsy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }));
}

#[test]
fn set_hover_channel_targets_port_handle_at_detail_lod() {
    let mut host = DagHost::default_demo();
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    set_hover_channel(&mut host.viewport, &mut host.graph, Some("combine"), Some("b"));
    assert_eq!(host.graph.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: DagChannelDirection::In }));
    set_hover_channel(&mut host.viewport, &mut host.graph, None, None);
    assert!(host.graph.hovered_channel().is_none());
}

#[test]
fn set_hover_channel_falls_back_to_node_at_compact_lod() {
    let mut host = DagHost::default_demo();
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("compact");
    set_hover_channel(&mut host.viewport, &mut host.graph, Some("combine"), Some("b"));
    assert_eq!(hovered_node_id(&host.viewport, &host.graph).as_deref(), Some("combine"));
    assert!(host.graph.hovered_channel().is_none());
}

#[test]
fn hovered_channel_decodes_port_handle_at_detail_lod() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    assert_eq!(host.graph.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: DagChannelDirection::In }));
}

#[test]
fn detail_lod_non_channel_body_hovers_and_selects_node() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    assert!(hovered_node_id(&host.viewport, &host.graph).as_deref() == Some("combine"));
    assert!(host.graph.engine.hover.is_some());
    assert!(!host.graph.engine.selection.node_ids.contains(&host.graph.node_id_for_widget_id("combine").expect("combine node id")));
    let divider_x = computation_column_divider_x(&combine).expect("divider");
    let (_, header_top, _, header_bottom) = channel_row_bounds(&combine, 0);
    let title_probe = canvas::Point::new(divider_x, (header_top + header_bottom) * 0.5);
    let (body_sx, body_sy) = world_to_screen_px(&host, title_probe);
    host.pointer_move_screen(body_sx, body_sy, false, false, false);
    assert!(hovered_node_id(&host.viewport, &host.graph).as_deref() == Some("combine"));
    assert!(host.graph.engine.hover.is_some());
    host.pointer_down(body_sx, body_sy, false);
    assert!(host.graph.selected_node_ids().contains(&"combine".to_string()));
    host.pointer_up(body_sx, body_sy);
}

#[test]
fn visible_handle_lod_row_center_does_not_start_edge_draw() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let scale = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "scale").expect("scale");
    let port_idx = scale.outputs().iter().position(|p| p.id == "out").expect("port out");
    let (x0, y0, x1, y1) = output_port_row_hit_bounds(scale, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "scale@out");
    assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(!matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }), "visible handles require anchor hit for wire draw");
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.pointer_down(hsx, hsy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }));
}

#[test]
fn detail_lod_channel_row_drags_node_without_prior_selection() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("micro");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "combine@b");
    assert!((row_center.x - handle.x).abs() > 4.0);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNode { .. }));
    assert!(host.graph.selected_node_ids().contains(&"combine".to_string()));
}

#[test]
fn detail_lod_title_row_drags_node() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "a").expect("port a");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let title_probe = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, title_probe);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNode { .. }));
}

#[test]
fn variadic_plus_hit_maps_insert_index() {
    let inputs = vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }, IoPortSpec { id: "1".into(), label: "1".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("dictionary.merge", &inputs, &outputs);
    let height = computation_node_height(2, 1, true, false);
    let host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec::computation("merge".into(), "Merge", "Merge", "emoji:🔀️".into(), inputs, outputs, true, false, 0.0, 0.0, width, height)],
        edges: vec![],
    });
    let positions = variadic_input_insert_positions(&host.graph.host_snapshot.nodes[0]);
    assert_eq!(positions.len(), 1);
    let (_, px, py) = positions[0];
    let hit = host.graph.port_insert_hit(px, py, 2.0).expect("hit");
    assert_eq!(hit.0, DagPortSide::Input);
    assert_eq!(hit.1, "merge");
    assert_eq!(hit.2, 2);
    assert!(host.graph.port_insert_hit(px, py, 1.0).is_none());
}

#[test]
fn variadic_output_plus_hit_maps_insert_index() {
    let outputs = vec![IoPortSpec { id: "0".into(), label: "i".into(), ..Default::default() }];
    let inputs = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }, IoPortSpec { id: "index".into(), label: "index".into(), ..Default::default() }];
    let width = computation_node_width("list.get", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, true);
    let host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec::computation("get".into(), "Get", "Get", "emoji:📋️".into(), inputs, outputs, false, true, 0.0, 0.0, width, height)],
        edges: vec![],
    });
    let positions = variadic_output_insert_positions(&host.graph.host_snapshot.nodes[0]);
    assert_eq!(positions.len(), 1);
    let (_, px, py) = positions[0];
    let hit = host.graph.port_insert_hit(px, py, 2.0).expect("hit");
    assert_eq!(hit.0, DagPortSide::Output);
    assert_eq!(hit.1, "get");
    assert_eq!(hit.2, 1);
}

#[test]
fn wheel_zoom_pins_draw_lod_until_gesture_ends() {
    let mut host = DagHost::default_demo();
    host.graph.host_snapshot.camera.zoom = 1.2;
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Normal);
    dag_set_wheel_zoom_active(&mut host.viewport, &mut host.graph, true);
    host.graph.host_snapshot.camera.zoom = 0.45;
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Normal);
    dag_set_wheel_zoom_active(&mut host.viewport, &mut host.graph, false);
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Overview);
}

#[test]
fn pointer_pan_gesture_moves_camera() {
    let mut host = DagHost::default_demo();
    host.graph.set_camera(0.0, 0.0, 2.0);
    host.pointer_down_screen(100.0, 100.0, 1, false, false, false, true);
    host.pointer_move_screen(150.0, 100.0, false, false, false);
    assert!((host.graph.host_snapshot.camera.x - -25.0).abs() < 1e-6);
}

#[test]
fn minimap_widget_label_overlay_includes_rect_when_visible() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(500.0, 400.0, 3.0);
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let minimap = raw.get("minimapWidget").and_then(|v| v.as_object()).expect("minimap widget json");
    assert!(minimap.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) > 0.0);
}

#[test]
fn minimap_widget_click_repositions_camera() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(500.0, 400.0, 3.0);
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let minimap = raw.get("minimapWidget").expect("minimap");
    let x = minimap["x"].as_f64().unwrap() + minimap["width"].as_f64().unwrap() * 0.5;
    let y = minimap["y"].as_f64().unwrap() + minimap["height"].as_f64().unwrap() * 0.5;
    let before_x = host.graph.host_snapshot.camera.x;
    host.pointer_down_screen(x, y, 0, false, false, false, false);
    host.pointer_up_screen(x, y, false, false, false);
    assert!((host.graph.host_snapshot.camera.x - before_x).abs() > 1.0);
}

#[test]
fn minimap_widget_paints_without_panic() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(120.0, 80.0, 0.75);
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
}

#[test]
fn minimap_widget_panel_uses_square_corners() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(-5000.0, -5000.0, 0.75);
    assert!(!host.graph.minimap_camera_fully_shows_content(&host.graph.minimap_widget_content_bounds().unwrap(), 1280, 800));
    let layout = minimap_widget_layout(&host.graph, &host.minimap, 1280, 800).expect("minimap layout");
    let (px0, py0, px1, py1) = layout.panel;
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    let encoded: serde_json::Value =
        serde_json::from_str(&canvas::draw_list::scene_draw_list_json(&scene, canvas::draw_list::DrawListOptions::default())).expect("draw list");
    let epsilon = 0.5;
    let panel_shape = |shape: &serde_json::Value| -> bool {
        if shape[0].as_str() != Some("r") {
            return false;
        }
        let coords = shape.as_array().expect("rect coords");
        assert_eq!(coords.len(), 5, "flat rect tuple");
        let nums = coords[1..].iter().map(|v| v.as_f64().expect("coord")).collect::<Vec<_>>();
        (nums[0] - px0).abs() < epsilon && (nums[1] - py0).abs() < epsilon && (nums[2] - px1).abs() < epsilon && (nums[3] - py1).abs() < epsilon
    };
    let panel_commands = encoded["commands"]
        .as_array()
        .expect("commands")
        .iter()
        .filter(|command| {
            let shape = match command[0].as_str() {
                Some("f") => &command[4],
                Some("s") => &command[8],
                _ => return false,
            };
            panel_shape(shape)
        })
        .count();
    assert_eq!(panel_commands, 2, "minimap panel must fill and stroke as axis-aligned rects, not rounded rects");
}

/// 🏷️ A captioned tier DELEGATES its captions to the overlay (`node_caption_delegated_to_js_overlay`),
/// so "still labelled at a forced low zoom" is a property of the overlay rows, not of the scene's
/// path count — which is node chrome only, two paths per node. This used to assert `> 12` paths and
/// measured 10 whichever string the tier served (recorded in
/// `26/09/09/PROCEDURAL-3D-END-TO-END/📓️node-graph-camera-fit-labels-2026-09-12.md`).
#[test]
fn dag_paint_scene_keeps_labels_when_lod_forced_at_low_zoom() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("compact");
    host.graph.host_snapshot.camera.zoom = 0.25;
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    assert!(scene.path_count() >= host.graph.host_snapshot.nodes.len() * 2, "a forced compact LOD must still paint every node's chrome");
    let state: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let rows = state["labels"].as_array().expect("label overlay rows");
    assert_eq!(rows.len(), host.graph.host_snapshot.nodes.len(), "a forced compact LOD must caption every node");
    for (row, node) in rows.iter().zip(&host.graph.host_snapshot.nodes) {
        assert_eq!(row["text"], semio_framework_pack_json::Value::from(node.name.as_str()), "a caption is the node's NAME at every captioned tier");
    }
}

#[test]
fn manual_lod_pins_draw_tier_until_automatic_restored() {
    let mut host = DagHost::default_demo();
    host.graph.host_snapshot.camera.zoom = 1.0;
    assert_eq!(dag_draw_lod_label(&host.viewport, &host.graph), "normal");
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("minimap");
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Minimap);
    host.graph.host_snapshot.camera.zoom = 5.0;
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Minimap);
    host.viewport.set_automatic_lod(true);
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Micro);
}

#[test]
fn fit_note_sizes_keeps_slider_height() {
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.graph.fit_note_sizes();
    assert_eq!(host.graph.host_snapshot.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
    let DagNodeKind::Note { text, .. } = &mut host.graph.host_snapshot.nodes[0].kind else {
        panic!("expected note");
    };
    *text = "a much longer note body".into();
    host.graph.fit_note_sizes();
    assert_eq!(host.graph.host_snapshot.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
    assert_eq!(host.graph.host_snapshot.nodes[0].width, DAG_COMPONENT_WIDTH);
}

#[test]
fn begin_note_edit_inserts_and_backspaces_text() {
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    let origin_x = note_text_origin_x(&host.graph.host_snapshot.nodes[0]);
    assert!(begin_note_edit(&mut host.graph, &mut host.notes, "note", origin_x + 100.0, 0.0));
    assert_eq!(host.notes.editing_note_id(), Some("note"));
    assert!(note_insert_text(&mut host.graph, &mut host.notes, "!"));
    {
        let DagNodeKind::Note { text, .. } = &host.graph.host_snapshot.nodes[0].kind else {
            panic!("expected note");
        };
        assert_eq!(text, "hi!");
    }
    assert!(note_backspace(&mut host.graph, &mut host.notes));
    {
        let DagNodeKind::Note { text, .. } = &host.graph.host_snapshot.nodes[0].kind else {
            panic!("expected note");
        };
        assert_eq!(text, "hi");
    }
    host.notes.note_commit_edit();
    assert_eq!(host.notes.editing_note_id(), None);
}

#[test]
fn note_label_overlay_skips_title_and_ports() {
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
    host.viewport.set_viewport(800, 600, 1.0);
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    assert!(labels.iter().all(|row| row["text"] != "Note" && row["text"] != "out"));
}

#[test]
fn dag_draw_lod_node_content_matrix() {
    assert!(!DagDrawLod::Minimap.node_icon_visible());
    assert_eq!(DagDrawLod::Minimap.node_label(), DagNodeLabel::None);
    assert!(DagDrawLod::Overview.node_icon_visible());
    assert_eq!(DagDrawLod::Overview.node_label(), DagNodeLabel::None);
    assert!(!DagDrawLod::Compact.node_icon_visible());
    assert_eq!(DagDrawLod::Compact.node_label(), DagNodeLabel::Name);
    assert!(!DagDrawLod::Normal.node_icon_visible());
    assert_eq!(DagDrawLod::Normal.node_label(), DagNodeLabel::Name);
    assert!(!DagDrawLod::Detail.node_icon_visible());
    assert_eq!(DagDrawLod::Detail.node_label(), DagNodeLabel::Name);
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
fn preview_tree_toggle_expands_and_resizes() {
    let json = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::object([("alpha".to_string(), semio_framework_pack_json::object([("beta".to_string(), Value::from(1))])), ("gamma".to_string(), Value::from("x"))]));
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
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
            kind: DagNodeKind::Preview { content: DagPreviewContent::Tree { json: json.clone() }, expanded: DagExpandedPaths::new(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.viewport.set_viewport(800, 600, 1.0);
    let collapsed_h = host.graph.host_snapshot.nodes[0].height;
    let layouts = preview_tree_row_layouts(&host.graph.host_snapshot.nodes[0], &json, &DagExpandedPaths::new());
    let row = layouts.iter().find(|entry| entry.path == "alpha").expect("alpha row");
    let (x0, y0, x1, y1) = row.row_rect;
    let world_x = x0 + (x1 - x0) * 0.75;
    let world_y = (y0 + y1) * 0.5;
    use canvas::Point;
    use canvas::camera::{Camera as CanvasCamera, Viewport, world_to_screen};
    let cam = CanvasCamera { x: host.graph.host_snapshot.camera.x, y: host.graph.host_snapshot.camera.y, zoom: host.graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let screen = world_to_screen(&cam, &viewport, Point::new(world_x, world_y));
    host.pointer_down(screen.x, screen.y, false);
    let expanded_h = host.graph.host_snapshot.nodes[0].height;
    assert!(expanded_h > collapsed_h);
    let DagNodeKind::Preview { expanded, .. } = &host.graph.host_snapshot.nodes[0].kind else {
        panic!("preview kind");
    };
    assert!(expanded.contains("alpha"));
}

#[test]
fn dag_paint_scene_smoke_at_overview_and_micro_zoom() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let mut scene = canvas::Scene::new();
    host.graph.host_snapshot.camera.zoom = 0.3;
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    host.graph.host_snapshot.camera.zoom = 5.0;
    host.paint_scene(&mut scene, 1280, 800, 1.0);
}
