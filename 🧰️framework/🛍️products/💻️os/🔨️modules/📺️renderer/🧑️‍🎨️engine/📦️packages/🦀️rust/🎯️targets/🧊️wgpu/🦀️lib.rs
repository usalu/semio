//! 🧪️ Renderer target boundary.
//!
//! Browser and native presentation compile only where `winit` has a platform backend. WASI keeps
//! the renderer's owned bounded-mailbox protocol available without linking a window system.

#[cfg(target_arch = "wasm32")]
extern crate semio_framework_async as wasm_bindgen_futures;

#[cfg(not(target_os = "wasi"))]
#[macro_export]
macro_rules! action_args_json {
    ($($tt:tt)*) => {
        semio_framework::optional_json_to_dsl(Some(serde_json::json!($($tt)*)))
    };
}

#[cfg(not(target_os = "wasi"))]
include!("🦀️.rs");

#[cfg(target_os = "wasi")]
#[path = "🦀️runtime_mailbox_core.rs"]
pub mod runtime_mailbox_core;
