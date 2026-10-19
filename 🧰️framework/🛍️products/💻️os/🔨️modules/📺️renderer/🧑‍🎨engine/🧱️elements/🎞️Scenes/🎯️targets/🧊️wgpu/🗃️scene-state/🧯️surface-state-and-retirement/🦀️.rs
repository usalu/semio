//! 🧯️ Surface state and retirement: the scene surface state, the value retirement, the surface retirement with its close step and the retired surface close. See [`super`].

use super::*;

//#region SceneRuntime
#[derive(Clone, Debug, Default)]
pub struct SceneSurfaceState {
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub mount_owner: Option<crate::element_contracts::ScenePointerTarget>,
    #[cfg(not(any(test, feature = "testkit")))]
    pub(crate) mount_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub(crate) engine_token: Option<EngineSurfaceToken>,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub scroll_offsets: BTreeMap<String, f32>,
    #[cfg(not(any(test, feature = "testkit")))]
    pub(crate) scroll_offsets: BTreeMap<String, f32>,
    pub viewport: Viewport,
    pub(crate) canvas_camera_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub canvas_framing_revision: Option<u32>,
    pub drag: Option<SceneDrag>,
    pub pointer_was_down: bool,
    pub last_click_ms: f64,
    pub last_click_target: Option<String>,
    /// 🖱️ The list-surface control the pointer last resolved over — this module's own hover
    /// authority, because a `ComponentScene` leaf registers exactly one retained hit target for the
    /// WHOLE surface (`retained_scene_hit`), so `InputState::hovered_id` can never name a row.
    pub hovered_control_id: Option<String>,
    #[cfg(test)]
    node_positions: BTreeMap<String, (f32, f32)>,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    pub selected_ids: BTreeSet<String>,
    #[cfg(not(any(test, feature = "testkit")))]
    pub(crate) selected_ids: BTreeSet<String>,
    pub paint_stroke_active: bool,
    pub vfs_expanded_ids: BTreeSet<String>,
    pub(crate) vfs_selection_anchor: Option<String>,
    pub map_marquee_points: Vec<(f32, f32)>,
    pub map_marquee_active: bool,
    pub map_last_hover_json: Option<String>,
    pub map_interaction_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub map_hover_owner: Option<crate::element_contracts::ScenePointerTarget>,
    pub touch_gesture: GestureRecognizer,
    pub ink_camera: Option<(f64, f64, f64)>,
    pub ink_overrides: BTreeMap<String, Value>,
    /// 🌊️ A move, resize, stroke or erase has streamed into the plugin's open ink transaction since the press — what a
    /// pointer cancel must abort.
    pub ink_gesture_open: bool,
    pub ink_marquee_points: Vec<(f32, f32)>,
    pub ink_edit: Option<InkEditState>,
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
    pub last_pointer_pos: (f32, f32),
    /// 🕒️ Controller for the mounted surface's settled camera publication.
    pub camera_dispatch_controller_id: Option<String>,
    pub canvas_click: Option<CanvasClick>,
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
    if let semio_framework_ui_retained_tree::tree::NodeKey::Explicit(key) = &mut owner.key {
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
//#endregion SceneRuntime
