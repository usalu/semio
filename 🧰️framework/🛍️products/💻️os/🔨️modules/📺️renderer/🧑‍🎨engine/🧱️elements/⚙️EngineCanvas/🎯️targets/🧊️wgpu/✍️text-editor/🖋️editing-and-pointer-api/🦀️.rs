//! 🖋️ Editing and pointer API: the draft queries, the selection replacement, the key, span, pointer and completion entry points, the caret and the rename preview. See [`super`].

use super::*;

//#region TextEditor
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
pub fn text_editor_replace_selection_into(scene: &UiComponentSceneNode, text: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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
pub fn text_editor_replace_all_into(scene: &UiComponentSceneNode, text: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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

pub fn text_editor_apply_key_into(scene: &UiComponentSceneNode, key: &KeyAction, modifiers: &PointerModifiers, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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

pub fn text_editor_select_span_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let sx = (x - inner.x) as f64;
    let sy = (y - inner.y) as f64;
    emit_text_editor_actions(scene, input, false, |host| host.text().len(), |host| host.select_span_at_screen(sx, sy))
}

pub fn text_editor_pointer_button_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, down: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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

pub fn text_editor_pointer_move_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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

pub fn text_editor_set_selection_into(scene: &UiComponentSceneNode, anchor: usize, caret: usize, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    emit_text_editor_actions(scene, input, false, |host| host.text().len(), |host| host.set_selection_range(anchor, caret))
}

pub fn text_editor_apply_completion_into(scene: &UiComponentSceneNode, prefix_start: usize, caret: usize, insert_text: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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

pub fn text_editor_pointer_click_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
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
        let mut batch = input.reserve_actions(2, selection_bytes.checked_mul(2).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?)?;
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
