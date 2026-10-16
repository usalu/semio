//! 📦️ Runtime instance registry, descriptor, opening and cleanup pumps of the plugin runtime.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[path = "🏷️metadata-owner/🦀️.rs"]
pub mod inst_metadata_owner;
#[path = "🖼️wire-frames/🦀️.rs"]
pub mod inst_wire_frames;
#[path = "🪪️descriptor/🦀️.rs"]
pub mod inst_descriptor;
#[path = "🚪️opening/🦀️.rs"]
pub mod inst_opening;
#[path = "🔁️cleanup-turns/🦀️.rs"]
pub mod inst_cleanup_turns;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
