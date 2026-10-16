//! 🧩️ Engine surface attach and per-kind scene sync.
//!
//! Attaching a retained scene to an engine surface and projecting its per-kind state: surface registrations,
//! the node graph, tiled map, board, raster and text editor scene syncs and the raster composite progress.

use semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_granularity_by_id;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{BoardSyncCache, ENGINE_SURFACES, ENGINE_SURFACE_CAPACITY, EngineSurface, EngineSurfaceId, EngineSurfaceSnapshot, EngineSurfaceToken, MapSyncCache, NodeGraphEngine, NodeGraphSyncCache, RasterSyncCache, engine_camera_from_json, engine_canvas_debug_log, engine_canvas_fault_log, ensure_engine_surface};
use semio_framework_os_renderer_wgpu_engine_graph::node_graph::log_graph_geometry_census;
use semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::{StagedEngineScene, STAGED_ENGINE_SCENES};
use semio_framework_os_renderer_wgpu_engine_text::text_editor::{synchronize_text_editor_host, text_editor_explicit_draft_settings};
use semio_framework_os_renderer_wgpu_engine_board::tiled_map::reserve_map_tile_fetches;
use crate::element_contracts::WorkerCell;
use canvas::Color;
use flow::FlowHost;
use framework_editor::EditorHost;
use infinite_world::world::tool_run_trace;
use semio_framework_canvas as canvas;
use semio_framework_os_node_graph::GraphHost;
use semio_framework_surface::paint::{PaintTarget, RasterHost};
use semio_framework_surface::tiled_map::MapHost;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use ui_wgpu::wgpu::{Rect, Rgba, SurfaceKind, Theme, UiComponentSceneNode};
use infinite_canvas::{board_set_camera, board_set_grid_visible};

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
    pub(super) slots: Box<[Option<EngineSurfaceRegistration>; ENGINE_SURFACE_CAPACITY]>,
    pub(super) len: usize,
    pub(super) faulted: bool,
}

impl Default for AttachedSurfaceRegistry {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; ENGINE_SURFACE_CAPACITY]), len: 0, faulted: false }
    }
}

impl AttachedSurfaceRegistry {
    pub(super) fn upsert(&mut self, registration: EngineSurfaceRegistration) {
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

    pub(super) fn take(&mut self) -> Vec<EngineSurfaceRegistration> {
        let taken = self.slots[..self.len].iter_mut().filter_map(Option::take).collect();
        self.len = 0;
        taken
    }
}

static ATTACHED_SURFACES: WorkerCell<AttachedSurfaceRegistry> = WorkerCell::new(Default::default);

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
fn node_graph_scene_uses_flow_engine(graph: &ui_wgpu::wgpu::NodeGraphScene) -> bool {
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

fn node_graph_engine_from_scene(graph: &ui_wgpu::wgpu::NodeGraphScene, dark: bool) -> NodeGraphEngine {
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

fn node_graph_apply_hover(engine: &mut NodeGraphEngine, node_id: Option<&str>, port_id: Option<&str>) {
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
fn sync_node_graph_engine(engine: &mut NodeGraphEngine, cache: &mut NodeGraphSyncCache, graph: &ui_wgpu::wgpu::NodeGraphScene) -> bool {
    let mut changed = false;
    let mut snapshot_changed = false;
    if cache.interaction_domain.as_deref() != graph.interaction_domain.as_ref() {
        cache.interaction_domain = graph.interaction_domain.clone().map(Arc::new);
        changed = true;
    }
    if let NodeGraphEngine::Flow(host) = engine {
        let operator_ids: Vec<String> = graph.operators.iter().map(|record| record.id.clone()).collect();
        if !operator_ids.is_empty() && cache.operator_ids.as_ref() != Some(&operator_ids) {
            host.set_neuron_kind_infos(&graph.operators);
            cache.operator_ids = Some(operator_ids);
            changed = true;
        }
        if cache.snapshot_json.as_deref() != graph.host_snapshot_json.as_deref() {
            if let Some(snapshot) = graph.host_snapshot_json.as_deref().and_then(|json| FlowHost::parse_host_snapshot_json(json).ok()) {
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
    let hover = graph.hover.as_ref().map(|hover| (hover.node_id.clone().unwrap_or_default(), hover.port_id.clone().unwrap_or_default()));
    if cache.hover != hover {
        let node_id = hover.as_ref().map(|(node, _)| node.as_str()).filter(|node| !node.is_empty());
        let port_id = hover.as_ref().map(|(_, port)| port.as_str()).filter(|port| !port.is_empty());
        node_graph_apply_hover(engine, node_id, port_id);
        cache.hover = hover;
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

/// 🧮️ The evaluation half of the scene sync, kept in its own pass for the ordering React documents:
/// `applyEvalOutputsJson` clears computing chrome, so eval is applied BEFORE `statusJson`/
/// `computingJson`, never after.
fn sync_node_graph_evaluation(engine: &mut NodeGraphEngine, cache: &mut NodeGraphSyncCache, graph: &ui_wgpu::wgpu::NodeGraphScene) -> bool {
    let mut changed = false;
    if cache.eval_json.as_deref() != graph.eval_json.as_deref() {
        if let (NodeGraphEngine::Flow(host), Some(json)) = (&mut *engine, graph.eval_json.as_deref()) {
            host.apply_eval_outputs_json(json);
        }
        cache.eval_json = graph.eval_json.clone();
        changed = true;
    }
    if cache.status_json.as_deref() != graph.status_json.as_deref() {
        if let Some(json) = graph.status_json.as_deref() {
            let mut observe=|progress:semio_framework_value::NativeDecodeProgress|progress.total<=64*1024;
            let mut control=semio_framework_value::NativeDecodeControl::new(256*1024,&mut observe);
            if let Ok(statuses)=infinite_canvas::board::io::text::dag_input::decode_dag_node_statuses_json(json,&mut control){
                match engine {
                    NodeGraphEngine::Flow(host)=>host.set_node_statuses(&statuses),
                    NodeGraphEngine::Dag(host)=>set_node_statuses(&mut host.dag.graph, &mut host.dag.paint, &statuses),
                }
            }
        }
        cache.status_json = graph.status_json.clone();
        changed = true;
    }
    if cache.computing_json.as_deref() != graph.computing_json.as_deref() {
        if let (None, Some(json)) = (graph.status_json.as_deref(), graph.computing_json.as_deref()) {
            let value = serde_json::from_str::<Value>(json).unwrap_or(Value::Null);
            let active = value.get("active").and_then(Value::as_str);
            let stale: Vec<String> = value.get("stale").and_then(Value::as_array).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_owned)).collect()).unwrap_or_default();
            match engine {
                NodeGraphEngine::Flow(host) => host.set_computing_progress(active, &stale),
                NodeGraphEngine::Dag(host) => set_computing_progress(&mut host.dag.graph, &mut host.dag.paint, active, &stale),
            }
        }
        cache.computing_json = graph.computing_json.clone();
        changed = true;
    }
    changed
}

/// 🕸️ Attaches — and drives — the node-graph engine host behind one `SurfaceKind::NodeGraph` scene.
/// Called from the retained scene paint (`scenes::render_component_scene_step`), so a window's first
/// painted frame after `SurfaceVisible` is the frame the host is constructed on, and every later frame
/// re-feeds only the scene fields that moved. Returns whether a live host now backs the surface.
pub fn sync_node_graph_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, panel: Rgba) -> bool {
    let dark = 0.2126 * panel.r + 0.7152 * panel.g + 0.0722 * panel.b < 0.5;
    let Some(graph) = scene.node_graph.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let created = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.host_id) else {
            return None;
        };
        let created = entry.node_graph.is_none();
        if created {
            entry.node_graph = Some(node_graph_engine_from_scene(graph, dark));
        }
        let Some(engine) = entry.node_graph.as_mut() else {
            return None;
        };
        let cache = &mut entry.sync_cache;
        let mut changed = created;
        if cache.viewport_pixels != Some((width, height)) {
            match engine {
                NodeGraphEngine::Flow(host) => host.set_viewport(width, height, 1.0),
                NodeGraphEngine::Dag(host) => host.set_viewport(width, height, 1.0),
            }
            cache.viewport_pixels = Some((width, height));
            changed = true;
        }
        if sync_node_graph_engine(engine, cache, graph) || changed {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        Some(created)
    });
    let Some(created) = created else {
        return false;
    };
    log_graph_geometry_census(&scene.host_id);
    register_engine_surface(scene, window_id, bounds, EngineSurfaceKindDetail::NodeGraph, created, Some(snapshot.identity.token));
    true
}

/// 🧩️ Records one attached surface for the shell's per-frame mirror. Every kind goes through it —
/// including `World3d`, whose host lives in the shell's own `world3d_states` — so the shell has ONE
/// drained witness of what the chrome walk actually painted this frame.
pub fn register_engine_surface(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, detail: EngineSurfaceKindDetail, created: bool, engine_token: Option<EngineSurfaceToken>) {
    ATTACHED_SURFACES.with(|cell| {
        cell.borrow_mut().upsert(EngineSurfaceRegistration { host_id: scene.host_id.clone(), surface_id: scene.surface_id.clone(), engine_token, window_id: window_id.to_string(), bounds, controller_id: scene.controller_id.clone(), detail, created })
    });
}

/// 🎨️ The map palette this shell theme implies, in the rgba8 document `MapHost::set_map_theme_from_json`
/// reads — the wgpu twin of React's `serializeMapCanvasThemeJson`. Fields with no unambiguous shell
/// counterpart are left out, so `MapPalette`'s own defaults stand for them exactly as they do in React.
fn map_theme_json(theme: &Theme) -> String {
    let rgba8 = |color: Rgba| json!([(color.r * 255.0).round() as u16, (color.g * 255.0).round() as u16, (color.b * 255.0).round() as u16, (color.a * 255.0).round() as u16]);
    json!({
        "surfaceClear": rgba8(theme.canvas_clear),
        "labelFill": rgba8(theme.text),
        "labelHalo": rgba8(theme.panel),
        "selectionStroke": rgba8(theme.accent),
        "hoverStroke": rgba8(theme.text_element),
    })
    .to_string()
}

/// 🗺️ Feeds one `TiledMapScene` into a live `MapHost`, field by field, applying only what changed —
/// the wgpu twin of React's `TiledMapHost` effect ladder (descriptor, then render mode/vector style/
/// lod, then layer documents, then interaction, then camera).
///
/// @see `🧱️elements/🧭️TiledMapHost/🟦️.tsx`
fn sync_map_engine(host: &mut MapHost, cache: &mut MapSyncCache, map: &ui_wgpu::wgpu::TiledMapScene, theme: &Theme, width: u32, height: u32) -> bool {
    let mut changed = false;
    let size_key = format!("{width}x{height}");
    if cache.size_key.as_deref() != Some(size_key.as_str()) {
        host.scene.set_size(width, height, 1.0);
        cache.size_key = Some(size_key);
        changed = true;
    }
    let theme_json = map_theme_json(theme);
    if cache.theme_json.as_deref() != Some(theme_json.as_str()) {
        let _ = host.set_map_theme_from_json(&theme_json);
        cache.theme_json = Some(theme_json);
        changed = true;
    }
    if cache.map_descriptor_json.as_deref() != Some(map.map_descriptor_json.as_str()) {
        let _ = host.sync_map_json(&map.map_descriptor_json);
        cache.map_descriptor_json = Some(map.map_descriptor_json.clone());
        changed = true;
    }
    if cache.render_mode.as_deref() != Some(map.render_mode.as_str()) {
        host.set_render_mode(&map.render_mode);
        cache.render_mode = Some(map.render_mode.clone());
        changed = true;
    }
    if cache.vector_style.as_deref() != Some(map.vector_style.as_str()) {
        host.set_vector_style(&map.vector_style);
        cache.vector_style = Some(map.vector_style.clone());
        changed = true;
    }
    if cache.lod_mode.as_deref() != Some(map.lod_mode.as_str()) {
        host.set_lod_mode(&map.lod_mode);
        cache.lod_mode = Some(map.lod_mode.clone());
        changed = true;
    }
    if cache.layer_visibility_json.as_deref() != Some(map.layer_visibility_json.as_str()) {
        let _ = host.set_layer_visibility_from_json(&map.layer_visibility_json);
        cache.layer_visibility_json = Some(map.layer_visibility_json.clone());
        changed = true;
    }
    if cache.layer_stroke_scale_json.as_deref() != Some(map.layer_stroke_scale_json.as_str()) {
        let _ = host.set_layer_stroke_scale_from_json(&map.layer_stroke_scale_json);
        cache.layer_stroke_scale_json = Some(map.layer_stroke_scale_json.clone());
        changed = true;
    }
    if cache.selection_json.as_deref() != Some(map.selection_json.as_str()) || cache.hover_json.as_deref() != Some(map.hover_json.as_str()) {
        host.sync_interaction_json(&map.selection_json, &map.hover_json);
        cache.selection_json = Some(map.selection_json.clone());
        cache.hover_json = Some(map.hover_json.clone());
        changed = true;
    }
    if cache.camera_json.as_deref() != Some(map.camera_json.as_str()) {
        match engine_camera_from_json(&map.camera_json) {
            Some((x, y, zoom)) => board_set_camera(&mut host.scene, &mut host.outbox, x, y, zoom),
            None => host.fit_world_camera(),
        }
        cache.camera_json = Some(map.camera_json.clone());
        changed = true;
    }
    changed
}

/// 🗺️ Attaches — and drives — the `MapHost` behind one `SurfaceKind::TiledMap` scene, on the same
/// production seam `sync_node_graph_scene` uses.
pub fn sync_tiled_map_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, theme: &Theme) -> bool {
    let Some(map) = scene.tiled_map.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let created = ENGINE_SURFACES.with(|cell| {
        let mut map_registry = cell.borrow_mut();
        let entry = map_registry.get_mut(&scene.host_id)?;
        let created = entry.map_host.is_none();
        if created {
            entry.map_host = Some(MapHost::new());
        }
        let host = entry.map_host.as_mut()?;
        if sync_map_engine(host, &mut entry.map_sync_cache, map, theme, width, height) || created {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        let (host, cache) = (entry.map_host.as_ref()?, &mut entry.map_sync_cache);
        reserve_map_tile_fetches(&scene.host_id, host, cache, map);
        Some(created)
    });
    let Some(created) = created else {
        return false;
    };
    register_engine_surface(scene, window_id, bounds, EngineSurfaceKindDetail::TiledMap { selection_method: map.selection_method.clone() }, created, Some(snapshot.identity.token));
    true
}

/// 🎲️ Feeds one `Board2dScene` into a live `BoardHost`, field by field, applying only what changed.
/// `load_board_snapshot_json` resets selection AND camera to the snapshot's own defaults, so a snapshot pass
/// re-applies both silently right after — the rule React's `applySnapshotToSession` states explicitly.
///
/// @see `🧱️elements/🖥️Board2dHost/🟦️.tsx` — `applySnapshotToSession`
///
/// 🎯️ Argument order on the normal port is `(nodes, edges, handles)`; the owning app's own
/// selectable-kind filter decides which granularity a pick may reach at all.
pub fn sync_board_engine(host: &mut infinite_canvas::BoardHost, cache: &mut BoardSyncCache, board: &ui_wgpu::wgpu::Board2dScene, width: u32, height: u32) -> bool {
    let mut changed = false;
    let size_key = format!("{width}x{height}");
    if cache.size_key.as_deref() != Some(size_key.as_str()) {
        host.scene.set_size(width, height, 1.0);
        cache.size_key = Some(size_key);
        changed = true;
    }
    let snapshot_applied = cache.snapshot_json.as_deref() != Some(board.snapshot_json.as_str());
    if snapshot_applied {
        {let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);infinite_canvas::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut host.scene, &mut host.focus, &mut host.style, &mut host.outbox,&board.snapshot_json,&mut control)};
        cache.granularity_by_id = board2d_granularity_by_id(&board.snapshot_json);
        cache.snapshot_json = Some(board.snapshot_json.clone());
        changed = true;
    }
    if cache.domain_id != board.domain_id {
        cache.domain_id = board.domain_id.clone();
        changed = true;
    }
    if cache.glyph_catalogs_json.as_deref() != Some(board.glyph_catalogs_json.as_str()) {
        let _ = host.style.set_board_kind_catalogs_from_json(&board.glyph_catalogs_json);
        cache.glyph_catalogs_json = Some(board.glyph_catalogs_json.clone());
        changed = true;
    }
    if cache.placement_naming != board.placement_naming {
        let _ = host.style.set_placement_naming_json(board.placement_naming.as_deref());
        cache.placement_naming = board.placement_naming.clone();
    }
    if cache.placement_compatibility_json.as_deref() != Some(board.placement_compatibility_json.as_str()) {
        let _ = host.style.set_handle_link_compat_from_json(&board.placement_compatibility_json);
        cache.placement_compatibility_json = Some(board.placement_compatibility_json.clone());
        changed = true;
    }
    let selectable_kinds = (board.selectable_nodes, board.selectable_edges, board.selectable_handles);
    if snapshot_applied || cache.selection_method.as_deref() != Some(board.selection_method.as_str()) || cache.selectable_kinds != Some(selectable_kinds) {
        host.focus.set_selection_options(&board.selection_method, "replace", selectable_kinds.0, selectable_kinds.1, selectable_kinds.2);
        cache.selection_method = Some(board.selection_method.clone());
        cache.selectable_kinds = Some(selectable_kinds);
        changed = true;
    }
    if cache.grid_visible != Some(board.grid_visible) {
        board_set_grid_visible(&mut host.scene, &mut host.style, board.grid_visible);
        cache.grid_visible = Some(board.grid_visible);
        changed = true;
    }
    if snapshot_applied || cache.selection_json.as_deref() != Some(board.selection_json.as_str()) {
        let ids = serde_json::from_str::<Vec<String>>(&board.selection_json).unwrap_or_default();
        set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &ids);
        cache.selection_json = Some(board.selection_json.clone());
        changed = true;
    }
    if snapshot_applied || cache.highlighted_ids_json.as_deref() != Some(board.highlighted_ids_json.as_str()) {
        host.focus.set_highlighted_ids(serde_json::from_str::<Vec<String>>(&board.highlighted_ids_json).unwrap_or_default());
        cache.highlighted_ids_json = Some(board.highlighted_ids_json.clone());
        changed = true;
    }
    if snapshot_applied || cache.camera_json.as_deref() != Some(board.camera_json.as_str()) {
        if let Some((x, y, zoom)) = engine_camera_from_json(&board.camera_json) {
            set_camera_silent(&mut host.scene, &mut host.outbox, x, y, zoom);
        }
        cache.camera_json = Some(board.camera_json.clone());
        changed = true;
    }
    if cache.hovered_id.as_deref() != board.hovered_id.as_deref() {
        host.focus.set_hovered_id_silent(board.hovered_id.clone());
        cache.hovered_id = board.hovered_id.clone();
        changed = true;
    }
    let active_utility = board.active_utility.clone().unwrap_or_else(|| "select".into());
    if cache.active_utility.as_deref() != Some(active_utility.as_str()) {
        host.set_active_utility(&active_utility);
        cache.active_utility = Some(active_utility);
        changed = true;
    }
    if cache.grid_snap_enabled != Some(board.grid_snap_enabled) {
        host.style.set_grid_snap_enabled(board.grid_snap_enabled);
        cache.grid_snap_enabled = Some(board.grid_snap_enabled);
        changed = true;
    }
    if cache.grid_factor != Some(board.grid_factor) {
        let _ = host.style.set_grid_factor(board.grid_factor);
        cache.grid_factor = Some(board.grid_factor);
        changed = true;
    }
    if board.suggestion_offset > 0.0 && cache.suggestion_offset != Some(board.suggestion_offset) {
        host.set_suggestion_offset(board.suggestion_offset);
        cache.suggestion_offset = Some(board.suggestion_offset);
        changed = true;
    }
    if cache.brush_weights_json.as_deref() != Some(board.brush_weights_json.as_str()) {
        host.set_brush_kind_weights(&board.brush_weights_json);
        cache.brush_weights_json = Some(board.brush_weights_json.clone());
        changed = true;
    }
    if cache.lod_mode.as_deref() != Some(board.lod_mode.as_str()) {
        if board.lod_mode == "automatic" {
            host.style.set_automatic_lod(true);
        } else {
            host.style.set_automatic_lod(false);
            host.style.set_forced_draw_lod_label(&board.lod_mode);
        }
        cache.lod_mode = Some(board.lod_mode.clone());
        changed = true;
    }
    let transform_flags = board.transform_flags.clone().unwrap_or_else(|| BOARD2D_DEFAULT_TRANSFORM_FLAGS_JSON.to_string());
    if cache.transform_flags.as_deref() != Some(transform_flags.as_str()) {
        let (move_enabled, rotate_enabled) = board2d_transform_flags_from_json(&transform_flags);
        set_transform_flags(&mut host.scene, &mut host.gestures, move_enabled, rotate_enabled);
        cache.transform_flags = Some(transform_flags);
        changed = true;
    }
    let area_brush_size = board.area_brush_size.clone().unwrap_or_else(|| BOARD2D_DEFAULT_AREA_BRUSH_SIZE_JSON.to_string());
    if cache.area_brush_size.as_deref() != Some(area_brush_size.as_str()) {
        let (width, height) = board2d_area_brush_size_from_json(&area_brush_size);
        host.gestures.set_area_brush_extent(width, height);
        cache.area_brush_size = Some(area_brush_size);
        changed = true;
    }
    changed
}

/// 🖍️ What a board that never declares `areaBrushSize` paints on a click — a non-positive axis is
/// refused by the engine itself, so the sentinel simply leaves the host's own default standing.
const BOARD2D_DEFAULT_AREA_BRUSH_SIZE_JSON: &str = "{\"width\":0,\"height\":0}";

/// 🖍️ Reads `{"width":f64,"height":f64}`; a malformed payload answers `(0, 0)`, which the engine
/// reads as "keep the extent you have" rather than collapsing the brush.
fn board2d_area_brush_size_from_json(json: &str) -> (f64, f64) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return (0.0, 0.0);
    };
    (value.get("width").and_then(serde_json::Value::as_f64).unwrap_or(0.0), value.get("height").and_then(serde_json::Value::as_f64).unwrap_or(0.0))
}

/// 🕹️ Both gumball handles on — what a board that never declares `transformFlags` composes.
const BOARD2D_DEFAULT_TRANSFORM_FLAGS_JSON: &str = "{\"move\":true,\"rotate\":true}";

/// 🕹️ Reads `{"move":bool,"rotate":bool}`; a malformed payload leaves both handles on rather than
/// silently disarming the gumball.
fn board2d_transform_flags_from_json(json: &str) -> (bool, bool) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return (true, true);
    };
    (value.get("move").and_then(serde_json::Value::as_bool).unwrap_or(true), value.get("rotate").and_then(serde_json::Value::as_bool).unwrap_or(true))
}

/// ⏯️ Feeds a board's `toolRunTrace` lane into its resident trace layer, refreshes the placement footprints when the kind
/// catalogs change and the verdict palette from `theme`, and remembers the window its cursor is echoed for. A malformed lane
/// keeps the previous records.
fn sync_board_tool_run_trace(entry: &mut EngineSurface, board: &ui_wgpu::wgpu::Board2dScene, window_id: &str, theme: &Theme, catalogs_changed: bool) -> bool {
    let applied = entry.board_trace.apply_lane(board.tool_run_trace.as_deref()).is_ok_and(|applied| applied.pages > 0 || applied.cleared);
    if catalogs_changed || entry.board_trace_shapes.is_empty() {
        entry.board_trace_shapes = tool_run_trace::board2d_tool_run_trace_shapes(&board.glyph_catalogs_json);
    }
    entry.board_trace_palette = Some(tool_run_trace::ToolRunTracePalette::from_theme(theme));
    if entry.board_sync_cache.tool_run_trace_window_id.as_deref() != Some(window_id) {
        entry.board_sync_cache.tool_run_trace_window_id = Some(window_id.to_string());
    }
    applied
}

/// 🖌️ Paints the board's placement2d trace over its vector scene: one filled footprint per visible record at the host's
/// live camera, the newest `testing` record outlined. Mirrors React's `ToolRunTrace2dLayer`.
fn append_board_tool_run_trace(painted: &mut canvas::Scene, host: &infinite_canvas::BoardHost, entry: &EngineSurface) {
    let Some(palette) = entry.board_trace_palette.as_ref() else { return };
    if entry.board_trace.is_empty() {
        return;
    }
    let origin = host.scene.world_to_screen(canvas::Point::new(0.0, 0.0));
    let unit = host.scene.world_to_screen(canvas::Point::new(1.0, 0.0));
    let zoom = (unit.x - origin.x).hypot(unit.y - origin.y);
    let highlight = palette.testing.with_alpha(1.0);
    for draw in entry.board_trace.placement_draws(palette, tool_run_trace::ToolRunTraceVisibility::default()) {
        let Some(shape) = entry.board_trace_shapes.get(draw.shape as usize) else { continue };
        let center = host.scene.world_to_screen(canvas::Point::new(f64::from(draw.position[0]), f64::from(draw.position[1])));
        let (sin, cos) = f64::from(draw.rotation).sin_cos();
        let transform = canvas::Affine::new([zoom * cos, zoom * sin, -zoom * sin, zoom * cos, center.x, center.y]);
        let color = canvas::Color::new([draw.color.r, draw.color.g, draw.color.b, draw.color.a]);
        let outline = draw.newest.then(|| (canvas::Stroke::new(ui_styling::metrics::tool_run::TESTING_OUTLINE_WIDTH / zoom.max(f64::EPSILON)), canvas::Color::new([highlight.r, highlight.g, highlight.b, highlight.a])));
        match *shape {
            tool_run_trace::ToolRunTraceShape2d::Circle { radius } => {
                let circle = canvas::Circle::new(canvas::Point::new(0.0, 0.0), radius);
                painted.fill(canvas::FillRule::NonZero, transform, color, None, &circle);
                if let Some((stroke, stroke_color)) = outline.as_ref() {
                    painted.stroke(stroke, transform, *stroke_color, None, &circle);
                }
            }
            tool_run_trace::ToolRunTraceShape2d::Rectangle { width, height } => {
                let rect = canvas::Rect::new(-width * 0.5, -height * 0.5, width * 0.5, height * 0.5);
                painted.fill(canvas::FillRule::NonZero, transform, color, None, &rect);
                if let Some((stroke, stroke_color)) = outline.as_ref() {
                    painted.stroke(stroke, transform, *stroke_color, None, &rect);
                }
            }
        }
    }
}

/// 🧭️ The `toolRunTraceCursor` every board surface echoes, keyed by the window it paints into — merged into the live view
/// state next to `world3d_tool_run_trace_cursors`.
pub fn board2d_tool_run_trace_cursors() -> HashMap<String, semio_framework_tool_run::ToolRunTraceCursor> {
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().values_mut().filter(|entry| entry.board_host.is_some()).filter_map(|entry| Some((entry.board_sync_cache.tool_run_trace_window_id.clone()?, entry.board_trace.cursor()?))).collect())
}

/// 🔍️ Resident trace records and footprints of one board surface, for the attach law.
#[cfg(any(test, feature = "testkit"))]
pub fn board2d_tool_run_trace_state(surface_id: &str) -> Option<(usize, usize, usize)> {
    ENGINE_SURFACES.with(|cell| {
        let registry = cell.borrow_mut();
        let entry = registry.get(surface_id)?;
        let palette = entry.board_trace_palette?;
        Some((entry.board_trace.len(), entry.board_trace_shapes.len(), entry.board_trace.placement_draws(&palette, tool_run_trace::ToolRunTraceVisibility::default()).len()))
    })
}

/// 🎲️ Attaches — and drives — the `BoardHost` behind one `SurfaceKind::Board2d` scene.
pub fn sync_board2d_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, theme: &Theme) -> bool {
    let Some(board) = scene.board2d.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let created = ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let entry = registry.get_mut(&scene.host_id)?;
        let created = entry.board_host.is_none();
        if created {
            entry.board_host = Some(ManuallyDrop::new(infinite_canvas::BoardHost::default()));
        }
        let catalogs_changed = entry.board_sync_cache.glyph_catalogs_json.as_deref() != Some(board.glyph_catalogs_json.as_str());
        let host = entry.board_host.as_mut()?;
        let engine_changed = sync_board_engine(host, &mut entry.board_sync_cache, board, width, height);
        let trace_changed = sync_board_tool_run_trace(entry, board, window_id, theme, catalogs_changed);
        if engine_changed || trace_changed || created {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        Some(created)
    });
    let Some(created) = created else {
        return false;
    };
    register_engine_surface(scene, window_id, bounds, EngineSurfaceKindDetail::Board2d { snapshot_json: board.snapshot_json.clone() }, created, Some(snapshot.identity.token));
    true
}

/** 🎨️ The canvas-theme document both `RasterHost::set_canvas_theme_from_json` and
 * `EditorHost::set_canvas_theme_from_json` read — the wgpu twin of `syncSessionCanvasTheme`
 * (`@semio-tech/ui-styling`), which reads the same CSS custom properties React's hosts do.
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `syncSessionCanvasTheme(session)` */
fn engine_canvas_theme_json(theme: &Theme) -> String {
    let rgba8 = |color: Rgba| json!([(color.r * 255.0).round() as u16, (color.g * 255.0).round() as u16, (color.b * 255.0).round() as u16, (color.a * 255.0).round() as u16]);
    json!({
        "rasterClear": rgba8(theme.canvas_clear),
        "gridMinorStroke": rgba8(theme.separator),
        "labelFill": rgba8(theme.text),
        "labelFillHovered": rgba8(theme.text_element),
        "labelHalo": rgba8(theme.panel),
        "nodeFillHovered": rgba8(theme.text_element),
        "nodeFillSelected": rgba8(theme.accent),
    })
    .to_string()
}

/** 🖌️ Feeds one `Paint2dScene` into a live [`RasterHost`], field by field, applying only what
 * changed — the wgpu twin of `Paint2dCanvasSurface`'s `syncAll` effect: document, then assets, then
 * utility/brush, then interaction, then the camera (fitted for the navigator view mode, taken from
 * the scene otherwise).
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `syncAll` */
fn sync_raster_engine(host: &mut RasterHost, cache: &mut RasterSyncCache, paint: &ui_wgpu::wgpu::Paint2dScene, theme: &Theme, width: u32, height: u32) -> bool {
    let mut changed = false;
    let size_key = format!("{width}x{height}");
    if cache.size_key.as_deref() != Some(size_key.as_str()) {
        host.scene.set_size(width, height, 1.0);
        cache.size_key = Some(size_key);
        changed = true;
    }
    let theme_json = engine_canvas_theme_json(theme);
    if cache.theme_json.as_deref() != Some(theme_json.as_str()) {
        let _ = host.set_canvas_theme_from_json(&theme_json);
        cache.theme_json = Some(theme_json);
        changed = true;
    }
    if cache.document_sync_json.as_deref() != Some(paint.document_sync_json.as_str()) {
        let _ = host.sync_document_json(&paint.document_sync_json);
        cache.document_sync_json = Some(paint.document_sync_json.clone());
        changed = true;
    }
    if cache.assets_json.as_deref() != Some(paint.assets_json.as_str()) {
        for (key, bytes) in paint2d_asset_bytes(&paint.assets_json) {
            let _ = host.upload_raster_image_key(&key, &bytes);
        }
        cache.assets_json = Some(paint.assets_json.clone());
        changed = true;
    }
    if cache.active_utility.as_deref() != Some(paint.active_utility.as_str()) {
        host.set_active_utility(&paint.active_utility);
        cache.active_utility = Some(paint.active_utility.clone());
        changed = true;
    }
    if let Ok(color) = u32::from_str_radix(paint.brush_color.trim_start_matches('#'), 16) {
        let brush = (paint.brush_size, paint.brush_opacity, color, paint.brush_hardness);
        if cache.brush != Some(brush) {
            host.set_brush_size(paint.brush_size as f32);
            host.set_brush_opacity(paint.brush_opacity as f32);
            host.set_brush_color([(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]);
            host.set_brush_hardness(paint.brush_hardness as f32);
            cache.brush = Some(brush);
            changed = true;
        }
    }
    if let Some(target)=match paint.paint_target.as_str(){"pixels"=>Some(PaintTarget::Pixels),"mask"=>Some(PaintTarget::Mask),_=>None}{
        if cache.paint_target!=Some(target){host.set_paint_target(target);cache.paint_target=Some(target);changed=true;}
    }
    if let Ok(value)=u8::try_from(paint.mask_value){if cache.mask_value!=Some(value){host.set_mask_value(value);cache.mask_value=Some(value);changed=true;}}
    if cache.selection_json.as_deref() != Some(paint.selection_json.as_str()) || cache.hovered_id.as_deref() != paint.hovered_id.as_deref() {
        let selected: Vec<String> = serde_json::from_str(&paint.selection_json).unwrap_or_default();
        host.sync_interaction(&selected, paint.hovered_id.as_deref());
        cache.selection_json = Some(paint.selection_json.clone());
        cache.hovered_id = paint.hovered_id.clone();
        changed = true;
    }
    let navigator = paint.view_mode == "navigator";
    if cache.view_mode.as_deref() != Some(paint.view_mode.as_str()) {
        cache.view_mode = Some(paint.view_mode.clone());
        changed = true;
    }
    let camera_json = if navigator { host.navigator_fit_camera_json(f64::from(width), f64::from(height)) } else { paint.camera_json.clone() };
    if cache.camera_json.as_deref() != Some(camera_json.as_str()) {
        if let Some((x, y, zoom)) = engine_camera_from_json(&camera_json) {
            board_set_camera(&mut host.scene, &mut host.outbox, x, y, zoom);
        }
        cache.camera_json = Some(camera_json);
        changed = true;
    }
    changed
}

/** 🖼️ `Paint2dScene.assets_json`'s `{ key: { mime, data } }` rows decoded to raw bytes — the wgpu
 * twin of `base64ToBytes(asset.data)` in `syncAll`'s asset loop. */
fn paint2d_asset_bytes(assets_json: &str) -> Vec<(String, Vec<u8>)> {
    let Ok(Value::Object(rows)) = serde_json::from_str::<Value>(assets_json) else {
        return Vec::new();
    };
    rows.into_iter()
        .filter_map(|(key, row)| {
            let data = row.get("data").and_then(Value::as_str)?;
            let payload = data.rsplit_once("base64,").map_or(data, |(_, tail)| tail);
            semio_framework_io_base64::base64_standard_decode(payload).ok().map(|bytes| (key, bytes))
        })
        .collect()
}

/// 🖌️ Attaches — and re-feeds — the `RasterHost` behind one `SurfaceKind::Paint2d` scene.
pub fn sync_paint2d_scene(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme) -> bool {
    let Some(paint) = scene.paint_2d.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(_snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else {
            return false;
        };
        let created = entry.raster_host.is_none();
        if created {
            entry.raster_host = Some(RasterHost::new());
        }
        let Some(host) = entry.raster_host.as_mut() else {
            return false;
        };
        if sync_raster_engine(host, &mut entry.raster_sync_cache, paint, theme, width, height) || created {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        true
    })
}

/** ✍️ Attaches — and re-feeds — the `EditorHost` behind one `SurfaceKind::TextEditor` scene. The
 * host is the SAME one `text_editor_pointer_button_into`/`text_editor_apply_key_into` mutate, so a
 * keystroke and the glyphs it produces are one object, not a render copy of a model. */
pub fn sync_text_editor_scene(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let Some(_snapshot) = ensure_engine_surface(&scene.host_id, &scene.surface_id, width, height) else { return false };
    let Ok(scene_json) = serde_json::to_string(editor) else {
        return false;
    };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else {
            return false;
        };
        let created = entry.editor.is_none();
        if created {
            entry.editor = Some(EditorHost::new());
        }
        let Some(host) = entry.editor.as_mut() else {
            return false;
        };
        let mut changed = created;
        let size_key = format!("{width}x{height}");
        if entry.editor_sync_cache.size_key.as_deref() != Some(size_key.as_str()) {
            host.scene.set_size(width, height, 1.0);
            entry.editor_sync_cache.size_key = Some(size_key);
            changed = true;
        }
        let theme_json = engine_canvas_theme_json(theme);
        if entry.editor_sync_cache.theme_json.as_deref() != Some(theme_json.as_str()) {
            let _ = host.set_canvas_theme_from_json(&theme_json);
            entry.editor_sync_cache.theme_json = Some(theme_json);
            changed = true;
        }
        if entry.editor_sync_cache.scene_json.as_deref() != Some(scene_json.as_str()) {
            let explicit = text_editor_explicit_draft_settings(scene).is_some();
            let external = if explicit && entry.editor_delivery.explicit_dirty {
                if editor.buffer == host.text() {
                    entry.editor_delivery.explicit_dirty = false;
                    entry.editor_delivery.explicit_base = Some(editor.buffer.clone());
                    entry.editor_delivery.explicit_changes = None;
                    entry.editor_delivery.explicit_conflict = false;
                    entry.editor_delivery.explicit_past.clear();
                    entry.editor_delivery.explicit_future.clear();
                    entry.editor_delivery.explicit_snapshot = Some(editor.buffer.clone());
                    entry.editor_delivery.explicit_publication = None;
                    entry.editor_delivery.explicit_active = None;
                    entry.editor_delivery.explicit_error = None;
                    true
                } else {
                    if entry.editor_delivery.explicit_base.as_deref() != Some(editor.buffer.as_str()) {
                        entry.editor_delivery.explicit_conflict = true;
                    }
                    false
                }
            } else if explicit {
                entry.editor_delivery.explicit_base = Some(editor.buffer.clone());
                entry.editor_delivery.explicit_changes = None;
                entry.editor_delivery.explicit_conflict = false;
                entry.editor_delivery.explicit_snapshot = Some(editor.buffer.clone());
                entry.editor_delivery.explicit_error = None;
                true
            } else {
                entry.editor_delivery.reconcile_buffer(&editor.buffer)
            };
            if let Err(fault) = synchronize_text_editor_host(host, editor, external) {
                engine_canvas_fault_log(&format!("[text-editor] surface {} refused its scene — {fault}", scene.host_id));
            }
            entry.editor_sync_cache.scene_json = Some(scene_json);
            changed = true;
        }
        if changed {
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
        }
        true
    })
}

/// 🧩️ THE production attach entry: one call, every `SurfaceKind` whose engine host the wgpu shell
/// composites through a vello texture. Called from the retained scene paint
/// (`scenes::render_component_scene_step`), so a window's first painted frame after `SurfaceVisible`
/// is the frame its host is constructed on, and every later frame re-feeds only what moved.
/// `SurfaceKind::World3d` is NOT here: its host is `World3dState`, which lives in the shell's own
/// `world3d_states` (the pick/asset/authority ladders address it there) and paints a real 3-D pass
/// into the window draw list instead of a vello raster — `scenes` attaches it against that map and
/// records it through [`register_engine_surface`] like every other kind.
pub fn sync_engine_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, theme: &Theme) -> bool {
    match scene.component_kind {
        SurfaceKind::NodeGraph => sync_node_graph_scene(scene, window_id, bounds, theme.panel),
        SurfaceKind::TiledMap => sync_tiled_map_scene(scene, window_id, bounds, theme),
        SurfaceKind::Board2d => sync_board2d_scene(scene, window_id, bounds, theme),
        SurfaceKind::Paint2d => sync_paint2d_scene(scene, bounds, theme),
        SurfaceKind::TextEditor => sync_text_editor_scene(scene, bounds, theme),
        _ => false,
    }
}

/// 🎨️ Paints the attached host through its OWN vector renderer and stages the result as this frame's
/// engine packet — the one paint entry for every vello-composited surface kind.
pub fn stage_engine_scene_paint(scene: &UiComponentSceneNode, bounds: Rect, clear: Rgba) -> bool {
    let width = bounds.w.max(1.0) as u32;
    let height = bounds.h.max(1.0) as u32;
    let staged = ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let identity = registry.identity(&scene.host_id)?;
        let entry = registry.get_mut(&scene.host_id)?;
        let painted = match scene.component_kind {
            SurfaceKind::NodeGraph => match entry.node_graph.as_ref()? {
                NodeGraphEngine::Flow(host) => {
                    let mut painted = canvas::Scene::new();
                    host.paint_scene(&mut painted, width, height, 1.0);
                    painted
                }
                NodeGraphEngine::Dag(host) => {
                    let mut painted = canvas::Scene::new();
                    host.paint_scene(&mut painted, width, height, 1.0);
                    painted
                }
            },
            SurfaceKind::TiledMap => {
                let host = entry.map_host.as_mut()?;
                host.prepare_visible_tiles();
                host.build_vector_scene()
            }
            SurfaceKind::Board2d => {
                let host = entry.board_host.as_ref()?;
                let mut painted = host.build_vector_scene();
                append_board_tool_run_trace(&mut painted, host, entry);
                painted
            }
            SurfaceKind::Paint2d => entry.raster_host.as_mut()?.build_current_scene(),
            SurfaceKind::TextEditor => entry.editor.as_ref()?.build_scene(),
            _ => return None,
        };
        Some(StagedEngineScene {
            surface: EngineSurfaceSnapshot { identity, metrics_generation: entry.metrics_generation },
            document_generation: entry.document_generation,
            scene_revision: entry.scene_revision,
            scene: painted,
            clear: Color::new([clear.r, clear.g, clear.b, clear.a]),
            width,
            height,
        })
    });
    let Some(staged) = staged else {
        return false;
    };
    STAGED_ENGINE_SCENES.with(|cell| cell.borrow_mut().upsert(staged));
    true
}

#[derive(Clone, Copy)]
pub(crate) struct RasterCompositeProgress {
    pub(crate) completed: u64,
    pub(crate) done: bool,
}

/// 🧩️ Advances raster preparation within one frame-build grant, retaining the previous presented image.
pub fn advance_raster_composite(scene: &UiComponentSceneNode) -> RasterCompositeProgress {
    if scene.component_kind != SurfaceKind::Paint2d {
        return RasterCompositeProgress { completed: 0, done: true };
    }
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(host) = registry.get_mut(&scene.host_id).and_then(|entry| entry.raster_host.as_mut()) else {
            return RasterCompositeProgress { completed: 0, done: true };
        };
        if let Some(paint) = &scene.paint_2d {
            let selection: Vec<String> = serde_json::from_str(&paint.selection_json).unwrap_or_default();
            host.set_composite_view(&paint.view_mode, selection.first().map(String::as_str));
        }
        match host.advance_composite(65536) {
            Ok(progress) => RasterCompositeProgress { completed: progress.completed as u64, done: progress.done },
            Err(error) => {
                engine_canvas_debug_log(&format!("Raster composite failed: {error}"));
                RasterCompositeProgress { completed: 0, done: true }
            }
        }
    })
}

/// 🔑️ The raster key an engine surface's composited texture is published under — the same key
/// `EngineGpuCandidate` reserves through `GpuContext::reserve_engine_texture`, so a retained
/// `push_raster_quad` under it composites the vello-rendered graph into the window's draw list.
pub fn engine_raster_key(surface_id: &str) -> Option<String> {
    EngineSurfaceId::try_from_str(surface_id).ok().map(|id| id.with_raster_key(str::to_owned))
}
//#endregion 🧩️EngineSurfaceAttach
