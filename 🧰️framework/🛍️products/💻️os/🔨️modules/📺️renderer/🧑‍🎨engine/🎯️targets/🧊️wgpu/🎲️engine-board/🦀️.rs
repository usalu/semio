//! 🎲️ Board2d, paint2d and tiled map hosts of an engine surface.
//!
//! The board2d host, its coalesced event queue and pointer settle, the paint2d layers, context menu targets and pointer handlers, and the
//! tiled map host with its tile fetch reservation.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_asset_authority as asset_authority;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs"]
pub mod board2d;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🖌️paint2d/🦀️.rs"]
pub mod paint2d;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗺️tiled-map/🦀️.rs"]
pub mod tiled_map;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
