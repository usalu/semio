//! 🗺️ The semio surface framework: one crate for every renderable 2D/GPU surface family.
//!
//! Each domain is a `🦀️.rs` in the owner tree; this entry file is pure wiring.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
pub use dsl::os_dsl;
#[path = "../../🎨️paint/🦀️.rs"]
pub mod paint;

#[path = "../../🏔️terrain/🦀️.rs"]
pub mod terrain;

#[path = "../../🕸️node-graph/🦀️.rs"]
pub mod node_graph;

#[path = "../../🗺️tiled-map/🦀️.rs"]
pub mod tiled_map;
