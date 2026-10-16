//! ⚓️ Dock model, panel anchors and window measures.
//!
//! Panel anchors, the dock tree and skeleton, dock UI persistence helpers and the window-measures projection
//! with its `impl ShellState` publication half.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellLifecycle

//#region 📏️WindowMeasures

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📏️wgpu-window-measures/🦀️.rs"]
mod window_measures_tests;
//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle
