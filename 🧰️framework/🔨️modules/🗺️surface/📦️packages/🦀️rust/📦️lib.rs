//! 🗺️ The semio surface framework: one crate for every renderable 2D/GPU surface family.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

#[path = "../../🎨️paint/🦀️component.rs"]
pub mod paint;

#[path = "../../🎲️board-2d/🦀️component.rs"]
pub mod board_2d;

#[path = "../../🏔️terrain/🦀️component.rs"]
pub mod terrain;

// [DEBUG] temporarily disabled to isolate a pre-existing broken transitive dep (see registrar-handoff)
// #[path = "../../🕸️node-graph/🦀️component.rs"]
// pub mod node_graph;

#[path = "../../🗺️tiled-map/🦀️component.rs"]
pub mod tiled_map;
