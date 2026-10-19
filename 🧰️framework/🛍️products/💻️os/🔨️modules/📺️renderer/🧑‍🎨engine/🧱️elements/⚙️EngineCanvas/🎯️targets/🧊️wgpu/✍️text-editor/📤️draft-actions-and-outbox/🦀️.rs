//! 📤️ Draft actions and outbox: the explicit draft settings, the wheel, the action emission, the delivery bytes and receipt token, the typing commit and end and the outbox step. See [`super`].

use super::*;

//#region TextEditor
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextEditorExplicitDraftSettings {
    pub edit_action: String,
    pub edit_argument: String,
    pub edit_arguments: serde_json::Map<String, Value>,
    pub apply_label: String,
    pub discard_label: String,
    pub conflict_label: String,
    pub applying_label: String,
    pub cancel_label: String,
    pub failed_label: String,
}

/// 📝 Reads the strict explicit-commit text draft settings shared with the React host.
pub fn text_editor_explicit_draft_settings(scene: &UiComponentSceneNode) -> Option<TextEditorExplicitDraftSettings> {
    let settings = scene.text_editor.as_ref()?.settings_json.as_deref()?;
    let Value::Object(settings) = serde_json::from_str::<Value>(settings).ok()? else { return None };
    if settings.get("readOnly") != Some(&Value::Bool(false)) || settings.get("commit")?.as_str()? != "explicit" {
        return None;
    }
    let edit_action = settings.get("editAction")?.as_str()?.to_owned();
    let edit_argument = settings.get("editArgument")?.as_str()?.to_owned();
    let apply_label = settings.get("applyLabel")?.as_str()?.to_owned();
    let discard_label = settings.get("discardLabel")?.as_str()?.to_owned();
    let conflict_label = settings.get("conflictLabel")?.as_str()?.to_owned();
    let applying_label = settings.get("applyingLabel")?.as_str()?.to_owned();
    let cancel_label = settings.get("cancelLabel")?.as_str()?.to_owned();
    let failed_label = settings.get("failedLabel")?.as_str()?.to_owned();
    if edit_action.is_empty() || edit_argument.is_empty() || apply_label.is_empty() || discard_label.is_empty() || conflict_label.is_empty() || applying_label.is_empty() || cancel_label.is_empty() || failed_label.is_empty() {
        return None;
    }
    let edit_arguments = match settings.get("editArguments") {
        Some(Value::Object(arguments)) => arguments.clone(),
        None => serde_json::Map::new(),
        _ => return None,
    };
    Some(TextEditorExplicitDraftSettings { edit_action, edit_argument, edit_arguments, apply_label, discard_label, conflict_label, applying_label, cancel_label, failed_label })
}

pub fn text_editor_wheel_into(scene: &UiComponentSceneNode, delta: f32) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.host_id) else {
            return false;
        };
        let Some(host) = entry.editor.as_mut() else {
            return false;
        };
        host.wheel_scroll_screen(delta as f64);
        true
    })
}

pub(super) fn emit_text_editor_actions(
    scene: &UiComponentSceneNode,
    _input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
    edited: bool,
    projected_document_bytes: impl FnOnce(&EditorHost) -> usize,
    mutate: impl FnOnce(&mut EditorHost),
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(&scene.host_id) else {
            return Ok(false);
        };
        let Some(host) = entry.editor.as_mut() else {
            return Ok(false);
        };
        if edited && entry.editor_delivery.read_only {
            return Ok(true);
        }
        if text_editor_explicit_draft_settings(scene).is_some() {
            if entry.editor_delivery.explicit_publication.is_some() || entry.editor_delivery.explicit_active.is_some() {
                return Ok(true);
            }
            if entry.editor_delivery.explicit_base.is_none() {
                entry.editor_delivery.explicit_base = scene.text_editor.as_ref().map(|editor| editor.buffer.clone());
            }
            let before = edited.then(|| host.text().to_owned());
            mutate(host);
            if edited {
                if before.as_deref() != Some(host.text()) {
                    let changes = entry.editor_delivery.explicit_changes.as_deref().and_then(|json| serde_json::from_str::<Vec<semio_framework_ui_declaration::ui::DraftChange>>(json).ok()).unwrap_or_default();
                    entry.editor_delivery.explicit_changes = Some(ui_wgpu::wgpu::draft_changes_json(&semio_framework_ui_declaration::ui::compose_draft_step(&changes, before.as_deref().unwrap_or_default(), host.text())));
                    if entry.editor_delivery.explicit_past.len() == TEXT_EDITOR_PENDING_ECHO_CAPACITY {
                        entry.editor_delivery.explicit_past.pop_front();
                    }
                    entry.editor_delivery.explicit_past.push_back(before.unwrap_or_default());
                    entry.editor_delivery.explicit_future.clear();
                }
                entry.editor_delivery.explicit_dirty = entry.editor_delivery.explicit_base.as_deref().is_some_and(|base| host.text() != base);
                entry.editor_delivery.explicit_snapshot = Some(host.text().to_owned());
                entry.editor_delivery.explicit_error = None;
            }
            return Ok(true);
        }
        let projected_bytes = projected_document_bytes(host);
        if edited && projected_bytes > semio_framework_ui_action::RETAINED_ACTION_STRING_BYTE_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::ByteCredits);
        }
        if edited {
            semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, "textEdit", "surfaceId", &scene.surface_id, "text"])?;
        } else {
            text_editor_action_bytes(scene, None)?;
        }
        if !edited && entry.editor_delivery.selection_undeclared {
            mutate(host);
            return Ok(true);
        }
        mutate(host);
        entry.editor_delivery.offer(TextEditorDeliverySnapshot { controller_id: scene.controller_id.clone(), surface_id: scene.surface_id.clone(), text: host.text().to_owned(), start: host.anchor(), end: host.caret() });
        Ok(true)
    })
}

pub enum TextEditorActionOutcome<'a> {
    Accepted,
    Refused(&'a str),
    Cancelled,
}

fn text_editor_delivery_bytes(snapshot: &TextEditorDeliverySnapshot, include_edit: bool, include_selection: bool) -> Result<(usize, usize, usize), semio_framework_ui_action::BoundedActionFault> {
    let edit = if include_edit { semio_framework_ui_action::checked_action_string_bytes(&[&snapshot.controller_id, "textEdit", "surfaceId", &snapshot.surface_id, "text", &snapshot.text, semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_BUFFER_ARG, &snapshot.surface_id])? } else { 0 };
    let selection = if include_selection { semio_framework_ui_action::checked_action_string_bytes(&[&snapshot.controller_id, "textSelect", "surfaceId", &snapshot.surface_id, "start", "end"])? } else { 0 };
    let total = edit.checked_add(selection).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
    Ok((edit, selection, total))
}

pub(super) fn text_editor_receipt_token(sequence: u64, slot: usize) -> Option<std::num::NonZeroU64> {
    if sequence == 0 || sequence >= (u64::MAX >> 8) || slot >= ENGINE_SURFACE_CAPACITY {
        return None;
    }
    std::num::NonZeroU64::new((sequence << 8) | slot as u64)
}

pub fn has_pending_text_editor_outbox() -> bool {
    ENGINE_SURFACES.with(|cell| {
        cell.borrow().slots.iter().any(|slot| {
            slot.value.as_ref().is_some_and(|surface| surface.editor_delivery.explicit_publication.is_some() || surface.editor_delivery.publication.is_some() || surface.editor_delivery.active.is_none() && surface.editor_delivery.latest.is_some() || surface.editor_delivery.typing.is_some() || surface.editor_delivery.typing_end.is_some())
        })
    })
}

/// 🏁️ Queues one typing-run commit signal (`textEdit` with `typing` + `typingCommit`, no text): the window publishes the run as
/// ONE edit. `false` when the batch has no credit this frame (the run stays open and the next frame retries).
fn write_text_editor_typing_commit(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, run: &TextEditorTypingRun, reason: &str) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[&run.controller_id, "textEdit", "surfaceId", &run.surface_id, semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_BUFFER_ARG, &run.surface_id, semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_COMMIT_ARG, reason])?;
    let mut batch = match input.reserve_actions(1, bytes) {
        Ok(batch) => batch,
        Err(semio_framework_ui_action::BoundedActionFault::ItemCredits | semio_framework_ui_action::BoundedActionFault::ByteCredits) => return Ok(false),
        Err(fault) => return Err(fault),
    };
    batch.action(&run.controller_id, "textEdit", bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &run.surface_id)?;
        builder.string(Some(semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_BUFFER_ARG), &run.surface_id)?;
        builder.string(Some(semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_COMMIT_ARG), reason)?;
        builder.end_container()
    })?;
    batch.publish_with_checked(|| true)?;
    Ok(true)
}

/// 🏁️ Ends the open typing run of the editor `host_id` for `end` (it lost keyboard focus, the page was hidden): the commit
/// signal goes out once every delivery it typed left, so the run keeps its last keystrokes. `false` when the editor holds no run
/// and has nothing in flight.
pub fn end_text_editor_typing(host_id: &str, end: TextEditorTypingEnd) -> bool {
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().get_mut(host_id).is_some_and(|surface| surface.editor_delivery.end_typing(end)))
}

/// 🏁️ Ends every editor's open typing run for `end` (the page was hidden or left); answers how many runs it ended.
pub fn end_every_text_editor_typing(end: TextEditorTypingEnd) -> usize {
    ENGINE_SURFACES.with(|cell| cell.borrow_mut().slots.iter_mut().filter_map(|slot| slot.value.as_mut()).map(|surface| surface.editor_delivery.end_typing(end)).filter(|ended| *ended).count())
}

/// 🫥️ The hidden-page door of the browser frame Worker (`host-page-hidden`): the page was hidden or left, so every open typing
/// run ends (`hidden`) on the Worker's next frame turn.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuHostPageHidden)]
pub fn semio_wgpu_host_page_hidden() {
    end_every_text_editor_typing(TextEditorTypingEnd::Hidden);
}

/// ⏱️ Ends the first typing run that was ended ([`end_text_editor_typing`]) or idles past
/// [`ui_wgpu::wgpu::TEXT_EDITOR_TYPING_IDLE_MS`] once its editor has nothing in flight, with ONE commit signal naming why. An end
/// that finds no run left (the window already took a caret move) is dropped. `true` when a signal went out this frame.
fn drive_text_editor_typing_end(registry: &mut EngineSurfaceRegistry, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, now_ms: f64) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let ended = registry.slots.iter().position(|slot| {
        slot.value.as_ref().is_some_and(|surface| {
            let delivery = &surface.editor_delivery;
            delivery.active.is_none()
                && delivery.latest.is_none()
                && delivery.publication.is_none()
                && (delivery.typing_end.is_some() || delivery.typing.as_ref().is_some_and(|run| now_ms - run.last_ms >= semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_IDLE_MS as f64))
        })
    });
    let Some(index) = ended else { return Ok(false) };
    let delivery = &mut registry.slots[index].value.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?.editor_delivery;
    let Some(run) = delivery.typing.clone() else {
        delivery.typing_end = None;
        return Ok(false);
    };
    if write_text_editor_typing_commit(input, &run, delivery.typing_end.map_or("idle", TextEditorTypingEnd::reason))? {
        delivery.typing = None;
        delivery.typing_end = None;
        return Ok(true);
    }
    Ok(false)
}

pub fn drive_text_editor_outbox_step(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let now_ms = text_editor_now_ms();
        if drive_text_editor_typing_end(&mut registry, input, now_ms)? {
            return Ok(true);
        }
        let start = registry.text_editor_outbox_cursor % ENGINE_SURFACE_CAPACITY;
        let Some(index) = (0..ENGINE_SURFACE_CAPACITY)
            .map(|offset| (start + offset) % ENGINE_SURFACE_CAPACITY)
            .find(|index| registry.slots[*index].value.as_ref().is_some_and(|surface| surface.editor_delivery.active.is_none() && surface.editor_delivery.publication.is_none() && surface.editor_delivery.latest.is_some()))
        else {
            return Ok(false);
        };
        let Some(slot) = registry.slots.get(index) else { return Err(semio_framework_ui_action::BoundedActionFault::Structure) };
        let source = EngineSurfaceToken { slot: index as u16, generation: slot.generation };
        let surface = slot.value.as_ref().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let snapshot = surface.editor_delivery.latest.clone().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let include_edit = !surface.editor_delivery.read_only && !surface.editor_delivery.guest_has_text(&snapshot.text);
        let include_selection = !surface.editor_delivery.selection_undeclared;
        if !include_edit && !include_selection {
            registry.slots[index].value.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?.editor_delivery.latest = None;
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            return Ok(true);
        }
        if include_edit && snapshot.text.len() > semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY {
            let descriptor = ActionDescriptor {
                controller_id: snapshot.controller_id.clone(),
                action: "textEdit".into(),
                args: Some(semio_framework::DslValue::Object(vec![("surfaceId".into(), semio_framework::DslValue::String(snapshot.surface_id.clone())), (semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_BUFFER_ARG.into(), semio_framework::DslValue::String(snapshot.surface_id.clone()))])),
            };
            let action = semio_framework_ui_action::RetainedStringAction::new(descriptor, "text".into(), snapshot.text.len())?;
            let publication = TextEditorDeliveryPublication { source: snapshot.text.clone(), action, snapshot, include_selection };
            registry.slots[index].value.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?.editor_delivery.publication = Some(publication);
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            return Ok(true);
        }
        if !include_edit {
            let run = registry.slots[index].value.as_ref().and_then(|surface| surface.editor_delivery.typing.clone());
            if let Some(run) = run {
                if !write_text_editor_typing_commit(input, &run, "selectionJump")? {
                    return Ok(false);
                }
                let delivery = &mut registry.slots[index].value.as_mut().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?.editor_delivery;
                delivery.typing = None;
                delivery.typing_end = None;
            }
        }
        let token = text_editor_receipt_token(registry.next_text_editor_receipt, index).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let (edit_bytes, selection_bytes, batch_bytes) = text_editor_delivery_bytes(&snapshot, include_edit, include_selection)?;
        let mut batch = match input.reserve_actions(usize::from(include_edit) + usize::from(include_selection), batch_bytes) {
            Ok(batch) => batch,
            Err(semio_framework_ui_action::BoundedActionFault::ItemCredits | semio_framework_ui_action::BoundedActionFault::ByteCredits) => return Ok(false),
            Err(fault) => return Err(fault),
        };
        if include_edit {
            batch.action(&snapshot.controller_id, "textEdit", edit_bytes, |builder| {
                builder.set_receipt(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor, token, member: 0, abort_correlation_on_error: true })?;
                builder.begin_object(None)?;
                builder.string(Some("surfaceId"), &snapshot.surface_id)?;
                builder.string(Some("text"), &snapshot.text)?;
                builder.string(Some(semio_framework_ui_declaration::ui::TEXT_EDITOR_TYPING_BUFFER_ARG), &snapshot.surface_id)?;
                builder.end_container()
            })?;
        }
        if include_selection {
            batch.action(&snapshot.controller_id, "textSelect", selection_bytes, |builder| {
                builder.set_receipt(semio_framework_ui_action::ActionQueueReceipt { source: semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor, token, member: 1, abort_correlation_on_error: false })?;
                builder.begin_object(None)?;
                builder.string(Some("surfaceId"), &snapshot.surface_id)?;
                builder.integer(Some("start"), snapshot.start as i64)?;
                builder.integer(Some("end"), snapshot.end as i64)?;
                builder.end_container()
            })?;
        }
        let next_sequence = registry.next_text_editor_receipt.checked_add(1).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        batch.publish_with_checked(|| {
            let Some(slot) = registry.slots.get_mut(index).filter(|slot| slot.generation == source.generation) else { return false };
            let Some(surface) = slot.value.as_mut() else { return false };
            if surface.editor_delivery.latest.as_ref() != Some(&snapshot) || !surface.editor_delivery.note_dispatched(source, token, include_edit, include_selection) {
                return false;
            }
            if include_edit {
                surface.editor_delivery.typing = Some(TextEditorTypingRun { controller_id: snapshot.controller_id.clone(), surface_id: snapshot.surface_id.clone(), last_ms: now_ms });
            }
            registry.next_text_editor_receipt = next_sequence;
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            true
        })?;
        Ok(true)
    })
}
//#endregion TextEditor
