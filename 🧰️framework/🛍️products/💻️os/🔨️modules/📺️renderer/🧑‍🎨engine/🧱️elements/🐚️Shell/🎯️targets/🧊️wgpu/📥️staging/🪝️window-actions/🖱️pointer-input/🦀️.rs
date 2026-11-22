//! 🪝️ `shell-window-actions` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
//#region 🫥️HostWindowBlur
/// 🫥️ Whether the host window lost the system focus since the shell last looked (native winit `Focused(false)`, the browser
/// page's `window` blur through the `semioWgpuHostWindowBlur` door). A worker cell, so a law's thread owns its own.
pub static HOST_WINDOW_BLUR_PENDING: semio_renderer_wgpu_contracts::WorkerCell<bool> = semio_renderer_wgpu_contracts::WorkerCell::new(Default::default);

//#endregion 🫥️HostWindowBlur
//#endregion ShellInput

//#region ShellInput
//#region 🚪️HostWindowLifecycle
/// 🚪️ The host window's lifecycle notes the shell answers on its next drain: a close the person asked for while a document holds edits the store never acknowledged, the discard that answers it, and the OS files dropped onto the window.
#[derive(Default)]
pub struct HostWindowLifecycle {
    unsaved: bool,
    close_requested: bool,
    close_approved: bool,
    dropped: Vec<std::path::PathBuf>,
    dropped_files: Vec<HostPickedFileRow>,
    local_catalog: Option<String>,
}

/// 🛬️ One OS file the browser page holds for the shell: the page's token for the `File`, its name, byte size and MIME claim.
pub type HostPickedFileRow = (u32, String, u64, String);

/// 🚪️ A worker cell, so a law's thread owns its own lifecycle notes.
pub static HOST_WINDOW_LIFECYCLE: semio_renderer_wgpu_contracts::WorkerCell<HostWindowLifecycle> = semio_renderer_wgpu_contracts::WorkerCell::new(Default::default);

/// 🚪️ The id of the close dialog on the chrome dialog stack.
pub const CLOSE_GUARD_DIALOG_ID: &str = "os.closeGuard";

/// 🚪️ The action the close dialog's "Close and discard" dispatches (the framework controller answers it by approving the close).
pub const CLOSE_DISCARD_ACTION_ID: &str = "os.closeDiscard";

/// 🔴️ The shell publishes whether leaving now could lose edits — React's unload predicate: a document that needs saving is not acknowledged saved, or has no store attachment.
pub fn publish_host_unsaved(unsaved: bool) {
    HOST_WINDOW_LIFECYCLE.borrow_mut().unsaved = unsaved;
}

/// 🔴️ Whether the shell last published that leaving could lose edits; the browser page asks before it unloads.
pub fn host_unsaved() -> bool {
    HOST_WINDOW_LIFECYCLE.borrow().unsaved
}

/// 🚪️ The host asks to close the window (winit `CloseRequested`): `true` closes at once, `false` holds the close until the shell's dialog answers.
pub fn request_host_close() -> bool {
    let mut lifecycle = HOST_WINDOW_LIFECYCLE.borrow_mut();
    if !lifecycle.unsaved {
        return true;
    }
    lifecycle.close_requested = true;
    false
}

/// 🚪️ Takes the held close request: the shell opens its close dialog for it.
pub fn take_host_close_request() -> bool {
    std::mem::take(&mut HOST_WINDOW_LIFECYCLE.borrow_mut().close_requested)
}

/// 🚪️ The person chose "Close and discard".
pub fn approve_host_close() {
    HOST_WINDOW_LIFECYCLE.borrow_mut().close_approved = true;
}

/// 🚪️ Takes the approval the host closes the window on.
pub fn take_host_close_approval() -> bool {
    std::mem::take(&mut HOST_WINDOW_LIFECYCLE.borrow_mut().close_approved)
}

/// 🛬️ An OS file was dropped onto the window (winit `DroppedFile`); the shell opens it like Open File.
pub fn note_host_file_drop(path: std::path::PathBuf) {
    HOST_WINDOW_LIFECYCLE.borrow_mut().dropped.push(path);
}

/// 🛬️ Takes the dropped files in drop order.
pub fn take_host_file_drops() -> Vec<std::path::PathBuf> {
    std::mem::take(&mut HOST_WINDOW_LIFECYCLE.borrow_mut().dropped)
}

/// 🛬️ OS files the browser page took from a drop (`semioWgpuHostFileDrop`, a JSON list of `{token, name, size, mime}`); an unreadable list drops nothing.
pub fn note_host_file_drop_json(json: &str) {
    #[derive(serde::Deserialize)]
    struct Row {
        token: u32,
        name: String,
        size: u64,
        #[serde(default)]
        mime: String,
    }
    let Ok(rows) = serde_json::from_str::<Vec<Row>>(json) else { return };
    HOST_WINDOW_LIFECYCLE.borrow_mut().dropped_files.extend(rows.into_iter().map(|row| (row.token, row.name, row.size, row.mime)));
}

/// 🗂️ The page's local document catalog (`semioWgpuHostLocalCatalog`, the snapshot JSON of the store facet) for the shell's next drain.
pub fn note_host_local_catalog_json(json: &str) {
    HOST_WINDOW_LIFECYCLE.borrow_mut().local_catalog = Some(json.to_string());
}

/// 🗂️ Takes the page's latest local catalog snapshot, if it published one since the last drain.
pub fn take_host_local_catalog() -> Option<String> {
    HOST_WINDOW_LIFECYCLE.borrow_mut().local_catalog.take()
}

/// 🛬️ Takes the browser page's dropped files in drop order.
pub fn take_host_picked_files() -> Vec<HostPickedFileRow> {
    std::mem::take(&mut HOST_WINDOW_LIFECYCLE.borrow_mut().dropped_files)
}

/// 🚪️ Whether a lifecycle note awaits the shell's next drain.
pub fn host_window_lifecycle_pending() -> bool {
    let lifecycle = HOST_WINDOW_LIFECYCLE.borrow();
    lifecycle.close_requested || !lifecycle.dropped.is_empty() || !lifecycle.dropped_files.is_empty() || lifecycle.local_catalog.is_some()
}
//#endregion 🚪️HostWindowLifecycle
//#endregion ShellInput

//#region ShellInput
/// 🌳️ Retained controls whose complete pointer gesture belongs to `EventRouter`.
pub fn retained_router_owns_pointer(kind: HitKind) -> bool {
    matches!(kind, HitKind::Button | HitKind::Input | HitKind::Select | HitKind::Toggle | HitKind::Slider | HitKind::NumberStepper | HitKind::Ring | HitKind::IconSelect | HitKind::TreeItem | HitKind::TreeDragHandle | HitKind::ComponentScene)
}
//#endregion ShellInput

//#region ShellInput
//#region 🫥️HostWindowBlur
/// 🫥️ The host window lost the system focus: every open text-editor typing run ends at once (`blur`, design §13.2 of ticket
/// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), and the shell's next drain blurs the focused pane for its program
/// ([`ShellState::arm_host_window_blur`]) — what React's pane `onBlur` does when the page loses focus.
pub fn note_host_window_blur() {
    semio_renderer_wgpu_engine_text::text_editor::end_every_text_editor_typing(semio_renderer_wgpu_engine_registry::engine_surface_registry::TextEditorTypingEnd::Blur);
    *HOST_WINDOW_BLUR_PENDING.borrow_mut() = true;
}
//#endregion 🫥️HostWindowBlur
//#endregion ShellInput
