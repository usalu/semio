//! ⚙️ Runtime operations of the plugin SDK: wire limits, codecs, document ops, media, surfaces, render, refresh, context menu, command ingress and typed operations.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[path = "🛡️wire-limits/🦀️.rs"]
pub mod ops_wire_limits;
#[path = "📄️document/🦀️.rs"]
pub mod ops_document;
#[path = "🎞️media/🦀️.rs"]
pub mod ops_media;
#[path = "🖥️surface-render/🦀️.rs"]
pub mod ops_surface_render;
#[path = "📥️ingress/🦀️.rs"]
pub mod ops_ingress;
#[path = "🧾️typed-operations/🦀️.rs"]
pub mod ops_typed_operations;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
#[cfg(test)]
extern crate semio_framework_os_kernel as dsl;
