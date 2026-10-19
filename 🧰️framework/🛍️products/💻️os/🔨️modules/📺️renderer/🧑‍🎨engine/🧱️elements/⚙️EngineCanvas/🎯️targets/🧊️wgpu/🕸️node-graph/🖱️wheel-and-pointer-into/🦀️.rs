//! 🖱️ Wheel and pointer into: the wheel, pointer down, move, up and cancel writers, the bounded publish, the graph edit planning, the interaction snapshot and the connection cursor. See [`super`].

use super::*;

//#region NodeGraph
pub fn node_graph_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, _ctrl: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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
    let mut reservation = input.reserve_actions(items, items * semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY)?;
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
                    dag_set_wheel_zoom_active(&mut host.dag.viewport, &mut host.dag.graph, true);
                    true
                }
                (NodeGraphEngine::Dag(host), NodeGraphWheelPlan::Dag(plan)) => {
                    if !host.commit_wheel(plan) {
                        return false;
                    }
                    dag_set_wheel_zoom_active(&mut host.dag.viewport, &mut host.dag.graph, true);
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
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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

pub fn node_graph_pointer_move_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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
fn node_graph_bounded_publish(surface_id: &str, controller_id: &str, plan: NodeGraphPointerPlan, snapshot: GraphInteractionSnapshot, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let edits = plan_node_graph_edits(surface_id, &plan);
    let dispatch = graph_interaction_dispatch(published_graph_interaction(surface_id), snapshot, node_graph_interaction_domain(surface_id).as_deref())?;
    let items = dispatch.item_count() + usize::from(!edits.is_empty());
    if items == 0 {
        return Ok(commit_node_graph_pointer(surface_id, plan));
    }
    let Some(wire_surface_id) = engine_surface_wire_id(surface_id) else { return Ok(false) };
    let mut reservation = input.reserve_actions(items, items * semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY)?;
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
            (NodeGraphEngine::Flow(host), NodeGraphPointerPlan::Flow(plan)) => host.dag.graph.plan_graph_edits(plan),
            (NodeGraphEngine::Dag(host), NodeGraphPointerPlan::Dag(plan)) => host.dag.graph.plan_graph_edits(plan),
            _ => Vec::new(),
        }
    })
}

pub fn node_graph_pointer_up_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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

pub struct GraphInteractionSnapshot {
    pub node_ids: Vec<String>,
    pub hovered_id: Option<String>,
    /// 🔌️ The `"{nodeId}@{portId}"` channel under the pointer, when the pick landed on a port rather
    /// than a node body. React qualifies its hover target the same way — `nodeGraphHoverActionArgs`
    /// emits `granularity: "handle"` with exactly this id — which is what makes hover bidirectional
    /// between the graph and a `World3d` preview keyed by the same channel id.
    pub hovered_handle: Option<String>,
    pub viewport: semio_framework_os_kernel::Viewport2d,
}

enum NodeGraphWheelPlan {
    Flow(flow::FlowWheelPlan),
    Dag(semio_framework_os_node_graph::GraphWheelPlan),
}

pub(super) enum NodeGraphPointerPlan {
    Flow(flow::dag::DagPointerPlan),
    Dag(flow::dag::DagPointerPlan),
}

impl NodeGraphWheelPlan {
    pub(in super::super) fn camera(&self) -> [f64; 3] {
        match self {
            Self::Flow(plan) => plan.camera(),
            Self::Dag(plan) => plan.camera(),
        }
    }
}

pub(super) fn graph_plan_fault(fault: flow::dag::DagInteractionPlanFault) -> semio_framework_ui_action::BoundedActionFault {
    match fault {
        flow::dag::DagInteractionPlanFault::NodeCredits => semio_framework_ui_action::BoundedActionFault::NodeCredits,
        flow::dag::DagInteractionPlanFault::StringCredits => semio_framework_ui_action::BoundedActionFault::StringCredits,
        flow::dag::DagInteractionPlanFault::Unsupported => semio_framework_ui_action::BoundedActionFault::Structure,
    }
}

pub fn graph_projection_snapshot(node_ids: Vec<String>, hovered_id: Option<String>, hovered_handle: Option<String>, camera: [f64; 3]) -> Result<GraphInteractionSnapshot, semio_framework_ui_action::BoundedActionFault> {
    let viewport = semio_framework_os_kernel::Viewport2d { x: camera[0], y: camera[1], zoom: camera[2] };
    viewport.validate().map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    let mut parts = Vec::with_capacity(node_ids.len() + 2);
    parts.extend([hovered_id.as_deref().unwrap_or_default(), hovered_handle.as_deref().unwrap_or_default()]);
    parts.extend(node_ids.iter().map(String::as_str));
    semio_framework_ui_action::checked_action_string_bytes(&parts)?;
    Ok(GraphInteractionSnapshot { node_ids, hovered_id, hovered_handle, viewport })
}

/// 🔌️ The channel the DAG's own hit-test reports under a screen point, in the `"{nodeId}@{portId}"`
/// pick-id grammar. A pure query on the live host — the bounded-action protocol has to know the
/// payload BEFORE the plan is committed, so hover qualification cannot be read back off the commit.
///
/// @see `🧱️elements/🕸️NodeGraph/🟦️.tsx` — `nodeGraphPickChannel`
pub(super) fn graph_hovered_handle(engine: &NodeGraphEngine, sx: f64, sy: f64) -> Option<String> {
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
            Some(NodeGraphEngine::Flow(host)) => connection_cursor_active_at_screen(&host.dag.viewport, &host.dag.graph, &host.dag.minimap, sx, sy),
            Some(NodeGraphEngine::Dag(host)) => connection_cursor_active_at_screen(&host.dag.viewport, &host.dag.graph, &host.dag.minimap, sx, sy),
            None => false,
        }
    })
}
//#endregion NodeGraph
