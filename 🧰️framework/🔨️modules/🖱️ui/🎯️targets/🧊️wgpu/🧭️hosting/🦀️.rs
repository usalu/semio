//! 🧭️ wgpu hosting: accessibility projection, cursors, clipboard host and scene slots.

#[path = "../♿️accessibility/🦀️.rs"]
pub mod accessibility;

#[path = "../👆️cursor/🦀️.rs"]
pub mod cursor;

#[path = "../🏃️host/🦀️.rs"]
#[cfg(not(target_os = "wasi"))]
pub mod host;

#[path = "../📨️scene_slots/🦀️.rs"]
pub mod scene_slots;
