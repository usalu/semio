//! 🗺️ Engine surface registry, prepared engine canvas and tutorial surface geometry: the bottom of EngineCanvas.
//!
//! The per-worker engine surface registry (surfaces, tokens, sync caches, retirement), the text editor delivery state, the published
//! graph interaction and the engine diagnostics every engine family reads, the prepared engine canvas packets and the tutorial geometry.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_asset_authority as asset_authority;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs"]
pub mod engine_surface_registry;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/📦️prepared-engine-canvas/🦀️.rs"]
pub mod prepared_engine_canvas;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🎓️tutorial-surface-geometry/🦀️.rs"]
pub mod tutorial_surface_geometry;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️fixtures/🦀️.rs"]
pub mod fixtures;
#[cfg(any(test, feature = "testkit"))]
pub use fixtures::{engine_surface_law_guard, with_engine_close_context, ENGINE_SURFACE_LAW_LOCK};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
