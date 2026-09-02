//! 🖊️ 2D geometry kernel: shared path-segment vocabulary, planar booleans, and bitmap autotrace.
//! 🪦 The scene-graph store (`DrawingStore`/`DrawingEngine`, SVG/PDF/DWG export) relocated to the
//! OS flow module's own drawing kernel — ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, superseded by `✳️drawing`'s
//! real `ArtifactStore` + 17 mutation triads + `🎛flattened-scene` inference.

pub use semio_framework_os_kernel::os_spr;

#[path = "../../⚙️engine/🦀️.rs"]
pub mod engine;
pub use engine::*;

#[path = "../../../../🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs"]
pub mod os_engine;
pub use os_engine::{Engine, EngineCache, EngineFault, EngineHandle as KernelEngineHandle, EngineKey};

#[cfg(feature = "booleans")]
#[path = "../../🔀️booleans/🦀️.rs"]
pub mod booleans;

#[cfg(feature = "trace")]
#[path = "../../🔍️trace/🦀️.rs"]
pub mod trace;
