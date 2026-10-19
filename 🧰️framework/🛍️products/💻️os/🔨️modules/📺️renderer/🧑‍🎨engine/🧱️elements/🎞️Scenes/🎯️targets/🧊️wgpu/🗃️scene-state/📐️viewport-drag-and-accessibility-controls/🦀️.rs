//! 📐️ Viewport, drag and accessibility controls: the viewport, the scene drag and ink edit state, the host temporal presentation and the accessibility control records. See [`super`].

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

    pub fn refuse(&mut self, generation: u64) {
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
pub struct SceneAccessibilityPresentation<T> {
    pub candidate: Option<Vec<T>>,
    pub candidate_epoch: Option<u64>,
    pub queued_candidate: Option<Vec<T>>,
    pub accepted: Vec<T>,
}

impl<T> Default for SceneAccessibilityPresentation<T> {
    fn default() -> Self {
        Self { candidate: None, candidate_epoch: None, queued_candidate: None, accepted: Vec::new() }
    }
}

impl<T> SceneAccessibilityPresentation<T> {
    pub fn stage(&mut self, controls: Vec<T>) {
        if self.candidate_epoch.is_some() {
            self.queued_candidate = Some(controls);
        } else {
            self.candidate = Some(controls);
        }
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
        self.accepted = self.candidate.take().unwrap_or_default();
        self.candidate = self.queued_candidate.take();
        self.candidate_epoch = None;
    }

    pub fn discard(&mut self, epoch: u64) {
        if self.candidate_epoch == Some(epoch) {
            self.candidate = self.queued_candidate.take();
            self.candidate_epoch = None;
        }
    }
}
//#endregion SceneRuntime
