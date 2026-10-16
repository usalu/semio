//! 🖱️ Shell input, part one: pointer handling and the retained hit registry.
//!
//! The chrome build-state input impl, key-action translation, tree helpers, host window blur, the pointer
//! handlers, the retained hit registry and the presented-input candidate.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellInput

//#region 🫥️HostWindowBlur

/// 🫥️ The host window lost the system focus: every open text-editor typing run ends at once (`blur`, design §13.2 of ticket
/// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), and the shell's next drain blurs the focused pane for its program
/// ([`ShellState::arm_host_window_blur`]) — what React's pane `onBlur` does when the page loses focus.
pub fn note_host_window_blur() {
    crate::engine_canvas::end_every_text_editor_typing(crate::engine_canvas::TextEditorTypingEnd::Blur);
    *HOST_WINDOW_BLUR_PENDING.borrow_mut() = true;
}

/// 🫥️ The browser frame Worker's window-blur door (`host-window-blur`): the page's `window` lost focus.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuHostWindowBlur)]
pub fn semio_wgpu_host_window_blur() {
    note_host_window_blur();
}
//#endregion 🫥️HostWindowBlur

//#endregion ShellInput
