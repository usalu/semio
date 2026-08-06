//! 🌐️ Shared 3D s-module: B-Rep kernel, half-edge mesh, scene math, and BVH spatial index.

//#region 🔖️Brep
/// @emoji 📐️ Boundary-representation kernel (native modules + brepkit backend + engine contracts).
#[cfg(feature = "brep")]
pub mod brep {
    #[path = "../../📐️brep/🚨️error/🦀️component.rs"]
    pub mod error;
    #[path = "../../📐️brep/➡️vector/🦀️component.rs"]
    pub mod vec;
    #[path = "../../📐️brep/🔢️matrix/🦀️component.rs"]
    pub mod mat;
    #[path = "../../📐️brep/📏️tolerance/🦀️component.rs"]
    pub mod tolerance;
    #[path = "../../📐️brep/⚖️predicates/🦀️component.rs"]
    pub mod predicates;
    #[path = "../../📐️brep/🔮️oracle/🦀️component.rs"]
    pub mod oracle;
    #[path = "../../📐️brep/〰️polynomial/🦀️component.rs"]
    pub mod poly;
    #[path = "../../📐️brep/🎢️bezier/🦀️component.rs"]
    pub mod bezier;
    #[path = "../../📐️brep/🪢️bspline/🦀️component.rs"]
    pub mod bspline;
    #[path = "../../📐️brep/➰️curve/🦀️component.rs"]
    pub mod curve;
    #[path = "../../📐️brep/✂️curve-ops/🦀️component.rs"]
    pub mod curve_ops;
    #[path = "../../📐️brep/🏄️surface/🦀️component.rs"]
    pub mod surface;
    #[path = "../../📐️brep/🪡️surface-ops/🦀️component.rs"]
    pub mod surface_ops;
    #[path = "../../📐️brep/🏟️arena/🦀️component.rs"]
    pub mod arena;
    #[path = "../../📐️brep/📜️history/🦀️component.rs"]
    pub mod history;
    #[path = "../../📐️brep/🕸️topology/🦀️component.rs"]
    pub mod topo;
    #[path = "../../📐️brep/🔺️euler/🦀️component.rs"]
    pub mod euler;
    #[path = "../../📐️brep/✅️validate/🦀️component.rs"]
    pub mod validate;
    #[path = "../../📐️brep/⚙️engine/🦀️component.rs"]
    pub mod engine;
    #[path = "../../📐️brep/🧰️kernel/🦀️component.rs"]
    pub mod kernel;
}
//#endregion 🔖️Brep

//#region 🔖️Mesh
#[path = "../../🥽️mesh/🦀️component.rs"]
pub mod mesh;
pub use mesh::*;
//#endregion 🔖️Mesh

//#region 🔖️Scene
#[path = "../../🎬️scene/🦀️component.rs"]
pub mod scene;
pub use scene::*;
//#endregion 🔖️Scene

//#region 🔖️Spatial
#[cfg(feature = "brep")]
#[path = "../../🗺️spatial/🦀️component.rs"]
pub mod spatial;
#[cfg(feature = "brep")]
pub use spatial::*;
//#endregion 🔖️Spatial

