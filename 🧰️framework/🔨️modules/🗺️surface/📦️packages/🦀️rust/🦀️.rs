//! 🗺️ The semio surface framework: one crate for every renderable 2D/GPU surface family.
//!
//! Each domain is a `🦀️.rs` in the owner tree; this entry file is pure wiring.

#[path = "../../🎨️paint/🦀️.rs"]
pub mod paint;

#[path = "../../🏔️terrain/🦀️.rs"]
pub mod terrain;

#[path = "../../🕸️node-graph/🦀️.rs"]
pub mod node_graph;

#[path = "../../🗺️tiled-map/🦀️.rs"]
pub mod tiled_map;
#[cfg(test)]
#[path = "../../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;
