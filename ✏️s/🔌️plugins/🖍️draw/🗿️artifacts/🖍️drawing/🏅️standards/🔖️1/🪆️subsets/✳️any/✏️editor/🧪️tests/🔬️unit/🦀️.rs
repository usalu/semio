//! 🔬️ Drawing editor unit laws of the shared unit context: gestures, canvas and panels, utilities and windows.

#[path = "🧬️prelude/🦀️.rs"]
pub mod prelude;
pub use prelude::context;
use prelude::*;

#[path = "🧩️harness/🦀️.rs"]
mod harness;
use harness::*;

semio_plugin::history_edit_acceptance_law!("draw", DrawingPlayApp, || semio_plugin::App { definition: create_drawing_app(), examples: Vec::new() }, "../../../../../../../../..");

#[path = "✋️gesture-laws/🦀️.rs"]
mod gesture_laws;
#[path = "🖼️canvas-laws/🦀️.rs"]
mod canvas_laws;
#[path = "🧰️utility-laws/🦀️.rs"]
mod utility_laws;
#[path = "🪟️window-laws/🦀️.rs"]
mod window_laws;
