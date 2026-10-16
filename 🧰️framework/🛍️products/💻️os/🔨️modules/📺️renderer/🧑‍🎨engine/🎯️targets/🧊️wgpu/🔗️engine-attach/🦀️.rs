//! 🔗️ Engine surface attach and per-kind scene sync.
//!
//! Attaching a retained scene to an engine surface and projecting its per-kind state: surface registrations, the node graph, tiled map,
//! board, raster and text editor scene syncs and the raster composite progress.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
extern crate infinite_canvas as infinite_world;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs"]
pub mod engine_surface_attach;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
