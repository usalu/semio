//! 🗃️ Sync caches and surface helpers: the map, board, raster and editor sync caches, the terminal checks, the surface table and the ensure, token and close helpers. See [`super`].

use super::*;

//#region Registry
#[derive(Default)]
pub struct MapSyncCache {
    pub map_descriptor_json: Option<String>,
    pub camera_json: Option<String>,
    pub render_mode: Option<String>,
    pub vector_style: Option<String>,
    pub lod_mode: Option<String>,
    pub layer_visibility_json: Option<String>,
    pub layer_stroke_scale_json: Option<String>,
    pub selection_json: Option<String>,
    pub hover_json: Option<String>,
    pub theme_json: Option<String>,
    pub size_key: Option<String>,
    /// 📡️ Tiles admitted into the bounded renderer-asset pipeline and not yet applied, keyed
    /// `(vector, z, x, y)` — the port of `MapRenderer`'s in-flight `Promise` set, and what keeps a
    /// per-frame re-offer from reserving the same URL twice.
    pub tile_pending: HashSet<(bool, u32, u32, u32)>,
    /// 🕳️ Tiles the server or the decoder refused — React's `tileMiss`/`vectorTileMiss`, cleared per
    /// lane whenever that lane's visible range revision changes.
    pub tile_misses: HashSet<(bool, u32, u32, u32)>,
    pub raster_tiles_revision: Option<u64>,
    pub vector_tiles_revision: Option<u64>,
}

#[derive(Default)]
pub struct BoardSyncCache {
    /// 🎯️ `Board2dScene.domain_id` — the framework interaction domain this board's hovers belong to.
    /// An app that declares none publishes NO `interactionHover`, exactly like React's
    /// `if (!interactionDomainId) return undefined` (`🖥️Board2dHost/🟦️.tsx:777`). Carried here
    /// because the pointer seams are addressed by surface id, not by scene.
    pub domain_id: Option<String>,
    /// 🎯️ Every entity id the snapshot carries, classified into the granularity a hover reports it
    /// under — the port of React's `board2dGranularityById` (`🖥️Board2dHost/🟦️.tsx:190`). Rebuilt
    /// only when the snapshot changes.
    pub granularity_by_id: HashMap<String, String>,
    pub snapshot_json: Option<String>,
    pub glyph_catalogs_json: Option<String>,
    /// 🏷️ `Board2dScene.placement_naming` — the owning domain's brush id prefixes, so a placement on this lane mints the domain's ids.
    pub placement_naming: Option<String>,
    pub placement_compatibility_json: Option<String>,
    pub selection_json: Option<String>,
    pub highlighted_ids_json: Option<String>,
    pub camera_json: Option<String>,
    pub hovered_id: Option<String>,
    pub active_utility: Option<String>,
    pub selection_method: Option<String>,
    pub selectable_kinds: Option<(bool, bool, bool)>,
    pub grid_visible: Option<bool>,
    pub grid_snap_enabled: Option<bool>,
    pub grid_factor: Option<f64>,
    pub suggestion_offset: Option<f64>,
    pub brush_weights_json: Option<String>,
    pub lod_mode: Option<String>,
    pub transform_flags: Option<String>,
    pub area_brush_size: Option<String>,
    pub size_key: Option<String>,
    pub tool_run_trace_window_id: Option<String>,
}

/** 🖌️ The last `Paint2dScene` fields pushed into this surface's [`RasterHost`] — the wgpu twin of
 * `Paint2dCanvasSurface`'s `documentSyncRef`/`assetsRef` guards plus the plain per-field equality its
 * `syncAll` effect re-runs on. Every entry is an owned `String`, so retirement drains it byte-wise.
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `syncAll` */
#[derive(Default)]
pub struct RasterSyncCache {
    pub document_sync_json: Option<String>,
    pub assets_json: Option<String>,
    pub camera_json: Option<String>,
    pub selection_json: Option<String>,
    pub hovered_id: Option<String>,
    pub active_utility: Option<String>,
    pub view_mode: Option<String>,
    pub brush: Option<(f64, f64, u32, f64)>,
    pub paint_target:Option<PaintTarget>,
    pub mask_value:Option<u8>,
    pub theme_json: Option<String>,
    pub size_key: Option<String>,
}

/** ✍️ The last `TextEditorScene` document pushed into this surface's [`EditorHost`], plus the
 * viewport/theme keys the host is sized and coloured by — the wgpu twin of `TextEditorHost`'s
 * `sceneRef`. */
#[derive(Default)]
pub struct EditorSyncCache {
    pub scene_json: Option<String>,
    pub theme_json: Option<String>,
    pub size_key: Option<String>,
}

pub(super) fn raster_sync_terminal(cache: &RasterSyncCache) -> bool {
    cache.document_sync_json.is_none()
        && cache.assets_json.is_none()
        && cache.camera_json.is_none()
        && cache.selection_json.is_none()
        && cache.hovered_id.is_none()
        && cache.active_utility.is_none()
        && cache.view_mode.is_none()
        && cache.brush.is_none()
        && cache.theme_json.is_none()
        && cache.size_key.is_none()
}

pub(super) fn editor_sync_terminal(cache: &EditorSyncCache) -> bool {
    cache.scene_json.is_none() && cache.theme_json.is_none() && cache.size_key.is_none()
}

pub(super) fn node_graph_sync_terminal(cache: &NodeGraphSyncCache) -> bool {
    cache.snapshot_json.is_none()
        && cache.selection.is_none()
        && cache.preview_off_json.is_none()
        && cache.computing_json.is_none()
        && cache.status_json.is_none()
        && cache.eval_json.is_none()
        && cache.lod_json.is_none()
        && cache.viewport.is_none()
        && cache.viewport_pixels.is_none()
        && cache.operator_ids.is_none()
        && cache.hover.is_none()
        && cache.scene_pack.is_none()
        && cache.interaction_domain.is_none()
}

pub(super) fn map_sync_terminal(cache: &MapSyncCache) -> bool {
    cache.map_descriptor_json.is_none()
        && cache.camera_json.is_none()
        && cache.render_mode.is_none()
        && cache.vector_style.is_none()
        && cache.lod_mode.is_none()
        && cache.layer_visibility_json.is_none()
        && cache.layer_stroke_scale_json.is_none()
        && cache.selection_json.is_none()
        && cache.hover_json.is_none()
        && cache.theme_json.is_none()
        && cache.size_key.is_none()
}

pub(super) fn board_sync_terminal(cache: &BoardSyncCache) -> bool {
    cache.snapshot_json.is_none()
        && cache.glyph_catalogs_json.is_none()
        && cache.placement_compatibility_json.is_none()
        && cache.selection_json.is_none()
        && cache.highlighted_ids_json.is_none()
        && cache.camera_json.is_none()
        && cache.hovered_id.is_none()
        && cache.active_utility.is_none()
        && cache.selection_method.is_none()
        && cache.selectable_kinds.is_none()
        && cache.grid_visible.is_none()
        && cache.grid_snap_enabled.is_none()
        && cache.grid_factor.is_none()
        && cache.suggestion_offset.is_none()
        && cache.brush_weights_json.is_none()
        && cache.lod_mode.is_none()
        && cache.transform_flags.is_none()
        && cache.area_brush_size.is_none()
        && cache.size_key.is_none()
        && cache.tool_run_trace_window_id.is_none()
}

pub static MAP_TILE_ASSET_FAULT: WorkerCell<Option<WorldAssetFault>> = WorkerCell::new(Default::default);

pub static ENGINE_SURFACES: WorkerCell<EngineSurfaceRegistry> = WorkerCell::new(Default::default);

pub fn empty_engine_surface(pw: u32, ph: u32) -> EngineSurface {
    EngineSurface {
        surface_id: None,
        node_graph: None,
        sync_cache: NodeGraphSyncCache::default(),
        map_host: None,
        map_sync_cache: MapSyncCache::default(),
        map_interaction_owner: None,
        board_host: None,
        board_sync_cache: BoardSyncCache::default(),
        board_pending_events: infinite_canvas::BoardEventQueue::default(),
        board_retiring_events: None,
        board_pointer_inside: false,
        board_pointer_claim: None,
        board_pointer_controller_id: None,
        board_hover_published: None,
        board_trace: tool_run_trace::ToolRunTraceLayer::default(),
        board_trace_shapes: Vec::new(),
        board_trace_palette: None,
        editor: None,
        editor_scene_pack: None,
        editor_sync_cache: EditorSyncCache::default(),
        editor_delivery: TextEditorDeliveryState::default(),
        raster_host: None,
        raster_sync_cache: RasterSyncCache::default(),
        width: pw.max(1),
        height: ph.max(1),
        metrics_generation: 1,
        document_generation: 0,
        scene_revision: 0,
        last_note_click: None,
        published_graph_interaction: PublishedGraphInteraction::default(),
        published_graph_hit: 0,
        published_graph_geometry: 0,
    }
}

/// 🧱️ Reserves — or resizes — the one fixed registry slot a live scene surface owns, returning the
/// generation-keyed snapshot a frame packet is admitted against. The single production entry into
/// [`ENGINE_SURFACES`]: every per-kind attach path (`sync_node_graph_scene` today) goes through it,
/// so a surface identity can never be published twice or outlive its slot generation.
pub fn ensure_engine_surface(host_id: &str, surface_id: &str, pw: u32, ph: u32) -> Option<EngineSurfaceSnapshot> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let surface_id = EngineSurfaceId::try_from_str(surface_id).ok()?;
        let needs_create = !map.contains_key(host_id);
        let needs_resize = map.get(host_id).is_some_and(|entry| entry.width != pw.max(1) || entry.height != ph.max(1));
        if needs_create {
            let Some(token) = map.reserve(host_id) else {
                return None;
            };
            let mut surface = empty_engine_surface(pw, ph);
            surface.surface_id = Some(surface_id);
            if map.publish_reserved(token, surface).is_err() {
                map.faulted = true;
                return None;
            }
        }
        if needs_resize {
            let Some(entry) = map.get_mut(host_id) else {
                map.faulted = true;
                return None;
            };
            let Some(metrics_generation) = entry.metrics_generation.checked_add(1) else {
                map.faulted = true;
                return None;
            };
            entry.width = pw.max(1);
            entry.height = ph.max(1);
            entry.metrics_generation = metrics_generation;
        }
        let entry = map.get_mut(host_id)?;
        entry.surface_id = Some(surface_id);
        let metrics_generation = entry.metrics_generation;
        let identity = map.identity(host_id)?;
        Some(EngineSurfaceSnapshot { identity, metrics_generation })
    })
}

pub fn engine_surface_wire_id(host_id: &str) -> Option<EngineSurfaceId> {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(host_id)?.surface_id)
}

pub fn engine_surface_token(host_id: &str) -> Option<EngineSurfaceToken> {
    ENGINE_SURFACES.with(|cell| cell.borrow().identity(host_id).map(|identity| identity.token))
}

pub(in super::super) fn engine_surface_live_freshness(token: EngineSurfaceToken) -> Result<Option<EngineSurfaceLiveFreshness>, ()> {
    ENGINE_SURFACES.with(|cell| {
        let registry = cell.try_borrow_mut().ok_or(())?;
        let Some(slot) = registry.slots.get(usize::from(token.slot)) else {
            return Ok(None);
        };
        if slot.generation != token.generation {
            return Ok(None);
        }
        let (Some(id), Some(value)) = (slot.id, slot.value.as_ref()) else {
            return Ok(None);
        };
        Ok(Some(EngineSurfaceLiveFreshness { identity: EngineSurfaceIdentity { token, id }, metrics_generation: value.metrics_generation, document_generation: value.document_generation, scene_revision: value.scene_revision }))
    })
}

pub fn engine_surface_token_at(index: usize) -> Result<Option<EngineSurfaceToken>, ()> {
    ENGINE_SURFACES.with(|cell| {
        let registry = cell.try_borrow_mut().ok_or(())?;
        let Some(slot) = registry.slots.get(index) else {
            return Ok(None);
        };
        Ok((slot.id.is_some() || slot.value.is_some() || slot.retirement.is_some()).then_some(EngineSurfaceToken { slot: index as u16, generation: slot.generation }))
    })
}

pub fn begin_engine_surface_close_token(token: EngineSurfaceToken) -> Result<bool, ()> {
    let (begun, identity) = ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.try_borrow_mut().ok_or(())?;
        let identity = registry.identity_for_token(token);
        Ok((registry.begin_close(token), identity))
    })?;
    if begun {
        let Some(identity) = identity else { return Err(()) };
        STAGED_ENGINE_SCENES.with(|cell| {
            let mut staged = cell.try_borrow_mut().ok_or(())?;
            staged.remove_exact(identity);
            Ok(())
        })?;
    }
    Ok(begun)
}

pub fn close_engine_surface_step(token: EngineSurfaceToken, context: &mut semio_framework_job::StepContext<'_>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    ENGINE_SURFACES.with(|cell| cell.try_borrow_mut().is_some_and(|mut registry| registry.close_step(token, context, input)))
}

pub fn engine_surface_terminal_nonopaque_is_empty(token: EngineSurfaceToken) -> Result<bool, ()> {
    ENGINE_SURFACES.with(|cell| cell.try_borrow_mut().map(|registry| registry.terminal_nonopaque_is_empty(token)).ok_or(()))
}
//#endregion Registry
