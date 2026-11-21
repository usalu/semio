//! ✒️ Unit tests of scene-ink; the work package moves the tests of its sources here.

use super::*;
use semio_ui_declaration::ui::SurfaceKind;

#[test]
fn point_eraser_matches_shared_geometry_cases() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("../../../../🧪️tests/🖊️ink-and-marquee-hosts/eraser.cases.json")).unwrap();
    for case in cases {
        let title = case["title"].as_str().unwrap();
        let block = json!({
            "id": "source", "kind": "stroke", "name": "Ink", "x": case["origin"][0], "y": case["origin"][1],
            "color": [0, 0, 0, 1], "strokeWidth": 3, "points": case["points"]
        });
        let actual = erase_ink_stroke_points_in_item(&block, case["center"][0].as_f64().unwrap(), case["center"][1].as_f64().unwrap(), case["radius"].as_f64().unwrap());
        let expected = case["fragments"].as_array().unwrap();
        assert_eq!(actual.len(), expected.len(), "{title}");
        if case["unchanged"] == true {
            assert_eq!(actual, vec![block], "{title}");
            continue;
        }
        for (index, (fragment, points)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(fragment["id"] == "source", index == 0, "{title}");
            assert_eq!(fragment["x"], block["x"], "{title}");
            assert_eq!(fragment["y"], block["y"], "{title}");
            assert_eq!(fragment["strokeWidth"], block["strokeWidth"], "{title}");
            assert_eq!(fragment["color"], block["color"], "{title}");
            let actual_points = fragment["points"].as_array().unwrap();
            let expected_points = points.as_array().unwrap();
            assert_eq!(actual_points.len(), expected_points.len(), "{title}");
            for (got, want) in actual_points.iter().zip(expected_points) {
                for axis in 0..2 {
                    assert!((got[axis].as_f64().unwrap() - want[axis].as_f64().unwrap()).abs() < 1e-8, "{title}: {got} vs {want}");
                }
            }
        }
    }
}

#[test]
fn point_eraser_publishes_nested_fragments_at_sequential_sibling_indices() {
    let stroke = |id: &str, points: Value| json!({ "id": id, "kind": "stroke", "name": "Ink", "points": points });
    let document = json!({
        "activeUtility": "eraserPoint",
        "eraserRadius": 5,
        "blocks": [{
            "id": "outer", "kind": "group", "children": [{
                "id": "inner", "kind": "group", "children": [
                    stroke("left", json!([[0, 0], [80, 0]])),
                    stroke("dot", json!([[40, 0]])),
                    stroke("right", json!([[0, 3], [80, 3]])),
                    { "id": "label", "kind": "text" }
                ]
            }]
        }]
    });
    let scene = UiComponentSceneNode {
        host_id: "ink-point-eraser-fixture".into(),
        surface_id: "ink-point-eraser-fixture".into(),
        controller_id: "ink-point-eraser-controller".into(),
        component_kind: SurfaceKind::InkCanvas,
        pane_id: None,
        binding_id: None,
        presence: Default::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: Some(semio_ui_scene_plane::InkCanvasScene {
            document_json: document.to_string(),
            selection_json: "[]".into(),
            hovered_id: None,
            active_utility: "eraserPoint".into(),
            view_mode: "edit".into(),
            interactive: true,
            interaction_domain: None,
        }),
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    };
    let pointer = InkInteractionEvent::PointerDown { x: 40.0, y: 0.0, button: 0, shift: false };
    let mut job = InkInteractionJob::new(1, 1, Some(ui_render::PointerId(7)), &scene, pointer).unwrap().unwrap();
    let mut input = semio_ui_wgpu_input::InputState::<ActionDescriptor>::default();
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let completed = (0..32).any(|_| matches!(job.step(1, &scene, bounds, &mut input).unwrap(), InkInteractionStep::Complete));
    assert!(completed);
    let action = input.take_action_step().unwrap().unwrap().into_descriptor().unwrap();
    assert_eq!(action.action, "inkApplyEvents");
    let args = Value::from(action.args.as_ref().unwrap());
    assert_eq!(args["phase"], "stream");
    let events: Vec<Value> = serde_json::from_str(args["eventsJson"].as_str().unwrap()).unwrap();
    let mut ids = vec!["left", "dot", "right", "label"];
    for event in &events {
        match event["operation"].as_str().unwrap() {
            "addBlock" => {
                assert_eq!(event["parentId"], "inner");
                ids.insert(event["index"].as_u64().unwrap() as usize, event["block"]["id"].as_str().unwrap());
            }
            "updateBlock" => assert_eq!(event["blockId"], event["block"]["id"]),
            "removeBlock" => {
                let index = ids.iter().position(|id| event["blockId"] == *id).unwrap();
                ids.remove(index);
            }
            operation => panic!("Unexpected eraser event {operation}"),
        }
    }
    assert_eq!(events.len(), 5);
    assert_eq!(ids.len(), 5);
    assert_eq!([ids[0], ids[2], ids[4]], ["left", "right", "label"]);
    assert_ne!(ids[1], ids[3]);
}
