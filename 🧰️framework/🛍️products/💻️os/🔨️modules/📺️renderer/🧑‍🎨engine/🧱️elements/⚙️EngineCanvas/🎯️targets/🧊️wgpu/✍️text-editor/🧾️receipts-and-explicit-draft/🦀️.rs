//! 🧾️ Receipts and explicit draft: the action receipt settling, the selection and draft value writers, the retained action step, the explicit draft status, discard and history. See [`super`].

use super::*;

//#region TextEditor
pub fn settle_text_editor_action_receipt(receipt: semio_framework_ui_action::ActionQueueReceipt, outcome: TextEditorActionOutcome<'_>) {
    if receipt.source != semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor { return; }
    let index = (receipt.token.get() & 0xff) as usize;
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(slot) = registry.slots.get_mut(index) else { return };
        let generation = slot.generation;
        let Some(surface) = slot.value.as_mut() else { return };
        if receipt.member == 2 && surface.editor_delivery.explicit_active == Some(receipt.token) {
            surface.editor_delivery.explicit_active = None;
            surface.editor_delivery.explicit_error = match outcome {
                TextEditorActionOutcome::Accepted => None,
                TextEditorActionOutcome::Refused(reason) => Some(reason.to_owned()),
                TextEditorActionOutcome::Cancelled => Some("cancelled".into()),
            };
            return;
        }
        let (text, pending_members, source) = {
            let Some(active) = surface.editor_delivery.active.as_mut().filter(|active| active.token == receipt.token && active.source.slot as usize == index && active.source.generation == generation) else { return };
            let member = 1u8.checked_shl(u32::from(receipt.member)).unwrap_or(0);
            if member == 0 || active.pending_members & member == 0 {
                return;
            }
            active.pending_members &= !member;
            (active.snapshot.text.clone(), active.pending_members, active.source)
        };
        if source.generation != generation {
            return;
        }
        let mut resync = false;
        match outcome {
            TextEditorActionOutcome::Accepted => {}
            TextEditorActionOutcome::Refused(reason) => {
                if receipt.member == 0 {
                    if matches!(reason, "undeclared-action" | "viewer-read-only") {
                        surface.editor_delivery.read_only = true;
                    }
                    resync = surface.editor_delivery.remove_pending_echo(&text) && surface.editor_delivery.pending_echoes.is_empty();
                } else if reason == "undeclared-action" {
                    surface.editor_delivery.selection_undeclared = true;
                }
            }
            TextEditorActionOutcome::Cancelled => {
                if receipt.member == 0 {
                    resync = surface.editor_delivery.remove_pending_echo(&text) && surface.editor_delivery.pending_echoes.is_empty();
                }
            }
        }
        if pending_members == 0 {
            surface.editor_delivery.active = None;
        }
        if resync {
            if let (Some(host), Some(scene_json)) = (surface.editor.as_mut(), surface.editor_sync_cache.scene_json.as_deref()) {
                let restored = serde_json::from_str::<semio_framework_ui_declaration::ui::TextEditorScene>(scene_json).map_err(|error| error.to_string()).and_then(|editor| synchronize_text_editor_host(host, &editor, true));
                if let Err(fault) = restored {
                    engine_canvas_fault_log(&format!("[text-editor] the scene behind a refused edit could not be restored — {fault}"));
                }
            }
        }
    });
}

pub(super) fn write_text_editor_selection(batch: &mut semio_framework_ui_action::BoundedActionBatchReservation<'_>, scene: &UiComponentSceneNode, host: &EditorHost) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let select_bytes = text_editor_action_bytes(scene, None)?;
    batch.action(&scene.controller_id, "textSelect", select_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.integer(Some("start"), host.anchor() as i64)?;
        builder.integer(Some("end"), host.caret() as i64)?;
        builder.end_container()
    })
}

pub(super) fn text_editor_action_bytes(scene: &UiComponentSceneNode, document_bytes: Option<usize>) -> Result<usize, semio_framework_ui_action::BoundedActionFault> {
    let selection = semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, "textSelect", "surfaceId", &scene.surface_id, "start", "end"])?;
    let Some(document_bytes) = document_bytes else { return Ok(selection) };
    let edit = semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, "textEdit", "surfaceId", &scene.surface_id, "text"])?;
    selection.checked_add(edit).and_then(|bytes| bytes.checked_add(document_bytes)).filter(|bytes| *bytes <= semio_framework_ui_action::ACTION_QUEUE_BYTE_CAPACITY).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)
}

fn write_text_editor_draft_value(builder: &mut semio_framework_ui_action::BoundedActionBuilder, key: Option<&str>, value: &Value) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    match value {
        Value::Null => builder.null(key),
        Value::Bool(value) => builder.boolean(key, *value),
        Value::Number(value) => builder.number(key, value.as_f64().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?),
        Value::String(value) => builder.string(key, value),
        Value::Array(values) => {
            builder.begin_array(key)?;
            for value in values {
                write_text_editor_draft_value(builder, None, value)?;
            }
            builder.end_container()
        }
        Value::Object(entries) => {
            builder.begin_object(key)?;
            for (key, value) in entries {
                write_text_editor_draft_value(builder, Some(key), value)?;
            }
            builder.end_container()
        }
    }
}

/// 💾 Publishes the current local draft through its configured artifact-owned action.
pub fn text_editor_apply_explicit_draft_into(scene: &UiComponentSceneNode, _input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(settings) = text_editor_explicit_draft_settings(scene) else { return Ok(false) };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else { return Err(semio_framework_ui_action::BoundedActionFault::Structure) };
        if !entry.editor_delivery.explicit_dirty || entry.editor_delivery.explicit_conflict || entry.editor_delivery.explicit_publication.is_some() || entry.editor_delivery.explicit_active.is_some() {
            return Ok(true);
        }
        let Some(host) = entry.editor.as_ref() else { return Err(semio_framework_ui_action::BoundedActionFault::Structure) };
        let text = entry.editor_delivery.explicit_snapshot.take().unwrap_or_else(|| host.text().to_owned());
        let (source, restore) = if settings.edit_argument == semio_framework_ui_declaration::ui::DRAFT_SPLICES_ARGUMENT { (entry.editor_delivery.explicit_changes.clone().unwrap_or_else(|| "[]".to_owned()), Some(text)) } else { (text, None) };
        let args = settings.edit_arguments.iter().map(|(key, value)| (key.clone(), semio_framework::DslValue::from(value))).collect();
        let descriptor = ActionDescriptor { controller_id: scene.controller_id.clone(), action: settings.edit_action.clone(), args: Some(semio_framework::DslValue::Object(args)) };
        let action = match semio_framework_ui_action::RetainedStringAction::new(descriptor, settings.edit_argument.clone(), source.len()) {
            Ok(action) => action,
            Err(fault) => {
                entry.editor_delivery.explicit_snapshot = Some(restore.unwrap_or(source));
                entry.editor_delivery.explicit_error = Some(format!("{fault:?}"));
                return Err(fault);
            }
        };
        entry.editor_delivery.explicit_publication = Some(TextEditorDraftPublication { source, restore, action });
        entry.editor_delivery.explicit_error = None;
        Ok(true)
    })
}

pub enum TextEditorRetainedActionStep {
    Idle,
    Pending,
    Ready(TextEditorRetainedActionBatch),
}

pub(crate) struct TextEditorRetainedActionBatch {
    pub first: semio_framework_ui_action::QueuedActionDescriptor,
    pub second: Option<semio_framework_ui_action::QueuedActionDescriptor>,
}

/// 📄 Copies one bounded source page and yields the single artifact action only after completion.
pub fn drive_text_editor_retained_action_step() -> Result<TextEditorRetainedActionStep, semio_framework_ui_action::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let start = registry.text_editor_outbox_cursor % ENGINE_SURFACE_CAPACITY;
        let Some(index) = (0..ENGINE_SURFACE_CAPACITY)
            .map(|offset| (start + offset) % ENGINE_SURFACE_CAPACITY)
            .find(|index| registry.slots[*index].value.as_ref().is_some_and(|surface| surface.editor_delivery.explicit_publication.is_some() || surface.editor_delivery.publication.is_some()))
        else {
            return Ok(TextEditorRetainedActionStep::Idle);
        };
        let slot_generation = registry.slots[index].generation;
        let surface = registry.slots[index].value.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        if surface.editor_delivery.explicit_publication.is_none() {
            let publication = surface.editor_delivery.publication.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            let begin = publication.action.copied_bytes();
            let mut end = begin.saturating_add(semio_framework_ui_action::RETAINED_ACTION_STRING_PAGE_BYTES).min(publication.source.len());
            while !publication.source.is_char_boundary(end) {
                end = end.checked_sub(1).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            }
            if publication.action.push_page(publication.source.get(begin..end).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?)? != semio_framework_ui_action::RetainedStringActionStep::Complete {
                registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
                return Ok(TextEditorRetainedActionStep::Pending);
            }
            let publication = surface.editor_delivery.publication.take().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            let token = text_editor_receipt_token(registry.next_text_editor_receipt, index).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            registry.next_text_editor_receipt = registry.next_text_editor_receipt.checked_add(1).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            let first = publication.action.finish(Some(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor, token, member: 0, abort_correlation_on_error: true }))?;
            let second = publication.include_selection.then(|| semio_framework_ui_action::QueuedActionDescriptor {
                descriptor: ActionDescriptor {
                    controller_id: publication.snapshot.controller_id.clone(),
                    action: "textSelect".into(),
                    args: Some(semio_framework::DslValue::Object(vec![
                        ("surfaceId".into(), semio_framework::DslValue::String(publication.snapshot.surface_id.clone())),
                        ("start".into(), semio_framework::DslValue::uint(publication.snapshot.start as u64)),
                        ("end".into(), semio_framework::DslValue::uint(publication.snapshot.end as u64)),
                    ])),
                },
                receipt: Some(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor, token, member: 1, abort_correlation_on_error: false }),
            });
            let slot = &mut registry.slots[index];
            if slot.generation != slot_generation {
                return Err(semio_framework_ui_action::BoundedActionFault::Structure);
            }
            let surface = slot.value.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            if surface.editor_delivery.latest.as_ref() == Some(&publication.snapshot) {
                surface.editor_delivery.latest = None;
            }
            if surface.editor_delivery.pending_echoes.len() == TEXT_EDITOR_PENDING_ECHO_CAPACITY {
                surface.editor_delivery.pending_echoes.pop_front();
            }
            surface.editor_delivery.pending_echoes.push_back(publication.snapshot.text.clone());
            surface.editor_delivery.active =
                Some(TextEditorDeliveryInFlight { source: EngineSurfaceToken { slot: index as u16, generation: slot_generation }, token, snapshot: publication.snapshot, pending_members: 1 | if second.is_some() { 2 } else { 0 } });
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            return Ok(TextEditorRetainedActionStep::Ready(TextEditorRetainedActionBatch { first, second }));
        }
        let publication = surface.editor_delivery.explicit_publication.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let begin = publication.action.copied_bytes();
        let mut end = begin.saturating_add(semio_framework_ui_action::RETAINED_ACTION_STRING_PAGE_BYTES).min(publication.source.len());
        while !publication.source.is_char_boundary(end) {
            end = end.checked_sub(1).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        }
        if publication.action.push_page(publication.source.get(begin..end).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?)? != semio_framework_ui_action::RetainedStringActionStep::Complete {
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            return Ok(TextEditorRetainedActionStep::Pending);
        }
        let publication = surface.editor_delivery.explicit_publication.take().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let token = text_editor_receipt_token(registry.next_text_editor_receipt, index).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        registry.next_text_editor_receipt = registry.next_text_editor_receipt.checked_add(1).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let receipt = semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor, token, member: 2, abort_correlation_on_error: false };
        let descriptor = publication.action.finish(Some(receipt))?;
        let slot = &mut registry.slots[index];
        if slot.generation != slot_generation {
            return Err(semio_framework_ui_action::BoundedActionFault::Structure);
        }
        let surface = slot.value.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        surface.editor_delivery.explicit_snapshot = Some(publication.restore.unwrap_or(publication.source));
        surface.editor_delivery.explicit_active = Some(token);
        registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
        Ok(TextEditorRetainedActionStep::Ready(TextEditorRetainedActionBatch { first: descriptor, second: None }))
    })
}

pub fn text_editor_cancel_explicit_publication(scene: &UiComponentSceneNode) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(surface) = registry.get_mut(&scene.host_id) else { return false };
        let Some(publication) = surface.editor_delivery.explicit_publication.take() else { return false };
        surface.editor_delivery.explicit_snapshot = Some(publication.restore.unwrap_or(publication.source));
        surface.editor_delivery.explicit_error = None;
        true
    })
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextEditorExplicitDraftStatus {
    pub pending: bool,
    pub cancellable: bool,
    pub progress: f32,
    pub error: Option<String>,
}

pub fn text_editor_explicit_draft_status(scene: &UiComponentSceneNode) -> TextEditorExplicitDraftStatus {
    ENGINE_SURFACES.with(|cell| {
        let registry = cell.borrow();
        let Some(surface) = registry.get(&scene.host_id) else { return TextEditorExplicitDraftStatus { pending: false, cancellable: false, progress: 0.0, error: None } };
        let progress = surface.editor_delivery.explicit_publication.as_ref().map_or_else(|| if surface.editor_delivery.explicit_active.is_some() { 1.0 } else { 0.0 }, |publication| publication.action.progress());
        TextEditorExplicitDraftStatus {
            pending: surface.editor_delivery.explicit_publication.is_some() || surface.editor_delivery.explicit_active.is_some(),
            cancellable: surface.editor_delivery.explicit_publication.is_some(),
            progress,
            error: surface.editor_delivery.explicit_error.clone(),
        }
    })
}

/// 🗑️ Restores the persisted scene buffer and clears the native local draft.
pub fn text_editor_discard_explicit_draft(scene: &UiComponentSceneNode) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else { return false };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else { return false };
        let Some(host) = entry.editor.as_mut() else { return false };
        if entry.editor_delivery.explicit_active.is_some() {
            return false;
        }
        entry.editor_delivery.explicit_publication = None;
        if let Err(fault) = synchronize_text_editor_host(host, editor, true) {
            engine_canvas_fault_log(&format!("[text-editor] surface {} refused its persisted scene — {fault}", scene.host_id));
            return false;
        }
        entry.editor_delivery.explicit_dirty = false;
        entry.editor_delivery.explicit_base = Some(editor.buffer.clone());
        entry.editor_delivery.explicit_changes = None;
        entry.editor_delivery.explicit_conflict = false;
        entry.editor_delivery.explicit_past.clear();
        entry.editor_delivery.explicit_future.clear();
        entry.editor_delivery.explicit_snapshot = Some(editor.buffer.clone());
        entry.editor_delivery.explicit_error = None;
        true
    })
}

/// ↩️ Moves within the native local draft history without dispatching document history actions.
pub fn text_editor_move_explicit_draft_history(scene: &UiComponentSceneNode, redo: bool) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else { return false };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else { return false };
        let Some(host) = entry.editor.as_mut() else { return false };
        if entry.editor_delivery.explicit_publication.is_some() || entry.editor_delivery.explicit_active.is_some() {
            return true;
        }
        let next = if redo { entry.editor_delivery.explicit_future.pop_back() } else { entry.editor_delivery.explicit_past.pop_back() };
        let Some(next) = next else { return true };
        let current = host.text().to_owned();
        if redo {
            entry.editor_delivery.explicit_past.push_back(current);
        } else {
            entry.editor_delivery.explicit_future.push_back(current);
        }
        let changes = entry.editor_delivery.explicit_changes.as_deref().and_then(|json| serde_json::from_str::<Vec<semio_framework_ui_declaration::ui::DraftChange>>(json).ok()).unwrap_or_default();
        entry.editor_delivery.explicit_changes = Some(ui_wgpu::wgpu::draft_changes_json(&semio_framework_ui_declaration::ui::compose_draft_step(&changes, host.text(), &next)));
        host.set_text(next.clone());
        entry.editor_delivery.explicit_dirty = entry.editor_delivery.explicit_base.as_deref().is_some_and(|base| next != base);
        entry.editor_delivery.explicit_snapshot = Some(next);
        entry.editor_delivery.explicit_error = None;
        true
    })
}
//#endregion TextEditor
