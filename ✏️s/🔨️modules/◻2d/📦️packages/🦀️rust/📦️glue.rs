//! 🖊️ 2D drawing kernel: engine contracts, scene-graph store (SVG/PDF/DWG export), planar
//! booleans, and bitmap autotrace.

#[path = "../../⚙️engine/🦀️component.rs"]
pub mod engine;
pub use engine::*;

#[cfg(feature = "booleans")]
#[path = "../../🔀️booleans/🦀️component.rs"]
pub mod booleans;

#[cfg(feature = "trace")]
#[path = "../../🔍️trace/🦀️component.rs"]
pub mod trace;

#[path = "../../🗄️store/🦀️component.rs"]
mod store;
pub use store::DrawingStore;
