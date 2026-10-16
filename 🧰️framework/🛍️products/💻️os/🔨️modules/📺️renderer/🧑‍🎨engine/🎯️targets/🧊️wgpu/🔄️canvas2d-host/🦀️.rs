//! 📐️ Canvas2d gumball: the wgpu twin of the Canvas2d transform gumball overlay (press hit test, gesture dispatch, paint).
//!
//! The press and move dispatch into the app's open tool transaction and the overlay paint; the handle geometry, the gesture algebra and the
//! cancellation slot live with the scene state this crate builds on.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

#[path = "../../../🧱️elements/📐️Canvas2dHost/🎯️targets/🧊️wgpu/🦀️.rs"]
mod gumball;
pub use gumball::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
