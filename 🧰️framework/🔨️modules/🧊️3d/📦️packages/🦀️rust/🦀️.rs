//! 🌐️ Shared 3D s-module: domain-neutral B-Rep geometry-transfer types and a half-edge mesh
//! kernel. Scene math
//! (camera/frustum/picking) relocated to `semio-framework-ui`'s `kernel_3d_scene` mount (ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MESH) — it has zero
//! artifact-document inputs (a camera and a screen rect, not a snapshot), so it stays renderer
//! infrastructure rather than dissolving into an artifact, and this crate no longer mounts it.

//#region 🔖️Engine
/// @emoji 📐️ Domain-neutral geometry transfer types (`Vec3`/`Aabb`/`ParamDomain`/`FaceGroup`/
/// `MeshTransfer`/`PointClassification`) shared by this crate's own algorithm modules and by
/// framework-tier consumers (`semio-framework-os`, `os/🌊️flow/📐️brep-geometry`) that structurally
/// cannot depend on stdio. Its twelve sibling foundation modules (`vec`/`mat`/`tolerance`/
/// `predicates`/`poly`/`bezier`/`bspline`/`curve`/`curve_ops`/`surface`/`surface_ops`/`error`)
/// moved to `semio_s_plugin_stdio`'s `✳️brep/🧬️schema/📸️snapshot` artifact in ticket
/// 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4 — they had zero
/// production consumers outside the former `📐️brep/` directory (now dissolved, wave FINISH) and
/// stdio, confirmed by repo-wide grep before the move.
#[cfg(feature = "brep")]
#[path = "../../⚙️engine/🦀️.rs"]
pub mod engine;
//#endregion 🔖️Engine

//#region 🔖️Mesh
/// @emoji 🥽️ Half-edge mesh kernel: topology, editing ops, tessellation, UV/decimation.
#[path = "../../🥽️mesh/🦀️.rs"]
pub mod mesh;
//#endregion 🔖️Mesh

//#region 🔖️Rigid
/// @emoji 🌀️ Single-precision rigid-body algebra: vectors, points, unit quaternions, isometries —
/// the framework-owned replacement for the `nalgebra` surface plugins used to reach for directly.
#[path = "../../🌀️rigid/🦀️component.rs"]
pub mod rigid;
//#endregion 🔖️Rigid

//#region 🔖️Collision
/// @emoji 🧿️ BVH-accelerated triangle-mesh collision queries: shape-vs-shape intersection and
/// winding-number point containment — the framework-owned replacement for `parry3d::shape` plus
/// `parry3d::query::intersection_test`.
#[path = "../../🧿️collision/🦀️component.rs"]
pub mod collision;
//#endregion 🔖️Collision
