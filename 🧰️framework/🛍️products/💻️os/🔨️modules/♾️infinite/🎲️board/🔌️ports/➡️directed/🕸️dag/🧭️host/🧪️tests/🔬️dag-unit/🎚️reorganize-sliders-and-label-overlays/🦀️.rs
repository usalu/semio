/// 🎚️ Reorganize, sliders and label overlays: reorganize, the demo fixture, slider drags and select clicks and the label overlay paint state JSON laws.

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
