//! 🎨️ framework/products/os/modules/renderer/engine/elements/⚙️EngineCanvas/component.rs — wgpu
//! render implementation for the EngineCanvas element, extracted from lib.rs's inline
//! `pub mod engine_canvas { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired
//! via `#[path = "../../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod engine_canvas;` in
//! lib.rs in place of the former inline block; the module name `engine_canvas` is unchanged, so
//! every existing `crate::engine_canvas::...` call site elsewhere in the crate keeps resolving
//! with zero other changes.
//! 🎨️ Embeds GraphHost, FlowHost, and EditorHost via vello offscreen compositing.

use crate::interpreter::FrameworkWidgetContext;
use flow::{dag::dag_screen_to_world, FlowHost};
use framework_editor::EditorHost;
use framework_surface_node_graph::node_graph::GraphHost;
use framework_surface_tiled_map::tiled_map::{MapHost, MapInteractionIntent};
use infinite_canvas as canvas;
use infinite_world::world::{WorldAssetFault, WorldAssetRequestKind};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::mem::ManuallyDrop;
use std::sync::{Mutex, MutexGuard, OnceLock};
use ui_wgpu::wgpu::{draw_text_overlay, FontAtlas, GpuContext, KeyAction, PointerModifiers, RasterTextureStageFault, Rect, Rgba, Theme};
use ui_wgpu::wgpu::{ActionDescriptor, UiComponentSceneNode};
use vello::peniko::Color;
use vello::wgpu;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions};

#[cfg(target_arch = "wasm32")]
use js_sys;


//#region Registry
enum NodeGraphEngine {
    Dag(GraphHost),
    Flow(FlowHost),
}

#[derive(Default)]
struct NodeGraphSyncCache {
    fixture_json: Option<String>,
    selection: Option<Vec<String>>,
    preview_off_json: Option<String>,
    catalogue_json: Option<String>,
    operators: Option<Vec<ui_wgpu::wgpu::NodeGraphOperatorRecord>>,
    computing_json: Option<String>,
    status_json: Option<String>,
    eval_json: Option<String>,
    lod_json: Option<String>,
    viewport: Option<ui_wgpu::wgpu::NodeGraphViewport>,
    scene_pack: Option<Vec<u8>>,
}



struct EngineSurface {
    node_graph: Option<NodeGraphEngine>,
    sync_cache: NodeGraphSyncCache,
    map_host: Option<MapHost>,
    map_sync_cache: MapSyncCache,
    board_host: Option<ManuallyDrop<infinite_canvas::BoardHost>>,
    board_sync_cache: BoardSyncCache,
    board_pending_events: infinite_canvas::BoardEventQueue,
    board_retiring_events: Option<infinite_canvas::BoardEventQueue>,
    board_pointer_inside: bool,
    board_pointer_claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    board_pointer_controller_id: Option<String>,
    editor: Option<EditorHost>,
    editor_scene_pack: Option<Vec<u8>>,
    width: u32,
    height: u32,
    metrics_generation: u64,
    document_generation: u64,
    scene_revision: u64,
    last_note_click: Option<(String, f64)>,
}

pub(crate) const ENGINE_SURFACE_CAPACITY: usize = 256;
const ENGINE_SURFACE_ID_BYTE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EngineSurfaceId {
    bytes: [u8; ENGINE_SURFACE_ID_BYTE_CAPACITY],
    len: u16,
}

impl EngineSurfaceId {
    #[cfg(test)]
    fn try_from_str(id: &str) -> Result<Self, ()> {
        if id.is_empty() || id.len() > ENGINE_SURFACE_ID_BYTE_CAPACITY {
            return Err(());
        }
        let mut bytes = [0; ENGINE_SURFACE_ID_BYTE_CAPACITY];
        bytes[..id.len()].copy_from_slice(id.as_bytes());
        Ok(Self { bytes, len: id.len() as u16 })
    }

    fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..usize::from(self.len)]) }
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }

    fn close_step(&mut self) -> bool {
        self.len = 0;
        true
    }

    fn with_raster_key<R>(&self, apply: impl FnOnce(&str) -> R) -> R {
        const PREFIX: &[u8] = b"engine:";
        let mut bytes = [0; ENGINE_SURFACE_ID_BYTE_CAPACITY + 7];
        bytes[..PREFIX.len()].copy_from_slice(PREFIX);
        let end = PREFIX.len() + usize::from(self.len);
        bytes[PREFIX.len()..end].copy_from_slice(&self.bytes[..usize::from(self.len)]);
        apply(unsafe { std::str::from_utf8_unchecked(&bytes[..end]) })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EngineSurfaceToken {
    pub(crate) slot: u16,
    pub(crate) generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EngineSurfaceIdentity {
    token: EngineSurfaceToken,
    id: EngineSurfaceId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(test)]
struct EngineSurfaceSnapshot {
    identity: EngineSurfaceIdentity,
    metrics_generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EngineSurfaceLiveFreshness {
    identity: EngineSurfaceIdentity,
    metrics_generation: u64,
    document_generation: u64,
    scene_revision: u64,
}

struct EngineSurfaceSlot {
    id: Option<EngineSurfaceId>,
    generation: u64,
    exhausted: bool,
    value: Option<EngineSurface>,
    retirement: Option<EngineSurfaceRetirement>,
}

struct EngineSurfaceRegistry {
    slots: Box<[EngineSurfaceSlot; ENGINE_SURFACE_CAPACITY]>,
    faulted: bool,
}

impl Default for EngineSurfaceRegistry {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| EngineSurfaceSlot { id: None, generation: 0, exhausted: false, value: None, retirement: None })), faulted: false }
    }
}

impl EngineSurfaceRegistry {
    fn slot_index(&self, id: &str) -> Option<usize> {
        self.slots.iter().position(|slot| slot.id.as_ref().is_some_and(|stored| stored.as_str() == id))
    }

    #[cfg(test)]
    fn contains_key(&self, id: &str) -> bool {
        self.slot_index(id).is_some()
    }

    fn get(&self, id: &str) -> Option<&EngineSurface> {
        self.slots.get(self.slot_index(id)?)?.value.as_ref()
    }

    fn get_mut(&mut self, id: &str) -> Option<&mut EngineSurface> {
        let index = self.slot_index(id)?;
        self.slots[index].value.as_mut()
    }

    #[cfg(test)]
    fn token(&self, id: &str) -> Option<EngineSurfaceToken> {
        let index = self.slot_index(id)?;
        Some(EngineSurfaceToken { slot: index as u16, generation: self.slots[index].generation })
    }

    #[cfg(test)]
    fn identity(&self, id: &str) -> Option<EngineSurfaceIdentity> {
        let index = self.slot_index(id)?;
        Some(EngineSurfaceIdentity { token: EngineSurfaceToken { slot: index as u16, generation: self.slots[index].generation }, id: self.slots[index].id? })
    }



    #[cfg(test)]
    fn reserve(&mut self, id: &str) -> Option<EngineSurfaceToken> {
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

    #[cfg(test)]
    fn publish_reserved(&mut self, token: EngineSurfaceToken, value: EngineSurface) -> Result<(), EngineSurface> {
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
    fn remove(&mut self, id: &str) -> Option<EngineSurface> {
        let index = self.slot_index(id)?;
        let slot = &mut self.slots[index];
        slot.generation = slot.generation.checked_add(1).unwrap_or_else(|| {
            slot.exhausted = true;
            u64::MAX
        });
        slot.id = None;
        slot.value.take()
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut EngineSurface> {
        self.slots.iter_mut().filter_map(|slot| slot.value.as_mut())
    }

    fn begin_close(&mut self, token: EngineSurfaceToken) -> bool {
        let Some(slot) = self.slots.get_mut(usize::from(token.slot)) else {
            return false;
        };
        if slot.generation != token.generation {
            return false;
        }
        if slot.retirement.is_some() {
            return true;
        }
        let surface = match slot.value.take() {
            Some(surface) => surface,
            None if slot.id.is_some() => empty_engine_surface(1, 1),
            None => return false,
        };
        slot.retirement = Some(EngineSurfaceRetirement::new(surface));
        true
    }

    fn close_step(&mut self, token: EngineSurfaceToken, context: &mut semio_framework_job::StepContext<'_>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
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

    fn terminal_nonopaque_is_empty(&self, token: EngineSurfaceToken) -> bool {
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
    Board,
    BoardSync,
    Scalars,
    Witness,
    Released,
}

enum NodeGraphEngineRetirement {
    Dag(framework_surface_node_graph::node_graph::GraphHostRetirement),
    Flow(flow::FlowHostRetirement),
}

impl NodeGraphEngineRetirement {
    fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        match self {
            Self::Dag(owner) => owner.close_step(context),
            Self::Flow(owner) => owner.close_step(context),
        }
    }

    fn terminal_nonopaque_is_empty(&self) -> bool {
        match self {
            Self::Dag(owner) => owner.terminal_nonopaque_is_empty(),
            Self::Flow(owner) => owner.terminal_nonopaque_is_empty(),
        }
    }
}

struct EngineSurfaceRetirement {
    node_graph_source: Option<NodeGraphEngine>,
    sync_cache: NodeGraphSyncCache,
    map_source: Option<MapHost>,
    map_sync_cache: MapSyncCache,
    board_source: Option<ManuallyDrop<infinite_canvas::BoardHost>>,
    board_sync_cache: BoardSyncCache,
    board_pending_events: infinite_canvas::BoardEventQueue,
    board_retiring_events: Option<infinite_canvas::BoardEventQueue>,
    board_pointer_claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    board_pointer_controller_id: Option<String>,
    editor_source: Option<EditorHost>,
    editor_scene_pack: Option<Vec<u8>>,
    last_note_click: Option<(String, f64)>,
    node_graph: Option<NodeGraphEngineRetirement>,
    map: Option<framework_surface_tiled_map::tiled_map::MapHostRetirement>,
    editor: Option<framework_editor::EditorHostRetirement>,
    board: Option<infinite_canvas::BoardHostRetirement>,
    phase: EngineSurfaceClosePhase,
    faulted: bool,
}

impl EngineSurfaceRetirement {
    fn new(surface: EngineSurface) -> Self {
        let EngineSurface {
            node_graph: node_graph_source,
            sync_cache,
            map_host: map_source,
            map_sync_cache,
            board_host: board_source,
            board_sync_cache,
            board_pending_events,
            board_retiring_events,
            board_pointer_inside: _,
            board_pointer_claim,
            board_pointer_controller_id,
            editor: editor_source,
            editor_scene_pack,
            width: _,
            height: _,
            metrics_generation: _,
            document_generation: _,
            scene_revision: _,
            last_note_click,
        } = surface;
        Self {
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
            last_note_click,
            node_graph: None,
            map: None,
            editor: None,
            board: None,
            phase: EngineSurfaceClosePhase::Claim,
            faulted: false,
        }
    }

    fn close_string(value: &mut Option<String>) -> bool {
        let Some(text) = value.as_mut() else {
            return false;
        };
        if text.pop().is_none() {
            *value = None;
        }
        true
    }

    fn close_bytes(value: &mut Option<Vec<u8>>) -> bool {
        let Some(bytes) = value.as_mut() else {
            return false;
        };
        if bytes.pop().is_none() {
            *value = None;
        }
        true
    }

    fn close_node_graph_sync(cache: &mut NodeGraphSyncCache) -> bool {
        if Self::close_string(&mut cache.fixture_json)
            || cache.selection.as_mut().is_some_and(|ids| ids.last_mut().is_some_and(|id| id.pop().is_some()))
            || cache.selection.as_mut().is_some_and(|ids| ids.pop().is_some())
            || Self::close_string(&mut cache.preview_off_json)
            || Self::close_string(&mut cache.catalogue_json)
            || cache.operators.as_mut().is_some_and(|operators| operators.pop().is_some())
            || Self::close_string(&mut cache.computing_json)
            || Self::close_string(&mut cache.status_json)
            || Self::close_string(&mut cache.eval_json)
            || Self::close_string(&mut cache.lod_json)
            || cache.viewport.take().is_some()
            || Self::close_bytes(&mut cache.scene_pack)
        {
            return false;
        }
        cache.selection = None;
        cache.operators = None;
        true
    }

    fn close_map_sync(cache: &mut MapSyncCache) -> bool {
        if Self::close_string(&mut cache.map_fixture_json)
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

    fn close_board_sync(cache: &mut BoardSyncCache) -> bool {
        if Self::close_string(&mut cache.fixture_json)
            || Self::close_string(&mut cache.glyph_catalogs_json)
            || Self::close_string(&mut cache.placement_compatibility_json)
            || Self::close_string(&mut cache.selection_json)
            || Self::close_string(&mut cache.camera_json)
            || Self::close_string(&mut cache.hovered_id)
            || Self::close_string(&mut cache.active_utility)
            || Self::close_string(&mut cache.selection_method)
            || cache.grid_snap_enabled.take().is_some()
            || cache.grid_factor.take().is_some()
            || cache.suggestion_offset.take().is_some()
            || Self::close_string(&mut cache.brush_weights_json)
            || Self::close_string(&mut cache.lod_mode)
            || Self::close_string(&mut cache.size_key)
        {
            return false;
        }
        true
    }

    fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
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
                            NodeGraphEngine::Dag(owner) => NodeGraphEngineRetirement::Dag(framework_surface_node_graph::node_graph::GraphHostRetirement::new(owner)),
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
                        self.map = Some(framework_surface_tiled_map::tiled_map::MapHostRetirement::new(owner));
                    } else {
                        self.phase = EngineSurfaceClosePhase::MapSync;
                    }
                } else if self.map.as_mut().is_some_and(framework_surface_tiled_map::tiled_map::MapHostRetirement::close_step) {
                    if !self.map.as_ref().is_some_and(framework_surface_tiled_map::tiled_map::MapHostRetirement::terminal_is_empty) {
                        self.faulted = true;
                        return false;
                    }
                    self.map = None;
                    self.phase = EngineSurfaceClosePhase::MapSync;
                }
            }
            EngineSurfaceClosePhase::MapSync => {
                if Self::close_map_sync(&mut self.map_sync_cache) {
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
                if self.last_note_click.as_mut().is_some_and(|(id, _)| id.pop().is_some()) {
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

    fn terminal_nonopaque_is_empty(&self) -> bool {
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
        debug_assert!(self.terminal_nonopaque_is_empty(), "EngineSurfaceRetirement must reach terminal-empty before release");
    }
}

//#region 📦️PreparedEngineCanvas
/// 📦️ An owned vector scene produced during worker-side product traversal. Device, queue, texture,
/// and window handles are deliberately absent; the UI presenter realizes this packet only after its
/// enclosing frame generation has passed the prepared-render gate.
pub(crate) struct EngineCanvasPacket {
    surface: EngineSurfaceIdentity,
    document_generation: u64,
    scene_revision: u64,
    metrics_generation: u64,
    scene: canvas::Scene,
    scene_retirement: Option<canvas::OpaqueSceneRetirementToken>,
    scene_retirement_faulted: bool,
    clear: Color,
    width: u32,
    height: u32,
}

impl EngineCanvasPacket {
    pub(crate) fn close_step(&mut self) -> bool {
        if !self.surface.id.terminal_is_empty() {
            self.surface.id.close_step();
            return false;
        }
        if let Some(token) = self.scene_retirement {
            return match canvas::advance_opaque_scene_retirement(token, 1, 4096) {
                canvas::OpaqueSceneRetirementStep::Blocked | canvas::OpaqueSceneRetirementStep::Pending { .. } => false,
                canvas::OpaqueSceneRetirementStep::Complete { .. } => {
                    self.scene_retirement = None;
                    false
                }
                canvas::OpaqueSceneRetirementStep::Fault => {
                    self.scene_retirement_faulted = true;
                    false
                }
            };
        }
        if self.scene.retirement_is_empty() {
            return !self.scene_retirement_faulted;
        }
        let Some(token) = canvas::reserve_opaque_scene_retirement() else {
            return false;
        };
        let scene = std::mem::take(&mut self.scene);
        canvas::publish_opaque_scene_retirement(token, scene);
        self.scene_retirement = Some(token);
        false
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.surface.id.terminal_is_empty() && self.scene.retirement_is_empty() && self.scene_retirement.is_none() && !self.scene_retirement_faulted
    }
}

const ENGINE_CANVAS_FRAME_PACKET_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(test)]
enum EngineCanvasPacketDestination {
    Ready(usize),
    Rejected(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(test)]
struct EngineCanvasPacketReservation {
    generation: u64,
    sequence: u64,
    destination: EngineCanvasPacketDestination,
    surface: EngineSurfaceSnapshot,
}

/// 🧰️ Worker-owned fixed resource sink threaded through chrome/scene traversal.
pub(crate) struct EngineCanvasBuildContext {
    #[cfg(test)]
    document_generation: u64,
    #[cfg(test)]
    scene_revision: u64,
    packets: Box<[Option<EngineCanvasPacket>; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]>,
    rejected: Box<[Option<EngineCanvasPacket>; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]>,
    len: usize,
    rejected_len: usize,
    reservation_sequence: u64,
    published_reservation_sequence: u64,
    outstanding_reservations: usize,
}

impl Default for EngineCanvasBuildContext {
    fn default() -> Self {
        Self {
            #[cfg(test)]
            document_generation: 0,
            #[cfg(test)]
            scene_revision: 0,
            packets: Box::new([const { None }; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]),
            rejected: Box::new([const { None }; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]),
            len: 0,
            rejected_len: 0,
            reservation_sequence: 0,
            published_reservation_sequence: 0,
            outstanding_reservations: 0,
        }
    }
}

impl EngineCanvasBuildContext {
    #[cfg(test)]
    fn new(document_generation: u64, scene_revision: u64) -> Self {
        Self { document_generation, scene_revision, ..Self::default() }
    }

    pub(crate) fn take_packet_step(&mut self) -> Result<Option<EngineCanvasPacket>, EngineCanvasPacket> {
        if let Some(index) = self.rejected_len.checked_sub(1) {
            self.rejected_len = index;
            if let Some(rejected) = self.rejected[index].take() {
                return Err(rejected);
            }
        }
        let Some(index) = self.len.checked_sub(1) else { return Ok(None) };
        self.len = index;
        Ok(self.packets[index].take())
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.rejected_len == 0 && self.reservation_sequence == self.published_reservation_sequence && self.outstanding_reservations == 0 && self.packets.iter().all(Option::is_none) && self.rejected.iter().all(Option::is_none)
    }



    #[cfg(test)]
    fn try_reserve_fresh_packet(&mut self, surface: EngineSurfaceSnapshot) -> Result<EngineCanvasPacketReservation, EngineSurfaceSnapshot> {
        let destination = if self.len < ENGINE_CANVAS_FRAME_PACKET_CAPACITY {
            let index = self.len;
            self.len += 1;
            EngineCanvasPacketDestination::Ready(index)
        } else if self.rejected_len < ENGINE_CANVAS_FRAME_PACKET_CAPACITY {
            let index = self.rejected_len;
            self.rejected_len += 1;
            EngineCanvasPacketDestination::Rejected(index)
        } else {
            return Err(surface);
        };
        let Some(sequence) = self.reservation_sequence.checked_add(1) else {
            match destination {
                EngineCanvasPacketDestination::Ready(_) => self.len -= 1,
                EngineCanvasPacketDestination::Rejected(_) => self.rejected_len -= 1,
            }
            return Err(surface);
        };
        let Some(outstanding_reservations) = self.outstanding_reservations.checked_add(1) else {
            match destination {
                EngineCanvasPacketDestination::Ready(_) => self.len -= 1,
                EngineCanvasPacketDestination::Rejected(_) => self.rejected_len -= 1,
            }
            return Err(surface);
        };
        self.reservation_sequence = sequence;
        self.outstanding_reservations = outstanding_reservations;
        Ok(EngineCanvasPacketReservation { generation: self.document_generation, sequence, destination, surface })
    }

    #[cfg(test)]
    fn publish_reserved(&mut self, reservation: EngineCanvasPacketReservation, scene: canvas::Scene, clear: Color, width: u32, height: u32) {
        self.published_reservation_sequence = reservation.sequence;
        let packet = EngineCanvasPacket {
            surface: reservation.surface.identity,
            document_generation: reservation.generation,
            scene_revision: self.scene_revision,
            metrics_generation: reservation.surface.metrics_generation,
            scene,
            scene_retirement: None,
            scene_retirement_faulted: false,
            clear,
            width,
            height,
        };
        match reservation.destination {
            EngineCanvasPacketDestination::Ready(index) => self.packets[index] = Some(packet),
            EngineCanvasPacketDestination::Rejected(index) => self.rejected[index] = Some(packet),
        }
        self.outstanding_reservations -= 1;
    }
}

#[cfg(not(target_arch = "wasm32"))]
const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<EngineCanvasPacket>();
    assert_send::<EngineCanvasBuildContext>();
};

struct EngineGpuSurface {
    vello: Renderer,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EngineGpuBuildPhase {
    Reserve,
    Texture,
    View,
    Renderer,
    Render,
    ReplacementTexture,
    ReplacementView,
    Stage,
    Publish,
    ClosingAdmission,
    ClosingRenderer,
    ClosingView,
    ClosingTexture,
    ClosingReplacementView,
    ClosingReplacementTexture,
    Terminal,
}

struct EngineGpuCandidate {
    surface: EngineSurfaceIdentity,
    document_generation: u64,
    scene_revision: u64,
    metrics_generation: u64,
    primary_metrics_generation: u64,
    raster_generation: ui_wgpu::wgpu::RasterTextureWitness,
    width: u32,
    height: u32,
    admission: Option<ui_wgpu::wgpu::RasterTextureAdmission>,
    renderer: Option<Renderer>,
    texture: Option<wgpu::Texture>,
    view: Option<wgpu::TextureView>,
    replacement_texture: Option<wgpu::Texture>,
    replacement_view: Option<wgpu::TextureView>,
    phase: EngineGpuBuildPhase,
}

impl EngineGpuCandidate {
    fn new(packet: &EngineCanvasPacket, raster_generation: ui_wgpu::wgpu::RasterTextureWitness, primary_metrics_generation: u64) -> Self {
        Self {
            surface: packet.surface,
            document_generation: packet.document_generation,
            scene_revision: packet.scene_revision,
            metrics_generation: packet.metrics_generation,
            primary_metrics_generation,
            raster_generation,
            width: packet.width.max(1),
            height: packet.height.max(1),
            admission: None,
            renderer: None,
            texture: None,
            view: None,
            replacement_texture: None,
            replacement_view: None,
            phase: EngineGpuBuildPhase::Reserve,
        }
    }

    fn matches(&self, packet: &EngineCanvasPacket, expected: ui_wgpu::wgpu::RasterTextureWitness, primary_metrics_generation: u64) -> bool {
        self.surface == packet.surface
            && self.document_generation == packet.document_generation
            && self.scene_revision == packet.scene_revision
            && self.metrics_generation == packet.metrics_generation
            && self.primary_metrics_generation == primary_metrics_generation
            && self.raster_generation == expected
            && self.width == packet.width.max(1)
            && self.height == packet.height.max(1)
    }

    fn matches_live(&self, live: EngineSurfaceLiveFreshness) -> bool {
        engine_gpu_freshness_matches(self.surface, self.document_generation, self.scene_revision, self.metrics_generation, live)
    }

    fn begin_close(&mut self) {
        if self.phase != EngineGpuBuildPhase::Terminal {
            self.phase = EngineGpuBuildPhase::ClosingAdmission;
        }
    }

    fn close_step(&mut self, gpu: &mut GpuContext) -> Result<bool, String> {
        match self.phase {
            EngineGpuBuildPhase::ClosingAdmission => {
                if let Some(admission) = self.admission.take() {
                    gpu.cancel_engine_texture_admission(admission)?;
                    return Ok(false);
                }
                self.phase = EngineGpuBuildPhase::ClosingRenderer;
            }
            EngineGpuBuildPhase::ClosingRenderer => {
                if self.renderer.take().is_some() {
                    return Ok(false);
                }
                self.phase = EngineGpuBuildPhase::ClosingView;
            }
            EngineGpuBuildPhase::ClosingView => {
                if self.view.take().is_some() {
                    return Ok(false);
                }
                self.phase = EngineGpuBuildPhase::ClosingTexture;
            }
            EngineGpuBuildPhase::ClosingTexture => {
                if self.texture.take().is_some() {
                    return Ok(false);
                }
                self.phase = EngineGpuBuildPhase::ClosingReplacementView;
            }
            EngineGpuBuildPhase::ClosingReplacementView => {
                if self.replacement_view.take().is_some() {
                    return Ok(false);
                }
                self.phase = EngineGpuBuildPhase::ClosingReplacementTexture;
            }
            EngineGpuBuildPhase::ClosingReplacementTexture => {
                if self.replacement_texture.take().is_some() {
                    return Ok(false);
                }
                self.phase = EngineGpuBuildPhase::Terminal;
            }
            EngineGpuBuildPhase::Terminal => return Ok(true),
            _ => self.begin_close(),
        }
        Ok(self.phase == EngineGpuBuildPhase::Terminal)
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == EngineGpuBuildPhase::Terminal && self.admission.is_none() && self.renderer.is_none() && self.texture.is_none() && self.view.is_none() && self.replacement_texture.is_none() && self.replacement_view.is_none()
    }
}

fn engine_gpu_freshness_matches(surface: EngineSurfaceIdentity, document_generation: u64, scene_revision: u64, metrics_generation: u64, live: EngineSurfaceLiveFreshness) -> bool {
    surface == live.identity && document_generation == live.document_generation && scene_revision == live.scene_revision && metrics_generation == live.metrics_generation
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EngineGpuRetirementPhase {
    Renderer,
    View,
    Texture,
    Terminal,
}

struct EngineGpuRetirement {
    renderer: Option<Renderer>,
    view: Option<wgpu::TextureView>,
    texture: Option<wgpu::Texture>,
    phase: EngineGpuRetirementPhase,
}

impl EngineGpuRetirement {
    fn new(surface: EngineGpuSurface) -> Self {
        Self { renderer: Some(surface.vello), view: Some(surface.view), texture: Some(surface.texture), phase: EngineGpuRetirementPhase::Renderer }
    }

    fn close_step(&mut self) -> bool {
        match self.phase {
            EngineGpuRetirementPhase::Renderer => {
                if self.renderer.take().is_some() {
                    return false;
                }
                self.phase = EngineGpuRetirementPhase::View;
            }
            EngineGpuRetirementPhase::View => {
                if self.view.take().is_some() {
                    return false;
                }
                self.phase = EngineGpuRetirementPhase::Texture;
            }
            EngineGpuRetirementPhase::Texture => {
                if self.texture.take().is_some() {
                    return false;
                }
                self.phase = EngineGpuRetirementPhase::Terminal;
            }
            EngineGpuRetirementPhase::Terminal => return true,
        }
        self.phase == EngineGpuRetirementPhase::Terminal
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == EngineGpuRetirementPhase::Terminal && self.renderer.is_none() && self.view.is_none() && self.texture.is_none()
    }
}

struct EngineGpuSlot {
    id: Option<EngineSurfaceId>,
    generation: u64,
    exhausted: bool,
    live: Option<EngineGpuSurface>,
    candidate: Option<EngineGpuCandidate>,
    retirement: Option<EngineGpuRetirement>,
    closing: bool,
}

impl EngineGpuSlot {
    fn new() -> Self {
        Self { id: None, generation: 0, exhausted: false, live: None, candidate: None, retirement: None, closing: false }
    }

    fn token(&self, slot: usize) -> Option<EngineSurfaceToken> {
        (self.id.is_some() || self.live.is_some() || self.candidate.is_some() || self.retirement.is_some()).then_some(EngineSurfaceToken { slot: slot as u16, generation: self.generation })
    }

    fn terminal_is_empty(&self) -> bool {
        self.id.is_none() && self.live.is_none() && self.candidate.is_none() && self.retirement.is_none()
    }

    fn publish_candidate(&mut self, packet: &EngineCanvasPacket, expected: ui_wgpu::wgpu::RasterTextureWitness, primary_metrics_generation: u64) -> Result<bool, String> {
        let Some(candidate) = self.candidate.as_mut() else {
            return Err("engine GPU publication candidate disappeared".to_string());
        };
        if !candidate.matches(packet, expected, primary_metrics_generation) || self.retirement.is_some() {
            candidate.begin_close();
            return Err("engine CPU/GPU publication freshness changed".to_string());
        }
        if candidate.renderer.is_none() || candidate.replacement_texture.is_none() || candidate.replacement_view.is_none() {
            candidate.begin_close();
            return Err("engine GPU publication candidate was incomplete".to_string());
        }
        let Some(mut candidate) = self.candidate.take() else {
            return Err("engine GPU publication candidate transfer failed".to_string());
        };
        let (Some(renderer), Some(texture), Some(view)) = (candidate.renderer.take(), candidate.replacement_texture.take(), candidate.replacement_view.take()) else {
            candidate.begin_close();
            self.candidate = Some(candidate);
            return Err("engine GPU publication owner transfer was incomplete".to_string());
        };
        let published = EngineGpuSurface { vello: renderer, texture, view };
        if let Some(displaced) = self.live.replace(published) {
            self.retirement = Some(EngineGpuRetirement::new(displaced));
        }
        candidate.phase = EngineGpuBuildPhase::Terminal;
        if !candidate.terminal_is_empty() {
            candidate.begin_close();
            self.candidate = Some(candidate);
            return Err("engine GPU publication retained an unclassified candidate owner".to_string());
        }
        self.candidate = Some(candidate);
        Ok(false)
    }
}

/// 🖥️ UI-capability owner for EngineCanvas GPU realization.
pub(crate) struct EngineCanvasPresenter {
    slots: ManuallyDrop<Option<Box<[EngineGpuSlot; ENGINE_SURFACE_CAPACITY]>>>,
    primary_metrics_generation: u64,
    metrics_invalidation_scan: Option<usize>,
}

impl Default for EngineCanvasPresenter {
    fn default() -> Self {
        Self { slots: ManuallyDrop::new(Some(Box::new(std::array::from_fn(|_| EngineGpuSlot::new())))), primary_metrics_generation: 0, metrics_invalidation_scan: None }
    }
}

impl EngineCanvasPresenter {
    fn slots(&self) -> Result<&[EngineGpuSlot; ENGINE_SURFACE_CAPACITY], String> {
        self.slots.as_deref().ok_or_else(|| "engine presenter fixed slot authority was abandoned".to_string())
    }

    fn slots_mut(&mut self) -> Result<&mut [EngineGpuSlot; ENGINE_SURFACE_CAPACITY], String> {
        self.slots.as_deref_mut().ok_or_else(|| "engine presenter fixed slot authority was abandoned".to_string())
    }

    pub(crate) fn observe_primary_metrics_generation(&mut self, generation: u64) -> bool {
        if generation <= self.primary_metrics_generation {
            return generation == self.primary_metrics_generation;
        }
        self.primary_metrics_generation = generation;
        self.metrics_invalidation_scan = Some(0);
        true
    }

    pub(crate) fn invalidate_primary_metrics_step(&mut self) -> bool {
        let Some(index) = self.metrics_invalidation_scan else { return true };
        if index == ENGINE_SURFACE_CAPACITY {
            self.metrics_invalidation_scan = None;
            return true;
        }
        if let Ok(slots) = self.slots_mut() {
            if let Some(candidate) = slots.get_mut(index).and_then(|slot| slot.candidate.as_mut()) {
                candidate.begin_close();
            }
        }
        self.metrics_invalidation_scan = index.checked_add(1);
        self.metrics_invalidation_scan.is_none()
    }

    pub(crate) fn realize_step(&mut self, gpu: &mut GpuContext, packet: &EngineCanvasPacket, candidate_generation: ui_wgpu::wgpu::RasterTextureWitness, expected: ui_wgpu::wgpu::RasterTextureWitness) -> Result<bool, String> {
        if candidate_generation != expected {
            return Err("engine raster operation authority was stale before realization".to_string());
        }
        if self.metrics_invalidation_scan.is_some() {
            return Err("engine primary metrics invalidation is pending".to_string());
        }
        let primary_metrics_generation = self.primary_metrics_generation;
        let index = usize::from(packet.surface.token.slot);
        let slot = self.slots_mut()?.get_mut(index).ok_or_else(|| "engine surface token exceeded fixed GPU slots".to_string())?;
        if slot.closing || slot.exhausted {
            return Err("engine surface slot was closing or exhausted".to_string());
        }
        if let Some(retirement) = slot.retirement.as_mut() {
            if retirement.close_step() && retirement.terminal_is_empty() {
                slot.retirement = None;
            }
            return Ok(false);
        }
        if slot.id.is_none() {
            if slot.generation != 0 && slot.generation != packet.surface.token.generation {
                return Err("engine surface GPU generation was stale before reservation".to_string());
            }
            slot.id = Some(packet.surface.id);
            slot.generation = packet.surface.token.generation;
        }
        if slot.generation != packet.surface.token.generation || slot.id != Some(packet.surface.id) {
            return Err("engine surface CPU/GPU identity pair was stale".to_string());
        }
        if slot.candidate.is_none() {
            slot.candidate = Some(EngineGpuCandidate::new(packet, candidate_generation, primary_metrics_generation));
            return Ok(false);
        }
        let build = slot.candidate.as_mut().ok_or_else(|| "engine GPU candidate disappeared".to_string())?;
        if !build.matches(packet, expected, primary_metrics_generation) {
            return Err("engine GPU candidate freshness changed before step".to_string());
        }
        match build.phase {
            EngineGpuBuildPhase::Reserve => {
                let admission = build.surface.id.with_raster_key(|key| gpu.reserve_engine_texture(key, build.width, build.height, candidate_generation, expected))?;
                build.admission = Some(admission);
                build.phase = EngineGpuBuildPhase::Texture;
            }
            EngineGpuBuildPhase::Texture => {
                let admission = build.admission.as_ref().ok_or_else(|| "engine texture admission was missing".to_string())?;
                gpu.validate_engine_target_texture_allocation(admission, expected)?;
                build.texture = Some(create_target_texture(gpu.device(), build.width, build.height));
                build.phase = EngineGpuBuildPhase::View;
            }
            EngineGpuBuildPhase::View => {
                let admission = build.admission.as_ref().ok_or_else(|| "engine view admission was missing".to_string())?;
                gpu.validate_engine_target_view_allocation(admission, expected)?;
                let texture = build.texture.as_ref().ok_or_else(|| "engine texture owner was missing".to_string())?;
                build.view = Some(texture.create_view(&wgpu::TextureViewDescriptor::default()));
                build.phase = EngineGpuBuildPhase::Renderer;
            }
            EngineGpuBuildPhase::Renderer => {
                let admission = build.admission.as_ref().ok_or_else(|| "engine renderer admission was missing".to_string())?;
                gpu.validate_engine_renderer_allocation(admission, expected)?;
                build.renderer = Some(
                    Renderer::new(gpu.device(), RendererOptions { use_cpu: false, antialiasing_support: AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None })
                        .map_err(|error| format!("vello renderer: {error:?}"))?,
                );
                build.phase = EngineGpuBuildPhase::Render;
            }
            EngineGpuBuildPhase::Render => {
                let renderer = build.renderer.as_mut().ok_or_else(|| "engine renderer owner was missing".to_string())?;
                let view = build.view.as_ref().ok_or_else(|| "engine render view owner was missing".to_string())?;
                let params = RenderParams { base_color: packet.clear, width: build.width, height: build.height, antialiasing_method: AaConfig::Area };
                let vello_scene = packet.scene.vello_scene();
                renderer.render_to_texture(gpu.device(), gpu.queue(), &vello_scene, view, &params).map_err(|error| format!("vello render: {error:?}"))?;
                build.phase = EngineGpuBuildPhase::ReplacementTexture;
            }
            EngineGpuBuildPhase::ReplacementTexture => {
                let admission = build.admission.as_ref().ok_or_else(|| "engine replacement texture admission was missing".to_string())?;
                gpu.validate_engine_replacement_texture_allocation(admission, expected)?;
                build.replacement_texture = Some(create_target_texture(gpu.device(), build.width, build.height));
                build.phase = EngineGpuBuildPhase::ReplacementView;
            }
            EngineGpuBuildPhase::ReplacementView => {
                let admission = build.admission.as_ref().ok_or_else(|| "engine replacement view admission was missing".to_string())?;
                gpu.validate_engine_replacement_view_allocation(admission, expected)?;
                let texture = build.replacement_texture.as_ref().ok_or_else(|| "engine replacement texture owner was missing".to_string())?;
                build.replacement_view = Some(texture.create_view(&wgpu::TextureViewDescriptor::default()));
                build.phase = EngineGpuBuildPhase::Stage;
            }
            EngineGpuBuildPhase::Stage => {
                let admission = build.admission.take().ok_or_else(|| "engine stage admission was missing".to_string())?;
                let texture = build.texture.take().ok_or_else(|| "engine rendered texture owner was missing".to_string())?;
                let view = build.view.take().ok_or_else(|| "engine rendered view owner was missing".to_string())?;
                match gpu.stage_engine_texture(admission, texture, view, expected) {
                    Ok(()) => build.phase = EngineGpuBuildPhase::Publish,
                    Err(RasterTextureStageFault::Returned { fault, admission, texture, view }) => {
                        build.admission = Some(admission);
                        build.texture = Some(texture);
                        build.view = Some(view);
                        build.begin_close();
                        return Err(fault.to_owned());
                    }
                    Err(RasterTextureStageFault::Retained(fault)) => {
                        build.begin_close();
                        return Err(fault.to_owned());
                    }
                }
            }
            EngineGpuBuildPhase::Publish => {
                let live = match engine_surface_live_freshness(packet.surface.token) {
                    Ok(Some(live)) => live,
                    Ok(None) => {
                        build.begin_close();
                        return Err("engine CPU surface disappeared before GPU publication".to_string());
                    }
                    Err(()) => return Ok(false),
                };
                if !build.matches_live(live) {
                    build.begin_close();
                    return Err("engine CPU surface freshness changed before GPU publication".to_string());
                }
                return slot.publish_candidate(packet, expected, primary_metrics_generation);
            }
            EngineGpuBuildPhase::ClosingAdmission
            | EngineGpuBuildPhase::ClosingRenderer
            | EngineGpuBuildPhase::ClosingView
            | EngineGpuBuildPhase::ClosingTexture
            | EngineGpuBuildPhase::ClosingReplacementView
            | EngineGpuBuildPhase::ClosingReplacementTexture => {
                return Err("engine GPU candidate requires retained close".to_string());
            }
            EngineGpuBuildPhase::Terminal => {
                slot.candidate = None;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(crate) fn close_active_candidate_step(&mut self, gpu: &mut GpuContext) -> Result<bool, String> {
        let slots = self.slots_mut()?;
        let Some(index) = slots.iter().position(|slot| slot.candidate.is_some()) else { return Ok(true) };
        let Some(candidate) = slots[index].candidate.as_mut() else { return Ok(false) };
        candidate.begin_close();
        if candidate.close_step(gpu)? && candidate.terminal_is_empty() {
            slots[index].candidate = None;
        }
        Ok(false)
    }

    pub(crate) fn surface_token_at(&self, index: usize) -> Option<EngineSurfaceToken> {
        self.slots().ok()?.get(index)?.token(index)
    }

    pub(crate) fn begin_surface_close(&mut self, token: EngineSurfaceToken) -> bool {
        let Ok(slots) = self.slots_mut() else { return false };
        let Some(slot) = slots.get_mut(usize::from(token.slot)) else { return false };
        if slot.generation != token.generation {
            return false;
        }
        slot.closing = true;
        if let Some(candidate) = slot.candidate.as_mut() {
            candidate.begin_close();
        }
        true
    }

    pub(crate) fn close_surface_step(&mut self, gpu: &mut GpuContext, token: EngineSurfaceToken) -> Result<bool, String> {
        let slot = self.slots_mut()?.get_mut(usize::from(token.slot)).ok_or_else(|| "engine close token exceeded fixed GPU slots".to_string())?;
        if slot.generation != token.generation || !slot.closing {
            return Err("engine GPU close token was stale".to_string());
        }
        if let Some(candidate) = slot.candidate.as_mut() {
            candidate.begin_close();
            if candidate.close_step(gpu)? && candidate.terminal_is_empty() {
                slot.candidate = None;
            }
            return Ok(false);
        }
        if slot.retirement.is_none() {
            if let Some(live) = slot.live.take() {
                slot.retirement = Some(EngineGpuRetirement::new(live));
                return Ok(false);
            }
        }
        if let Some(retirement) = slot.retirement.as_mut() {
            if retirement.close_step() && retirement.terminal_is_empty() {
                slot.retirement = None;
            }
            return Ok(false);
        }
        if let Some(id) = slot.id.as_mut() {
            if !id.terminal_is_empty() {
                id.close_step();
                return Ok(false);
            }
            slot.id = None;
            return Ok(false);
        }
        let Some(generation) = slot.generation.checked_add(1) else {
            slot.exhausted = true;
            slot.closing = false;
            return Ok(true);
        };
        slot.generation = generation;
        slot.closing = false;
        Ok(true)
    }

    pub(crate) fn surface_terminal_is_empty(&self, token: EngineSurfaceToken) -> bool {
        self.slots().ok().and_then(|slots| slots.get(usize::from(token.slot))).is_none_or(|slot| slot.generation != token.generation || slot.terminal_is_empty())
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.metrics_invalidation_scan.is_none() && self.slots().is_ok_and(|slots| slots.iter().all(EngineGpuSlot::terminal_is_empty))
    }
}

impl Drop for EngineCanvasPresenter {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty(), "EngineCanvasPresenter must reach terminal-empty before release");
        if self.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.slots) };
        }
    }
}
//#endregion 📦️PreparedEngineCanvas

#[derive(Default)]
struct MapSyncCache {
    map_fixture_json: Option<String>,
    camera_json: Option<String>,
    render_mode: Option<String>,
    vector_style: Option<String>,
    lod_mode: Option<String>,
    layer_visibility_json: Option<String>,
    layer_stroke_scale_json: Option<String>,
    selection_json: Option<String>,
    hover_json: Option<String>,
    theme_json: Option<String>,
    size_key: Option<String>,
}

#[derive(Default)]
struct BoardSyncCache {
    fixture_json: Option<String>,
    glyph_catalogs_json: Option<String>,
    placement_compatibility_json: Option<String>,
    selection_json: Option<String>,
    camera_json: Option<String>,
    hovered_id: Option<String>,
    active_utility: Option<String>,
    selection_method: Option<String>,
    grid_snap_enabled: Option<bool>,
    grid_factor: Option<f64>,
    suggestion_offset: Option<f64>,
    brush_weights_json: Option<String>,
    lod_mode: Option<String>,
    size_key: Option<String>,
}

fn node_graph_sync_terminal(cache: &NodeGraphSyncCache) -> bool {
    cache.fixture_json.is_none()
        && cache.selection.is_none()
        && cache.preview_off_json.is_none()
        && cache.catalogue_json.is_none()
        && cache.operators.is_none()
        && cache.computing_json.is_none()
        && cache.status_json.is_none()
        && cache.eval_json.is_none()
        && cache.lod_json.is_none()
        && cache.viewport.is_none()
        && cache.scene_pack.is_none()
}

fn map_sync_terminal(cache: &MapSyncCache) -> bool {
    cache.map_fixture_json.is_none()
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
    cache.fixture_json.is_none()
        && cache.glyph_catalogs_json.is_none()
        && cache.placement_compatibility_json.is_none()
        && cache.selection_json.is_none()
        && cache.camera_json.is_none()
        && cache.hovered_id.is_none()
        && cache.active_utility.is_none()
        && cache.selection_method.is_none()
        && cache.grid_snap_enabled.is_none()
        && cache.grid_factor.is_none()
        && cache.suggestion_offset.is_none()
        && cache.brush_weights_json.is_none()
        && cache.lod_mode.is_none()
        && cache.size_key.is_none()
}

/// 🧵️ Worker-safe retained cell whose existing `with`/`borrow` call shape keeps scene code concise.
struct WorkerCell<T> {
    inner: OnceLock<Mutex<T>>,
}

impl<T> WorkerCell<T> {
    const fn new() -> Self {
        Self { inner: OnceLock::new() }
    }
}

impl<T: Default> WorkerCell<T> {
    fn state(&self) -> &Mutex<T> {
        self.inner.get_or_init(|| Mutex::new(T::default()))
    }

    fn borrow(&self) -> MutexGuard<'_, T> {
        self.state().lock().expect("worker canvas state")
    }

    fn borrow_mut(&self) -> MutexGuard<'_, T> {
        self.borrow()
    }

    fn try_borrow_mut(&self) -> Option<MutexGuard<'_, T>> {
        self.state().try_lock().ok()
    }

    fn with<R>(&self, apply: impl FnOnce(&Self) -> R) -> R {
        apply(self)
    }
}

static MAP_TILE_ASSET_FAULT: WorkerCell<Option<WorldAssetFault>> = WorkerCell::new();












static ENGINE_SURFACES: WorkerCell<EngineSurfaceRegistry> = WorkerCell::new();

#[cfg(test)]
include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs");





























fn empty_engine_surface(pw: u32, ph: u32) -> EngineSurface {
    EngineSurface {
        node_graph: None,
        sync_cache: NodeGraphSyncCache::default(),
        map_host: None,
        map_sync_cache: MapSyncCache::default(),
        board_host: None,
        board_sync_cache: BoardSyncCache::default(),
        board_pending_events: infinite_canvas::BoardEventQueue::default(),
        board_retiring_events: None,
        board_pointer_inside: false,
        board_pointer_claim: None,
        board_pointer_controller_id: None,
        editor: None,
        editor_scene_pack: None,
        width: pw.max(1),
        height: ph.max(1),
        metrics_generation: 1,
        document_generation: 0,
        scene_revision: 0,
        last_note_click: None,
    }
}







fn engine_surface_live_freshness(token: EngineSurfaceToken) -> Result<Option<EngineSurfaceLiveFreshness>, ()> {
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

pub(crate) fn engine_surface_token_at(index: usize) -> Result<Option<EngineSurfaceToken>, ()> {
    ENGINE_SURFACES.with(|cell| {
        let registry = cell.try_borrow_mut().ok_or(())?;
        let Some(slot) = registry.slots.get(index) else {
            return Ok(None);
        };
        Ok((slot.id.is_some() || slot.value.is_some() || slot.retirement.is_some()).then_some(EngineSurfaceToken { slot: index as u16, generation: slot.generation }))
    })
}

pub(crate) fn begin_engine_surface_close_token(token: EngineSurfaceToken) -> Result<bool, ()> {
    ENGINE_SURFACES.with(|cell| cell.try_borrow_mut().map(|mut registry| registry.begin_close(token)).ok_or(()))
}

pub(crate) fn close_engine_surface_step(token: EngineSurfaceToken, context: &mut semio_framework_job::StepContext<'_>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    ENGINE_SURFACES.with(|cell| cell.try_borrow_mut().is_some_and(|mut registry| registry.close_step(token, context, input)))
}

pub(crate) fn engine_surface_terminal_nonopaque_is_empty(token: EngineSurfaceToken) -> Result<bool, ()> {
    ENGINE_SURFACES.with(|cell| cell.try_borrow_mut().map(|registry| registry.terminal_nonopaque_is_empty(token)).ok_or(()))
}


fn create_target_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("engine_canvas_target"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}

//#endregion Registry

//#region NodeGraph



pub fn node_graph_apply_note_edit_key(action: KeyAction, modifiers: &PointerModifiers) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        for entry in map.values_mut() {
            let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() else {
                continue;
            };
            if host.dag.editing_note_id().is_none() {
                continue;
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
            return true;
        }
        false
    })
}

pub fn node_graph_sync_caret_blink(visible: bool) {
    ENGINE_SURFACES.with(|cell| {
        for entry in cell.borrow_mut().values_mut() {
            if let Some(NodeGraphEngine::Flow(host)) = entry.node_graph.as_mut() {
                if host.dag.editing_note_id().is_some() {
                    host.set_note_caret_visible(visible);
                }
            }
        }
    });
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

pub fn node_graph_flow_widget_drop_action(x: f32, y: f32, drag_data: &HashMap<String, String>, surfaces: &[(&str, Rect, &str)]) -> Option<ActionDescriptor> {
    let raw = drag_data.get(FLOW_WIDGET_DRAG_MIME)?;
    let descriptor: Value = serde_json::from_str(raw).ok()?;
    for (surface_id, bounds, controller_id) in surfaces {
        if !bounds.contains(x, y) {
            continue;
        }
        let world = node_graph_world_at(surface_id, bounds, x, y)?;
        return Some(ActionDescriptor {
            controller_id: (*controller_id).to_string(),
            action: "addWidget".into(),
            args: crate::action_args_json!({
                "kind": descriptor.get("kind").and_then(|value| value.as_str()).unwrap_or("inputSlider"),
                "neuronKind": descriptor.get("neuronKind").and_then(|value| value.as_str()),
                "x": world.0,
                "y": world.1,
            }),
        });
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
        eprintln!("[DEBUG] catalogue workflow drop surface={surface_id} controller={controller_id} program={plugin_id} app={app_id} world=({:.1},{:.1})", world.0, world.1);
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
#[path = "../../🧪️tests/🔬️wgpu-catalogue-workflow-drop/🦀️.rs"]
mod catalogue_workflow_drop_tests;

pub fn node_graph_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, _ctrl: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let mut reservation = input.reserve_actions(3, 3 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let planned = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(entry) = map.get(surface_id) else {
            return Ok(None);
        };
        let plan = match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => NodeGraphWheelPlan::Flow(host.plan_wheel(sx, sy, 0.0, delta as f64, true)),
            Some(NodeGraphEngine::Dag(host)) => NodeGraphWheelPlan::Dag(host.plan_wheel(sx, sy, delta as f64, true)),
            None => return Ok(None),
        };
        graph_interaction_snapshot(entry, Some(plan.camera())).map(|snapshot| Some((plan, snapshot)))
    })?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    write_graph_interaction_actions(&mut reservation, surface_id, controller_id, snapshot)?;
    reservation.publish_with_checked(|| {
        ENGINE_SURFACES.with(|cell| {
            let mut map = cell.borrow_mut();
            let Some(engine) = map.get_mut(surface_id).and_then(|entry| entry.node_graph.as_mut()) else {
                return false;
            };
            match (engine, plan) {
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
            }
        })
    })?;
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
    let mut reservation = input.reserve_actions(3, 3 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let pan = node_graph_pan_gesture(button, alt, space_pressed);
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let planned = plan_node_graph_pointer(surface_id, flow::dag::DagPointerIntent { phase: flow::dag::DagPointerPhase::Down, x: sx, y: sy, button: button.max(0) as u8, shift, ctrl_or_meta: ctrl, alt, pan })?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    write_graph_interaction_actions(&mut reservation, surface_id, controller_id, snapshot)?;
    reservation.publish_with_checked(|| commit_node_graph_pointer(surface_id, plan))?;
    Ok(true)
}

pub fn node_graph_pointer_move_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let mut reservation = input.reserve_actions(3, 3 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let planned = plan_node_graph_pointer(surface_id, flow::dag::DagPointerIntent { phase: flow::dag::DagPointerPhase::Move, x: sx, y: sy, button: 0, shift, ctrl_or_meta: ctrl, alt, pan: false })?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    write_graph_interaction_actions(&mut reservation, surface_id, controller_id, snapshot)?;
    reservation.publish_with_checked(|| commit_node_graph_pointer(surface_id, plan))?;
    Ok(true)
}

pub fn node_graph_pointer_up_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl: bool, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let mut reservation = input.reserve_actions(3, 3 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    let planned = plan_node_graph_pointer(surface_id, flow::dag::DagPointerIntent { phase: flow::dag::DagPointerPhase::Up, x: sx, y: sy, button: 0, shift, ctrl_or_meta: ctrl, alt, pan: false })?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    write_graph_interaction_actions(&mut reservation, surface_id, controller_id, snapshot)?;
    reservation.publish_with_checked(|| commit_node_graph_pointer(surface_id, plan))?;
    Ok(true)
}









struct GraphInteractionSnapshot {
    node_ids: Vec<String>,
    hovered_id: Option<String>,
    viewport_json: String,
}

enum NodeGraphWheelPlan {
    Flow(flow::FlowWheelPlan),
    Dag(framework_surface_node_graph::node_graph::GraphWheelPlan),
}

enum NodeGraphPointerPlan {
    Flow(flow::dag::DagPointerPlan),
    Dag(flow::dag::DagPointerPlan),
}

impl NodeGraphWheelPlan {
    fn camera(&self) -> [f64; 3] {
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

fn graph_projection_snapshot(node_ids: Vec<String>, hovered_id: Option<String>, camera: [f64; 3]) -> Result<GraphInteractionSnapshot, ui_wgpu::wgpu::BoundedActionFault> {
    let viewport_json = json!({ "x": camera[0], "y": camera[1], "zoom": camera[2] }).to_string();
    let mut parts = Vec::with_capacity(node_ids.len() + 2);
    parts.extend([hovered_id.as_deref().unwrap_or_default(), viewport_json.as_str()]);
    parts.extend(node_ids.iter().map(String::as_str));
    ui_wgpu::wgpu::checked_action_string_bytes(&parts)?;
    Ok(GraphInteractionSnapshot { node_ids, hovered_id, viewport_json })
}

fn plan_node_graph_pointer(surface_id: &str, intent: flow::dag::DagPointerIntent) -> Result<Option<(NodeGraphPointerPlan, GraphInteractionSnapshot)>, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(engine) = map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) else {
            return Ok(None);
        };
        match engine {
            NodeGraphEngine::Flow(host) => {
                let plan = host.plan_pointer(intent).map_err(graph_plan_fault)?;
                let flow::dag::DagPointerSnapshot { node_ids, hovered_id, camera } = host.pointer_projection_snapshot(&plan).map_err(graph_plan_fault)?;
                Ok(Some((NodeGraphPointerPlan::Flow(plan), graph_projection_snapshot(node_ids, hovered_id, camera)?)))
            }
            NodeGraphEngine::Dag(host) => {
                let plan = host.plan_pointer(intent).map_err(graph_plan_fault)?;
                let flow::dag::DagPointerSnapshot { node_ids, hovered_id, camera } = host.pointer_projection_snapshot(&plan).map_err(graph_plan_fault)?;
                Ok(Some((NodeGraphPointerPlan::Dag(plan), graph_projection_snapshot(node_ids, hovered_id, camera)?)))
            }
        }
    })
}

fn commit_node_graph_pointer(surface_id: &str, plan: NodeGraphPointerPlan) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(engine) = map.get_mut(surface_id).and_then(|entry| entry.node_graph.as_mut()) else {
            return false;
        };
        match (engine, plan) {
            (NodeGraphEngine::Flow(host), NodeGraphPointerPlan::Flow(plan)) => host.commit_pointer(plan),
            (NodeGraphEngine::Dag(host), NodeGraphPointerPlan::Dag(plan)) => host.commit_pointer(plan),
            _ => false,
        }
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
            (selected, hovered, [host.fixture.camera.x, host.fixture.camera.y, host.fixture.camera.zoom])
        }
        Some(NodeGraphEngine::Dag(host)) => {
            let (selected, hovered) = bounded_graph_selection(&host.dag)?;
            (selected, hovered, [host.dag.fixture.camera.x, host.dag.fixture.camera.y, host.dag.fixture.camera.zoom])
        }
        None => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
    };
    graph_projection_snapshot(node_ids, hovered_id, camera.unwrap_or(current_camera))
}

fn write_graph_interaction_actions(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, surface_id: &str, controller_id: &str, snapshot: GraphInteractionSnapshot) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let select_action = "interactionSelect";
    let select_targets = serde_json::to_string(&snapshot.node_ids.iter().map(|id| json!({ "granularity": "node", "id": id })).collect::<Vec<_>>()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let select_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, select_action, "domainId", "graph", "targets", select_targets.as_str(), "merge", "replace", "method", "pick"])?;
    batch.action(controller_id, select_action, select_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("domainId"), "graph")?;
        builder.string(Some("targets"), &select_targets)?;
        builder.string(Some("merge"), "replace")?;
        builder.string(Some("method"), "pick")?;
        builder.end_container()
    })?;
    let hover_action = "interactionHover";
    let hover_targets = serde_json::to_string(&snapshot.hovered_id.iter().map(|id| json!({ "granularity": "node", "id": id })).collect::<Vec<_>>()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let hover_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, hover_action, "domainId", "graph", "channel", "pointer", "targets", hover_targets.as_str()])?;
    batch.action(controller_id, hover_action, hover_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("domainId"), "graph")?;
        builder.string(Some("channel"), "pointer")?;
        builder.string(Some("targets"), &hover_targets)?;
        builder.end_container()
    })?;
    let viewport_action = "nodeGraphViewport";
    let viewport_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, viewport_action, "surfaceId", surface_id, "viewportJson", snapshot.viewport_json.as_str()])?;
    batch.action(controller_id, viewport_action, viewport_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.string(Some("viewportJson"), &snapshot.viewport_json)?;
        builder.end_container()
    })?;
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
    hovered_id: Option<String>,
    dimmed_ids: Vec<String>,
}

fn label_chrome_from_flow(host: &FlowHost) -> LabelInteractionChrome {
    let selected: Vec<String> = serde_json::from_str(&host.selected_widget_ids_json()).unwrap_or_default();
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

fn clamp_label_font_px(atlas: &mut FontAtlas, text: &str, target_px: f32, max_w: f32, max_h: f32) -> f32 {
    let px = target_px.max(4.0).round();
    let (w, h) = atlas.measure_text(text, px);
    if w <= max_w && h * 1.2 <= max_h {
        return px;
    }
    let mut low = 4.0_f32;
    let mut high = px;
    let mut best = 4.0_f32;
    while low <= high {
        let mid = ((low + high) * 0.5).floor();
        let (w, h) = atlas.measure_text(text, mid);
        if w <= max_w && h * 1.2 <= max_h {
            best = mid;
            low = mid + 1.0;
        } else {
            high = mid - 1.0;
        }
    }
    best
}

fn clamp_port_label_font_px(atlas: &mut FontAtlas, text: &str, target_px: f32, max_w: f32, max_h: f32) -> f32 {
    let px = target_px.max(8.0).round();
    let (w, _) = atlas.measure_text(text, px);
    if w <= max_w && px * 1.25 <= max_h {
        return px;
    }
    let mut low = 8.0_f32;
    let mut high = px;
    let mut best = 8.0_f32;
    while low <= high {
        let mid = ((low + high) * 0.5).floor();
        let (w, _) = atlas.measure_text(text, mid);
        if w <= max_w {
            best = mid;
            low = mid + 1.0;
        } else {
            high = mid - 1.0;
        }
    }
    best
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
    let max_w = (node_w * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32;
    let max_h = if is_port {
        row.get("maxScreenH").and_then(|v| v.as_f64()).filter(|h| *h > 0.0).map(|h| h as f32).unwrap_or((node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32)
    } else {
        (node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32
    };
    let target_px = row.get("fontScreenPx").and_then(|v| v.as_f64()).filter(|px| *px > 0.0).map(|px| px as f32).unwrap_or(DAG_LABEL_SCREEN_PX);
    let font_px = if is_port { clamp_port_label_font_px(&mut ctx.atlas, text, target_px, max_w, max_h) } else { clamp_label_font_px(&mut ctx.atlas, text, target_px, max_w, max_h) };
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
        let entry = map.get(&scene.surface_id)?;
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
    let Some(snapshot) = node_graph_overlay_snapshot(&scene.surface_id) else {
        return;
    };
    let points = parse_selection_preview_points(&snapshot.preview_points_json);
    paint_node_graph_selection_marquee(ctx, inner, &points, snapshot.preview_crossing, &snapshot.preview_method, ctx.theme);
    paint_node_graph_selection_bounds(ctx, inner, &snapshot.selection_bounds_json, ctx.theme);
}
//#endregion NodeGraph

//#region TiledMap


pub fn take_map_tile_asset_fault() -> Option<WorldAssetFault> {
    MAP_TILE_ASSET_FAULT.with(|cell| cell.borrow_mut().take())
}




pub fn apply_map_tile_bytes(kind: WorldAssetRequestKind, bytes: &[u8]) {
    let WorldAssetRequestKind::MapTile { surface, key: _, vector, z, x, y } = kind else { return };
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface.as_str()) else {
            return;
        };
        let Some(host) = entry.map_host.as_mut() else {
            return;
        };
        let _ = if vector { host.upload_vector_tile(z, x, y, bytes) } else { host.upload_tile(z, x, y, bytes) };
    });
}


pub fn with_map_host_mut<R>(surface_id: &str, f: impl FnOnce(&mut MapHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        let host = entry.map_host.as_mut()?;
        Some(f(host))
    })
}

pub fn with_map_host<R>(surface_id: &str, f: impl FnOnce(&MapHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        let host = entry.map_host.as_ref()?;
        Some(f(host))
    })
}



pub fn map_local_pointer(inner: Rect, x: f32, y: f32) -> (f64, f64) {
    ((x - inner.x) as f64, (y - inner.y) as f64)
}

pub fn map_marquee_mode(shift: bool, ctrl_or_meta: bool) -> &'static str {
    if shift && ctrl_or_meta {
        "invertive"
    } else if shift {
        "additive"
    } else if ctrl_or_meta {
        "subtractive"
    } else {
        "default"
    }
}

pub fn map_marquee_crossing(method: &str, start_x: f32, end_x: f32) -> bool {
    if method == "lasso" {
        end_x < start_x
    } else {
        end_x < start_x
    }
}

pub fn map_merge_selection(mode: &str, current_positions: &[String], current_routes: &[String], next_positions: &[String], next_routes: &[String]) -> (Vec<String>, Vec<String>) {
    let mut positions: HashSet<String> = current_positions.iter().cloned().collect();
    let mut routes: HashSet<String> = current_routes.iter().cloned().collect();
    let next_pos: HashSet<String> = next_positions.iter().cloned().collect();
    let next_routes: HashSet<String> = next_routes.iter().cloned().collect();
    match mode {
        "additive" => {
            positions.extend(next_pos);
            routes.extend(next_routes);
        }
        "subtractive" => {
            positions.retain(|id| !next_pos.contains(id));
            routes.retain(|id| !next_routes.contains(id));
        }
        "invertive" => {
            for id in next_pos {
                if !positions.insert(id.clone()) {
                    positions.remove(&id);
                }
            }
            for id in next_routes {
                if !routes.insert(id.clone()) {
                    routes.remove(&id);
                }
            }
        }
        _ => {
            positions = next_pos;
            routes = next_routes;
        }
    }
    (positions.into_iter().collect(), routes.into_iter().collect())
}

pub fn parse_map_feature_hit(hit_json: &str) -> (Vec<String>, Vec<String>) {
    let hit: Value = serde_json::from_str(hit_json).unwrap_or(Value::Null);
    let positions = hit.get("positions").and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect::<Vec<_>>()).unwrap_or_default();
    let routes = hit.get("routes").and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect::<Vec<_>>()).unwrap_or_default();
    (positions, routes)
}

pub fn parse_map_hover(hit_json: &str) -> Value {
    if hit_json == "null" {
        return Value::Null;
    }
    serde_json::from_str(hit_json).unwrap_or(Value::Null)
}



struct MapInteractionSnapshot {
    camera: [f64; 3],
    positions: Vec<String>,
    routes: Vec<String>,
    hover: Option<(String, String)>,
}

fn map_interaction_snapshot(host: &MapHost, camera: [f64; 3]) -> Result<MapInteractionSnapshot, ui_wgpu::wgpu::BoundedActionFault> {
    let position_count = host.selected_position_ids().len();
    let route_count = host.selected_route_ids().len();
    if position_count.checked_add(route_count).ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)? > ui_wgpu::wgpu::action::ACTION_NODE_CAPACITY - 4 {
        return Err(ui_wgpu::wgpu::BoundedActionFault::NodeCredits);
    }
    let mut bytes = 0usize;
    for part in host.selected_position_ids().chain(host.selected_route_ids()).chain(host.hovered_kind()).chain(host.hovered_id()) {
        if part.len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        bytes = bytes.checked_add(part.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        if bytes > ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
    }
    let positions = host.selected_position_ids().map(str::to_owned).collect();
    let routes = host.selected_route_ids().map(str::to_owned).collect();
    let hover = host.hovered_kind().zip(host.hovered_id()).map(|(kind, id)| (kind.to_owned(), id.to_owned()));
    Ok(MapInteractionSnapshot { camera, positions, routes, hover })
}

fn write_map_interaction_actions(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, surface_id: &str, controller_id: &str, snapshot: MapInteractionSnapshot) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let camera_action = ui_wgpu::wgpu::tiled_map_actions::SET_CAMERA;
    let camera_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, camera_action, "surfaceId", surface_id, "camera", "x", "y", "zoom"])?;
    batch.action(controller_id, camera_action, camera_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.begin_object(Some("camera"))?;
        builder.number(Some("x"), snapshot.camera[0])?;
        builder.number(Some("y"), snapshot.camera[1])?;
        builder.number(Some("zoom"), snapshot.camera[2])?;
        builder.end_container()?;
        builder.end_container()
    })?;
    let selection_action = ui_wgpu::wgpu::tiled_map_actions::SET_FEATURE_SELECTION;
    let selection_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, selection_action, "surfaceId", surface_id, "positions", "routes"])?
        + snapshot.positions.iter().map(String::len).sum::<usize>()
        + snapshot.routes.iter().map(String::len).sum::<usize>();
    batch.action(controller_id, selection_action, selection_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.begin_array(Some("positions"))?;
        for id in &snapshot.positions {
            builder.string(None, id)?;
        }
        builder.end_container()?;
        builder.begin_array(Some("routes"))?;
        for id in &snapshot.routes {
            builder.string(None, id)?;
        }
        builder.end_container()?;
        builder.end_container()
    })?;
    let hover_action = ui_wgpu::wgpu::tiled_map_actions::SET_HOVER;
    let (kind, id) = snapshot.hover.as_ref().map(|(kind, id)| (kind.as_str(), id.as_str())).unwrap_or(("", ""));
    let hover_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, hover_action, "surfaceId", surface_id, "hover", "kind", "id", kind, id])?;
    batch.action(controller_id, hover_action, hover_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        if snapshot.hover.is_some() {
            builder.begin_object(Some("hover"))?;
            builder.string(Some("kind"), kind)?;
            builder.string(Some("id"), id)?;
            builder.end_container()?;
        } else {
            builder.null(Some("hover"))?;
        }
        builder.end_container()
    })?;
    Ok(())
}

pub fn with_map_interaction_into(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, intent: MapInteractionIntent) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let mut reservation = input.reserve_actions(3, 3 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let planned = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(host) = map.get(surface_id).and_then(|entry| entry.map_host.as_ref()) else {
            return Ok(None);
        };
        let plan = host.plan_interaction(intent);
        map_interaction_snapshot(host, plan.camera()).map(|snapshot| Some((plan, snapshot)))
    })?;
    let Some((plan, snapshot)) = planned else {
        return Ok(false);
    };
    write_map_interaction_actions(&mut reservation, surface_id, controller_id, snapshot)?;
    reservation.publish_with_checked(|| ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(surface_id).and_then(|entry| entry.map_host.as_mut()).is_some_and(|host| host.commit_interaction(plan))))?;
    Ok(true)
}

pub fn tiled_map_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, ctrl: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    let delta_y = if ctrl { delta as f64 * 2.5 } else { delta as f64 };
    with_map_interaction_into(surface_id, controller_id, input, MapInteractionIntent::Wheel { sx, sy, delta_y })
}






//#endregion TiledMap

//#region Board2d
/// @emoji 🧩️ Raw event row drained from {@link infinite_canvas::BoardHost::drain_events_json}; mirrors the TS `BoardEventRow` shape.
#[cfg(test)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BoardEventRow {
    pub name: String,
    #[serde(default)]
    pub payload: Value,
}

pub struct CoalescedBoardEvents {
    pub flush_now: bool,
    pub events_json: String,
}

#[cfg(test)]
const PUZZLE2D_TRANSIENT_EVENT_NAMES: &[&str] = &["preselect", "brushPreview", "linkCompatibleNodes", "linkTargetRing"];
#[cfg(test)]
const PUZZLE2D_FLUSH_NOW_EVENT_NAMES: &[&str] = &["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"];

/// @emoji 📬️ Drops transient rows, coalesces `camera` to its latest value and `nodeMove` to one row per id (unless a `nodeDragEnd` follows), and flags whether the buffer should flush immediately. Port of `coalesceBoard2dEvents` in the React host.






pub fn with_board_host_mut<R>(surface_id: &str, f: impl FnOnce(&mut infinite_canvas::BoardHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        let host = entry.board_host.as_mut()?;
        Some(f(host))
    })
}

pub fn with_board_host<R>(surface_id: &str, f: impl FnOnce(&infinite_canvas::BoardHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        let host = entry.board_host.as_ref()?;
        Some(f(host))
    })
}



/// @emoji 🎯️ Most-specific pick target at a screen point, mirroring `pickMostSpecificCanvasTarget`.
pub fn board_pick_best_target_id(surface_id: &str, sx: f64, sy: f64) -> Option<String> {
    with_board_host(surface_id, |host| {
        let json = host.pick_targets_at_screen_json(sx, sy);
        let targets: Vec<Value> = serde_json::from_str(&json).unwrap_or_default();
        targets.into_iter().max_by_key(|t| t.get("generality").and_then(Value::as_u64).unwrap_or(0)).and_then(|t| t.get("id").and_then(|v| v.as_str()).map(str::to_string))
    })
    .flatten()
}

fn board_event_transient(kind: infinite_canvas::BoardEventKind) -> bool {
    use infinite_canvas::BoardEventKind;
    matches!(kind, BoardEventKind::Preselect | BoardEventKind::BrushPreview | BoardEventKind::LinkCompatibleNodes | BoardEventKind::LinkTargetRing)
}

fn board_event_flush_now(kind: infinite_canvas::BoardEventKind) -> bool {
    use infinite_canvas::BoardEventKind;
    matches!(kind, BoardEventKind::Select | BoardEventKind::PreselectCancel | BoardEventKind::BrushCandidates | BoardEventKind::BrushPlace | BoardEventKind::EdgeCreate | BoardEventKind::EdgeDelete | BoardEventKind::NodeDelete)
}

fn append_board_owned_event(output: &mut String, first: &mut bool, event: &infinite_canvas::BoardOwnedEvent) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    if !*first {
        output.push(',');
    }
    *first = false;
    event.write_json(output);
    if output.len() > ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
    }
    Ok(())
}

fn coalesce_owned_board_events(queue: &infinite_canvas::BoardEventQueue) -> Result<CoalescedBoardEvents, ui_wgpu::wgpu::BoundedActionFault> {
    use infinite_canvas::BoardEventKind;
    let has_drag_end = queue.iter().any(|event| event.kind() == BoardEventKind::NodeDragEnd);
    let mut output = String::from("[");
    let mut first = true;
    let mut flush_now = false;
    if let Some(camera) = queue.iter().filter(|event| event.kind() == BoardEventKind::Camera).last() {
        append_board_owned_event(&mut output, &mut first, camera)?;
    }
    if !has_drag_end {
        for (index, event) in queue.iter().enumerate() {
            if event.kind() != BoardEventKind::NodeMove {
                continue;
            }
            let key = event.key();
            if queue.iter().take(index).any(|candidate| candidate.kind() == BoardEventKind::NodeMove && candidate.key() == key) {
                continue;
            }
            let latest = queue.iter().skip(index).filter(|candidate| candidate.kind() == BoardEventKind::NodeMove && candidate.key() == key).last().expect("first node move is a latest candidate");
            append_board_owned_event(&mut output, &mut first, latest)?;
        }
    }
    for event in queue.iter() {
        let kind = event.kind();
        if board_event_flush_now(kind) {
            flush_now = true;
        }
        if kind == BoardEventKind::Camera || kind == BoardEventKind::NodeMove || board_event_transient(kind) {
            continue;
        }
        append_board_owned_event(&mut output, &mut first, event)?;
    }
    output.push(']');
    Ok(CoalescedBoardEvents { flush_now, events_json: output })
}

fn board_retirement_step(entry: &mut EngineSurface) -> bool {
    if let Some(retiring) = entry.board_retiring_events.as_mut() {
        if retiring.close_step() {
            entry.board_retiring_events = None;
        }
        return true;
    }
    false
}

fn board_drain_into_buffer(surface_id: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return false;
        };
        let retired = board_retirement_step(entry);
        if retired {
            return true;
        }
        let Some(host) = entry.board_host.as_mut() else {
            return false;
        };
        let Some(bytes) = host.peek_owned_event().map(|event| event.owned_bytes()) else {
            return false;
        };
        if entry.board_pending_events.reserve(1, bytes).is_err() {
            return false;
        }
        let event = host.pop_owned_event().expect("peeked board event remains owned by host");
        entry.board_pending_events.push(event).is_ok()
    })
}



fn board_peek_buffer_coalesced(surface_id: &str) -> Option<String> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        if entry.board_retiring_events.is_some() {
            return None;
        }
        let queue = &entry.board_pending_events;
        if queue.is_empty() {
            return None;
        }
        let coalesced = coalesce_owned_board_events(queue).ok()?;
        (coalesced.events_json != "[]").then_some(coalesced.events_json)
    })
}

/// @emoji 📤️ Unconditional drain + coalesce + dispatch, mirroring `flushBoardEvents` (used after pointer-up, pointer-leave, and wheel).


/// @emoji 📤️ Drains into the buffer and only dispatches if a flush-now event (select, brushPlace, edgeCreate, ...) is pending, mirroring `drainAndMaybeFlush` (used on pointer-move).




fn write_board_events_flat(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, events_json: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let action = "applyBoardEvents";
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "eventsJson", events_json])?;
    batch.action(controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("eventsJson"), events_json)?;
        builder.end_container()
    })
}

fn write_board_camera_flat(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, camera: [f64; 3]) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let action = "setCamera";
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, action, "camera", "x", "y", "zoom"])?;
    batch.action(controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.begin_object(Some("camera"))?;
        builder.number(Some("x"), camera[0])?;
        builder.number(Some("y"), camera[1])?;
        builder.number(Some("zoom"), camera[2])?;
        builder.end_container()?;
        builder.end_container()
    })
}

fn board_set_pointer_inside(surface_id: &str, inside: bool) {
    ENGINE_SURFACES.with(|cell| {
        if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
            entry.board_pointer_inside = inside;
        }
    });
}

fn board_pointer_plan_fault(fault: infinite_canvas::BoardPointerPlanFault) -> ui_wgpu::wgpu::BoundedActionFault {
    match fault {
        infinite_canvas::BoardPointerPlanFault::ItemCredits => ui_wgpu::wgpu::BoundedActionFault::ItemCredits,
        infinite_canvas::BoardPointerPlanFault::ByteCredits => ui_wgpu::wgpu::BoundedActionFault::ByteCredits,
        infinite_canvas::BoardPointerPlanFault::Unsupported => ui_wgpu::wgpu::BoundedActionFault::Structure,
    }
}

fn plan_board_pointer(surface_id: &str, intent: infinite_canvas::BoardPointerIntent) -> Result<Option<infinite_canvas::BoardPointerPlan>, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(host) = map.get(surface_id).and_then(|entry| entry.board_host.as_ref()) else {
            return Ok(None);
        };
        host.plan_pointer(intent).map(Some).map_err(board_pointer_plan_fault)
    })
}

fn commit_board_pointer(surface_id: &str, plan: &infinite_canvas::BoardPointerPlan, pointer_inside: Option<bool>) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return false;
        };
        let Some(host) = entry.board_host.as_mut() else {
            return false;
        };
        if !host.commit_pointer(plan) {
            return false;
        }
        if let Some(pointer_inside) = pointer_inside {
            entry.board_pointer_inside = pointer_inside;
        }
        true
    })
}

fn begin_board_pointer_commit(
    surface_id: &str,
    controller_id: &str,
    plan: infinite_canvas::BoardPointerPlan,
    pointer_inside: Option<bool>,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let emits = plan.event_count() > 0;
    let claim = if emits {
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", plan.events_json()])?;
        Some(input.claim_action(bytes)?)
    } else {
        None
    };
    let admitted = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        };
        if entry.board_pointer_claim.is_some() || entry.board_pointer_controller_id.is_some() {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
        }
        let Some(host) = entry.board_host.as_mut() else {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        };
        host.begin_pointer_commit(plan).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
        entry.board_pointer_claim = claim;
        entry.board_pointer_controller_id = emits.then(|| controller_id.to_owned());
        if let Some(pointer_inside) = pointer_inside {
            entry.board_pointer_inside = pointer_inside;
        }
        Ok(())
    });
    if admitted.is_err() {
        if let Some(claim) = claim {
            input.release_action_claim(claim)?;
        }
    }
    admitted
}

pub fn drive_board_authority_step(surface_id: &str, context: &mut semio_framework_job::StepContext<'_>) -> infinite_canvas::BoardAuthorityStep {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(host) = map.get_mut(surface_id).and_then(|entry| entry.board_host.as_mut()) else {
            return infinite_canvas::BoardAuthorityStep::Complete;
        };
        if !host.pointer_authority_terminal_is_empty() {
            return host.step_pointer_commit(context);
        }
        host.step_event_authority(context)
    })
}

pub fn publish_board_pointer_step(surface_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else {
            return Ok(false);
        };
        let Some(host) = entry.board_host.as_mut() else { return Ok(false) };
        let Some(publication) = host.pointer_publication() else {
            return Ok(false);
        };
        let claim = entry.board_pointer_claim.ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let controller_id = entry.board_pointer_controller_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let events_json = publication.events_json();
        let mut reservation = input.reserve_claimed_action(claim, controller_id, "applyBoardEvents")?;
        reservation.builder().begin_object(None)?;
        reservation.builder().string(Some("eventsJson"), events_json)?;
        reservation.builder().end_container()?;
        reservation.publish_with_checked(|| {
            let Some(mut publication) = host.take_pointer_publication() else {
                return false;
            };
            publication.close_step() && publication.terminal_is_empty()
        })?;
        entry.board_pointer_claim = None;
        entry.board_pointer_controller_id = None;
        Ok(true)
    })
}

pub fn release_board_pointer_claim(surface_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let claim = ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        entry.board_pointer_controller_id = None;
        entry.board_pointer_claim.take()
    });
    match claim {
        Some(claim) => input.release_action_claim(claim),
        None => Ok(()),
    }
}

pub fn publish_board_event_step(surface_id: &str, controller_id: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(host) = map.get_mut(surface_id).and_then(|entry| entry.board_host.as_mut()) else {
            return Ok(false);
        };
        let Some(event) = host.peek_owned_event() else {
            return Ok(false);
        };
        let mut events_json = String::with_capacity(event.owned_bytes().saturating_add(32));
        events_json.push('[');
        event.write_json(&mut events_json);
        events_json.push(']');
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", &events_json])?;
        let mut reservation = input.reserve_actions(1, bytes)?;
        write_board_events_flat(&mut reservation, controller_id, &events_json)?;
        reservation.publish_with_checked(|| host.pop_owned_event().is_some())?;
        Ok(true)
    })
}

pub fn puzzle_board_pointer_down(surface_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool) {
    let (sx, sy) = map_local_pointer(inner, x, y);
    with_board_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, button.max(0) as u8, shift, ctrl_or_meta));
    board_set_pointer_inside(surface_id, true);
}

pub fn puzzle_board_pointer_move_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    let plan = plan_board_pointer(surface_id, infinite_canvas::BoardPointerIntent { phase: infinite_canvas::BoardPointerPhase::Move, x: sx, y: sy, shift, ctrl_or_meta, alt })?;
    let Some(plan) = plan else {
        return Ok(false);
    };
    if plan.requires_retained_commit() {
        let emits = plan.event_count() > 0;
        begin_board_pointer_commit(surface_id, controller_id, plan, Some(true), input)?;
        return Ok(emits);
    }
    if plan.event_count() == 0 {
        if !commit_board_pointer(surface_id, &plan, Some(true)) {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        return Ok(false);
    }
    let events_json = plan.events_json();
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", events_json])?;
    let mut reservation = input.reserve_actions(1, bytes)?;
    write_board_events_flat(&mut reservation, controller_id, events_json)?;
    reservation.publish_with_checked(|| commit_board_pointer(surface_id, &plan, Some(true)))?;
    Ok(true)
}

pub fn puzzle_board_pointer_up_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let (sx, sy) = map_local_pointer(inner, x, y);
    let plan = plan_board_pointer(surface_id, infinite_canvas::BoardPointerIntent { phase: infinite_canvas::BoardPointerPhase::Up, x: sx, y: sy, shift, ctrl_or_meta, alt })?;
    let Some(plan) = plan else {
        return Ok(false);
    };
    if plan.requires_retained_commit() {
        let emits = plan.event_count() > 0;
        begin_board_pointer_commit(surface_id, controller_id, plan, None, input)?;
        return Ok(emits);
    }
    if plan.event_count() == 0 {
        if !commit_board_pointer(surface_id, &plan, None) {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        return Ok(false);
    }
    let events_json = plan.events_json();
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", events_json])?;
    let mut reservation = input.reserve_actions(1, bytes)?;
    write_board_events_flat(&mut reservation, controller_id, events_json)?;
    reservation.publish_with_checked(|| commit_board_pointer(surface_id, &plan, None))?;
    Ok(true)
}

pub fn puzzle_board_pointer_leave_into(surface_id: &str, controller_id: &str, alt: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let was_inside = ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).is_some_and(|entry| entry.board_pointer_inside));
    if !was_inside {
        return Ok(false);
    }
    let plan = plan_board_pointer(surface_id, infinite_canvas::BoardPointerIntent { phase: infinite_canvas::BoardPointerPhase::Leave, x: 0.0, y: 0.0, shift: false, ctrl_or_meta: false, alt })?;
    let Some(plan) = plan else {
        return Ok(false);
    };
    if plan.requires_retained_commit() {
        let emits = plan.event_count() > 0;
        begin_board_pointer_commit(surface_id, controller_id, plan, Some(false), input)?;
        return Ok(emits);
    }
    if plan.event_count() == 0 {
        if !commit_board_pointer(surface_id, &plan, Some(false)) {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        return Ok(false);
    }
    let events_json = plan.events_json();
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "applyBoardEvents", "eventsJson", events_json])?;
    let mut reservation = input.reserve_actions(1, bytes)?;
    write_board_events_flat(&mut reservation, controller_id, events_json)?;
    reservation.publish_with_checked(|| commit_board_pointer(surface_id, &plan, Some(false)))?;
    Ok(true)
}







/// @emoji 🖐️ True while a node drag or area-select gesture is in flight, so pointer-up outside the surface bounds still reaches the host (mirrors `tiled_map_drag_active`).
pub fn board_drag_active(surface_id: &str) -> bool {
    with_board_host(surface_id, |host| host.defers_descriptor_sync_from_js() || host.is_dragging_area_select()).unwrap_or(false)
}

pub fn puzzle_board_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let mut reservation = input.reserve_actions(2, 2 * ui_wgpu::wgpu::action::ACTION_ITEM_BYTE_CAPACITY)?;
    let (sx, sy) = map_local_pointer(inner, x, y);
    let plan = with_board_host(surface_id, |host| host.plan_wheel(sx, sy, delta as f64)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
    let camera = plan.camera();
    board_drain_into_buffer(surface_id);
    let events_json = board_peek_buffer_coalesced(surface_id);
    let retire_events = events_json.is_some();
    write_board_camera_flat(&mut reservation, controller_id, camera)?;
    if let Some(events_json) = events_json.as_deref() {
        write_board_events_flat(&mut reservation, controller_id, events_json)?;
    }
    reservation.publish_partial_with_checked(|| {
        let committed = with_board_host_mut(surface_id, |host| host.commit_wheel(&plan)).unwrap_or(false);
        if committed && retire_events {
            ENGINE_SURFACES.with(|cell| {
                if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
                    let pending = std::mem::take(&mut entry.board_pending_events);
                    entry.board_retiring_events = Some(pending);
                }
            });
        }
        committed
    })?;
    Ok(true)
}


//#endregion Board2d

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"]
mod board2d_engine_tests;

//#region TextEditor





pub fn text_editor_wheel_into(scene: &UiComponentSceneNode, delta: f32) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return false;
        };
        let Some(host) = entry.editor.as_mut() else {
            return false;
        };
        host.wheel_scroll_screen(delta as f64);
        true
    })
}









fn emit_text_editor_actions(
    scene: &UiComponentSceneNode,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    projected_document_bytes: impl FnOnce(&EditorHost) -> usize,
    mutate: impl FnOnce(&mut EditorHost),
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Ok(false);
        };
        let Some(host) = entry.editor.as_mut() else {
            return Ok(false);
        };
        let projected_bytes = projected_document_bytes(host);
        if projected_bytes > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        let selection_bytes = 17usize.checked_add(2 * decimal_digits(projected_bytes)).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let batch_bytes = text_editor_pair_bytes(scene, projected_bytes, selection_bytes)?;
        let mut batch = input.reserve_actions(2, batch_bytes)?;
        mutate(host);
        write_text_editor_action_pair(&mut batch, scene, host)?;
        batch.publish()?;
        Ok(true)
    })
}

fn write_text_editor_action_pair(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, scene: &UiComponentSceneNode, host: &EditorHost) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let selection = format!("{{\"start\":{},\"end\":{}}}", host.anchor(), host.caret());
    let select_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "textSelect", "surfaceId", &scene.surface_id, "selectionJson", &selection])?;
    batch.action(&scene.controller_id, "textSelect", select_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.string(Some("selectionJson"), &selection)?;
        builder.end_container()
    })?;
    let edit_bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "textEdit", "surfaceId", &scene.surface_id, "document", host.text()])?;
    batch.action(&scene.controller_id, "textEdit", edit_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.string(Some("document"), host.text())?;
        builder.end_container()
    })
}

fn decimal_digits(value: usize) -> usize {
    if value == 0 {
        1
    } else {
        value.ilog10() as usize + 1
    }
}

fn text_editor_pair_bytes(scene: &UiComponentSceneNode, document_bytes: usize, selection_bytes: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    let fixed = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "textSelect", "surfaceId", &scene.surface_id, "selectionJson", &scene.controller_id, "textEdit", "surfaceId", &scene.surface_id, "document"])?;
    fixed.checked_add(document_bytes).and_then(|bytes| bytes.checked_add(selection_bytes)).filter(|bytes| *bytes <= ui_wgpu::wgpu::action::ACTION_QUEUE_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
}

pub fn text_editor_apply_key_into(scene: &UiComponentSceneNode, key: &KeyAction, modifiers: &PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let supported = match key {
        KeyAction::Char(_) if !(modifiers.meta || modifiers.ctrl) => true,
        KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("a") => true,
        KeyAction::Backspace | KeyAction::Delete => true,
        _ => false,
    };
    if !supported {
        return Ok(false);
    }
    emit_text_editor_actions(
        scene,
        input,
        |host| {
            let text = host.text();
            let start = host.anchor().min(host.caret()).min(text.len());
            let end = host.anchor().max(host.caret()).min(text.len());
            match key {
                KeyAction::Char(ch) if !(modifiers.meta || modifiers.ctrl) => text.len().saturating_sub(end - start).saturating_add(ch.len()),
                KeyAction::Backspace if start != end => text.len().saturating_sub(end - start),
                KeyAction::Backspace => text[..start].chars().next_back().map_or(text.len(), |ch| text.len().saturating_sub(ch.len_utf8())),
                KeyAction::Delete if start != end => text.len().saturating_sub(end - start),
                KeyAction::Delete => text[end..].chars().next().map_or(text.len(), |ch| text.len().saturating_sub(ch.len_utf8())),
                _ => text.len(),
            }
        },
        |host| match key {
            KeyAction::Char(ch) if !(modifiers.meta || modifiers.ctrl) => host.insert_text(ch),
            KeyAction::Backspace => host.backspace(),
            KeyAction::Delete => host.delete_forward(),
            KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("a") => host.select_all(),
            _ => {}
        },
    )
}

pub fn text_editor_select_span_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    emit_text_editor_actions(scene, input, |host| host.text().len(), |host| host.select_span_at_screen(sx, sy))
}

pub fn text_editor_pointer_button_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, down: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    emit_text_editor_actions(
        scene,
        input,
        |host| host.text().len(),
        |host| {
            if down {
                host.pointer_down_screen(sx, sy, button as i32);
            } else {
                host.pointer_up_screen(sx, sy, button as i32);
            }
        },
    )
}

pub fn text_editor_pointer_move_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    emit_text_editor_actions(scene, input, |host| host.text().len(), |host| host.pointer_move_screen(sx, sy, 0))
}

pub fn text_editor_set_selection_into(scene: &UiComponentSceneNode, anchor: usize, caret: usize, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    emit_text_editor_actions(scene, input, |host| host.text().len(), |host| host.set_selection_range(anchor, caret))
}

pub fn text_editor_apply_completion_into(scene: &UiComponentSceneNode, prefix_start: usize, caret: usize, insert_text: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    emit_text_editor_actions(
        scene,
        input,
        |host| host.text().len().saturating_sub(caret.saturating_sub(prefix_start)).saturating_add(insert_text.len()),
        |host| {
            host.set_selection_range(prefix_start, caret);
            host.replace_selection(insert_text);
        },
    )
}

pub fn text_editor_pointer_click_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.surface_id) else {
            return Ok(false);
        };
        let Some(host) = entry.editor.as_mut() else {
            return Ok(false);
        };
        if host.text().len() > ui_wgpu::wgpu::action::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        let selection_bytes = 17usize.checked_add(2 * decimal_digits(host.text().len())).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let pair_bytes = text_editor_pair_bytes(scene, host.text().len(), selection_bytes)?;
        let mut batch = input.reserve_actions(4, pair_bytes.checked_mul(2).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?)?;
        host.pointer_down_screen(sx, sy, button as i32);
        write_text_editor_action_pair(&mut batch, scene, host)?;
        host.pointer_up_screen(sx, sy, 0);
        write_text_editor_action_pair(&mut batch, scene, host)?;
        batch.publish()?;
        Ok(true)
    })
}

//#region 🔖️ScenesInteropAdditions
// 🧭️ WGPU-RENDERER-FULL-PARITY (2026-07): five narrow additive wrappers, each a one-line delegation
// mirroring `text_editor_pointer_down`/`_move`/`_up` immediately above — `scenes::TextEditor`'s
// `render_text_editor` needs them to reach `EditorHost` capabilities (double-click word-select, explicit
// selection ranges, completion commit, and caret read-back for popup placement) that already exist on
// `EditorHost` (`framework/editor/rs`) but weren't exposed past this module boundary. No existing
// signature changed; nothing removed.

/// 🖱️ Double-click-to-select-word: delegates to `EditorHost::select_span_at_screen` (same screen-space
/// hit-testing as `text_editor_pointer_down`), mirroring `WasmEditorSurface`'s `session.selectSpanAtScreen`
/// (`framework/renderer/react/components/text-editor-host.tsx`). Also reused by the context menu's
/// "Select Token" action at the original right-click point.


/// 🎯️ Sets an explicit byte-offset selection range (anchor, caret) — used by the "Select Line" context-menu
/// action, whose range is computed from the buffer text rather than a screen point.


/// ✅️ Commits a completion: replaces `[prefix_start, caret)` with `insert_text`, mirroring
/// `WasmEditorSurface.applyCompletion` (`setSelectionRange` + `replaceSelection`).


/// 🔎️ Read-only `(anchor, caret)` byte-offset accessor — lets `scenes::TextEditor` compute the
/// completion-prefix boundary without duplicating `EditorHost`'s own state.
pub fn text_editor_caret(scene: &UiComponentSceneNode) -> (usize, usize) {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(entry) = map.get(&scene.surface_id) else {
            return (0, 0);
        };
        let Some(host) = entry.editor.as_ref() else {
            return (0, 0);
        };
        (host.anchor(), host.caret())
    })
}

/// 📍️ Screen-space caret position (surface-local, i.e. already offset by `inner.x/y`), for placing the
/// completions dropdown and the rename input near the caret — mirrors `WasmEditorSurface.caretScreenPosition`
/// (`caretWorldJson` + `worldToScreenJson`).
pub fn text_editor_caret_screen(scene: &UiComponentSceneNode, inner: Rect) -> Option<(f32, f32)> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(&scene.surface_id)?;
        let host = entry.editor.as_ref()?;
        let world: Value = serde_json::from_str(&host.caret_world_json()).ok()?;
        let wx = world.get("x")?.as_f64()?;
        let wy = world.get("y")?.as_f64()?;
        let screen: Value = serde_json::from_str(&host.world_to_screen_json(wx, wy)).ok()?;
        let sx = screen.get("x")?.as_f64()? as f32;
        let sy = screen.get("y")?.as_f64()? as f32;
        Some((inner.x + sx, inner.y + sy))
    })
}
//#endregion 🔖️ScenesInteropAdditions
//#endregion TextEditor
