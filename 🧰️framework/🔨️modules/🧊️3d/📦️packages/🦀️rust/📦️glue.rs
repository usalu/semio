//! 🌐️ Shared 3D s-module: B-Rep kernel, half-edge mesh, and BVH spatial index. Scene math
//! (camera/frustum/picking) relocated to `semio-framework-ui`'s `kernel_3d_scene` mount (ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MESH) — it has zero
//! artifact-document inputs (a camera and a screen rect, not a snapshot), so it stays renderer
//! infrastructure rather than dissolving into an artifact, and this crate no longer mounts it.

//#region 🔖️Brep
/// @emoji 📐️ Boundary-representation kernel (native modules + native backend + engine contracts).
#[cfg(feature = "brep")]
#[path = "."]
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
    #[path = "../../📐️brep/⚙️engine/🦀️component.rs"]
    pub mod engine;
}
//#endregion 🔖️Brep

//#region 🔖️Mesh
/// @emoji 🥽️ Half-edge mesh kernel: topology, editing ops, tessellation, UV/decimation.
#[path = "../../🥽️mesh/🦀️component.rs"]
pub mod mesh;
//#endregion 🔖️Mesh

