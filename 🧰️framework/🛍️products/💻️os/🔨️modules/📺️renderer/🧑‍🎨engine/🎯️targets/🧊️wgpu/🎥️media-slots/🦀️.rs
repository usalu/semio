//! 🎥️ Presented media slots: bounded descriptors of the media a presented tree asks the host to composite, their identity registry and the retained collection cursors.
//!
//! 🧭️ Native and wasm32. The module tree stays in `🎬️media-slots`; this crate root mounts it as `media_slots` and re-exports its surface.

#[path = "../🎬️media-slots/🦀️.rs"]
mod media_slots;
pub use media_slots::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
