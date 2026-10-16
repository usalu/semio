//! 🕸️ Node graph interaction of an engine surface.
//!
//! Node graph camera, catalogue drag and drop, wheel, pointer, connection cursor, labels and overlays of an engine surface.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
pub use semio_framework_os_renderer_wgpu_host_base::action_args_json;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🦀️.rs"]
pub mod node_graph;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
