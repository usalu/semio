
use super::*;

fn canvas_scene(surface_id: &str, layers_json: String) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: "controller".into(),
        component_kind: SurfaceKind::Canvas2d,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: Some(ui_wgpu::wgpu::Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json, snapshot: None }),
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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
        render_canvas_2d(&node, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
    }
    let vertex_colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).map(|v| v.color).collect();
    let ring = CANVAS2D_SELECTION_RING;
    let glow = CANVAS2D_SELECTION_GLOW;
    assert!(vertex_colors.contains(&[ring.r, ring.g, ring.b, ring.a]), "expected the crisp amber ring color among vertices, got {vertex_colors:?}");
    assert!(vertex_colors.contains(&[glow.r, glow.g, glow.b, glow.a]), "expected the soft amber glow color among vertices, got {vertex_colors:?}");
    assert!(!vertex_colors.contains(&[theme.accent.r, theme.accent.g, theme.accent.b, theme.accent.a]), "the selection ring must no longer use theme.accent (the app's red/crimson token), got {vertex_colors:?}");
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
    assert!(
        immediate.iter().all(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) != Some(surface_id)),
        "sweeping immediately (before the ~350ms settle window) must not yet report this surface"
    );
    let due = sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
    let matched = due.iter().find(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) == Some(surface_id)).expect("this surface's setCamera fires once its deadline has passed");
    assert_eq!(matched.controller_id, "controller");
    assert_eq!(matched.action, "setCamera");
}

/// 🕒️ A Canvas2d pan-drag (`SceneDragMode::PanViewport`) gets the identical settle-then-dispatch
/// treatment as wheel-zoom — same deadline map, same sweep.
#[test]
fn canvas2d_pan_drag_schedules_a_settled_camera_dispatch() {
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
        schedule_scene_camera_dispatch(&format!("surface-{index}"));
    }
    schedule_scene_camera_dispatch("overflow");
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
