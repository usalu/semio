//! 🧊️ Renderer library package adapter.

#[cfg(not(target_os = "wasi"))]
include!("../../../🧊️renderer/🦀️.rs");

#[cfg(target_os = "wasi")]
pub extern crate semio_framework_os_renderer_wgpu_mailbox_core as runtime_mailbox_core;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../../../⌨️native-entrypoint/🦀️.rs"]
mod native_entrypoint;

#[cfg(not(target_arch = "wasm32"))]
pub use native_entrypoint::run_native_entrypoint;

#[cfg(test)]
#[path = "../../../../../🧪️tests/🎨️wgpu-paint-policy/🦀️.rs"]
mod paint_policy_tests;

#[cfg(all(test, not(target_os = "wasi")))]
#[global_allocator]
static RENDERER_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
