//! 📤️ Ink event pages and edit routing: the event JSON and raw pages, the event action writers, the gesture abort, the selection and hover writers and the edit key, blur and cancel routing. See [`super`].

use super::*;

//#region InkCanvas
//#region InkCanvasState
pub struct InkEventJsonPages {
    pub(crate) bytes: Box<[u8; INK_EVENT_JSON_BYTE_CAPACITY]>,
    pub len: usize,
    pub items: usize,
    sealed: bool,
}

pub struct InkRawPages {
    pub(crate) bytes: Box<[u8; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]>,
    pub(crate) spans: Box<[Option<(u16, u16)>; INK_INTERACTION_ITEM_CAPACITY]>,
    pub byte_len: usize,
    pub(crate) head: usize,
    pub len: usize,
}

pub(super) struct InkOwnedRaw {
    pub(crate) bytes: Box<[u8; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]>,
    pub(crate) len: usize,
}

impl InkOwnedRaw {
    pub(crate) fn new(raw: &str) -> Result<Self, semio_framework_ui_action::BoundedActionFault> {
        if raw.len() > INK_INTERACTION_DOCUMENT_BYTE_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
        }
        let mut bytes = Box::new([0; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]);
        bytes[..raw.len()].copy_from_slice(raw.as_bytes());
        Ok(Self { bytes, len: raw.len() })
    }

    pub(crate) fn as_str(&self) -> Result<&str, semio_framework_ui_action::BoundedActionFault> {
        std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)
    }
}

impl Default for InkRawPages {
    fn default() -> Self {
        Self { bytes: Box::new([0; INK_INTERACTION_DOCUMENT_BYTE_CAPACITY]), spans: Box::new(std::array::from_fn(|_| None)), byte_len: 0, head: 0, len: 0 }
    }
}

impl InkRawPages {
    pub fn push(&mut self, raw: &str) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
        if self.len == self.spans.len() {
            return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
        }
        let end = self.byte_len.checked_add(raw.len()).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
        if end > self.bytes.len() {
            return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
        }
        self.bytes[self.byte_len..end].copy_from_slice(raw.as_bytes());
        let index = (self.head + self.len) % self.spans.len();
        self.spans[index] = Some((u16::try_from(self.byte_len).map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)?, u16::try_from(end).map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)?));
        self.byte_len = end;
        self.len += 1;
        Ok(())
    }

    pub fn front(&self) -> Result<Option<&str>, semio_framework_ui_action::BoundedActionFault> {
        let Some((start, end)) = self.spans[self.head] else {
            return Ok(None);
        };
        std::str::from_utf8(&self.bytes[usize::from(start)..usize::from(end)]).map(Some).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)
    }

    pub fn pop_front(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.spans[self.head] = None;
        self.head = (self.head + 1) % self.spans.len();
        self.len -= 1;
        if self.len == 0 {
            self.head = 0;
            self.byte_len = 0;
        }
        true
    }
}

impl Default for InkEventJsonPages {
    fn default() -> Self {
        let mut bytes = Box::new([0; INK_EVENT_JSON_BYTE_CAPACITY]);
        bytes[0] = b'[';
        Self { bytes, len: 1, items: 0, sealed: false }
    }
}

impl std::io::Write for InkEventJsonPages {
    fn write(&mut self, source: &[u8]) -> std::io::Result<usize> {
        let end = self.len.checked_add(source.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::WriteZero, "ink event byte credits exhausted"))?;
        if end >= INK_EVENT_JSON_BYTE_CAPACITY {
            return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "ink event byte credits exhausted"));
        }
        self.bytes[self.len..end].copy_from_slice(source);
        self.len = end;
        Ok(source.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl InkEventJsonPages {
    pub fn push(&mut self, value: &Value) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
        if self.sealed || self.items == INK_INTERACTION_ITEM_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
        }
        let checkpoint = self.len;
        let result = (|| {
            if self.items != 0 {
                self.write_all(b",").map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
            }
            serde_json::to_writer(&mut *self, value).map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)
        })();
        if let Err(fault) = result {
            self.len = checkpoint;
            return Err(fault);
        }
        self.items += 1;
        Ok(())
    }

    pub(super) fn push_add_block_raw(&mut self, block: &str) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
        if self.sealed || self.items == INK_INTERACTION_ITEM_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
        }
        let checkpoint = self.len;
        let result = (|| {
            if self.items != 0 {
                self.write_all(b",").map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
            }
            self.write_all(b"{\"operation\":\"addBlock\",\"block\":").map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
            self.write_all(block.as_bytes()).map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
            self.write_all(b"}").map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)
        })();
        if let Err(fault) = result {
            self.len = checkpoint;
            return Err(fault);
        }
        self.items += 1;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
        if self.sealed {
            return Ok(());
        }
        self.write_all(b"]").map_err(|_| semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
        self.sealed = true;
        Ok(())
    }

    pub fn as_str(&self) -> Result<&str, semio_framework_ui_action::BoundedActionFault> {
        if !self.sealed {
            return Err(semio_framework_ui_action::BoundedActionFault::Structure);
        }
        std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)
    }
}

/// 📨️ One `inkApplyEvents` action: a `stream` tick into the plugin's open ink transaction, the `commit` that publishes it
/// as ONE edit, or (no phase) a one-shot — with the drag record of a block move instead of per-block events.
pub(super) fn write_ink_events_action(
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
    scene: &UiComponentSceneNode,
    events: &InkEventJsonPages,
    phase: Option<&str>,
    gesture: Option<&str>,
    select_id: Option<&str>,
    mutate: impl FnOnce(),
) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let events = events.as_str()?;
    let mut parts: Vec<&str> = vec![&scene.controller_id, "inkApplyEvents", "surfaceId", &scene.surface_id, "eventsJson", events];
    if let Some(phase) = phase {
        parts.extend(["phase", phase]);
    }
    if let Some(gesture) = gesture {
        parts.extend(["gestureJson", gesture]);
    }
    let base = semio_framework_ui_action::checked_action_string_bytes(&parts)?;
    let bytes = match select_id {
        Some(id) => base.checked_add(semio_framework_ui_action::checked_action_string_bytes(&["selectIds", id])?).filter(|bytes| *bytes <= semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?,
        None => base,
    };
    let mut reservation = input.reserve_action(&scene.controller_id, "inkApplyEvents", bytes)?;
    let streams = phase == Some("stream");
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("eventsJson"), events)?;
    if let Some(phase) = phase {
        builder.string(Some("phase"), phase)?;
    }
    if let Some(gesture) = gesture {
        builder.string(Some("gestureJson"), gesture)?;
    }
    if let Some(id) = select_id {
        builder.begin_array(Some("selectIds"))?;
        builder.string(None, id)?;
        builder.end_container()?;
    }
    builder.end_container()?;
    reservation.publish_with(|| {
        mutate();
        if streams {
            mutate_scene_state(&scene.host_id, |state| state.ink_gesture_open = true);
        }
    })
}

/// 🧯️ The plugin half of an ink pointer cancel: a gesture that streamed into the plugin's open ink transaction is
/// aborted with ONE `inkApplyEvents` (`phase: "abort"`, `reason: "captureLost"`), so the plugin drops it with zero trace;
/// a press that never streamed publishes nothing. Answers whether an abort was published; runs before
/// [`ink_pointer_cancel_into`] clears the local drag.
pub fn ink_gesture_abort_into(controller_id: &str, surface_id: &str, host_id: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    if !SCENE_STATE.with(|cell| cell.borrow().get(host_id).is_some_and(|state| state.ink_gesture_open)) {
        return Ok(false);
    }
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[controller_id, "inkApplyEvents", "surfaceId", surface_id, "eventsJson", "[]", "phase", "abort", "reason", "captureLost"])?;
    let mut reservation = input.reserve_action(controller_id, "inkApplyEvents", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    builder.string(Some("eventsJson"), "[]")?;
    builder.string(Some("phase"), "abort")?;
    builder.string(Some("reason"), "captureLost")?;
    builder.end_container()?;
    reservation.publish_with(|| mutate_scene_state(host_id, |state| state.ink_gesture_open = false))?;
    Ok(true)
}

pub(super) fn ink_interaction_domain(scene: &UiComponentSceneNode) -> Option<(&str, &str)> {
    let ink = scene.ink_canvas.as_ref()?;
    let domain = ink.interaction_domain.as_ref()?;
    (!domain.id.is_empty() && !domain.granularity_id.is_empty()).then_some((domain.id.as_str(), domain.granularity_id.as_str()))
}

pub(super) fn ink_interaction_id(block: &Value) -> Option<&str> {
    block.get("interactionId").and_then(Value::as_str).filter(|id| !id.is_empty())
}

pub(super) fn write_ink_interaction_selection(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, ids: &[String], merge: &str, method: &str, mutate: impl FnOnce()) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let Some((domain_id, granularity)) = ink_interaction_domain(scene) else {
        mutate();
        return Ok(());
    };
    if ids.len() > INK_SELECTION_ITEM_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::ItemCredits);
    }
    let targets = serde_json::to_string(&ids.iter().map(|id| json!({ "granularity": granularity, "id": id })).collect::<Vec<_>>()).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, "interactionSelect", "surfaceId", &scene.surface_id, "domainId", domain_id, "targets", &targets, "merge", merge, "method", method])?;
    let mut reservation = input.reserve_action(&scene.controller_id, "interactionSelect", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("domainId"), domain_id)?;
    builder.string(Some("targets"), &targets)?;
    builder.string(Some("merge"), merge)?;
    builder.string(Some("method"), method)?;
    builder.end_container()?;
    reservation.publish_with(mutate)
}

pub(super) fn write_ink_interaction_hover(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, scene: &UiComponentSceneNode, id: Option<&str>) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let Some((domain_id, granularity)) = ink_interaction_domain(scene) else { return Ok(()) };
    let targets = serde_json::to_string(&id.map(|id| vec![json!({ "granularity": granularity, "id": id })]).unwrap_or_default()).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, "interactionHover", "surfaceId", &scene.surface_id, "domainId", domain_id, "channel", "pointer", "targets", &targets])?;
    let mut reservation = input.reserve_action(&scene.controller_id, "interactionHover", bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), &scene.surface_id)?;
    builder.string(Some("domainId"), domain_id)?;
    builder.string(Some("channel"), "pointer")?;
    builder.string(Some("targets"), &targets)?;
    builder.end_container()?;
    reservation.publish()
}

pub(crate) fn clear_ink_pointer_state(surface_id: &str) {
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.pointer_was_down = false;
        state.ink_gesture_open = false;
        state.ink_marquee_points.clear();
    });
}

fn ink_edit_updated_block(edit: &InkEditState) -> Result<Value, semio_framework_ui_action::BoundedActionFault> {
    let mut block = edit.block.clone();
    match edit.kind {
        InkEditKind::Text => {
            let Some(object) = block.as_object_mut() else { return Err(semio_framework_ui_action::BoundedActionFault::Structure) };
            object.insert("paragraphs".into(), json!([{ "runs": [{ "text": edit.draft }] }]));
        }
        InkEditKind::Table { row, col } => {
            let Some(rows) = block.get_mut("rows").and_then(Value::as_array_mut) else { return Err(semio_framework_ui_action::BoundedActionFault::Structure) };
            let Some(cells) = rows.get_mut(row).and_then(Value::as_array_mut) else { return Err(semio_framework_ui_action::BoundedActionFault::Structure) };
            let Some(cell) = cells.get_mut(col).and_then(Value::as_object_mut) else { return Err(semio_framework_ui_action::BoundedActionFault::Structure) };
            cell.insert("content".into(), Value::String(edit.draft.clone()));
        }
    }
    Ok(block)
}

fn ink_edit_next_cell(edit: &InkEditState, updated: &Value) -> Option<InkEditState> {
    let InkEditKind::Table { row, col } = edit.kind else { return None };
    let columns = updated.get("columns")?.as_array()?.len();
    let rows = updated.get("rows")?.as_array()?;
    if columns == 0 {
        return None;
    }
    let (next_row, next_col) = if col + 1 < columns { (row, col + 1) } else { (row + 1, 0) };
    let draft = rows.get(next_row)?.get(next_col)?.get("content").and_then(Value::as_str).unwrap_or_default().to_owned();
    let mut screen_rect = edit.screen_rect;
    if next_row == row {
        screen_rect.x += screen_rect.w;
    } else {
        screen_rect.x -= col as f32 * screen_rect.w;
        screen_rect.y += screen_rect.h;
    }
    Some(InkEditState { block: updated.clone(), kind: InkEditKind::Table { row: next_row, col: next_col }, draft, replace_on_input: true, screen_rect })
}

fn ink_publish_edit(scene: &UiComponentSceneNode, edit: InkEditState, advance: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let updated = ink_edit_updated_block(&edit)?;
    let block_id = ink_item_id(&updated).to_owned();
    let next = advance.then(|| ink_edit_next_cell(&edit, &updated)).flatten();
    let next_focus = next.as_ref().map(|next| (ink_edit_control_id(&scene.host_id, next), next.draft.clone()));
    let state_block = updated.clone();
    let mut events = InkEventJsonPages::default();
    events.push(&json!({ "operation": "updateBlock", "blockId": block_id.clone(), "block": updated }))?;
    events.seal()?;
    write_ink_events_action(input, scene, &events, None, None, None, || {
        mutate_scene_state(&scene.host_id, |state| {
            state.ink_overrides.insert(block_id, state_block);
            state.ink_edit = next;
        });
    })?;
    match next_focus {
        Some((control_id, draft)) => input.focus_input_owned(control_id, draft),
        None => input.blur_input(),
    }
    Ok(())
}

pub fn ink_edit_key_into(scene: &UiComponentSceneNode, key: &KeyAction, modifiers: &semio_framework_ui_wgpu_input::PointerModifiers, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(mut edit) = scene_state(&scene.host_id).ink_edit else { return Ok(false) };
    if modifiers.ctrl_or_meta() {
        return Ok(false);
    }
    match key {
        KeyAction::Escape => {
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = None);
            input.blur_input();
        }
        KeyAction::Enter if matches!(edit.kind, InkEditKind::Table { .. }) => ink_publish_edit(scene, edit, true, input)?,
        KeyAction::Tab if matches!(edit.kind, InkEditKind::Table { .. }) => ink_publish_edit(scene, edit, true, input)?,
        KeyAction::Tab => ink_publish_edit(scene, edit, false, input)?,
        KeyAction::Enter => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            }
            edit.draft.push('\n');
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        KeyAction::Backspace | KeyAction::Delete => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            } else {
                edit.draft.pop();
            }
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        KeyAction::Char(text) => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            }
            edit.draft.push_str(text);
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        KeyAction::Space(true) => {
            if edit.replace_on_input {
                edit.draft.clear();
                edit.replace_on_input = false;
            }
            edit.draft.push(' ');
            mutate_scene_state(&scene.host_id, |state| state.ink_edit = Some(edit));
        }
        _ => {}
    }
    Ok(true)
}

pub fn ink_edit_blur_into(scene: &UiComponentSceneNode, x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(edit) = scene_state(&scene.host_id).ink_edit else { return Ok(false) };
    if edit.screen_rect.contains(x, y) {
        return Ok(false);
    }
    ink_publish_edit(scene, edit, false, input)?;
    Ok(true)
}

pub fn cancel_ink_edit(surface_id: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) {
    mutate_scene_state(surface_id, |state| state.ink_edit = None);
    if input.focused_id.as_deref().is_some_and(|id| id.starts_with(&format!("{surface_id}.ink."))) {
        input.blur_input();
    }
}

pub fn ink_edit_active(surface_id: &str) -> bool {
    scene_state(surface_id).ink_edit.is_some()
}
//#endregion InkCanvasState
//#endregion InkCanvas
