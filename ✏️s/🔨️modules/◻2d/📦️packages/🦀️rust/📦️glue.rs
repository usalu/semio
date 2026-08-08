//! 🖊️ 2D drawing kernel: engine contracts, scene-graph store (SVG/PDF/DWG export), planar
//! booleans, and bitmap autotrace.

pub use semio_framework_os_kernel::os_spr;

#[path = "../../⚙️engine/🦀️component.rs"]
pub mod engine;
pub use engine::*;

#[path = "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs"]
pub mod os_engine;
pub use os_engine::{Engine, EngineCache, EngineFault, EngineHandle as KernelEngineHandle, EngineKey};

#[cfg(feature = "booleans")]
#[path = "../../🔀️booleans/🦀️component.rs"]
pub mod booleans;

#[cfg(feature = "trace")]
#[path = "../../🔍️trace/🦀️component.rs"]
pub mod trace;

#[path = "../../🗄️store/🦀️component.rs"]
mod store;
pub use store::{DrawingEngine, DrawingStore};
