//! 🌐️ Shared 3D s-module: a half-edge mesh kernel, single-precision rigid-body algebra, inertia algebra and
//! BVH-accelerated collision queries. The boundary-representation kernel lives in its own crates
//! (`semio-framework-3d-brep-{representation,queries,operations,engine}`, wave E4 crate cut). Scene math (camera/frustum/picking) relocated to
//! `semio-framework-ui`'s `kernel_3d_scene` mount (ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MESH) — it has zero
//! artifact-document inputs (a camera and a screen rect, not a snapshot), so it stays renderer
//! infrastructure rather than dissolving into an artifact, and this crate no longer mounts it.
//!
//! Its `⚙️engine` B-Rep geometry-transfer types (`Vec3`/`Aabb`/`ParamDomain`/`FaceGroup`/
//! `MeshTransfer`/`PointClassification`) moved OUT in ticket
//! 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave 1 (W1-A): the stdio `🧊️brep` kernel now owns
//! its own neutral contract (`semio_s_plugin_stdio::…::subsets::brep::schema::engine::contract`)
//! instead of reaching back across this crate for it — this crate had zero production consumers
//! of those types left of its own (confirmed by repo-wide grep before the move).

// 🧬️ `#[derive(ToValue, FromValue)]` alias. This crate has no `semio-framework-os-kernel` dependency, so every
// `#[value(...)]` container names the value crate itself via `#[value(crate = "::semio_framework_value")]`.
extern crate semio_framework_value_derive as value_derive;

//#region 🔖️Mesh
/// 🥽️ Half-edge mesh kernel: topology, editing ops, tessellation, UV/decimation.
#[path = "../../🥽️mesh/🦀️.rs"]
pub mod mesh;
//#endregion 🔖️Mesh

//#region 🔖️Rigid
/// 🌀️ Single-precision rigid-body algebra: vectors, points, unit quaternions, isometries —
/// the framework-owned replacement for the `nalgebra` surface plugins used to reach for directly.
#[path = "../../🌀️rigid/🦀️.rs"]
pub mod rigid;
//#endregion 🔖️Rigid

//#region 🔖️Collision
/// 🧿️ BVH-accelerated triangle-mesh collision queries: shape-vs-shape intersection and
/// winding-number point containment — the framework-owned replacement for `parry3d::shape` plus
/// `parry3d::query::intersection_test`.
#[path = "../../🧿️collision/🦀️.rs"]
pub mod collision;
//#endregion 🔖️Collision

//#region 🔖️Inertia
/// 🛞️ Symmetric 3x3 eigen solver, principal inertia and parallel-axis algebra shared by the
/// B-Rep and mesh analysis queries.
#[path = "../../🛞️inertia/🦀️.rs"]
pub mod inertia;
//#endregion 🔖️Inertia

//#region 🧪️TestSupport
#[cfg(test)]
#[path = "../../🧪️tests/🧰️support/🦀️.rs"]
pub(crate) mod test_support;
//#endregion 🧪️TestSupport
