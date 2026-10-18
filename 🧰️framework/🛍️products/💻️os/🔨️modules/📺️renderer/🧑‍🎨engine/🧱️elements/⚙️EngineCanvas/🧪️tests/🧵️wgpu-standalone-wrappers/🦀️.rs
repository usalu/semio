/// 🧵️ Fixture wrappers of the engine canvas standalone laws: the node graph, map and board pointer helpers that collect the actions an engine entry point queued, shared by the EngineCanvas and the Scenes standalone laws (each mounts this file in its own `engine_canvas` module).
#[cfg(test)]
pub fn node_graph_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, ctrl: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_wheel_into(surface_id, controller_id, inner, x, y, delta, ctrl, &mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn node_graph_pointer_down(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl: bool, alt: bool, space_pressed: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_pointer_down_into(surface_id, controller_id, inner, x, y, button, shift, ctrl, alt, space_pressed, &mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn node_graph_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_pointer_move_into(surface_id, controller_id, inner, x, y, shift, ctrl, alt, &mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn node_graph_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = node_graph_pointer_up_into(surface_id, controller_id, inner, x, y, shift, ctrl, alt, &mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn map_action(controller_id: &str, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: Some(semio_framework::DslValue::from(args)) }
}

#[cfg(test)]
pub fn map_interaction_actions(surface_id: &str, controller_id: &str, host: &MapHost) -> Vec<ActionDescriptor> {
    vec![map_action(controller_id, semio_framework_ui_declaration::ui::tiled_map_actions::SET_CAMERA, json!({ "surfaceId": surface_id, "camera": serde_json::from_str::<Value>(&host.camera_json()).unwrap_or(json!({})) }))]
}

#[cfg(test)]
pub fn tiled_map_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, ctrl: bool) -> Vec<ActionDescriptor> {
    let mut input = ui_wgpu::wgpu::InputState::default();
    let _ = tiled_map_wheel_into(surface_id, controller_id, inner, x, y, delta, ctrl, &mut input);
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input)
}

#[cfg(test)]
pub fn board_action(controller_id: &str, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: Some(semio_framework::DslValue::from(args)) }
}

#[cfg(test)]
fn board_take_buffer_coalesced(surface_id: &str) -> Option<String> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        if entry.board_retiring_events.is_some() {
            return None;
        }
        let coalesced = coalesce_owned_board_events(&entry.board_pending_events).ok()?;
        let pending = std::mem::take(&mut entry.board_pending_events);
        entry.board_retiring_events = Some(pending);
        (coalesced.events_json != "[]").then_some(coalesced.events_json)
    })
}

#[cfg(test)]
fn board_flush_events_action(surface_id: &str, controller_id: &str) -> Option<ActionDescriptor> {
    while board_drain_into_buffer(surface_id) {}
    let events_json = board_take_buffer_coalesced(surface_id)?;
    Some(board_action(controller_id, "applyBoardEvents", json!({ "eventsJson": events_json })))
}

#[cfg(test)]
fn board_drain_and_maybe_flush(surface_id: &str, controller_id: &str) -> Vec<ActionDescriptor> {
    while board_drain_into_buffer(surface_id) {}
    let flush_now = ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).and_then(|entry| coalesce_owned_board_events(&entry.board_pending_events).ok()).is_some_and(|events| events.flush_now));
    if !flush_now {
        return Vec::new();
    }
    match board_take_buffer_coalesced(surface_id) {
        Some(events_json) => vec![board_action(controller_id, "applyBoardEvents", json!({ "eventsJson": events_json }))],
        None => Vec::new(),
    }
}

#[cfg(test)]
fn board_camera_action(surface_id: &str, controller_id: &str) -> Option<ActionDescriptor> {
    with_board_host(surface_id, |host| board_action(controller_id, "setCamera", json!({ "camera": { "x": host.scene.camera.x, "y": host.scene.camera.y, "zoom": host.scene.camera.zoom } })))
}

#[cfg(test)]
pub fn board2d_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt));
    board_set_pointer_inside(surface_id, true);
    board_drain_and_maybe_flush(surface_id, controller_id)
}

#[cfg(test)]
pub fn board2d_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt));
    board_flush_events_action(surface_id, controller_id).into_iter().collect()
}

#[cfg(test)]
pub fn board2d_pointer_leave(surface_id: &str, controller_id: &str, alt: bool) -> Vec<ActionDescriptor> {
    let was_inside = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return false;
        };
        let was = entry.board_pointer_inside;
        entry.board_pointer_inside = false;
        was
    });
    if !was_inside {
        return Vec::new();
    }
    with_board_host_mut(surface_id, |host| host.pointer_leave_screen(alt));
    board_flush_events_action(surface_id, controller_id).into_iter().collect()
}

#[cfg(test)]
pub fn board2d_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| wheel_screen(&mut host.scene, &mut host.outbox, sx, sy, delta as f64));
    let mut actions = Vec::new();
    if let Some(camera_action) = board_camera_action(surface_id, controller_id) {
        actions.push(camera_action);
    }
    if let Some(events_action) = board_flush_events_action(surface_id, controller_id) {
        actions.push(events_action);
    }
    actions
}
