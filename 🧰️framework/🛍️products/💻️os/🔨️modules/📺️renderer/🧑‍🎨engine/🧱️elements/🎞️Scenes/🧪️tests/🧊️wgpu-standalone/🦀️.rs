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
fn queue_surface_action(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, action: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, action, "surfaceId", &scene.surface_id])?;
    let mut reservation = input.reserve_action(&scene.controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.end_container()?;
    reservation.publish()
}

/// 🧪️ ⚙️EngineCanvas's own `#[cfg(test)] include!` sibling — where the 2026-09-08 sweep moved every
/// wrapper that must not be production-capable, exactly as this file holds 🎞️Scenes'.
/// 🧪️ This file itself: the `#[cfg(test)] include!` the production Scenes target splices in, and
/// therefore where every helper that must NOT be production-capable now lives.
#[cfg(test)]
#[test]
fn production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only() {
    const SCENES_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    const INTERPRETER_SOURCE: &str = include_str!("../../../🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs");
    const ENGINE_CANVAS_SOURCE: &str = include_str!("../../../⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs");
    const ENGINE_CANVAS_STANDALONE: &str = include_str!("../../../⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs");
    const STANDALONE_SOURCE: &str = include_str!("🦀️.rs");
    assert!(!SCENES_SOURCE.contains(concat!("queue_", "event(")));
    assert!(!INTERPRETER_SOURCE.contains(concat!("queue_", "event(")));
    assert!(!SCENES_SOURCE.contains(concat!("drain_", "keys(")));
    assert!(INTERPRETER_SOURCE.contains("struct SceneInteractionIntent"));
    assert!(INTERPRETER_SOURCE.contains("drive_scene_interaction_step"));
    assert!(INTERPRETER_SOURCE.contains("tree_revision"));
    assert!(!INTERPRETER_SOURCE.contains("Some(scene.clone())"));
    assert!(!INTERPRETER_SOURCE.contains("reserve_actions(ui_wgpu::wgpu::action::ACTION_BATCH_ITEM_CAPACITY"));
    for function in ["handle_scene_wheel", "handle_scene_pointer_move", "handle_scene_pointer_button"] {
        assert!(STANDALONE_SOURCE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must be test-only");
    }
    for function in ["ink_apply_events_action", "ink_set_selection_action", "ink_set_hover_action", "ink_set_camera_action", "ink_pointer_down", "ink_pointer_up", "ink_hover_move", "ink_wheel"] {
        assert!(STANDALONE_SOURCE.contains(&format!("#[cfg(test)]\nfn {function}")), "{function} must be test-only");
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
        assert!(ENGINE_CANVAS_STANDALONE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must not remain production-capable");
        assert!(!ENGINE_CANVAS_SOURCE.contains(&format!("pub fn {function}(")), "{function} must not be reachable from the production ⚙️EngineCanvas target");
    }
    for function in ["node_graph_wheel", "node_graph_pointer_down", "node_graph_pointer_move", "node_graph_pointer_up", "tiled_map_wheel", "puzzle_board_pointer_move", "puzzle_board_pointer_up", "puzzle_board_pointer_leave", "puzzle_board_wheel"] {
        assert!(ENGINE_CANVAS_STANDALONE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must remain test-only");
        assert!(!ENGINE_CANVAS_SOURCE.contains(&format!("pub fn {function}(")), "{function} must not be reachable from the production ⚙️EngineCanvas target");
    }
    for function in ["tiled_map_pointer_down", "tiled_map_pointer_move", "tiled_map_pointer_up", "puzzle_board_pointer_move", "puzzle_board_pointer_up", "puzzle_board_pointer_leave", "puzzle_board_wheel"] {
        assert!(STANDALONE_SOURCE.contains(&format!("#[cfg(test)]\npub fn {function}")), "{function} must remain test-only");
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

#[cfg(test)]
fn canvas_world_pointer_json(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, extra: Value) -> Value {
    let state = scene_state(&scene.surface_id);
    let (wx, wy) = state.viewport.screen_to_world(x, y, inner);
    let shift = extra.get("extend").and_then(Value::as_bool).unwrap_or(false);
    let mut payload = json!({
        "surfaceId": scene.surface_id,
        "x": x - inner.x,
        "y": y - inner.y,
        "width": inner.w,
        "height": inner.h,
        "shift": shift,
        "ctrl": false,
        "meta": false,
        "alt": false,
        "extend": shift,
        "worldX": wx,
        "worldY": wy,
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
                // 🫳️ A row transfer resolves on the RELEASE (`scene_transfer_drop_action`), so the
                // move itself carries no state of its own — the same no-op the pan modes take.
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
                        if let Some(block) = test_find_ink_item(&doc.blocks, id) {
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
                        if let Some(block) = test_find_ink_item(&doc.blocks, id) {
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
                    let current = state.ink_overrides.get(block_id).cloned().or_else(|| test_find_ink_item(&doc.blocks, block_id).cloned());
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
            actions.push(scene_action(scene, "canvasPointerMove", canvas_world_pointer_json(scene, inner, x, y, json!({ "samples": [[x - inner.x, y - inner.y]] }))));
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
                actions.push(scene_action(scene, "canvasPointerUp", canvas_world_pointer_json(scene, inner, x, y, json!({ "cancelled": false }))));
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
fn test_find_ink_item<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
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
fn ink_maybe_snap(doc: &InkDocumentJson, x: f64, y: f64) -> (f64, f64) {
    ink_maybe_snap_fields(doc.snap_enabled, doc.snap_grid_spacing, x, y)
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
                    if let Some(b) = test_find_ink_item(&doc.blocks, id) {
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
                if let Some(block) = state.ink_overrides.get(id).cloned().or_else(|| test_find_ink_item(&doc.blocks, id).cloned()) {
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
