/// 🎯️ Canvas pointer and ink scroll dispatch: the canvas2d pointer down action, the React shaped pointer payload, the ink canvas scroll action and the ink scene driving helpers.

//#region 🔖️SceneCommandTests

#[test]
fn scene_command_dispatches_a_canvas2d_pointer_down_action() {
    let window_id = "apply-ui-commands-scene-canvas2d-pointer-down";
    let node = seed_scene_window(window_id, "s1", semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &mut vec![ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 10.0, y: 10.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        }],
        Some(ui_render::PointerId(1)),
        &mut input,
    );

    let queued = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert!(queued.iter().any(|action| action.action == "canvasPointerDown"), "a real per-event PointerDown over a canvas-2d scene should reach the same handler apply_scene_pointer used to sample, got {queued:?}");
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
}

/// ⚖️ Law (packet W2j): the canvas-2d pointer wire is `📐️Canvas2dHost`'s wire. React's gesture lane
/// publishes `{ x, y, button, shift, ctrl, meta, alt, width, height }` with `x`/`y` SCREEN-LOGICAL
/// inside the canvas and `width`/`height` its logical size, because every plugin reader converts them
/// itself (`🖍️draw`'s `canvas_point_to_world(viewport, x, y, width, height)`, `📏️layout`'s
/// `hit_test_at(doc, cfg, x, y, width, height)`). wgpu used to publish world coordinates in the `x`/`y`
/// slots with no `width`/`height` at all, which converted twice against a 0×0 viewport.
#[test]
fn canvas2d_pointer_payload_is_react_shaped_screen_logical_with_a_world_lane() {
    let window_id = "apply-ui-commands-scene-canvas2d-pointer-payload";
    let node = seed_scene_window(window_id, "s1", semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d);
    let rect = Rect::new(30.0, 40.0, 200.0, 120.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &mut vec![ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d,
            rect,
            event: ui_wgpu::wgpu::UiEvent::PointerDown { x: 50.0, y: 70.0, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() },
        }],
        Some(ui_render::PointerId(1)),
        &mut input,
    );

    let queued = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    let down = queued.iter().find(|action| action.action == "canvasPointerDown").expect("the press publishes canvasPointerDown");
    let args = down.args.as_ref().expect("the pointer payload is present");
    let number = |key: &str| args.get(key).and_then(semio_framework::DslValue::as_f64);
    assert_eq!(number("x"), Some(20.0), "x is logical pixels inside the surface rect, not a world coordinate");
    assert_eq!(number("y"), Some(30.0), "y is logical pixels inside the surface rect, not a world coordinate");
    assert_eq!(number("width"), Some(200.0), "width is the surface's logical width — draw/layout divide by it");
    assert_eq!(number("height"), Some(120.0), "height is the surface's logical height");
    assert_eq!(number("button"), Some(0.0));
    for flag in ["shift", "ctrl", "meta", "alt", "extend"] {
        assert_eq!(args.get(flag).and_then(semio_framework::DslValue::as_bool), Some(false), "{flag} must be present under React's own name");
    }
    assert!(number("worldX").is_some() && number("worldY").is_some(), "the optional world lane 🖍️draw declares rides along, got {args:?}");
    semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture(&mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
}

/// 🎨️ `ink_wheel` reads straight from the scene's own `ink_canvas` payload (unlike TextEditor,
/// it needs no separate lazily-render-created host state) — mirrors `RenderEntry::ink_scene`'s
/// own fixture (`apply_scene_wheel_dispatches_actions_for_a_previously_dead_surface`).
#[test]
fn scene_command_dispatches_an_ink_canvas_scroll_action() {
    let window_id = "apply-ui-commands-scene-ink-canvas-scroll";
    let mut scene_node = component_scene_ui("s1", semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas);
    if let UiNode::ComponentScene(scene) = &mut scene_node {
        scene.ink_canvas = Some(semio_framework_ui_scene_plane::InkCanvasScene { document_json: "{}".into(), selection_json: "[]".into(), hovered_id: None, active_utility: String::new(), view_mode: "canvas".into(), interactive: true, interaction_domain: None });
    }
    let node = seed_scene_window_with(window_id, scene_node);
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();

    apply_ui_commands(
        &mut vec![ui_wgpu::wgpu::UiCommand::Scene {
            window_id: window_id.into(),
            node,
            surface_id: "s1".into(),
            kind: semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas,
            rect,
            event: ui_wgpu::wgpu::UiEvent::Scroll { x: 10.0, y: 10.0, delta_x: 0.0, delta_y: -1.0, modifiers: Default::default() },
        }],
        None,
        &mut input,
    );

    let queued = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert!(queued.iter().any(|action| action.action == "setCamera"), "a real per-event Scroll over an ink-canvas scene should reach handle_scene_wheel, got {queued:?}");
}

fn ink_editing_law() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🖋️ink-canvas-editing/🔣️.json")).expect("shared InkCanvas editing law parses")
}

fn ink_editing_scene(law: &Value) -> UiNode {
    let mut scene_node = component_scene_ui(law["scene"]["surfaceId"].as_str().expect("surface id"), semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas);
    if let UiNode::ComponentScene(scene) = &mut scene_node {
        scene.controller_id = law["scene"]["controllerId"].as_str().expect("controller id").into();
        scene.ink_canvas = Some(semio_framework_ui_scene_plane::InkCanvasScene {
            document_json: serde_json::to_string(&law["document"]).expect("InkCanvas document serializes"),
            selection_json: law["scene"]["inkCanvas"]["selectionJson"].as_str().expect("selection").into(),
            hovered_id: None,
            active_utility: law["scene"]["inkCanvas"]["activeUtility"].as_str().expect("utility").into(),
            view_mode: law["scene"]["inkCanvas"]["viewMode"].as_str().expect("view mode").into(),
            interactive: law["scene"]["inkCanvas"]["interactive"].as_bool().expect("interactive"),
            interaction_domain: None,
        });
    }
    scene_node
}

#[track_caller]
fn drive_ink_scene_terminal(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    for _ in 0..4096 {
        if !drive_scene_interaction_step(input) {
            assert!(scene_interaction_terminal_is_empty());
            return;
        }
    }
    panic!("InkCanvas interaction did not reach a terminal state");
}

#[track_caller]
fn retire_presented_document(window_id: &str) {
    assert!(request_ui_document_close(window_id));
    drain_presented_document_close(window_id);
}

#[track_caller]
fn drain_presented_document_close(window_id: &str) {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    for _ in 0..262_144 {
        if !ui_document_close_pending() {
            break;
        }
        assert!(caller.turn(|cx|close_ui_document_one(cx)).expect("original fixture caller turn"));
    }
    assert!(!ui_document_close_pending());
    assert!(UI_ENGINE.with(|cell| cell.borrow().tree(window_id).is_none()));
    caller.finish();
}

fn dispatch_ink_pointer(window_id: &str, node: NodeId, surface_id: &str, rect: Rect, x: f32, y: f32, down: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) {
    let event = if down {
        ui_wgpu::wgpu::UiEvent::PointerDown { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }
    } else {
        ui_wgpu::wgpu::UiEvent::PointerUp { x, y, button: ui_wgpu::wgpu::PointerButton::Primary, modifiers: Default::default() }
    };
    apply_ui_commands(&mut vec![ui_wgpu::wgpu::UiCommand::Scene { window_id: window_id.into(), node, surface_id: surface_id.into(), kind: semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas, rect, event }], Some(ui_render::PointerId(1)), input);
    drive_ink_scene_terminal(input);
}
//#endregion 🔖️SceneCommandTests
