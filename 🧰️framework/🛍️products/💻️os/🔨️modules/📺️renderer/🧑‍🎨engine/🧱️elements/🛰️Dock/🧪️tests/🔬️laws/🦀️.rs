//! 🧪️ Companion white-box laws of `semio-framework-os-renderer-wgpu-dock`.

#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]


#[cfg(not(target_arch = "wasm32"))]
#[path = "../🪟️app-mode-layouts/🦀️.rs"]
mod app_mode_layout_tests;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../🪟️window-template-drag/🦀️.rs"]
mod window_template_drag_tests;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../🎯️stack-drop-destination/🦀️.rs"]
mod stack_drop_destination_tests;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../🔬️wgpu-unit/🦀️.rs"]
mod unit_tests;
