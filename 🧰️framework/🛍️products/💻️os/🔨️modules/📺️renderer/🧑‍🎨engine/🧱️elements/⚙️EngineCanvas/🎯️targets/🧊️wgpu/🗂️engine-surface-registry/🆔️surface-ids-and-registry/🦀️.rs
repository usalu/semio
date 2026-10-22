//! 🆔️ Surface ids and registry: the node graph engine and sync cache, the engine surface, the close page helpers, the surface id, token, identity, snapshot and freshness and the surface registry. See [`super`].

use super::*;

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
    pub interaction_domain: Option<Arc<semio_framework_ui_declaration::ui::NodeGraphInteractionDomain>>,
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
    pub board_pointer_claim: Option<semio_framework_ui_action::BoundedActionClaim>,
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
    pub(super) last_note_click: Option<(String, f64)>,
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
pub(super) fn retire_bytes_page(bytes: &mut Vec<u8>) -> bool {
    if bytes.is_empty() {
        return false;
    }
    bytes.truncate(bytes.len().saturating_sub(ENGINE_CLOSE_PAGE_BYTES));
    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineSurfaceId {
    pub(in super::super) bytes: [u8; ENGINE_SURFACE_ID_BYTE_CAPACITY],
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
    pub exhausted: bool,
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
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| EngineSurfaceSlot { id: None, generation: 0, exhausted: false, value: None, retirement: None }), next_text_editor_receipt: 1, text_editor_outbox_cursor: 0, faulted: false }
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn token(&self, id: &str) -> Option<EngineSurfaceToken> {
        let index = self.slot_index(id)?;
        Some(EngineSurfaceToken { slot: index as u16, generation: self.slots[index].generation })
    }

    pub fn identity(&self, id: &str) -> Option<EngineSurfaceIdentity> {
        let index = self.slot_index(id)?;
        Some(EngineSurfaceIdentity { token: EngineSurfaceToken { slot: index as u16, generation: self.slots[index].generation }, id: self.slots[index].id? })
    }

    pub(super) fn identity_for_token(&self, token: EngineSurfaceToken) -> Option<EngineSurfaceIdentity> {
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

    #[cfg(any(test, feature = "testkit"))]
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

    pub fn close_step(&mut self, token: EngineSurfaceToken, context: &mut semio_framework_job::StepContext<'_>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
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
//#endregion Registry
