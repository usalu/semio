//! ✍️ Text editor scene state, popups and popup drive.

use super::*;

//#region TextEditor
//#region State
/// 🗂️ Per-surface interaction state for double-click-to-select-word / completions / context-menu / rename.
///
/// 🔀️ Mid-session reconciliation note (concurrent `w2-scene-wiring` session): this region originally also
/// drove plain single-click-to-caret and drag-to-select itself, reading `ctx.input` pointer state directly
/// during render (mirroring the pre-existing focus-on-click code below), because at the time nothing
/// called `SceneInput::handle_scene_pointer_button`/`handle_scene_pointer_move` for any surface kind.
/// `w2-scene-wiring` landed `apply_scene_pointer` in `RenderEntry` next, calling those for every
/// non-bespoke surface kind including `✏️TextEditor` from a once-per-render-frame `InputState` sample —
/// and `w4-scene-input` (`.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`) has
/// since replaced THAT with a real per-event route (`ui_wgpu::wgpu::UiCommand::Scene` ->
/// `interpreter::apply_scene_ui_command`, calling the same two handlers), deleting `apply_scene_pointer`
/// itself. Plain click/drag still reaches `EditorHost` via that generic path today, just per real event
/// now rather than sampled once per frame. That single-click/drag code was removed here to avoid
/// double-dispatching `textSelect`/`textEdit`; what remains below (double-click word-select, right-click
/// context menu, completions, rename) is *not* covered by the generic path and
/// stays. 🐛️➡️✅️ W4 fix (`.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`):
/// `EditorHost::pointer_down_screen` used to no-operate entirely for `button != 0`, so both the generic
/// path's raw-button-passthrough call AND this region's own right-click handling had to force `button`
/// to `0` to reposition the caret at all. `pointer_down_screen` now repositions the caret for every
/// button (only a primary press also starts a drag-selection), so this region's `pointer_down`/
/// `pointer_up` pair below passes the real button through instead of forcing it.
/** ✍️ One text-editor surface's POPUP state — the completions dropdown, the multi-span rename
 * draft and the double-click edge, i.e. exactly the `useState` bucket React's `TextEditor` keeps
 * beside its wasm session (`completionsOpen`/`completionIndex`/`renameDraft`/`renamePosition`,
 * `🧱️elements/✏️TextEditor/🟦️.tsx:267-330`). The BUFFER, selection, carets and tokens are the
 * `EditorHost`'s own state and are never mirrored here.
 *
 * Per surface rather than per window: two editor panes of one window each own their popups, the same
 * way two mounted `TextEditor` hosts each own their own React state. */

fn mutate_text_editor_ui<R: Default>(host_id: &str, apply: impl FnOnce(&mut TextEditorUiState) -> R) -> R {
    SCENE_STATE.with(|cell| cell.borrow_mut().get_or_insert_with(host_id.to_owned(), SceneSurfaceState::default).map(|state| apply(&mut state.text_editor_ui)).unwrap_or_default())
}

/// 📋️ Mirrors `CompletionItem` (`🧱️elements/✏️TextEditor/🟦️.tsx`); `insertText` has no producer yet
/// anywhere in the codebase (`jack_completions_json` only ever emits `label`/`detail`) so
/// `insert_text` falls back to `label`, exactly like `identifierPrefixStart`/`applyCompletion` do on
/// the React side.
#[derive(Clone, Deserialize)]
struct TextEditorCompletionItem {
    pub(crate) label: String,
    #[serde(default)]
    pub(crate) detail: Option<String>,
    #[serde(default, rename = "insertText")]
    insert_text: Option<String>,
}

#[derive(Clone, Copy, Deserialize)]
struct TextEditorSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

/// ✏️ Mirrors `RenameInfo` (`🧱️elements/✏️TextEditor/🟦️.tsx`), parsed from `TextEditorScene::rename_json`.
#[derive(Clone, Deserialize)]
struct TextEditorRenameInfo {
    pub(crate) name: String,
    pub(crate) occurrences: Vec<TextEditorSpan>,
}

fn text_editor_completions(editor: &semio_framework_ui_declaration::ui::TextEditorScene) -> Vec<TextEditorCompletionItem> {
    editor.completions_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default()
}

fn text_editor_rename_info(editor: &semio_framework_ui_declaration::ui::TextEditorScene) -> Option<TextEditorRenameInfo> {
    editor.rename_json.as_deref().and_then(|json| serde_json::from_str(json).ok())
}

/// ✂️ Identifier-prefix scan back from `caret` — `identifierPrefixStart`
/// (`🧱️elements/✏️TextEditor/🟦️.tsx:98`) — the replacement range a completion commit overwrites.
fn identifier_prefix_start(text: &str, caret: usize) -> usize {
    let bytes = text.as_bytes();
    let mut start = caret.min(bytes.len());
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    start
}

/// 📏️ `[start, end)` byte range of the line containing `offset` — `lineRangeAt`
/// (`🧱️elements/✏️TextEditor/🟦️.tsx:91`), the "Select Line" menu row's own range.
fn text_editor_line_range(text: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(text.len());
    let start = text[..clamped].rfind('\n').map_or(0, |index| index + 1);
    let end = text[clamped..].find('\n').map_or(text.len(), |index| clamped + index);
    (start, end)
}

/// ✏️ `multiSpanReplace` (`🧱️elements/✏️TextEditor/🟦️.tsx:80`): rewrites every occurrence back-to-front
/// so earlier spans keep their offsets, and answers the post-rewrite spans in document order.
fn multi_span_replace(text: &str, occurrences: &[(usize, usize)], next_name: &str) -> (String, Vec<(usize, usize)>) {
    let mut sorted: Vec<(usize, usize)> = occurrences.to_vec();
    sorted.sort_by(|left, right| right.0.cmp(&left.0));
    let mut out = text.to_string();
    let mut next = Vec::with_capacity(sorted.len());
    for (start, end) in sorted {
        if start > out.len() || end > out.len() || start > end {
            continue;
        }
        out.replace_range(start..end, next_name);
        next.insert(0, (start, start + next_name.len()));
    }
    (out, next)
}

//#region 🍿️Popups
/// 📍️ Anchor (page space) for the completions dropdown: near the caret, falling back to a fixed
/// offset if the host isn't ready yet — mirrors React's `renamePosition ? … : { left: 12, top: 12 }`.
fn text_editor_popup_anchor(scene: &UiComponentSceneNode, inner: Rect) -> (f32, f32) {
    semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_caret_screen(scene, inner).unwrap_or((inner.x + 12.0, inner.y + 12.0))
}

fn text_editor_completion_row_rect(anchor: (f32, f32), theme: &Theme, index: usize) -> Rect {
    Rect::new(anchor.0, anchor.1 + 18.0 + index as f32 * theme.control_height_small, TEXT_EDITOR_COMPLETION_WIDTH, theme.control_height_small)
}

/// 📋️ The completion row under a point, for a click-to-commit press on the dropdown.
fn text_editor_completion_hit(scene: &UiComponentSceneNode, inner: Rect, theme: &Theme, count: usize, x: f32, y: f32) -> Option<usize> {
    let anchor = text_editor_popup_anchor(scene, inner);
    (0..count).find(|&index| text_editor_completion_row_rect(anchor, theme, index).contains(x, y))
}

const TEXT_EDITOR_COMPLETION_WIDTH: f32 = 220.0;
const TEXT_EDITOR_RENAME_WIDTH: f32 = 180.0;
/// 🧾️ The retained-item grant one editor overlay paint needs: the two popup frames plus a bounded
/// completion list (the dropdown is a dropdown, not a document — React never scrolls it either).
pub const TEXT_EDITOR_OVERLAY_ITEMS: usize = 256;

/** 🍿️ Paints the editor's popup chrome ABOVE the composited `EditorHost` texture: the
 * completions dropdown and the live rename input. Drawn in the scene's own last cursor phase, so it
 * lands after `push_raster_quad` staged the editor texture and therefore over it — the wgpu
 * equivalent of React's absolutely-positioned `z-50` overlays inside the host element
 * (`🧱️elements/✏️TextEditor/🟦️.tsx:503-608`).
 *
 * The CONTEXT MENU is deliberately not painted here: a right-click on this surface resolves through
 * `scene_context_menu_target` and opens the SHELL's own context-menu chrome
 * (`ShellState::open_context_menu`), which is what carries React's title row, submenu nesting,
 * keyboard navigation and dismissal rules. The seven rows React handles locally rather than
 * dispatching to the guest are executed by [`text_editor_local_menu_action`] from that menu. */
pub fn render_text_editor_overlays(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let Some(editor) = scene.text_editor.as_ref() else {
        return;
    };
    if let Some((action, x, y)) = mutate_text_editor_ui(&scene.host_id, |state| state.pending_menu_action.take()) {
        if let Err(fault) = text_editor_local_menu_action(scene, bounds, &action, x, y, ctx.input) {
            ctx.input.record_action_fault(fault);
        }
    }
    let ui = text_editor_ui(&scene.host_id);
    if let Some(draft) = ui.rename.as_ref() {
        render_text_editor_rename_input(scene, bounds, ctx, &draft.text);
    }
    render_text_editor_explicit_draft_controls(scene, bounds, ctx);
    if !ui.completions_open {
        return;
    }
    let completions = text_editor_completions(editor);
    render_text_editor_completions(scene, bounds, ctx, &completions, ui.completion_index);
}

fn text_editor_explicit_draft_button_rects(bounds: Rect, theme: &Theme) -> (Rect, Rect) {
    let width = 96.0;
    let gap = 8.0;
    let height = theme.control_height_small;
    let y = bounds.y + bounds.h - height - 8.0;
    let apply = Rect::new(bounds.x + bounds.w - width - 8.0, y, width, height);
    let discard = Rect::new(apply.x - width - gap, y, width, height);
    (discard, apply)
}

fn render_text_editor_explicit_draft_controls(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let Some(settings) = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_settings(scene) else { return };
    let dirty = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_is_dirty(scene);
    let conflicted = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_has_conflict(scene);
    let status = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_status(scene);
    let (discard, apply) = text_editor_explicit_draft_button_rects(bounds, ctx.theme);
    let footer = Rect::new(bounds.x, discard.y - 8.0, bounds.w, discard.h + 16.0);
    ctx.draw.push_rounded([footer.x, footer.y, footer.w, footer.h], ctx.theme.panel, 0.0);
    if conflicted {
        draw_text(ctx, &settings.conflict_label, footer.x + 8.0, footer.y + footer.h * 0.62, ctx.theme.font_size_small, ctx.theme.error);
    } else if let Some(error) = status.error.as_deref() {
        draw_text(ctx, &format!("{}: {error}", settings.failed_label), footer.x + 8.0, footer.y + footer.h * 0.62, ctx.theme.font_size_small, ctx.theme.error);
    } else if status.pending {
        draw_text(ctx, &format!("{} {}%", settings.applying_label, (status.progress * 100.0).round() as u32), footer.x + 8.0, footer.y + footer.h * 0.62, ctx.theme.font_size_small, ctx.theme.text_muted);
    }
    let apply_id = if status.cancellable { "cancel" } else { "apply" };
    let apply_label = if status.cancellable {
        settings.cancel_label.as_str()
    } else if status.pending {
        settings.applying_label.as_str()
    } else {
        settings.apply_label.as_str()
    };
    for (id, rect, label, fill, enabled) in
        [("discard", discard, settings.discard_label.as_str(), ctx.theme.panel, !status.pending && (dirty || conflicted)), (apply_id, apply, apply_label, ctx.theme.accent, status.cancellable || !status.pending && dirty && !conflicted)]
    {
        ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], fill, ctx.theme.border_radius * 0.5);
        draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, ctx.theme.panel_border, 1.0);
        draw_text(ctx, label, rect.x + 8.0, rect.y + rect.h * 0.68, ctx.theme.font_size_small, if enabled { ctx.theme.text } else { ctx.theme.text_muted });
        ctx.input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("{}.editor.draft.{id}", scene.host_id)), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    }
}

/// 📋️ The completions dropdown — React's `rounded border border-border bg-popover p-1 shadow-md`
/// list, active row in `bg-accent text-accent-foreground`.
fn render_text_editor_completions(scene: &UiComponentSceneNode, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>, completions: &[TextEditorCompletionItem], active_index: usize) {
    if completions.is_empty() {
        return;
    }
    let theme = ctx.theme;
    let anchor = text_editor_popup_anchor(scene, inner);
    let pad = 4.0;
    let row_h = theme.control_height_small;
    let container = Rect::new(anchor.0 - pad, anchor.1 + 18.0 - pad, TEXT_EDITOR_COMPLETION_WIDTH + pad * 2.0, completions.len() as f32 * row_h + pad * 2.0);
    ctx.draw.push_rounded([container.x, container.y, container.w, container.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, container.x, container.y, container.w, container.h, theme.panel_border, 1.0);
    for (index, item) in completions.iter().enumerate() {
        let row = text_editor_completion_row_rect(anchor, theme, index);
        let (bg, fg) = if index == active_index { (theme.accent, theme.active_foreground) } else { (theme.panel, theme.text) };
        ctx.draw.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius * 0.5);
        draw_text(ctx, &item.label, row.x + 8.0, row.y + row.h * 0.68, theme.font_size_small, fg);
        if let Some(detail) = &item.detail {
            let label_w = item.label.len() as f32 * theme.font_size_small * 0.6;
            let detail_fg = if index == active_index { fg } else { theme.text_muted };
            draw_text(ctx, detail, row.x + 12.0 + label_w, row.y + row.h * 0.68, theme.font_size_small, detail_fg);
        }
        ctx.input.register_hit(HitTarget { rect: row, event: None, control_id: Some(format!("{}.editor.completion.{index}", scene.host_id)), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    }
}

/// ✏️ The rename input — React's `border border-border` glass field pinned near the caret, carrying
/// the draft name as the multi-span preview is typed.
fn render_text_editor_rename_input(scene: &UiComponentSceneNode, inner: Rect, ctx: &mut FrameworkWidgetContext<'_>, text: &str) {
    let theme = ctx.theme;
    let (x, y) = text_editor_popup_anchor(scene, inner);
    let rect = Rect::new(x, (y - theme.control_height_small * 0.5).max(inner.y), TEXT_EDITOR_RENAME_WIDTH, theme.control_height_small);
    ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, theme.panel_border, 1.0);
    draw_text(ctx, text, rect.x + 8.0, rect.y + rect.h * 0.68, theme.font_size_small, theme.text);
    ctx.input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("{}.editor.rename", scene.host_id)), kind: HitKind::Input, drag_axis: None, drag_data: None });
}
//#endregion 🍿️Popups

//#region ✍️PopupDrive
/** 📋️ Opens the completions dropdown at the first row — React's `openCompletions`, which is a
 * no-operation when the scene carries no completions. `true` when the popup actually opened, so a
 * caller can report the key as consumed the way `event.preventDefault()` does. */
pub fn text_editor_open_completions(scene: &UiComponentSceneNode) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    if text_editor_completions(editor).is_empty() {
        return false;
    }
    mutate_text_editor_ui(&scene.host_id, |state| {
        state.completions_open = true;
        state.completion_index = 0;
    });
    true
}

pub fn text_editor_completions_open(surface_id: &str) -> bool {
    text_editor_ui(surface_id).completions_open
}

pub fn text_editor_close_completions(surface_id: &str) {
    mutate_text_editor_ui(surface_id, |state| {
        state.completions_open = false;
        state.completion_index = 0;
    });
}

/// 📋️ Wraps the dropdown highlight — React's `(index ± 1 + length) % length`.
pub fn text_editor_move_completion(scene: &UiComponentSceneNode, down: bool) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    let count = text_editor_completions(editor).len();
    if count == 0 {
        return false;
    }
    mutate_text_editor_ui(&scene.host_id, |state| {
        state.completion_index = if down { (state.completion_index + 1) % count } else { (state.completion_index + count - 1) % count };
    });
    true
}

/** ✅️ Commits the highlighted completion — `applyCompletion`: replace
 * `[identifier_prefix_start(text, caret), caret)` with the item's insert text, publish the buffer as
 * `textEdit`, and close the dropdown. */
pub fn text_editor_apply_completion(scene: &UiComponentSceneNode, index: Option<usize>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(editor) = scene.text_editor.as_ref() else {
        return Ok(false);
    };
    let completions = text_editor_completions(editor);
    if completions.is_empty() {
        return Ok(false);
    }
    let active = index.unwrap_or_else(|| text_editor_ui(&scene.host_id).completion_index);
    let Some(item) = completions.get(active).or_else(|| completions.first()) else {
        return Ok(false);
    };
    let (_, caret) = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_caret(scene);
    let prefix_start = identifier_prefix_start(&editor.buffer, caret);
    let insert = item.insert_text.clone().unwrap_or_else(|| item.label.clone());
    let applied = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_apply_completion_into(scene, prefix_start, caret, &insert, input)?;
    text_editor_close_completions(&scene.host_id);
    Ok(applied)
}

/** ✏️ Arms the multi-span rename — React's `startRename`: the draft carries the scene's own
 * `rename_json` occurrences and current name, and the input takes keyboard focus so the next
 * keystrokes type the new name instead of editing the buffer. */
pub fn text_editor_start_rename(scene: &UiComponentSceneNode, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let Some(info) = scene.text_editor.as_ref().and_then(text_editor_rename_info) else {
        return false;
    };
    let occurrences: Vec<(usize, usize)> = info.occurrences.iter().map(|span| (span.start, span.end)).collect();
    input.focus_input_owned(format!("{}.editor.rename", scene.host_id), info.name.clone());
    mutate_text_editor_ui(&scene.host_id, |state| {
        state.rename = Some(TextEditorRenameDraft { occurrences, text: info.name.clone() });
        state.completions_open = false;
    });
    true
}

pub fn text_editor_rename_active(surface_id: &str) -> bool {
    text_editor_ui(surface_id).rename.is_some()
}

/** ✏️ Retypes the rename draft and republishes the multi-span PREVIEW into the editor host —
 * React's `updateRenamePreview` (`setText` + `setSelectionOccurrencesJson` + `setExtraCaretsJson`).
 * The preview is host-local; nothing is dispatched until the commit. */
pub fn text_editor_update_rename(scene: &UiComponentSceneNode, next_text: &str) -> bool {
    let Some(editor) = scene.text_editor.as_ref() else {
        return false;
    };
    let Some(draft) = text_editor_ui(&scene.host_id).rename else {
        return false;
    };
    let (preview, occurrences) = multi_span_replace(&editor.buffer, &draft.occurrences, next_text);
    semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_preview_rename(scene, &preview, &occurrences);
    mutate_text_editor_ui(&scene.host_id, |state| {
        if let Some(draft) = state.rename.as_mut() {
            draft.text = next_text.to_string();
        }
    });
    true
}

/// ✏️ Commits the rename — `commitRename { occurrences, text }`, React's own payload — and disarms
/// the draft.
pub fn text_editor_commit_rename(scene: &UiComponentSceneNode, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(draft) = text_editor_ui(&scene.host_id).rename else {
        return Ok(false);
    };
    let action = semio_framework_ui_declaration::ui::text_editor_actions::COMMIT_RENAME;
    let spans = serde_json::to_string(&draft.occurrences.iter().map(|(start, end)| json!({ "start": start, "end": end })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into());
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[&scene.controller_id, action, "surfaceId", &scene.surface_id, "text", &draft.text, "occurrences", &spans])?;
    let mut reservation = input.reserve_action(&scene.controller_id, action, bytes)?;
    {
        let builder = reservation.builder();
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.string(Some("text"), &draft.text)?;
        builder.begin_array(Some("occurrences"))?;
        for (start, end) in &draft.occurrences {
            builder.begin_object(None)?;
            builder.number(Some("start"), *start as f64)?;
            builder.number(Some("end"), *end as f64)?;
            builder.end_container()?;
        }
        builder.end_container()?;
        builder.end_container()?;
    }
    reservation.publish()?;
    mutate_text_editor_ui(&scene.host_id, |state| state.rename = None);
    Ok(true)
}

/// ✏️ Cancels the rename and restores the committed buffer into the host — React's `cancelRename`.
pub fn text_editor_cancel_rename(scene: &UiComponentSceneNode) -> bool {
    if text_editor_ui(&scene.host_id).rename.is_none() {
        return false;
    }
    if let Some(editor) = scene.text_editor.as_ref() {
        semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_preview_rename(scene, &editor.buffer, &[]);
    }
    mutate_text_editor_ui(&scene.host_id, |state| state.rename = None);
    true
}

/** 🖱️ The context-menu rows a text editor answers ITSELF instead of dispatching to the guest —
 * React's `localActions` map (`🧱️elements/✏️TextEditor/🟦️.tsx:419-470`). `Ok(false)` means the row is
 * not local and the caller must dispatch it as a normal action.
 *
 * `cut`/`copy`/`paste` are absent on purpose: they are `document.execCommand` on the React side and
 * this target has no OS clipboard binding at all (`ui_wgpu::wgpu::events`' `UiCommand::Clipboard*`
 * still says the read/write is an unwired host concern), so pretending to handle them would swallow
 * the row instead of leaving it dispatchable. */
pub fn text_editor_local_menu_action(scene: &UiComponentSceneNode, inner: Rect, action: &str, x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(editor) = scene.text_editor.as_ref() else {
        return Ok(false);
    };
    match action {
        "requestCompletions" | "suggestCompletions" => {
            text_editor_open_completions(scene);
        }
        "selectToken" => {
            semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_select_span_into(scene, inner, x, y, input)?;
        }
        "selectLine" => {
            let (_, caret) = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_caret(scene);
            let (start, end) = text_editor_line_range(&editor.buffer, caret);
            semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_set_selection_into(scene, start, end, input)?;
        }
        "selectAll" => {
            semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_set_selection_into(scene, 0, editor.buffer.len(), input)?;
        }
        "commitRename" | "rename" => {
            if !text_editor_start_rename(scene, input) {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/** 🖱️ Parks one context-menu row for this surface to answer itself on the next paint, and
 * answers whether the row IS local. The shell calls this before dispatching a menu row: `true` means
 * the row never reaches the guest, exactly as React's `dispatchTextEditorMenu` short-circuits an id
 * present in its own `localActions` map. */
pub fn text_editor_queue_menu_action(surface_id: &str, action: &str, x: f32, y: f32) -> bool {
    if !TEXT_EDITOR_LOCAL_MENU_ACTIONS.contains(&action) {
        return false;
    }
    mutate_text_editor_ui(surface_id, |state| state.pending_menu_action = Some((action.to_string(), x, y)));
    true
}

/// 🖱️ The rows a text editor answers itself — React's `localActions` keys, minus the three clipboard
/// verbs this target has no OS binding for (see [`text_editor_local_menu_action`]).
const TEXT_EDITOR_LOCAL_MENU_ACTIONS: [&str; 6] = ["requestCompletions", "suggestCompletions", "selectToken", "selectLine", "selectAll", "commitRename"];

/// ✏️ An armed rename input has KEYBOARD FOCUS, so it consumes every key: React's `<input>` is a
/// real focused element and `onKeyDown` stops propagation. The draft text is tracked here rather
/// than read back off `InputState::text_view` — that is a PAGED projection that is empty until the
/// text pump has run, so a rename would have previewed the empty string on its first keystroke.
/** ⌨️ The popup half of one keystroke, offered BEFORE the buffer sees it — React's
 * `onKeyDown` prelude (`🧱️elements/✏️TextEditor/🟦️.tsx:556-608`): `Ctrl/Cmd+Space` opens completions,
 * `F2` starts a rename, and while either popup is open its own arrow/commit/dismiss keys win over
 * editing. `false` leaves the key to `semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_apply_key_into`. */
pub fn text_editor_popup_key(scene: &UiComponentSceneNode, key: &KeyAction, modifiers: &semio_framework_ui_wgpu_input::PointerModifiers, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let ui = text_editor_ui(&scene.host_id);
    let accelerator = modifiers.ctrl || modifiers.meta;
    if semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_settings(scene).is_some() {
        if accelerator && matches!(key, KeyAction::Char(ch) if ch.eq_ignore_ascii_case("z")) {
            return Ok(semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_move_explicit_draft_history(scene, modifiers.shift));
        }
        if modifiers.ctrl && matches!(key, KeyAction::Char(ch) if ch.eq_ignore_ascii_case("y")) {
            return Ok(semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_move_explicit_draft_history(scene, true));
        }
        if accelerator && matches!(key, KeyAction::Enter) {
            return semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_apply_explicit_draft_into(scene, input);
        }
        if matches!(key, KeyAction::Escape) {
            if semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_cancel_explicit_publication(scene) {
                return Ok(true);
            }
            return Ok(semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_discard_explicit_draft(scene));
        }
    }
    if let Some(draft) = ui.rename.as_ref() {
        match key {
            KeyAction::Escape => return Ok(text_editor_cancel_rename(scene)),
            KeyAction::Enter => return text_editor_commit_rename(scene, input),
            KeyAction::Backspace => {
                let mut next = draft.text.clone();
                next.pop();
                text_editor_update_rename(scene, &next);
            }
            KeyAction::Char(ch) if !accelerator && ch.chars().count() == 1 => {
                text_editor_update_rename(scene, &format!("{}{ch}", draft.text));
            }
            _ => {}
        }
        return Ok(true);
    }
    if ui.completions_open {
        match key {
            KeyAction::ArrowDown => return Ok(text_editor_move_completion(scene, true)),
            KeyAction::ArrowUp => return Ok(text_editor_move_completion(scene, false)),
            KeyAction::Enter | KeyAction::Tab => return text_editor_apply_completion(scene, None, input),
            KeyAction::Escape => {
                text_editor_close_completions(&scene.host_id);
                return Ok(true);
            }
            _ => {}
        }
    }
    if accelerator && matches!(key, KeyAction::Space(_)) {
        return Ok(text_editor_open_completions(scene));
    }
    if matches!(key, KeyAction::Function(2)) {
        return Ok(text_editor_start_rename(scene, input));
    }
    Ok(false)
}

/** 🖱️ The popup half of one press. An ALT-click with completions available opens the dropdown
 * (React's `event.altKey && completions.length > 0` branch in `onContextMenu`); a press on an open
 * dropdown row commits that row; a press anywhere else closes it, the way an outside press dismisses
 * any popup. `false` leaves the press to the caret/selection path. */
pub fn text_editor_popup_pointer(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, alt: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(editor) = scene.text_editor.as_ref() else {
        return Ok(false);
    };
    if semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_settings(scene).is_some() {
        let (discard, apply) = text_editor_explicit_draft_button_rects(inner, &scene_input_theme());
        if discard.contains(x, y) {
            if semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_status(scene).pending {
                return Ok(true);
            }
            return Ok(semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_discard_explicit_draft(scene));
        }
        if apply.contains(x, y) {
            let status = semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_explicit_draft_status(scene);
            if status.pending && !status.cancellable {
                return Ok(true);
            }
            if semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_cancel_explicit_publication(scene) {
                return Ok(true);
            }
            return semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_apply_explicit_draft_into(scene, input);
        }
    }
    if text_editor_ui(&scene.host_id).completions_open {
        let count = text_editor_completions(editor).len();
        if let Some(index) = text_editor_completion_hit(scene, inner, &scene_input_theme(), count, x, y) {
            return text_editor_apply_completion(scene, Some(index), input);
        }
        text_editor_close_completions(&scene.host_id);
        return Ok(false);
    }
    if alt {
        return Ok(text_editor_open_completions(scene));
    }
    Ok(false)
}
//#endregion ✍️PopupDrive

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-text-editor/🦀️.rs"]
mod text_editor_tests;
#[cfg(any(test, feature = "testkit"))]
pub fn cursor_from_click(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, scroll: f32) -> usize {
    let Some(editor) = &scene.text_editor else {
        return 0;
    };
    let line_h = 18.0;
    let line_index = ((y - inner.y - 8.0 + scroll) / line_h).max(0.0) as usize;
    let lines: Vec<&str> = editor.buffer.lines().collect();
    let line = lines.get(line_index).copied().unwrap_or("");
    let rel_x = (x - inner.x - 8.0).max(0.0);
    let mut cursor = 0usize;
    let mut width = 0.0f32;
    for (index, ch) in line.chars().enumerate() {
        let advance = if ch == '\t' { 8.0 } else { 7.0 };
        if width + advance * 0.5 > rel_x {
            cursor = index;
            break;
        }
        width += advance;
        cursor = index + 1;
    }
    lines.iter().take(line_index).map(|l| l.len() + 1).sum::<usize>() + cursor
}

#[cfg(any(test, feature = "testkit"))]
pub fn line_col_at(text: &str, cursor: usize) -> (usize, usize) {
    let mut index = 0usize;
    for (line_index, line) in text.lines().enumerate() {
        let next = index + line.len() + 1;
        if cursor < next {
            return (line_index, cursor.saturating_sub(index));
        }
        index = next;
    }
    let line_count = text.lines().count();
    (line_count.saturating_sub(1), 0)
}

//#endregion State
//#endregion TextEditor
