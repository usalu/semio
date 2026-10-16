//! 🪢️ Program bridge: the plugin bridge for wasm C-ABI modules (browser JS loader and wasmtime host on the kernel crates), the retained program exchange and the native runtime module manifest readers.
//!
//! 🧭️ Native and wasm32. The `🌉️ProgramBridge` module file keeps its place in the element tree and the native runtime module readers theirs in the native entrypoint tree; this crate root mounts them and re-exports their surface.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate semio_framework_os_kernel as store;
use semio_framework_os_renderer_wgpu_host_base::{frame_latency, plugin_catalog, run_renderer_io};

#[path = "../../../🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"]
mod program_bridge;
pub use program_bridge::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../⌨️native-entrypoint/📦️modules/🦀️.rs"]
pub mod native_runtime_modules;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
