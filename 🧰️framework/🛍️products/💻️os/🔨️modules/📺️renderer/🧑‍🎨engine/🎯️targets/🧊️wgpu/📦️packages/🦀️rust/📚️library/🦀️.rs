//! 🧊️ Renderer library package adapter.

#[cfg(not(target_os = "wasi"))]
#[path = "../../../🧊️renderer/📇️registry/🦀️.rs"]
mod renderer_registration;

#[cfg(not(target_os = "wasi"))]
include!("../../../🧊️renderer/🦀️.rs");

#[cfg(target_os = "wasi")]
#[path = "../../../📮️runtime-mailbox-core/🦀️.rs"]
pub mod runtime_mailbox_core;
