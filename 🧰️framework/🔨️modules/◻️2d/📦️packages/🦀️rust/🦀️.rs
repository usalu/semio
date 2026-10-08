//! 🖊️ 2D geometry kernel: shared path-segment vocabulary, planar booleans, and bitmap autotrace.
//! 🪦 The scene-graph store (`DrawingStore`/`DrawingEngine`, SVG/PDF/DWG export) relocated to the
//! OS flow module's own drawing kernel — ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, superseded by `✳️drawing`'s
//! real `ArtifactStore` + 17 mutation triads + `🎛flattened-scene` inference.


#[path = "../../⚙️engine/🦀️.rs"]
pub mod engine;
pub use engine::*;

#[path = "../../🧮️compute/🦀️.rs"]
pub mod compute;

#[cfg(feature = "booleans")]
#[path = "../../🔀️booleans/🦀️.rs"]
pub mod booleans;

#[cfg(feature = "booleans")]
#[path = "../../🧱️regions/🦀️.rs"]
pub mod regions;

#[cfg(feature = "trace")]
#[path = "../../🔍️trace/🦀️.rs"]
pub mod trace;

#[path = "../../📝️text/🦀️.rs"]
pub mod text;

#[path = "../../🛤️path/📏️flatten/🦀️.rs"]
pub mod flatten;

#[path = "../../🛤️path/🖊️stroke/🦀️.rs"]
pub mod stroke;

#[path="../../🧹️retire/🦀️.rs"]
pub mod retirement;
