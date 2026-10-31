//! 🖐️ Puzzle3d editor gesture laws — example switching, mutation latency, gesture kinds, selection scale, the transform tool machine and the demo session asset.

#![allow(clippy::result_large_err)]

include!("../🧰️prelude/🦀️.rs");

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
#[path = "../🔬️example-switch/🦀️.rs"]
mod example_switch;
#[path = "../🔬️pane-smoke/🦀️.rs"]
mod pane_smoke;
#[path = "../🔬️mutation-latency/🦀️.rs"]
mod mutation_latency;
#[path = "../🔬️gesture-kinds/🦀️.rs"]
mod gesture_kinds;
#[path = "../🔬️selection-scale/🦀️.rs"]
mod selection_scale;
#[path = "../🧪️transform-tool/🦀️.rs"]
mod transform_tool;
#[path = "../../📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod demo_session_example;
