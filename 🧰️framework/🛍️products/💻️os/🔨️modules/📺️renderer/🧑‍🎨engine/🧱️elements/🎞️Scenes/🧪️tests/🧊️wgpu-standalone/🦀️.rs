#[cfg(test)]
pub fn sweep_expired_scene_camera_dispatches(now_ms: f64) -> Vec<ActionDescriptor> {
    let mut cursor = SceneCameraDispatchCursor::begin(now_ms);
    let mut actions = Vec::new();
    loop {
        match cursor.step() {
            SceneCameraDispatchStep::Pending => {}
            SceneCameraDispatchStep::Action(action) => actions.push(action),
            SceneCameraDispatchStep::Complete | SceneCameraDispatchStep::Fault(_) => return actions,
        }
    }
}

#[cfg(test)]
fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.into(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

#[cfg(test)]
fn queue_surface_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, action: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, action, "surfaceId", &scene.surface_id])?;
    let mut reservation = input.reserve_action(&scene.controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.end_container()?;
    reservation.publish()
}

#[cfg(test)]
#[test]
fn production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only() {
    const SCENES_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    const INTERPRETER_SOURCE: &str = include_str!("../../../🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs");
    const ENGINE_CANVAS_SOURCE: &str = include_str!("../../../⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs");
    assert!(!SCENES_SOURCE.contains(concat!("queue_", "event(")));
    assert!(!INTERPRETER_SOURCE.contains(concat!("queue_", "event(")));
    assert!(!SCENES_SOURCE.contains(concat!("drain_", "keys(")));
    assert!(INTERPRETER_SOURCE.contains("struct SceneInteractionIntent"));
    assert!(INTERPRETER_SOURCE.contains("drive_scene_interaction_step"));
    assert!(INTERPRETER_SOURCE.contains("tree_revision"));
    assert!(!INTERPRETER_SOURCE.contains("Some(scene.clone())"));
    assert!(!INTERPRETER_SOURCE.contains("reserve_actions(ui_wgpu::wgpu::action::ACTION_BATCH_ITEM_CAPACITY"));
    for function in ["handle_scene_wheel", "handle_scene_pointer_move", "handle_scene_pointer_button"] {
        assert!(SCENES_SOURCE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must be test-only");
    }
    for function in ["ink_apply_events_action", "ink_set_selection_action", "ink_set_hover_action", "ink_set_camera_action", "ink_pointer_down", "ink_pointer_up", "ink_hover_move", "ink_wheel"] {
        assert!(SCENES_SOURCE.contains(&format!("#[cfg(test)]\nfn {function}")), "{function} must be test-only");
    }
    assert!(SCENES_SOURCE.contains("struct InkEventJsonPages"));
    assert!(SCENES_SOURCE.contains("struct InkInteractionJob"));
    assert!(SCENES_SOURCE.contains("struct InkInteractionDocument"));
    assert!(SCENES_SOURCE.contains("Box<[Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY]>"));
    assert!(!SCENES_SOURCE.contains("VecDeque<Value>"));
    assert!(!SCENES_SOURCE.contains("stroke_update: Option<(String, Value)>"));
    assert!(!SCENES_SOURCE.contains("pending_fragments: VecDeque"));
    for variant in ["InkMove", "InkResize", "InkStroke", "InkEraser", "InkMarqueeDrag", "InkPan"] {
        assert!(SCENES_SOURCE.contains(&format!("SceneDragMode::{variant}")), "{variant} must have a retained route");
    }
    assert!(!SCENES_SOURCE.contains(concat!("mem::", "forget")));
    for function in ["text_editor_apply_key", "text_editor_pointer_down", "text_editor_pointer_move", "text_editor_pointer_up", "text_editor_select_span_at_screen", "text_editor_set_selection", "text_editor_apply_completion"] {
        assert!(ENGINE_CANVAS_SOURCE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must not remain production-capable");
    }
    for function in ["node_graph_wheel", "node_graph_pointer_down", "node_graph_pointer_move", "node_graph_pointer_up", "tiled_map_wheel", "puzzle_board_pointer_move", "puzzle_board_pointer_up", "puzzle_board_pointer_leave", "puzzle_board_wheel"] {
        assert!(ENGINE_CANVAS_SOURCE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must remain test-only");
    }
    for function in ["tiled_map_pointer_down", "tiled_map_pointer_move", "tiled_map_pointer_up", "puzzle_board_pointer_move", "puzzle_board_pointer_up", "puzzle_board_pointer_leave", "puzzle_board_wheel"] {
        assert!(SCENES_SOURCE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must remain test-only");
    }
}

#[cfg(test)]
#[test]
fn ink_event_pages_preserve_order_and_fail_before_exceeding_fixed_storage() {
    let mut pages = InkEventJsonPages::default();
    pages.push(&json!({ "operation": "first", "value": "quoted\\\"" })).unwrap();
    pages.push(&json!({ "operation": "second" })).unwrap();
    pages.seal().unwrap();
    let decoded: Vec<Value> = serde_json::from_str(pages.as_str().unwrap()).unwrap();
    assert_eq!(decoded[0]["operation"], "first");
    assert_eq!(decoded[1]["operation"], "second");

    let mut saturated = InkEventJsonPages::default();
    let oversized = json!({ "value": "x".repeat(INK_EVENT_JSON_BYTE_CAPACITY) });
    assert_eq!(saturated.push(&oversized), Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits));
    assert_eq!(saturated.items, 0);
}

#[cfg(test)]
#[test]
fn ink_raw_fragment_pages_are_single_slab_fifo_and_reject_max_plus_one() {
    let mut pages = InkRawPages::default();
    pages.push("{\"id\":\"first\"}").unwrap();
    pages.push("{\"id\":\"second\"}").unwrap();
    assert_eq!(pages.front().unwrap(), Some("{\"id\":\"first\"}"));
    assert!(pages.pop_front());
    assert_eq!(pages.front().unwrap(), Some("{\"id\":\"second\"}"));
    assert!(pages.pop_front());
    assert!(!pages.pop_front());

    let mut saturated = InkRawPages::default();
    assert_eq!(saturated.push(&"x".repeat(INK_INTERACTION_DOCUMENT_BYTE_CAPACITY + 1)), Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits));
    assert_eq!(saturated.len, 0);
    assert_eq!(saturated.byte_len, 0);
}

#[cfg(test)]
#[test]
fn ink_block_cursor_visits_nested_groups_in_stable_depth_first_order() {
    let source = json!({
        "blocks": [{
            "id": "group",
            "kind": "group",
            "children": [
                { "id": "a", "kind": "text" },
                {
                    "id": "nested",
                    "kind": "group",
                    "children": [{ "id": "b", "kind": "text" }]
                }
            ]
        }, {
            "id": "c",
            "kind": "text",
            "children": [{ "id": "ignored", "kind": "text" }]
        }]
    })
    .to_string();
    let mut spans = Box::new(std::array::from_fn(|_| None));
    let mut span_len = 0;
    collect_ink_document_block_spans(source.as_bytes(), &mut spans, &mut span_len).unwrap();
    let header = InkDocumentJson::default();
    let document = InkInteractionDocument {
        source,
        spans,
        span_len,
        schema: header.schema,
        id: header.id,
        camera: header.camera,
        active_utility: header.active_utility,
        snap_enabled: header.snap_enabled,
        snap_grid_spacing: header.snap_grid_spacing,
        eraser_radius: header.eraser_radius,
    };
    let mut cursor = InkBlockCursor::default();
    let mut ids = Vec::new();
    while let Some(block) = cursor.next(&document).unwrap() {
        ids.push(ink_item_id(&block).to_owned());
    }
    let legacy: InkDocumentJson = serde_json::from_str(&document.source).unwrap();
    let legacy_ids: Vec<&str> = flatten_ink_items(&legacy.blocks).into_iter().map(ink_item_id).collect();
    assert_eq!(ids, ["group", "a", "nested", "b", "c"]);
    assert_eq!(ids.iter().map(String::as_str).collect::<Vec<_>>(), legacy_ids);
    assert!(cursor.next(&document).unwrap().is_none());
}

#[cfg(test)]
#[test]
fn ink_nested_value_admission_rejects_hostile_depth() {
    let mut value = Value::Null;
    for _ in 0..=ui_wgpu::wgpu::action::ACTION_DEPTH_CAPACITY {
        value = Value::Array(vec![value]);
    }
    let mut nodes = 0usize;
    assert_eq!(validate_ink_value(&value, 0, &mut nodes), Err(ui_wgpu::wgpu::BoundedActionFault::DepthCredits));
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
fn now_ms() -> f64 {
    web_sys::window().and_then(|window| window.performance()).map(|perf| perf.now()).unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
fn now_ms() -> f64 {
    0.0
}

#[cfg(test)]
fn canvas_world_pointer_json(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, extra: Value) -> Value {
    let state = scene_state(&scene.surface_id);
    let (wx, wy) = state.viewport.screen_to_world(x, y, inner);
    let mut payload = json!({
        "surfaceId": scene.surface_id,
        "x": wx,
        "y": wy,
    });
    if let (Some(base), Some(patch)) = (payload.as_object_mut(), extra.as_object()) {
        for (key, value) in patch {
            base.insert(key.clone(), value.clone());
        }
    }
    payload
}

#[cfg(test)]
pub fn handle_scene_wheel(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, delta: f32, ctrl: bool) -> Vec<ActionDescriptor> {
    if !bounds.contains(x, y) {
        return Vec::new();
    }
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    match scene.component_kind {
        SurfaceKind::Table => {
            let current = scroll_offset(&scene.surface_id, "body");
            set_scroll_offset(&scene.surface_id, "body", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::TextEditor => Vec::new(),
        SurfaceKind::VirtualFileSystem => {
            let current = scroll_offset(&scene.surface_id, "vfs");
            set_scroll_offset(&scene.surface_id, "vfs", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::Canvas2d => {
            mutate_scene_state(&scene.surface_id, |state| {
                let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.125, 8.0);
                state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
            });
            schedule_scene_camera_dispatch(&scene.surface_id);
            Vec::new()
        }
        SurfaceKind::Paint2d => {
            if let Some(paint_2d) = &scene.paint_2d {
                let doc: Paint2dDocSyncJson = serde_json::from_str(&paint_2d.document_sync_json).unwrap_or_default();
                mutate_scene_state(&scene.surface_id, |state| {
                    if state.viewport.zoom <= 0.0 {
                        state.viewport = Viewport { x: doc.camera.x as f32, y: doc.camera.y as f32, zoom: doc.camera.zoom as f32 };
                    }
                    let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                    state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.05, 32.0);
                    state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
                });
                schedule_scene_camera_dispatch(&scene.surface_id);
            }
            Vec::new()
        }
        SurfaceKind::NodeGraph => engine_canvas::node_graph_wheel(&scene.surface_id, &scene.controller_id, inner, x, y, delta, ctrl),
        SurfaceKind::TiledMap => engine_canvas::tiled_map_wheel(&scene.surface_id, &scene.controller_id, inner, x, y, delta, ctrl),
        SurfaceKind::InkCanvas => ink_wheel(scene, inner, x, y, delta),
        SurfaceKind::GraphTimeline => {
            let current = scroll_offset(&scene.surface_id, "history");
            set_scroll_offset(&scene.surface_id, "history", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::DiffView => {
            let current = scroll_offset(&scene.surface_id, "diff");
            set_scroll_offset(&scene.surface_id, "diff", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::EventFeed => {
            let current = scroll_offset(&scene.surface_id, "feed");
            set_scroll_offset(&scene.surface_id, "feed", current + delta * 0.5);
            Vec::new()
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
pub fn handle_scene_pointer_move(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, down: bool, _button: i16, drag_dx: f32, drag_dy: f32) -> Vec<ActionDescriptor> {
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    let mut actions = Vec::new();
    let state = scene_state(&scene.surface_id);
    if down {
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::PanViewport => {
                    let vp = state.viewport;
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.viewport.x -= drag_dx / vp.zoom.max(0.01);
                        state.viewport.y -= drag_dy / vp.zoom.max(0.01);
                        state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
                    });
                    schedule_scene_camera_dispatch(&scene.surface_id);
                }
                SceneDragMode::MapMarquee { start_x, start_y, method, .. } => {
                    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
                    let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(&scene.surface_id, |state| {
                        if distance >= MAP_MARQUEE_THRESHOLD_PX {
                            state.map_marquee_active = true;
                        }
                        if state.map_marquee_active {
                            if method == "lasso" {
                                state.map_marquee_points.push((sx as f32, sy as f32));
                            } else {
                                state.map_marquee_points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                            }
                        }
                    });
                }
                SceneDragMode::MapPan => {}
                SceneDragMode::InkPan { start_x, start_y, camera_x, camera_y, zoom } => {
                    let dx = (x - start_x) as f64;
                    let dy = (y - start_y) as f64;
                    let next = InkCameraF { x: camera_x + dx, y: camera_y + dy, zoom: *zoom };
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.ink_camera = Some((next.x, next.y, next.zoom));
                    });
                    actions.push(ink_set_camera_action(scene, next));
                }
                SceneDragMode::InkMove { origins, start_x, start_y } => {
                    let camera = ink_current_camera(scene);
                    let dx = (x - start_x) as f64 / camera.zoom.max(0.0001);
                    let dy = (y - start_y) as f64 / camera.zoom.max(0.0001);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let mut events = Vec::new();
                    let mut new_overrides = Vec::new();
                    for (id, (ox, oy)) in origins.iter() {
                        if let Some(block) = find_ink_item(&doc.blocks, id) {
                            let updated = ink_item_with_position(block, ox + dx, oy + dy);
                            events.push(json!({ "operation": "updateBlock", "blockId": id, "block": updated }));
                            new_overrides.push((id.clone(), updated));
                        }
                    }
                    if !events.is_empty() {
                        mutate_scene_state(&scene.surface_id, |state| {
                            for (id, block) in new_overrides {
                                state.ink_overrides.insert(id, block);
                            }
                        });
                        actions.push(ink_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::InkResize { handle, from, start_x, start_y, selected_ids } => {
                    let camera = ink_current_camera(scene);
                    let dx = (x - start_x) as f64 / camera.zoom.max(0.0001);
                    let dy = (y - start_y) as f64 / camera.zoom.max(0.0001);
                    let to = ink_resize_bounds(*from, handle, dx, dy, 8.0);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let mut events = Vec::new();
                    let mut new_overrides = Vec::new();
                    for id in selected_ids {
                        if let Some(block) = find_ink_item(&doc.blocks, id) {
                            let updated = scale_ink_item(block, *from, to);
                            events.push(json!({ "operation": "updateBlock", "blockId": id, "block": updated }));
                            new_overrides.push((id.clone(), updated));
                        }
                    }
                    if !events.is_empty() {
                        mutate_scene_state(&scene.surface_id, |state| {
                            for (id, block) in new_overrides {
                                state.ink_overrides.insert(id, block);
                            }
                        });
                        actions.push(ink_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::InkStroke { block_id } => {
                    let camera = ink_current_camera(scene);
                    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let current = state.ink_overrides.get(block_id).cloned().or_else(|| find_ink_item(&doc.blocks, block_id).cloned());
                    if let Some(mut block) = current {
                        let bx = ink_item_num(&block, "x");
                        let by = ink_item_num(&block, "y");
                        let local = json!([world_x - bx, world_y - by]);
                        if let Some(obj) = block.as_object_mut() {
                            let mut points = obj.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
                            points.push(local);
                            obj.insert("points".into(), Value::Array(points));
                        }
                        let block_id = block_id.clone();
                        mutate_scene_state(&scene.surface_id, |state| {
                            state.ink_overrides.insert(block_id.clone(), block.clone());
                        });
                        actions.push(ink_apply_events_action(scene, &[json!({ "operation": "updateBlock", "blockId": block_id, "block": block })], "live", None));
                    }
                }
                SceneDragMode::InkEraser { mode } => {
                    let camera = ink_current_camera(scene);
                    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let events = if mode == "eraserStroke" { erase_ink_stroke_events(&doc.blocks, world_x, world_y, 8.0) } else { erase_ink_stroke_points_events(&doc.blocks, world_x, world_y, doc.eraser_radius.unwrap_or(12.0)) };
                    if !events.is_empty() {
                        actions.push(ink_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::InkMarqueeDrag { start_x, start_y } => {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.ink_marquee_points = vec![(*start_x, *start_y), (x, y)];
                    });
                }
            }
        }
    }
    match scene.component_kind {
        SurfaceKind::InkCanvas if !down => {
            actions.extend(ink_hover_move(scene, inner, x, y));
        }
        SurfaceKind::Canvas2d if down => {
            actions.push(scene_action(scene, "canvasPointerMove", canvas_world_pointer_json(scene, inner, x, y, json!({}))));
        }
        SurfaceKind::NodeGraph if down => {
            actions.extend(engine_canvas::node_graph_pointer_move(&scene.surface_id, &scene.controller_id, inner, x, y, false, false, false));
        }
        SurfaceKind::NodeGraph if !down => {
            actions.extend(engine_canvas::node_graph_pointer_move(&scene.surface_id, &scene.controller_id, inner, x, y, false, false, false));
        }
        SurfaceKind::TextEditor => {}
        _ => {}
    }
    actions
}

#[cfg(test)]
pub fn handle_scene_pointer_button(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, down: bool, button: i16, shift: bool) -> Vec<ActionDescriptor> {
    let inner = bounds;
    if !inner.contains(x, y) {
        if !down {
            mutate_scene_state(&scene.surface_id, |state| {
                state.drag = None;
                state.pointer_was_down = false;
            });
        }
        return Vec::new();
    }
    let mut actions = Vec::new();
    if down {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = true;
        });
        match scene.component_kind {
            SurfaceKind::Canvas2d => {
                if button == 0 {
                    mutate_scene_state(&scene.surface_id, |state| {
                        if !state.paint_stroke_active {
                            state.paint_stroke_active = true;
                        }
                    });
                    actions.push(scene_action(scene, "paintStrokeBegin", json!({ "surfaceId": scene.surface_id })));
                }
                actions.push(scene_action(scene, "canvasPointerDown", canvas_world_pointer_json(scene, inner, x, y, json!({ "button": button, "extend": shift }))));
                if button == 1 || button == 2 {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport });
                    });
                }
            }
            SurfaceKind::Paint2d => {
                if button == 1 || button == 2 {
                    if let Some(paint_2d) = &scene.paint_2d {
                        let doc: Paint2dDocSyncJson = serde_json::from_str(&paint_2d.document_sync_json).unwrap_or_default();
                        mutate_scene_state(&scene.surface_id, |state| {
                            if state.viewport.zoom <= 0.0 {
                                state.viewport = Viewport { x: doc.camera.x as f32, y: doc.camera.y as f32, zoom: doc.camera.zoom as f32 };
                            }
                            state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport });
                        });
                    }
                }
            }
            SurfaceKind::NodeGraph => {
                actions.extend(engine_canvas::node_graph_pointer_down(&scene.surface_id, &scene.controller_id, inner, x, y, button, shift, false, false, false));
            }
            SurfaceKind::TextEditor => {}
            SurfaceKind::InkCanvas => {
                actions.extend(ink_pointer_down(scene, inner, x, y, button, shift));
            }
            _ => {}
        }
    } else {
        match scene.component_kind {
            SurfaceKind::InkCanvas => {
                actions.extend(ink_pointer_up(scene, inner, x, y));
            }
            SurfaceKind::Canvas2d => {
                actions.push(scene_action(scene, "canvasPointerUp", canvas_world_pointer_json(scene, inner, x, y, json!({}))));
                mutate_scene_state(&scene.surface_id, |state| {
                    if state.paint_stroke_active {
                        state.paint_stroke_active = false;
                    }
                });
                actions.push(scene_action(scene, "paintStrokeEnd", json!({ "surfaceId": scene.surface_id })));
            }
            SurfaceKind::NodeGraph => {
                actions.extend(engine_canvas::node_graph_pointer_up(&scene.surface_id, &scene.controller_id, inner, x, y, shift, false, false));
            }
            SurfaceKind::TextEditor => {}
            _ => {}
        }
        if let Some(target) = hit_double_click_target(scene, inner, x, y) {
            let now = now_ms();
            let prior = scene_state(&scene.surface_id);
            if prior.last_click_target.as_deref() == Some(target.as_str()) && now - prior.last_click_ms < 400.0 {
                if let Some(action) = double_click_action(scene, &target, inner, x, y) {
                    actions.push(action);
                }
            }
            mutate_scene_state(&scene.surface_id, |state| {
                state.last_click_target = Some(target);
                state.last_click_ms = now;
            });
        }
        mutate_scene_state(&scene.surface_id, |state| {
            state.drag = None;
            state.pointer_was_down = false;
        });
    }
    actions
}

#[cfg(test)]
fn hit_double_click_target(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Option<String> {
    match scene.component_kind {
        SurfaceKind::VirtualFileSystem => {
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let body_y = inner.y + 24.0;
            let index = ((y - body_y + scroll) / row_h).floor() as i32;
            if index < 0 {
                return None;
            }
            Some(format!("{}.vfs.index.{index}", scene.surface_id))
        }
        SurfaceKind::NodeGraph => hit_graph_node(scene, inner, x, y).map(|id| format!("{}.node.{}", scene.surface_id, id)),
        _ => None,
    }
}

#[cfg(test)]
fn double_click_action(scene: &UiComponentSceneNode, target: &str, inner: Rect, _x: f32, y: f32) -> Option<ActionDescriptor> {
    match scene.component_kind {
        SurfaceKind::VirtualFileSystem => {
            let vfs = scene.virtual_file_system.as_ref()?;
            let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).ok()?;
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let index = ((y - inner.y - 24.0 + scroll) / row_h).floor() as usize;
            rows.get(index).and_then(|row| vfs_double_click_action(scene, row))
        }
        SurfaceKind::NodeGraph => {
            let node_id = target.strip_prefix(&format!("{}.node.", scene.surface_id))?;
            let record = find_graph_node(scene, node_id)?;
            let instance_id = record.instance_id.as_deref()?;
            Some(scene_action(scene, "openInstance", json!({ "surfaceId": scene.surface_id, "instanceId": instance_id })))
        }
        _ => None,
    }
}

#[cfg(test)]
fn render_placeholder(kind: &str, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    draw_text(ctx, &format!("{kind} host"), bounds.x + 12.0, bounds.y + 24.0, theme.font_size_body, theme.text_muted);
}

#[cfg(test)]
fn paint2d_navigator_fit_viewport(flat: &[Paint2dFlatLayer], inner: Rect) -> Viewport {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for layer in flat {
        let w = layer.width as f64 * layer.scale_x;
        let h = layer.height as f64 * layer.scale_y;
        let x0 = (layer.x - w * 0.5) as f32;
        let y0 = (layer.y - h * 0.5) as f32;
        let x1 = (layer.x + w * 0.5) as f32;
        let y1 = (layer.y + h * 0.5) as f32;
        min_x = min_x.min(x0.min(x1));
        min_y = min_y.min(y0.min(y1));
        max_x = max_x.max(x0.max(x1));
        max_y = max_y.max(y0.max(y1));
    }
    if !min_x.is_finite() || !min_y.is_finite() || !max_x.is_finite() || !max_y.is_finite() {
        return Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
    }
    let content_w = (max_x - min_x).max(1.0);
    let content_h = (max_y - min_y).max(1.0);
    let inner_w = (inner.w - PAINT2D_NAVIGATOR_PADDING * 2.0).max(1.0);
    let inner_h = (inner.h - PAINT2D_NAVIGATOR_PADDING * 2.0).max(1.0);
    let zoom = (inner_w / content_w).min(inner_h / content_h).clamp(0.05, 32.0);
    Viewport { x: min_x + (max_x - min_x) * 0.5, y: min_y + (max_y - min_y) * 0.5, zoom }
}

#[cfg(test)]
fn paint2d_navigator_overlay_rect(content_camera_json: &str, content_viewport_json: Option<&str>, navigator_viewport: &Viewport, navigator_inner: Rect) -> Option<Rect> {
    let content_viewport_json = content_viewport_json?;
    let content_camera = Viewport::from_json(content_camera_json);
    let size: Value = serde_json::from_str(content_viewport_json).ok()?;
    let content_w = size.get("width").and_then(Value::as_f64).unwrap_or(0.0) as f32;
    let content_h = size.get("height").and_then(Value::as_f64).unwrap_or(0.0) as f32;
    if content_w <= 0.0 || content_h <= 0.0 {
        return None;
    }
    let content_rect = Rect::new(0.0, 0.0, content_w, content_h);
    let (wx0, wy0) = content_camera.screen_to_world(0.0, 0.0, content_rect);
    let (wx1, wy1) = content_camera.screen_to_world(content_w, content_h, content_rect);
    let (sx0, sy0) = navigator_viewport.world_to_screen(wx0, wy0, navigator_inner);
    let (sx1, sy1) = navigator_viewport.world_to_screen(wx1, wy1, navigator_inner);
    Some(Rect::new(sx0.min(sx1), sy0.min(sy1), (sx1 - sx0).abs(), (sy1 - sy0).abs()))
}

#[cfg(test)]
fn merge_action_args(base: &ActionDescriptor, patch: Value) -> ActionDescriptor {
    let mut args = match &base.args {
        Some(dsl) => match Value::from(dsl) {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        },
        None => serde_json::Map::new(),
    };
    if let Value::Object(patch_map) = patch {
        args.extend(patch_map);
    }
    ActionDescriptor { controller_id: base.controller_id.clone(), action: base.action.clone(), args: semio_framework::optional_json_to_dsl(Some(Value::Object(args))) }
}

#[cfg(test)]
fn render_table_cell(cell: &Value, rect: Rect, ctx: &mut FrameworkWidgetContext<'_>) -> Option<String> {
    let Ok(payload) = serde_json::from_value::<TableCellPayload>(cell.clone()) else {
        return Some(match cell {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        });
    };
    match payload {
        TableCellPayload::Text { value } => Some(value),
        TableCellPayload::Number { value } => Some(value.to_string()),
        TableCellPayload::Stepper { value, min, max, step, action } => {
            let seg = rect.w / 3.0;
            let minus = Rect::new(rect.x, rect.y, seg, rect.h);
            let center = Rect::new(rect.x + seg, rect.y, seg, rect.h);
            let plus = Rect::new(rect.x + seg * 2.0, rect.y, seg, rect.h);
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "−".into(), event: (value > min).then(|| merge_action_args(&action, json!({ "delta": -step }))) }, minus, ctx);
            render_widget(&WidgetNode::Text { value: format!("{value:.0}"), emphasize: false }, center, ctx);
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "+".into(), event: (value < max).then(|| merge_action_args(&action, json!({ "delta": step }))) }, plus, ctx);
            None
        }
        TableCellPayload::Buttons { buttons } => {
            let seg = if buttons.is_empty() { rect.w } else { rect.w / buttons.len() as f32 };
            for (index, button) in buttons.iter().enumerate() {
                let button_rect = Rect::new(rect.x + index as f32 * seg, rect.y, seg, rect.h);
                render_widget(&WidgetNode::Button { id: None, icon_id: IconName::from_str(&button.icon_id), label: button.label.clone().unwrap_or_default(), event: Some(button.action.clone()) }, button_rect, ctx);
            }
            None
        }
    }
}

#[cfg(test)]
fn render_table(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(table) = &scene.table else {
        return render_placeholder("table", bounds, ctx);
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let selected_ids: Vec<String> =
        table.selection_json.as_deref().and_then(|json| serde_json::from_str::<Value>(json).ok()).and_then(|value| value.get("selectedIds").cloned()).and_then(|value| serde_json::from_value(value).ok()).unwrap_or_default();
    let sort: Option<TableSortJson> = table.sort_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    let inner = bounds;
    let header_h = theme.control_height * 1.33;
    let row_h = theme.control_height;
    let pad = theme.padding_standard;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    let col_w = if columns.is_empty() { inner.w } else { inner.w / columns.len() as f32 };
    for (index, column) in columns.iter().enumerate() {
        let x = inner.x + index as f32 * col_w;
        let sorted_here = sort.as_ref().filter(|s| s.column_id == column.id);
        let label = match sorted_here {
            Some(s) if s.direction == "desc" => format!("{} \u{25BC}", column.label),
            Some(_) => format!("{} \u{25B2}", column.label),
            None => column.label.clone(),
        };
        draw_text(ctx, &label, x + pad, inner.y + header_h * 0.65, theme.font_size_small, if sorted_here.is_some() { theme.text } else { theme.text_muted });
        if column.sortable {
            let next_direction = match sorted_here {
                Some(s) if s.direction == "asc" => "desc",
                _ => "asc",
            };
            ctx.input.register_hit(HitTarget {
                rect: Rect::new(x, inner.y, col_w, header_h),
                event: Some(scene_action(scene, "sortTable", json!({ "surfaceId": scene.surface_id, "columnId": column.id, "direction": next_direction }))),
                control_id: Some(format!("{}.header.{}", scene.surface_id, column.id)),
                kind: HitKind::Generic,
                drag_axis: None,
                drag_data: None,
            });
        }
    }
    ctx.draw.push_line(inner.x, inner.y + header_h, inner.x + inner.w, inner.y + header_h, theme.separator, 1.0);
    let body = Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h);
    let scroll = scroll_offset(&scene.surface_id, "body");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.surface_id, "body")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    let hovered_row = ctx.input.hovered_id.clone();
    if rows.is_empty() {
        let message = "No rows";
        draw_text(ctx, message, body.x + body.w * 0.5 - 40.0, body.y + body.h * 0.5, theme.font_size_small, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = row.get("id").or_else(|| row.get("pluginId")).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let control_id = format!("{}.row.{}", scene.surface_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let selected = selected_ids.iter().any(|id| id == &row_id);
        if selected {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);
        for (col_index, column) in columns.iter().enumerate() {
            let x = body.x + col_index as f32 * col_w;
            let cell_rect = Rect::new(x + pad, y, col_w - pad * 2.0, row_h);
            let text = match row.get(&column.id) {
                Some(value) => render_table_cell(value, cell_rect, ctx),
                None => Some("—".into()),
            };
            if let Some(text) = text {
                draw_text(ctx, &text, x + pad, y + row_h * 0.65, theme.font_size_small, if selected || hovered { theme.active_foreground } else { theme.text });
            }
        }
        let drag_data = table.row_drag_mime.as_ref().and_then(|mime| row.get("_drag").map(|payload| HashMap::from([(mime.clone(), payload.to_string())])));
        ctx.input.register_hit(HitTarget { rect: row_rect, event: Some(scene_action(scene, "selectRow", json!({ "surfaceId": scene.surface_id, "row": row }))), control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data });
    }
    ctx.draw.pop_scissor();
}

#[cfg(test)]
fn render_block_list(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(block_list) = &scene.block_list else {
        return render_placeholder("block-list", bounds, ctx);
    };
    let steps: Vec<BlockListStepJson> = serde_json::from_str(&block_list.steps_json).unwrap_or_default();
    let palette: Vec<BlockListPaletteEntryJson> = serde_json::from_str(&block_list.palette_json).unwrap_or_default();
    let selected_id = block_list.selected_id.as_deref();

    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    let btn_w = 26.0;
    let palette_w = (bounds.w * 0.22).clamp(140.0, 220.0);
    let main = Rect::new(bounds.x, bounds.y, (bounds.w - palette_w).max(0.0), bounds.h);
    let palette_rect = Rect::new(main.x + main.w, bounds.y, palette_w, bounds.h);

    //#region Steps
    let header_rect = Rect::new(main.x, main.y, main.w, row_h);
    draw_text(ctx, "Steps", header_rect.x + pad, header_rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    let add_step_rect = Rect::new(header_rect.x + header_rect.w - 108.0, header_rect.y + 4.0, 104.0, row_h - 8.0);
    render_widget(&WidgetNode::Button { id: Some(format!("{}.addStep", scene.surface_id)), icon_id: Some("plus".into()), label: "Add Step".into(), event: Some(scene_action(scene, "addStep", json!({}))) }, add_step_rect, ctx);

    let body = Rect::new(main.x, main.y + row_h, main.w, (main.h - row_h).max(0.0));
    let scroll = scroll_offset(&scene.surface_id, "blockList");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.surface_id, "blockList")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    if steps.is_empty() {
        draw_text(ctx, "No steps", body.x + pad, body.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    let step_count = steps.len();
    let mut y = body.y - scroll;
    for (step_index, step) in steps.iter().enumerate() {
        let block_count = step.blocks.len();
        let description_h = step.description.as_ref().map_or(0.0, |_| theme.font_size_small + pad);
        let step_h = row_h + description_h + block_count as f32 * row_h + pad * 2.0;
        if y + step_h >= body.y && y <= body.y + body.h {
            let step_rect = Rect::new(body.x + pad, y, (body.w - pad * 2.0).max(0.0), step_h);
            let step_selected = selected_id == Some(step.id.as_str());
            ctx.draw.push_rounded([step_rect.x, step_rect.y, step_rect.w, step_rect.h], if step_selected { theme.selected } else { theme.button }, theme.border_radius);
            // 🖼️ Full four-side stroke, matching `StepCard`'s `border border-border` box in
            // `index.tsx` — previously only top/bottom hairlines were drawn, so cards had no visible
            // left/right edge.
            draw_ink_rect_outline(ctx.draw, step_rect.x, step_rect.y, step_rect.w, step_rect.h, theme.border_normal, theme.stroke_hairline);

            let title_color = if step_selected { theme.active_foreground } else { theme.text };
            draw_text(ctx, &step.title, step_rect.x + pad, step_rect.y + row_h * 0.65, theme.font_size_body, title_color);

            let btn_y = step_rect.y + (row_h - theme.control_height_small) * 0.5;
            let up_rect = Rect::new(step_rect.x + step_rect.w - pad - btn_w * 3.0, btn_y, btn_w, theme.control_height_small);
            let down_rect = Rect::new(step_rect.x + step_rect.w - pad - btn_w * 2.0, btn_y, btn_w, theme.control_height_small);
            let remove_rect = Rect::new(step_rect.x + step_rect.w - pad - btn_w, btn_y, btn_w, theme.control_height_small);
            render_widget(
                &WidgetNode::Button {
                    id: Some(format!("{}.step.{}.moveUp", scene.surface_id, step.id)),
                    icon_id: Some("chevron-up".into()),
                    label: String::new(),
                    event: (step_index > 0).then(|| scene_action(scene, "moveStep", json!({ "stepId": step.id, "index": step_index - 1 }))),
                },
                up_rect,
                ctx,
            );
            render_widget(
                &WidgetNode::Button {
                    id: Some(format!("{}.step.{}.moveDown", scene.surface_id, step.id)),
                    icon_id: Some("chevron-down".into()),
                    label: String::new(),
                    event: (step_index + 1 < step_count).then(|| scene_action(scene, "moveStep", json!({ "stepId": step.id, "index": step_index + 1 }))),
                },
                down_rect,
                ctx,
            );
            render_widget(
                &WidgetNode::Button { id: Some(format!("{}.step.{}.remove", scene.surface_id, step.id)), icon_id: Some("trash-2".into()), label: String::new(), event: Some(scene_action(scene, "removeStep", json!({ "stepId": step.id }))) },
                remove_rect,
                ctx,
            );

            let mut inner_y = step_rect.y + row_h;
            if let Some(description) = &step.description {
                draw_text(ctx, description, step_rect.x + pad, inner_y + theme.font_size_small, theme.font_size_small, theme.text_muted);
                inner_y += description_h;
            }
            for (block_index, block) in step.blocks.iter().enumerate() {
                let block_rect = Rect::new(step_rect.x + pad, inner_y, (step_rect.w - pad * 2.0).max(0.0), row_h);
                let block_selected = selected_id == Some(block.id.as_str());
                if block_selected {
                    ctx.draw.push_rounded([block_rect.x, block_rect.y, block_rect.w, block_rect.h], theme.selected, theme.border_radius.min(4.0));
                }
                let block_color = if block_selected { theme.active_foreground } else { theme.text };
                draw_text(ctx, &block.label, block_rect.x + pad, block_rect.y + row_h * 0.65, theme.font_size_small, block_color);
                draw_text(ctx, &block.kind, block_rect.x + block_rect.w * 0.5, block_rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);

                let bblock_btn_y = block_rect.y + (row_h - theme.control_height_small) * 0.5;
                let bup_rect = Rect::new(block_rect.x + block_rect.w - pad - btn_w * 3.0, bblock_btn_y, btn_w, theme.control_height_small);
                let bdown_rect = Rect::new(block_rect.x + block_rect.w - pad - btn_w * 2.0, bblock_btn_y, btn_w, theme.control_height_small);
                let bremove_rect = Rect::new(block_rect.x + block_rect.w - pad - btn_w, bblock_btn_y, btn_w, theme.control_height_small);
                render_widget(
                    &WidgetNode::Button {
                        id: Some(format!("{}.block.{}.moveUp", scene.surface_id, block.id)),
                        icon_id: Some("chevron-up".into()),
                        label: String::new(),
                        event: (block_index > 0).then(|| scene_action(scene, "moveBlock", json!({ "blockId": block.id, "fromStepId": step.id, "toStepId": step.id, "index": block_index - 1 }))),
                    },
                    bup_rect,
                    ctx,
                );
                render_widget(
                    &WidgetNode::Button {
                        id: Some(format!("{}.block.{}.moveDown", scene.surface_id, block.id)),
                        icon_id: Some("chevron-down".into()),
                        label: String::new(),
                        event: (block_index + 1 < block_count).then(|| scene_action(scene, "moveBlock", json!({ "blockId": block.id, "fromStepId": step.id, "toStepId": step.id, "index": block_index + 1 }))),
                    },
                    bdown_rect,
                    ctx,
                );
                render_widget(
                    &WidgetNode::Button {
                        id: Some(format!("{}.block.{}.remove", scene.surface_id, block.id)),
                        icon_id: Some("trash-2".into()),
                        label: String::new(),
                        event: Some(scene_action(scene, "removeBlock", json!({ "stepId": step.id, "blockId": block.id }))),
                    },
                    bremove_rect,
                    ctx,
                );
                inner_y += row_h;
            }
        }
        y += step_h + theme.gap_standard;
    }
    ctx.draw.pop_scissor();
    //#endregion Steps

    //#region Palette
    ctx.draw.push_line(palette_rect.x, palette_rect.y, palette_rect.x, palette_rect.y + palette_rect.h, theme.separator, theme.stroke_hairline);
    draw_text(ctx, "Palette", palette_rect.x + pad, palette_rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    let mut py = palette_rect.y + row_h;
    for entry in &palette {
        let entry_rect = Rect::new(palette_rect.x + pad, py, (palette_rect.w - pad * 2.0).max(0.0), row_h);
        render_widget(
            &WidgetNode::Button {
                id: Some(format!("{}.palette.{}", scene.surface_id, entry.block_kind)),
                icon_id: IconName::from_str(&entry.icon_id),
                label: entry.label.clone(),
                event: Some(scene_action(scene, "addBlock", json!({ "kind": entry.block_kind }))),
            },
            entry_rect,
            ctx,
        );
        py += row_h + 2.0;
    }
    //#endregion Palette
}

#[cfg(test)]
fn diff_lines<'a>(before: &[&'a str], after: &[&'a str]) -> Vec<DiffLine<'a>> {
    let (n, m) = (before.len(), after.len());
    if n.saturating_mul(m) > DIFF_LCS_CELL_BUDGET {
        let mut out = Vec::with_capacity(n + m);
        for i in 0..n.max(m) {
            match (before.get(i).copied(), after.get(i).copied()) {
                (Some(b), Some(a)) if b == a => out.push(DiffLine { operation: DiffLineOperation::Equal, text: b }),
                (Some(b), Some(a)) => {
                    out.push(DiffLine { operation: DiffLineOperation::Removed, text: b });
                    out.push(DiffLine { operation: DiffLineOperation::Added, text: a });
                }
                (Some(b), None) => out.push(DiffLine { operation: DiffLineOperation::Removed, text: b }),
                (None, Some(a)) => out.push(DiffLine { operation: DiffLineOperation::Added, text: a }),
                (None, None) => {}
            }
        }
        return out;
    }
    let mut table = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            table[i][j] = if before[i] == after[j] { table[i + 1][j + 1] + 1 } else { table[i + 1][j].max(table[i][j + 1]) };
        }
    }
    let mut out = Vec::with_capacity(n + m);
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if before[i] == after[j] {
            out.push(DiffLine { operation: DiffLineOperation::Equal, text: before[i] });
            i += 1;
            j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i] });
            i += 1;
        } else {
            out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j] });
            j += 1;
        }
    }
    while i < n {
        out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i] });
        i += 1;
    }
    while j < m {
        out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j] });
        j += 1;
    }
    out
}

#[cfg(test)]
fn render_diff_view(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(diff) = &scene.diff_view else {
        return render_placeholder("diff-view", bounds, ctx);
    };
    let before_lines: Vec<&str> = diff.before.split('\n').collect();
    let after_lines: Vec<&str> = diff.after.split('\n').collect();
    let operations = diff_lines(&before_lines, &after_lines);
    let inner = bounds;
    let pad = theme.padding_standard;
    let row_h = theme.font_size_small + pad * 0.5;
    let split = diff.mode.as_deref() == Some("split");

    let scroll = scroll_offset(&scene.surface_id, "diff");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "diff")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    if operations.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        ctx.draw.pop_scissor();
        return;
    }

    let col_w = if split { (inner.w * 0.5).max(1.0) } else { inner.w };
    let right_x = inner.x + col_w;
    for (row_index, line) in operations.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        if split {
            match line.operation {
                DiffLineOperation::Removed => {
                    draw_text(ctx, line.text, inner.x + pad, y + row_h * 0.7, theme.font_size_small, theme.error);
                }
                DiffLineOperation::Added => {
                    draw_text(ctx, line.text, right_x + pad, y + row_h * 0.7, theme.font_size_small, theme.accent);
                }
                DiffLineOperation::Equal => {
                    draw_text(ctx, line.text, inner.x + pad, y + row_h * 0.7, theme.font_size_small, theme.text);
                    draw_text(ctx, line.text, right_x + pad, y + row_h * 0.7, theme.font_size_small, theme.text);
                }
            }
            ctx.draw.push_line(right_x, y, right_x, y + row_h, theme.separator, theme.stroke_hairline);
        } else {
            let (marker, color) = match line.operation {
                DiffLineOperation::Added => ('+', theme.accent),
                DiffLineOperation::Removed => ('-', theme.error),
                DiffLineOperation::Equal => (' ', theme.text),
            };
            draw_text(ctx, &format!("{marker} {}", line.text), inner.x + pad, y + row_h * 0.7, theme.font_size_small, color);
        }
    }
    ctx.draw.pop_scissor();
}

#[cfg(test)]
fn event_feed_time_of_day_utc(timestamp_ms: i64) -> String {
    let ms_in_day = timestamp_ms.rem_euclid(86_400_000);
    let total_seconds = ms_in_day / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

#[cfg(test)]
fn event_feed_tone_color(tone: Option<&str>, theme: &Theme) -> Rgba {
    match tone {
        Some("error") | Some("danger") => theme.error,
        Some("success") | Some("positive") => theme.accent,
        Some("pending") | Some("warning") => theme.temporary,
        _ => theme.text_muted,
    }
}

#[cfg(test)]
fn event_feed_row_height(entry: &EventFeedEntryJson, row_h: f32, theme: &Theme) -> f32 {
    row_h + entry.detail.as_ref().map_or(0.0, |_| theme.font_size_small + theme.padding_standard * 0.25)
}

#[cfg(test)]
fn render_event_feed(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(feed) = &scene.event_feed else {
        return render_placeholder("event-feed", bounds, ctx);
    };
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let inner = bounds;
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    if entries.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }

    let heights: Vec<f32> = entries.iter().map(|entry| event_feed_row_height(entry, row_h, theme)).collect();
    let content_h: f32 = heights.iter().sum();
    if feed.follow.unwrap_or(false) {
        set_scroll_offset(&scene.surface_id, "feed", (content_h - inner.h).max(0.0));
    }
    let scroll = scroll_offset(&scene.surface_id, "feed");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "feed")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    let hovered_row = ctx.input.hovered_id.clone();
    let mut y = inner.y - scroll;
    for entry in entries.iter() {
        let entry_h = event_feed_row_height(entry, row_h, theme);
        if y + entry_h < inner.y || y > inner.y + inner.h {
            y += entry_h;
            continue;
        }
        let control_id = format!("{}.feed.{}", scene.surface_id, entry.id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, entry_h);
        if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);

        let tone_color = event_feed_tone_color(entry.tone.as_deref(), theme);
        // ℹ `FEED_TONE_CLASS`'s `info` case is `text-foreground`, not a muted tone — only
        // success/warning/error get an actual tone color on the title.
        let title_tone_color = match entry.tone.as_deref() {
            None | Some("info") => theme.text,
            _ => tone_color,
        };
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([inner.x + pad, dot_y - 3.0, 6.0, 6.0], tone_color, 3.0);
        let mut title_x = inner.x + pad + 6.0 + pad * 0.5;

        if !entry.icon_id.is_empty() {
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(&entry.icon_id) {
                    ctx.draw.push_textured([title_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, theme.text_element);
                    title_x += 14.0 + pad * 0.5;
                }
            }
        }

        if entry.timestamp_ms != 0 {
            let time_label = event_feed_time_of_day_utc(entry.timestamp_ms);
            draw_text(ctx, &time_label, title_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
            title_x += 56.0;
        }
        // 🎨️ `FEED_TONE_CLASS` tints the title span itself in `event-feed-host.tsx`
        // (info→foreground, success/warning/error→their tone color) — plain `theme.text` here
        // previously dropped the tone cue from the one place React actually shows it.
        draw_text(ctx, &entry.title, title_x, y + row_h * 0.65, theme.font_size_small, title_tone_color);
        if let Some(detail) = &entry.detail {
            draw_text(ctx, detail, inner.x + pad, y + row_h + theme.font_size_small * 0.9, theme.font_size_small, theme.text_muted);
        }
        if let Some(action) = &feed.activate_action {
            ctx.input.register_hit(HitTarget { rect: row_rect, event: Some(scene_action(scene, action, json!({ "entryId": entry.id }))), control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data: None });
        }
        y += entry_h;
    }
    ctx.draw.pop_scissor();
}

#[cfg(test)]
fn history_lane_count(columns: &[HistoryColumnJson]) -> usize {
    columns.iter().map(|column| column.lane + 1).max().unwrap_or(1).max(1)
}

#[cfg(test)]
fn history_graph_width(lane_count: usize) -> f32 {
    (HISTORY_LANE_PAD * 2.0 + lane_count as f32 * HISTORY_LANE_PITCH).max(56.0)
}

#[cfg(test)]
fn history_lane_x(lane: usize, lane_count: usize, graph_width: f32) -> f32 {
    if lane_count <= 1 {
        return graph_width * 0.5;
    }
    HISTORY_LANE_PAD + lane as f32 * HISTORY_LANE_PITCH + HISTORY_LANE_PITCH * 0.5
}

#[cfg(test)]
fn history_row_lane_guides(columns: &[HistoryColumnJson], lane_count: usize) -> Vec<Vec<bool>> {
    let mut guides = vec![vec![false; lane_count]; columns.len()];
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();
    for (row_index, column) in columns.iter().enumerate() {
        if column.lane < lane_count {
            guides[row_index][column.lane] = true;
        }
        let Some(parent_row) = column.parent_checkpoint_id.as_deref().and_then(|id| row_by_id.get(id).copied()) else {
            continue;
        };
        let parent_lane = columns[parent_row].lane;
        if column.lane == parent_lane {
            for row in (row_index + 1)..parent_row {
                guides[row][column.lane] = true;
            }
            continue;
        }
        let elbow_row = if row_index + 1 < parent_row { row_index + 1 } else { parent_row };
        for row in (row_index + 1)..=elbow_row {
            if column.lane < lane_count {
                guides[row][column.lane] = true;
            }
        }
        for row in elbow_row..parent_row {
            if parent_lane < lane_count {
                guides[row][parent_lane] = true;
            }
        }
    }
    guides
}

#[cfg(test)]
fn graph_timeline_avatar_initials(name: &str) -> String {
    let letters: String = name.split_whitespace().filter_map(|word| word.chars().next()).take(2).flat_map(char::to_uppercase).collect();
    if letters.is_empty() {
        "?".to_string()
    } else {
        letters
    }
}

#[cfg(test)]
fn render_graph_timeline(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(history) = &scene.graph_timeline else {
        return render_placeholder("graph-timeline", bounds, ctx);
    };
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let inner = bounds;
    let row_h = theme.control_height * 1.33;
    let pad = theme.padding_standard;
    if columns.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }
    let lane_count = history_lane_count(&columns);
    let graph_width = history_graph_width(lane_count);
    let graph_col_w = graph_width + HISTORY_AUTHOR_SLOT;
    let labels_col_w = (inner.w * 0.28).max(96.0);
    let guides = history_row_lane_guides(&columns, lane_count);
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();

    let scroll = scroll_offset(&scene.surface_id, "history");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "history")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    let hovered_row = ctx.input.hovered_id.clone();
    let graph_x0 = inner.x + labels_col_w;
    let desc_x = inner.x + labels_col_w + graph_col_w;

    for (row_index, column) in columns.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        let control_id = format!("{}.history.{}", scene.surface_id, column.checkpoint_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, row_h);
        if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);

        let mut label_x = inner.x + pad;
        if column.labels.is_empty() {
            draw_text(ctx, "checkpoint", label_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        } else {
            for label in &column.labels {
                let chip_w = (label.len() as f32 * 6.0 + pad * 2.0).min((inner.x + labels_col_w - label_x).max(0.0));
                if chip_w <= 0.0 {
                    break;
                }
                ctx.draw.push_rounded([label_x, y + row_h * 0.5 - 9.0, chip_w, 18.0], theme.accent, 4.0);
                draw_text(ctx, label, label_x + 4.0, y + row_h * 0.5 + 4.0, theme.font_size_small, theme.active_foreground);
                label_x += chip_w + 4.0;
            }
        }

        // 🪢️ `color-mix(in oklab, var(--muted-foreground) 40%, transparent)` on the lane guides and
        // parent connectors in `graph-timeline-host.tsx` — a translucent line; opaque `theme.separator`
        // previously read as visibly heavier than React's thin, faded rail.
        let guide_stroke = theme.separator.with_alpha(theme.separator.a * 0.4);
        for lane in 0..lane_count {
            if guides[row_index][lane] {
                let lx = graph_x0 + history_lane_x(lane, lane_count, graph_width);
                ctx.draw.push_line(lx, y, lx, y + row_h, guide_stroke, 1.0);
            }
        }
        if let Some(parent_id) = column.parent_checkpoint_id.as_deref() {
            if let Some(&parent_row) = row_by_id.get(parent_id) {
                let x0 = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
                let parent_lane = columns[parent_row].lane;
                let x1 = graph_x0 + history_lane_x(parent_lane, lane_count, graph_width);
                let y0 = y + row_h * 0.5;
                let y1 = inner.y + parent_row as f32 * row_h - scroll + row_h * 0.5;
                if (x0 - x1).abs() < 0.5 {
                    ctx.draw.push_line(x0, y0, x1, y1, guide_stroke, 1.5);
                } else {
                    let elbow_y = y + row_h;
                    ctx.draw.push_line(x0, y0, x0, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x0, elbow_y, x1, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x1, elbow_y, x1, y1, guide_stroke, 1.5);
                }
            }
        }
        let dot_x = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([dot_x - 3.0, dot_y - 3.0, 6.0, 6.0], theme.text, 3.0);

        let avatar_size = 20.0;
        let avatar_x = graph_x0 + graph_width + 4.0;
        let avatar_y = y + row_h * 0.5 - avatar_size * 0.5;
        let initial = column.authors.first().map(|author| graph_timeline_avatar_initials(&author.name)).unwrap_or_else(|| "?".into());
        ctx.draw.push_rounded([avatar_x, avatar_y, avatar_size, avatar_size], theme.button, avatar_size * 0.5);
        let initial_x_frac = if initial.chars().count() >= 2 { 0.18 } else { 0.32 };
        draw_text(ctx, &initial, avatar_x + avatar_size * initial_x_frac, avatar_y + avatar_size * 0.7, theme.font_size_small, theme.text);

        if let Some(description) = &column.description {
            draw_text(ctx, description, desc_x + pad, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        }

        ctx.input.register_hit(HitTarget {
            rect: row_rect,
            event: Some(scene_action(scene, "checkoutCheckpoint", json!({ "checkpointId": column.checkpoint_id }))),
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data: None,
        });
    }
    ctx.draw.pop_scissor();
}

#[cfg(test)]
fn canvas2d_packet_text_size() -> f64 {
    11.0
}

#[cfg(test)]
fn canvas_layer_should_render(layer: &CanvasLayer) -> bool {
    layer.role.as_deref() != Some("meta") && layer.visible.unwrap_or(true)
}

#[cfg(test)]
fn decode_canvas_image_source(data_url: &str) -> Option<Vec<u8>> {
    let payload = data_url.strip_prefix("data:image/png;base64,").or_else(|| data_url.strip_prefix("data:image/jpeg;base64,")).unwrap_or(data_url);
    base64::engine::general_purpose::STANDARD.decode(payload).ok()
}

#[cfg(test)]
fn decode_canvas_image_bytes(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let image = image::load_from_memory(&bytes).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some((rgba.into_raw(), width, height))
}

#[cfg(test)]
pub(crate) fn queue_canvas_image_upload_sized(surface_id: &str, layer_id: &str, data_url: &str) -> (Option<String>, Option<(u32, u32)>) {
    let dimensions = std::cell::Cell::new(None);
    let key = queue_canvas_image_upload_with(
        surface_id,
        layer_id,
        data_url.as_bytes(),
        || {
            let bytes = decode_canvas_image_source(data_url).ok_or_else(Vec::new)?;
            let measured = match image::ImageReader::new(std::io::Cursor::new(bytes.as_slice())).with_guessed_format().ok().and_then(|reader| reader.into_dimensions().ok()) {
                Some(measured) => measured,
                None => return Err(bytes),
            };
            dimensions.set(Some(measured));
            Ok((measured.0, measured.1, bytes))
        },
        |bytes| decode_canvas_image_bytes(bytes).map(|(pixels, _, _)| pixels),
    );
    (key, dimensions.get())
}

#[cfg(test)]
pub(crate) fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    queue_canvas_image_upload_sized(surface_id, layer_id, data_url).0
}

#[cfg(test)]
fn draw_checkerboard(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme, extent: f32) {
    let cell = 16.0;
    let half = extent * 0.5;
    let light = theme.checker_light;
    let dark = theme.checker_dark;
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (-half, half, -half, half);
    if viewport.zoom > 0.0 {
        let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
        let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
        min_x = min_x.max(wx0.min(wx1) - cell);
        max_x = max_x.min(wx0.max(wx1) + cell);
        min_y = min_y.max(wy0.min(wy1) - cell);
        max_y = max_y.min(wy0.max(wy1) + cell);
    }
    let start_row = ((min_y - (-half)) / cell).floor().max(0.0) as i64;
    let start_col = ((min_x - (-half)) / cell).floor().max(0.0) as i64;
    let mut row = start_row;
    let mut wy = -half + start_row as f32 * cell;
    while wy < max_y {
        let mut col = start_col;
        let mut wx = -half + start_col as f32 * cell;
        while wx < max_x {
            let color = if (row + col) % 2 == 0 { light } else { dark };
            let (sx, sy) = viewport.world_to_screen(wx, wy, inner);
            let (sx1, sy1) = viewport.world_to_screen(wx + cell, wy + cell, inner);
            let w = (sx1 - sx).abs().max(1.0);
            let h = (sy1 - sy).abs().max(1.0);
            draw.push_solid([sx.min(sx1), sy.min(sy1), w, h], color);
            wx += cell;
            col += 1;
        }
        wy += cell;
        row += 1;
    }
}

#[cfg(test)]
fn draw_canvas_infinite_grid(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme) {
    if viewport.zoom <= 0.0 {
        return;
    }
    let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
    let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
    let min_x = wx0.min(wx1);
    let max_x = wx0.max(wx1);
    let min_y = wy0.min(wy1);
    let max_y = wy0.max(wy1);
    let color = theme.separator.with_alpha((theme.separator.a * 0.35).max(0.08));
    let steps: [(f32, f32, f32); 4] = [(10.0, 1.0, 0.0), (2.5, 0.72, 8.0), (0.5, 0.48, 10.0), (0.1, 0.32, 12.0)];
    for (world_step, stroke_px, min_screen) in steps {
        let screen = world_step * viewport.zoom;
        if screen < min_screen {
            continue;
        }
        let width = (stroke_px).max(0.5);
        let start_x = (min_x / world_step).floor() * world_step;
        let start_y = (min_y / world_step).floor() * world_step;
        let mut x = start_x;
        while x <= max_x {
            let (sx0, sy0) = viewport.world_to_screen(x, min_y, inner);
            let (sx1, sy1) = viewport.world_to_screen(x, max_y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            x += world_step;
        }
        let mut y = start_y;
        while y <= max_y {
            let (sx0, sy0) = viewport.world_to_screen(min_x, y, inner);
            let (sx1, sy1) = viewport.world_to_screen(max_x, y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            y += world_step;
        }
    }
}

#[cfg(test)]
fn draw_dashed_line(draw: &mut ui_wgpu::wgpu::DrawList, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let dash = 4.0f32;
    let gap = 4.0f32;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            let sx0 = x0 + ux * traveled;
            let sy0 = y0 + uy * traveled;
            let sx1 = x0 + ux * next;
            let sy1 = y0 + uy * next;
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
        }
        traveled = next;
        drawing = !drawing;
    }
}

#[cfg(test)]
fn canvas_color_channels(v: &[f64], opacity: f32) -> Rgba {
    let r = v.first().copied().unwrap_or(0.58) as f32;
    let g = v.get(1).copied().unwrap_or(0.64) as f32;
    let b = v.get(2).copied().unwrap_or(0.72) as f32;
    let a = v.get(3).copied().unwrap_or(1.0) as f32;
    Rgba::new(r, g, b, (a * opacity).clamp(0.0, 1.0))
}

#[cfg(test)]
fn canvas_mix_rgba(a: Rgba, b: Rgba, t: f32) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    Rgba::new(a.r + (b.r - a.r) * t, a.g + (b.g - a.g) * t, a.b + (b.b - a.b) * t, a.a + (b.a - a.a) * t)
}

#[cfg(test)]
fn canvas_gradient_color_at(stops: &[CanvasGradientStopJson], t: f32, opacity: f32) -> Rgba {
    if stops.is_empty() {
        return Rgba::new(0.58, 0.64, 0.72, 0.95 * opacity);
    }
    let t = t.clamp(0.0, 1.0);
    if stops.len() == 1 {
        return canvas_color_channels(stops[0].color.as_deref().unwrap_or(&[]), opacity);
    }
    let last = stops.len() - 1;
    for i in 0..last {
        let a_off = stops[i].offset.clamp(0.0, 1.0) as f32;
        let b_off = stops[i + 1].offset.clamp(0.0, 1.0) as f32;
        if t <= b_off || i == last - 1 {
            let span = (b_off - a_off).max(0.0001);
            let local = ((t - a_off) / span).clamp(0.0, 1.0);
            let ca = canvas_color_channels(stops[i].color.as_deref().unwrap_or(&[]), opacity);
            let cb = canvas_color_channels(stops[i + 1].color.as_deref().unwrap_or(&[]), opacity);
            return canvas_mix_rgba(ca, cb, local);
        }
    }
    canvas_color_channels(stops[last].color.as_deref().unwrap_or(&[]), opacity)
}

#[cfg(test)]
fn canvas_blend_channel(mode: &str, cb: f32, cs: f32) -> f32 {
    match mode {
        "multiply" => cb * cs,
        "screen" => cb + cs - cb * cs,
        "overlay" => canvas_blend_channel("hardLight", cs, cb),
        "darken" => cb.min(cs),
        "lighten" => cb.max(cs),
        "colorDodge" => {
            if cb <= 0.0 {
                0.0
            } else if cs >= 1.0 {
                1.0
            } else {
                (cb / (1.0 - cs)).min(1.0)
            }
        }
        "colorBurn" => {
            if cb >= 1.0 {
                1.0
            } else if cs <= 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - cb) / cs).min(1.0)
            }
        }
        "hardLight" => {
            if cs <= 0.5 {
                2.0 * cb * cs
            } else {
                1.0 - 2.0 * (1.0 - cb) * (1.0 - cs)
            }
        }
        "softLight" => {
            let d = if cb <= 0.25 { ((16.0 * cb - 12.0) * cb + 4.0) * cb } else { cb.sqrt() };
            if cs <= 0.5 {
                cb - (1.0 - 2.0 * cs) * cb * (1.0 - cb)
            } else {
                cb + (2.0 * cs - 1.0) * (d - cb)
            }
        }
        "difference" => (cb - cs).abs(),
        "exclusion" => cb + cs - 2.0 * cb * cs,
        _ => cs,
    }
}

#[cfg(test)]
fn canvas_apply_blend_mode(mode: Option<&str>, backdrop: Rgba, source: Rgba) -> Rgba {
    let mode = match mode {
        None | Some("") | Some("normal") => return source,
        Some(m) => m,
    };
    let cb = [backdrop.r, backdrop.g, backdrop.b];
    let cs = [source.r, source.g, source.b];
    let blended = match mode {
        "hue" => canvas_set_lum(canvas_set_sat(cs, canvas_sat(cb)), canvas_lum(cb)),
        "saturation" => canvas_set_lum(canvas_set_sat(cb, canvas_sat(cs)), canvas_lum(cb)),
        "color" => canvas_set_lum(cs, canvas_lum(cb)),
        "luminosity" => canvas_set_lum(cb, canvas_lum(cs)),
        _ => [canvas_blend_channel(mode, cb[0], cs[0]), canvas_blend_channel(mode, cb[1], cs[1]), canvas_blend_channel(mode, cb[2], cs[2])],
    };
    Rgba::new(blended[0].clamp(0.0, 1.0), blended[1].clamp(0.0, 1.0), blended[2].clamp(0.0, 1.0), source.a)
}

#[cfg(test)]
fn push_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, is_circle: bool) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        draw.push_triangle_fan(&canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    } else {
        draw.push_rounded([rect.x, rect.y, rect.w, rect.h], color, 4.0);
    }
}

#[cfg(test)]
fn push_circle_outline(draw: &mut ui_wgpu::wgpu::DrawList, cx: f32, cy: f32, radius: f32, color: Rgba, width: f32) {
    let points = canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS);
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        draw.push_line(a[0], a[1], b[0], b[1], color, width);
    }
}

#[cfg(test)]
fn push_shape_outline(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, width: f32, is_circle: bool, dash: Option<&[f64]>) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        push_circle_outline(draw, cx, cy, radius, color, width);
        return;
    }
    if dash.is_some_and(|d| !d.is_empty()) {
        draw_dashed_line(draw, rect.x, rect.y, rect.x + rect.w, rect.y, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y, rect.x + rect.w, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y + rect.h, rect.x, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x, rect.y + rect.h, rect.x, rect.y, color, width);
    } else {
        draw_ink_rect_outline(draw, rect.x, rect.y, rect.w, rect.h, color, width);
    }
}

#[cfg(test)]
fn push_linear_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (sx1, sy1) = viewport.world_to_screen((origin_x + fill.x1) as f32, (origin_y + fill.y1) as f32, inner);
    let (sx2, sy2) = viewport.world_to_screen((origin_x + fill.x2) as f32, (origin_y + fill.y2) as f32, inner);
    let dx = sx2 - sx1;
    let dy = sy2 - sy1;
    let len = (dx * dx + dy * dy).sqrt();
    draw.push_scissor(clip);
    if len < 0.5 {
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, 1.0, opacity));
        draw.push_rounded([clip.x, clip.y, clip.w, clip.h], color, 0.0);
        draw.pop_scissor();
        return;
    }
    let ux = dx / len;
    let uy = dy / len;
    let px = -uy;
    let py = ux;
    let overhang = (clip.w + clip.h).max(32.0);
    for i in 0..CANVAS_GRADIENT_BANDS {
        let t0 = i as f32 / CANVAS_GRADIENT_BANDS as f32;
        let t1 = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, (t0 + t1) * 0.5, opacity));
        let a0 = if i == 0 { -overhang } else { t0 * len };
        let a1 = if i == CANVAS_GRADIENT_BANDS - 1 { len + overhang } else { t1 * len };
        let bx = sx1 + ux * a0;
        let by = sy1 + uy * a0;
        let ex = sx1 + ux * a1;
        let ey = sy1 + uy * a1;
        let points = [[bx + px * overhang, by + py * overhang], [ex + px * overhang, ey + py * overhang], [ex - px * overhang, ey - py * overhang], [bx - px * overhang, by - py * overhang]];
        draw.push_triangle_fan(&points, color);
    }
    draw.pop_scissor();
}

#[cfg(test)]
fn push_radial_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (scx, scy) = viewport.world_to_screen((origin_x + fill.cx) as f32, (origin_y + fill.cy) as f32, inner);
    let sr = (fill.r as f32 * viewport.zoom).max(0.5);
    draw.push_scissor(clip);
    let outer_radius = sr.max(clip.w.max(clip.h));
    for i in (0..CANVAS_GRADIENT_BANDS).rev() {
        let t = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let radius = if i == CANVAS_GRADIENT_BANDS - 1 { outer_radius } else { sr * t };
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, t, opacity));
        draw.push_triangle_fan(&canvas_circle_points(scx, scy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    }
    draw.pop_scissor();
}

#[cfg(test)]
fn render_canvas_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, shape_rect: Rect, layer: &CanvasLayer, opacity: f32, fallback_fill: Rgba, backdrop: Rgba, is_circle: bool) {
    let blend = layer.blend_mode.as_deref();
    match &layer.fill {
        Some(fill) if fill.kind.as_deref() == Some("linearGradient") && !fill.stops.is_empty() => {
            push_linear_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.kind.as_deref() == Some("radialGradient") && !fill.stops.is_empty() => {
            push_radial_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.color.is_some() => {
            let solid = canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(fill.color.as_deref().unwrap_or(&[]), opacity));
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
        _ => {
            let solid = canvas_apply_blend_mode(blend, backdrop, fallback_fill);
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
    }
    if let Some(stroke) = &layer.stroke {
        let color = stroke.color.as_deref().map(|c| canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(c, opacity))).unwrap_or_else(|| Rgba::new(0.58, 0.64, 0.72, 0.9 * opacity));
        let width = (stroke.width.unwrap_or(1.0) as f32).max(1.0);
        push_shape_outline(draw, shape_rect, color, width, is_circle, stroke.dash.as_deref());
    }
}

#[cfg(test)]
fn render_canvas2d_packet_item(item: &Canvas2dPacketItem<'_>, viewport: &Viewport, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let color = if item.id.starts_with("residual-field-") {
        Rgba::new(0.918, 0.702, 0.031, 1.0)
    } else if item.id.starts_with("reaction-field-") || item.id.starts_with("load-") {
        Rgba::new(0.984, 0.443, 0.522, 1.0)
    } else if item.id.starts_with("displacement-field-") || item.id.starts_with("mode-field-") {
        Rgba::new(0.847, 0.42, 0.91, 1.0)
    } else {
        theme.diagram_accent
    };
    if item.kind == "line" {
        let (x0, y0) = viewport.world_to_screen(item.x0 as f32, item.y0 as f32, inner);
        let (x1, y1) = viewport.world_to_screen(item.x1 as f32, item.y1 as f32, inner);
        ctx.draw.push_line(x0, y0, x1, y1, color, (2.0 * viewport.zoom).max(1.0));
    } else if item.kind == "circle" {
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        let width = (item.width as f32 * viewport.zoom).max(4.0);
        let height = (item.height as f32 * viewport.zoom).max(4.0);
        ctx.draw.push_solid([x, y, width, height], color);
    } else if item.kind == "text" {
        let Some(text) = item.text.as_ref() else { return };
        let (x, y) = viewport.world_to_screen(item.x as f32, item.y as f32, inner);
        draw_text(ctx, text.content, x, y + text.size as f32, (text.size as f32).max(8.0), theme.text);
    }
}

#[cfg(test)]
fn render_canvas_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(canvas) = &scene.canvas_2d else {
        return render_placeholder("canvas-2d", bounds, ctx);
    };
    let inner = bounds;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let mut viewport = Viewport { x: canvas.camera_x as f32, y: canvas.camera_y as f32, zoom: canvas.zoom as f32 };
    let local = scene_state(&scene.surface_id);
    if local.viewport.zoom > 0.0 && scene.component_kind == SurfaceKind::Canvas2d {
        viewport = local.viewport;
    }
    draw_canvas_infinite_grid(ctx.draw, &viewport, inner, theme);
    if let Some(snapshot) = canvas.snapshot {
        for page_index in 0..snapshot.page_count {
            let _ = ui_wgpu::wgpu::canvas2d_snapshot_with_page(snapshot, page_index, |page| {
                for item in serde_json::Deserializer::from_slice(page.bytes()).into_iter::<Canvas2dPacketItem<'_>>().flatten() {
                    render_canvas2d_packet_item(&item, &viewport, inner, ctx);
                }
            });
        }
        ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
        return;
    }
    let layers: Vec<CanvasLayer> = serde_json::from_str(&canvas.layers_json).unwrap_or_default();
    let has_polyline = layers.iter().any(|layer| layer.kind == "polyline");
    if has_polyline {
        draw_checkerboard(ctx.draw, &viewport, inner, ctx.theme, 1024.0);
    }
    for (index, layer) in layers.iter().enumerate() {
        // 🗒️ `role === "meta"` (activeUtility bookkeeping) and `visible === false` records are
        // non-visual — skip rendering entirely, matches `layers.filter(role !== "meta")` in
        // `canvas-2d-host.tsx`'s `JsonLayersCanvasSession.renderFrame`.
        if !canvas_layer_should_render(layer) {
            continue;
        }
        let opacity = layer.opacity.unwrap_or(1.0).clamp(0.0, 1.0);
        let blend = layer.blend_mode.as_deref();
        if layer.kind == "image" {
            let source = layer.data_url.clone().or_else(|| layer.image.as_ref().and_then(|image| image.src.clone()));
            if let Some(data_url) = source.filter(|src| src.starts_with("data:")) {
                if let Some(key) = queue_canvas_image_upload(&scene.surface_id, &layer.id, &data_url) {
                    let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
                    let iw = layer.image.as_ref().and_then(|image| image.width).unwrap_or(layer.width);
                    let ih = layer.image.as_ref().and_then(|image| image.height).unwrap_or(layer.height);
                    let w = iw as f32 * viewport.zoom;
                    let h = ih as f32 * viewport.zoom;
                    ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], opacity);
                }
            }
            continue;
        }
        if layer.kind == "polyline" {
            if let Some(points) = &layer.points {
                let stroke = theme.diagram_stroke.with_alpha(theme.diagram_stroke.a * opacity);
                let seam_stroke = theme.diagram_seam.with_alpha(theme.diagram_seam.a * opacity);
                let width = (1.5 * viewport.zoom).max(1.0);
                for (edge_index, chunk) in points.chunks(2).enumerate() {
                    if chunk.len() < 2 {
                        continue;
                    }
                    let (x0, y0) = viewport.world_to_screen(chunk[0][0] as f32, chunk[0][1] as f32, inner);
                    let (x1, y1) = viewport.world_to_screen(chunk[1][0] as f32, chunk[1][1] as f32, inner);
                    let is_seam = layer.seams.as_ref().and_then(|seams| seams.get(edge_index)).copied().unwrap_or(0) != 0;
                    if is_seam {
                        draw_dashed_line(ctx.draw, x0, y0, x1, y1, seam_stroke, width);
                    } else {
                        ctx.draw.push_line(x0, y0, x1, y1, stroke, width);
                    }
                }
            }
            continue;
        }
        let hue = (index * 47 % 360) as f32;
        if layer.kind == "line" || layer.x0.is_some() {
            let x0 = layer.x0.unwrap_or(layer.x) as f32;
            let y0 = layer.y0.unwrap_or(layer.y) as f32;
            let x1 = layer.x1.unwrap_or(layer.x + layer.width) as f32;
            let y1 = layer.y1.unwrap_or(layer.y + layer.height) as f32;
            let (sx0, sy0) = viewport.world_to_screen(x0, y0, inner);
            let (sx1, sy1) = viewport.world_to_screen(x1, y1, inner);
            let base_stroke = layer
                .stroke
                .as_ref()
                .and_then(|stroke| stroke.color.as_deref())
                .map(|c| canvas_color_channels(c, opacity))
                .unwrap_or_else(|| Rgba::new(theme.diagram_accent.r + hue / 720.0, theme.diagram_accent.g, theme.diagram_accent.b, theme.diagram_accent.a * opacity));
            let stroke = canvas_apply_blend_mode(blend, theme.canvas_clear, base_stroke);
            ctx.draw.push_line(sx0, sy0, sx1, sy1, stroke, (2.0 * viewport.zoom).max(1.0));
            continue;
        }
        // 🖼️ Generic bounds-rect (or `kind === "circle"`) draw record — resolves solid/gradient
        // fill, blend-mode approximation, and stroke, matching `drawSceneNode`'s bounds-layer path.
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let w = (layer.width as f32 * viewport.zoom).max(8.0);
        let h = (layer.height as f32 * viewport.zoom).max(8.0);
        let shape_rect = Rect::new(sx, sy, w, h);
        let is_circle = layer.kind == "circle";
        let fallback_fill = Rgba::new(theme.diagram_accent_fill.r + hue / 720.0, theme.diagram_accent_fill.g, theme.diagram_accent_fill.b, theme.diagram_accent_fill.a * opacity);
        render_canvas_shape_fill(ctx.draw, &viewport, inner, shape_rect, layer, opacity, fallback_fill, theme.canvas_clear, is_circle);
        // 🖊️ Overlay annotation: a two-pass selection highlight (soft outer glow + crisp amber ring)
        // drawn on top of the shape, matches `drawBoundsLayer`'s `isSelected` glow+ring pair in
        // `canvas-2d-host.tsx` (glow at +4px/width 5, ring at +0px/width 2.5, both amber).
        if layer.selected.unwrap_or(false) {
            if is_circle {
                let cx = sx + w * 0.5;
                let cy = sy + h * 0.5;
                let r = w.min(h) * 0.5;
                push_circle_outline(ctx.draw, cx, cy, r + 4.0, CANVAS2D_SELECTION_GLOW, 5.0);
                push_circle_outline(ctx.draw, cx, cy, r, CANVAS2D_SELECTION_RING, 2.5);
            } else {
                draw_ink_rect_outline(ctx.draw, sx - 4.0, sy - 4.0, w + 8.0, h + 8.0, CANVAS2D_SELECTION_GLOW, 5.0);
                draw_ink_rect_outline(ctx.draw, sx, sy, w, h, CANVAS2D_SELECTION_RING, 2.5);
            }
        }
        if let Some(text) = layer.text.as_ref().and_then(|text| text.content.as_deref()) {
            let size = layer.text.as_ref().and_then(|t| t.size).unwrap_or(14.0) as f32;
            draw_text(ctx, text, sx + 2.0, sy + size.max(8.0), size.max(8.0), theme.text);
        } else {
            let label = if layer.name.is_empty() { layer.id.as_str() } else { layer.name.as_str() };
            if !label.is_empty() {
                draw_text(ctx, label, sx + 4.0, sy + 14.0, theme.font_size_small, theme.text);
            }
        }
    }
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
}

#[cfg(test)]
fn ink_effective_bounds(block: &Value, overrides: &HashMap<String, Value>) -> InkBoundsF {
    match overrides.get(ink_item_id(block)) {
        Some(over) => ink_item_bounds(over),
        None => ink_item_bounds(block),
    }
}

#[cfg(test)]
fn flatten_ink_items(blocks: &[Value]) -> Vec<&Value> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [Value], out: &mut Vec<&'a Value>) {
        for block in blocks {
            out.push(block);
            if ink_item_kind(block) == "group" {
                if let Some(children) = block.get("children").and_then(Value::as_array) {
                    visit(children, out);
                }
            }
        }
    }
    visit(blocks, &mut out);
    out
}

#[cfg(test)]
fn find_ink_item<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
    flatten_ink_items(blocks).into_iter().find(|block| ink_item_id(block) == id)
}

#[cfg(test)]
fn ink_items_at_point<'a>(blocks: &'a [Value], overrides: &HashMap<String, Value>, x: f64, y: f64) -> Vec<&'a Value> {
    let mut flat = flatten_ink_items(blocks);
    flat.reverse();
    flat.into_iter().filter(|block| ink_effective_bounds(block, overrides).contains_point(x, y)).collect()
}

#[cfg(test)]
fn ink_items_intersecting_rect(blocks: &[Value], overrides: &HashMap<String, Value>, rect: InkBoundsF) -> Vec<String> {
    flatten_ink_items(blocks).into_iter().filter(|block| ink_effective_bounds(block, overrides).intersects(&rect)).map(|block| ink_item_id(block).to_string()).collect()
}

#[cfg(test)]
fn ink_selection_bounds(blocks: &[Value], overrides: &HashMap<String, Value>, ids: &[String]) -> Option<InkBoundsF> {
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let selected: Vec<InkBoundsF> = flatten_ink_items(blocks).into_iter().filter(|block| id_set.contains(ink_item_id(block))).map(|block| ink_effective_bounds(block, overrides)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for bounds in &selected {
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.w);
        max_y = max_y.max(bounds.y + bounds.h);
    }
    Some(InkBoundsF { x: min_x, y: min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) })
}

#[cfg(test)]
fn ink_maybe_snap(doc: &InkDocumentJson, x: f64, y: f64) -> (f64, f64) {
    ink_maybe_snap_fields(doc.snap_enabled, doc.snap_grid_spacing, x, y)
}

#[cfg(test)]
fn ink_text_plain(block: &Value) -> String {
    block
        .get("paragraphs")
        .and_then(Value::as_array)
        .map(|paragraphs| {
            paragraphs.iter().map(|paragraph| paragraph.get("runs").and_then(Value::as_array).map(|runs| runs.iter().filter_map(|run| run.get("text").and_then(Value::as_str)).collect::<String>()).unwrap_or_default()).collect::<Vec<_>>().join("\n")
        })
        .unwrap_or_default()
}

#[cfg(test)]
fn erase_ink_stroke_events(blocks: &[Value], x: f64, y: f64, threshold: f64) -> Vec<Value> {
    flatten_ink_items(blocks).into_iter().filter(|block| ink_item_kind(block) == "stroke" && ink_hits_point(block, x, y, threshold)).map(|block| json!({ "operation": "removeBlock", "blockId": ink_item_id(block) })).collect()
}

#[cfg(test)]
fn erase_ink_stroke_points_events(blocks: &[Value], x: f64, y: f64, radius: f64) -> Vec<Value> {
    let mut events = Vec::new();
    for block in flatten_ink_items(blocks) {
        if ink_item_kind(block) != "stroke" {
            continue;
        }
        let fragments = erase_ink_stroke_points_in_item(block, x, y, radius);
        if fragments.len() == 1 && fragments[0] == *block {
            continue;
        }
        events.push(json!({ "operation": "removeBlock", "blockId": ink_item_id(block) }));
        for fragment in fragments {
            events.push(json!({ "operation": "addBlock", "block": fragment }));
        }
    }
    events
}

#[cfg(test)]
fn ink_world_to_screen(camera: InkCameraF, inner: Rect, wx: f64, wy: f64) -> (f32, f32) {
    (inner.x + (wx * camera.zoom + camera.x) as f32, inner.y + (wy * camera.zoom + camera.y) as f32)
}

#[cfg(test)]
fn ink_current_camera(scene: &UiComponentSceneNode) -> InkCameraF {
    let state = scene_state(&scene.surface_id);
    if let Some((x, y, zoom)) = state.ink_camera {
        return InkCameraF { x, y, zoom };
    }
    scene.ink_canvas.as_ref().and_then(|ink| serde_json::from_str::<InkDocumentJson>(&ink.document_json).ok()).map(|doc| InkCameraF::from(doc.camera)).unwrap_or_default()
}

#[cfg(test)]
fn ink_events_json(events: &[Value]) -> String {
    Value::Array(events.to_vec()).to_string()
}

#[cfg(test)]
fn ink_apply_events_action(scene: &UiComponentSceneNode, events: &[Value], phase: &str, select_ids: Option<&[String]>) -> ActionDescriptor {
    let mut args = json!({
        "surfaceId": scene.surface_id,
        "eventsJson": ink_events_json(events),
        "phase": phase,
    });
    if let Some(ids) = select_ids {
        args["selectIds"] = json!(ids);
    }
    scene_action(scene, "inkApplyEvents", args)
}

#[cfg(test)]
fn ink_set_selection_action(scene: &UiComponentSceneNode, ids: &[String]) -> ActionDescriptor {
    scene_action(scene, "setSelection", json!({ "surfaceId": scene.surface_id, "ids": ids }))
}

#[cfg(test)]
fn ink_set_hover_action(scene: &UiComponentSceneNode, id: Option<&str>) -> ActionDescriptor {
    scene_action(scene, "setHover", json!({ "surfaceId": scene.surface_id, "id": id }))
}

#[cfg(test)]
fn ink_set_camera_action(scene: &UiComponentSceneNode, camera: InkCameraF) -> ActionDescriptor {
    scene_action(scene, "setCamera", json!({ "surfaceId": scene.surface_id, "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom } }))
}

#[cfg(test)]
fn ink_resize_handle_screen_pos(handle: &str, sx: f32, sy: f32, w: f32, h: f32, size: f32) -> (f32, f32) {
    let half = size * 0.5;
    let x = if handle.contains('w') {
        sx - half
    } else if handle.contains('e') {
        sx + w - half
    } else {
        sx + w * 0.5 - half
    };
    let y = if handle.contains('n') {
        sy - half
    } else if handle.contains('s') {
        sy + h - half
    } else {
        sy + h * 0.5 - half
    };
    (x, y)
}

#[cfg(test)]
fn ink_resize_handle_at(bounds: InkBoundsF, camera: InkCameraF, inner: Rect, sx: f32, sy: f32, hit_radius: f32) -> Option<&'static str> {
    let (bx, by) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    for handle in INK_RESIZE_HANDLES {
        let (hx, hy) = ink_resize_handle_screen_pos(handle, bx, by, w, h, 8.0);
        let cx = hx + 4.0;
        let cy = hy + 4.0;
        if ((sx - cx).powi(2) + (sy - cy).powi(2)).sqrt() <= hit_radius {
            return Some(handle);
        }
    }
    None
}

#[cfg(test)]
fn ink_pointer_down(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, shift: bool) -> Vec<ActionDescriptor> {
    let Some(ink) = &scene.ink_canvas else {
        return Vec::new();
    };
    if ink.view_mode == "navigator" || !ink.interactive {
        return Vec::new();
    }
    let doc: InkDocumentJson = serde_json::from_str(&ink.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&ink.selection_json).unwrap_or_default();
    let state = scene_state(&scene.surface_id);
    let camera = state.ink_camera.map(|(cx, cy, cz)| InkCameraF { x: cx, y: cy, zoom: cz }).unwrap_or_else(|| InkCameraF::from(doc.camera.clone()));
    let utility = doc.active_utility.clone().unwrap_or_else(|| "selectDirect".into());
    let mut actions = Vec::new();

    let selection_bounds = ink_selection_bounds(&doc.blocks, &state.ink_overrides, &selected_ids);
    let show_handles = (utility == "selectDirect" || utility == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if button == 0 && show_handles {
        if let Some(bounds) = selection_bounds {
            if let Some(handle) = ink_resize_handle_at(bounds, camera, inner, x, y, 8.0) {
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag { mode: SceneDragMode::InkResize { handle: handle.to_string(), from: bounds, start_x: x, start_y: y, selected_ids: selected_ids.clone() } });
                });
                return actions;
            }
        }
    }

    if utility == "pan" || button == 1 {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkPan { start_x: x, start_y: y, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom } });
        });
        return actions;
    }

    if button != 0 {
        return actions;
    }

    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);

    if utility == "eraserStroke" || utility == "eraserPoint" {
        let events = if utility == "eraserStroke" { erase_ink_stroke_events(&doc.blocks, world_x, world_y, 8.0) } else { erase_ink_stroke_points_events(&doc.blocks, world_x, world_y, doc.eraser_radius.unwrap_or(12.0)) };
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkEraser { mode: utility.clone() } });
        });
        if !events.is_empty() {
            actions.push(ink_apply_events_action(scene, &events, "begin", None));
        }
        return actions;
    }

    if utility == "selectMarquee" {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkMarqueeDrag { start_x: x, start_y: y } });
            s.ink_marquee_points = vec![(x, y)];
        });
        return actions;
    }

    if utility == "pencil" {
        let block = create_ink_item("stroke", world_x, world_y);
        let block_id = ink_item_id(&block).to_string();
        mutate_scene_state(&scene.surface_id, |s| {
            s.ink_overrides.insert(block_id.clone(), block.clone());
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkStroke { block_id: block_id.clone() } });
        });
        actions.push(ink_apply_events_action(scene, &[json!({ "operation": "addBlock", "block": block })], "begin", Some(&[block_id])));
        return actions;
    }

    if utility == "text" || utility == "image" || utility == "table" || utility == "math" {
        let (px, py) = ink_maybe_snap(&doc, world_x, world_y);
        let block = create_ink_item(&utility, px, py);
        let block_id = ink_item_id(&block).to_string();
        actions.push(ink_apply_events_action(scene, &[json!({ "operation": "addBlock", "block": block })], "atomic", Some(&[block_id])));
        return actions;
    }

    let hits = ink_items_at_point(&doc.blocks, &state.ink_overrides, world_x, world_y);
    let top = hits.first().copied();
    match top {
        Some(top_block) if !ink_item_locked(top_block) => {
            if utility == "selectDirect" {
                let top_id = ink_item_id(top_block).to_string();
                let next_selection = if shift {
                    let mut ids: Vec<String> = selected_ids.clone();
                    if !ids.contains(&top_id) {
                        ids.push(top_id.clone());
                    }
                    ids
                } else {
                    vec![top_id.clone()]
                };
                actions.push(ink_set_selection_action(scene, &next_selection));
                let move_ids: Vec<String> = if selected_ids.contains(&top_id) { selected_ids.clone() } else { vec![top_id.clone()] };
                let mut origins = HashMap::new();
                for id in &move_ids {
                    if let Some(b) = find_ink_item(&doc.blocks, id) {
                        let eff = state.ink_overrides.get(id).unwrap_or(b);
                        origins.insert(id.clone(), (ink_item_num(eff, "x"), ink_item_num(eff, "y")));
                    }
                }
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag { mode: SceneDragMode::InkMove { origins, start_x: x, start_y: y } });
                });
            }
        }
        _ => {
            if utility == "selectDirect" {
                actions.push(ink_set_selection_action(scene, &[]));
            }
        }
    }
    actions
}

#[cfg(test)]
fn ink_pointer_up(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    let state = scene_state(&scene.surface_id);
    let Some(drag) = state.drag.clone() else {
        return actions;
    };
    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
    match &drag.mode {
        SceneDragMode::InkMove { origins, .. } => {
            let mut events = Vec::new();
            for id in origins.keys() {
                if let Some(block) = state.ink_overrides.get(id).cloned().or_else(|| find_ink_item(&doc.blocks, id).cloned()) {
                    let updated = if doc.snap_enabled.unwrap_or(false) {
                        let spacing = doc.snap_grid_spacing.unwrap_or(8.0);
                        let (sx, sy) = ink_snap_point(ink_item_num(&block, "x"), ink_item_num(&block, "y"), spacing);
                        ink_item_with_position(&block, sx, sy)
                    } else {
                        block
                    };
                    events.push(json!({ "operation": "updateBlock", "blockId": id, "block": updated }));
                }
            }
            actions.push(ink_apply_events_action(scene, &events, "commit", None));
        }
        SceneDragMode::InkResize { selected_ids, .. } => {
            let mut events = Vec::new();
            for id in selected_ids {
                if let Some(block) = state.ink_overrides.get(id).cloned() {
                    events.push(json!({ "operation": "updateBlock", "blockId": id, "block": block }));
                }
            }
            actions.push(ink_apply_events_action(scene, &events, "commit", None));
        }
        SceneDragMode::InkStroke { block_id } => {
            if let Some(block) = state.ink_overrides.get(block_id).cloned() {
                actions.push(ink_apply_events_action(scene, &[json!({ "operation": "updateBlock", "blockId": block_id, "block": block })], "commit", None));
            } else {
                actions.push(ink_apply_events_action(scene, &[], "commit", None));
            }
        }
        SceneDragMode::InkEraser { .. } => {
            actions.push(ink_apply_events_action(scene, &[], "commit", None));
        }
        SceneDragMode::InkMarqueeDrag { start_x, start_y } => {
            let x0 = start_x.min(x);
            let y0 = start_y.min(y);
            let w = (x - start_x).abs();
            let h = (y - start_y).abs();
            if w >= 4.0 || h >= 4.0 {
                let camera = ink_current_camera(scene);
                let (wx0, wy0) = ink_screen_to_world(camera, inner, x0, y0);
                let (wx1, wy1) = ink_screen_to_world(camera, inner, x0 + w, y0 + h);
                let world_rect = InkBoundsF { x: wx0.min(wx1), y: wy0.min(wy1), w: (wx1 - wx0).abs(), h: (wy1 - wy0).abs() };
                let ids = ink_items_intersecting_rect(&doc.blocks, &state.ink_overrides, world_rect);
                actions.push(ink_set_selection_action(scene, &ids));
            }
        }
        _ => {}
    }
    mutate_scene_state(&scene.surface_id, |s| {
        s.drag = None;
        s.ink_marquee_points.clear();
    });
    actions
}

#[cfg(test)]
fn ink_hover_move(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let Some(ink) = &scene.ink_canvas else {
        return Vec::new();
    };
    if ink.view_mode == "navigator" || !ink.interactive {
        return Vec::new();
    }
    let doc: InkDocumentJson = serde_json::from_str(&ink.document_json).unwrap_or_default();
    let camera = ink_current_camera(scene);
    let (wx, wy) = ink_screen_to_world(camera, inner, x, y);
    let state = scene_state(&scene.surface_id);
    let hits = ink_items_at_point(&doc.blocks, &state.ink_overrides, wx, wy);
    let top_id = hits.first().map(|block| ink_item_id(block).to_string());
    if ink.hovered_id.as_deref() == top_id.as_deref() {
        return Vec::new();
    }
    vec![ink_set_hover_action(scene, top_id.as_deref())]
}

#[cfg(test)]
fn ink_wheel(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    let Some(ink) = &scene.ink_canvas else {
        return Vec::new();
    };
    if ink.view_mode == "navigator" {
        return Vec::new();
    }
    let camera = ink_current_camera(scene);
    let zoom_factor: f64 = if delta < 0.0 { 1.08 } else { 0.92 };
    let next_zoom = (camera.zoom * zoom_factor).clamp(0.1, 8.0);
    let (wx, wy) = ink_screen_to_world(camera, inner, x, y);
    let next = InkCameraF { x: (x - inner.x) as f64 - wx * next_zoom, y: (y - inner.y) as f64 - wy * next_zoom, zoom: next_zoom };
    mutate_scene_state(&scene.surface_id, |s| {
        s.ink_camera = Some((next.x, next.y, next.zoom));
    });
    vec![ink_set_camera_action(scene, next)]
}

#[cfg(test)]
fn draw_ink_rect_outline(draw: &mut ui_wgpu::wgpu::DrawList, x: f32, y: f32, w: f32, h: f32, color: Rgba, width: f32) {
    draw.push_line(x, y, x + w, y, color, width);
    draw.push_line(x + w, y, x + w, y + h, color, width);
    draw.push_line(x + w, y + h, x, y + h, color, width);
    draw.push_line(x, y + h, x, y, color, width);
}

#[cfg(test)]
fn draw_ink_table(ctx: &mut FrameworkWidgetContext<'_>, block: &Value, sx: f32, sy: f32, w: f32, h: f32, theme: &Theme) {
    let columns: Vec<String> = block.get("columns").and_then(Value::as_array).map(|c| c.iter().filter_map(|v| v.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let rows: Vec<Vec<String>> = block
        .get("rows")
        .and_then(Value::as_array)
        .map(|rows| rows.iter().map(|row| row.as_array().map(|cells| cells.iter().map(|cell| cell.get("content").and_then(Value::as_str).unwrap_or("").to_string()).collect()).unwrap_or_default()).collect())
        .unwrap_or_default();
    let col_count = columns.len().max(1);
    let row_count = rows.len() + 1;
    let col_w = w / col_count as f32;
    let row_h = h / row_count as f32;
    let font = theme.font_size_small.min(row_h * 0.6).max(6.0);
    for (index, label) in columns.iter().enumerate() {
        draw_text(ctx, label, sx + index as f32 * col_w + 3.0, sy + row_h * 0.7, font, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let ry = sy + (row_index + 1) as f32 * row_h;
        for (col_index, cell) in row.iter().enumerate() {
            draw_text(ctx, cell, sx + col_index as f32 * col_w + 3.0, ry + row_h * 0.7, font, theme.text);
        }
    }
    for index in 0..=col_count {
        let x = sx + index as f32 * col_w;
        ctx.draw.push_line(x, sy, x, sy + h, theme.separator, 0.5);
    }
    for index in 0..=row_count {
        let y = sy + index as f32 * row_h;
        ctx.draw.push_line(sx, y, sx + w, y, theme.separator, 0.5);
    }
}

#[cfg(test)]
fn draw_ink_image(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, doc: &InkDocumentJson, sx: f32, sy: f32, w: f32, h: f32) {
    let theme = ctx.theme;
    let image_key = ink_item_str(block, "imageKey");
    if let Some(asset) = doc.assets.get(image_key) {
        let mime = asset.get("mime").and_then(Value::as_str).unwrap_or("image/png");
        let data = asset.get("data").and_then(Value::as_str).unwrap_or("");
        let data_url = if data.starts_with("data:") { data.to_string() } else { format!("data:{mime};base64,{data}") };
        if let Some(key) = queue_canvas_image_upload(&scene.surface_id, ink_item_id(block), &data_url) {
            ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], 1.0);
            return;
        }
    }
    draw_text(ctx, image_key, sx + 6.0, sy + h * 0.5, theme.font_size_small, theme.text_muted);
}

#[cfg(test)]
fn draw_ink_item(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, camera: InkCameraF, inner: Rect, doc: &InkDocumentJson, selected: bool, hovered: bool) {
    let theme = ctx.theme;
    let kind = ink_item_kind(block);
    let bounds = ink_item_bounds(block);
    let (sx, sy) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;

    if kind == "stroke" {
        let points = ink_points(block);
        if points.len() >= 2 {
            let color = block
                .get("color")
                .and_then(Value::as_array)
                .map(|c| {
                    let get = |i: usize| c.get(i).and_then(Value::as_f64).unwrap_or(0.0) as f32;
                    Rgba::new(get(0), get(1), get(2), get(3))
                })
                .unwrap_or(theme.text);
            let stroke_width = (ink_item_num(block, "strokeWidth") as f32 * camera.zoom as f32).max(1.0);
            let screen_points: Vec<(f32, f32)> = points.iter().map(|p| ink_world_to_screen(camera, inner, p.0, p.1)).collect();
            for pair in screen_points.windows(2) {
                ctx.draw.push_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, color, stroke_width);
            }
        }
        return;
    }

    // 🎨️ `bg-background/90` in `ink-canvas-host.tsx` — `theme.panel` (the app-chrome surface token)
    // previously stood in for the canvas-item card token, which is `background`, not `panel`.
    let bg = theme.background;
    ctx.draw.push_rounded([sx, sy, w.max(4.0), h.max(4.0)], bg.with_alpha(0.9), theme.border_radius.min(6.0));

    match kind {
        "text" => {
            let text = ink_text_plain(block);
            let font_size = (ink_item_num(block, "fontSize").max(8.0) as f32 * camera.zoom as f32).max(6.0);
            draw_text_wrapped(ctx, &text, sx + 6.0, sy + 4.0, (w - 12.0).max(1.0), font_size, theme.text);
        }
        "math" => {
            let tex = ink_item_str(block, "tex");
            draw_text(ctx, tex, sx + 8.0, sy + h * 0.5 + 4.0, theme.font_size_body.max(8.0), theme.text);
        }
        "table" => draw_ink_table(ctx, block, sx, sy, w.max(4.0), h.max(4.0), theme),
        "image" => draw_ink_image(ctx, scene, block, doc, sx, sy, w.max(4.0), h.max(4.0)),
        "group" => {
            let children_len = block.get("children").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
            draw_text(ctx, &format!("Group · {children_len} children"), sx + 6.0, sy + 16.0, theme.font_size_small, theme.text_muted);
        }
        _ => {}
    }

    let border = if selected {
        theme.accent
    } else if hovered {
        theme.accent.with_alpha(theme.accent.a * 0.6)
    } else {
        theme.panel_border
    };
    let border_w = if selected { 2.0 } else { 1.0 };
    draw_ink_rect_outline(ctx.draw, sx, sy, w.max(4.0), h.max(4.0), border, border_w);
}

#[cfg(test)]
fn find_graph_node(scene: &UiComponentSceneNode, node_id: &str) -> Option<ui_wgpu::wgpu::NodeGraphNodeRecord> {
    scene.node_graph.as_ref().and_then(|graph| graph.nodes.iter().find(|n| n.id == node_id).cloned())
}

#[cfg(test)]
fn hit_graph_node(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Option<String> {
    let graph = scene.node_graph.as_ref()?;
    let state = scene_state(&scene.surface_id);
    let viewport = if state.viewport.zoom > 0.0 { state.viewport } else { Viewport::from_typed(graph.viewport.as_ref()) };
    for node in graph.nodes.iter().rev() {
        let (nx, ny) = state.node_positions.get(&node.id).copied().unwrap_or((node.x as f32, node.y as f32));
        let (sx, sy) = viewport.world_to_screen(nx, ny, inner);
        let w = node.width as f32 * viewport.zoom;
        let h = node.height as f32 * viewport.zoom;
        let rect = Rect::new(sx, sy, w, h);
        if rect.contains(x, y) {
            return Some(node.id.clone());
        }
    }
    None
}

#[cfg(test)]
pub fn tiled_map_pointer_down(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool, selection_method: &str) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if button == 0 {
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag { mode: SceneDragMode::MapMarquee { start_x: sx as f32, start_y: sy as f32, method: selection_method.to_string(), merge_mode: engine_canvas::map_marquee_mode(shift, ctrl_or_meta).to_string() } });
            state.map_marquee_points = vec![(sx as f32, sy as f32)];
            state.map_marquee_active = false;
        });
        return Vec::new();
    }
    if button == 1 {
        engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, 1));
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag { mode: SceneDragMode::MapPan });
        });
        return engine_canvas::with_map_host_mut(surface_id, |host| engine_canvas::map_interaction_actions(surface_id, controller_id, host)).unwrap_or_default();
    }
    let _ = controller_id;
    Vec::new()
}

#[cfg(test)]
pub fn tiled_map_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, down: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if down {
        let state = scene_state(surface_id);
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::MapPan => {
                    engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_move_screen(sx, sy));
                    return engine_canvas::with_map_host_mut(surface_id, |host| engine_canvas::map_interaction_actions(surface_id, controller_id, host)).unwrap_or_default();
                }
                SceneDragMode::MapMarquee { start_x, start_y, method, .. } => {
                    let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(surface_id, |state| {
                        if distance >= MAP_MARQUEE_THRESHOLD_PX {
                            state.map_marquee_active = true;
                        }
                        if state.map_marquee_active {
                            if method == "lasso" {
                                if state.map_marquee_points.last().copied() != Some((sx as f32, sy as f32)) {
                                    state.map_marquee_points.push((sx as f32, sy as f32));
                                }
                            } else {
                                state.map_marquee_points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                            }
                        }
                    });
                }
                _ => {}
            }
        }
        return Vec::new();
    }
    let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
    let hover = engine_canvas::parse_map_hover(&hit_json);
    let hover_json = if hover.is_null() { "null".into() } else { hover.to_string() };
    let prior = scene_state(surface_id).map_last_hover_json;
    if prior.as_deref() == Some(hover_json.as_str()) {
        return Vec::new();
    }
    mutate_scene_state(surface_id, |state| {
        state.map_last_hover_json = Some(hover_json.clone());
    });
    vec![engine_canvas::map_action(controller_id, ui_wgpu::wgpu::tiled_map_actions::SET_HOVER, json!({ "surfaceId": surface_id, "hover": hover }))]
}

#[cfg(test)]
pub fn tiled_map_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    let state = scene_state(surface_id);
    let Some(drag) = state.drag.clone() else {
        return Vec::new();
    };
    let mut actions = Vec::new();
    match drag.mode {
        SceneDragMode::MapPan => {
            engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_up_screen(sx, sy));
            actions.extend(engine_canvas::with_map_host_mut(surface_id, |host| engine_canvas::map_interaction_actions(surface_id, controller_id, host)).unwrap_or_default());
        }
        SceneDragMode::MapMarquee { start_x, start_y, method, merge_mode } => {
            let distance = ((sx as f32 - start_x).powi(2) + (sy as f32 - start_y).powi(2)).sqrt();
            if state.map_marquee_active && distance >= MAP_MARQUEE_THRESHOLD_PX {
                let mut points = state.map_marquee_points.clone();
                if method == "lasso" {
                    points.push((sx as f32, sy as f32));
                } else {
                    points = vec![(start_x, start_y), (sx as f32, sy as f32)];
                }
                let crossing = engine_canvas::map_marquee_crossing(&method, start_x, sx as f32);
                let (positions, routes) = engine_canvas::with_map_host(surface_id, |host| query_map_feature_hits(host, &method, &points, crossing)).unwrap_or_default();
                actions.push(engine_canvas::map_action(
                    controller_id,
                    ui_wgpu::wgpu::tiled_map_actions::SET_FEATURE_SELECTION,
                    json!({
                        "surfaceId": surface_id,
                        "positions": positions,
                        "routes": routes,
                        "mode": merge_mode,
                    }),
                ));
            } else if distance < MAP_MARQUEE_THRESHOLD_PX {
                let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
                let hit: Value = serde_json::from_str(&hit_json).unwrap_or(Value::Null);
                let (kind, id) = (hit.get("kind").and_then(|value| value.as_str()), hit.get("id").and_then(|value| value.as_str()));
                if let (Some(kind), Some(id)) = (kind, id) {
                    actions.push(engine_canvas::map_action(
                        controller_id,
                        ui_wgpu::wgpu::tiled_map_actions::SET_FEATURE_SELECTION,
                        json!({
                            "surfaceId": surface_id,
                            "positions": if kind == "position" { vec![id] } else { Vec::<&str>::new() },
                            "routes": if kind == "route" { vec![id] } else { Vec::<&str>::new() },
                            "mode": merge_mode,
                        }),
                    ));
                }
            }
        }
        _ => {}
    }
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.map_marquee_points.clear();
        state.map_marquee_active = false;
    });
    actions
}

#[cfg(test)]
fn paint_icon_render_chrome(ctx: &mut FrameworkWidgetContext<'_>, bounds: Rect, frame: Rect, request: &IconRenderRequestFields, shape: &str, footer: Option<&str>) {
    let theme = ctx.theme;
    // 🖼️ 2px, matching `IconShotFrame`'s `border-2 border-accent` in `icon-render-host.tsx` —
    // `theme.stroke_hairline` (1px) previously halved the frame width relative to React.
    let hair = 2.0_f32;
    ctx.draw.push_solid([frame.x, frame.y, frame.w, hair], theme.accent);
    ctx.draw.push_solid([frame.x, frame.y + frame.h - hair, frame.w, hair], theme.accent);
    ctx.draw.push_solid([frame.x, frame.y, hair, frame.h], theme.accent);
    ctx.draw.push_solid([frame.x + frame.w - hair, frame.y, hair, frame.h], theme.accent);

    let badge = format!("{}×{} · {}", request.width.round() as i64, request.height.round() as i64, shape);
    let badge_size = theme.font_size_small;
    let (badge_text_w, badge_text_h) = ctx.atlas.measure_text(&badge, badge_size);
    let pad = 4.0;
    let badge_w = badge_text_w + pad * 2.0;
    let badge_h = badge_text_h + pad * 2.0;
    let badge_x = frame.x + frame.w - badge_w - 4.0;
    let badge_y = frame.y + frame.h - badge_h - 4.0;
    // 🏷️ `bg-background/80`, not `panel` — the badge chip sits on the transparent canvas frame in
    // `icon-render-host.tsx`, not on a panel surface.
    ctx.draw.push_rounded([badge_x, badge_y, badge_w, badge_h], theme.background.with_alpha(0.8), 2.0);
    draw_text(ctx, &badge, badge_x + pad, badge_y + pad + badge_text_h * 0.8, badge_size, theme.text_muted);

    if let Some(footer) = footer {
        let footer_size = theme.font_size_small;
        let footer_w = ctx.atlas.measure_text(footer, footer_size).0;
        draw_text(ctx, footer, bounds.x + (bounds.w - footer_w) * 0.5, bounds.y + bounds.h - 8.0, footer_size, theme.text_muted);
    }
}

#[cfg(test)]
pub fn puzzle_board_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_move(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt)
}

#[cfg(test)]
pub fn puzzle_board_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_up(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt)
}

#[cfg(test)]
pub fn puzzle_board_pointer_leave(surface_id: &str, controller_id: &str, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_leave(surface_id, controller_id, alt)
}

#[cfg(test)]
pub fn puzzle_board_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_wheel(surface_id, controller_id, inner, x, y, delta)
}

#[cfg(test)]
fn vfs_glyph_icon<'a>(schema: &'a VfsSchema, row: &Value) -> &'a str {
    let kind_id = row.get("fileNodeKindId").and_then(|v| v.as_str()).unwrap_or("file");
    if let Some(icon) = schema.file_node_kinds.get(kind_id).and_then(|k| k.icon.as_deref()) {
        return icon;
    }
    match kind_id {
        "root" | "studio" | "folder" => "folder",
        "instance" => "box",
        _ => "file-text",
    }
}

#[cfg(test)]
fn vfs_double_click_action(scene: &UiComponentSceneNode, row: &Value) -> Option<ActionDescriptor> {
    let uri = row.get("navigateUri").and_then(|v| v.as_str())?;
    if uri.starts_with("os://instance/") {
        return Some(scene_action(
            scene,
            "openInstance",
            json!({
                "surfaceId": scene.surface_id,
                "instanceId": uri.trim_start_matches("os://instance/"),
            }),
        ));
    }
    if uri.starts_with("os://export/") {
        let parts: Vec<&str> = uri.split('/').collect();
        if parts.len() >= 5 {
            return Some(scene_action(
                scene,
                "exportMedia",
                json!({
                    "surfaceId": scene.surface_id,
                    "instanceId": parts[2],
                    "format": parts[4],
                }),
            ));
        }
    }
    if uri.starts_with("/spaces/") {
        let space_id = uri.split('/').nth(2)?;
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", json!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    if let Some(space_id) = uri.strip_prefix("studio:") {
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", json!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    None
}

#[cfg(test)]
fn text_editor_completions(editor: &ui_wgpu::wgpu::TextEditorScene) -> Vec<TextEditorCompletionItem> {
    editor.completions_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default()
}

#[cfg(test)]
fn text_editor_rename_info(editor: &ui_wgpu::wgpu::TextEditorScene) -> Option<TextEditorRenameInfo> {
    editor.rename_json.as_deref().and_then(|json| serde_json::from_str(json).ok())
}

#[cfg(test)]
fn identifier_prefix_start(text: &str, caret: usize) -> usize {
    let bytes = text.as_bytes();
    let mut start = caret.min(bytes.len());
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    start
}

#[cfg(test)]
fn text_editor_line_range(buffer: &str, cursor: usize) -> (usize, usize) {
    let (line_index, _) = line_col_at(buffer, cursor);
    let mut offset = 0usize;
    for (index, line) in buffer.lines().enumerate() {
        let end = offset + line.len();
        if index == line_index {
            return (offset, end);
        }
        offset = end + 1;
    }
    (buffer.len(), buffer.len())
}

#[cfg(test)]
fn text_editor_context_menu_items(editor: &ui_wgpu::wgpu::TextEditorScene) -> Vec<TextEditorMenuItem> {
    let mut items = Vec::new();
    if !text_editor_completions(editor).is_empty() {
        items.push(TextEditorMenuItem { id: "suggest", label: "Suggest Completions" });
    }
    items.push(TextEditorMenuItem { id: "select-token", label: "Select Token" });
    items.push(TextEditorMenuItem { id: "select-line", label: "Select Line" });
    items.push(TextEditorMenuItem { id: "select-all", label: "Select All" });
    if text_editor_rename_info(editor).is_some() {
        items.push(TextEditorMenuItem { id: "rename", label: "Rename" });
    }
    items.push(TextEditorMenuItem { id: "format", label: "Format Document" });
    items.push(TextEditorMenuItem { id: "lint", label: "Lint Document" });
    items
}

#[cfg(test)]
fn text_editor_run_menu_action(scene: &UiComponentSceneNode, editor: &ui_wgpu::wgpu::TextEditorScene, inner: Rect, menu: &TextEditorContextMenu, action_id: &str, ctx: &mut FrameworkWidgetContext<'_>, ui_state: &mut TextEditorUiState) -> bool {
    match action_id {
        "suggest" => {
            ui_state.completions_open = true;
            ui_state.completion_index = 0;
        }
        "select-token" => {
            if let Err(fault) = engine_canvas::text_editor_select_span_into(scene, inner, menu.x, menu.y, ctx.input) {
                ctx.input.record_action_fault(fault);
                return false;
            }
        }
        "select-line" => {
            let offset = cursor_from_click(scene, inner, menu.x, menu.y, 0.0);
            let (start, end) = text_editor_line_range(&editor.buffer, offset);
            if let Err(fault) = engine_canvas::text_editor_set_selection_into(scene, start, end, ctx.input) {
                ctx.input.record_action_fault(fault);
                return false;
            }
        }
        "select-all" => {
            let modifiers = ui_wgpu::wgpu::PointerModifiers { ctrl: true, ..Default::default() };
            if let Err(fault) = engine_canvas::text_editor_apply_key_into(scene, &KeyAction::Char("a".to_string()), &modifiers, ctx.input) {
                ctx.input.record_action_fault(fault);
                return false;
            }
        }
        "rename" => {
            if let Some(info) = text_editor_rename_info(editor) {
                ctx.input.focus_input_owned(format!("{}.editor.rename", scene.surface_id), info.name);
                ui_state.rename_active = true;
                ui_state.rename_occurrences = info.occurrences.iter().map(|span| (span.start, span.end)).collect();
            }
        }
        "format" => {
            if let Err(fault) = queue_surface_action(ctx.input, scene, "formatDocument") {
                ctx.input.record_action_fault(fault);
                return false;
            }
        }
        "lint" => {
            if let Err(fault) = queue_surface_action(ctx.input, scene, "lintDocument") {
                ctx.input.record_action_fault(fault);
                return false;
            }
        }
        _ => {}
    }
    true
}

#[cfg(test)]
fn text_editor_completion_anchor(scene: &UiComponentSceneNode, inner: Rect) -> (f32, f32) {
    engine_canvas::text_editor_caret_screen(scene, inner).unwrap_or((inner.x + 12.0, inner.y + 12.0))
}

#[cfg(test)]
fn text_editor_completion_row_rect(anchor: (f32, f32), theme: &Theme, index: usize) -> Rect {
    let row_h = theme.control_height_small;
    Rect::new(anchor.0, anchor.1 + 18.0 + index as f32 * row_h, 220.0, row_h)
}

#[cfg(test)]
fn text_editor_menu_row_rect(menu: &TextEditorContextMenu, theme: &Theme, index: usize) -> Rect {
    let row_h = theme.control_height;
    Rect::new(menu.x + 4.0, menu.y + 4.0 + index as f32 * row_h, 200.0 - 8.0, row_h)
}

#[cfg(test)]
fn text_editor_menu_hit(menu: &TextEditorContextMenu, theme: &Theme, x: f32, y: f32) -> Option<usize> {
    (0..menu.items.len()).find(|&index| text_editor_menu_row_rect(menu, theme, index).contains(x, y))
}

#[cfg(test)]
fn render_text_editor_completions(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, scene: &UiComponentSceneNode, completions: &[TextEditorCompletionItem], active_index: usize) {
    let theme = ctx.theme;
    let anchor = text_editor_completion_anchor(scene, inner);
    if completions.is_empty() {
        return;
    }
    // 🪟️ Outer popover container — matches `rounded border border-border bg-popover p-1 shadow-md`
    // on the completions list in `text-editor-host.tsx`; previously only per-row backgrounds were
    // drawn, with no enclosing frame at all.
    let pad = 4.0;
    let row_h = text_editor_completion_row_rect(anchor, theme, 0).h;
    let container = Rect::new(anchor.0 - pad, anchor.1 + 18.0 - pad, 220.0 + pad * 2.0, completions.len() as f32 * row_h + pad * 2.0);
    ctx.draw.push_rounded([container.x, container.y, container.w, container.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, container.x, container.y, container.w, container.h, theme.panel_border, 1.0);
    for (index, item) in completions.iter().enumerate() {
        let row = text_editor_completion_row_rect(anchor, theme, index);
        // 🎯️ Active row uses `bg-accent text-accent-foreground` in React — `theme.selected` (the
        // generic row-highlight token) previously stood in for the accent pairing used nowhere else
        // for completion rows.
        let (bg, fg) = if index == active_index { (theme.accent, theme.active_foreground) } else { (theme.panel, theme.text) };
        ctx.draw.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius * 0.5);
        draw_text(ctx, &item.label, row.x + 8.0, row.y + row.h * 0.68, theme.font_size_small, fg);
        if let Some(detail) = &item.detail {
            let label_w = item.label.len() as f32 * theme.font_size_small * 0.6;
            let detail_fg = if index == active_index { fg } else { theme.text_muted };
            draw_text(ctx, detail, row.x + 12.0 + label_w, row.y + row.h * 0.68, theme.font_size_small, detail_fg);
        }
    }
}

#[cfg(test)]
fn render_text_editor_context_menu(ctx: &mut FrameworkWidgetContext<'_>, menu: &TextEditorContextMenu) {
    let theme = ctx.theme;
    let row_h = theme.control_height;
    let w = 200.0;
    let h = menu.items.len() as f32 * row_h + 8.0;
    ctx.draw.push_rounded([menu.x, menu.y, w, h], theme.panel, theme.border_radius);
    // 🖊️ `ContextMenuChrome`/`WindowChrome` in React paints this as a titled floating window with a
    // glass material and its own border; the full window-chrome treatment is `shell`-owned (out of
    // scope here — see `render_context_menu`'s own top-level overlay), so this at least adds the
    // border stroke React's window frame always carries instead of a flat, edgeless panel fill.
    draw_ink_rect_outline(ctx.draw, menu.x, menu.y, w, h, theme.panel_border, 1.0);
    for (index, item) in menu.items.iter().enumerate() {
        let row = text_editor_menu_row_rect(menu, theme, index);
        ctx.draw.push_rounded([row.x, row.y, row.w, row.h], theme.button, theme.border_radius * 0.5);
        draw_text(ctx, item.label, row.x + 8.0, row.y + row.h * 0.68, theme.font_size_small, theme.text);
    }
}

#[cfg(test)]
fn render_text_editor_rename_input(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, scene: &UiComponentSceneNode) {
    let theme = ctx.theme;
    let text = ctx.input.text_view().to_string();
    let (x, y) = engine_canvas::text_editor_caret_screen(scene, inner).unwrap_or((inner.x + 12.0, inner.y + 12.0));
    let rect = Rect::new(x, (y - theme.control_height_small * 0.5).max(inner.y), 180.0, theme.control_height_small);
    // 🖊️ `border border-border bg-panel` on the rename input in `text-editor-host.tsx` — previously
    // drawn as an unbordered `theme.input_bg` fill (a different token, and no stroke at all).
    ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, theme.panel_border, 1.0);
    draw_text(ctx, &text, rect.x + 8.0, rect.y + rect.h * 0.68, theme.font_size_small, theme.text);
}

#[cfg(test)]
fn cursor_from_click(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, scroll: f32) -> usize {
    let Some(editor) = &scene.text_editor else {
        return 0;
    };
    let line_h = 18.0;
    let line_index = ((y - inner.y - 8.0 + scroll) / line_h).max(0.0) as usize;
    let lines: Vec<&str> = editor.buffer.lines().collect();
    let line = lines.get(line_index).copied().unwrap_or("");
    let rel_x = (x - inner.x - 8.0).max(0.0);
    let mut cursor = 0usize;
    let mut width = 0.0f32;
    for (index, ch) in line.chars().enumerate() {
        let advance = if ch == '\t' { 8.0 } else { 7.0 };
        if width + advance * 0.5 > rel_x {
            cursor = index;
            break;
        }
        width += advance;
        cursor = index + 1;
    }
    lines.iter().take(line_index).map(|l| l.len() + 1).sum::<usize>() + cursor
}

#[cfg(test)]
fn line_col_at(text: &str, cursor: usize) -> (usize, usize) {
    let mut index = 0usize;
    for (line_index, line) in text.lines().enumerate() {
        let next = index + line.len() + 1;
        if cursor < next {
            return (line_index, cursor.saturating_sub(index));
        }
        index = next;
    }
    let line_count = text.lines().count();
    (line_count.saturating_sub(1), 0)
}
