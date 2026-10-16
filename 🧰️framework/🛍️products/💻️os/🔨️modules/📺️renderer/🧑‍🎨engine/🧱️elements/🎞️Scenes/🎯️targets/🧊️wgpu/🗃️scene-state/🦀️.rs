//! 🗃️ Scene runtime state: admitted surface map, viewport, drag and ink-edit state, host temporal presentation, accessibility controls, surface retirement and canvas gestures.

use super::*;

//#region SceneRuntime


#[derive(Clone, Copy, Debug, Default)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Viewport {
    pub fn from_json(raw: &str) -> Self {
        serde_json::from_str::<Value>(raw)
            .ok()
            .map(|value| Self { x: value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, y: value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, zoom: value.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32 })
            .unwrap_or_default()
    }

    fn from_typed(viewport: Option<&semio_framework_os_kernel::Viewport2d>) -> Self {
        viewport.map(|viewport| Self { x: viewport.x as f32, y: viewport.y as f32, zoom: viewport.zoom as f32 }).unwrap_or(Self { x: 0.0, y: 0.0, zoom: 1.0 })
    }

    pub fn screen_to_world(&self, sx: f32, sy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        ((sx - cx) / self.zoom + self.x, (sy - cy) / self.zoom + self.y)
    }

    pub fn world_to_screen(&self, wx: f32, wy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        (cx + (wx - self.x) * self.zoom, cy + (wy - self.y) * self.zoom)
    }
}

#[derive(Clone, Debug)]
pub enum SceneDragMode {
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
pub struct SceneDrag {
    pub mode: SceneDragMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InkEditKind {
    Text,
    Table { row: usize, col: usize },
}

#[derive(Clone, Debug)]
pub struct InkEditState {
    pub block: Value,
    pub kind: InkEditKind,
    pub draft: String,
    pub replace_on_input: bool,
    pub screen_rect: Rect,
}

#[derive(Clone, Debug, Default)]
pub struct HostTemporalPresentation {
    pub request: Option<ui_contract::HostTemporalFormatRequestV1>,
    request_generation: u64,
    pending_generation: Option<u64>,
    pub candidate: Option<ui_contract::HostTemporalFormatReplyV1>,
    pub(crate) candidate_epoch: Option<u64>,
    pub(crate) queued_candidate: Option<ui_contract::HostTemporalFormatReplyV1>,
    pub accepted: Option<ui_contract::HostTemporalFormatReplyV1>,
    pub last_request_ms: f64,
}

fn host_temporal_requests_match(left: &ui_contract::HostTemporalFormatRequestV1, right: &ui_contract::HostTemporalFormatRequestV1) -> bool {
    left.values == right.values && (left.now_ms == right.now_ms || !left.values.iter().any(|value| value.format == ui_contract::HostTemporalFormatV1::Relative))
}

impl HostTemporalPresentation {
    pub fn observe(&mut self, request: &ui_contract::HostTemporalFormatRequestV1) -> Option<u64> {
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
        let now = crate::host_base::app_now_ms();
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

    pub fn publish(&mut self, generation: u64, request: &ui_contract::HostTemporalFormatRequestV1, reply: ui_contract::HostTemporalFormatReplyV1) -> bool {
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

    pub fn visible(&self) -> Option<&ui_contract::HostTemporalFormatReplyV1> {
        self.candidate.as_ref().or(self.accepted.as_ref())
    }

    pub fn seal(&mut self, epoch: u64) {
        if self.candidate.is_some() && self.candidate_epoch.is_none() {
            self.candidate_epoch = Some(epoch);
        }
    }

    pub fn acknowledge(&mut self, epoch: u64) {
        if self.candidate_epoch != Some(epoch) {
            return;
        }
        self.accepted = self.candidate.take();
        self.candidate = self.queued_candidate.take();
        self.candidate_epoch = None;
    }

    pub fn discard(&mut self, epoch: u64) {
        if self.candidate_epoch == Some(epoch) {
            self.candidate_epoch = None;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VfsAccessibilityControlKind {
    Row,
    Chevron,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VfsAccessibilityControl {
    pub key: String,
    pub row_id: String,
    pub label: String,
    pub kind: VfsAccessibilityControlKind,
    pub rect: Rect,
    pub expanded: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockListAccessibilityControl {
    pub key: String,
    pub label: String,
    pub rect: Rect,
    pub action: ActionDescriptor,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventFeedAccessibilityControl {
    pub key: String,
    pub entry_id: String,
    pub label: String,
    pub rect: Rect,
    pub action: Option<ActionDescriptor>,
}

/// ♿️ One node of a world surface's modelling-overlay accessibility tree (annotation list, scalar-field legend, mismatch status), staged by the paint and published with the frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dAccessibilityControl {
    pub key: String,
    pub role: &'static str,
    pub label: String,
    pub depth: u8,
    pub polite: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraphTimelineAccessibilityControl {
    pub key: String,
    pub checkpoint_id: String,
    pub label: String,
    pub rect: Rect,
    pub action: ActionDescriptor,
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
pub struct SceneSurfaceState {
    pub(crate) mount_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub(crate) engine_token: Option<EngineSurfaceToken>,
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
    pub text_editor_ui: TextEditorUiState,
    pub host_temporal: HostTemporalPresentation,
    pub table_accessibility: SceneAccessibilityPresentation<TableStepperAccessibilityCell>,
    pub table_editable_accessibility: SceneAccessibilityPresentation<TableEditableTextAccessibilityCell>,
    pub table_button_accessibility: SceneAccessibilityPresentation<TableButtonAccessibilityCell>,
    pub vfs_accessibility: SceneAccessibilityPresentation<VfsAccessibilityControl>,
    pub block_list_accessibility: SceneAccessibilityPresentation<BlockListAccessibilityControl>,
    pub event_feed_accessibility: SceneAccessibilityPresentation<EventFeedAccessibilityControl>,
    pub world3d_accessibility: SceneAccessibilityPresentation<World3dAccessibilityControl>,
    pub graph_timeline_accessibility: SceneAccessibilityPresentation<GraphTimelineAccessibilityControl>,
    /// 📏️ The label track this surface's last GraphTimeline paint measured — the one geometry its hit test reads.
    pub graph_timeline_label_track: f32,
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
    pub(crate) external_engine_token: Option<EngineSurfaceToken>,
    pub(crate) external_close: Option<crate::ComponentSurfaceCloseToken>,
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
                let Ok(token) = crate::request_component_surface_close(target.clone(), self.external_engine_token) else { return false };
                self.external_close = Some(token);
                return false;
            };
            if !crate::component_surface_close_terminal(token) {
                return false;
            }
            if !crate::acknowledge_component_surface_close(token) {
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
pub fn close_retired_scene_surface_one() -> bool {
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
pub struct CanvasClick {
    pub x: f32,
    pub y: f32,
    pub button: i16,
    pub at_ms: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanvasGestureKind {
    Document,
    Pan,
}

#[derive(Clone, Debug)]
pub struct CanvasGesture {
    pub pointer_id: ui_render::PointerId,
    pub kind: CanvasGestureKind,
    pub window_id: String,
    pub document_generation: u64,
    pub host_id: String,
    pub surface_id: String,
    pub controller_id: String,
    pub rect: Rect,
    pub viewport: Viewport,
    pub last_x: f32,
    pub last_y: f32,
    pub button: i16,
    pub cancellation_requested: bool,
}

pub struct CanvasGestureSlots {
    pub(crate) slots: Box<[Option<CanvasGesture>; crate::element_contracts::SCENE_POINTER_OWNER_CAPACITY]>,
}

impl Default for CanvasGestureSlots {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)) }
    }
}

impl CanvasGestureSlots {
    pub fn get(&self, pointer_id: ui_render::PointerId) -> Option<&CanvasGesture> {
        self.slots.iter().flatten().find(|gesture| gesture.pointer_id == pointer_id)
    }

    pub fn get_mut(&mut self, pointer_id: ui_render::PointerId) -> Option<&mut CanvasGesture> {
        self.slots.iter_mut().flatten().find(|gesture| gesture.pointer_id == pointer_id)
    }

    pub(crate) fn has_capacity_for(&self, pointer_id: ui_render::PointerId) -> bool {
        self.get(pointer_id).is_some() || self.slots.iter().any(Option::is_none)
    }

    pub fn insert(&mut self, gesture: CanvasGesture) -> bool {
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
pub struct CanvasCatalogueHover {
    pub window_id: String,
    pub document_generation: u64,
    pub host_id: String,
    pub surface_id: String,
    pub controller_id: String,
    pub last_x: f32,
    pub last_y: f32,
    pub last_dispatch_ms: f64,
    pub cancellation_requested: bool,
}

//#region ⌨️SceneInputKit

//#region SceneInput
pub const MAP_MARQUEE_THRESHOLD_PX: f32 = 6.0;

pub fn canvas_active_utility(scene: &UiComponentSceneNode) -> Option<String> {
    let layers: Vec<CanvasLayer> = serde_json::from_str(scene.canvas_2d.as_ref()?.layers_json.as_str()).ok()?;
    layers.into_iter().find(|layer| layer.role.as_deref() == Some("meta")).and_then(|layer| layer.utility)
}

pub fn request_canvas_pointer_gesture_cancel_for_host(host_id: &str) {
    request_cancel(|_, host, _| host == host_id);
    CANVAS_GESTURE.with(|cell| {
        for active in cell.borrow_mut().slots.iter_mut().flatten().filter(|active| active.host_id == host_id) {
            active.cancellation_requested = true;
        }
    });
    CANVAS_CATALOGUE_HOVER.with(|cell| {
        if let Some(active) = cell.borrow_mut().as_mut().filter(|active| active.host_id == host_id) {
            active.cancellation_requested = true;
        }
    });
}

pub fn write_scene_action_batch(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, actions: &[ActionDescriptor], commit: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = actions.iter().try_fold(0usize, |sum, action| sum.checked_add(scene_action_bytes(action)?).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits))?;
    let mut batch = input.reserve_actions(actions.len(), bytes)?;
    for action in actions {
        write_scene_action_to_batch(&mut batch, action, scene_action_bytes(action)?)?;
    }
    batch.publish_with(commit)
}

//#region SceneRowTransfer
/// 🧩️ `PALETTE_DRAG_MIME` in `🧩️BlockListHost/🟦️.tsx:41` — the block-kind mime its palette rows set
/// and its step cards read back.
pub const BLOCK_LIST_PALETTE_DRAG_MIME: &str = "application/x-semio-block-list-block-kind";

pub struct SceneListTransferStart {
    pub source: SceneListTransferSource,
    pub rect: Rect,
}

pub struct SceneListTransferRelease {
    pub suppress_click: bool,
    pub action: Option<ActionDescriptor>,
}

/// ⌨️🖱️ The modifier set a scene press carries, as `UiEvent`'s pointer variants deliver it. Kept as
/// this module's own tiny `Copy` type rather than threading `ui_wgpu::wgpu::EventModifiers` through
/// every hit signature, so the merge-mode policy below has ONE home.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SceneModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl From<ui_wgpu::wgpu::EventModifiers> for SceneModifiers {
    fn from(modifiers: ui_wgpu::wgpu::EventModifiers) -> Self {
        Self { shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }
}

impl SceneModifiers {
    /// 🍎️ The platform-neutral "add to the selection" key — `event.metaKey || event.ctrlKey`, exactly
    /// as `VirtualFileSystem`'s own row handler folds it (`⚙️VirtualFileSystem/🟦️.tsx:520`).
    pub fn additive(self) -> bool {
        self.ctrl || self.meta
    }

    /// 🎯️ The wgpu twin of `interactionMergeFromModifiers` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:945`) — the ONE
    /// click-select modifier→merge policy every surface routes through. Priority is shift, then
    /// ctrl/meta, then alt, so a chord holding more than one picks the first that matches and shift
    /// always wins a range pick. Distinct from `paint2d_merge_mode`'s marquee-drag policy, which never
    /// produces `range` because a drag has no ordered topology to range over.
    pub fn interaction_merge(self) -> &'static str {
        if self.shift {
            "range"
        } else if self.additive() {
            "invertive"
        } else if self.alt {
            "subtractive"
        } else {
            "replace"
        }
    }
}

pub struct SceneListHit {
    pub control_id: String,
    pub action: Option<ActionDescriptor>,
    /// 📁️ Set when the press landed on a virtual-file-system row's own expand/collapse chevron,
    /// which mutates renderer-local expansion state instead of dispatching a program action.
    pub toggle_expanded: Option<String>,
}

impl SceneListHit {
    pub fn row(control_id: String, action: Option<ActionDescriptor>) -> Self {
        Self { control_id, action, toggle_expanded: None }
    }
}

//#endregion ⌨️SceneInputKit

//#region 🎨️SceneRenderKit

//#region RenderEntry
pub fn render_placeholder(kind: &str, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    draw_text(ctx, &format!("{kind} host"), bounds.x + 12.0, bounds.y + 24.0, theme.font_size_body, theme.text_muted);
}

/// 🎞️ Advances one retained scene identifier scalar or one pre-admitted chrome output item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualFileSystemChromeLabels {
    pub name: &'static str,
    pub no_file_system_nodes: &'static str,
    pub expand: &'static str,
    pub collapse: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockListChromeLabels {
    pub steps: &'static str,
    pub add_step: &'static str,
    pub delete: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconRenderChromeLabels {
    pub empty_scene: &'static str,
    pub rendering: &'static str,
    pub failed: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SceneChromeLabels {
    pub virtual_file_system: VirtualFileSystemChromeLabels,
    pub block_list: BlockListChromeLabels,
    pub icon_render: IconRenderChromeLabels,
    /// 🌍️ Whether the labels are the German bundle; the world surface writes its annotation and legend text in the same language.
    pub german: bool,
}

impl SceneChromeLabels {
    pub const fn english() -> Self {
        Self {
            german: false,
            virtual_file_system: VirtualFileSystemChromeLabels { name: "Name", no_file_system_nodes: "No file system nodes", expand: "Expand", collapse: "Collapse" },
            block_list: BlockListChromeLabels { steps: "Steps", add_step: "Add Step", delete: "Delete" },
            icon_render: IconRenderChromeLabels { empty_scene: "No scene", rendering: "Rendering…", failed: "Icon rendering failed" },
        }
    }
}

/// 🧩️ The per-frame engine state the retained scene paint needs beyond its own draw list: the shell's
/// `World3dState` map (a `World3d` surface's host, addressed there by every pick/asset/authority
/// ladder) and this frame's `World3dBuildContext` (built in `FrameBuildPhase::WorldResources`, drained
/// again in `FrameBuildPhase::WorldTransfer`). Borrowed for exactly one chrome walk and never stored —
/// the same reason `Ui::frame` takes its `SceneHost` as a parameter rather than owning one.
pub struct SceneEngineHosts<'a> {
    pub chrome_labels: SceneChromeLabels,
    pub world3d_states: &'a mut AdmittedSurfaceMap<infinite_world::world::World3dState>,
    pub world_resources: &'a mut infinite_world::world::World3dBuildContext,
    /// 🪟️ The window instance whose body this walk is painting — the retention authority every
    /// attached engine surface is recorded against, so the shell can keep a surface alive through
    /// frames its window did not repaint. See
    /// `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`.
    pub window_id: &'a str,
}

//#endregion 🎨️SceneRenderKit

//#region 🧮️TableAccessibilityKit

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableStepperFocusTarget {
    pub row_id: String,
    pub column_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableEditableTextFocusTarget {
    pub row_id: String,
    pub column_id: String,
    pub value: String,
}

impl TableEditableTextFocusTarget {
    pub fn control_id(&self, host_id: &str) -> String {
        format!("{host_id}.row.{}.{}.editable", self.row_id, self.column_id)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableStepperAccessibilityCell {
    pub key: String,
    pub label: String,
    pub target: TableStepperFocusTarget,
    pub rect: Rect,
    pub min: f64,
    pub max: f64,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableEditableTextAccessibilityCell {
    pub key: String,
    pub label: String,
    pub target: TableEditableTextFocusTarget,
    pub rect: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableButtonAccessibilityCell {
    pub key: String,
    pub label: String,
    pub row_id: String,
    pub column_id: String,
    pub button_index: usize,
    pub rect: Rect,
    pub action: ActionDescriptor,
}

//#endregion 🧮️TableAccessibilityKit

//#region 🧭️TiledMapRetirement

pub fn retire_tiled_map_scene_identity(owner: &crate::element_contracts::ScenePointerTarget) -> bool {
    if owner.kind != ui_wgpu::wgpu::SurfaceKind::TiledMap {
        return false;
    }
    let (retired_interaction, retired_hover) = SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let Some(state) = states.get_mut(&owner.host_id) else { return (false, false) };
        let interaction = state.map_interaction_owner.as_ref().is_some_and(|current| current.same_component_host(owner));
        let hover = state.map_hover_owner.as_ref().is_some_and(|current| current.same_component_host(owner));
        if interaction {
            state.drag = None;
            state.map_marquee_points.clear();
            state.map_marquee_active = false;
            state.map_interaction_owner = None;
        }
        if hover {
            state.map_last_hover_json = None;
            state.map_hover_owner = None;
        }
        (interaction, hover)
    });
    if retired_interaction {
        semio_framework_os_renderer_wgpu_engine_board::tiled_map::retire_map_interaction_owner(owner);
    }
    retired_interaction || retired_hover
}

//#endregion 🧭️TiledMapRetirement

//#region ✍️TextEditorUiKit

#[derive(Clone, Debug, Default)]
pub struct TextEditorUiState {
    pub completions_open: bool,
    pub completion_index: usize,
    pub rename: Option<TextEditorRenameDraft>,
    /// 🖱️ A context-menu row this surface answers itself, parked until the next paint — the shell
    /// resolves and clears its menu without an `InputState` in hand, and every local row commits
    /// through the bounded action queue, which only the paint walk carries.
    pub pending_menu_action: Option<(String, f32, f32)>,
}

/// ✏️ One live rename gesture: the spans the commit rewrites and the name typed so far — React's
/// `renameDraft` (`{ occurrences, text }`).
#[derive(Clone, Debug)]
pub struct TextEditorRenameDraft {
    pub occurrences: Vec<(usize, usize)>,
    pub text: String,
}

pub fn text_editor_ui(host_id: &str) -> TextEditorUiState {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map(|state| state.text_editor_ui.clone()).unwrap_or_default())
}

//#endregion ✍️TextEditorUiKit

//#region ✒️InkBounds

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InkBoundsF {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl InkBoundsF {
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    pub fn intersects(&self, other: &InkBoundsF) -> bool {
        self.x < other.x + other.w && self.x + self.w > other.x && self.y < other.y + other.h && self.y + self.h > other.y
    }
}

//#endregion ✒️InkBounds

//#region 🖋️InkOutline

pub fn draw_ink_rect_outline(draw: &mut ui_wgpu::wgpu::DrawList, x: f32, y: f32, w: f32, h: f32, color: Rgba, width: f32) {
    draw.push_line(x, y, x + w, y, color, width);
    draw.push_line(x + w, y, x + w, y + h, color, width);
    draw.push_line(x + w, y + h, x, y + h, color, width);
    draw.push_line(x, y + h, x, y, color, width);
}

//#endregion 🖋️InkOutline

//#region 🧩️VfsFocusKit

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VfsAccessibilityFocusTarget {
    pub row_id: String,
    pub kind: VfsAccessibilityControlKind,
}

//#endregion 🧩️VfsFocusKit

//#region 📐️GumballState

pub const MOVE_EPSILON: f64 = 1e-9;

pub const TURN_EPSILON: f64 = 1e-6;

/// 🎛️ The handle set a gumball arms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GumballConfig {
    pub move_axes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

/// 🗺️ The affine map from model to layer units per axis: `layer = model × scale + offset`.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct ModelToLayer {
    pub scale: [f64; 2],
    pub offset: [f64; 2],
}

/// 🔭️ The host view the gumball is painted and measured in: the camera and the canvas's logical size, points local to
/// the canvas's top-left corner (`Canvas2dHost`'s `worldToScreenLogical`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GumballView {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub width: f64,
    pub height: f64,
}

impl GumballView {
    fn zoom(&self) -> f64 {
        if self.zoom == 0.0 || self.zoom.is_nan() {
            1.0
        } else {
            self.zoom
        }
    }

    fn world_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.camera_x) * self.zoom() + self.width * 0.5, (y - self.camera_y) * self.zoom() + self.height * 0.5)
    }

    fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.width * 0.5) / self.zoom() + self.camera_x, (y - self.height * 0.5) / self.zoom() + self.camera_y)
    }
}

/// ✋️ One gumball handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GumballHandle {
    MoveX,
    MoveY,
    Rotate,
    ScaleX,
    ScaleY,
    ScaleUniform,
}

impl GumballHandle {
    /// 🎚️ The verb the handle drags with.
    pub fn verb(self) -> &'static str {
        match self {
            Self::MoveX | Self::MoveY => "translateSelection",
            Self::Rotate => "rotateSelection",
            Self::ScaleX | Self::ScaleY | Self::ScaleUniform => "scaleSelection",
        }
    }

    fn enabled(self, config: &GumballConfig) -> bool {
        match self {
            Self::MoveX | Self::MoveY => config.move_axes,
            Self::ScaleX | Self::ScaleY => config.scale_axes,
            Self::ScaleUniform => config.scale_uniform,
            Self::Rotate => config.rotate,
        }
    }
}

/// 📍️ The pivot in local screen pixels under `view`.
pub fn pivot_screen(pivot_layer: [f64; 2], view: &GumballView) -> (f64, f64) {
    view.world_to_screen(pivot_layer[0], pivot_layer[1])
}

//#region 🧮️Algebra
/// 🎬️ A pose delta in model units: an offset, a turn or a pair of factors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GumballMotion {
    Translate { dx: f64, dy: f64 },
    Rotate { angle: f64 },
    Scale { sx: f64, sy: f64 },
}

/// 🤏️ One gumball gesture in flight: the handle, the local press point, the pivot, the map, the pinned ids, whether it
/// streams, the last cumulative pose delta and whether a stream tick went out.
#[derive(Clone, Debug, PartialEq)]
pub struct GumballGesture {
    pub kind: GumballHandle,
    pub start: (f64, f64),
    pub pivot_layer: [f64; 2],
    pub model_to_layer: ModelToLayer,
    pub ids: Vec<String>,
    pub live: bool,
    pub total: Option<GumballMotion>,
    pub streamed: bool,
}

fn layer_to_model(map: &ModelToLayer, (x, y): (f64, f64)) -> (f64, f64) {
    ((x - map.offset[0]) / map.scale[0], (y - map.offset[1]) / map.scale[1])
}

fn wrap_angle(angle: f64) -> f64 {
    angle - 2.0 * std::f64::consts::PI * (angle / (2.0 * std::f64::consts::PI)).round()
}

/// 🧮️ The cumulative pose delta from the press to local `(x, y)` — a turn unwrapped against the last total; `None` when
/// it moves nothing.
pub fn total(gesture: &GumballGesture, view: &GumballView, x: f64, y: f64) -> Option<GumballMotion> {
    let pivot = pivot_screen(gesture.pivot_layer, view);
    let (start_x, start_y) = gesture.start;
    match gesture.kind {
        GumballHandle::MoveX | GumballHandle::MoveY => {
            let before = layer_to_model(&gesture.model_to_layer, view.screen_to_world(start_x, start_y));
            let after = layer_to_model(&gesture.model_to_layer, view.screen_to_world(x, y));
            let dx = if gesture.kind == GumballHandle::MoveX { after.0 - before.0 } else { 0.0 };
            let dy = if gesture.kind == GumballHandle::MoveY { after.1 - before.1 } else { 0.0 };
            (dx.abs() >= MOVE_EPSILON || dy.abs() >= MOVE_EPSILON).then_some(GumballMotion::Translate { dx, dy })
        }
        GumballHandle::Rotate => {
            let raw = (y - pivot.1).atan2(x - pivot.0) - (start_y - pivot.1).atan2(start_x - pivot.0);
            let previous = match gesture.total {
                Some(GumballMotion::Rotate { angle }) => angle,
                _ => 0.0,
            };
            let angle = previous + wrap_angle(raw - previous);
            (angle.abs() >= TURN_EPSILON).then_some(GumballMotion::Rotate { angle })
        }
        GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleUniform => {
            let reach0 = (start_x - pivot.0).hypot(start_y - pivot.1);
            let reach1 = (x - pivot.0).hypot(y - pivot.1);
            let ratio = if reach0 > 1e-6 { reach1 / reach0 } else { 1.0 };
            let (sx, sy) = match gesture.kind {
                GumballHandle::ScaleUniform => (ratio, ratio),
                GumballHandle::ScaleX => (ratio, 1.0),
                _ => (1.0, ratio),
            };
            ((ratio - 1.0).abs() >= TURN_EPSILON).then_some(GumballMotion::Scale { sx, sy })
        }
    }
}

//#region 🖱️Input
/// 🪪️ The one gumball gesture the canvas owns: its pointer, window, host and document generation, the scene's address,
/// the gesture and the last local pointer (the ghost's anchor).
#[derive(Clone, Debug)]
pub struct GumballSlot {
    pointer_id: ui_render::PointerId,
    window_id: String,
    host_id: String,
    document_generation: u64,
    controller_id: String,
    surface_id: String,
    view: GumballView,
    gesture: GumballGesture,
    pointer: (f64, f64),
    cancellation_requested: bool,
}

thread_local! {
    pub static GUMBALL: RefCell<Option<GumballSlot>> = const { RefCell::new(None) };
}

/// 🚩️ Asks the next terminal step to cancel the gesture `matches(window_id, host_id, document_generation)` selects.
pub fn request_cancel(mut matches: impl FnMut(&str, &str, u64) -> bool) {
    GUMBALL.with(|cell| {
        if let Some(slot) = cell.borrow_mut().as_mut().filter(|slot| matches(&slot.window_id, &slot.host_id, slot.document_generation)) {
            slot.cancellation_requested = true;
        }
    });
}

//#endregion 📐️GumballState

//#region 🖌️CanvasLayerWire

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanvasFillJson {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub color: Option<Vec<f64>>,
    #[serde(default)]
    pub x1: f64,
    #[serde(default)]
    pub y1: f64,
    #[serde(default)]
    pub x2: f64,
    #[serde(default)]
    pub y2: f64,
    #[serde(default)]
    pub cx: f64,
    #[serde(default)]
    pub cy: f64,
    #[serde(default)]
    pub r: f64,
    #[serde(default)]
    pub stops: Vec<CanvasGradientStopJson>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CanvasGradientStopJson {
    #[serde(default)]
    pub offset: f64,
    #[serde(default)]
    pub color: Option<Vec<f64>>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasStrokeJson {
    #[serde(default)]
    pub(crate) color: Option<Vec<f64>>,
    #[serde(default)]
    pub(crate) width: Option<f64>,
    #[serde(default)]
    pub(crate) dash: Option<Vec<f64>>,
    #[serde(default)]
    pub(crate) cap: Option<String>,
    #[serde(default)]
    pub(crate) join: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasImageFieldJson {
    #[serde(default)]
    pub(crate) src: Option<String>,
    #[serde(default)]
    pub(crate) width: Option<f64>,
    #[serde(default)]
    pub(crate) height: Option<f64>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanvasTextFieldJson {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) size: Option<f64>,
    pub(crate) font_family: TextFace,
}

#[derive(Deserialize)]
pub struct CanvasLayer {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    #[serde(default)]
    pub x0: Option<f64>,
    #[serde(default)]
    pub y0: Option<f64>,
    #[serde(default)]
    pub x1: Option<f64>,
    #[serde(default)]
    pub y1: Option<f64>,
    #[serde(default, rename = "dataUrl")]
    pub data_url: Option<String>,
    #[serde(default)]
    pub points: Option<Vec<[f64; 2]>>,
    #[serde(default)]
    pub seams: Option<Vec<u8>>,
    /** 🗒️ `role === "meta"` marks a non-visual bookkeeping record (e.g. carries the host's active
     * pointer-tool id) — filtered from rendering, matches `JsonLayersCanvasSession.renderFrame`'s
     * `records.find(role === "meta")` / `layers.filter(role !== "meta")` split. */
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub(crate) utility: Option<String>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub opacity: Option<f32>,
    #[serde(default, rename = "blendMode")]
    pub blend_mode: Option<String>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub fill: Option<CanvasFillJson>,
    #[serde(default)]
    pub stroke: Option<CanvasStrokeJson>,
    #[serde(default)]
    pub image: Option<CanvasImageFieldJson>,
    #[serde(default)]
    pub text: Option<CanvasTextFieldJson>,
    #[serde(default)]
    pub segments: Option<Vec<crate::canvas2d_paint::ScenePathSegment>>,
    #[serde(default)]
    pub transform: Option<Vec<f64>>,
    #[serde(default, rename = "fillRule")]
    pub fill_rule: Option<String>,
}

//#endregion 🖌️CanvasLayerWire
