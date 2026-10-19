//! 🧯️ Surface retirement ladder: the close phases, the node graph engine retirement and the engine surface retirement with its close steps. See [`super`].

use super::*;

//#region Registry
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EngineSurfaceClosePhase {
    Claim,
    Events,
    NodeGraph,
    NodeGraphSync,
    Map,
    MapSync,
    Editor,
    EditorPack,
    EditorSync,
    EditorDelivery,
    Raster,
    RasterSync,
    Board,
    BoardSync,
    Scalars,
    Witness,
    Released,
}

enum NodeGraphEngineRetirement {
    Dag(semio_framework_os_node_graph::GraphHostRetirement),
    Flow(flow::FlowHostRetirement),
}

impl NodeGraphEngineRetirement {
    pub(in super::super) fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        match self {
            Self::Dag(owner) => owner.close_step(context),
            Self::Flow(owner) => owner.close_step(context),
        }
    }

    pub(in super::super) fn terminal_nonopaque_is_empty(&self) -> bool {
        match self {
            Self::Dag(owner) => owner.terminal_nonopaque_is_empty(),
            Self::Flow(owner) => owner.terminal_nonopaque_is_empty(),
        }
    }
}

pub(super) struct EngineSurfaceRetirement {
    pub(in super::super) surface_id: Option<EngineSurfaceId>,
    node_graph_source: Option<NodeGraphEngine>,
    pub(in super::super) sync_cache: NodeGraphSyncCache,
    map_source: Option<MapHost>,
    pub(in super::super) map_sync_cache: MapSyncCache,
    board_source: Option<ManuallyDrop<infinite_canvas::BoardHost>>,
    pub(in super::super) board_sync_cache: BoardSyncCache,
    pub(in super::super) board_pending_events: infinite_canvas::BoardEventQueue,
    pub(in super::super) board_retiring_events: Option<infinite_canvas::BoardEventQueue>,
    pub(in super::super) board_pointer_claim: Option<semio_framework_ui_action::BoundedActionClaim>,
    pub(in super::super) board_pointer_controller_id: Option<String>,
    editor_source: Option<EditorHost>,
    pub(in super::super) editor_scene_pack: Option<Vec<u8>>,
    pub(in super::super) editor_sync_cache: EditorSyncCache,
    pub(in super::super) editor_delivery: TextEditorDeliveryState,
    raster_source: Option<RasterHost>,
    pub(in super::super) raster_sync_cache: RasterSyncCache,
    pub(super) last_note_click: Option<(String, f64)>,
    pub(in super::super) node_graph: Option<NodeGraphEngineRetirement>,
    pub(in super::super) map: Option<semio_framework_surface::tiled_map::MapHostRetirement>,
    pub(in super::super) editor: Option<framework_editor::EditorHostRetirement>,
    raster: Option<semio_framework_surface::paint::RasterHostRetirement>,
    pub(in super::super) board: Option<infinite_canvas::BoardHostRetirement>,
    pub(in super::super) phase: EngineSurfaceClosePhase,
    pub(in super::super) faulted: bool,
}

impl EngineSurfaceRetirement {
    pub(in super::super) fn new(surface: EngineSurface, host_id: Option<EngineSurfaceId>) -> Self {
        let EngineSurface {
            surface_id: _,
            node_graph: node_graph_source,
            sync_cache,
            map_host: map_source,
            map_sync_cache,
            map_interaction_owner: _,
            board_host: board_source,
            board_sync_cache,
            board_pending_events,
            board_retiring_events,
            board_pointer_inside: _,
            board_pointer_claim,
            board_pointer_controller_id,
            board_hover_published: _,
            board_trace: _,
            board_trace_shapes: _,
            board_trace_palette: _,
            editor: editor_source,
            editor_scene_pack,
            editor_sync_cache,
            editor_delivery,
            raster_host: raster_source,
            raster_sync_cache,
            width: _,
            height: _,
            metrics_generation: _,
            document_generation: _,
            scene_revision: _,
            last_note_click,
            published_graph_interaction: _,
            published_graph_hit: _,
            published_graph_geometry: _,
        } = surface;
        Self {
            surface_id: host_id,
            node_graph_source,
            sync_cache,
            map_source,
            map_sync_cache,
            board_source,
            board_sync_cache,
            board_pending_events,
            board_retiring_events,
            board_pointer_claim,
            board_pointer_controller_id,
            editor_source,
            editor_scene_pack,
            editor_sync_cache,
            editor_delivery,
            raster_source,
            raster_sync_cache,
            last_note_click,
            node_graph: None,
            map: None,
            editor: None,
            raster: None,
            board: None,
            phase: EngineSurfaceClosePhase::Claim,
            faulted: false,
        }
    }

    pub(in super::super) fn close_string(value: &mut Option<String>) -> bool {
        let Some(text) = value.as_mut() else {
            return false;
        };
        if !retire_string_page(text) {
            *value = None;
        }
        true
    }

    fn close_bytes(value: &mut Option<Vec<u8>>) -> bool {
        let Some(bytes) = value.as_mut() else {
            return false;
        };
        if !retire_bytes_page(bytes) {
            *value = None;
        }
        true
    }

    fn close_node_graph_interaction_domain(value: &mut Option<Arc<semio_framework_ui_declaration::ui::NodeGraphInteractionDomain>>) -> bool {
        let Some(shared) = value.as_mut() else { return false };
        let Some(domain) = Arc::get_mut(shared) else { return true };
        if retire_string_page(&mut domain.id) || retire_string_page(&mut domain.node_target_prefix) || retire_string_page(&mut domain.edge_target_prefix) || retire_string_page(&mut domain.handle_target_prefix) {
            return true;
        }
        *value = None;
        true
    }

    fn close_node_graph_sync(cache: &mut NodeGraphSyncCache) -> bool {
        if Self::close_string(&mut cache.snapshot_json)
            || cache.selection.as_mut().is_some_and(|ids| ids.last_mut().is_some_and(retire_string_page))
            || cache.selection.as_mut().is_some_and(|ids| ids.pop().is_some())
            || Self::close_string(&mut cache.preview_off_json)
            || Self::close_string(&mut cache.computing_json)
            || Self::close_string(&mut cache.status_json)
            || Self::close_string(&mut cache.eval_json)
            || Self::close_string(&mut cache.lod_json)
            || cache.viewport.take().is_some()
            || cache.viewport_pixels.take().is_some()
            || cache.hover.take().is_some()
            || cache.operator_ids.as_mut().is_some_and(|ids| ids.last_mut().is_some_and(retire_string_page))
            || cache.operator_ids.as_mut().is_some_and(|ids| ids.pop().is_some())
            || Self::close_bytes(&mut cache.scene_pack)
            || Self::close_node_graph_interaction_domain(&mut cache.interaction_domain)
        {
            return false;
        }
        cache.selection = None;
        cache.operator_ids = None;
        true
    }

    fn close_map_sync(cache: &mut MapSyncCache, host_id: Option<&EngineSurfaceId>) -> bool {
        if let Some(host_id) = host_id {
            crate::asset_authority::cancel_renderer_map_tile_assets(host_id.as_str());
            cache.tile_pending.clear();
            cache.tile_misses.clear();
            cache.raster_tiles_revision = None;
            cache.vector_tiles_revision = None;
        }
        if Self::close_string(&mut cache.map_descriptor_json)
            || Self::close_string(&mut cache.camera_json)
            || Self::close_string(&mut cache.render_mode)
            || Self::close_string(&mut cache.vector_style)
            || Self::close_string(&mut cache.lod_mode)
            || Self::close_string(&mut cache.layer_visibility_json)
            || Self::close_string(&mut cache.layer_stroke_scale_json)
            || Self::close_string(&mut cache.selection_json)
            || Self::close_string(&mut cache.hover_json)
            || Self::close_string(&mut cache.theme_json)
            || Self::close_string(&mut cache.size_key)
        {
            return false;
        }
        true
    }

    fn close_editor_sync(cache: &mut EditorSyncCache) -> bool {
        !(Self::close_string(&mut cache.scene_json) || Self::close_string(&mut cache.theme_json) || Self::close_string(&mut cache.size_key))
    }

    fn close_raster_sync(cache: &mut RasterSyncCache) -> bool {
        if Self::close_string(&mut cache.document_sync_json)
            || Self::close_string(&mut cache.assets_json)
            || Self::close_string(&mut cache.camera_json)
            || Self::close_string(&mut cache.selection_json)
            || Self::close_string(&mut cache.hovered_id)
            || Self::close_string(&mut cache.active_utility)
            || Self::close_string(&mut cache.view_mode)
            || cache.brush.take().is_some()
            || Self::close_string(&mut cache.theme_json)
            || Self::close_string(&mut cache.size_key)
        {
            return false;
        }
        true
    }

    fn close_board_sync(cache: &mut BoardSyncCache) -> bool {
        if Self::close_string(&mut cache.snapshot_json)
            || Self::close_string(&mut cache.glyph_catalogs_json)
            || Self::close_string(&mut cache.placement_compatibility_json)
            || Self::close_string(&mut cache.selection_json)
            || Self::close_string(&mut cache.highlighted_ids_json)
            || Self::close_string(&mut cache.camera_json)
            || Self::close_string(&mut cache.hovered_id)
            || Self::close_string(&mut cache.active_utility)
            || Self::close_string(&mut cache.selection_method)
            || cache.selectable_kinds.take().is_some()
            || cache.grid_visible.take().is_some()
            || cache.grid_snap_enabled.take().is_some()
            || cache.grid_factor.take().is_some()
            || cache.suggestion_offset.take().is_some()
            || Self::close_string(&mut cache.brush_weights_json)
            || Self::close_string(&mut cache.lod_mode)
            || Self::close_string(&mut cache.transform_flags)
            || Self::close_string(&mut cache.area_brush_size)
            || Self::close_string(&mut cache.size_key)
            || Self::close_string(&mut cache.tool_run_trace_window_id)
        {
            return false;
        }
        true
    }

    pub(in super::super) fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
        if context.should_yield() || self.faulted {
            return false;
        }
        match self.phase {
            EngineSurfaceClosePhase::Claim => {
                if let Some(claim) = self.board_pointer_claim.take() {
                    if input.release_action_claim(claim).is_err() {
                        self.faulted = true;
                        return false;
                    }
                } else if Self::close_string(&mut self.board_pointer_controller_id) {
                } else {
                    self.phase = EngineSurfaceClosePhase::Events;
                }
            }
            EngineSurfaceClosePhase::Events => {
                if let Some(events) = self.board_retiring_events.as_mut() {
                    if events.close_step() {
                        self.board_retiring_events = None;
                    }
                } else if self.board_pending_events.close_step() {
                    self.phase = EngineSurfaceClosePhase::NodeGraph;
                }
            }
            EngineSurfaceClosePhase::NodeGraph => {
                if self.node_graph.is_none() {
                    if let Some(owner) = self.node_graph_source.take() {
                        self.node_graph = Some(match owner {
                            NodeGraphEngine::Dag(owner) => NodeGraphEngineRetirement::Dag(semio_framework_os_node_graph::GraphHostRetirement::new(owner)),
                            NodeGraphEngine::Flow(owner) => NodeGraphEngineRetirement::Flow(flow::FlowHostRetirement::new(owner)),
                        });
                    } else {
                        self.phase = EngineSurfaceClosePhase::NodeGraphSync;
                    }
                    context.consume_fuel(1);
                } else if self.node_graph.as_mut().is_some_and(|owner| owner.close_step(context)) {
                    if !self.node_graph.as_ref().is_some_and(NodeGraphEngineRetirement::terminal_nonopaque_is_empty) {
                        self.faulted = true;
                        return false;
                    }
                    self.node_graph = None;
                    self.phase = EngineSurfaceClosePhase::NodeGraphSync;
                }
                return false;
            }
            EngineSurfaceClosePhase::NodeGraphSync => {
                if Self::close_node_graph_sync(&mut self.sync_cache) {
                    self.phase = EngineSurfaceClosePhase::Map;
                }
            }
            EngineSurfaceClosePhase::Map => {
                if self.map.is_none() {
                    if let Some(owner) = self.map_source.take() {
                        self.map = Some(semio_framework_surface::tiled_map::MapHostRetirement::new(owner));
                    } else {
                        self.phase = EngineSurfaceClosePhase::MapSync;
                    }
                } else if self.map.as_mut().is_some_and(semio_framework_surface::tiled_map::MapHostRetirement::close_step) {
                    if !self.map.as_ref().is_some_and(semio_framework_surface::tiled_map::MapHostRetirement::terminal_is_empty) {
                        self.faulted = true;
                        return false;
                    }
                    self.map = None;
                    self.phase = EngineSurfaceClosePhase::MapSync;
                }
            }
            EngineSurfaceClosePhase::MapSync => {
                if Self::close_map_sync(&mut self.map_sync_cache, self.surface_id.as_ref()) {
                    self.phase = EngineSurfaceClosePhase::Editor;
                }
            }
            EngineSurfaceClosePhase::Editor => {
                if self.editor.is_none() {
                    if let Some(owner) = self.editor_source.take() {
                        self.editor = Some(framework_editor::EditorHostRetirement::new(owner));
                    } else {
                        self.phase = EngineSurfaceClosePhase::EditorPack;
                    }
                } else if self.editor.as_mut().is_some_and(framework_editor::EditorHostRetirement::close_step) {
                    if !self.editor.as_ref().is_some_and(framework_editor::EditorHostRetirement::terminal_is_empty) {
                        self.faulted = true;
                        return false;
                    }
                    self.editor = None;
                    self.phase = EngineSurfaceClosePhase::EditorPack;
                }
            }
            EngineSurfaceClosePhase::EditorPack => {
                if !Self::close_bytes(&mut self.editor_scene_pack) {
                    self.phase = EngineSurfaceClosePhase::EditorSync;
                }
            }
            EngineSurfaceClosePhase::EditorSync => {
                if Self::close_editor_sync(&mut self.editor_sync_cache) {
                    self.phase = EngineSurfaceClosePhase::EditorDelivery;
                }
            }
            EngineSurfaceClosePhase::EditorDelivery => {
                if self.editor_delivery.close_step() {
                    self.phase = EngineSurfaceClosePhase::Raster;
                }
            }
            EngineSurfaceClosePhase::Raster => {
                if self.raster.is_none() {
                    if let Some(owner) = self.raster_source.take() {
                        self.raster = Some(semio_framework_surface::paint::RasterHostRetirement::new(owner));
                    } else {
                        self.phase = EngineSurfaceClosePhase::RasterSync;
                    }
                } else if self.raster.as_mut().is_some_and(semio_framework_surface::paint::RasterHostRetirement::close_step) {
                    if !self.raster.as_ref().is_some_and(semio_framework_surface::paint::RasterHostRetirement::terminal_is_empty) {
                        self.faulted = true;
                        return false;
                    }
                    self.raster = None;
                    self.phase = EngineSurfaceClosePhase::RasterSync;
                }
            }
            EngineSurfaceClosePhase::RasterSync => {
                if Self::close_raster_sync(&mut self.raster_sync_cache) {
                    self.phase = EngineSurfaceClosePhase::Board;
                }
            }
            EngineSurfaceClosePhase::Board => {
                if self.board.is_none() {
                    if let Some(host) = self.board_source.take() {
                        self.board = Some(infinite_canvas::BoardHostRetirement::new(ManuallyDrop::into_inner(host)));
                        context.consume_fuel(1);
                    } else {
                        self.phase = EngineSurfaceClosePhase::BoardSync;
                        context.consume_fuel(1);
                    }
                } else if self.board.as_mut().is_some_and(|owner| owner.close_step(context)) {
                    if !self.board.as_ref().is_some_and(infinite_canvas::BoardHostRetirement::terminal_nonopaque_is_empty) {
                        self.faulted = true;
                        return false;
                    }
                    self.board = None;
                    self.phase = EngineSurfaceClosePhase::BoardSync;
                }
                return false;
            }
            EngineSurfaceClosePhase::BoardSync => {
                if Self::close_board_sync(&mut self.board_sync_cache) {
                    self.phase = EngineSurfaceClosePhase::Scalars;
                }
            }
            EngineSurfaceClosePhase::Scalars => {
                if self.last_note_click.as_mut().is_some_and(|(id, _)| retire_string_page(id)) {
                } else if self.last_note_click.take().is_none() {
                    self.phase = EngineSurfaceClosePhase::Witness;
                }
            }
            EngineSurfaceClosePhase::Witness => {
                if self.node_graph_source.is_some()
                    || self.map_source.is_some()
                    || self.editor_source.is_some()
                    || self.board_source.is_some()
                    || self.editor_scene_pack.is_some()
                    || !editor_sync_terminal(&self.editor_sync_cache)
                    || !self.editor_delivery.terminal_is_empty()
                    || self.raster_source.is_some()
                    || self.raster.is_some()
                    || !raster_sync_terminal(&self.raster_sync_cache)
                    || self.board_pointer_claim.is_some()
                    || self.board_pointer_controller_id.is_some()
                    || self.board_retiring_events.is_some()
                    || !self.board_pending_events.terminal_is_empty()
                    || !node_graph_sync_terminal(&self.sync_cache)
                    || !map_sync_terminal(&self.map_sync_cache)
                    || !board_sync_terminal(&self.board_sync_cache)
                    || self.last_note_click.is_some()
                    || self.node_graph.is_some()
                    || self.map.is_some()
                    || self.editor.is_some()
                    || self.board.is_some()
                {
                    self.faulted = true;
                    return false;
                }
                self.phase = EngineSurfaceClosePhase::Released;
            }
            EngineSurfaceClosePhase::Released => return true,
        }
        context.consume_fuel(1);
        self.phase == EngineSurfaceClosePhase::Released
    }

    pub(in super::super) fn terminal_nonopaque_is_empty(&self) -> bool {
        self.phase == EngineSurfaceClosePhase::Released
            && self.node_graph_source.is_none()
            && node_graph_sync_terminal(&self.sync_cache)
            && self.map_source.is_none()
            && map_sync_terminal(&self.map_sync_cache)
            && self.board_source.is_none()
            && board_sync_terminal(&self.board_sync_cache)
            && self.board_pending_events.terminal_is_empty()
            && self.board_retiring_events.is_none()
            && self.board_pointer_claim.is_none()
            && self.board_pointer_controller_id.is_none()
            && self.editor_source.is_none()
            && self.editor_scene_pack.is_none()
            && editor_sync_terminal(&self.editor_sync_cache)
            && self.editor_delivery.terminal_is_empty()
            && self.raster_source.is_none()
            && self.raster.is_none()
            && raster_sync_terminal(&self.raster_sync_cache)
            && self.last_note_click.is_none()
            && self.node_graph.is_none()
            && self.map.is_none()
            && self.editor.is_none()
            && self.board.is_none()
            && !self.faulted
    }
}

impl Drop for EngineSurfaceRetirement {
    fn drop(&mut self) {
        debug_assert!(self.terminal_nonopaque_is_empty() || std::thread::panicking(), "EngineSurfaceRetirement must reach terminal-empty before release");
    }
}
//#endregion Registry
