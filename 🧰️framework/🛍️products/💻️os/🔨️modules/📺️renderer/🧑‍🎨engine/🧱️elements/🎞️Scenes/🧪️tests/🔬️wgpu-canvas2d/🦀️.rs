use super::*;

#[test]
fn removed_scene_returns_its_credit_only_after_nested_retirement_and_preserves_the_successor() {
    let mut owner = crate::interpreter::fixture_scene_pointer_target("nested-close-window", "nested-close-surface", "old-mount");
    owner.kind = SurfaceKind::Canvas2d;
    assert!(mount_scene_identity(&owner));
    SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let state = states.get_mut(&owner.surface_id).unwrap();
        state.ink_overrides.insert("retiring-block".into(), json!({"children": [{"text": "retained nested payload"}, [1, 2, 3]]}));
        state.selected_ids.insert("selected".into());
        state.vfs_expanded_ids.insert("folder".into());
        state.scroll_offsets.insert("column".into(), 20.0);
        state.map_marquee_points.push((1.0, 2.0));
    });
    assert!(retire_scene_identity(&owner));
    assert_eq!(SCENE_STATE.with(|cell| cell.borrow().external_reservations), 1);
    let mut successor = owner.clone();
    successor.component_generation += 1;
    assert!(!mount_scene_identity(&successor));
    assert!(close_retired_scene_surface_one());
    assert_eq!(SCENE_STATE.with(|cell| cell.borrow().external_reservations), 1);
    for _ in 0..262_144 {
        if !close_retired_scene_surface_one() {
            break;
        }
    }
    assert!(SCENE_SURFACE_RETIREMENT.with(|cell| cell.borrow().is_none()));
    assert_eq!(SCENE_STATE.with(|cell| cell.borrow().external_reservations), 0);
    assert!(mount_scene_identity(&successor));
    assert!(!retire_scene_identity(&owner));
    assert_eq!(SCENE_STATE.with(|cell| cell.borrow().get(&successor.surface_id).and_then(|state| state.mount_owner.clone())), Some(successor.clone()));
    assert!(retire_scene_identity(&successor));
    for _ in 0..262_144 {
        if !close_retired_scene_surface_one() {
            break;
        }
    }
    assert!(!SCENE_STATE.with(|cell| cell.borrow().contains_key(&owner.surface_id)));
}

#[test]
fn a_second_component_retirement_yields_before_detaching_its_live_owner() {
    let mut first = crate::interpreter::fixture_scene_pointer_target("retirement-window", "retirement-document-a", "scene-a");
    first.kind = SurfaceKind::Canvas2d;
    first.host_id = "retirement-host-a".into();
    let mut second = crate::interpreter::fixture_scene_pointer_target("retirement-window", "retirement-document-b", "scene-b");
    second.kind = SurfaceKind::Canvas2d;
    second.host_id = "retirement-host-b".into();
    assert!(mount_scene_identity(&first));
    assert!(mount_scene_identity(&second));
    assert!(retire_scene_identity(&first));
    assert!(!retire_scene_identity(&second));
    assert_eq!(SCENE_STATE.with(|cell| cell.borrow().get(&second.host_id).and_then(|state| state.mount_owner.clone())), Some(second.clone()));
    for _ in 0..262_144 {
        if !close_retired_scene_surface_one() {
            break;
        }
    }
    assert!(retire_scene_identity(&second));
    for _ in 0..262_144 {
        if !close_retired_scene_surface_one() {
            break;
        }
    }
}

fn canvas_scene(surface_id: &str, layers_json: String) -> UiComponentSceneNode {
    UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::Canvas2d,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: Some(ui_wgpu::wgpu::Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() }),
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    }
}

fn canvas_camera_gesture_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🧭️canvas2d-camera-gestures/🔣️.json")).expect("shared Canvas2d camera gesture fixture")
}

fn fixture_camera(value: &Value) -> Viewport {
    Viewport { x: value["x"].as_f64().unwrap() as f32, y: value["y"].as_f64().unwrap() as f32, zoom: value["zoom"].as_f64().unwrap() as f32 }
}

fn seed_catalogue_hover(scene: &UiComponentSceneNode) {
    CANVAS_CATALOGUE_HOVER.with(|cell| {
        *cell.borrow_mut() = Some(CanvasCatalogueHover {
            window_id: "canvas-catalogue-terminal-window".into(),
            document_generation: 7,
            host_id: scene.host_id.clone(),
            surface_id: scene.surface_id.clone(),
            controller_id: scene.controller_id.clone(),
            last_x: 0.0,
            last_y: 0.0,
            last_dispatch_ms: 0.0,
            cancellation_requested: false,
        });
    });
}

fn catalogue_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    crate::collect_fixture_actions(input)
}

#[test]
fn catalogue_drop_publishes_one_terminal_leave_drop_slice_and_preserves_raw_payload() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🛒️canvas-catalogue-terminal/🔣️.json")).expect("shared catalogue terminal fixture");
    let mut scene = canvas_scene(fixture["surface"]["id"].as_str().unwrap(), "[]".into());
    scene.controller_id = fixture["surface"]["controllerId"].as_str().unwrap().into();
    let inner = Rect::new(0.0, 0.0, fixture["surface"]["width"].as_f64().unwrap() as f32, fixture["surface"]["height"].as_f64().unwrap() as f32);

    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["terminal"] != "nonCatalogue") {
        seed_catalogue_hover(&scene);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let point = &row["point"];
        let accepted = canvas_catalogue_drop_into(&scene, inner, "canvas-catalogue-terminal-window", 7, point["x"].as_f64().unwrap() as f32, point["y"].as_f64().unwrap() as f32, row["rawPayload"].as_str().unwrap(), &mut input)
            .expect("terminal action batch admits");
        assert_eq!(accepted, row["terminal"] != "empty");
        let actions = catalogue_actions(&mut input);
        assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), row["expectedActions"].as_array().unwrap().iter().map(|action| action.as_str().unwrap()).collect::<Vec<_>>());
        if let Some(drop) = actions.iter().find(|action| action.action == "canvasDrop") {
            let args = Value::from(drop.args.as_ref().unwrap());
            assert_eq!(args["dragData"], row["rawPayload"], "malformed catalogue text stays byte-for-byte owned by the application decoder");
            assert_eq!(args["x"].as_f64(), point["x"].as_f64());
            assert_eq!(args["y"].as_f64(), point["y"].as_f64());
        }
        assert!(CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().is_none()), "a published terminal slice clears the hover owner");
    }
}

#[test]
fn catalogue_drop_refusal_keeps_hover_and_never_publishes_a_partial_pair() {
    let scene = canvas_scene("canvas-catalogue-refusal", "[]".into());
    let inner = Rect::new(0.0, 0.0, 320.0, 200.0);
    seed_catalogue_hover(&scene);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for index in 0..ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY - 1 {
        let action = format!("occupied-{index}");
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&["fixture", action.as_str()]).unwrap();
        input.reserve_action("fixture", &action, bytes).unwrap().publish().unwrap();
    }
    assert_eq!(canvas_catalogue_drop_into(&scene, inner, "canvas-catalogue-terminal-window", 7, 73.0, 41.0, "{\"kind\":\"rect\"}", &mut input), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits),);
    assert!(CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().is_some()), "refusal retains the terminal hover owner for a bounded retry");
    let first = input.take_action_step().unwrap().unwrap().into_descriptor().unwrap();
    assert_eq!(first.action, "occupied-0", "refusal leaves the pre-existing FIFO unchanged and publishes neither terminal member");
    assert!(canvas_catalogue_drop_into(&scene, inner, "canvas-catalogue-terminal-window", 7, 73.0, 41.0, "{\"kind\":\"rect\"}", &mut input).unwrap());
    let actions = catalogue_actions(&mut input);
    assert_eq!(actions[actions.len() - 2..].iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasDragLeave", "canvasDrop"]);
    assert!(CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().is_none()));
}

#[test]
fn foreign_drop_is_inert_and_does_not_consume_the_catalogue_hover_owner() {
    let scene = canvas_scene("canvas-catalogue-foreign", "[]".into());
    seed_catalogue_hover(&scene);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let drag_data = std::collections::HashMap::from([("text/plain".to_string(), "foreign".to_string())]);
    assert!(!crate::interpreter::canvas_catalogue_drop("unpublished-window", 10.0, 10.0, &drag_data, &mut input));
    assert!(catalogue_actions(&mut input).is_empty());
    assert!(CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().is_some()));
    CANVAS_CATALOGUE_HOVER.with(|cell| *cell.borrow_mut() = None);
}

/// 🖊️ A shape-selection ring should be tinted amber (matching `drawBoundsLayer`'s hardcoded
/// `"rgba(251, 191, 36, ...)"` literals in `canvas-2d-host.tsx`), not `theme.accent` (the app's
/// red/crimson design token) — see `CANVAS2D_SELECTION_RING`/`CANVAS2D_SELECTION_GLOW`.
#[test]
fn selected_shape_draws_the_amber_ring_and_glow_not_theme_accent() {
    let layers = json!([{ "kind": "rectangle", "id": "r1", "x": 10.0, "y": 10.0, "width": 40.0, "height": 20.0, "selected": true }]);
    let node = canvas_scene("s1", layers.to_string());
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_canvas_2d(&node, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
    }
    let vertex_colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).map(|v| v.color).collect();
    let ring = CANVAS2D_SELECTION_RING;
    let glow = CANVAS2D_SELECTION_GLOW;
    assert!(vertex_colors.contains(&[ring.r, ring.g, ring.b, ring.a]), "expected the crisp amber ring color among vertices, got {vertex_colors:?}");
    assert!(vertex_colors.contains(&[glow.r, glow.g, glow.b, glow.a]), "expected the soft amber glow color among vertices, got {vertex_colors:?}");
    assert!(!vertex_colors.contains(&[theme.accent.r, theme.accent.g, theme.accent.b, theme.accent.a]), "the selection ring must no longer use theme.accent (the app's red/crimson token), got {vertex_colors:?}");
}

/// 🫙️ An empty layer list paints `"Empty canvas"`, the literal `canvas-2d-host.tsx`'s `renderFrame`
/// fills when `layers.length === 0` — the grid/checkerboard chrome pushes no glyphs, so the glyph
/// instance count IS the label.
#[test]
fn empty_layer_list_paints_the_react_empty_canvas_label() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_canvas_2d(&canvas_scene("empty-canvas-label", "[]".to_string()), Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
    }
    let glyphs = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.params[2] == ui_wgpu::wgpu::draw_types::KIND_GLYPH).count();
    assert_eq!(glyphs, CANVAS_2D_EMPTY_LABEL.chars().count(), "an empty canvas paints exactly the label React paints, got {glyphs} glyphs");
}

/// 🕒️ `scene_camera_action`'s exact key names — `{surfaceId, camera: {x, y, zoom}}`, matching
/// `ink_set_camera_action`'s own shape (this crate's other camera-from-viewport builder).
#[test]
fn scene_camera_action_uses_surface_id_and_nested_camera_xyz_keys() {
    let action = scene_camera_action("s-cam-1", "controller-1", Viewport { x: 12.5, y: -3.0, zoom: 2.0 });
    assert_eq!(action.controller_id, "controller-1");
    assert_eq!(action.action, "setCamera");
    let args = action.args.expect("setCamera always carries args");
    assert_eq!(args.get("surfaceId").and_then(semio_framework::DslValue::as_str), Some("s-cam-1"));
    assert_eq!(args.get("camera").and_then(|value| value.get("x")).and_then(semio_framework::DslValue::as_f64), Some(12.5));
    assert_eq!(args.get("camera").and_then(|value| value.get("y")).and_then(semio_framework::DslValue::as_f64), Some(-3.0));
    assert_eq!(args.get("camera").and_then(|value| value.get("zoom")).and_then(semio_framework::DslValue::as_f64), Some(2.0));
}

/// 🕒️ A Canvas2d wheel tick mutates the viewport immediately but only SCHEDULES its `setCamera` —
/// nothing is due yet, and nothing is dispatched, until the deadline sweep says otherwise.
#[test]
fn canvas2d_wheel_schedules_a_settled_camera_dispatch_without_firing_immediately() {
    let surface_id = "wheel-settle-canvas2d";
    let node = canvas_scene(surface_id, "[]".to_string());
    let actions = handle_scene_wheel(&node, Rect::new(0.0, 0.0, 400.0, 300.0), 50.0, 50.0, -100.0, false);
    assert!(actions.is_empty(), "wheel-zoom never dispatches inline anymore");
    let immediate = sweep_expired_scene_camera_dispatches(crate::app_now_ms());
    assert!(immediate.iter().all(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) != Some(surface_id)), "sweeping immediately must not yet report this surface");
    let due = sweep_expired_scene_camera_dispatches(crate::app_now_ms() + canvas_camera_gesture_fixture()["wheel"]["settleDelayMs"].as_f64().unwrap());
    let matched = due.iter().find(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) == Some(surface_id)).expect("this surface's setCamera fires once its deadline has passed");
    assert_eq!(matched.controller_id, "controller");
    assert_eq!(matched.action, "setCamera");
}

#[test]
fn canvas2d_wheel_uses_the_react_factors_limits_and_exact_cursor_anchor() {
    let fixture = canvas_camera_gesture_fixture();
    let wheel = &fixture["wheel"];
    let surface = &fixture["surface"];
    let surface_id = surface["id"].as_str().unwrap();
    let node = canvas_scene(surface_id, "[]".to_string());
    let bounds = Rect::new(0.0, 0.0, surface["width"].as_f64().unwrap() as f32, surface["height"].as_f64().unwrap() as f32);
    let anchor_x = wheel["anchor"]["x"].as_f64().unwrap() as f32;
    let anchor_y = wheel["anchor"]["y"].as_f64().unwrap() as f32;
    let initial = fixture_camera(&wheel["initialCamera"]);
    let world_x = (anchor_x - bounds.w * 0.5) / initial.zoom + initial.x;
    let world_y = (anchor_y - bounds.h * 0.5) / initial.zoom + initial.y;
    mutate_scene_state(surface_id, |state| state.viewport = initial);
    assert!(canvas_wheel_into(&node, bounds, anchor_x, anchor_y, wheel["zoomInDeltaY"].as_f64().unwrap() as f32));
    let zoomed_in = scene_state(surface_id).viewport;
    assert_eq!(zoomed_in.zoom, initial.zoom * wheel["zoomInFactor"].as_f64().unwrap() as f32);
    assert_eq!((anchor_x - bounds.w * 0.5) / zoomed_in.zoom + zoomed_in.x, world_x, "wheel zoom keeps the exact world x under the off-centre cursor");
    assert_eq!((anchor_y - bounds.h * 0.5) / zoomed_in.zoom + zoomed_in.y, world_y, "wheel zoom keeps the exact world y under the off-centre cursor");

    mutate_scene_state(surface_id, |state| state.viewport = initial);
    assert!(canvas_wheel_into(&node, bounds, anchor_x, anchor_y, wheel["zoomOutDeltaY"].as_f64().unwrap() as f32));
    assert_eq!(scene_state(surface_id).viewport.zoom, initial.zoom * wheel["zoomOutFactor"].as_f64().unwrap() as f32);
    let minimum = wheel["zoomMinimum"].as_f64().unwrap() as f32;
    mutate_scene_state(surface_id, |state| state.viewport = Viewport { zoom: minimum, ..initial });
    assert!(canvas_wheel_into(&node, bounds, anchor_x, anchor_y, wheel["zoomOutDeltaY"].as_f64().unwrap() as f32));
    assert_eq!(scene_state(surface_id).viewport.zoom, minimum);
    let maximum = wheel["zoomMaximum"].as_f64().unwrap() as f32;
    mutate_scene_state(surface_id, |state| state.viewport = Viewport { zoom: maximum, ..initial });
    assert!(canvas_wheel_into(&node, bounds, anchor_x, anchor_y, wheel["zoomInDeltaY"].as_f64().unwrap() as f32));
    assert_eq!(scene_state(surface_id).viewport.zoom, maximum);
}

fn assert_canvas_pan_case(case_id: &str) {
    let fixture = canvas_camera_gesture_fixture();
    let surface = &fixture["surface"];
    let row = fixture["panCases"].as_array().unwrap().iter().find(|row| row["id"] == case_id).unwrap();
    let surface_id = format!("{}-{case_id}", surface["id"].as_str().unwrap());
    let layers = json!([{ "role": "meta", "utility": row["activeUtility"] }]);
    let node = canvas_scene(&surface_id, layers.to_string());
    let bounds = Rect::new(0.0, 0.0, surface["width"].as_f64().unwrap() as f32, surface["height"].as_f64().unwrap() as f32);
    let initial = fixture_camera(&fixture["wheel"]["initialCamera"]);
    mutate_scene_state(&surface_id, |state| state.viewport = initial);
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().clear());
    CANVAS_GESTURE.with(|cell| *cell.borrow_mut() = CanvasGestureSlots::default());
    let start_x = row["start"]["x"].as_f64().unwrap() as f32;
    let start_y = row["start"]["y"].as_f64().unwrap() as f32;
    let end_x = row["end"]["x"].as_f64().unwrap() as f32;
    let end_y = row["end"]["y"].as_f64().unwrap() as f32;
    let button = row["button"].as_i64().unwrap() as i16;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(canvas_pointer_button_into(&node, bounds, ui_render::PointerId(1), "canvas2d-camera-window", 1, start_x, start_y, true, button, SceneModifiers::default(), &mut input).unwrap());
    assert!(canvas_pointer_move_into(&node, bounds, ui_render::PointerId(1), "canvas2d-camera-window", 1, end_x, end_y, &mut input).unwrap());
    assert!(canvas_pointer_button_into(&node, bounds, ui_render::PointerId(1), "canvas2d-camera-window", 1, end_x, end_y, false, button, SceneModifiers::default(), &mut input).unwrap());
    let actions = catalogue_actions(&mut input);
    assert_eq!(
        actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(),
        row["expectedActions"].as_array().unwrap().iter().map(|action| action.as_str().unwrap()).collect::<Vec<_>>(),
        "{case_id} publishes exactly React's document-action sequence"
    );
    let expected = fixture_camera(&row["expectedCamera"]);
    let actual = scene_state(&surface_id).viewport;
    assert_eq!((actual.x, actual.y, actual.zoom), (expected.x, expected.y, expected.zoom), "{case_id} owns React's exact camera");
    let camera_actions = sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
    assert_eq!(
        camera_actions.iter().any(|action| action.action == fixture["wheel"]["expectedAction"].as_str().unwrap() && action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) == Some(surface_id.as_str())),
        row["expectedCameraAction"].as_bool().unwrap()
    );
}

#[test]
fn canvas2d_primary_drag_with_the_active_pan_utility_changes_only_the_camera() {
    assert_canvas_pan_case("primary-active-pan");
}

#[test]
fn canvas2d_middle_drag_changes_only_the_camera() {
    assert_canvas_pan_case("middle-button-pan");
}

#[test]
fn canvas2d_right_drag_remains_a_document_gesture_and_does_not_pan() {
    assert_canvas_pan_case("right-button-gesture");
}

#[test]
fn canvas2d_middle_pan_cancellation_retires_locally_without_a_document_action() {
    let node = canvas_scene("canvas2d-middle-pan-cancel", "[]".to_string());
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let pointer = ui_render::PointerId(17);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    CANVAS_GESTURE.with(|cell| *cell.borrow_mut() = CanvasGestureSlots::default());

    assert!(canvas_pointer_button_into(&node, bounds, pointer, "canvas2d-middle-pan-window", 1, 40.0, 50.0, true, 1, SceneModifiers::default(), &mut input).unwrap());
    assert!(canvas_state_snapshot(&node.surface_id).1);
    assert!(cancel_canvas_pointer_gesture_for(pointer, &mut input));
    assert!(catalogue_actions(&mut input).is_empty(), "React cancels a local camera pan without a document pointer action");
    assert!(!canvas_state_snapshot(&node.surface_id).1);
    assert!(CANVAS_GESTURE.with(|cell| cell.borrow().get(pointer).is_none()));
}

#[test]
fn foreign_pointer_cancel_preserves_the_canvas_gesture_owner() {
    let node = canvas_scene("canvas2d-pointer-owner", "[]".to_string());
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let owner = ui_render::PointerId(77);
    let foreign = ui_render::PointerId(78);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    CANVAS_GESTURE.with(|cell| *cell.borrow_mut() = CanvasGestureSlots::default());

    assert!(canvas_pointer_button_into(&node, bounds, owner, "canvas2d-owner-window", 1, 40.0, 50.0, true, 0, SceneModifiers::default(), &mut input).unwrap());
    assert!(!cancel_canvas_pointer_gesture_for(foreign, &mut input));
    assert!(CANVAS_GESTURE.with(|cell| cell.borrow().get(owner).is_some()));
    assert!(canvas_pointer_move_into(&node, bounds, owner, "canvas2d-owner-window", 1, 80.0, 90.0, &mut input).unwrap());
    assert!(canvas_pointer_button_into(&node, bounds, owner, "canvas2d-owner-window", 1, 80.0, 90.0, false, 0, SceneModifiers::default(), &mut input).unwrap());
    assert_eq!(catalogue_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp"]);
}

#[test]
fn two_canvas_documents_keep_independent_pointer_gestures() {
    let first = canvas_scene("canvas2d-pointer-first", "[]".to_string());
    let second = canvas_scene("canvas2d-pointer-second", "[]".to_string());
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let first_pointer = ui_render::PointerId(77);
    let second_pointer = ui_render::PointerId(78);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    CANVAS_GESTURE.with(|cell| *cell.borrow_mut() = CanvasGestureSlots::default());

    assert!(canvas_pointer_button_into(&first, bounds, first_pointer, "canvas2d-first-window", 1, 40.0, 50.0, true, 0, SceneModifiers::default(), &mut input).unwrap());
    assert!(canvas_pointer_button_into(&second, bounds, second_pointer, "canvas2d-second-window", 1, 140.0, 150.0, true, 0, SceneModifiers::default(), &mut input).unwrap());
    assert!(cancel_canvas_pointer_gesture_for(second_pointer, &mut input));
    assert!(CANVAS_GESTURE.with(|cell| cell.borrow().get(first_pointer).is_some()));
    assert!(CANVAS_GESTURE.with(|cell| cell.borrow().get(second_pointer).is_none()));
    assert!(canvas_pointer_move_into(&first, bounds, first_pointer, "canvas2d-first-window", 1, 80.0, 90.0, &mut input).unwrap());
    assert!(canvas_pointer_button_into(&first, bounds, first_pointer, "canvas2d-first-window", 1, 80.0, 90.0, false, 0, SceneModifiers::default(), &mut input).unwrap());

    let actions = catalogue_actions(&mut input);
    let addressed = actions
        .iter()
        .map(|action| {
            let args = Value::from(action.args.as_ref().unwrap());
            (args["surfaceId"].as_str().unwrap().to_string(), action.action.clone(), args["cancelled"].as_bool().unwrap_or(false))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        addressed,
        [
            ("canvas2d-pointer-first".into(), "canvasPointerDown".into(), false),
            ("canvas2d-pointer-second".into(), "canvasPointerDown".into(), false),
            ("canvas2d-pointer-second".into(), "canvasPointerUp".into(), true),
            ("canvas2d-pointer-first".into(), "canvasPointerMove".into(), false),
            ("canvas2d-pointer-first".into(), "canvasPointerUp".into(), false),
        ]
    );
}

/// 🕒️ A Canvas2d pan-drag (`SceneDragMode::PanViewport`) gets the identical settle-then-dispatch
/// treatment as wheel-zoom — same deadline map, same sweep.
///
/// 🧹️ The settle map is process-wide and the sweep drains EVERY surface's deadline, so this law
/// clears it first; otherwise a neighbouring scene's armed camera is counted here
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
#[test]
fn canvas2d_pan_drag_schedules_a_settled_camera_dispatch() {
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().clear());
    let surface_id = "pan-settle-canvas2d";
    let node = canvas_scene(surface_id, "[]".to_string());
    mutate_scene_state(surface_id, |state| {
        state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport });
    });
    let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
    let actions = handle_scene_pointer_move(&node, bounds, 60.0, 60.0, true, 1, 5.0, 5.0);
    assert!(
        actions.iter().all(|action| action.action != "setCamera"),
        "the pan itself never dispatches setCamera inline, even though Canvas2d's own \
         canvasPointerMove tracking action still fires alongside it"
    );
    let due = sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), Some(surface_id));
}

#[test]
fn scene_camera_deadlines_saturate_before_ownership_and_close_cursor_restores_one_per_step() {
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().clear());
    SCENE_CAMERA_DISPATCH_FAULT.with(|cell| *cell.borrow_mut() = None);
    for index in 0..SCENE_CAMERA_DISPATCH_CAPACITY {
        schedule_scene_camera_dispatch(&format!("surface-{index}"), &format!("surface-{index}"));
    }
    schedule_scene_camera_dispatch("overflow", "overflow");
    let mut cursor = SceneCameraDispatchCursor::begin(crate::app_now_ms());
    assert!(matches!(cursor.step(), SceneCameraDispatchStep::Fault("scene camera deadline credits exceeded")));
    for remaining in (0..SCENE_CAMERA_DISPATCH_CAPACITY).rev() {
        assert!(!cursor.close_step());
        assert_eq!(cursor.entries.len(), remaining);
    }
    assert!(cursor.close_step());
    assert!(cursor.terminal_is_empty());
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| assert_eq!(cell.borrow().len(), SCENE_CAMERA_DISPATCH_CAPACITY));
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow_mut().clear());
}
