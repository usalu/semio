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

/// 🫥️ The browser frame Worker's window-blur door (`host-window-blur`): the page's `window` lost focus.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuHostWindowBlur)]
pub fn semio_wgpu_host_window_blur() {
    note_host_window_blur();
}
//#endregion 🫥️HostWindowBlur

//#endregion ShellInput
