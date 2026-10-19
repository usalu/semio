//! 🚪️ Native host I/O retains its original invocation, worker, result and physical cleanup.

#[cfg(not(target_arch="wasm32"))]
#[path="🎟️invocation/🦀️.rs"]
mod original_invocation;
#[cfg(not(target_arch="wasm32"))]
pub use original_invocation::RendererOriginalIoNode;

#[cfg(not(target_arch="wasm32"))]
#[path="🎟️source/🦀️.rs"]
mod original_source;
#[cfg(not(target_arch="wasm32"))]
pub use original_source::{RendererIoPath,RendererSourceIoHandle,RendererSourceIoError,RendererSourceIoFailureKind,open_renderer_source_io,read_renderer_source_bytes,read_renderer_resident_bytes,find_renderer_wasm_path,start_renderer_modified,renderer_source_io_pending};
#[cfg(all(test,not(target_arch="wasm32")))]
pub use original_source::original_native_source_test_guard;

#[cfg(not(target_arch="wasm32"))]
pub fn pump_renderer_io_sessions(maximum:usize)->usize{original_source::pump_renderer_source_io_sessions(maximum)}
