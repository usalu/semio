//! 🎥️ Scene camera and drag drop: the component scene note edit helpers, the node graph camera fit, catalogue and wheel zoom, the drag payloads and ghosts and the flow widget and catalogue drop actions. See [`super`].

use super::*;

//#region NodeGraph
pub fn component_scene_has_editable_caret(scene: &UiComponentSceneNode) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let surfaces = cell.borrow();
        let Some(entry) = surfaces.get(&scene.host_id) else { return false };
        entry.surface_id.as_ref().is_some_and(|surface| surface.as_str() == scene.surface_id)
            && match (&scene.component_kind, entry.node_graph.as_ref()) {
                (SurfaceKind::NodeGraph, Some(NodeGraphEngine::Flow(host))) => host.dag.notes.editing_note_id().is_some(),
                _ => false,
            }
    })
}

#[cfg(any(test, feature = "testkit"))]
pub fn component_scene_begin_note_edit_fixture(scene: &UiComponentSceneNode, widget_id: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(entry) = surfaces.get_mut(&scene.host_id) else { return false };
        if entry.surface_id.as_ref().is_none_or(|surface| surface.as_str() != scene.surface_id) || scene.component_kind != SurfaceKind::NodeGraph {
            return false;
        }
        let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() else { return false };
        host.begin_note_edit(widget_id, 0.0, 0.0);
        host.dag.notes.editing_note_id() == Some(widget_id)
    })
}

pub fn component_scene_apply_note_edit_key(scene: &UiComponentSceneNode, action: KeyAction, modifiers: &PointerModifiers) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let applied = {
            let Some(entry) = surfaces.get_mut(&scene.host_id) else { return false };
            if entry.surface_id.as_ref().is_none_or(|surface| surface.as_str() != scene.surface_id) || scene.component_kind != SurfaceKind::NodeGraph {
                return false;
            }
            let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() else { return false };
            if host.dag.notes.editing_note_id().is_none() {
                return false;
            }
            match action {
                KeyAction::Char(ch) if !modifiers.ctrl_or_meta() => host.note_insert_text(&ch),
                KeyAction::Backspace => host.note_backspace(),
                KeyAction::Delete => host.note_delete_forward(),
                KeyAction::ArrowLeft => {
                    let _ = host.note_move_caret("left", modifiers.shift);
                }
                KeyAction::ArrowRight => {
                    let _ = host.note_move_caret("right", modifiers.shift);
                }
                KeyAction::Enter | KeyAction::Escape => host.note_commit_edit(),
                _ => return false,
            }
            true
        };
        if applied {
            mark_engine_scene_repaint(&mut surfaces, &scene.host_id);
        }
        applied
    })
}

/// ✍️ Applies one accepted component scene's caret phase to its exact mounted engine surface — addressed by the scene's
/// identity alone (`host_id`, `surface_id`, `kind`), so a caret phase never copies the scene's payload.
pub fn component_scene_set_caret_visible(host_id: &str, surface_id: &str, kind: SurfaceKind, visible: bool) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let changed = {
            let Some(entry) = surfaces.get_mut(host_id) else { return false };
            if entry.surface_id.as_ref().is_none_or(|surface| surface.as_str() != surface_id) {
                return false;
            }
            match kind {
                SurfaceKind::TextEditor => {
                    let Some(host) = entry.editor.as_mut() else { return false };
                    host.set_caret_visible(visible);
                    true
                }
                SurfaceKind::NodeGraph => {
                    let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() else { return false };
                    if host.dag.notes.editing_note_id().is_none() {
                        return false;
                    }
                    host.set_note_caret_visible(visible);
                    true
                }
                _ => false,
            }
        };
        if changed {
            mark_engine_scene_repaint(&mut surfaces, host_id);
        }
        changed
    })
}

/// 🖼️ Frames the whole graph of one node-graph surface and answers the camera it installed, or
/// `None` when the surface has no live engine or no graph to frame. The wgpu twin of React's
/// `fitGraphToView` (`🧱️elements/🕸️NodeGraph/🟦️.tsx`); the caller persists the answer through
/// `nodeGraphViewport` exactly the way a pan or a zoom gesture is persisted, so the next open
/// honours it.
///
/// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::fit_camera_to_content`
pub fn node_graph_fit_camera(surface_id: &str) -> Option<[f64; 3]> {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let engine = surfaces.get_mut(surface_id)?.node_graph.as_mut()?;
        match engine {
            NodeGraphEngine::Flow(host) => host.fit_camera_to_content().then(|| host.camera()),
            NodeGraphEngine::Dag(host) => fit_camera_to_content(&mut host.dag.viewport, &mut host.dag.graph).then(|| [host.dag.graph.host_snapshot.camera.x, host.dag.graph.host_snapshot.camera.y, host.dag.graph.host_snapshot.camera.zoom]),
        }
    })
}

/// 🛍️ Installs the app-static operator/palette catalogue on one node-graph surface's engine host.
/// Idempotent and cheap: an unchanged payload costs one string compare, which is what makes it safe
/// to call from the shell's per-refresh `publish_app_catalogue` sweep. Returns whether a live host
/// took it — `false` when the surface has no engine yet (the shell simply republishes next refresh).
///
/// @see `semio_framework_os_node_graph::GraphHost::set_catalogue_json`
/// @see `semio_framework_os_flow::FlowHost::set_host_catalogue_json`
pub fn node_graph_set_catalogue_json(surface_id: &str, json: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(entry) = surfaces.get_mut(surface_id) else { return false };
        match entry.node_graph.as_mut() {
            Some(NodeGraphEngine::Dag(host)) => {
                if host.catalogue_json != json {
                    host.set_catalogue_json(json);
                }
                true
            }
            Some(NodeGraphEngine::Flow(host)) => {
                host.set_host_catalogue_json(json);
                true
            }
            None => false,
        }
    })
}

pub(super) fn node_graph_pan_gesture(button: i16, alt: bool, space_pressed: bool) -> bool {
    button == 1 || (button == 0 && (alt || space_pressed))
}

fn node_graph_set_wheel_zoom_active(entry: &mut EngineSurface, active: bool) {
    match entry.node_graph.as_mut() {
        Some(NodeGraphEngine::Flow(host)) => dag_set_wheel_zoom_active(&mut host.dag.viewport, &mut host.dag.graph, active),
        Some(NodeGraphEngine::Dag(host)) => dag_set_wheel_zoom_active(&mut host.dag.viewport, &mut host.dag.graph, active),
        None => {}
    }
}

pub fn node_graph_clear_wheel_zoom_active() {
    ENGINE_SURFACES.with(|cell| {
        for entry in cell.borrow_mut().values_mut() {
            node_graph_set_wheel_zoom_active(entry, false);
        }
    });
}

pub const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";

pub const CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

/// 🧩️ Parses a Puzzle 3D catalogue drag payload (`objectKind`, optional `meshUrl`) — mirrors React `parsePuzzle3dCatalogueDragPayload`.
pub fn puzzle3d_catalogue_drag_payload_json(raw: &str) -> Option<(String, Option<String>)> {
    let payload: Value = serde_json::from_str(raw).ok()?;
    let object_kind = payload.get("objectKind").and_then(|value| value.as_str()).filter(|kind| !kind.is_empty())?;
    let mesh_url = payload.get("meshUrl").and_then(|value| value.as_str()).filter(|url| !url.is_empty()).map(str::to_string);
    Some((object_kind.to_string(), mesh_url))
}

/// 🧩️ Reads the Puzzle 3D catalogue drag payload from a tree drag's mime map.
pub fn puzzle3d_catalogue_drag_payload(drag_data: &HashMap<String, String>) -> Option<(String, Option<String>)> {
    drag_data.get(CATALOGUE_DRAG_MIME).and_then(|raw| puzzle3d_catalogue_drag_payload_json(raw))
}

/// 👻️ Ghost descriptor JSON for a catalogue app drag (mirrors React `catalogueGhostDescriptorJson`).
pub fn catalogue_ghost_descriptor_json(raw: &str) -> Option<String> {
    let payload: Value = serde_json::from_str(raw).ok()?;
    let plugin_id = payload.get("pluginId").and_then(|value| value.as_str())?;
    let app_id = payload.get("appId").and_then(|value| value.as_str())?;
    if plugin_id.is_empty() || app_id.is_empty() {
        return None;
    }
    let neuron_kind = payload.get("label").and_then(|value| value.as_str()).filter(|label| !label.is_empty()).unwrap_or(app_id);
    Some(json!({ "kind": "neuron", "neuronKind": neuron_kind }).to_string())
}

pub fn node_graph_drag_ghost_descriptor(drag_data: &HashMap<String, String>) -> Option<String> {
    if let Some(raw) = drag_data.get(FLOW_WIDGET_DRAG_MIME) {
        return Some(raw.clone());
    }
    drag_data.get(CATALOGUE_DRAG_MIME).and_then(|raw| catalogue_ghost_descriptor_json(raw))
}

fn node_graph_world_at(surface_id: &str, bounds: &Rect, x: f32, y: f32) -> Option<(f64, f64)> {
    let sx = (x - bounds.x) as f64;
    let sy = (y - bounds.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        cell.borrow().get(surface_id).and_then(|entry| {
            let NodeGraphEngine::Flow(host) = entry.node_graph.as_ref()? else {
                return None;
            };
            Some(dag_screen_to_world(&host.dag.viewport, &host.dag.graph, sx, sy))
        })
    })
}

pub fn node_graph_clear_all_ghost_widgets() {
    ENGINE_SURFACES.with(|cell| {
        for entry in cell.borrow_mut().values_mut() {
            if let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() {
                host.clear_ghost_widget();
            }
        }
    });
}

pub fn node_graph_sync_flow_widget_ghost(x: f32, y: f32, drag_data: &HashMap<String, String>, surfaces: &[(&str, Rect)]) {
    let Some(descriptor) = node_graph_drag_ghost_descriptor(drag_data) else {
        node_graph_clear_all_ghost_widgets();
        return;
    };
    let mut over_graph = false;
    for (surface_id, bounds) in surfaces {
        if !bounds.contains(x, y) {
            continue;
        }
        let sx = (x - bounds.x) as f64;
        let sy = (y - bounds.y) as f64;
        ENGINE_SURFACES.with(|cell| {
            if let Some(entry) = cell.borrow_mut().get_mut(*surface_id) {
                if let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() {
                    let (world_x, world_y) = dag_screen_to_world(&host.dag.viewport, &host.dag.graph, sx, sy);
                    let _ = host.set_ghost_widget(&descriptor, world_x, world_y);
                    over_graph = true;
                }
            }
        });
        break;
    }
    if !over_graph {
        node_graph_clear_all_ghost_widgets();
    }
}

/// 🖱️ `addWidget` args for a flow-widget drop, including format and action when the descriptor has them.
pub fn flow_widget_drop_args(descriptor: &Value, x: f64, y: f64) -> Option<semio_framework::DslValue> {
    crate::action_args_json!({
        "kind": descriptor.get("kind").and_then(|value| value.as_str()).unwrap_or("inputSlider"),
        "neuronKind": descriptor.get("neuronKind").and_then(|value| value.as_str()),
        "label": descriptor.get("label").and_then(|value| value.as_str()),
        "format": descriptor.get("format").and_then(|value| value.as_str()),
        "action": descriptor.get("action").and_then(|value| value.as_str()),
        "x": x,
        "y": y,
    })
}

pub fn node_graph_flow_widget_drop_action(x: f32, y: f32, drag_data: &HashMap<String, String>, surfaces: &[(&str, Rect, &str)]) -> Option<ActionDescriptor> {
    let raw = drag_data.get(FLOW_WIDGET_DRAG_MIME)?;
    let descriptor: Value = serde_json::from_str(raw).ok()?;
    for (surface_id, bounds, controller_id) in surfaces {
        if !bounds.contains(x, y) {
            continue;
        }
        let world = node_graph_world_at(surface_id, bounds, x, y)?;
        return Some(ActionDescriptor { controller_id: (*controller_id).to_string(), action: "addWidget".into(), args: flow_widget_drop_args(&descriptor, world.0, world.1) });
    }
    None
}

/// 📦️ `spawnApp` action when a catalogue app is dropped on a flow node-graph surface.
pub fn node_graph_catalogue_drop_action(x: f32, y: f32, drag_data: &HashMap<String, String>, surfaces: &[(&str, Rect, &str)]) -> Option<ActionDescriptor> {
    let raw = drag_data.get(CATALOGUE_DRAG_MIME)?;
    let payload: Value = serde_json::from_str(raw).ok()?;
    let plugin_id = payload.get("pluginId").and_then(|value| value.as_str())?;
    let app_id = payload.get("appId").and_then(|value| value.as_str())?;
    if plugin_id.is_empty() || app_id.is_empty() {
        return None;
    }
    for (surface_id, bounds, controller_id) in surfaces {
        if !bounds.contains(x, y) {
            continue;
        }
        let world = node_graph_world_at(surface_id, bounds, x, y).unwrap_or_else(|| ((x - bounds.x) as f64, (y - bounds.y) as f64));
        eprintln!("[TRACE] catalogue workflow drop surface={surface_id} controller={controller_id} program={plugin_id} app={app_id} world=({:.1},{:.1})", world.0, world.1);
        return Some(ActionDescriptor {
            controller_id: (*controller_id).to_string(),
            action: "spawnApp".into(),
            args: crate::action_args_json!({
                "pluginId": plugin_id,
                "appId": app_id,
                "position": { "x": world.0, "y": world.1 },
            }),
        });
    }
    None
}
//#endregion NodeGraph
