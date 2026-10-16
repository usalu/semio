//! 🖥️ Native accessibility bridge and temporal adapters of the wgpu renderer.
//!
//! 🧭️ Native only: every window's accessibility projection as one AccessKit tree (NSAccessibility, UI Automation, AT-SPI) with AccessKit kept behind this crate. The module file stays in the target tree; this crate root mounts it as the public module `native_accessibility`. The host temporal formatter port arrives from the scenes package through `📥️lifted/🎞️Scenes`, and the temporal adapters behind it live in the public module `native_temporal`.

#[path = "../♿️native-accessibility/🦀️.rs"]
pub mod native_accessibility;

#[cfg(not(target_arch = "wasm32"))]
#[path = "📥️lifted/🎞️Scenes/🦀️.rs"]
mod host_temporal_formatter;
#[cfg(not(target_arch = "wasm32"))]
pub use host_temporal_formatter::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../🕰️native-temporal/🦀️.rs"]
pub mod native_temporal;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
