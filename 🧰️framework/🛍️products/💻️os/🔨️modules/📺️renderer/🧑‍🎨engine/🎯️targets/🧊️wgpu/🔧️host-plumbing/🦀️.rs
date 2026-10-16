//! 🔧️ Surface lane, input wire, render snapshot, deadlines, native caller and browser output adapters.
//!
//! 🧭️ Native and wasm32: the deadline scheduler, the native original-actor caller, the render snapshot with its output loan, the browser original output adapter, the surface lane job and the browser input wire. The module files keep their place in the renderer tree and are mounted here as public modules.

extern crate semio_framework_os_renderer_wgpu_media_slots as media_slots;

#[path = "../⏰️deadlines/🦀️.rs"]
pub mod deadlines;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../🎟️native-caller/🦀️.rs"]
pub mod native_caller;

#[path = "../📸️render-snapshot/🦀️.rs"]
pub mod render_snapshot;

#[path = "../🌐️browser-worker/📤️output/🦀️.rs"]
pub mod browser_original_output;

#[path = "../📐️surface-lane/🦀️.rs"]
pub mod surface_lane;

#[path = "../🎮️input-wire/🦀️.rs"]
pub mod input_wire;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
