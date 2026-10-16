//! 🪝️ `shell-window-actions` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
//#region 🫥️HostWindowBlur
/// 🫥️ Whether the host window lost the system focus since the shell last looked (native winit `Focused(false)`, the browser
/// page's `window` blur through the `semioWgpuHostWindowBlur` door). A worker cell, so a law's thread owns its own.
pub static HOST_WINDOW_BLUR_PENDING: semio_framework_os_renderer_wgpu_contracts::WorkerCell<bool> = semio_framework_os_renderer_wgpu_contracts::WorkerCell::new(Default::default);

//#endregion 🫥️HostWindowBlur
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
    semio_framework_os_renderer_wgpu_engine_text::text_editor::end_every_text_editor_typing(semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::TextEditorTypingEnd::Blur);
    *HOST_WINDOW_BLUR_PENDING.borrow_mut() = true;
}
//#endregion 🫥️HostWindowBlur
//#endregion ShellInput
