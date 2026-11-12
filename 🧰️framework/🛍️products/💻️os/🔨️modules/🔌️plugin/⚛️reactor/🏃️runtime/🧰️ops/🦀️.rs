//! ⚙️ Runtime operations of the plugin SDK: wire limits, codecs, document ops, media, surfaces, render, refresh, context menu, command ingress and typed operations.

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;

pub(crate) use semio_plugin_job_receiving::original_steps::{adopted,begun,built,built_with,retired,retired_poll,unbuilt};

#[cfg(test)]
use semio_framework_value_testing_allocator as _;

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

