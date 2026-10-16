//! 🌱️ Composition emission, metadata, root and genesis of the plugin SDK.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[path = "🪪️metadata/🦀️.rs"]
pub mod metadata;
#[path = "♻️root/🦀️.rs"]
pub mod root;
#[path = "📨️emission/🌱️genesis/📄️input/🦀️.rs"]
pub mod private_child_input;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
