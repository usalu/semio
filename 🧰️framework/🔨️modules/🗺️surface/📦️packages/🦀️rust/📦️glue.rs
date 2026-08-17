//! 🗺️ The semio surface framework: one crate for every renderable 2D/GPU surface family.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as dsl;
pub use dsl::os_dsl;
#[path = "../../🎨️paint/🦀️component.rs"]
pub mod paint;

#[path = "../../🏔️terrain/🦀️component.rs"]
pub mod terrain;

#[path = "../../🕸️node-graph/🦀️component.rs"]
pub mod node_graph;

#[path = "../../🗺️tiled-map/🦀️component.rs"]
pub mod tiled_map;
