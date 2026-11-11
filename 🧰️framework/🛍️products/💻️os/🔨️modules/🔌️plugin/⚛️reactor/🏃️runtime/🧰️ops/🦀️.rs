//! ⚙️ Runtime operations of the plugin SDK: wire limits, codecs, document ops, media, surfaces, render, refresh, context menu, command ingress and typed operations.

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;

pub(crate) use semio_plugin_job_receiving::original_steps::{adopted,begun,built,built_with,retired,retired_poll,unbuilt};

/// 🧮️ Weighs the allocator events that the paid-receipt laws of this crate's own test binary compare with their receipts.
#[cfg(test)]
#[global_allocator]
static RUNTIME_OPS_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

pub use semio_plugin_runtime_ops_wire as ops_wire_limits;
pub use semio_plugin_runtime_ops_document as ops_document;
#[path = "🎞️media/🦀️.rs"]
pub mod ops_media;
pub use semio_plugin_runtime_ops_surface as ops_surface_render;
#[path = "📥️ingress/🦀️.rs"]
pub mod ops_ingress;
#[path = "🧾️typed-operations/🦀️.rs"]
pub mod ops_typed_operations;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
#[cfg(test)]
extern crate semio_os_kernel as dsl;

