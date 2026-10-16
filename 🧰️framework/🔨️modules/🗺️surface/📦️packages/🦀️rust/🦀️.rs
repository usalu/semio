//! 🗺️ The semio surface framework facade: one library for every renderable 2D/GPU surface family.
//!
//! Each family is its own producer crate with its own gate budget; this entry file only re-exports them under their former module names.

pub use semio_framework_surface_node_graph as node_graph;
pub use semio_framework_surface_paint as paint;
pub use semio_framework_surface_terrain as terrain;
pub use semio_framework_surface_tiled_map as tiled_map;
