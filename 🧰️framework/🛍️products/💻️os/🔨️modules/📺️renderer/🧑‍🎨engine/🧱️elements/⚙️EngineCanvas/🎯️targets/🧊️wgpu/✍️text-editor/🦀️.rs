//! ✍️ Text editor scene document, delivery and interaction.
//!
//! The editor scene document, the typing delivery state and echo ledger, the explicit draft settings and the public
//! text editor pointer, key, wheel and action API.

use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{ENGINE_SURFACES, ENGINE_SURFACE_CAPACITY, EngineSurfaceRegistry, EngineSurfaceToken, TEXT_EDITOR_PENDING_ECHO_CAPACITY, TextEditorDeliveryInFlight, TextEditorDeliveryPublication, TextEditorDeliverySnapshot, TextEditorDraftPublication, TextEditorTypingEnd, TextEditorTypingRun, engine_canvas_fault_log, retire_string_page, text_editor_now_ms};
use framework_editor::EditorHost;
#[cfg(target_arch = "wasm32")]
use js_sys;
use serde_json::{json, Value};
use std::collections::VecDeque;
use ui_wgpu::wgpu::{ActionDescriptor, KeyAction, PointerModifiers, Rect, UiComponentSceneNode};

//#region ✍️EditorSceneDocument
/// 📏️ The bytes one decoded editor scene may own — the authority React's twin grants the same decode
/// (`session.synchronizeScene(pack, 0, 16_777_216, 16_777_216, …)`, `🧱️elements/✏️TextEditor/🟦️.tsx`).
pub const TEXT_EDITOR_SCENE_OWNED_BYTES: usize = 16_777_216;

/// ↔️ One byte range of the canonical scene: the selection, an occurrence.
#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
struct EditorSceneRange {
    pub(super) start: usize,
    pub(super) end: usize,
}

/// 🎨️ One semantic token of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorSceneToken {
    pub(super) start: usize,
    pub(super) end: usize,
    class: String,
}

/// 🩺️ One diagnostic of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorSceneDiagnostic {
    pub(super) start: usize,
    pub(super) end: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    severity: Option<String>,
    message: String,
}

/// 🪧️ One placeholder of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorScenePlaceholder {
    offset: usize,
    pub(super) label: String,
}

/// 🎯️ One selectable span of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorSceneSpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) kind: String,
}

/// ⚙️ The editor ENGINE's settings — the four members the canonical scene declares. The contract's `settingsJson` also
/// carries the host's own members (`readOnly`, `commit`, the explicit-draft labels, read by
/// [`text_editor_explicit_draft_settings`]); the canonical scene refuses a member it does not declare, so only these four
/// cross, and an absent one takes the engine's default.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font_px: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    show_line_numbers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tab_size: Option<usize>,
}

/// 🔦️ The hover and selection occurrence lists of the canonical scene.
#[derive(serde::Serialize)]
struct EditorSceneOccurrences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) hover: Option<Vec<EditorSceneRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) selection: Option<Vec<EditorSceneRange>>,
}

/// 📷️ The canonical scene's camera: one scrolling axis.
#[derive(serde::Serialize)]
struct EditorSceneCamera {
    pub(super) y: f64,
}

/// 🚧️ The canonical scene's overlays.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneOverlays {
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_line_y: Option<f64>,
}

/// 🐁️ The canonical scene's hover: cleared, or one range.
#[derive(serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum EditorSceneHover {
    Clear,
    Range { start: usize, end: usize },
}

/// 🧾️ The editor engine's canonical scene document (`framework.editor.canonical-scene/v1`) as this host writes it. A
/// member left out leaves the engine's own value unchanged.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) buffer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) selection: Option<EditorSceneRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tokens: Option<Vec<EditorSceneToken>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics: Option<Vec<EditorSceneDiagnostic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholders: Option<Vec<EditorScenePlaceholder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    occurrences: Option<EditorSceneOccurrences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_carets: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selectable_spans: Option<Vec<EditorSceneSpan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<EditorSceneSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) camera: Option<EditorSceneCamera>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlays: Option<EditorSceneOverlays>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) hover: Option<EditorSceneHover>,
}

/// 🧷️ The contract's `selectionJson`: a caret alone is a collapsed range.
#[derive(serde::Deserialize)]
struct EditorSceneSelectionField {
    #[serde(default)]
    pub(super) start: usize,
    #[serde(default)]
    pub(super) end: Option<usize>,
}

/// 🗂️ The contract's `occurrencesJson`: each list is itself an encoded scene field.
#[derive(serde::Deserialize)]
struct EditorSceneOccurrenceFields {
    #[serde(default)]
    pub(super) hover: Option<String>,
    #[serde(default)]
    pub(super) selection: Option<String>,
}

/// 🎥️ The contract's `cameraJson`: a whole camera, of which the editor scrolls one axis.
#[derive(serde::Deserialize)]
struct EditorSceneCameraField {
    #[serde(default)]
    pub(super) y: f64,
}

/// 🧱️ The contract's `overlaysJson`.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneOverlaysField {
    #[serde(default)]
    dead_line_y: Option<f64>,
}

/// 🫥️ The contract's `hoverJson`: a range hovers, anything short of one clears.
#[derive(serde::Deserialize)]
struct EditorSceneHoverField {
    #[serde(default)]
    pub(super) start: Option<usize>,
    #[serde(default)]
    pub(super) end: Option<usize>,
}

/// 🔎️ One `*Json` member of the contract's scene in its typed form: the encoded field expanded
/// (`semio_framework_os_kernel::os_pack::pack_rt::scene_field_json_text`, React's `parseSceneJsonField`) and read; a JSON `null` is no value.
fn text_editor_scene_member<T: serde::de::DeserializeOwned>(member: &str, raw: &str) -> Result<Option<T>, String> {
    let json = semio_framework_os_kernel::os_pack::pack_rt::scene_field_json_text(raw).map_err(|error| format!("{member}: {error}"))?;
    serde_json::from_str::<Option<T>>(&json).map_err(|error| format!("{member}: {error}"))
}

/// 📥️ [`text_editor_scene_member`] for a member that may be absent; one it cannot read is named in `faults` and left out.
fn text_editor_scene_field<T: serde::de::DeserializeOwned>(member: &str, raw: Option<&str>, faults: &mut Vec<String>) -> Option<T> {
    match text_editor_scene_member(member, raw?) {
        Ok(value) => value,
        Err(fault) => {
            faults.push(fault);
            None
        }
    }
}

/// 🏗️ Builds the editor engine's canonical scene document from the UI contract's text-editor scene — the ONE place this
/// host turns the contract's `*Json` string members into the typed members `EditorHost::synchronize_scene` takes, the
/// wgpu twin of React's `textEditorScenePacket` (`🧱️elements/✏️TextEditor/🟦️.tsx`).
///
/// `carries_text` is whether the scene's buffer and selection are news to the host: a scene that only echoes the host's
/// own edit leaves both out, so the local draft and its caret stay (React's `sceneWithoutEchoedTextV1`). A member this
/// host cannot read is left out — unchanged in the engine, by the document's own rule — and named in the second answer;
/// it is never guessed.
pub fn text_editor_scene_document(editor: &ui_wgpu::wgpu::TextEditorScene, carries_text: bool) -> (String, Vec<String>) {
    let mut faults = Vec::new();
    let selection = if carries_text { text_editor_scene_field::<EditorSceneSelectionField>("selectionJson", editor.selection_json.as_deref(), &mut faults) } else { None };
    let occurrences = text_editor_scene_field::<EditorSceneOccurrenceFields>("occurrencesJson", editor.occurrences_json.as_deref(), &mut faults);
    let hover = editor.hover_json.as_deref().and_then(|raw| match text_editor_scene_member::<EditorSceneHoverField>("hoverJson", raw) {
        Ok(Some(EditorSceneHoverField { start: Some(start), end: Some(end) })) => Some(EditorSceneHover::Range { start, end }),
        Ok(_) => Some(EditorSceneHover::Clear),
        Err(fault) => {
            faults.push(fault);
            None
        }
    });
    let document = EditorSceneDocument {
        buffer: carries_text.then(|| editor.buffer.clone()),
        selection: selection.map(|field| EditorSceneRange { start: field.start, end: field.end.unwrap_or(field.start) }),
        tokens: text_editor_scene_field("tokensJson", editor.tokens_json.as_deref(), &mut faults),
        diagnostics: text_editor_scene_field("diagnosticsJson", editor.diagnostics_json.as_deref(), &mut faults),
        placeholders: text_editor_scene_field("placeholdersJson", editor.placeholders_json.as_deref(), &mut faults),
        occurrences: occurrences.map(|fields| EditorSceneOccurrences {
            hover: text_editor_scene_field("occurrencesJson.hover", fields.hover.as_deref(), &mut faults),
            selection: text_editor_scene_field("occurrencesJson.selection", fields.selection.as_deref(), &mut faults),
        }),
        extra_carets: text_editor_scene_field("extraCaretsJson", editor.extra_carets_json.as_deref(), &mut faults),
        selectable_spans: text_editor_scene_field("selectableSpansJson", editor.selectable_spans_json.as_deref(), &mut faults),
        settings: text_editor_scene_field("settingsJson", editor.settings_json.as_deref(), &mut faults),
        camera: text_editor_scene_field::<EditorSceneCameraField>("cameraJson", editor.camera_json.as_deref(), &mut faults).map(|field| EditorSceneCamera { y: field.y }),
        overlays: text_editor_scene_field::<EditorSceneOverlaysField>("overlaysJson", editor.overlays_json.as_deref(), &mut faults).map(|field| EditorSceneOverlays { dead_line_y: field.dead_line_y }),
        hover,
    };
    (serde_json::to_string(&document).unwrap_or_else(|_| "{}".to_string()), faults)
}

/// 🎬️ Commits one contract scene into its editor host: the canonical document ([`text_editor_scene_document`]) decoded
/// under a bounded authority ([`TEXT_EDITOR_SCENE_OWNED_BYTES`]) and handed to `EditorHost::synchronize_scene`. A member
/// the builder left out is told on the console; a document the engine refuses is the caller's `Err` — never silence.
pub fn synchronize_text_editor_host(host: &mut EditorHost, editor: &ui_wgpu::wgpu::TextEditorScene, carries_text: bool) -> Result<(), String> {
    let (document, faults) = text_editor_scene_document(editor, carries_text);
    for fault in &faults {
        engine_canvas_fault_log(&format!("[text-editor] scene member left unchanged — {fault}"));
    }
    let mut admitted = |_| true;
    let mut control = semio_framework_value::NativeDecodeControl::new(TEXT_EDITOR_SCENE_OWNED_BYTES, &mut admitted);
    let scene = framework_editor::scene::from_json(&document, &mut control).map_err(|error| error.to_string())?;
    host.synchronize_scene(scene);
    Ok(())
}
//#endregion ✍️EditorSceneDocument

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

fn emit_text_editor_actions(
    scene: &UiComponentSceneNode,
    _input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    edited: bool,
    projected_document_bytes: impl FnOnce(&EditorHost) -> usize,
    mutate: impl FnOnce(&mut EditorHost),
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
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
                    let changes = entry.editor_delivery.explicit_changes.as_deref().and_then(|json| serde_json::from_str::<Vec<ui_wgpu::wgpu::DraftChange>>(json).ok()).unwrap_or_default();
                    entry.editor_delivery.explicit_changes = Some(ui_wgpu::wgpu::draft_changes_json(&ui_wgpu::wgpu::compose_draft_step(&changes, before.as_deref().unwrap_or_default(), host.text())));
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
        if edited && projected_bytes > ui_wgpu::wgpu::RETAINED_ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        if edited {
            ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "textEdit", "surfaceId", &scene.surface_id, "text"])?;
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

fn text_editor_delivery_bytes(snapshot: &TextEditorDeliverySnapshot, include_edit: bool, include_selection: bool) -> Result<(usize, usize, usize), ui_wgpu::wgpu::BoundedActionFault> {
    let edit = if include_edit { ui_wgpu::wgpu::checked_action_string_bytes(&[&snapshot.controller_id, "textEdit", "surfaceId", &snapshot.surface_id, "text", &snapshot.text, ui_wgpu::wgpu::TEXT_EDITOR_TYPING_BUFFER_ARG, &snapshot.surface_id])? } else { 0 };
    let selection = if include_selection { ui_wgpu::wgpu::checked_action_string_bytes(&[&snapshot.controller_id, "textSelect", "surfaceId", &snapshot.surface_id, "start", "end"])? } else { 0 };
    let total = edit.checked_add(selection).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
    Ok((edit, selection, total))
}

fn text_editor_receipt_token(sequence: u64, slot: usize) -> Option<std::num::NonZeroU64> {
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
fn write_text_editor_typing_commit(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, run: &TextEditorTypingRun, reason: &str) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&run.controller_id, "textEdit", "surfaceId", &run.surface_id, ui_wgpu::wgpu::TEXT_EDITOR_TYPING_BUFFER_ARG, &run.surface_id, ui_wgpu::wgpu::TEXT_EDITOR_TYPING_COMMIT_ARG, reason])?;
    let mut batch = match input.reserve_actions(1, bytes) {
        Ok(batch) => batch,
        Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => return Ok(false),
        Err(fault) => return Err(fault),
    };
    batch.action(&run.controller_id, "textEdit", bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &run.surface_id)?;
        builder.string(Some(ui_wgpu::wgpu::TEXT_EDITOR_TYPING_BUFFER_ARG), &run.surface_id)?;
        builder.string(Some(ui_wgpu::wgpu::TEXT_EDITOR_TYPING_COMMIT_ARG), reason)?;
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
fn drive_text_editor_typing_end(registry: &mut EngineSurfaceRegistry, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, now_ms: f64) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let ended = registry.slots.iter().position(|slot| {
        slot.value.as_ref().is_some_and(|surface| {
            let delivery = &surface.editor_delivery;
            delivery.active.is_none()
                && delivery.latest.is_none()
                && delivery.publication.is_none()
                && (delivery.typing_end.is_some() || delivery.typing.as_ref().is_some_and(|run| now_ms - run.last_ms >= ui_wgpu::wgpu::TEXT_EDITOR_TYPING_IDLE_MS as f64))
        })
    });
    let Some(index) = ended else { return Ok(false) };
    let delivery = &mut registry.slots[index].value.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.editor_delivery;
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

pub fn drive_text_editor_outbox_step(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
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
        let Some(slot) = registry.slots.get(index) else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
        let source = EngineSurfaceToken { slot: index as u16, generation: slot.generation };
        let surface = slot.value.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let snapshot = surface.editor_delivery.latest.clone().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let include_edit = !surface.editor_delivery.read_only && !surface.editor_delivery.guest_has_text(&snapshot.text);
        let include_selection = !surface.editor_delivery.selection_undeclared;
        if !include_edit && !include_selection {
            registry.slots[index].value.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.editor_delivery.latest = None;
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            return Ok(true);
        }
        if include_edit && snapshot.text.len() > ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY {
            let descriptor = ActionDescriptor {
                controller_id: snapshot.controller_id.clone(),
                action: "textEdit".into(),
                args: Some(semio_framework::DslValue::Object(vec![("surfaceId".into(), semio_framework::DslValue::String(snapshot.surface_id.clone())), (ui_wgpu::wgpu::TEXT_EDITOR_TYPING_BUFFER_ARG.into(), semio_framework::DslValue::String(snapshot.surface_id.clone()))])),
            };
            let action = ui_wgpu::wgpu::RetainedStringAction::new(descriptor, "text".into(), snapshot.text.len())?;
            let publication = TextEditorDeliveryPublication { source: snapshot.text.clone(), action, snapshot, include_selection };
            registry.slots[index].value.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.editor_delivery.publication = Some(publication);
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            return Ok(true);
        }
        if !include_edit {
            let run = registry.slots[index].value.as_ref().and_then(|surface| surface.editor_delivery.typing.clone());
            if let Some(run) = run {
                if !write_text_editor_typing_commit(input, &run, "selectionJump")? {
                    return Ok(false);
                }
                let delivery = &mut registry.slots[index].value.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?.editor_delivery;
                delivery.typing = None;
                delivery.typing_end = None;
            }
        }
        let token = text_editor_receipt_token(registry.next_text_editor_receipt, index).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let (edit_bytes, selection_bytes, batch_bytes) = text_editor_delivery_bytes(&snapshot, include_edit, include_selection)?;
        let mut batch = match input.reserve_actions(usize::from(include_edit) + usize::from(include_selection), batch_bytes) {
            Ok(batch) => batch,
            Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => return Ok(false),
            Err(fault) => return Err(fault),
        };
        if include_edit {
            batch.action(&snapshot.controller_id, "textEdit", edit_bytes, |builder| {
                builder.set_receipt(ui_wgpu::wgpu::ActionQueueReceipt { source: ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor, token, member: 0, abort_correlation_on_error: true })?;
                builder.begin_object(None)?;
                builder.string(Some("surfaceId"), &snapshot.surface_id)?;
                builder.string(Some("text"), &snapshot.text)?;
                builder.string(Some(ui_wgpu::wgpu::TEXT_EDITOR_TYPING_BUFFER_ARG), &snapshot.surface_id)?;
                builder.end_container()
            })?;
        }
        if include_selection {
            batch.action(&snapshot.controller_id, "textSelect", selection_bytes, |builder| {
                builder.set_receipt(ui_wgpu::wgpu::ActionQueueReceipt { source: ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor, token, member: 1, abort_correlation_on_error: false })?;
                builder.begin_object(None)?;
                builder.string(Some("surfaceId"), &snapshot.surface_id)?;
                builder.integer(Some("start"), snapshot.start as i64)?;
                builder.integer(Some("end"), snapshot.end as i64)?;
                builder.end_container()
            })?;
        }
        let next_sequence = registry.next_text_editor_receipt.checked_add(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
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

pub fn settle_text_editor_action_receipt(receipt: ui_wgpu::wgpu::ActionQueueReceipt, outcome: TextEditorActionOutcome<'_>) {
    if receipt.source != ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor { return; }
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
                let restored = serde_json::from_str::<ui_wgpu::wgpu::TextEditorScene>(scene_json).map_err(|error| error.to_string()).and_then(|editor| synchronize_text_editor_host(host, &editor, true));
                if let Err(fault) = restored {
                    engine_canvas_fault_log(&format!("[text-editor] the scene behind a refused edit could not be restored — {fault}"));
                }
            }
        }
    });
}

fn write_text_editor_selection(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, scene: &UiComponentSceneNode, host: &EditorHost) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let select_bytes = text_editor_action_bytes(scene, None)?;
    batch.action(&scene.controller_id, "textSelect", select_bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.integer(Some("start"), host.anchor() as i64)?;
        builder.integer(Some("end"), host.caret() as i64)?;
        builder.end_container()
    })
}

fn text_editor_action_bytes(scene: &UiComponentSceneNode, document_bytes: Option<usize>) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    let selection = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "textSelect", "surfaceId", &scene.surface_id, "start", "end"])?;
    let Some(document_bytes) = document_bytes else { return Ok(selection) };
    let edit = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "textEdit", "surfaceId", &scene.surface_id, "text"])?;
    selection.checked_add(edit).and_then(|bytes| bytes.checked_add(document_bytes)).filter(|bytes| *bytes <= ui_wgpu::wgpu::action::ACTION_QUEUE_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
}

fn write_text_editor_draft_value(builder: &mut ui_wgpu::wgpu::BoundedActionBuilder, key: Option<&str>, value: &Value) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    match value {
        Value::Null => builder.null(key),
        Value::Bool(value) => builder.boolean(key, *value),
        Value::Number(value) => builder.number(key, value.as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?),
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
pub fn text_editor_apply_explicit_draft_into(scene: &UiComponentSceneNode, _input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(settings) = text_editor_explicit_draft_settings(scene) else { return Ok(false) };
    ENGINE_SURFACES.with(|cell| {
        let mut registry = cell.borrow_mut();
        let Some(entry) = registry.get_mut(&scene.host_id) else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
        if !entry.editor_delivery.explicit_dirty || entry.editor_delivery.explicit_conflict || entry.editor_delivery.explicit_publication.is_some() || entry.editor_delivery.explicit_active.is_some() {
            return Ok(true);
        }
        let Some(host) = entry.editor.as_ref() else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure) };
        let text = entry.editor_delivery.explicit_snapshot.take().unwrap_or_else(|| host.text().to_owned());
        let (source, restore) = if settings.edit_argument == ui_wgpu::wgpu::DRAFT_SPLICES_ARGUMENT { (entry.editor_delivery.explicit_changes.clone().unwrap_or_else(|| "[]".to_owned()), Some(text)) } else { (text, None) };
        let args = settings.edit_arguments.iter().map(|(key, value)| (key.clone(), semio_framework::DslValue::from(value))).collect();
        let descriptor = ActionDescriptor { controller_id: scene.controller_id.clone(), action: settings.edit_action.clone(), args: Some(semio_framework::DslValue::Object(args)) };
        let action = match ui_wgpu::wgpu::RetainedStringAction::new(descriptor, settings.edit_argument.clone(), source.len()) {
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
    pub(crate) first: ui_wgpu::wgpu::QueuedActionDescriptor,
    pub(crate) second: Option<ui_wgpu::wgpu::QueuedActionDescriptor>,
}

/// 📄 Copies one bounded source page and yields the single artifact action only after completion.
pub fn drive_text_editor_retained_action_step() -> Result<TextEditorRetainedActionStep, ui_wgpu::wgpu::BoundedActionFault> {
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
        let surface = registry.slots[index].value.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if surface.editor_delivery.explicit_publication.is_none() {
            let publication = surface.editor_delivery.publication.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let begin = publication.action.copied_bytes();
            let mut end = begin.saturating_add(ui_wgpu::wgpu::RETAINED_ACTION_STRING_PAGE_BYTES).min(publication.source.len());
            while !publication.source.is_char_boundary(end) {
                end = end.checked_sub(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            }
            if publication.action.push_page(publication.source.get(begin..end).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?)? != ui_wgpu::wgpu::RetainedStringActionStep::Complete {
                registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
                return Ok(TextEditorRetainedActionStep::Pending);
            }
            let publication = surface.editor_delivery.publication.take().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let token = text_editor_receipt_token(registry.next_text_editor_receipt, index).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            registry.next_text_editor_receipt = registry.next_text_editor_receipt.checked_add(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let first = publication.action.finish(Some(ui_wgpu::wgpu::ActionQueueReceipt { source: ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor, token, member: 0, abort_correlation_on_error: true }))?;
            let second = publication.include_selection.then(|| ui_wgpu::wgpu::QueuedActionDescriptor {
                descriptor: ActionDescriptor {
                    controller_id: publication.snapshot.controller_id.clone(),
                    action: "textSelect".into(),
                    args: Some(semio_framework::DslValue::Object(vec![
                        ("surfaceId".into(), semio_framework::DslValue::String(publication.snapshot.surface_id.clone())),
                        ("start".into(), semio_framework::DslValue::uint(publication.snapshot.start as u64)),
                        ("end".into(), semio_framework::DslValue::uint(publication.snapshot.end as u64)),
                    ])),
                },
                receipt: Some(ui_wgpu::wgpu::ActionQueueReceipt { source: ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor, token, member: 1, abort_correlation_on_error: false }),
            });
            let slot = &mut registry.slots[index];
            if slot.generation != slot_generation {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
            let surface = slot.value.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
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
        let publication = surface.editor_delivery.explicit_publication.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let begin = publication.action.copied_bytes();
        let mut end = begin.saturating_add(ui_wgpu::wgpu::RETAINED_ACTION_STRING_PAGE_BYTES).min(publication.source.len());
        while !publication.source.is_char_boundary(end) {
            end = end.checked_sub(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        }
        if publication.action.push_page(publication.source.get(begin..end).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?)? != ui_wgpu::wgpu::RetainedStringActionStep::Complete {
            registry.text_editor_outbox_cursor = (index + 1) % ENGINE_SURFACE_CAPACITY;
            return Ok(TextEditorRetainedActionStep::Pending);
        }
        let publication = surface.editor_delivery.explicit_publication.take().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let token = text_editor_receipt_token(registry.next_text_editor_receipt, index).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        registry.next_text_editor_receipt = registry.next_text_editor_receipt.checked_add(1).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let receipt = ui_wgpu::wgpu::ActionQueueReceipt { source: ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor, token, member: 2, abort_correlation_on_error: false };
        let descriptor = publication.action.finish(Some(receipt))?;
        let slot = &mut registry.slots[index];
        if slot.generation != slot_generation {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        let surface = slot.value.as_mut().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
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
        let changes = entry.editor_delivery.explicit_changes.as_deref().and_then(|json| serde_json::from_str::<Vec<ui_wgpu::wgpu::DraftChange>>(json).ok()).unwrap_or_default();
        entry.editor_delivery.explicit_changes = Some(ui_wgpu::wgpu::draft_changes_json(&ui_wgpu::wgpu::compose_draft_step(&changes, host.text(), &next)));
        host.set_text(next.clone());
        entry.editor_delivery.explicit_dirty = entry.editor_delivery.explicit_base.as_deref().is_some_and(|base| next != base);
        entry.editor_delivery.explicit_snapshot = Some(next);
        entry.editor_delivery.explicit_error = None;
        true
    })
}

/// 📝 Reports whether the native editor has an unapplied explicit draft.
pub fn text_editor_explicit_draft_is_dirty(scene: &UiComponentSceneNode) -> bool {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(&scene.host_id).is_some_and(|entry| entry.editor_delivery.explicit_dirty))
}

/// ⚔️ Reports whether a persisted collaborator edit diverged from the local draft base.
pub fn text_editor_explicit_draft_has_conflict(scene: &UiComponentSceneNode) -> bool {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(&scene.host_id).is_some_and(|entry| entry.editor_delivery.explicit_conflict))
}

/// 📋️ Reads the exact UTF-8 selection owned by the mounted editor host.
pub fn text_editor_selection_text(scene: &UiComponentSceneNode) -> Option<String> {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(&scene.host_id).and_then(|entry| entry.editor.as_ref()).map(EditorHost::selection_text))
}

/// ♿️ Reads the mounted editor's optimistic local text for its accessibility value.
pub fn text_editor_accessibility_value(scene: &UiComponentSceneNode) -> Option<String> {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(&scene.host_id).and_then(|entry| entry.editor.as_ref()).map(|host| host.text().to_string()))
}

/// 🔒️ Reports the refusal-derived editor mode used by the accepted textarea projection.
pub fn text_editor_is_read_only(scene: &UiComponentSceneNode) -> bool {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(&scene.host_id).is_some_and(|entry| entry.editor_delivery.read_only))
}

/// 📋️ Replaces the mounted editor's current selection and publishes the same ordered edit/select
/// pair React's textarea host emits for paste, cut and a committed composition.
pub fn text_editor_replace_selection_into(scene: &UiComponentSceneNode, text: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    emit_text_editor_actions(
        scene,
        input,
        true,
        |host| {
            let source = host.text();
            let start = host.anchor().min(host.caret()).min(source.len());
            let end = host.anchor().max(host.caret()).min(source.len());
            source.len().saturating_sub(end - start).saturating_add(text.len())
        },
        |host| host.replace_selection(text),
    )
}

/// ♿️ Applies an accessibility textarea's complete value through the editor's ordered action pair.
pub fn text_editor_replace_all_into(scene: &UiComponentSceneNode, text: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    emit_text_editor_actions(
        scene,
        input,
        true,
        |_| text.len(),
        |host| {
            host.select_all();
            host.replace_selection(text);
        },
    )
}

pub fn text_editor_apply_key_into(scene: &UiComponentSceneNode, key: &KeyAction, modifiers: &PointerModifiers, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let supported = match key {
        KeyAction::Char(_) | KeyAction::Space(true) if !(modifiers.meta || modifiers.ctrl || modifiers.alt) => true,
        KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("a") => true,
        KeyAction::Backspace | KeyAction::Delete | KeyAction::Enter | KeyAction::Tab | KeyAction::ArrowLeft | KeyAction::ArrowRight | KeyAction::ArrowUp | KeyAction::ArrowDown | KeyAction::Home | KeyAction::End => true,
        _ => false,
    };
    if !supported {
        return Ok(false);
    }
    if matches!(key, KeyAction::Enter) {
        if let Some(gates) = scene.text_editor.as_ref().and_then(|editor| editor.newline_gates_json.as_deref()).filter(|gates| !gates.is_empty()) {
            let caret = text_editor_caret(scene).1;
            if !serde_json::from_str::<Vec<usize>>(gates).unwrap_or_default().contains(&caret) {
                return Ok(true);
            }
        }
    }
    let selection_only = matches!(key, KeyAction::ArrowLeft | KeyAction::ArrowRight | KeyAction::ArrowUp | KeyAction::ArrowDown | KeyAction::Home | KeyAction::End)
        || matches!(key, KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("a"));
    emit_text_editor_actions(
        scene,
        input,
        !selection_only,
        |host| {
            let text = host.text();
            let start = host.anchor().min(host.caret()).min(text.len());
            let end = host.anchor().max(host.caret()).min(text.len());
            match key {
                KeyAction::Char(ch) if !(modifiers.meta || modifiers.ctrl || modifiers.alt) => text.len().saturating_sub(end - start).saturating_add(ch.len()),
                KeyAction::Space(true) | KeyAction::Enter => text.len().saturating_sub(end - start).saturating_add(1),
                KeyAction::Tab => text.len().saturating_sub(end - start).saturating_add(host.tab_insert_text().len()),
                KeyAction::Backspace if start != end => text.len().saturating_sub(end - start),
                KeyAction::Backspace => text[..start].chars().next_back().map_or(text.len(), |ch| text.len().saturating_sub(ch.len_utf8())),
                KeyAction::Delete if start != end => text.len().saturating_sub(end - start),
                KeyAction::Delete => text[end..].chars().next().map_or(text.len(), |ch| text.len().saturating_sub(ch.len_utf8())),
                _ => text.len(),
            }
        },
        |host| match key {
            KeyAction::Char(ch) if !(modifiers.meta || modifiers.ctrl || modifiers.alt) => host.insert_text(ch),
            KeyAction::Space(true) => host.insert_text(" "),
            KeyAction::Enter => host.insert_text("\n"),
            KeyAction::Tab => host.insert_text(&host.tab_insert_text()),
            KeyAction::ArrowLeft => host.move_left(modifiers.shift),
            KeyAction::ArrowRight => host.move_right(modifiers.shift),
            KeyAction::ArrowUp => host.move_up(modifiers.shift),
            KeyAction::ArrowDown => host.move_down(modifiers.shift),
            KeyAction::Home => host.move_line_start(modifiers.shift),
            KeyAction::End => host.move_line_end(modifiers.shift),
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
    emit_text_editor_actions(scene, input, false, |host| host.text().len(), |host| host.select_span_at_screen(sx, sy))
}

pub fn text_editor_pointer_button_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, down: bool, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    emit_text_editor_actions(
        scene,
        input,
        false,
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
    emit_text_editor_actions(scene, input, false, |host| host.text().len(), |host| host.pointer_move_screen(sx, sy, 0))
}

pub fn text_editor_pointer_cancel_into(surface_id: &str) -> bool {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(host) = map.get_mut(surface_id).and_then(|entry| entry.editor.as_mut()) else { return false };
        host.pointer_cancel_screen();
        true
    })
}

pub fn text_editor_set_selection_into(scene: &UiComponentSceneNode, anchor: usize, caret: usize, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    emit_text_editor_actions(scene, input, false, |host| host.text().len(), |host| host.set_selection_range(anchor, caret))
}

pub fn text_editor_apply_completion_into(scene: &UiComponentSceneNode, prefix_start: usize, caret: usize, insert_text: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    emit_text_editor_actions(
        scene,
        input,
        true,
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
        let Some(entry) = map.get_mut(&scene.host_id) else {
            return Ok(false);
        };
        let Some(host) = entry.editor.as_mut() else {
            return Ok(false);
        };
        let selection_bytes = text_editor_action_bytes(scene, None)?;
        let mut batch = input.reserve_actions(2, selection_bytes.checked_mul(2).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?)?;
        host.pointer_down_screen(sx, sy, button as i32);
        write_text_editor_selection(&mut batch, scene, host)?;
        host.pointer_up_screen(sx, sy, 0);
        write_text_editor_selection(&mut batch, scene, host)?;
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
        let Some(entry) = map.get(&scene.host_id) else {
            return (0, 0);
        };
        let Some(host) = entry.editor.as_ref() else {
            return (0, 0);
        };
        (host.anchor(), host.caret())
    })
}

/** ✏️ Publishes the multi-span rename PREVIEW into the live `EditorHost`: the rewritten
 * buffer, the occurrence highlights and one extra caret per occurrence — React's
 * `updateRenamePreview` (`setText` + `setSelectionOccurrencesJson` + `setExtraCaretsJson` +
 * `renderFrame`, `🧱️elements/✏️TextEditor/🟦️.tsx:301-309`). Host-local only: nothing is dispatched
 * until the commit, so a cancelled rename is undone by re-previewing the committed buffer. */
pub fn text_editor_preview_rename(scene: &UiComponentSceneNode, text: &str, occurrences: &[(usize, usize)]) -> bool {
    let spans = serde_json::to_string(&occurrences.iter().map(|(start, end)| json!({ "start": start, "end": end })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
    let carets = serde_json::to_string(&occurrences.iter().map(|(start, _)| *start).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
    ENGINE_SURFACES
        .with(|cell| {
            let mut map = cell.borrow_mut();
            let entry = map.get_mut(&scene.host_id)?;
            let host = entry.editor.as_mut()?;
            host.set_text(text.to_string());
            host.set_selection_occurrences_json(&spans);
            host.set_extra_carets_json(&carets);
            entry.scene_revision = entry.scene_revision.wrapping_add(1);
            Some(true)
        })
        .unwrap_or(false)
}

/// 📍️ Screen-space caret position (surface-local, i.e. already offset by `inner.x/y`), for placing the
/// completions dropdown and the rename input near the caret — mirrors `WasmEditorSurface.caretScreenPosition`
/// (`caretWorldJson` + `worldToScreenJson`).
pub fn text_editor_caret_screen(scene: &UiComponentSceneNode, inner: Rect) -> Option<(f32, f32)> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(&scene.host_id)?;
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
