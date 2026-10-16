//! 🗂️ Engine surface registry, sync caches and retirement.
//!
//! The per-worker engine surface registry: node graph engines, the surface slots and their ids, tokens and live
//! freshness, the four per-kind sync caches, the terminal checks and the retirement ladder that closes a surface.

use super::prepared_engine_canvas::STAGED_ENGINE_SCENES;
use crate::element_contracts::{ScenePointerTarget, WorkerCell};
use flow::FlowHost;
use framework_editor::EditorHost;
#[cfg(target_arch = "wasm32")]
use js_sys;
use infinite_world::world::{tool_run_trace, WorldAssetFault};
use serde_json::{json, Value};
use semio_framework_os_node_graph::GraphHost;
use semio_framework_surface::paint::{PaintTarget, RasterHost};
use semio_framework_surface::tiled_map::MapHost;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use ui_wgpu::wgpu::ActionDescriptor;
use vello::wgpu;

//#region Registry
pub enum NodeGraphEngine {
    Dag(GraphHost),
    Flow(FlowHost),
}

#[derive(Default)]
pub struct NodeGraphSyncCache {
    pub snapshot_json: Option<String>,
    pub selection: Option<Vec<String>>,
    pub preview_off_json: Option<String>,
    pub computing_json: Option<String>,
    pub status_json: Option<String>,
    pub eval_json: Option<String>,
    pub lod_json: Option<String>,
    pub viewport: Option<semio_framework_os_kernel::Viewport2d>,
    pub viewport_pixels: Option<(u32, u32)>,
    pub operator_ids: Option<Vec<String>>,
    pub hover: Option<(String, String)>,
    pub scene_pack: Option<Vec<u8>>,
    pub interaction_domain: Option<Arc<ui_wgpu::wgpu::NodeGraphInteractionDomain>>,
}

pub struct EngineSurface {
    pub surface_id: Option<EngineSurfaceId>,
    pub node_graph: Option<NodeGraphEngine>,
    pub sync_cache: NodeGraphSyncCache,
    pub map_host: Option<MapHost>,
    pub map_sync_cache: MapSyncCache,
    pub map_interaction_owner: Option<ScenePointerTarget>,
    pub board_host: Option<ManuallyDrop<infinite_canvas::BoardHost>>,
    pub board_sync_cache: BoardSyncCache,
    pub board_pending_events: infinite_canvas::BoardEventQueue,
    pub board_retiring_events: Option<infinite_canvas::BoardEventQueue>,
    pub board_pointer_inside: bool,
    pub board_pointer_claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    pub board_pointer_controller_id: Option<String>,
    /// 🐁️ The hover target last published on this board's `interactionHover` lane — outer `None` is
    /// "never published", `Some(None)` the explicit clear. React's `createCoalescingActionDispatcher`
    /// keeps the same witness, so an unchanged hover costs no guest round trip.
    pub board_hover_published: Option<Option<String>>,
    /// ⏯️ The board's resident tool run trace (`Board2dScene.tool_run_trace`), its placement footprints by kind index and
    /// the theme verdict palette it paints with. Every column is plain data, so retirement drops it in O(batches).
    pub board_trace: tool_run_trace::ToolRunTraceLayer,
    pub board_trace_shapes: Vec<tool_run_trace::ToolRunTraceShape2d>,
    pub board_trace_palette: Option<tool_run_trace::ToolRunTracePalette>,
    pub editor: Option<EditorHost>,
    pub editor_scene_pack: Option<Vec<u8>>,
    pub editor_sync_cache: EditorSyncCache,
    pub editor_delivery: TextEditorDeliveryState,
    pub raster_host: Option<RasterHost>,
    pub raster_sync_cache: RasterSyncCache,
    pub width: u32,
    pub height: u32,
    pub metrics_generation: u64,
    pub document_generation: u64,
    pub scene_revision: u64,
    last_note_click: Option<(String, f64)>,
    pub published_graph_interaction: PublishedGraphInteraction,
    pub published_graph_hit: u64,
    pub published_graph_geometry: u64,
}

pub const ENGINE_SURFACE_CAPACITY: usize = 256;
pub const ENGINE_SURFACE_ID_BYTE_CAPACITY: usize = 256;

/// ♻️ The bytes one close grant releases from a retained buffer — a page, as the prepared raster owners and the store's paged
/// retirement do: a large document costs its size over the page in grants, never a grant per scalar.
const ENGINE_CLOSE_PAGE_BYTES: usize = 64 * 1024;

/// ♻️ Releases up to one page of `text` from its end, cutting on a char boundary; `false` once it is empty.
pub fn retire_string_page(text: &mut String) -> bool {
    if text.is_empty() {
        return false;
    }
    let mut cut = text.len().saturating_sub(ENGINE_CLOSE_PAGE_BYTES);
    while !text.is_char_boundary(cut) {
        cut += 1;
    }
    text.truncate(cut);
    true
}

/// ♻️ Releases up to one page of `bytes` from its end; `false` once it is empty.
fn retire_bytes_page(bytes: &mut Vec<u8>) -> bool {
    if bytes.is_empty() {
        return false;
    }
    bytes.truncate(bytes.len().saturating_sub(ENGINE_CLOSE_PAGE_BYTES));
    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineSurfaceId {
    pub(super) bytes: [u8; ENGINE_SURFACE_ID_BYTE_CAPACITY],
    pub len: u16,
}

impl EngineSurfaceId {
    pub fn try_from_str(id: &str) -> Result<Self, ()> {
        if id.is_empty() || id.len() > ENGINE_SURFACE_ID_BYTE_CAPACITY {
            return Err(());
        }
        let mut bytes = [0; ENGINE_SURFACE_ID_BYTE_CAPACITY];
        bytes[..id.len()].copy_from_slice(id.as_bytes());
        Ok(Self { bytes, len: id.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..usize::from(self.len)]) }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn close_step(&mut self) -> bool {
        self.len = 0;
        true
    }

    pub fn with_raster_key<R>(&self, apply: impl FnOnce(&str) -> R) -> R {
        const PREFIX: &[u8] = b"engine:";
        let mut bytes = [0; ENGINE_SURFACE_ID_BYTE_CAPACITY + 7];
        bytes[..PREFIX.len()].copy_from_slice(PREFIX);
        let end = PREFIX.len() + usize::from(self.len);
        bytes[PREFIX.len()..end].copy_from_slice(&self.bytes[..usize::from(self.len)]);
        apply(unsafe { std::str::from_utf8_unchecked(&bytes[..end]) })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineSurfaceToken {
    pub slot: u16,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineSurfaceIdentity {
    pub token: EngineSurfaceToken,
    pub id: EngineSurfaceId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineSurfaceSnapshot {
    pub identity: EngineSurfaceIdentity,
    pub metrics_generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineSurfaceLiveFreshness {
    pub identity: EngineSurfaceIdentity,
    pub metrics_generation: u64,
    pub document_generation: u64,
    pub scene_revision: u64,
}

pub struct EngineSurfaceSlot {
    pub id: Option<EngineSurfaceId>,
    pub generation: u64,
    pub(super) exhausted: bool,
    pub value: Option<EngineSurface>,
    pub retirement: Option<EngineSurfaceRetirement>,
}

pub struct EngineSurfaceRegistry {
    pub slots: Box<[EngineSurfaceSlot; ENGINE_SURFACE_CAPACITY]>,
    pub next_text_editor_receipt: u64,
    pub text_editor_outbox_cursor: usize,
    pub faulted: bool,
}

impl Default for EngineSurfaceRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| EngineSurfaceSlot { id: None, generation: 0, exhausted: false, value: None, retirement: None }), next_text_editor_receipt: 1, text_editor_outbox_cursor: 0, faulted: false }
    }
}

impl EngineSurfaceRegistry {
    fn slot_index(&self, id: &str) -> Option<usize> {
        self.slots.iter().position(|slot| slot.id.as_ref().is_some_and(|stored| stored.as_str() == id))
    }

    pub fn contains_key(&self, id: &str) -> bool {
        self.slot_index(id).is_some()
    }

    pub fn get(&self, id: &str) -> Option<&EngineSurface> {
        self.slots.get(self.slot_index(id)?)?.value.as_ref()
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut EngineSurface> {
        let index = self.slot_index(id)?;
        self.slots[index].value.as_mut()
    }

    #[cfg(test)]
    pub fn token(&self, id: &str) -> Option<EngineSurfaceToken> {
        let index = self.slot_index(id)?;
        Some(EngineSurfaceToken { slot: index as u16, generation: self.slots[index].generation })
    }

    pub fn identity(&self, id: &str) -> Option<EngineSurfaceIdentity> {
        let index = self.slot_index(id)?;
        Some(EngineSurfaceIdentity { token: EngineSurfaceToken { slot: index as u16, generation: self.slots[index].generation }, id: self.slots[index].id? })
    }

    fn identity_for_token(&self, token: EngineSurfaceToken) -> Option<EngineSurfaceIdentity> {
        let slot = self.slots.get(usize::from(token.slot))?;
        (slot.generation == token.generation).then_some(EngineSurfaceIdentity { token, id: slot.id? })
    }

    pub fn reserve(&mut self, id: &str) -> Option<EngineSurfaceToken> {
        let Ok(id) = EngineSurfaceId::try_from_str(id) else {
            self.faulted = true;
            return None;
        };
        if self.contains_key(id.as_str()) {
            self.faulted = true;
            return None;
        }
        let Some(index) = self.slots.iter().position(|slot| !slot.exhausted && slot.value.is_none() && slot.retirement.is_none() && slot.id.is_none()) else {
            self.faulted = true;
            return None;
        };
        let slot = &mut self.slots[index];
        let Some(generation) = slot.generation.checked_add(1).filter(|generation| *generation != 0) else {
            slot.exhausted = true;
            self.faulted = true;
            return None;
        };
        slot.generation = generation;
        slot.id = Some(id);
        Some(EngineSurfaceToken { slot: index as u16, generation: slot.generation })
    }

    pub fn publish_reserved(&mut self, token: EngineSurfaceToken, value: EngineSurface) -> Result<(), EngineSurface> {
        let Some(slot) = self.slots.get_mut(usize::from(token.slot)) else {
            return Err(value);
        };
        if slot.generation != token.generation || slot.id.is_none() || slot.value.is_some() || slot.retirement.is_some() {
            return Err(value);
        }
        slot.value = Some(value);
        Ok(())
    }

    #[cfg(test)]
    pub fn remove(&mut self, id: &str) -> Option<EngineSurface> {
        let index = self.slot_index(id)?;
        let slot = &mut self.slots[index];
        slot.generation = slot.generation.checked_add(1).unwrap_or_else(|| {
            slot.exhausted = true;
            u64::MAX
        });
        slot.id = None;
        slot.value.take()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut EngineSurface> {
        self.slots.iter_mut().filter_map(|slot| slot.value.as_mut())
    }

    pub fn begin_close(&mut self, token: EngineSurfaceToken) -> bool {
        let Some(slot) = self.slots.get_mut(usize::from(token.slot)) else {
            return false;
        };
        if slot.generation != token.generation {
            return false;
        }
        if slot.retirement.is_some() {
            return true;
        }
        let host_id = slot.id;
        let surface = match slot.value.take() {
            Some(surface) => surface,
            None if host_id.is_some() => empty_engine_surface(1, 1),
            None => return false,
        };
        slot.retirement = Some(EngineSurfaceRetirement::new(surface, host_id));
        true
    }

    pub fn close_step(&mut self, token: EngineSurfaceToken, context: &mut semio_framework_job::StepContext<'_>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        let Some(slot) = self.slots.get_mut(usize::from(token.slot)) else {
            return false;
        };
        if slot.generation != token.generation {
            return false;
        }
        let Some(retirement) = slot.retirement.as_mut() else {
            return false;
        };
        if !retirement.close_step(context, input) {
            return false;
        }
        if !retirement.terminal_nonopaque_is_empty() {
            self.faulted = true;
            return false;
        }
        slot.retirement = None;
        slot.id = None;
        let Some(generation) = slot.generation.checked_add(1) else {
            slot.exhausted = true;
            return true;
        };
        slot.generation = generation;
        true
    }

    pub fn terminal_nonopaque_is_empty(&self, token: EngineSurfaceToken) -> bool {
        match self.slots.get(usize::from(token.slot)) {
            None => true,
            Some(slot) => slot.generation != token.generation || (slot.value.is_none() && slot.retirement.is_none() && slot.id.is_none()),
        }
    }
}

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
    pub(super) fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        match self {
            Self::Dag(owner) => owner.close_step(context),
            Self::Flow(owner) => owner.close_step(context),
        }
    }

    pub(super) fn terminal_nonopaque_is_empty(&self) -> bool {
        match self {
            Self::Dag(owner) => owner.terminal_nonopaque_is_empty(),
            Self::Flow(owner) => owner.terminal_nonopaque_is_empty(),
        }
    }
}

struct EngineSurfaceRetirement {
    pub(super) surface_id: Option<EngineSurfaceId>,
    node_graph_source: Option<NodeGraphEngine>,
    pub(super) sync_cache: NodeGraphSyncCache,
    map_source: Option<MapHost>,
    pub(super) map_sync_cache: MapSyncCache,
    board_source: Option<ManuallyDrop<infinite_canvas::BoardHost>>,
    pub(super) board_sync_cache: BoardSyncCache,
    pub(super) board_pending_events: infinite_canvas::BoardEventQueue,
    pub(super) board_retiring_events: Option<infinite_canvas::BoardEventQueue>,
    pub(super) board_pointer_claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    pub(super) board_pointer_controller_id: Option<String>,
    editor_source: Option<EditorHost>,
    pub(super) editor_scene_pack: Option<Vec<u8>>,
    pub(super) editor_sync_cache: EditorSyncCache,
    pub(super) editor_delivery: TextEditorDeliveryState,
    raster_source: Option<RasterHost>,
    pub(super) raster_sync_cache: RasterSyncCache,
    last_note_click: Option<(String, f64)>,
    pub(super) node_graph: Option<NodeGraphEngineRetirement>,
    pub(super) map: Option<semio_framework_surface::tiled_map::MapHostRetirement>,
    pub(super) editor: Option<framework_editor::EditorHostRetirement>,
    raster: Option<semio_framework_surface::paint::RasterHostRetirement>,
    pub(super) board: Option<infinite_canvas::BoardHostRetirement>,
    pub(super) phase: EngineSurfaceClosePhase,
    pub(super) faulted: bool,
}

impl EngineSurfaceRetirement {
    pub(super) fn new(surface: EngineSurface, host_id: Option<EngineSurfaceId>) -> Self {
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

    pub(super) fn close_string(value: &mut Option<String>) -> bool {
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

    fn close_node_graph_interaction_domain(value: &mut Option<Arc<ui_wgpu::wgpu::NodeGraphInteractionDomain>>) -> bool {
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

    pub(super) fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
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

    pub(super) fn terminal_nonopaque_is_empty(&self) -> bool {
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
struct EditorSyncCache {
    pub(super) scene_json: Option<String>,
    pub(super) theme_json: Option<String>,
    pub(super) size_key: Option<String>,
}

fn raster_sync_terminal(cache: &RasterSyncCache) -> bool {
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

fn editor_sync_terminal(cache: &EditorSyncCache) -> bool {
    cache.scene_json.is_none() && cache.theme_json.is_none() && cache.size_key.is_none()
}

fn node_graph_sync_terminal(cache: &NodeGraphSyncCache) -> bool {
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

fn map_sync_terminal(cache: &MapSyncCache) -> bool {
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

fn board_sync_terminal(cache: &BoardSyncCache) -> bool {
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

pub(super) fn engine_surface_live_freshness(token: EngineSurfaceToken) -> Result<Option<EngineSurfaceLiveFreshness>, ()> {
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

pub fn close_engine_surface_step(token: EngineSurfaceToken, context: &mut semio_framework_job::StepContext<'_>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    ENGINE_SURFACES.with(|cell| cell.try_borrow_mut().is_some_and(|mut registry| registry.close_step(token, context, input)))
}

pub fn engine_surface_terminal_nonopaque_is_empty(token: EngineSurfaceToken) -> Result<bool, ()> {
    ENGINE_SURFACES.with(|cell| cell.try_borrow_mut().map(|registry| registry.terminal_nonopaque_is_empty(token)).ok_or(()))
}


//#endregion Registry

//#region ✍️TextEditorDelivery

pub const TEXT_EDITOR_PENDING_ECHO_CAPACITY: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEditorDeliverySnapshot {
    pub controller_id: String,
    pub surface_id: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// ⌨️ The editor's open typing run as its host sees it (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the
/// window folds every `textEdit` delivery that names the buffer (`typing`) into ONE run; the host ends it on idle, on a pure
/// caret move, when the editor loses focus and when the page is hidden, with one commit signal, never one edit per keystroke.
#[derive(Clone, Debug, PartialEq)]
pub struct TextEditorTypingRun {
    pub controller_id: String,
    pub surface_id: String,
    last_ms: f64,
}

/// 🏁️ Why the host ends an open typing run before its idle bound (the `typingCommit` reasons of the text-splice corpus
/// `hostSignals`): the editor lost keyboard focus, or the page was hidden or left.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextEditorTypingEnd {
    Blur,
    Hidden,
}

impl TextEditorTypingEnd {
    /// 🏷️ The `typingCommit` reason the commit signal carries.
    pub fn reason(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::Hidden => "hidden",
        }
    }
}

/// ⏰️ The host clock the typing run's idle bound reads.
#[cfg(target_arch = "wasm32")]
pub fn text_editor_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn text_editor_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

pub struct TextEditorDeliveryInFlight {
    source: EngineSurfaceToken,
    pub token: std::num::NonZeroU64,
    pub snapshot: TextEditorDeliverySnapshot,
    pending_members: u8,
}

pub struct TextEditorDraftPublication {
    source: String,
    restore: Option<String>,
    pub action: ui_wgpu::wgpu::RetainedStringAction,
}

pub struct TextEditorDeliveryPublication {
    source: String,
    pub action: ui_wgpu::wgpu::RetainedStringAction,
    pub snapshot: TextEditorDeliverySnapshot,
    include_selection: bool,
}

#[derive(Default)]
pub struct TextEditorDeliveryState {
    pub active: Option<TextEditorDeliveryInFlight>,
    latest: Option<TextEditorDeliverySnapshot>,
    publication: Option<TextEditorDeliveryPublication>,
    pub pending_echoes: VecDeque<String>,
    acknowledged: Option<String>,
    pub read_only: bool,
    selection_undeclared: bool,
    pub(super) explicit_dirty: bool,
    pub(super) explicit_base: Option<String>,
    pub(super) explicit_changes: Option<String>,
    pub(super) explicit_conflict: bool,
    pub(super) explicit_past: VecDeque<String>,
    pub(super) explicit_future: VecDeque<String>,
    pub(super) explicit_snapshot: Option<String>,
    pub(super) explicit_publication: Option<TextEditorDraftPublication>,
    pub(super) explicit_active: Option<std::num::NonZeroU64>,
    pub(super) explicit_error: Option<String>,
    typing: Option<TextEditorTypingRun>,
    typing_end: Option<TextEditorTypingEnd>,
}

impl TextEditorDeliveryState {
    pub fn reconcile_buffer(&mut self, buffer: &str) -> bool {
        if let Some(index) = self.pending_echoes.iter().position(|pending| pending == buffer) {
            self.pending_echoes.drain(..=index);
            self.acknowledged = Some(buffer.to_owned());
            return false;
        }
        if self.acknowledged.as_deref() == Some(buffer) {
            return false;
        }
        self.pending_echoes.clear();
        self.acknowledged = Some(buffer.to_owned());
        true
    }

    pub fn offer(&mut self, snapshot: TextEditorDeliverySnapshot) {
        if self.latest.as_ref() == Some(&snapshot) || self.active.as_ref().is_some_and(|active| active.snapshot == snapshot) {
            return;
        }
        if self.publication.as_ref().is_some_and(|publication| publication.snapshot != snapshot) {
            self.publication = None;
        }
        self.latest = Some(snapshot);
    }

    /// 🏁️ Marks the open typing run (or the one its in-flight keystroke opens) to end for `end` once nothing is in flight;
    /// `false` when the editor neither types nor has a delivery in flight.
    fn end_typing(&mut self, end: TextEditorTypingEnd) -> bool {
        let typing = self.typing.is_some() || self.active.is_some() || self.latest.is_some() || self.publication.is_some();
        if typing {
            self.typing_end = Some(end);
        }
        typing
    }

    pub fn guest_has_text(&self, text: &str) -> bool {
        self.pending_echoes.back().map_or_else(|| self.acknowledged.as_deref() == Some(text), |pending| pending == text)
    }

    pub fn note_dispatched(&mut self, source: EngineSurfaceToken, token: std::num::NonZeroU64, include_edit: bool, include_selection: bool) -> bool {
        if self.active.is_some() {
            return false;
        }
        let Some(snapshot) = self.latest.take() else { return false };
        if include_edit {
            if self.pending_echoes.len() == TEXT_EDITOR_PENDING_ECHO_CAPACITY {
                self.pending_echoes.pop_front();
            }
            self.pending_echoes.push_back(snapshot.text.clone());
        }
        let pending_members = u8::from(include_edit) | (u8::from(include_selection) << 1);
        self.active = Some(TextEditorDeliveryInFlight { source, token, snapshot, pending_members });
        true
    }

    pub fn remove_pending_echo(&mut self, text: &str) -> bool {
        let Some(index) = self.pending_echoes.iter().position(|pending| pending == text) else { return false };
        self.pending_echoes.remove(index);
        true
    }

    pub(super) fn close_string(value: &mut String) -> bool {
        retire_string_page(value)
    }

    fn close_snapshot(snapshot: &mut TextEditorDeliverySnapshot) -> bool {
        Self::close_string(&mut snapshot.text) || Self::close_string(&mut snapshot.surface_id) || Self::close_string(&mut snapshot.controller_id)
    }

    pub(super) fn close_step(&mut self) -> bool {
        if self.latest.as_mut().is_some_and(Self::close_snapshot) {
            return false;
        }
        self.latest = None;
        if self.publication.as_mut().is_some_and(|publication| Self::close_string(&mut publication.source) || Self::close_snapshot(&mut publication.snapshot)) {
            return false;
        }
        self.publication = None;
        if self.active.as_mut().is_some_and(|active| Self::close_snapshot(&mut active.snapshot)) {
            return false;
        }
        self.active = None;
        if self.pending_echoes.back_mut().is_some_and(Self::close_string) {
            return false;
        }
        if self.pending_echoes.pop_back().is_some() {
            return false;
        }
        if self.acknowledged.as_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.acknowledged = None;
        self.read_only = false;
        self.selection_undeclared = false;
        self.explicit_dirty = false;
        if self.explicit_base.as_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.explicit_base = None;
        if self.explicit_changes.as_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.explicit_changes = None;
        self.explicit_conflict = false;
        if self.explicit_past.back_mut().is_some_and(Self::close_string) || self.explicit_future.back_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.explicit_past.pop_back();
        self.explicit_future.pop_back();
        if !self.explicit_past.is_empty() || !self.explicit_future.is_empty() {
            return false;
        }
        self.explicit_snapshot = None;
        self.explicit_publication = None;
        self.explicit_active = None;
        self.explicit_error = None;
        if self.typing.as_mut().is_some_and(|run| Self::close_string(&mut run.controller_id) || Self::close_string(&mut run.surface_id)) {
            return false;
        }
        self.typing = None;
        self.typing_end = None;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.active.is_none()
            && self.latest.is_none()
            && self.publication.is_none()
            && self.pending_echoes.is_empty()
            && self.acknowledged.is_none()
            && !self.read_only
            && !self.selection_undeclared
            && !self.explicit_dirty
            && self.explicit_base.is_none()
            && !self.explicit_conflict
            && self.explicit_past.is_empty()
            && self.explicit_future.is_empty()
            && self.explicit_snapshot.is_none()
            && self.explicit_publication.is_none()
            && self.explicit_active.is_none()
            && self.explicit_error.is_none()
            && self.typing.is_none()
            && self.typing_end.is_none()
    }
}

//#endregion ✍️TextEditorDelivery

//#region 🪵️EngineCanvasDiagnostics

/// 🚨️ Console ERROR line for a refusal at the engine edge — a scene the editor engine would not take, a member this
/// host could not read. `eprintln!` is a no-op inside a `wasm32-unknown-unknown` Worker.
pub fn engine_canvas_fault_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 🩺️ Console line for a screen-path gesture. `eprintln!` is a no-op inside a
/// `wasm32-unknown-unknown` Worker, so a wire drawn there would otherwise leave no trace at all.
pub fn engine_canvas_debug_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 📌️ The fingerprints of the three interaction actions a node-graph surface last dispatched.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PublishedGraphInteraction {
    pub select: u64,
    pub hover: u64,
    pub viewport: u64,
}

pub fn published_graph_interaction(surface_id: &str) -> PublishedGraphInteraction {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).map(|entry| entry.published_graph_interaction).unwrap_or_default())
}

//#endregion 🪵️EngineCanvasDiagnostics

//#region 📷️EngineCamera

/// 🎥️ A `{x, y, zoom}` camera document — the shape React's `parseCameraJson`/`parseBoardCamera` read
/// and the shape both hosts' `set_camera` take.
pub fn engine_camera_from_json(json: &str) -> Option<(f64, f64, f64)> {
    let value = serde_json::from_str::<Value>(json).ok()?;
    let x = value.get("x").and_then(Value::as_f64)?;
    let y = value.get("y").and_then(Value::as_f64)?;
    let zoom = value.get("zoom").and_then(Value::as_f64).filter(|zoom| *zoom > 0.0)?;
    Some((x, y, zoom))
}

//#endregion 📷️EngineCamera
