//! 🗃️ Scene runtime state: admitted surface map, viewport, drag and ink-edit state, host temporal presentation, accessibility controls, surface retirement and canvas gestures.

use super::*;

//#region SceneRuntime
pub const SCENE_SURFACE_CAPACITY: usize = 256;
pub const SCENE_SURFACE_ID_BYTE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedSurfaceToken {
    pub(crate) slot: u16,
    pub(crate) epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmittedSurfaceFault {
    IdCapacity,
    ItemCapacity,
    ReplacementPending,
    RejectedPending,
    Closing,
}

#[derive(Debug)]
pub struct AdmittedSurfaceRejected<T> {
    pub fault: AdmittedSurfaceFault,
    pub id: String,
    pub value: T,
}

pub struct AdmittedSurfaceCloseOwner<T> {
    pub id: String,
    pub value: T,
}

struct AdmittedSurfaceEntry<T> {
    pub(crate) id: String,
    pub(crate) epoch: u64,
    pub(crate) value: T,
}

pub struct AdmittedSurfaceMap<T> {
    pub(crate) slots: Box<[Option<AdmittedSurfaceEntry<T>>; SCENE_SURFACE_CAPACITY]>,
    pub(crate) epochs: [u64; SCENE_SURFACE_CAPACITY],
    pub(crate) order: [Option<u16>; SCENE_SURFACE_CAPACITY],
    order_len: usize,
    pub(crate) external_reservations: usize,
    pub(crate) fault: Option<&'static str>,
    pub(crate) rejected: Option<Box<AdmittedSurfaceRejected<T>>>,
    retired: Option<Box<AdmittedSurfaceCloseOwner<T>>>,
    pub(crate) closing: bool,
}

impl<T> Default for AdmittedSurfaceMap<T> {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; SCENE_SURFACE_CAPACITY], order: [None; SCENE_SURFACE_CAPACITY], order_len: 0, external_reservations: 0, fault: None, rejected: None, retired: None, closing: false }
    }
}

impl<T> AdmittedSurfaceMap<T> {
    fn existing_slot(&self, id: &str) -> Option<usize> {
        self.slots.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.id == id))
    }

    fn admit_slot(&mut self, id: &str) -> Result<usize, AdmittedSurfaceFault> {
        if self.closing {
            return Err(AdmittedSurfaceFault::Closing);
        }
        if let Some(slot) = self.existing_slot(id) {
            return Ok(slot);
        }
        if id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            self.fault = Some("scene surface identifier exceeded fixed credits");
            return Err(AdmittedSurfaceFault::IdCapacity);
        }
        if self.order_len.checked_add(self.external_reservations).map_or(true, |owners| owners >= SCENE_SURFACE_CAPACITY) {
            self.fault = Some("scene surface item credits exceeded");
            return Err(AdmittedSurfaceFault::ItemCapacity);
        }
        let slot = self.slots.iter().position(Option::is_none).expect("surface order credits imply one free fixed slot");
        self.order[self.order_len] = Some(slot as u16);
        self.order_len += 1;
        Ok(slot)
    }

    pub fn try_insert(&mut self, id: String, value: T) -> Result<AdmittedSurfaceToken, AdmittedSurfaceRejected<T>> {
        let slot = match self.admit_slot(&id) {
            Ok(slot) => slot,
            Err(fault) => return Err(AdmittedSurfaceRejected { fault, id, value }),
        };
        if let Some(entry) = self.slots[slot].as_mut() {
            if self.retired.is_some() {
                return Err(AdmittedSurfaceRejected { fault: AdmittedSurfaceFault::ReplacementPending, id, value });
            }
            self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
            entry.epoch = self.epochs[slot];
            let previous = std::mem::replace(&mut entry.value, value);
            self.retired = Some(Box::new(AdmittedSurfaceCloseOwner { id, value: previous }));
            return Ok(AdmittedSurfaceToken { slot: slot as u16, epoch: entry.epoch });
        }
        self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
        self.slots[slot] = Some(AdmittedSurfaceEntry { id, epoch: self.epochs[slot], value });
        Ok(AdmittedSurfaceToken { slot: slot as u16, epoch: self.epochs[slot] })
    }

    pub fn retain_rejected(&mut self, rejected: AdmittedSurfaceRejected<T>) -> Result<(), AdmittedSurfaceRejected<T>> {
        if self.rejected.is_some() {
            return Err(AdmittedSurfaceRejected { fault: AdmittedSurfaceFault::RejectedPending, ..rejected });
        }
        self.rejected = Some(Box::new(rejected));
        Ok(())
    }

    pub fn retain_first_rejected(&mut self, rejected: AdmittedSurfaceRejected<T>) {
        assert!(self.rejected.is_none(), "surface producer must stop while one exact rejected owner is retained");
        self.rejected = Some(Box::new(rejected));
    }

    pub fn admission_blocked(&self) -> bool {
        self.closing || self.rejected.is_some() || self.retired.is_some()
    }

    /// 🚦️ Reserves the fixed scene-owner grants for owners draining outside this map.
    pub fn set_external_reservations(&mut self, reservations: usize) -> bool {
        if reservations > SCENE_SURFACE_CAPACITY.saturating_sub(self.order_len) {
            self.record_fault("scene surface external reservations exceeded item credits");
            return false;
        }
        self.external_reservations = reservations;
        true
    }

    pub fn get_or_insert_with(&mut self, id: String, create: impl FnOnce() -> T) -> Option<&mut T> {
        let slot = self.admit_slot(&id).ok()?;
        if self.slots[slot].is_none() {
            self.epochs[slot] = self.epochs[slot].wrapping_add(1).max(1);
            self.slots[slot] = Some(AdmittedSurfaceEntry { id, epoch: self.epochs[slot], value: create() });
        }
        self.slots[slot].as_mut().map(|entry| &mut entry.value)
    }

    pub fn id_at(&self, index: usize) -> Option<&str> {
        let slot = usize::from(self.order.get(index).copied().flatten()?);
        self.slots.get(slot).and_then(Option::as_ref).map(|entry| entry.id.as_str())
    }

    pub fn get_mut_at(&mut self,index:usize)->Option<&mut T>{
        let slot=usize::from(self.order.get(index).copied().flatten()?);
        self.slots.get_mut(slot).and_then(Option::as_mut).map(|entry|&mut entry.value)
    }

    pub fn len(&self) -> usize {
        self.order_len
    }

    pub fn is_empty(&self) -> bool {
        self.order_len == 0
    }

    pub fn contains_key(&self, id: &str) -> bool {
        self.existing_slot(id).is_some()
    }

    pub fn token(&self, id: &str) -> Option<AdmittedSurfaceToken> {
        let slot = self.existing_slot(id)?;
        Some(AdmittedSurfaceToken { slot: slot as u16, epoch: self.slots[slot].as_ref()?.epoch })
    }

    pub fn get_token(&self, token: AdmittedSurfaceToken) -> Option<&T> {
        self.slots.get(usize::from(token.slot)).and_then(Option::as_ref).filter(|entry| entry.epoch == token.epoch).map(|entry| &entry.value)
    }

    pub fn get_token_mut(&mut self, token: AdmittedSurfaceToken) -> Option<&mut T> {
        self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|entry| entry.epoch == token.epoch).map(|entry| &mut entry.value)
    }

    pub fn get(&self, id: &str) -> Option<&T> {
        self.existing_slot(id).and_then(|slot| self.slots[slot].as_ref().map(|entry| &entry.value))
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        let slot = self.existing_slot(id)?;
        self.slots[slot].as_mut().map(|entry| &mut entry.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.order[..self.order_len].iter().filter_map(|slot| {
            let entry = self.slots[usize::from((*slot).expect("admitted surface order slot"))].as_ref()?;
            Some((&entry.id, &entry.value))
        })
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.iter().map(|(id, _)| id)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.slots.iter().flatten().map(|entry| &entry.value)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut T)> {
        self.slots.iter_mut().flatten().map(|entry| (&entry.id, &mut entry.value))
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.slots.iter_mut().flatten().map(|entry| &mut entry.value)
    }

    pub fn remove(&mut self, id: &str) -> Option<T> {
        let slot = self.existing_slot(id)?;
        let Some(index) = (0..self.order_len).find(|index| self.order[*index] == Some(slot as u16)) else {
            self.fault = Some("scene surface order lost ownership");
            return self.slots[slot].take().map(|entry| entry.value);
        };
        let value = self.slots[slot].take().map(|entry| entry.value);
        for cursor in index..self.order_len - 1 {
            self.order[cursor] = self.order[cursor + 1];
        }
        self.order_len -= 1;
        self.order[self.order_len] = None;
        value
    }

    pub fn take_exact_to_retirement(&mut self, token: AdmittedSurfaceToken) -> Option<AdmittedSurfaceCloseOwner<T>> {
        let slot = usize::from(token.slot);
        let entry = self.slots.get(slot)?.as_ref()?;
        if entry.epoch != token.epoch {
            return None;
        }
        let index = (0..self.order_len).find(|index| self.order[*index] == Some(token.slot))?;
        let entry = self.slots[slot].take()?;
        for cursor in index..self.order_len - 1 {
            self.order[cursor] = self.order[cursor + 1];
        }
        self.order_len -= 1;
        self.order[self.order_len] = None;
        self.external_reservations += 1;
        Some(AdmittedSurfaceCloseOwner { id: entry.id, value: entry.value })
    }

    pub fn acknowledge_retired_owner(&mut self) {
        self.external_reservations = self.external_reservations.checked_sub(1).expect("one admitted owner is awaiting retirement");
    }

    pub fn clear(&mut self) {
        self.closing = true;
        self.record_fault("scene surface clear requires retained close pumping");
    }

    pub fn take_fault(&mut self) -> Option<&'static str> {
        self.fault.take()
    }

    pub fn record_fault(&mut self, fault: &'static str) {
        if self.fault.is_none() {
            self.fault = Some(fault);
        }
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> Option<AdmittedSurfaceCloseOwner<T>> {
        if let Some(rejected) = self.rejected.take() {
            return Some(AdmittedSurfaceCloseOwner { id: rejected.id, value: rejected.value });
        }
        if let Some(retired) = self.retired.take() {
            return Some(*retired);
        }
        if self.order_len == 0 {
            return None;
        }
        self.order_len -= 1;
        let slot = usize::from(self.order[self.order_len].take().expect("admitted close order slot"));
        self.slots[slot].take().map(|entry| AdmittedSurfaceCloseOwner { id: entry.id, value: entry.value })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.order_len == 0 && self.external_reservations == 0 && self.slots.iter().all(Option::is_none) && self.rejected.is_none() && self.retired.is_none()
    }
}

impl<'a, T> IntoIterator for &'a AdmittedSurfaceMap<T> {
    type Item = (&'a String, &'a T);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs"]
mod admitted_surface_map_tests;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Viewport {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) zoom: f32,
}

impl Viewport {
    pub(crate) fn from_json(raw: &str) -> Self {
        serde_json::from_str::<Value>(raw)
            .ok()
            .map(|value| Self { x: value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, y: value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, zoom: value.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32 })
            .unwrap_or_default()
    }

    fn from_typed(viewport: Option<&semio_framework_os_kernel::Viewport2d>) -> Self {
        viewport.map(|viewport| Self { x: viewport.x as f32, y: viewport.y as f32, zoom: viewport.zoom as f32 }).unwrap_or(Self { x: 0.0, y: 0.0, zoom: 1.0 })
    }

    pub(crate) fn screen_to_world(&self, sx: f32, sy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        ((sx - cx) / self.zoom + self.x, (sy - cy) / self.zoom + self.y)
    }

    pub(crate) fn world_to_screen(&self, wx: f32, wy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        (cx + (wx - self.x) * self.zoom, cy + (wy - self.y) * self.zoom)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SceneDragMode {
    PanViewport,
    MapMarquee { start_x: f32, start_y: f32, method: String, merge_mode: String },
    MapPan,
    InkPan { start_x: f32, start_y: f32, camera_x: f64, camera_y: f64, zoom: f64 },
    InkMove { origins: BTreeMap<String, (f64, f64)>, start_x: f32, start_y: f32 },
    InkResize { handle: String, from: InkBoundsF, start_x: f32, start_y: f32, selected_ids: Vec<String> },
    InkStroke { block_id: String },
    InkEraser { mode: String },
    InkMarqueeDrag { start_x: f32, start_y: f32 },
}

#[derive(Clone, Debug)]
pub(crate) struct SceneDrag {
    pub(crate) mode: SceneDragMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InkEditKind {
    Text,
    Table { row: usize, col: usize },
}

#[derive(Clone, Debug)]
pub(crate) struct InkEditState {
    pub(crate) block: Value,
    pub(crate) kind: InkEditKind,
    pub(crate) draft: String,
    pub(crate) replace_on_input: bool,
    pub(crate) screen_rect: Rect,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct HostTemporalPresentation {
    pub(crate) request: Option<ui_contract::HostTemporalFormatRequestV1>,
    request_generation: u64,
    pending_generation: Option<u64>,
    pub(crate) candidate: Option<ui_contract::HostTemporalFormatReplyV1>,
    pub(crate) candidate_epoch: Option<u64>,
    pub(crate) queued_candidate: Option<ui_contract::HostTemporalFormatReplyV1>,
    pub(crate) accepted: Option<ui_contract::HostTemporalFormatReplyV1>,
    pub(crate) last_request_ms: f64,
}

fn host_temporal_requests_match(left: &ui_contract::HostTemporalFormatRequestV1, right: &ui_contract::HostTemporalFormatRequestV1) -> bool {
    left.values == right.values && (left.now_ms == right.now_ms || !left.values.iter().any(|value| value.format == ui_contract::HostTemporalFormatV1::Relative))
}

impl HostTemporalPresentation {
    pub(crate) fn observe(&mut self, request: &ui_contract::HostTemporalFormatRequestV1) -> Option<u64> {
        let relative = request.values.iter().any(|value| value.format == ui_contract::HostTemporalFormatV1::Relative);
        let values_changed = self.request.as_ref().is_none_or(|current| current.values != request.values);
        if values_changed {
            self.request = Some(request.clone());
            self.request_generation = self.request_generation.wrapping_add(1).max(1);
            self.pending_generation = None;
            self.candidate = None;
            self.candidate_epoch = None;
            self.queued_candidate = None;
            self.accepted = None;
            self.last_request_ms = 0.0;
        } else if relative && self.request.as_ref().is_some_and(|current| current.now_ms.abs_diff(request.now_ms) >= 1_000) && self.pending_generation.is_none() && self.candidate.is_none() && self.queued_candidate.is_none() {
            self.request = Some(request.clone());
        }
        if relative && self.request.as_ref().is_some_and(|current| current.now_ms != request.now_ms) {
            return None;
        }
        if request.values.is_empty() || self.pending_generation.is_some() || self.candidate.is_some() || self.queued_candidate.is_some() {
            return None;
        }
        let now = crate::app_now_ms();
        if self.last_request_ms > 0.0 && now - self.last_request_ms < 1_000.0 {
            return None;
        }
        if self.accepted.is_some() && now - self.last_request_ms < if relative { 1_000.0 } else { 60_000.0 } {
            return None;
        }
        self.request_generation = self.request_generation.wrapping_add(1).max(1);
        self.pending_generation = Some(self.request_generation);
        self.last_request_ms = now;
        Some(self.request_generation)
    }

    fn latest_profile_revision(&self) -> u64 {
        self.accepted.iter().chain(self.candidate.iter()).chain(self.queued_candidate.iter()).map(|reply| reply.profile.profile_revision).max().unwrap_or(0)
    }

    pub(crate) fn publish(&mut self, generation: u64, request: &ui_contract::HostTemporalFormatRequestV1, reply: ui_contract::HostTemporalFormatReplyV1) -> bool {
        if self.pending_generation != Some(generation) || self.request.as_ref().is_none_or(|current| !host_temporal_requests_match(current, request)) {
            return false;
        }
        self.pending_generation = None;
        if !reply.matches(request) || reply.profile.profile_revision < self.latest_profile_revision() {
            return false;
        }
        if self.candidate_epoch.is_some() {
            self.queued_candidate = Some(reply);
        } else {
            self.candidate = Some(reply);
        }
        true
    }

    pub(crate) fn refuse(&mut self, generation: u64) {
        if self.pending_generation == Some(generation) {
            self.pending_generation = None;
        }
    }

    pub(crate) fn visible(&self) -> Option<&ui_contract::HostTemporalFormatReplyV1> {
        self.candidate.as_ref().or(self.accepted.as_ref())
    }

    pub(crate) fn seal(&mut self, epoch: u64) {
        if self.candidate.is_some() && self.candidate_epoch.is_none() {
            self.candidate_epoch = Some(epoch);
        }
    }

    pub(crate) fn acknowledge(&mut self, epoch: u64) {
        if self.candidate_epoch != Some(epoch) {
            return;
        }
        self.accepted = self.candidate.take();
        self.candidate = self.queued_candidate.take();
        self.candidate_epoch = None;
    }

    pub(crate) fn discard(&mut self, epoch: u64) {
        if self.candidate_epoch == Some(epoch) {
            self.candidate_epoch = None;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VfsAccessibilityControlKind {
    Row,
    Chevron,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VfsAccessibilityControl {
    pub(crate) key: String,
    pub(crate) row_id: String,
    pub(crate) label: String,
    pub(crate) kind: VfsAccessibilityControlKind,
    pub(crate) rect: Rect,
    pub(crate) expanded: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BlockListAccessibilityControl {
    pub(crate) key: String,
    pub(crate) label: String,
    pub(crate) rect: Rect,
    pub(crate) action: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EventFeedAccessibilityControl {
    pub(crate) key: String,
    pub(crate) entry_id: String,
    pub(crate) label: String,
    pub(crate) rect: Rect,
    pub(crate) action: Option<ActionDescriptor>,
}

/// ♿️ One node of a world surface's modelling-overlay accessibility tree (annotation list, scalar-field legend, mismatch status), staged by the paint and published with the frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct World3dAccessibilityControl {
    pub(crate) key: String,
    pub(crate) role: &'static str,
    pub(crate) label: String,
    pub(crate) depth: u8,
    pub(crate) polite: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GraphTimelineAccessibilityControl {
    pub(crate) key: String,
    pub(crate) checkpoint_id: String,
    pub(crate) label: String,
    pub(crate) rect: Rect,
    pub(crate) action: ActionDescriptor,
}

#[derive(Clone, Debug)]
struct SceneAccessibilityPresentation<T> {
    pub(crate) candidate: Option<Vec<T>>,
    pub(crate) candidate_epoch: Option<u64>,
    pub(crate) queued_candidate: Option<Vec<T>>,
    pub(crate) accepted: Vec<T>,
}

impl<T> Default for SceneAccessibilityPresentation<T> {
    fn default() -> Self {
        Self { candidate: None, candidate_epoch: None, queued_candidate: None, accepted: Vec::new() }
    }
}

impl<T> SceneAccessibilityPresentation<T> {
    pub(crate) fn stage(&mut self, controls: Vec<T>) {
        if self.candidate_epoch.is_some() {
            self.queued_candidate = Some(controls);
        } else {
            self.candidate = Some(controls);
        }
    }

    pub(crate) fn seal(&mut self, epoch: u64) {
        if self.candidate.is_some() && self.candidate_epoch.is_none() {
            self.candidate_epoch = Some(epoch);
        }
    }

    pub(crate) fn acknowledge(&mut self, epoch: u64) {
        if self.candidate_epoch != Some(epoch) {
            return;
        }
        self.accepted = self.candidate.take().unwrap_or_default();
        self.candidate = self.queued_candidate.take();
        self.candidate_epoch = None;
    }

    pub(crate) fn discard(&mut self, epoch: u64) {
        if self.candidate_epoch == Some(epoch) {
            self.candidate = self.queued_candidate.take();
            self.candidate_epoch = None;
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SceneSurfaceState {
    pub(crate) mount_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub(crate) engine_token: Option<engine_canvas::EngineSurfaceToken>,
    pub(crate) scroll_offsets: BTreeMap<String, f32>,
    pub(crate) viewport: Viewport,
    pub(crate) canvas_camera_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub(crate) canvas_framing_revision: Option<u32>,
    pub(crate) drag: Option<SceneDrag>,
    pub(crate) pointer_was_down: bool,
    pub(crate) last_click_ms: f64,
    pub(crate) last_click_target: Option<String>,
    /// 🖱️ The list-surface control the pointer last resolved over — this module's own hover
    /// authority, because a `ComponentScene` leaf registers exactly one retained hit target for the
    /// WHOLE surface (`retained_scene_hit`), so `InputState::hovered_id` can never name a row.
    pub(crate) hovered_control_id: Option<String>,
    #[cfg(test)]
    node_positions: BTreeMap<String, (f32, f32)>,
    pub(crate) selected_ids: BTreeSet<String>,
    pub(crate) paint_stroke_active: bool,
    pub(crate) vfs_expanded_ids: BTreeSet<String>,
    pub(crate) vfs_selection_anchor: Option<String>,
    pub(crate) map_marquee_points: Vec<(f32, f32)>,
    pub(crate) map_marquee_active: bool,
    pub(crate) map_last_hover_json: Option<String>,
    pub(crate) map_interaction_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub(crate) map_hover_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub(crate) touch_gesture: GestureRecognizer,
    pub(crate) ink_camera: Option<(f64, f64, f64)>,
    pub(crate) ink_overrides: BTreeMap<String, Value>,
    /// 🌊️ A move, resize, stroke or erase has streamed into the plugin's open ink transaction since the press — what a
    /// pointer cancel must abort.
    pub(crate) ink_gesture_open: bool,
    pub(crate) ink_marquee_points: Vec<(f32, f32)>,
    pub(crate) ink_edit: Option<InkEditState>,
    pub(crate) text_editor_ui: TextEditorUiState,
    pub(crate) host_temporal: HostTemporalPresentation,
    pub(crate) table_accessibility: SceneAccessibilityPresentation<TableStepperAccessibilityCell>,
    pub(crate) table_editable_accessibility: SceneAccessibilityPresentation<TableEditableTextAccessibilityCell>,
    pub(crate) table_button_accessibility: SceneAccessibilityPresentation<TableButtonAccessibilityCell>,
    pub(crate) vfs_accessibility: SceneAccessibilityPresentation<VfsAccessibilityControl>,
    pub(crate) block_list_accessibility: SceneAccessibilityPresentation<BlockListAccessibilityControl>,
    pub(crate) event_feed_accessibility: SceneAccessibilityPresentation<EventFeedAccessibilityControl>,
    pub(crate) world3d_accessibility: SceneAccessibilityPresentation<World3dAccessibilityControl>,
    pub(crate) graph_timeline_accessibility: SceneAccessibilityPresentation<GraphTimelineAccessibilityControl>,
    /// 📏️ The label track this surface's last GraphTimeline paint measured — the one geometry its hit test reads.
    pub(crate) graph_timeline_label_track: f32,
    pub(crate) last_pointer_pos: (f32, f32),
    /// 🕒️ Controller for the mounted surface's settled camera publication.
    pub(crate) camera_dispatch_controller_id: Option<String>,
    pub(crate) canvas_click: Option<CanvasClick>,
}

pub(crate) enum SceneValueRetirement {
    Value(Value),
    Array(Vec<Value>),
    Object(serde_json::map::IntoIter),
    Text(String),
}

pub(crate) struct SceneSurfaceRetirement {
    pub(crate) host_id: Box<str>,
    pub(crate) owner: AdmittedSurfaceCloseOwner<SceneSurfaceState>,
    pub(crate) external_target: Option<crate::element_contracts::ScenePointerTarget>,
    pub(crate) external_engine_token: Option<engine_canvas::EngineSurfaceToken>,
    pub(crate) external_close: Option<crate::os_host::ComponentSurfaceCloseToken>,
    pub(crate) text: String,
    pub(crate) values: Vec<SceneValueRetirement>,
    pub(crate) gesture: Option<CanvasGesture>,
    pub(crate) hover: Option<CanvasCatalogueHover>,
    pub(crate) raster: Option<PendingRasterSurfaceRetirement>,
    pub(crate) raster_started: bool,
    pub(crate) transfer: Option<SceneListTransferSession>,
}

fn retire_scene_text(text: &mut String) -> bool {
    if !text.is_empty() {
        let mut end = text.len().saturating_sub(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        while !text.is_char_boundary(end) {
            end += 1;
        }
        text.truncate(end);
        return true;
    }
    if text.capacity() != 0 {
        *text = String::new();
        return true;
    }
    false
}

fn retire_scene_target(target: &mut Option<crate::element_contracts::ScenePointerTarget>) -> bool {
    let Some(owner) = target.as_mut() else { return false };
    if retire_scene_text(&mut owner.host_id) || retire_scene_text(&mut owner.window_id) || retire_scene_text(&mut owner.surface_id) {
        return true;
    }
    if let ui_wgpu::wgpu::NodeKey::Explicit(key) = &mut owner.key {
        if retire_scene_text(key) {
            return true;
        }
    }
    *target = None;
    true
}

fn retire_host_temporal_reply(reply: &mut Option<ui_contract::HostTemporalFormatReplyV1>, text: &mut String) -> bool {
    let Some(owner) = reply.as_mut() else { return false };
    if let Some(label) = owner.labels.pop() {
        *text = label.text;
        return true;
    }
    if owner.labels.capacity() != 0 {
        owner.labels = Vec::new();
        return true;
    }
    if retire_scene_text(&mut owner.profile.locale) || retire_scene_text(&mut owner.profile.time_zone) {
        return true;
    }
    *reply = None;
    true
}

impl SceneSurfaceRetirement {
    pub(crate) fn close_step(&mut self) -> bool {
        if !self.raster_started {
            let taken = PENDING_RASTER_STATE.with(|cell| {
                let mut surfaces = cell.borrow_mut();
                let Some(token) = surfaces.token(&self.host_id) else { return Some(None) };
                if surfaces.get_token(token).is_some_and(|surface| surface.queue.checked_out.is_some()) {
                    return None;
                }
                Some(surfaces.take_exact_to_retirement(token))
            });
            let Some(taken) = taken else { return false };
            self.raster = taken.map(PendingRasterSurfaceRetirement::new);
            self.raster_started = true;
            return false;
        }
        if let Some(raster) = self.raster.as_mut() {
            if !raster.close_step() {
                return false;
            }
            assert!(raster.terminal_is_empty(), "component raster owner must be empty before releasing its grant");
            self.raster = None;
            PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().acknowledge_retired_owner());
            return false;
        }
        if let Some(target) = self.external_target.as_ref() {
            let token = if let Some(token) = self.external_close {
                token
            } else {
                let Ok(token) = crate::os_host::request_component_surface_close(target.clone(), self.external_engine_token) else { return false };
                self.external_close = Some(token);
                return false;
            };
            if !crate::os_host::component_surface_close_terminal(token) {
                return false;
            }
            if !crate::os_host::acknowledge_component_surface_close(token) {
                return false;
            }
            self.external_close = None;
            self.external_engine_token = None;
            self.external_target = None;
            return false;
        }
        if let Some(active) = self.gesture.as_mut() {
            if retire_scene_text(&mut active.host_id) || retire_scene_text(&mut active.window_id) || retire_scene_text(&mut active.surface_id) || retire_scene_text(&mut active.controller_id) {
                return false;
            }
            self.gesture = None;
            return false;
        }
        self.gesture = CANVAS_GESTURE.with(|cell| {
            let mut gestures = cell.borrow_mut();
            gestures.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|active| active.host_id == self.host_id.as_ref())).and_then(Option::take)
        });
        if self.gesture.is_some() {
            return false;
        }
        if let Some(active) = self.hover.as_mut() {
            if retire_scene_text(&mut active.host_id) || retire_scene_text(&mut active.window_id) || retire_scene_text(&mut active.surface_id) || retire_scene_text(&mut active.controller_id) {
                return false;
            }
            self.hover = None;
            return false;
        }
        self.hover = CANVAS_CATALOGUE_HOVER.with(|cell| {
            let mut hover = cell.borrow_mut();
            if hover.as_ref().is_some_and(|active| active.host_id == self.host_id.as_ref()) {
                hover.take()
            } else {
                None
            }
        });
        if self.hover.is_some() {
            return false;
        }
        if let Some(active) = self.transfer.as_mut() {
            if retire_scene_text(&mut active.source_window_id) || retire_scene_text(&mut active.source_host_id) {
                return false;
            }
            let pending = match &mut active.source {
                SceneListTransferSource::TableRow { row_id, mime, payload } => retire_scene_text(row_id) || retire_scene_text(mime) || retire_scene_text(payload),
                SceneListTransferSource::BlockListStep { step_id, .. } => retire_scene_text(step_id),
                SceneListTransferSource::BlockListBlock { step_id, block_id, .. } => retire_scene_text(step_id) || retire_scene_text(block_id),
                SceneListTransferSource::BlockListPalette { kind, mime, payload } => retire_scene_text(kind) || retire_scene_text(mime) || retire_scene_text(payload),
            };
            if pending {
                return false;
            }
            self.transfer = None;
            return false;
        }
        self.transfer = SCENE_LIST_TRANSFER.with(|cell| {
            let mut authority = cell.borrow_mut();
            if authority.active.as_ref().is_some_and(|active| active.source_host_id == self.host_id.as_ref()) {
                authority.active.take()
            } else {
                None
            }
        });
        if self.transfer.is_some() {
            return false;
        }
        if retire_scene_text(&mut self.text) {
            return false;
        }
        if let Some(value) = self.values.pop() {
            match value {
                SceneValueRetirement::Value(Value::String(text)) => self.text = text,
                SceneValueRetirement::Value(Value::Array(values)) => self.values.push(SceneValueRetirement::Array(values)),
                SceneValueRetirement::Value(Value::Object(values)) => self.values.push(SceneValueRetirement::Object(values.into_iter())),
                SceneValueRetirement::Array(mut values) => {
                    if let Some(value) = values.pop() {
                        self.values.push(SceneValueRetirement::Array(values));
                        self.values.push(SceneValueRetirement::Value(value));
                    }
                }
                SceneValueRetirement::Object(mut values) => {
                    if let Some((key, value)) = values.next() {
                        self.values.push(SceneValueRetirement::Object(values));
                        self.values.push(SceneValueRetirement::Value(value));
                        self.values.push(SceneValueRetirement::Text(key));
                    }
                }
                SceneValueRetirement::Text(text) => self.text = text,
                SceneValueRetirement::Value(_) => {}
            }
            return false;
        }
        if self.values.capacity() != 0 {
            self.values = Vec::new();
            return false;
        }
        let state = &mut self.owner.value;
        if state.touch_gesture.retire_step() {
            return false;
        }
        for reply in [&mut state.host_temporal.candidate, &mut state.host_temporal.queued_candidate, &mut state.host_temporal.accepted] {
            if retire_host_temporal_reply(reply, &mut self.text) {
                return false;
            }
        }
        if let Some(request) = state.host_temporal.request.as_mut() {
            if let Some(value) = request.values.pop() {
                self.text = value.id;
                return false;
            }
            if request.values.capacity() != 0 {
                request.values = Vec::new();
                return false;
            }
            state.host_temporal.request = None;
            return false;
        }
        for controls in [&mut state.table_accessibility.candidate, &mut state.table_accessibility.queued_candidate] {
            if let Some(entries) = controls.as_mut() {
                if let Some(entry) = entries.pop() {
                    self.values.push(SceneValueRetirement::Text(entry.key));
                    self.values.push(SceneValueRetirement::Text(entry.label));
                    self.values.push(SceneValueRetirement::Text(entry.target.row_id));
                    self.values.push(SceneValueRetirement::Text(entry.target.column_id));
                    return false;
                }
                if entries.capacity() != 0 {
                    *entries = Vec::new();
                    return false;
                }
                *controls = None;
                return false;
            }
        }
        if let Some(entry) = state.table_accessibility.accepted.pop() {
            self.values.push(SceneValueRetirement::Text(entry.key));
            self.values.push(SceneValueRetirement::Text(entry.label));
            self.values.push(SceneValueRetirement::Text(entry.target.row_id));
            self.values.push(SceneValueRetirement::Text(entry.target.column_id));
            return false;
        }
        if state.table_accessibility.accepted.capacity() != 0 {
            state.table_accessibility.accepted = Vec::new();
            return false;
        }
        for controls in [&mut state.table_editable_accessibility.candidate, &mut state.table_editable_accessibility.queued_candidate] {
            if let Some(entries) = controls.as_mut() {
                if let Some(entry) = entries.pop() {
                    self.values.push(SceneValueRetirement::Text(entry.key));
                    self.values.push(SceneValueRetirement::Text(entry.label));
                    self.values.push(SceneValueRetirement::Text(entry.target.row_id));
                    self.values.push(SceneValueRetirement::Text(entry.target.column_id));
                    self.values.push(SceneValueRetirement::Text(entry.target.value));
                    return false;
                }
                if entries.capacity() != 0 {
                    *entries = Vec::new();
                    return false;
                }
                *controls = None;
                return false;
            }
        }
        if let Some(entry) = state.table_editable_accessibility.accepted.pop() {
            self.values.push(SceneValueRetirement::Text(entry.key));
            self.values.push(SceneValueRetirement::Text(entry.label));
            self.values.push(SceneValueRetirement::Text(entry.target.row_id));
            self.values.push(SceneValueRetirement::Text(entry.target.column_id));
            self.values.push(SceneValueRetirement::Text(entry.target.value));
            return false;
        }
        if state.table_editable_accessibility.accepted.capacity() != 0 {
            state.table_editable_accessibility.accepted = Vec::new();
            return false;
        }
        for controls in [&mut state.vfs_accessibility.candidate, &mut state.vfs_accessibility.queued_candidate] {
            if let Some(entries) = controls.as_mut() {
                if let Some(entry) = entries.pop() {
                    self.values.push(SceneValueRetirement::Text(entry.key));
                    self.values.push(SceneValueRetirement::Text(entry.row_id));
                    self.values.push(SceneValueRetirement::Text(entry.label));
                    return false;
                }
                if entries.capacity() != 0 {
                    *entries = Vec::new();
                    return false;
                }
                *controls = None;
                return false;
            }
        }
        if let Some(entry) = state.vfs_accessibility.accepted.pop() {
            self.values.push(SceneValueRetirement::Text(entry.key));
            self.values.push(SceneValueRetirement::Text(entry.row_id));
            self.values.push(SceneValueRetirement::Text(entry.label));
            return false;
        }
        if state.vfs_accessibility.accepted.capacity() != 0 {
            state.vfs_accessibility.accepted = Vec::new();
            return false;
        }
        if let Some(draft) = state.text_editor_ui.rename.as_mut() {
            if retire_scene_text(&mut draft.text) {
                return false;
            }
            if draft.occurrences.pop().is_some() {
                return false;
            }
            if draft.occurrences.capacity() != 0 {
                draft.occurrences = Vec::new();
                return false;
            }
            state.text_editor_ui.rename = None;
            return false;
        }
        if let Some((text, _, _)) = state.text_editor_ui.pending_menu_action.as_mut() {
            if retire_scene_text(text) {
                return false;
            }
            state.text_editor_ui.pending_menu_action = None;
            return false;
        }
        if let Some(id) = state.selected_ids.pop_first().or_else(|| state.vfs_expanded_ids.pop_first()) {
            self.text = id;
            return false;
        }
        if let Some((key, _)) = state.scroll_offsets.pop_first() {
            self.text = key;
            return false;
        }
        #[cfg(test)]
        if let Some((key, _)) = state.node_positions.pop_first() {
            self.text = key;
            return false;
        }
        if let Some((key, value)) = state.ink_overrides.pop_first() {
            self.text = key;
            self.values.push(SceneValueRetirement::Value(value));
            return false;
        }
        for points in [&mut state.map_marquee_points, &mut state.ink_marquee_points] {
            if points.pop().is_some() {
                return false;
            }
            if points.capacity() != 0 {
                *points = Vec::new();
                return false;
            }
        }
        if let Some(drag) = state.drag.as_mut() {
            match &mut drag.mode {
                SceneDragMode::InkMove { origins, .. } => {
                    if let Some((key, _)) = origins.pop_first() {
                        self.text = key;
                        return false;
                    }
                }
                SceneDragMode::InkResize { handle, selected_ids, .. } => {
                    if let Some(key) = selected_ids.pop() {
                        self.text = key;
                        return false;
                    }
                    if selected_ids.capacity() != 0 {
                        *selected_ids = Vec::new();
                        return false;
                    }
                    if retire_scene_text(handle) {
                        return false;
                    }
                }
                SceneDragMode::MapMarquee { method, merge_mode, .. } => {
                    if retire_scene_text(method) || retire_scene_text(merge_mode) {
                        return false;
                    }
                }
                SceneDragMode::InkStroke { block_id } => {
                    if retire_scene_text(block_id) {
                        return false;
                    }
                }
                SceneDragMode::InkEraser { mode } => {
                    if retire_scene_text(mode) {
                        return false;
                    }
                }
                _ => {}
            }
            state.drag = None;
            return false;
        }
        if let Some(edit) = state.ink_edit.as_mut() {
            if retire_scene_text(&mut edit.draft) {
                return false;
            }
            if !edit.block.is_null() {
                self.values.push(SceneValueRetirement::Value(std::mem::take(&mut edit.block)));
                return false;
            }
            state.ink_edit = None;
            return false;
        }
        for text in [&mut state.last_click_target, &mut state.hovered_control_id, &mut state.vfs_selection_anchor, &mut state.map_last_hover_json, &mut state.camera_dispatch_controller_id] {
            if let Some(value) = text.as_mut() {
                if retire_scene_text(value) {
                    return false;
                }
                *text = None;
                return false;
            }
        }
        for owner in [&mut state.mount_owner, &mut state.canvas_camera_owner, &mut state.map_interaction_owner, &mut state.map_hover_owner] {
            if retire_scene_target(owner) {
                return false;
            }
        }
        !retire_scene_text(&mut self.owner.id)
    }
}

/// 🧵️ Releases one retained scene allocation before a window or replacement mount advances.
pub(crate) fn close_retired_scene_surface_one() -> bool {
    SCENE_SURFACE_RETIREMENT.with(|cell| {
        let mut slot = cell.borrow_mut();
        let Some(owner) = slot.as_mut() else { return false };
        if owner.close_step() {
            *slot = None;
            SCENE_STATE.with(|cell| cell.borrow_mut().acknowledge_retired_owner());
        }
        true
    })
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CanvasClick {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) button: i16,
    pub(crate) at_ms: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CanvasGestureKind {
    Document,
    Pan,
}

#[derive(Clone, Debug)]
pub(crate) struct CanvasGesture {
    pub(crate) pointer_id: ui_render::PointerId,
    pub(crate) kind: CanvasGestureKind,
    pub(crate) window_id: String,
    pub(crate) document_generation: u64,
    pub(crate) host_id: String,
    pub(crate) surface_id: String,
    pub(crate) controller_id: String,
    pub(crate) rect: Rect,
    pub(crate) viewport: Viewport,
    pub(crate) last_x: f32,
    pub(crate) last_y: f32,
    pub(crate) button: i16,
    pub(crate) cancellation_requested: bool,
}

pub(crate) struct CanvasGestureSlots {
    pub(crate) slots: Box<[Option<CanvasGesture>; crate::element_contracts::SCENE_POINTER_OWNER_CAPACITY]>,
}

impl Default for CanvasGestureSlots {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)) }
    }
}

impl CanvasGestureSlots {
    pub(crate) fn get(&self, pointer_id: ui_render::PointerId) -> Option<&CanvasGesture> {
        self.slots.iter().flatten().find(|gesture| gesture.pointer_id == pointer_id)
    }

    pub(crate) fn get_mut(&mut self, pointer_id: ui_render::PointerId) -> Option<&mut CanvasGesture> {
        self.slots.iter_mut().flatten().find(|gesture| gesture.pointer_id == pointer_id)
    }

    pub(crate) fn has_capacity_for(&self, pointer_id: ui_render::PointerId) -> bool {
        self.get(pointer_id).is_some() || self.slots.iter().any(Option::is_none)
    }

    pub(crate) fn insert(&mut self, gesture: CanvasGesture) -> bool {
        let index = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|active| active.pointer_id == gesture.pointer_id)).or_else(|| self.slots.iter().position(Option::is_none));
        let Some(index) = index else { return false };
        self.slots[index] = Some(gesture);
        true
    }

    pub(crate) fn take(&mut self, pointer_id: ui_render::PointerId) -> Option<CanvasGesture> {
        self.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|gesture| gesture.pointer_id == pointer_id)).and_then(Option::take)
    }
}
//#endregion SceneRuntime

#[derive(Clone, Debug)]
pub(crate) struct CanvasCatalogueHover {
    pub(crate) window_id: String,
    pub(crate) document_generation: u64,
    pub(crate) host_id: String,
    pub(crate) surface_id: String,
    pub(crate) controller_id: String,
    pub(crate) last_x: f32,
    pub(crate) last_y: f32,
    pub(crate) last_dispatch_ms: f64,
    pub(crate) cancellation_requested: bool,
}
