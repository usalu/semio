//! 🔗️ Screen pointer and commit: the screen path gesture, the screen pointer outcome and application, the bounded edit rows, the pointer planning and commit and the interaction snapshot. See [`super`].

use super::*;

//#region NodeGraph
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
pub(super) fn node_graph_gesture_is_screen_path(surface_id: &str, at: Option<(f64, f64)>) -> bool {
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
        let hit = screen_hit(&dag.viewport, &dag.graph, &dag.minimap, sx, sy);
        let trace = screen_hit_json(&dag.viewport, &dag.graph, &dag.minimap, sx, sy);
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
    pub(super) edits: Vec<flow::dag::DagGraphEdit>,
    pub(in super::super) snapshot: GraphInteractionSnapshot,
}

/// 🖱️ Runs one gesture through `DagHost`'s screen pointer entry — the same one React drives — and
/// reads back what it did. The mutation happens INSIDE the caller's reservation window, so the
/// action credits a dispatch needs are already held before the host is allowed to change.
///
/// 🎯️ The gather is this host's own per-gesture pick batch; the selection action
/// written below reports the SAME live selection, so leaving it queued would only
/// let a later caller dispatch the gesture twice.
fn apply_node_graph_screen_pointer(surface_id: &str, intent: flow::dag::DagPointerIntent) -> Result<Option<GraphScreenPointerOutcome>, semio_framework_ui_action::BoundedActionFault> {
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
                let edits = host.dag.graph.take_graph_edits();
                let _ = host.dag.graph.take_journal_refusal();
                let camera = &host.dag.graph.host_snapshot.camera;
                (edits, host.dag.graph.selected_node_ids(), hovered_node_id_ref(&host.dag.viewport, &host.dag.graph).map(str::to_owned), [camera.x, camera.y, camera.zoom])
            }
            NodeGraphEngine::Dag(host) => {
                host.resync_interaction_projection();
                let edits = host.dag.graph.take_graph_edits();
                let _ = host.dag.graph.take_journal_refusal();
                let camera = &host.dag.graph.host_snapshot.camera;
                (edits, host.dag.graph.selected_node_ids(), hovered_node_id_ref(&host.dag.viewport, &host.dag.graph).map(str::to_owned), [camera.x, camera.y, camera.zoom])
            }
        };
        mark_engine_scene_repaint(&mut map, surface_id);
        Ok(Some(GraphScreenPointerOutcome { edits, snapshot: graph_projection_snapshot(node_ids, hovered_id, hovered_handle, camera)? }))
    })
}

/// ✍️ The bounded action builder as a sink of the ONE node-graph row encoder (`flow::dag::write_dag_graph_edit_rows`), so the
/// rows wgpu dispatches are the rows React dispatches from the same journal, written by the same function (design §13.3).
struct BoundedGraphEditRows<'b>(&'b mut semio_framework_ui_action::BoundedActionBuilder);

impl<'a> flow::dag::DagGraphEditRowSink<'a> for BoundedGraphEditRows<'_> {
    type Error = semio_framework_ui_action::BoundedActionFault;
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
        self.0.integer(Some(field), i64::try_from(value).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?)
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
pub(super) fn write_graph_edit_action(batch: &mut semio_framework_ui_action::BoundedActionBatchReservation<'_>, controller_id: &str, edits: &[flow::dag::DagGraphEdit]) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    if edits.is_empty() {
        return Ok(());
    }
    let edit_action = "nodeGraphEdit";
    let mut parts: Vec<&str> = vec![controller_id, edit_action, "operations"];
    parts.extend(flow::dag::dag_graph_edit_row_strings(edits));
    let edit_bytes = semio_framework_ui_action::checked_action_string_bytes(&parts)?;
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
pub(super) fn node_graph_screen_pointer_into(surface_id: &str, controller_id: &str, intent: flow::dag::DagPointerIntent, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(wire_surface_id) = engine_surface_wire_id(surface_id) else { return Ok(false) };
    let may_mutate = !matches!(intent.phase, flow::dag::DagPointerPhase::Move) || node_graph_gesture_is_screen_path(surface_id, None);
    if may_mutate {
        let mut reservation = input.reserve_actions(4, 4 * semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY)?;
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
    let mut reservation = input.reserve_actions(items, items * semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY)?;
    write_graph_interaction_actions(&mut reservation, wire_surface_id.as_str(), controller_id, &dispatch)?;
    write_graph_edit_action(&mut reservation, controller_id, &outcome.edits)?;
    reservation.publish()?;
    record_graph_interaction(surface_id, dispatch.digest);
    Ok(true)
}
//#endregion 🔗️NodeGraphScreenPointer

pub(super) fn plan_node_graph_pointer(surface_id: &str, intent: flow::dag::DagPointerIntent) -> Result<Option<(NodeGraphPointerPlan, GraphInteractionSnapshot)>, semio_framework_ui_action::BoundedActionFault> {
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
pub(super) fn mark_engine_scene_repaint(registry: &mut EngineSurfaceRegistry, surface_id: &str) {
    if let Some(entry) = registry.get_mut(surface_id) {
        entry.scene_revision = entry.scene_revision.wrapping_add(1);
    }
}

pub(super) fn commit_node_graph_pointer(surface_id: &str, plan: NodeGraphPointerPlan) -> bool {
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

fn bounded_graph_selection(dag: &flow::dag::DagHost) -> Result<(Vec<String>, Option<String>), semio_framework_ui_action::BoundedActionFault> {
    if dag.graph.selected_node_count() > semio_framework_ui_action::ACTION_NODE_CAPACITY - 2 {
        return Err(semio_framework_ui_action::BoundedActionFault::NodeCredits);
    }
    let hovered = hovered_node_id_ref(&dag.viewport, &dag.graph);
    let mut bytes = 0usize;
    for id in dag.graph.selected_node_id_refs().chain(hovered) {
        if id.len() > semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::StringCredits);
        }
        bytes = bytes.checked_add(id.len()).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
        if bytes > semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
        }
    }
    Ok((dag.graph.selected_node_id_refs().map(str::to_owned).collect(), hovered.map(str::to_owned)))
}

pub(super) fn graph_interaction_snapshot(entry: &EngineSurface, camera: Option<[f64; 3]>) -> Result<GraphInteractionSnapshot, semio_framework_ui_action::BoundedActionFault> {
    let (node_ids, hovered_id, current_camera) = match entry.node_graph.as_ref() {
        Some(NodeGraphEngine::Flow(host)) => {
            let (selected, hovered) = bounded_graph_selection(&host.dag)?;
            (selected, hovered, [host.host_snapshot.camera.x, host.host_snapshot.camera.y, host.host_snapshot.camera.zoom])
        }
        Some(NodeGraphEngine::Dag(host)) => {
            let (selected, hovered) = bounded_graph_selection(&host.dag)?;
            (selected, hovered, [host.dag.graph.host_snapshot.camera.x, host.dag.graph.host_snapshot.camera.y, host.dag.graph.host_snapshot.camera.zoom])
        }
        None => return Err(semio_framework_ui_action::BoundedActionFault::Structure),
    };
    graph_projection_snapshot(node_ids, hovered_id, None, camera.unwrap_or(current_camera))
}
//#endregion NodeGraph
