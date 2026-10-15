//! 🕸️ Node graph interaction.
//!
//! Node graph camera, catalogue drag and drop, wheel, pointer, connection cursor, published interaction,
//! labels and overlays of an engine surface.

use super::engine_surface_registry::{
    engine_surface_wire_id, EngineSurface, EngineSurfaceRegistry, NodeGraphEngine, ENGINE_SURFACES,
};
use super::paint2d::{paint2d_marquee_overlay, paint2d_navigator_overlay_rect};
use crate::element_contracts::FrameworkWidgetContext;
use flow::dag::dag_screen_to_world;
use flow::FlowHost;
use semio_framework_canvas as canvas;
use semio_framework_os_node_graph::GraphHost;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use ui_wgpu::wgpu::{
    draw_text_overlay, ActionDescriptor, FontAtlas, KeyAction, PointerModifiers, Rect, Rgba,
    SurfaceKind, Theme, UiComponentSceneNode,
};

//#region NodeGraph

pub fn component_scene_has_editable_caret(scene: &UiComponentSceneNode) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let surfaces = cell.borrow();
        let Some(entry) = surfaces.get(&scene.host_id) else { return false };
        entry.surface_id.as_ref().is_some_and(|surface| surface.as_str() == scene.surface_id)
            && match (&scene.component_kind, entry.node_graph.as_ref()) {
                (SurfaceKind::NodeGraph, Some(NodeGraphEngine::Flow(host))) => host.dag.editing_note_id().is_some(),
                _ => false,
            }
    })
}

#[cfg(test)]
pub(crate) fn component_scene_begin_note_edit_fixture(scene: &UiComponentSceneNode, widget_id: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(entry) = surfaces.get_mut(&scene.host_id) else { return false };
        if entry.surface_id.as_ref().is_none_or(|surface| surface.as_str() != scene.surface_id) || scene.component_kind != SurfaceKind::NodeGraph {
            return false;
        }
        let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() else { return false };
        host.begin_note_edit(widget_id, 0.0, 0.0);
        host.dag.editing_note_id() == Some(widget_id)
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
            if host.dag.editing_note_id().is_none() {
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
                    if host.dag.editing_note_id().is_none() {
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
            NodeGraphEngine::Dag(host) => host.dag.fit_camera_to_content().then(|| [host.dag.host_snapshot.camera.x, host.dag.host_snapshot.camera.y, host.dag.host_snapshot.camera.zoom]),
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

fn node_graph_pan_gesture(button: i16, alt: bool, space_pressed: bool) -> bool {
    button == 1 || (button == 0 && (alt || space_pressed))
}

fn node_graph_set_wheel_zoom_active(entry: &mut EngineSurface, active: bool) {
    match entry.node_graph.as_mut() {
        Some(NodeGraphEngine::Flow(host)) => host.dag.set_wheel_zoom_active(active),
        Some(NodeGraphEngine::Dag(host)) => host.dag.set_wheel_zoom_active(active),
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

const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
const CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

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

fn node_graph_drag_ghost_descriptor(drag_data: &HashMap<String, String>) -> Option<String> {
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
            Some(dag_screen_to_world(&host.dag, sx, sy))
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
                    let (world_x, world_y) = dag_screen_to_world(&host.dag, sx, sy);
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
fn flow_widget_drop_args(descriptor: &Value, x: f64, y: f64) -> Option<semio_framework::DslValue> {
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

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-catalogue-workflow-drop/🦀️.rs"]
mod catalogue_workflow_drop_tests;

pub fn node_graph_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, _ctrl: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let planned = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(entry) = map.get(surface_id) else {
            return Ok(None);
        };
        let plan = match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => NodeGraphWheelPlan::Flow(host.plan_wheel(sx, sy, 0.0, delta as f64, true)),
            Some(NodeGraphEngine::Dag(host)) => NodeGraphWheelPlan::Dag(host.plan_wheel(sx, sy, 0.0, delta as f64, true)),
            None => return Ok(None),
        };
        graph_interaction_snapshot(entry, Some(plan.camera())).map(|snapshot| Some((plan, snapshot)))
    })?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    let dispatch = graph_interaction_dispatch(published_graph_interaction(surface_id), snapshot, node_graph_interaction_domain(surface_id).as_deref())?;
    let items = dispatch.item_count();
    if items == 0 {
        return Ok(false);
    }
    let Some(wire_surface_id) = engine_surface_wire_id(surface_id) else { return Ok(false) };
    let mut reservation = input.reserve_actions(items, items * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    write_graph_interaction_actions(&mut reservation, wire_surface_id.as_str(), controller_id, &dispatch)?;
    reservation.publish_with_checked(|| {
        ENGINE_SURFACES.with(|cell| {
            let mut map = cell.borrow_mut();
            let Some(engine) = map.get_mut(surface_id).and_then(|entry| entry.node_graph.as_mut()) else {
                return false;
            };
            let committed = match (engine, plan) {
                (NodeGraphEngine::Flow(host), NodeGraphWheelPlan::Flow(plan)) => {
                    if !host.commit_wheel(plan) {
                        return false;
                    }
                    host.dag.set_wheel_zoom_active(true);
                    true
                }
                (NodeGraphEngine::Dag(host), NodeGraphWheelPlan::Dag(plan)) => {
                    if !host.commit_wheel(plan) {
                        return false;
                    }
                    host.dag.set_wheel_zoom_active(true);
                    true
                }
                _ => false,
            };
            if committed {
                mark_engine_scene_repaint(&mut map, surface_id);
            }
            committed
        })
    })?;
    record_graph_interaction(surface_id, dispatch.digest);
    Ok(true)
}

pub fn node_graph_pointer_down_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    button: i16,
    shift: bool,
    ctrl: bool,
    alt: bool,
    space_pressed: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let pan = node_graph_pan_gesture(button, alt, space_pressed);
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let intent = flow::dag::DagPointerIntent { phase: flow::dag::DagPointerPhase::Down, x: sx, y: sy, button: button.max(0) as u8, shift, ctrl_or_meta: ctrl, alt, pan };
    if node_graph_gesture_is_screen_path(surface_id, Some((sx, sy))) {
        return node_graph_screen_pointer_into(surface_id, controller_id, intent, input);
    }
    let planned = plan_node_graph_pointer(surface_id, intent)?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    node_graph_bounded_publish(surface_id, controller_id, plan, snapshot, input)
}

pub fn node_graph_pointer_move_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let intent = flow::dag::DagPointerIntent { phase: flow::dag::DagPointerPhase::Move, x: sx, y: sy, button: 0, shift, ctrl_or_meta: ctrl, alt, pan: false };
    if node_graph_gesture_is_screen_path(surface_id, Some((sx, sy))) {
        return node_graph_screen_pointer_into(surface_id, controller_id, intent, input);
    }
    let planned = plan_node_graph_pointer(surface_id, intent)?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    node_graph_bounded_publish(surface_id, controller_id, plan, snapshot, input)
}

/// 🫃️ The bounded path's one dispatch point: the interaction actions this gesture CHANGED, plus
/// the node moves it commits when it ends a drag, reserved exactly and published atomically with the
/// host mutation. A gesture that changed nothing reserves nothing — see `📌️GraphInteractionChange`.
fn node_graph_bounded_publish(surface_id: &str, controller_id: &str, plan: NodeGraphPointerPlan, snapshot: GraphInteractionSnapshot, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let edits = plan_node_graph_edits(surface_id, &plan);
    let dispatch = graph_interaction_dispatch(published_graph_interaction(surface_id), snapshot, node_graph_interaction_domain(surface_id).as_deref())?;
    let items = dispatch.item_count() + usize::from(!edits.is_empty());
    if items == 0 {
        return Ok(commit_node_graph_pointer(surface_id, plan));
    }
    let Some(wire_surface_id) = engine_surface_wire_id(surface_id) else { return Ok(false) };
    let mut reservation = input.reserve_actions(items, items * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    write_graph_interaction_actions(&mut reservation, wire_surface_id.as_str(), controller_id, &dispatch)?;
    write_graph_edit_action(&mut reservation, controller_id, &edits)?;
    reservation.publish_with_checked(|| commit_node_graph_pointer(surface_id, plan))?;
    record_graph_interaction(surface_id, dispatch.digest);
    Ok(true)
}

/// 🫃️ The guest-vocabulary edits a bounded plan commits — today the node moves a finished drag
/// performed. Read off the PLAN, never off the committed host, because the bounded protocol has to
/// know every byte it will publish before the mutation is allowed to happen.
fn plan_node_graph_edits(surface_id: &str, plan: &NodeGraphPointerPlan) -> Vec<flow::dag::DagGraphEdit> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(engine) = map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) else {
            return Vec::new();
        };
        match (engine, plan) {
            (NodeGraphEngine::Flow(host), NodeGraphPointerPlan::Flow(plan)) => host.dag.plan_graph_edits(plan),
            (NodeGraphEngine::Dag(host), NodeGraphPointerPlan::Dag(plan)) => host.dag.plan_graph_edits(plan),
            _ => Vec::new(),
        }
    })
}

pub fn node_graph_pointer_up_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let intent = flow::dag::DagPointerIntent { phase: flow::dag::DagPointerPhase::Up, x: sx, y: sy, button: 0, shift, ctrl_or_meta: ctrl, alt, pan: false };
    if node_graph_gesture_is_screen_path(surface_id, Some((sx, sy))) {
        return node_graph_screen_pointer_into(surface_id, controller_id, intent, input);
    }
    let planned = plan_node_graph_pointer(surface_id, intent)?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    node_graph_bounded_publish(surface_id, controller_id, plan, snapshot, input)
}

pub fn node_graph_pointer_cancel_into(surface_id: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else { return false };
        let Some(engine) = entry.node_graph.as_mut() else { return false };
        match engine {
            NodeGraphEngine::Flow(host) => host.pointer_cancel_screen(),
            NodeGraphEngine::Dag(host) => {
                host.pointer_cancel_screen();
            }
        }
        entry.scene_revision = entry.scene_revision.wrapping_add(1);
        true
    })
}

pub(super) struct GraphInteractionSnapshot {
    pub(super) node_ids: Vec<String>,
    pub(super) hovered_id: Option<String>,
    /// 🔌️ The `"{nodeId}@{portId}"` channel under the pointer, when the pick landed on a port rather
    /// than a node body. React qualifies its hover target the same way — `nodeGraphHoverActionArgs`
    /// emits `granularity: "handle"` with exactly this id — which is what makes hover bidirectional
    /// between the graph and a `World3d` preview keyed by the same channel id.
    pub(super) hovered_handle: Option<String>,
    pub(super) viewport: semio_framework_os_kernel::Viewport2d,
}

enum NodeGraphWheelPlan {
    Flow(flow::FlowWheelPlan),
    Dag(semio_framework_os_node_graph::GraphWheelPlan),
}

enum NodeGraphPointerPlan {
    Flow(flow::dag::DagPointerPlan),
    Dag(flow::dag::DagPointerPlan),
}

impl NodeGraphWheelPlan {
    pub(super) fn camera(&self) -> [f64; 3] {
        match self {
            Self::Flow(plan) => plan.camera(),
            Self::Dag(plan) => plan.camera(),
        }
    }
}

fn graph_plan_fault(fault: flow::dag::DagInteractionPlanFault) -> ui_wgpu::wgpu::BoundedActionFault {
    match fault {
        flow::dag::DagInteractionPlanFault::NodeCredits => ui_wgpu::wgpu::BoundedActionFault::NodeCredits,
        flow::dag::DagInteractionPlanFault::StringCredits => ui_wgpu::wgpu::BoundedActionFault::StringCredits,
        flow::dag::DagInteractionPlanFault::Unsupported => ui_wgpu::wgpu::BoundedActionFault::Structure,
    }
}

pub(super) fn graph_projection_snapshot(node_ids: Vec<String>, hovered_id: Option<String>, hovered_handle: Option<String>, camera: [f64; 3]) -> Result<GraphInteractionSnapshot, ui_wgpu::wgpu::BoundedActionFault> {
    let viewport = semio_framework_os_kernel::Viewport2d { x: camera[0], y: camera[1], zoom: camera[2] };
    viewport.validate().map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let mut parts = Vec::with_capacity(node_ids.len() + 2);
    parts.extend([hovered_id.as_deref().unwrap_or_default(), hovered_handle.as_deref().unwrap_or_default()]);
    parts.extend(node_ids.iter().map(String::as_str));
    ui_wgpu::wgpu::checked_action_string_bytes(&parts)?;
    Ok(GraphInteractionSnapshot { node_ids, hovered_id, hovered_handle, viewport })
}

/// 🔌️ The channel the DAG's own hit-test reports under a screen point, in the `"{nodeId}@{portId}"`
/// pick-id grammar. A pure query on the live host — the bounded-action protocol has to know the
/// payload BEFORE the plan is committed, so hover qualification cannot be read back off the commit.
///
/// @see `🧱️elements/🕸️NodeGraph/🟦️.tsx` — `nodeGraphPickChannel`
fn graph_hovered_handle(engine: &NodeGraphEngine, sx: f64, sy: f64) -> Option<String> {
    let targets = match engine {
        NodeGraphEngine::Flow(host) => host.pick_targets_at_screen_json(sx, sy),
        NodeGraphEngine::Dag(host) => host.pick_targets_at_screen_json(sx, sy),
    };
    let rows = serde_json::from_str::<Vec<Value>>(&targets).ok()?;
    rows.iter().find(|row| row.get("domain").and_then(Value::as_str) == Some("handle")).and_then(|row| row.get("id").and_then(Value::as_str)).map(str::to_owned)
}

/// 🔌️ Reads the accepted live graph owner for React Flow's connection-handle cursor semantic.
/// A wire keeps the centered cursor after leaving its source handle until the live `DrawEdge` owner
/// retires; candidate scene data never participates in this query.
pub fn node_graph_connection_cursor_active(surface_id: &str, sx: f64, sy: f64) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        match map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) {
            Some(NodeGraphEngine::Flow(host)) => host.dag.connection_cursor_active_at_screen(sx, sy),
            Some(NodeGraphEngine::Dag(host)) => host.dag.connection_cursor_active_at_screen(sx, sy),
            None => false,
        }
    })
}

//#region 🔗️NodeGraphScreenPointer
// 🖱️ The SECOND pointer path a node-graph surface needs, and the one wgpu did not have.
//
// `DagHost` exposes two pointer entries. `derive_pointer_plan`/`apply_pointer_plan` is the bounded,
// atomically-admitted one — a fixed-capacity projection planned before it is committed — and its
// whole gesture vocabulary is `Idle | Pan | Drag | Select`. `pointer_down_screen`/`_move_screen`/
// `_up_screen` is the one React drives (`🕸️NodeGraph/🟦️.tsx`'s `pointerDownScreen`), and it is the
// only one that hit-tests PORTS and HANDLES (entering `InteractionMode::DrawEdge`), the minimap
// widget, port insertion targets and inline widget sliders.
//
// 🩸️ wgpu called ONLY the bounded entry, whose `bounded_node_hit_index` answers
// `DagInteractionPlanFault::Unsupported` for exactly those four hits — so a press on a port aborted
// the dispatch with a `Structure` fault instead of starting a wire, and clicking the minimap
// navigated nowhere. Wire creation, wire deletion and minimap click-to-navigate were unreachable
// from wgpu's side (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
// `📓️audit-wgpu-parity-2026-09-13.md` gaps #2/#5).
//
// So the discriminator below routes precisely those gestures — and every follow-up move/up while one
// is in flight — through the screen entry, and dispatches what the host did afterwards: the same
// `interactionSelect`/`interactionHover`/`nodeGraphViewport` triple the bounded path publishes, plus
// a `nodeGraphEdit` carrying the wire edits the gesture actually made. React reaches the same end by
// re-publishing the WHOLE snapshot after every gesture (`commitFixture`); a snapshot JSON does not fit
// a 16 KiB bounded action, and it does not have to — `FlowNodeGraphEditOp` already declares the
// narrow `connect` operation, four ids wide.

/// 🚨️ Console ERROR line for a refusal at the engine edge — a scene the editor engine would not take, a member this
/// host could not read. `eprintln!` is a no-op inside a `wasm32-unknown-unknown` Worker.
pub(super) fn engine_canvas_fault_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 🩺️ Console line for a screen-path gesture. `eprintln!` is a no-op inside a
/// `wasm32-unknown-unknown` Worker, so a wire drawn there would otherwise leave no trace at all.
pub(super) fn engine_canvas_debug_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 🖱️ Whether this surface's gesture belongs to the screen pointer path: one is already in flight, or
/// the point itself is one only `pointer_*_screen` can describe.
///
/// 🩸️ `at` is asked on EVERY phase, not just a press. A hover over a port, a wire handle, a port
/// insertion target or the minimap is exactly what `bounded_node_hit_index` answers `Unsupported`
/// for; routing only presses meant a plain MOVE across a port fell through to `plan_pointer`, which
/// faulted with `BoundedActionFault::Structure` and aborted the whole dispatch — measured on 6118 as
/// `wgpu-shell graph move fault surface=procedural-main fault=Structure`, after which the frame loop
/// published nothing further for the rest of the session (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-node-graph-surface-retention-2026-09-13.md`).
///
/// 🫳️ A gesture belongs to the path that STARTED it. A bounded drag whose pointer crossed a port
/// or an inline widget was handed to the screen path mid-flight, and that path's
/// `resync_interaction_projection` rebuilt the projection as IDLE — so the drag's own release saw
/// no gesture to close: no `move` edit was ever journalled and the undo baseline it had armed
/// stayed armed. Measured on 6118: a 90x60 px drag of `radius` published its selection and then
/// nothing at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn node_graph_gesture_is_screen_path(surface_id: &str, at: Option<(f64, f64)>) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else { return false };
        let Some(engine) = entry.node_graph.as_ref() else { return false };
        let (dag, bounded_gesture) = match engine {
            NodeGraphEngine::Flow(host) => (&host.dag, host.bounded_pointer_gesture_active()),
            NodeGraphEngine::Dag(host) => (&host.dag, host.bounded_pointer_gesture_active()),
        };
        if dag.screen_pointer_gesture_active() {
            return true;
        }
        if bounded_gesture {
            return false;
        }
        let Some((sx, sy)) = at else { return false };
        let hit = dag.screen_hit(sx, sy);
        let trace = dag.screen_hit_json(sx, sy);
        let digest = graph_payload_digest(&[trace.as_str()]);
        if entry.published_graph_hit != digest {
            entry.published_graph_hit = digest;
            engine_canvas_debug_log(&format!("[TRACE] wgpu node-graph hit surface={surface_id} sx={sx:.1} sy={sy:.1} trace={trace}"));
        }
        hit.is_screen_path()
    })
}

/// 🔗️ What one screen-path gesture left behind: the wire edits it performed and the interaction
/// state to publish alongside them.
struct GraphScreenPointerOutcome {
    edits: Vec<flow::dag::DagGraphEdit>,
    pub(super) snapshot: GraphInteractionSnapshot,
}

/// 🖱️ Runs one gesture through `DagHost`'s screen pointer entry — the same one React drives — and
/// reads back what it did. The mutation happens INSIDE the caller's reservation window, so the
/// action credits a dispatch needs are already held before the host is allowed to change.
///
/// 🎯️ The gather is this host's own per-gesture pick batch; the selection action
/// written below reports the SAME live selection, so leaving it queued would only
/// let a later caller dispatch the gesture twice.
fn apply_node_graph_screen_pointer(surface_id: &str, intent: flow::dag::DagPointerIntent) -> Result<Option<GraphScreenPointerOutcome>, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(engine) = map.get_mut(surface_id).and_then(|entry| entry.node_graph.as_mut()) else {
            return Ok(None);
        };
        match engine {
            NodeGraphEngine::Flow(host) => match intent.phase {
                flow::dag::DagPointerPhase::Down => host.pointer_down_screen(intent.x, intent.y, intent.button, intent.shift, intent.ctrl_or_meta, intent.alt, intent.pan),
                flow::dag::DagPointerPhase::Move => host.pointer_move_screen(intent.x, intent.y, intent.shift, intent.ctrl_or_meta, intent.alt),
                flow::dag::DagPointerPhase::Up => host.pointer_up_screen(intent.x, intent.y, intent.shift, intent.ctrl_or_meta, intent.alt),
                flow::dag::DagPointerPhase::Leave => host.pointer_cancel_screen(),
            },
            NodeGraphEngine::Dag(host) => match intent.phase {
                flow::dag::DagPointerPhase::Down => host.pointer_down_screen([intent.x, intent.y], intent.button, intent.shift, intent.ctrl_or_meta, intent.alt, intent.pan),
                flow::dag::DagPointerPhase::Move => host.pointer_move_screen(intent.x, intent.y, intent.shift, intent.ctrl_or_meta, intent.alt),
                flow::dag::DagPointerPhase::Up => {
                    host.pointer_up_screen(intent.x, intent.y, intent.shift, intent.ctrl_or_meta, intent.alt);
                    let _ = host.take_selection_gather();
                }
                flow::dag::DagPointerPhase::Leave => host.pointer_cancel_screen(),
            },
        }
        let hovered_handle = graph_hovered_handle(engine, intent.x, intent.y);
        let (edits, node_ids, hovered_id, camera) = match engine {
            NodeGraphEngine::Flow(host) => {
                host.resync_interaction_projection();
                let edits = host.dag.take_graph_edits();
                let _ = host.dag.take_journal_refusal();
                let camera = &host.dag.host_snapshot.camera;
                (edits, host.dag.selected_node_ids(), host.dag.hovered_node_id_ref().map(str::to_owned), [camera.x, camera.y, camera.zoom])
            }
            NodeGraphEngine::Dag(host) => {
                host.resync_interaction_projection();
                let edits = host.dag.take_graph_edits();
                let _ = host.dag.take_journal_refusal();
                let camera = &host.dag.host_snapshot.camera;
                (edits, host.dag.selected_node_ids(), host.dag.hovered_node_id_ref().map(str::to_owned), [camera.x, camera.y, camera.zoom])
            }
        };
        mark_engine_scene_repaint(&mut map, surface_id);
        Ok(Some(GraphScreenPointerOutcome { edits, snapshot: graph_projection_snapshot(node_ids, hovered_id, hovered_handle, camera)? }))
    })
}

/// ✍️ The bounded action builder as a sink of the ONE node-graph row encoder (`flow::dag::write_dag_graph_edit_rows`), so the
/// rows wgpu dispatches are the rows React dispatches from the same journal, written by the same function (design §13.3).
struct BoundedGraphEditRows<'b>(&'b mut ui_wgpu::wgpu::BoundedActionBuilder);

impl<'a> flow::dag::DagGraphEditRowSink<'a> for BoundedGraphEditRows<'_> {
    type Error = ui_wgpu::wgpu::BoundedActionFault;
    fn begin_row(&mut self) -> Result<(), Self::Error> {
        self.0.begin_object(None)
    }
    fn text(&mut self, field: &'static str, value: &'a str) -> Result<(), Self::Error> {
        self.0.string(Some(field), value)
    }
    fn number(&mut self, field: &'static str, value: f64) -> Result<(), Self::Error> {
        self.0.number(Some(field), value)
    }
    fn integer(&mut self, field: &'static str, value: u64) -> Result<(), Self::Error> {
        self.0.integer(Some(field), i64::try_from(value).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?)
    }
    fn texts(&mut self, field: &'static str, values: &'a [String]) -> Result<(), Self::Error> {
        self.0.begin_array(Some(field))?;
        for value in values {
            self.0.string(None, value)?;
        }
        self.0.end_container()
    }
    fn end_row(&mut self) -> Result<(), Self::Error> {
        self.0.end_container()
    }
}

/// 🔗️ Writes the `nodeGraphEdit` one gesture's graph edits ask for — `{operations: [...]}` through the shared row encoder,
/// its string credits priced from the encoder's own string census.
fn write_graph_edit_action(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, edits: &[flow::dag::DagGraphEdit]) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if edits.is_empty() {
        return Ok(());
    }
    let edit_action = "nodeGraphEdit";
    let mut parts: Vec<&str> = vec![controller_id, edit_action, "operations"];
    parts.extend(flow::dag::dag_graph_edit_row_strings(edits));
    let edit_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&parts)?;
    batch.action(controller_id, edit_action, edit_bytes, |builder| {
        builder.begin_object(None)?;
        builder.begin_array(Some("operations"))?;
        flow::dag::write_dag_graph_edit_rows(edits, &mut BoundedGraphEditRows(builder))?;
        builder.end_container()?;
        builder.end_container()
    })
}

/// 🖱️ One gesture on the screen pointer path, dispatched whole: the interaction state this
/// gesture CHANGED plus any wire edit it performed.
///
/// 🩸️ A gesture that can still MUTATE the graph — a press, a release, or a move while a wire draw,
/// minimap drag or widget drag is in flight — reserves its four credits BEFORE the host is allowed to
/// change, because a bounded dispatch must never mutate what it cannot then publish. A plain hover
/// over a port or the minimap can produce no edit at all, so it pays only for the actions it actually
/// changes and, when it changes none, takes no reservation whatsoever — see
/// `📌️GraphInteractionChange` for the sweep that died without that rule.
fn node_graph_screen_pointer_into(surface_id: &str, controller_id: &str, intent: flow::dag::DagPointerIntent, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(wire_surface_id) = engine_surface_wire_id(surface_id) else { return Ok(false) };
    let may_mutate = !matches!(intent.phase, flow::dag::DagPointerPhase::Move) || node_graph_gesture_is_screen_path(surface_id, None);
    if may_mutate {
        let mut reservation = input.reserve_actions(4, 4 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
        let Some(outcome) = apply_node_graph_screen_pointer(surface_id, intent)? else {
            return Ok(false);
        };
        let dispatch = graph_interaction_dispatch(published_graph_interaction(surface_id), outcome.snapshot, node_graph_interaction_domain(surface_id).as_deref())?;
        write_graph_interaction_actions(&mut reservation, wire_surface_id.as_str(), controller_id, &dispatch)?;
        write_graph_edit_action(&mut reservation, controller_id, &outcome.edits)?;
        reservation.publish_partial()?;
        record_graph_interaction(surface_id, dispatch.digest);
        return Ok(true);
    }
    let Some(outcome) = apply_node_graph_screen_pointer(surface_id, intent)? else {
        return Ok(false);
    };
    let dispatch = graph_interaction_dispatch(published_graph_interaction(surface_id), outcome.snapshot, node_graph_interaction_domain(surface_id).as_deref())?;
    let items = dispatch.item_count() + usize::from(!outcome.edits.is_empty());
    if items == 0 {
        return Ok(true);
    }
    let mut reservation = input.reserve_actions(items, items * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    write_graph_interaction_actions(&mut reservation, wire_surface_id.as_str(), controller_id, &dispatch)?;
    write_graph_edit_action(&mut reservation, controller_id, &outcome.edits)?;
    reservation.publish()?;
    record_graph_interaction(surface_id, dispatch.digest);
    Ok(true)
}
//#endregion 🔗️NodeGraphScreenPointer

fn plan_node_graph_pointer(surface_id: &str, intent: flow::dag::DagPointerIntent) -> Result<Option<(NodeGraphPointerPlan, GraphInteractionSnapshot)>, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(engine) = map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) else {
            return Ok(None);
        };
        let hovered_handle = graph_hovered_handle(engine, intent.x, intent.y);
        match engine {
            NodeGraphEngine::Flow(host) => {
                let plan = host.plan_pointer(intent).map_err(graph_plan_fault)?;
                let flow::dag::DagPointerSnapshot { node_ids, hovered_id, camera } = host.pointer_projection_snapshot(&plan).map_err(graph_plan_fault)?;
                Ok(Some((NodeGraphPointerPlan::Flow(plan), graph_projection_snapshot(node_ids, hovered_id, hovered_handle, camera)?)))
            }
            NodeGraphEngine::Dag(host) => {
                let plan = host.plan_pointer(intent).map_err(graph_plan_fault)?;
                let flow::dag::DagPointerSnapshot { node_ids, hovered_id, camera } = host.pointer_projection_snapshot(&plan).map_err(graph_plan_fault)?;
                Ok(Some((NodeGraphPointerPlan::Dag(plan), graph_projection_snapshot(node_ids, hovered_id, hovered_handle, camera)?)))
            }
        }
    })
}

/// 🔄️ Marks one engine surface's painted content stale because a gesture mutated its host directly
/// rather than through a scene field.
///
/// 🩸️ `scene_revision` is the staged raster's whole freshness witness: `EngineGpuCandidate::matches`
/// compares it against the packet it already uploaded, so a host the pointer changed under a
/// revision that did not move composites the PREVIOUS picture. A wire the user just drew, a node
/// they just dragged and a camera a minimap click just moved were all invisible until some unrelated
/// scene field happened to change (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn mark_engine_scene_repaint(registry: &mut EngineSurfaceRegistry, surface_id: &str) {
    if let Some(entry) = registry.get_mut(surface_id) {
        entry.scene_revision = entry.scene_revision.wrapping_add(1);
    }
}

fn commit_node_graph_pointer(surface_id: &str, plan: NodeGraphPointerPlan) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(engine) = map.get_mut(surface_id).and_then(|entry| entry.node_graph.as_mut()) else {
            return false;
        };
        let committed = match (engine, plan) {
            (NodeGraphEngine::Flow(host), NodeGraphPointerPlan::Flow(plan)) => host.commit_pointer(plan),
            (NodeGraphEngine::Dag(host), NodeGraphPointerPlan::Dag(plan)) => host.commit_pointer(plan),
            _ => false,
        };
        if committed {
            mark_engine_scene_repaint(&mut map, surface_id);
        }
        committed
    })
}

fn bounded_graph_selection(dag: &flow::dag::DagHost) -> Result<(Vec<String>, Option<String>), ui_wgpu::wgpu::BoundedActionFault> {
    if dag.selected_node_count() > ui_wgpu::wgpu::action::ACTION_NODE_CAPACITY - 2 {
        return Err(ui_wgpu::wgpu::BoundedActionFault::NodeCredits);
    }
    let hovered = dag.hovered_node_id_ref();
    let mut bytes = 0usize;
    for id in dag.selected_node_id_refs().chain(hovered) {
        if id.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        bytes = bytes.checked_add(id.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        if bytes > ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
    }
    Ok((dag.selected_node_id_refs().map(str::to_owned).collect(), hovered.map(str::to_owned)))
}

fn graph_interaction_snapshot(entry: &EngineSurface, camera: Option<[f64; 3]>) -> Result<GraphInteractionSnapshot, ui_wgpu::wgpu::BoundedActionFault> {
    let (node_ids, hovered_id, current_camera) = match entry.node_graph.as_ref() {
        Some(NodeGraphEngine::Flow(host)) => {
            let (selected, hovered) = bounded_graph_selection(&host.dag)?;
            (selected, hovered, [host.host_snapshot.camera.x, host.host_snapshot.camera.y, host.host_snapshot.camera.zoom])
        }
        Some(NodeGraphEngine::Dag(host)) => {
            let (selected, hovered) = bounded_graph_selection(&host.dag)?;
            (selected, hovered, [host.dag.host_snapshot.camera.x, host.dag.host_snapshot.camera.y, host.dag.host_snapshot.camera.zoom])
        }
        None => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
    };
    graph_projection_snapshot(node_ids, hovered_id, None, camera.unwrap_or(current_camera))
}

//#region 📌️GraphInteractionChange
// 📮️ WHAT A POINTER MOVE IS ALLOWED TO COST.
//
// Every pointer move over a node graph published `interactionSelect` + `interactionHover` +
// `nodeGraphViewport` whether or not any of the three had changed. The bounded action queue holds
// `ACTION_QUEUE_ITEM_CAPACITY` = 256 items and is drained one step per host turn, so a hover sweep
// denser than the frame loop filled it: `BoundedActionFault::ItemCredits` at admission, after which
// the session published NOTHING for the rest of its life — measured on 6118 as one
// `graph move fault … fault=ItemCredits` followed by 0 of 440 sweep points and 0 of 6 presses ever
// reaching the host (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-gestures-2026-09-13.md`).
//
// A plain move is not a mutation and must not be priced like one. The surface remembers the
// fingerprint of each action it last dispatched, a gesture publishes only the ones whose payload
// actually changed, and it reserves exactly that many credits — zero of them when nothing changed,
// in which case no reservation is taken at all and the plan is simply committed. React reaches the
// same end structurally: `emitIntent` answers `undefined` for an unchanged intent, and
// `NodeGraphHost`'s hover effect is keyed on the hovered id rather than on the pointer.

/// 📌️ The fingerprints of the three interaction actions a node-graph surface last dispatched.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct PublishedGraphInteraction {
    pub(super) select: u64,
    pub(super) hover: u64,
    pub(super) viewport: u64,
}

/// 🔢️ FNV-1a over a payload's own bytes — the change witness is the dispatched payload itself,
/// never a hand-maintained shadow of the fields that feed it.
fn graph_payload_digest(parts: &[&str]) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    for part in parts {
        for byte in part.as_bytes().iter().chain(std::iter::once(&0xff)) {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}

/// 📮️ One gesture's interaction payloads, with the three flags that say which of them this surface
/// has not already told the guest.
struct GraphInteractionDispatch {
    pub(super) domain_id: Option<String>,
    select_targets: String,
    hover_targets: String,
    pub(super) viewport: semio_framework_os_kernel::Viewport2d,
    digest: PublishedGraphInteraction,
    pub(super) publish_select: bool,
    pub(super) publish_hover: bool,
    publish_viewport: bool,
}

impl GraphInteractionDispatch {
    pub(super) fn item_count(&self) -> usize {
        usize::from(self.publish_select) + usize::from(self.publish_hover) + usize::from(self.publish_viewport)
    }
}

fn node_graph_interaction_domain(surface_id: &str) -> Option<Arc<ui_wgpu::wgpu::NodeGraphInteractionDomain>> {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).and_then(|entry| entry.sync_cache.interaction_domain.clone()))
}

fn node_graph_target_prefix<'a>(domain: &'a ui_wgpu::wgpu::NodeGraphInteractionDomain, granularity: &str) -> Option<&'a str> {
    match granularity {
        "node" => Some(&domain.node_target_prefix),
        "edge" => Some(&domain.edge_target_prefix),
        "handle" => Some(&domain.handle_target_prefix),
        _ => None,
    }
}

pub(super) fn node_graph_domain_targets_json(domain: &ui_wgpu::wgpu::NodeGraphInteractionDomain, granularity: &str, ids: &[String]) -> Result<String, ui_wgpu::wgpu::BoundedActionFault> {
    let prefix = node_graph_target_prefix(domain, granularity).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    serde_json::to_string(&ids.iter().map(|id| json!({ "granularity": granularity, "id": format!("{prefix}{id}") })).collect::<Vec<_>>()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
}

pub(super) fn graph_interaction_dispatch(published: PublishedGraphInteraction, snapshot: GraphInteractionSnapshot, domain: Option<&ui_wgpu::wgpu::NodeGraphInteractionDomain>) -> Result<GraphInteractionDispatch, ui_wgpu::wgpu::BoundedActionFault> {
    let select_targets = match domain {
        Some(domain) => node_graph_domain_targets_json(domain, "node", &snapshot.node_ids)?,
        None => "[]".into(),
    };
    let hover_targets = match (domain, snapshot.hovered_handle.as_ref(), snapshot.hovered_id.as_ref()) {
        (Some(domain), Some(handle), _) => node_graph_domain_targets_json(domain, "handle", std::slice::from_ref(handle))?,
        (Some(domain), None, Some(node)) => node_graph_domain_targets_json(domain, "node", std::slice::from_ref(node))?,
        _ => "[]".into(),
    };
    let viewport_text = format!("{}|{}|{}", snapshot.viewport.x, snapshot.viewport.y, snapshot.viewport.zoom);
    let domain_id = domain.map(|domain| domain.id.clone());
    let digest = PublishedGraphInteraction {
        select: graph_payload_digest(&[domain_id.as_deref().unwrap_or_default(), select_targets.as_str()]),
        hover: graph_payload_digest(&[domain_id.as_deref().unwrap_or_default(), hover_targets.as_str()]),
        viewport: graph_payload_digest(&[viewport_text.as_str()]),
    };
    Ok(GraphInteractionDispatch {
        domain_id,
        select_targets,
        hover_targets,
        viewport: snapshot.viewport,
        digest,
        publish_select: domain.is_some() && digest.select != published.select,
        publish_hover: domain.is_some() && digest.hover != published.hover,
        publish_viewport: digest.viewport != published.viewport,
    })
}

/// 🗺️ One line naming where every node and port of a graph surface IS, emitted once per CHANGE of
/// that geometry rather than once per frame — the census a browser probe needs so that finding the
/// graph costs nothing, instead of costing the host's whole pointer budget in a hover sweep.
pub(super) fn log_graph_geometry_census(surface_id: &str) {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else { return };
        let Some(engine) = entry.node_graph.as_ref() else { return };
        let census = match engine {
            NodeGraphEngine::Flow(host) => host.dag.screen_geometry_census_json(),
            NodeGraphEngine::Dag(host) => host.dag.screen_geometry_census_json(),
        };
        let digest = graph_payload_digest(&[census.as_str()]);
        if entry.published_graph_geometry == digest {
            return;
        }
        entry.published_graph_geometry = digest;
        engine_canvas_debug_log(&format!("[TRACE] wgpu node-graph geometry surface={surface_id} entities={census}"));
    });
}

pub(super) fn published_graph_interaction(surface_id: &str) -> PublishedGraphInteraction {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).map(|entry| entry.published_graph_interaction).unwrap_or_default())
}

fn record_graph_interaction(surface_id: &str, digest: PublishedGraphInteraction) {
    ENGINE_SURFACES.with(|cell| {
        if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
            entry.published_graph_interaction = digest;
        }
    });
}
//#endregion 📌️GraphInteractionChange

fn write_graph_interaction_actions(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, surface_id: &str, controller_id: &str, dispatch: &GraphInteractionDispatch) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if dispatch.publish_select {
        let domain_id = dispatch.domain_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let select_action = "interactionSelect";
        let select_targets = dispatch.select_targets.as_str();
        let select_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, select_action, "domainId", domain_id, "targets", select_targets, "merge", "replace", "method", "pick"])?;
        batch.action(controller_id, select_action, select_bytes, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain_id)?;
            builder.string(Some("targets"), select_targets)?;
            builder.string(Some("merge"), "replace")?;
            builder.string(Some("method"), "pick")?;
            builder.end_container()
        })?;
    }
    if dispatch.publish_hover {
        let domain_id = dispatch.domain_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let hover_action = "interactionHover";
        let hover_targets = dispatch.hover_targets.as_str();
        let hover_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, hover_action, "domainId", domain_id, "channel", "pointer", "targets", hover_targets])?;
        batch.action(controller_id, hover_action, hover_bytes, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain_id)?;
            builder.string(Some("channel"), "pointer")?;
            builder.string(Some("targets"), hover_targets)?;
            builder.end_container()
        })?;
    }
    if dispatch.publish_viewport {
        let viewport_action = "nodeGraphViewport";
        let viewport = dispatch.viewport;
        let viewport_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, viewport_action, "surfaceId", surface_id, "viewport", "x", "y", "zoom"])?;
        batch.action(controller_id, viewport_action, viewport_bytes, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface_id)?;
            builder.begin_object(Some("viewport"))?;
            builder.number(Some("x"), viewport.x)?;
            builder.number(Some("y"), viewport.y)?;
            builder.number(Some("zoom"), viewport.zoom)?;
            builder.end_container()?;
            builder.end_container()
        })?;
    }
    Ok(())
}

fn world_to_screen_inner(inner: Rect, cam_x: f64, cam_y: f64, zoom: f64, wx: f64, wy: f64) -> (f32, f32) {
    let zoom = zoom.max(0.05) as f32;
    let cx = inner.w * 0.5;
    let cy = inner.h * 0.5;
    let sx = inner.x + (wx - cam_x) as f32 * zoom + cx;
    let sy = inner.y + (wy - cam_y) as f32 * zoom + cy;
    (sx, sy)
}

const DAG_LABEL_SCREEN_PX: f32 = 11.0;
const LABEL_INSET: f32 = 0.88;

struct LabelInteractionChrome {
    selected_ids: HashSet<String>,
    highlighted_ids: HashSet<String>,
    pub(super) hovered_id: Option<String>,
    dimmed_ids: Vec<String>,
}

fn label_chrome_from_flow(host: &FlowHost) -> LabelInteractionChrome {
    let selected=host.selected_widget_ids();
    let preselect: Value = serde_json::from_str(&host.preselect_widget_ids_json()).unwrap_or(json!({}));
    let pre_ids: Vec<String> = preselect.get("ids").and_then(|v| v.as_array()).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let removed: Vec<String> = preselect.get("removedIds").and_then(|v| v.as_array()).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let (selected_ids, highlighted_ids) = if pre_ids.is_empty() && removed.is_empty() { (selected.into_iter().collect(), HashSet::new()) } else { (pre_ids.into_iter().collect(), removed.into_iter().collect()) };
    LabelInteractionChrome { selected_ids, highlighted_ids, hovered_id: host.hovered_widget_id(), dimmed_ids: host.preview_off_widget_ids() }
}

fn label_chrome_from_graph(host: &GraphHost) -> LabelInteractionChrome {
    let selected = host.dag.selected_node_ids();
    let pre_ids = host.dag.preselect_widget_ids();
    let removed = host.dag.preselect_removed_widget_ids();
    let (selected_ids, highlighted_ids) = if pre_ids.is_empty() && removed.is_empty() { (selected.into_iter().collect(), HashSet::new()) } else { (pre_ids.into_iter().collect(), removed.into_iter().collect()) };
    LabelInteractionChrome { selected_ids, highlighted_ids, hovered_id: host.dag.hovered_node_id(), dimmed_ids: Vec::new() }
}

/// 🔠️ Smallest font an overlay label is allowed to shrink to. Below this a caption is a smudge, not
/// a word — which is why width is answered by `canvas::text::ellipsize_by_measure` and never by
/// shrinking further.
const LABEL_LEGIBLE_MIN_PX: f32 = ui_styling::metrics::label::LEGIBLE_MIN_PX as f32;

/// 📐️ Overlay label font size: the row's own target, shrunk ONLY to fit the row's height, never
/// below {@link LABEL_LEGIBLE_MIN_PX}. Width is not a font decision — see
/// {@link fit_overlay_label_text}. This used to binary-search the font down to 4 px on width too,
/// which is how a node title wider than its body became an unreadable smear.
fn clamp_label_font_px(atlas: &mut FontAtlas, text: &str, target_px: f32, max_h: f32) -> f32 {
    let px = target_px.max(LABEL_LEGIBLE_MIN_PX).round();
    if atlas.measure_text(text, px).1 * 1.2 <= max_h {
        return px;
    }
    let mut low = LABEL_LEGIBLE_MIN_PX;
    let mut high = px;
    let mut best = LABEL_LEGIBLE_MIN_PX;
    while low <= high {
        let mid = ((low + high) * 0.5).floor();
        if atlas.measure_text(text, mid).1 * 1.2 <= max_h {
            best = mid;
            low = mid + 1.0;
        } else {
            high = mid - 1.0;
        }
    }
    best
}

/// ✂️ The wgpu shell's half of the shared caption law: clip by the GLYPH ATLAS's own measure. Its
/// browser twin is `dagEllipsizeOverlayLabel` in `🕸️NodeGraph/🟦️.tsx`, and both are pinned to the
/// rows of `🧰️framework/🔨️modules/🖼️canvas/🧫️fixtures/🏷️label-fit/🔣️.json`.
fn fit_overlay_label_text(atlas: &mut FontAtlas, text: &str, font_px: f32, max_w: f32) -> String {
    canvas::text::ellipsize_by_measure(text, f64::from(max_w), |candidate| f64::from(atlas.measure_text(candidate, font_px).0))
}

fn label_overlay_fill(theme: &Theme, node_id: &str, ghost: bool, chrome: &LabelInteractionChrome) -> Rgba {
    if ghost {
        return theme.text_muted;
    }
    if chrome.dimmed_ids.iter().any(|id| id == node_id) {
        return theme.text_muted.with_alpha(0.5);
    }
    if chrome.selected_ids.contains(node_id) {
        return theme.active_foreground;
    }
    if chrome.highlighted_ids.contains(node_id) {
        return theme.text_muted;
    }
    if chrome.hovered_id.as_deref() == Some(node_id) {
        return theme.active_foreground;
    }
    theme.text_element
}

/// 📐️ The host publishes the caption's own screen budget (`maxScreenW`) because only it knows
/// whether the caption sits INSIDE the node body or above it; `nodeW` is the fallback.
fn paint_label_overlay_row(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, cam_x: f64, cam_y: f64, zoom: f64, row: &Value, chrome: &LabelInteractionChrome) {
    let Some(text) = row.get("text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) else {
        return;
    };
    let wx = row.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let wy = row.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let node_w = row.get("nodeW").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let node_h = row.get("nodeH").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let align = row.get("align").and_then(|v| v.as_str());
    let ghost = row.get("ghost").and_then(|v| v.as_bool()).unwrap_or(false);
    let node_id = row.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let is_port = row.get("kind").and_then(|v| v.as_str()) == Some("port") || matches!(align, Some("left") | Some("right"));
    let zoom_f = zoom.max(0.05) as f32;
    let max_w = row.get("maxScreenW").and_then(|v| v.as_f64()).filter(|w| *w > 0.0).unwrap_or_else(|| (node_w * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0)) as f32;
    let max_h = if is_port {
        row.get("maxScreenH").and_then(|v| v.as_f64()).filter(|h| *h > 0.0).map(|h| h as f32).unwrap_or((node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32)
    } else {
        (node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32
    };
    let target_px = row.get("fontScreenPx").and_then(|v| v.as_f64()).filter(|px| *px > 0.0).map(|px| px as f32).unwrap_or(DAG_LABEL_SCREEN_PX);
    let font_px = clamp_label_font_px(&mut ctx.atlas, text, target_px, max_h);
    let fitted = fit_overlay_label_text(&mut ctx.atlas, text, font_px, max_w);
    let text = fitted.as_str();
    let (anchor_x, anchor_y) = world_to_screen_inner(inner, cam_x, cam_y, zoom, wx, wy);
    let (text_w, text_h) = ctx.atlas.measure_text(text, font_px);
    let tx = match align {
        Some("left") => anchor_x,
        Some("right") => anchor_x - text_w,
        _ => anchor_x - text_w * 0.5,
    };
    let ty = anchor_y + text_h * 0.5;
    let fill = label_overlay_fill(ctx.theme, node_id, ghost, chrome);
    let alpha = if ghost {
        0.85
    } else if chrome.dimmed_ids.iter().any(|id| id == node_id) {
        0.5
    } else {
        1.0
    };
    draw_text_overlay(ctx, text, tx, ty, font_px, fill.with_alpha(fill.a * alpha));
}

pub fn paint_node_graph_labels(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, inner: Rect) {
    let snapshot = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(&scene.host_id)?;
        match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => {
                let state_json = host.label_overlay_paint_state_json().ok()?;
                Some((state_json, label_chrome_from_flow(host)))
            }
            Some(NodeGraphEngine::Dag(host)) => {
                let state_json = host.label_overlay_paint_state_json().ok()?;
                Some((state_json, label_chrome_from_graph(host)))
            }
            None => None,
        }
    });
    let Some((state_json, chrome)) = snapshot else {
        return;
    };
    let Ok(state) = serde_json::from_str::<Value>(&state_json) else {
        return;
    };
    let cam = state.get("camera").cloned().unwrap_or(json!({}));
    let cam_x = cam.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let cam_y = cam.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let zoom = cam.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let labels = state.get("labels").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    for row in &labels {
        paint_label_overlay_row(ctx, inner, cam_x, cam_y, zoom, row, &chrome);
    }
}

struct NodeGraphOverlaySnapshot {
    preview_points_json: String,
    preview_crossing: bool,
    preview_method: String,
    selection_bounds_json: String,
}

fn node_graph_overlay_snapshot(surface_id: &str) -> Option<NodeGraphOverlaySnapshot> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => Some(NodeGraphOverlaySnapshot {
                preview_points_json: host.selection_preview_points_json(),
                preview_crossing: host.selection_preview_crossing(),
                preview_method: host.selection_preview_method().to_string(),
                selection_bounds_json: host.selection_union_bounds_screen_json(),
            }),
            Some(NodeGraphEngine::Dag(host)) => Some(NodeGraphOverlaySnapshot {
                preview_points_json: host.dag.selection_preview_points_json(),
                preview_crossing: host.dag.selection_preview_crossing(),
                preview_method: host.dag.selection_preview_method().to_string(),
                selection_bounds_json: host.dag.selection_union_bounds_screen_json(),
            }),
            None => None,
        }
    })
}

fn parse_selection_preview_points(json: &str) -> Vec<(f32, f32)> {
    serde_json::from_str::<Vec<[f64; 2]>>(json).unwrap_or_default().into_iter().map(|point| (point[0] as f32, point[1] as f32)).collect()
}

fn paint_node_graph_selection_marquee(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, points: &[(f32, f32)], crossing: bool, method: &str, theme: &Theme) {
    if points.len() < 2 {
        return;
    }
    let lasso = method == "lasso";
    let global: Vec<[f32; 2]> = points.iter().map(|(x, y)| [inner.x + x, inner.y + y]).collect();
    ui_wgpu::wgpu::paint_selection_marquee(&mut ctx.draw, theme, crossing, lasso, &global, true);
}

/// 🖌️🧭️ The two paint-2d overlays React layers OVER its canvas: a live marquee
/// (`SelectionMarquee`, `🖌️Paint2dHost/🟦️.tsx:178`) and the navigator's "you are here" viewport
/// rectangle (`overlayRect`, `:275`). Both are chrome — neither belongs in the vello raster the
/// composite phase pushes — so they are painted here, in the scene's own overlay phase.
pub fn paint_paint2d_overlays(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, inner: Rect) {
    let Some(paint) = scene.paint_2d.as_ref() else {
        return;
    };
    let theme = *ctx.theme;
    if let Some(rect) = paint2d_navigator_overlay_rect(scene) {
        let bounds = [inner.x + rect[0], inner.y + rect[1], rect[2], rect[3]];
        ctx.draw.push_solid(bounds, theme.accent.with_alpha(0.12));
        ctx.draw.push_line(bounds[0], bounds[1], bounds[0] + bounds[2], bounds[1], theme.accent, theme.stroke_hairline);
        ctx.draw.push_line(bounds[0], bounds[1] + bounds[3], bounds[0] + bounds[2], bounds[1] + bounds[3], theme.accent, theme.stroke_hairline);
        ctx.draw.push_line(bounds[0], bounds[1], bounds[0], bounds[1] + bounds[3], theme.accent, theme.stroke_hairline);
        ctx.draw.push_line(bounds[0] + bounds[2], bounds[1], bounds[0] + bounds[2], bounds[1] + bounds[3], theme.accent, theme.stroke_hairline);
        return;
    }
    if let Some((points, crossing, lasso)) = paint2d_marquee_overlay(&scene.host_id, &paint.active_utility) {
        ui_wgpu::wgpu::paint_selection_marquee(ctx.draw, &theme, crossing, lasso, &points, true);
    }
}

fn paint_node_graph_selection_bounds(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, bounds_json: &str, theme: &Theme) {
    if bounds_json.trim() == "null" {
        return;
    }
    let Ok(value) = serde_json::from_str::<Value>(bounds_json) else {
        return;
    };
    let x = value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let y = value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let w = value.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let h = value.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let rx = inner.x + x;
    let ry = inner.y + y;
    let stroke = theme.text_element.with_alpha(0.95);
    ctx.draw.push_line_overlay(rx, ry, rx + w, ry, stroke, 1.0);
    ctx.draw.push_line_overlay(rx + w, ry, rx + w, ry + h, stroke, 1.0);
    ctx.draw.push_line_overlay(rx + w, ry + h, rx, ry + h, stroke, 1.0);
    ctx.draw.push_line_overlay(rx, ry + h, rx, ry, stroke, 1.0);
}

pub fn paint_node_graph_overlays(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, inner: Rect) {
    let Some(snapshot) = node_graph_overlay_snapshot(&scene.host_id) else {
        return;
    };
    let points = parse_selection_preview_points(&snapshot.preview_points_json);
    paint_node_graph_selection_marquee(ctx, inner, &points, snapshot.preview_crossing, &snapshot.preview_method, ctx.theme);
    paint_node_graph_selection_bounds(ctx, inner, &snapshot.selection_bounds_json, ctx.theme);
}
//#endregion NodeGraph
