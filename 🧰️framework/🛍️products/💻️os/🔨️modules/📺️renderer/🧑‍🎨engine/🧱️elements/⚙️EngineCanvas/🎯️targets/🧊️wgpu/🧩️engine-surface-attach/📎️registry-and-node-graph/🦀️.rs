//! 📎️ Registry and node graph: the surface kind detail and registration, the attached surface registry, the node graph engine helpers and the node graph engine sync. See [`super`].

use super::*;

//#region 🧩️EngineSurfaceAttach
/// 🧩️ The per-kind payload one attached surface projects into the shell's own bespoke pointer state
/// map — `NodeGraphSurface`/`TiledMapSurface`/`Board2dSurface` each carry a different tail beyond
/// bounds+controller, and `World3d` carries the scene's own compute-status document because the
/// CANCEL CONTRACT lives there: `World3dScene.statusJson`'s `cancellable`/`cancelAction` pair is what
/// the shell offers an affordance for, and `World3dState` (the shell's entry) keeps geometry, never
/// status (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// @see `🔨️modules/🖱️ui/🎬️scene/🟦️.ts` — `World3dComputeStatusV1`/`world3dComputeStatusV1`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineSurfaceKindDetail {
    NodeGraph,
    TiledMap { selection_method: String },
    Board2d { snapshot_json: String },
    World3d { status_json: Option<String> },
}

/// 🧩️ One live engine surface a paint pass attached, projected for the shell's own
/// `node_graph_states`/`tiled_map_states`/`board2d_states` — the bounds its pointer/wheel dispatch
/// hit-tests against and the controller every observation is addressed to.
///
/// 🧲️ This is a REFRESH, never a lease. Retained painting means a window whose document did not
/// change does not repaint, so absence from a drain says only "this window did not repaint" — the
/// shell keeps the surface alive against [`Self::window_id`] instead, exactly as long as that window
/// instance is in the layout. Evicting on absence is what made the node graph unreachable by any
/// pointer in nearly every frame (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`).
#[derive(Clone, Debug)]
pub struct EngineSurfaceRegistration {
    pub host_id: String,
    pub surface_id: String,
    pub engine_token: Option<EngineSurfaceToken>,
    /// 🪟️ The window instance whose body this surface is painted into — the shell's one retention
    /// authority for it.
    pub window_id: String,
    pub bounds: Rect,
    pub controller_id: String,
    pub detail: EngineSurfaceKindDetail,
    /// 🆕️ This attach is the one that constructed the engine host — the shell installs the
    /// app-static catalogue on exactly these, instead of re-pushing ~100 kB every frame.
    pub created: bool,
}

struct AttachedSurfaceRegistry {
    pub(in super::super) slots: Box<[Option<EngineSurfaceRegistration>; ENGINE_SURFACE_CAPACITY]>,
    pub(in super::super) len: usize,
    pub(in super::super) faulted: bool,
}

impl Default for AttachedSurfaceRegistry {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; ENGINE_SURFACE_CAPACITY]), len: 0, faulted: false }
    }
}

impl AttachedSurfaceRegistry {
    pub(in super::super) fn upsert(&mut self, registration: EngineSurfaceRegistration) {
        if let Some(slot) = self.slots[..self.len].iter_mut().flatten().find(|slot| slot.host_id == registration.host_id) {
            slot.surface_id = registration.surface_id;
            slot.engine_token = registration.engine_token;
            slot.window_id = registration.window_id;
            slot.bounds = registration.bounds;
            slot.controller_id = registration.controller_id;
            slot.detail = registration.detail;
            slot.created |= registration.created;
            return;
        }
        if self.len == ENGINE_SURFACE_CAPACITY {
            self.faulted = true;
            return;
        }
        self.slots[self.len] = Some(registration);
        self.len += 1;
    }

    pub(in super::super) fn take(&mut self) -> Vec<EngineSurfaceRegistration> {
        let taken = self.slots[..self.len].iter_mut().filter_map(Option::take).collect();
        self.len = 0;
        taken
    }
}

pub(super) static ATTACHED_SURFACES: WorkerCell<AttachedSurfaceRegistry> = WorkerCell::new(Default::default);

/// 🧩️ Drains the surfaces attached since the last drain, for the shell to mirror into its bespoke
/// pointer state maps. Taking rather than reading keeps the drain a record of what this frame
/// actually painted; what SURVIVES a frame is the shell's own question, answered against each
/// entry's `window_id` — see [`EngineSurfaceRegistration`].
pub fn take_engine_surface_registrations() -> Vec<EngineSurfaceRegistration> {
    ATTACHED_SURFACES.with(|cell| cell.borrow_mut().take())
}

/// 🕸️ Which engine a scene selects — the wgpu twin of React's
/// `isFlowGraphScene(capabilitiesJson) || Boolean(snapshotJson)`.
///
/// @see `🧱️elements/🕸️NodeGraph/🟦️.tsx` — `NodeGraphHost`
/// @see `🧱️elements/🪪️WasmSessionLoader/🟦️.tsx` — `isFlowGraphScene`
fn node_graph_scene_uses_flow_engine(graph: &semio_ui_declaration::ui::NodeGraphScene) -> bool {
    if graph.host_snapshot_json.as_deref().is_some_and(|json| !json.trim().is_empty()) {
        return true;
    }
    let Some(capabilities) = graph.capabilities_json.as_deref() else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<Value>(capabilities) else {
        return false;
    };
    value.get("engine").and_then(Value::as_str) == Some("flow") || value.get("spotlight").and_then(Value::as_bool) == Some(true) || value.get("noteEdit").and_then(Value::as_bool) == Some(true)
}

pub(super) fn node_graph_engine_from_scene(graph: &semio_ui_declaration::ui::NodeGraphScene, dark: bool) -> NodeGraphEngine {
    if node_graph_scene_uses_flow_engine(graph) {
        let snapshot = graph.host_snapshot_json.as_deref().and_then(|json| FlowHost::parse_host_snapshot_json(json).ok()).unwrap_or_default();
        let mut host = FlowHost::from_host_snapshot(snapshot);
        host.set_canvas_theme_dark(dark);
        return NodeGraphEngine::Flow(host);
    }
    let mut host = GraphHost::default();
    host.set_canvas_theme_dark(dark);
    NodeGraphEngine::Dag(host)
}

fn node_graph_lod_from_json(engine: &mut NodeGraphEngine, lod_json: &str) {
    let Ok(value) = serde_json::from_str::<Value>(lod_json) else {
        return;
    };
    let automatic = value.get("automatic").and_then(Value::as_bool) != Some(false);
    let forced = value.get("forcedLabel").or_else(|| value.get("lod")).and_then(Value::as_str);
    match engine {
        NodeGraphEngine::Flow(host) => {
            host.style.set_automatic_lod(automatic);
            if let Some(label) = forced {
                host.style.set_forced_draw_lod_label(label);
            }
        }
        NodeGraphEngine::Dag(host) => {
            host.dag.viewport.set_automatic_lod(automatic);
            if let Some(label) = forced {
                host.dag.viewport.set_forced_draw_lod_label(label);
            }
        }
    }
}

type GraphHover<'a> = (&'a str, &'a str, &'a str);

fn node_graph_current_hover(dag: &flow::dag::DagHost) -> Option<GraphHover<'_>> {
    if let Some(edge) = dag.graph.hovered_edge_id() { return Some(("", "", edge)); }
    if let Some(channel) = dag.graph.engine.hover.and_then(|id| dag.graph.channel_view(id)) {
        return Some((channel.widget_id, channel.port, ""));
    }
    hovered_node_id_ref(&dag.viewport, &dag.graph).map(|node| (node, "", ""))
}

fn copy_node_graph_hover(dag: &flow::dag::DagHost) -> Option<(String, String, String)> {
    node_graph_current_hover(dag).map(|(node, port, edge)| (node.to_owned(), port.to_owned(), edge.to_owned()))
}

fn node_graph_hover_view(mark: &(String, String, String)) -> GraphHover<'_> {
    (&mark.0, &mark.1, &mark.2)
}

fn node_graph_scene_hover_update<'a>(
    previous: Option<GraphHover<'_>>,
    incoming: Option<GraphHover<'a>>,
    live: Option<GraphHover<'_>>,
    scene_owns_hover: bool,
) -> Option<Option<GraphHover<'a>>> {
    if previous == incoming { return None; }
    match incoming {
        Some(mark) if Some(mark) != live => Some(Some(mark).filter(|mark| *mark != ("", "", ""))),
        None if scene_owns_hover => Some(None),
        _ => None,
    }
}

fn node_graph_apply_hover(engine: &mut NodeGraphEngine, hover: Option<GraphHover<'_>>) {
    let (node, port, edge) = hover.unwrap_or_default();
    if !edge.is_empty() {
        match engine {
            NodeGraphEngine::Flow(host) => host.dag.graph.set_hover_edge(Some(edge)),
            NodeGraphEngine::Dag(host) => host.dag.graph.set_hover_edge(Some(edge)),
        }
        return;
    }
    let node_id = (!node.is_empty()).then_some(node);
    let port_id = (!port.is_empty()).then_some(port);
    match engine {
        NodeGraphEngine::Flow(host) => match port_id {
            Some(port) => host.set_hover_channel(node_id, Some(port)),
            None => host.set_hover(node_id),
        },
        NodeGraphEngine::Dag(host) => match port_id {
            Some(port) => set_hover_channel(&mut host.dag.viewport, &mut host.dag.graph, node_id, Some(port)),
            None => host.dag.graph.set_hover(node_id),
        },
    }
}

/// 🖼️ Whether this graph offers the engine something to frame — either measured content bounds or an
/// empty graph, which has nothing to wait for. An opening camera decided against neither is a
/// decision taken too early.
fn node_graph_frames_content(dag: &flow::dag::DagHost) -> bool {
    dag.graph.content_world_bounds().is_some() || dag.graph.host_snapshot.nodes.is_empty()
}

/// 🕸️ Feeds one `NodeGraphScene` into a live engine host, field by field, applying only what
/// actually changed since the last frame — the wgpu twin of React's `syncFlowSessionFromScene`
/// (structure, then selection/hover, then preview-off/lod/camera, then eval BEFORE computing chrome).
/// Returns whether anything was applied, so the caller can bump the surface's scene revision and let
/// the frame's engine packet rebuild.
///
/// @see `🧱️elements/🕸️NodeGraph/🟦️.tsx` — `syncFlowSessionStructureFromScene`/`syncFlowSessionEvalFromScene`
///
/// 🖼️ The opening camera is a DECISION, not a copy: a stored camera that does not frame this
/// graph is replaced by a fit. Every later viewport the plugin publishes is one this renderer
/// itself persisted from a gesture, so it is adopted verbatim. Twin of `dagStartupCamera` in
/// `🧱️elements/🕸️NodeGraph/🟦️.tsx`.
/// ⏳️ And it is a decision the engine can only take once it HAS the content to frame: the first
/// viewport reaches this sync before the graph's own nodes are measured, where `startup_camera`
/// has nothing to fit against and keeps the stored camera. Remembering the viewport then retires
/// the decision forever — on the live generation3d flow graph that pinned the opening camera and
/// left every node off screen. The decision is therefore retried until the content exists.
///
/// 🔀️ An example switch can move the whole graph out from under a live camera; only then is
/// the camera the viewer set re-fitted (`CONTENT_REFIT_MAX_COVERAGE`).
pub(super) fn sync_node_graph_engine(engine: &mut NodeGraphEngine, cache: &mut NodeGraphSyncCache, graph: &semio_ui_declaration::ui::NodeGraphScene) -> bool {
    let mut changed = false;
    let mut snapshot_changed = false;
    let mut original_hover = None;
    if cache.interaction_domain.as_deref() != graph.interaction_domain.as_ref() {
        cache.interaction_domain = graph.interaction_domain.clone().map(Arc::new);
        changed = true;
    }
    if let NodeGraphEngine::Flow(host) = engine {
        let operator_ids: Vec<String> = graph.operators.iter().map(|record| record.id.clone()).collect();
        if !operator_ids.is_empty() && cache.operator_ids.as_ref() != Some(&operator_ids) {
            original_hover = Some(copy_node_graph_hover(&host.dag));
            host.set_neuron_kind_infos(&graph.operators);
            cache.operator_ids = Some(operator_ids);
            changed = true;
        }
        if cache.snapshot_json.as_deref() != graph.host_snapshot_json.as_deref() {
            if let Some(snapshot) = graph.host_snapshot_json.as_deref().and_then(|json| FlowHost::parse_host_snapshot_json(json).ok()) {
                original_hover.get_or_insert_with(|| copy_node_graph_hover(&host.dag));
                host.resync_host_snapshot_from_scene(snapshot);
            }
            cache.snapshot_json = graph.host_snapshot_json.clone();
            snapshot_changed = true;
            changed = true;
        }
    }
    if let NodeGraphEngine::Dag(host) = engine {
        let payload = serde_json::to_vec(graph).unwrap_or_default();
        if cache.scene_pack.as_deref() != Some(payload.as_slice()) {
            if let Ok(json) = std::str::from_utf8(&payload) {
                original_hover = Some(copy_node_graph_hover(&host.dag));
                let _ = host.sync_from_scene_json(json);
            }
            cache.scene_pack = Some(payload);
            snapshot_changed = true;
            changed = true;
        }
    }
    if cache.selection.as_ref() != Some(&graph.selection) {
        match engine {
            NodeGraphEngine::Flow(host) => host.set_selection(&graph.selection),
            NodeGraphEngine::Dag(host) => host.dag.graph.set_selection(&graph.selection),
        }
        cache.selection = Some(graph.selection.clone());
        changed = true;
    }
    let incoming = graph.hover.as_ref().map(|hover| {
        if let Some(edge) = hover.edge_id.as_deref().filter(|id| !id.is_empty()) {
            ("", "", edge)
        } else if let Some(node) = hover.node_id.as_deref().filter(|id| !id.is_empty()) {
            (node, hover.port_id.as_deref().unwrap_or_default(), "")
        } else {
            ("", "", "")
        }
    });
    let previous = cache.hover.as_ref().map(node_graph_hover_view);
    let current = node_graph_current_hover(match &*engine {
        NodeGraphEngine::Flow(host) => &host.dag,
        NodeGraphEngine::Dag(host) => &host.dag,
    });
    let preserved = original_hover.as_ref().map(|mark| mark.as_ref().map(node_graph_hover_view));
    let update = node_graph_scene_hover_update(previous, incoming, preserved.unwrap_or(current), cache.scene_owns_hover);
    if previous != incoming {
        cache.scene_owns_hover = update.flatten().is_some()
            || (update.is_none() && incoming.is_some() && cache.scene_owns_hover);
        cache.hover = incoming.map(|(node, port, edge)| (node.to_owned(), port.to_owned(), edge.to_owned()));
    }
    if let Some(desired) = update.or(preserved).filter(|desired| *desired != current) {
        node_graph_apply_hover(engine, desired);
        changed = true;
    }
    if cache.preview_off_json.as_deref() != graph.preview_off_json.as_deref() {
        if let Some(json) = graph.preview_off_json.as_deref() {
            match engine {
                NodeGraphEngine::Flow(host) => host.set_preview_off_json(json),
                NodeGraphEngine::Dag(host) => {
                    if let Ok(ids) = serde_json::from_str::<Vec<String>>(json) {
                        set_dimmed(&mut host.dag.graph, &mut host.dag.paint, &ids);
                    }
                }
            }
        }
        cache.preview_off_json = graph.preview_off_json.clone();
        changed = true;
    }
    if cache.lod_json.as_deref() != graph.lod_json.as_deref() {
        if let Some(json) = graph.lod_json.as_deref() {
            node_graph_lod_from_json(engine, json);
        }
        cache.lod_json = graph.lod_json.clone();
        changed = true;
    }
    if cache.viewport.as_ref() != graph.viewport.as_ref() {
        let first = cache.viewport.is_none();
        let mut decided = true;
        if let Some(viewport) = graph.viewport.as_ref() {
            match engine {
                NodeGraphEngine::Flow(host) if first => {
                    host.adopt_camera_or_fit(viewport.x, viewport.y, viewport.zoom);
                    decided = node_graph_frames_content(&host.dag);
                }
                NodeGraphEngine::Flow(host) => board_set_camera(&mut host.scene, &mut host.outbox, viewport.x, viewport.y, viewport.zoom),
                NodeGraphEngine::Dag(host) if first => {
                    adopt_camera_or_fit(&mut host.dag.viewport, &mut host.dag.graph, viewport.x, viewport.y, viewport.zoom);
                    decided = node_graph_frames_content(&host.dag);
                }
                NodeGraphEngine::Dag(host) => host.dag.graph.set_camera(viewport.x, viewport.y, viewport.zoom),
            }
        }
        if decided {
            cache.viewport = graph.viewport.clone();
        }
        changed = true;
    } else if snapshot_changed {
        let refitted = match engine {
            NodeGraphEngine::Flow(host) => host.refit_camera_if_content_left_view(),
            NodeGraphEngine::Dag(host) => refit_camera_if_content_left_view(&mut host.dag.viewport, &mut host.dag.graph),
        };
        changed |= refitted;
    }
    changed | sync_node_graph_evaluation(engine, cache, graph)
}
//#endregion 🧩️EngineSurfaceAttach
