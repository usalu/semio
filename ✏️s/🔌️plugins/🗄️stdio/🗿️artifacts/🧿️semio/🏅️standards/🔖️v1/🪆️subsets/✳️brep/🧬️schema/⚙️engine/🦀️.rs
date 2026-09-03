//! 🧠 Native B-Rep kernel: consumer contract (`BrepKernel`, `GeometryHandle`, `GeometryKind`,
//! `block_on`) plus its sole implementor `Brep`, which delegates every operation to
//! `semio_framework_3d::engine::*`'s pure algorithm modules over a `&mut Body`/`&Body` arena.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/{⚙️engine,🧰️kernel}` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave G5 ("the brep flip") — this is
//! the temporary forward edge (`stdio → semio-framework-3d`) that lets the other framework-3d
//! brep subdirs (arena, topology, boolean, tessellate, euler, …) peel into `✳️brep`'s compute
//! subdirs one at a time without ever touching a consumer.
//!
//! 📐️ `🔖️contract` (below) moved IN ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave
//! 1 (W1-A): `MeshTransfer`/`Vec3`/`Aabb`/`ParamDomain`/`FaceGroup`/`PointClassification` no
//! longer live in `semio_framework_3d::engine` — they, plus the new `EdgeGroup`/`FaceInfo`/
//! `EdgeInfo`/`SurfaceKind`/`CurveKind`/`OpQuality` types the CAD renderer bridge and the
//! Phase 0/1 capability audit need, are this file's own neutral contract now. Every remaining
//! `semio_framework_3d::engine::*` algorithm-module return/accept site across the repo was
//! repointed at this module in the same wave.
//!
//! `📦️mesh-io` (below) moved IN wave DEDUP: it was brep↔mesh bridging/IO code whose only real
//! consumer was already this file, and its DWG calls were the last framework-tier caller of the
//! (now-deleted) `semio_framework::mesh_to_dwg_drawing`/`dwg_from_bytes`/`dwg_to_bytes` re-exports —
//! moving it here (instead of pointing framework-3d at stdio's real `dwg` artifact, which would be
//! an actual crate cycle given the forward edge above) dissolves that dependency entirely.
//!
//! 📄️ `📄️step` (below) moved IN wave PEEL3: framework-3d's hand-rolled ISO 10303-21 Part-21
//! reader/writer, the last thing this file's `export_step`/`import_step` needed from the
//! framework side. Known pre-existing duplicate of stdio's own, separately-complete AP214
//! `SemioBrepToStep`/`SemioBrepFromStep` walk (`✳️brep/🚪️io`) — reconciling the two requires
//! rewiring this file's `BrepKernel` impl, which is out of scope here (see `📌️important.md`,
//! "BrepKernel — do NOT attempt"). Relocated verbatim only, to satisfy the crate-direction law
//! now that arena/topo/euler/history/primitives moved into this same crate too.

use std::collections::HashMap;


#[path = "📦️mesh-io/🦀️.rs"]
mod mesh_io;

#[path = "📄️step/🦀️.rs"]
mod step;

#[path = "🔖️contract/🦀️.rs"]
pub mod contract;
pub use contract::*;

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::blend::{chamfer_edges, fillet_edges, fillet_variable};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::boolean::{boolean_solid, compound_cut, section_solid_by_plane, split_solid_by_plane, BooleanOp};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::make_vertex;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect::intersect_curve_curve;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect::intersect_curve_surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect::intersect_surface_surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::offset::{draft_angle, offset_face, offset_solid, shell_solid_with_open_faces, thicken_face};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{
    make_box, make_cone, make_convex_hull, make_cylinder, make_planar_face_from_points, make_planar_face_from_wire, make_polyline_wire, make_rectangle_wire, make_regular_polygon_wire, make_sphere, make_torus, Wire,
};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::sew::{convert_to_nurbs, defeature, heal_solid, sew_faces};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::sweep::{extrude_face, helical_sweep, loft_profiles, pipe, revolve_face, sweep_along_path};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{closest_point_on_solid, distance_solid_solid, edge_length, face_area, solid_bounding_box, solid_center_of_mass, solid_surface_area, solid_volume};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::tessellation::{tessellate_face, tessellate_solid, tessellate_wire};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, EdgeId, FaceId, SolidId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::{approximate_curve_with_count, coons_patch_nurbs, interpolate_curve, interpolate_surface_grid, ParamMethod};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3 as NativeVec3};
use mesh_io::{export_solid_dwg, export_solid_glb, export_solid_obj, export_solid_stl, import_dwg_to_body, import_glb_to_body, import_obj_to_body, import_stl_to_body, mesh_to_mesh_data, triangle_mesh_from_transfer};
use contract::Vec3 as EVec3;
use step::{read_step, write_step};

// #region 🔖️ContractTypes

/// 🧭️ Geometry entity kind carried by a handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum GeometryKind {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    Compound,
    Curve,
    Surface,
}

/// 🧭️ Opaque content-addressed geometry handle (hex-encoded OS engine key).
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[value(transparent)]
pub struct GeometryHandle(pub String);

impl GeometryHandle {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🧩️ Topology handles extracted from a B-Rep shape.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BrepTopology {
    pub vertices: Vec<GeometryHandle>,
    pub edges: Vec<GeometryHandle>,
    pub faces: Vec<GeometryHandle>,
    pub shells: Vec<GeometryHandle>,
}

/// 📏️ Closest-point / distance query result.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ClosestPoint {
    pub distance: f64,
    pub point: Vec3,
    pub parameter: Option<f64>,
    pub uv: Option<[f64; 2]>,
}

// #endregion 🔖️ContractTypes

// #region ⚠️ Errors
/// ⚠️ Kernel operation error.
#[derive(Clone, Debug, PartialEq)]
pub enum BrepError {
    InvalidInput(String),
    MissingHandle(String),
    Operation(String),
}

impl std::fmt::Display for BrepError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(formatter, "invalid input: {message}"),
            Self::MissingHandle(handle) => write!(formatter, "missing handle: {handle}"),
            Self::Operation(message) => write!(formatter, "operation failed: {message}"),
        }
    }
}

impl std::error::Error for BrepError {}
// #endregion ⚠️ Errors

// #region 🔖️Kernel
/// 🔌️ Model-free synchronous BREP kernel interface.
pub trait BrepKernel {
    // #region Primitives
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn convex_hull(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError>;
    // #endregion Primitives

    // #region Curves
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn line_curve(&mut self, start: Vec3, end: Vec3) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn circle_curve(&mut self, center: Vec3, normal: Vec3, radius: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn arc_curve(&mut self, center: Vec3, normal: Vec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn ellipse_curve(&mut self, center: Vec3, normal: Vec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn polyline_wire(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn rectangle_wire(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn regular_polygon_wire(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn interpolate_curve(&mut self, points: &[Vec3], degree: usize) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn approximate_curve(&mut self, points: &[Vec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn helix_curve(&mut self, origin: Vec3, axis: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError>;
    // #endregion Curves

    // #region Surfaces
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn plane_surface(&mut self, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn planar_face_from_points(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn planar_face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn nurbs_surface_from_grid(&mut self, points: &[Vec<Vec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn coons_patch(&mut self, curves: &[Vec<Vec3>]) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn offset_face(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn thicken_face(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError>;
    // #endregion Surfaces

    // #region Sweeps
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn extrude_wire(&mut self, wire: &GeometryHandle, vector: Vec3) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn extrude(&mut self, face: &GeometryHandle, direction: Vec3, distance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn revolve(&mut self, face: &GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn loft(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn pipe(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn helical_sweep(&mut self, profile: &GeometryHandle, axis_origin: Vec3, axis_dir: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError>;
    // #endregion Sweeps

    // #region Booleans
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn compound_cut(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    // #endregion Booleans

    // #region Transforms
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn translate(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn rotate(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mirror(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn copy_shape(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn linear_pattern(&mut self, shape: &GeometryHandle, direction: Vec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn circular_pattern(&mut self, shape: &GeometryHandle, axis: Vec3, count: usize) -> Result<GeometryHandle, BrepError>;
    #[allow(clippy::too_many_arguments, reason = "grid pattern needs an independent spacing/count pair per axis; grouping into a params struct would ripple through every out-of-scope BrepKernel implementor and caller")]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn grid_pattern(&mut self, shape: &GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError>;
    // #endregion Transforms

    // #region Features
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fillet_variable(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError>;
    /// 🎯️ Fillets only the given edges instead of every edge of the solid — avoids paying the
    /// cost of a full-solid fillet when only a handful of edges are actually selected.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fillet_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chamfer_asymmetric(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError>;
    /// 🎯️ Chamfers only the given edges instead of every edge of the solid.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chamfer_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn shell(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn draft(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: Vec3, neutral_point: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn offset_solid(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn defeature(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    // #endregion Features

    // #region Intersect
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn section(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<Vec<GeometryHandle>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn split(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<(GeometryHandle, GeometryHandle), BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_curve_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_surface_intersect(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_surface_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError>;
    // #endregion Intersect

    // #region Evaluate
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_point(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_tangent(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_domain(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_curvature(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_point(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_normal(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError>;
    // #endregion Evaluate

    // #region Measure
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn area(&self, shape: &GeometryHandle) -> Result<f64, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn length(&self, shape: &GeometryHandle) -> Result<f64, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn center_of_mass(&self, shape: &GeometryHandle) -> Result<Vec3, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bounding_box(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn distance(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn closest_point(&self, shape: &GeometryHandle, point: Vec3) -> Result<ClosestPoint, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn classify_point(&self, solid: &GeometryHandle, point: Vec3) -> Result<PointClassification, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validate(&self, shape: &GeometryHandle) -> Result<String, BrepError>;
    // #endregion Measure

    // #region Construct
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn vertex(&mut self, point: Vec3) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sew_faces(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn heal_solid(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn convert_to_nurbs(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn deconstruct(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError>;
    // #endregion Construct

    // #region IO
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_step(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_obj(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_gltf(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_step(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_stl(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_obj(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_dwg(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_dwg(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError>;
    // #endregion IO

    // #region Core
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dispose(&mut self, handle: &GeometryHandle);
    /// 🧹️ Drops every registry entry whose handle isn't in `live`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn retain(&mut self, live: &std::collections::HashSet<String>);
    /// 📊️ Number of geometry handles currently held by the kernel's registry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn registry_len(&self) -> usize;
    /// 🧩️ Every shell of `solid` as its own first-class handle (see `GeometryKind::Shell`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn solid_shells(&mut self, solid: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError>;
    /// 🧩️ Bundles `solids` behind one `GeometryKind::Compound` handle — the collection identity
    /// `import_step`/patterns/booleans return when the operation naturally yields more than one
    /// solid, mirroring [`Self::explode`]'s inverse.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn compound(&mut self, solids: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    /// 🧩️ The inverse of [`Self::compound`]: the member solids' own handles.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn explode(&mut self, compound: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError>;
    /// 🏷️ The document-scoped [`PersistentLabel`] a handle currently resolves to — stable across
    /// a `dispose`d-and-regranted handle for the same entity, unlike the handle string itself.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn label(&self, handle: &GeometryHandle) -> Option<u64>;
    // #endregion Core
}
// #endregion 🔖️Kernel

// #region 🔖️Types

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::ShellId;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::EntityRef;

/// 🧠 One live registry entry. Vertex/Edge/Face/Shell/Solid wrap the arena id whose own
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]
/// [`label_of_entity`] resolves through; Wire/Curve/Surface/Compound carry no arena identity of
/// their own (a `Wire` bundles arena ids but isn't itself stored in `Body`; a bare `Curve3`/
/// `Surface` constructed via `register_curve`/`register_surface` never enters `body.curves3`/
/// `body.surfaces` either) so each instance is stamped with its own fresh label at registration
/// time instead.
#[derive(Clone)]
enum Entity {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire(Wire, PersistentLabel),
    Face(FaceId),
    Shell(ShellId),
    Solid(SolidId),
    Compound(Vec<SolidId>, PersistentLabel),
    Curve(Curve3, PersistentLabel),
    Surface(Surface, PersistentLabel),
}

/// 🧠 Native B-Rep session.
pub struct Brep {
    body: Body,
    live: HashMap<String, Entity>,
}

impl Default for Brep {
    fn default() -> Self {
        Self::new()
    }
}

impl Brep {
    /// 🏗️ Empty native kernel session.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self { body: Body::new(), live: HashMap::new() }
    }
}

// #endregion 🔖️Types

// #region 🧮Convert

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pnt(v: EVec3) -> Pnt3 {
    Pnt3::new(v[0], v[1], v[2])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn evec(p: Pnt3) -> EVec3 {
    [p.x, p.y, p.z]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vec3(v: EVec3) -> NativeVec3 {
    NativeVec3::new(v[0], v[1], v[2])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn map_err(e: KernelError) -> BrepError {
    BrepError::Operation(e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn map_step(e: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::StepError) -> BrepError {
    BrepError::Operation(e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rotate_point_around_axis(point: Pnt3, origin: Pnt3, axis: NativeVec3, angle: f64) -> Pnt3 {
    let v = point - origin;
    let cos = angle.cos();
    let sin = angle.sin();
    let parallel = axis * v.dot(axis);
    let lateral = v - parallel;
    origin + lateral * cos + axis.cross(lateral) * sin + parallel
}

impl Brep {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transform_solid_mesh<F>(&mut self, solid: SolidId, map: F) -> Result<GeometryHandle, BrepError>
    where
        F: Fn(Pnt3) -> Pnt3,
    {
        let transfer = tessellate_solid(&self.body, solid, 0.05).map_err(map_err)?;
        let n = transfer.position.len() / 3;
        if n == 0 || transfer.index.len() < 3 {
            return Err(BrepError::InvalidInput("cannot transform empty solid mesh".into()));
        }
        let mut triangles = Vec::with_capacity(transfer.index.len() / 3);
        for tri in transfer.index.chunks_exact(3) {
            let mut pts = [Pnt3::new(0.0, 0.0, 0.0); 3];
            for (k, &idx) in tri.iter().enumerate() {
                let i = idx as usize * 3;
                let p = Pnt3::new(transfer.position[i] as f64, transfer.position[i + 1] as f64, transfer.position[i + 2] as f64);
                pts[k] = map(p);
            }
            triangles.push(pts);
        }
        let mut rec = OpRecorder::new();
        let out = crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::solid_from_triangle_soup(&mut self.body, &triangles, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
}

/// 📦 Converts a tessellation [`MeshTransfer`] into `semio-framework-mesh-engine`'s `MeshData`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mesh_data_from_mesh_transfer(transfer: &MeshTransfer) -> semio_framework_mesh_engine::MeshData {
    let mut data = mesh_to_mesh_data(&triangle_mesh_from_transfer(transfer));
    data.edge_positions = transfer.edges.clone();
    data
}

// #endregion 🧮Convert

// #region 🧮Registry

/// 🏷️ The [`PersistentLabel`] a live [`Entity`] resolves to — the identity [`Brep::mint`] hashes a
/// handle from. Vertex/Edge/Face/Shell/Solid read it back out of the `Body` arena entry they wrap
/// (stable for the entity's whole lifetime, independent of when/how often it's registered); the
/// other variants carry a label they were stamped with at registration time (see [`Entity`]'s own
/// docstring for why).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn label_of_entity(body: &Body, entity: &Entity) -> Option<PersistentLabel> {
    match entity {
        Entity::Vertex(id) => body.vertices.get(*id).map(|v| v.label),
        Entity::Edge(id) => body.edges.get(*id).map(|e| e.label),
        Entity::Face(id) => body.faces.get(*id).map(|f| f.label),
        Entity::Shell(id) => body.shells.get(*id).map(|s| s.label),
        Entity::Solid(id) => body.solids.get(*id).map(|s| s.label),
        Entity::Wire(_, label) | Entity::Compound(_, label) | Entity::Curve(_, label) | Entity::Surface(_, label) => Some(*label),
    }
}

impl Brep {
    /// 🏗️ Mints a handle deterministic in `(kind, entity's PersistentLabel)` — never a counter —
    /// so registering the *same* labelled entity again (e.g. two `deconstruct` calls on an
    /// untouched shape) always yields byte-identical handles. Panics only if `entity` carries no
    /// resolvable label, which would mean a caller minted against an id it never actually inserted
    /// into `self.body` — a caller bug, not a runtime condition to recover from.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mint(&mut self, kind: GeometryKind, entity: Entity) -> GeometryHandle {
        let label = label_of_entity(&self.body, &entity).expect("entity must carry a resolvable PersistentLabel");
        let payload = format!("{kind:?}:{}", label.0);
        let handle = GeometryHandle(semio_framework_hash::hash_bytes(payload.as_bytes()));
        self.live.insert(handle.as_str().to_string(), entity);
        handle
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn register_solid(&mut self, solid: SolidId) -> GeometryHandle {
        self.mint(GeometryKind::Solid, Entity::Solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn register_face(&mut self, face: FaceId) -> GeometryHandle {
        self.mint(GeometryKind::Face, Entity::Face(face))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn register_shell(&mut self, shell: ShellId) -> GeometryHandle {
        self.mint(GeometryKind::Shell, Entity::Shell(shell))
    }
    /// 🧩️ `solids` (already registered elsewhere) bundled behind one fresh-labelled compound
    /// handle — the compound's label is its own, not shared with any member solid's.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn register_compound(&mut self, solids: Vec<SolidId>) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Compound, Entity::Compound(solids, label))
    }
    /// 🧩️ A `Wire` bundles arena ids but has no [`Body`]-stored identity of its own, so it is
    /// stamped with a fresh label at registration time (see [`Entity`]'s own docstring).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn register_wire(&mut self, wire: Wire) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Wire, Entity::Wire(wire, label))
    }
    /// 🧩️ A bare `Curve3` built via one of the curve constructors never enters `body.curves3`
    /// (only a curve reached through an `Edge` does), so it too is stamped with a fresh label.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn register_curve(&mut self, curve: Curve3) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Curve, Entity::Curve(curve, label))
    }
    /// 🧩️ Mirror of [`Self::register_curve`] for a bare `Surface`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn register_surface(&mut self, surface: Surface) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Surface, Entity::Surface(surface, label))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn entity(&self, handle: &GeometryHandle) -> Result<&Entity, BrepError> {
        self.live.get(handle.as_str()).ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))
    }

    /// ♻️ [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::EntityRef`] roots for every entity currently kept alive by a live
    /// handle — the protection set [`Body::reachable_from`] walks before [`Body::compact`] frees
    /// anything, so `dispose`/`retain` never reclaim geometry a surviving handle still needs. A
    /// `Wire`'s member edges/vertices are included even though the wire itself isn't a `Body` root,
    /// since nothing else would otherwise keep them alive while the wire handle is live.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn live_roots(&self) -> Vec<EntityRef> {
        let mut roots = Vec::new();
        for entity in self.live.values() {
            match entity {
                Entity::Vertex(id) => roots.push(EntityRef::Vertex(*id)),
                Entity::Edge(id) => roots.push(EntityRef::Edge(*id)),
                Entity::Face(id) => roots.push(EntityRef::Face(*id)),
                Entity::Shell(id) => roots.push(EntityRef::Shell(*id)),
                Entity::Solid(id) => roots.push(EntityRef::Solid(*id)),
                Entity::Compound(solids, _) => roots.extend(solids.iter().map(|s| EntityRef::Solid(*s))),
                Entity::Wire(wire, _) => {
                    roots.extend(wire.members.iter().map(|(edge, _)| EntityRef::Edge(*edge)));
                    roots.extend(wire.vertices.iter().map(|v| EntityRef::Vertex(*v)));
                }
                Entity::Curve(_, _) | Entity::Surface(_, _) => {}
            }
        }
        roots
    }

    /// ♻️ Frees every arena entity no surviving live handle still reaches — the GC step `dispose`
    /// and `retain` both run after touching `self.live`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn compact_unreachable(&mut self) {
        let roots = self.live_roots();
        let keep = self.body.reachable_from(&roots);
        self.body.compact(&keep);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn solid_id(&self, handle: &GeometryHandle) -> Result<SolidId, BrepError> {
        match self.entity(handle)? {
            Entity::Solid(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a solid", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn face_id(&self, handle: &GeometryHandle) -> Result<FaceId, BrepError> {
        match self.entity(handle)? {
            Entity::Face(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a face", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn wire_ref(&self, handle: &GeometryHandle) -> Result<&Wire, BrepError> {
        match self.entity(handle)? {
            Entity::Wire(w, _) => Ok(w),
            _ => Err(BrepError::InvalidInput(format!("{} is not a wire", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_ref(&self, handle: &GeometryHandle) -> Result<&Curve3, BrepError> {
        match self.entity(handle)? {
            Entity::Curve(c, _) => Ok(c),
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_ref(&self, handle: &GeometryHandle) -> Result<&Surface, BrepError> {
        match self.entity(handle)? {
            Entity::Surface(s, _) => Ok(s),
            _ => Err(BrepError::InvalidInput(format!("{} is not a surface", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edge_id(&self, handle: &GeometryHandle) -> Result<EdgeId, BrepError> {
        match self.entity(handle)? {
            Entity::Edge(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not an edge", handle.as_str()))),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_tag(e: &Entity) -> String {
    match e {
        Entity::Vertex(id) => format!("v{id}"),
        Entity::Edge(id) => format!("e{id}"),
        Entity::Wire(w, _) => format!("w{}", w.members.len()),
        Entity::Face(id) => format!("f{id}"),
        Entity::Shell(id) => format!("sh{id}"),
        Entity::Solid(id) => format!("s{id}"),
        Entity::Compound(solids, _) => format!("cp{}", solids.len()),
        Entity::Curve(_, _) => "c".into(),
        Entity::Surface(_, _) => "S".into(),
    }
}

// #endregion 🧮Registry

// #region 🔖️SyncApi

impl Brep {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn box_prim_sync(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut self.body, width, depth, height, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sphere_prim_sync(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_sphere(&mut self.body, radius, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cylinder_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cylinder(&mut self.body, radius, height, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cone_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cone(&mut self.body, radius, height, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn torus_prim_sync(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_torus(&mut self.body, major, minor, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn convex_hull_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let solid = make_convex_hull(&mut self.body, &pts, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn line_curve_sync(&mut self, start: EVec3, end: EVec3) -> Result<GeometryHandle, BrepError> {
        Ok(self.register_curve(Curve3::Line { origin: pnt(start), dir: pnt(end) - pnt(start) }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn circle_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad circle frame".into()))?;
        Ok(self.register_curve(Curve3::Circle { frame, radius }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn arc_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        if !(radius.is_finite() && radius > 0.0) {
            return Err(BrepError::InvalidInput("arc radius must be positive".into()));
        }
        if !start_angle.is_finite() || !end_angle.is_finite() {
            return Err(BrepError::InvalidInput("arc angles must be finite".into()));
        }
        if (end_angle - start_angle).abs() <= 1e-15 {
            return Err(BrepError::InvalidInput("arc start and end angles must differ".into()));
        }
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad arc frame".into()))?;
        let circle = Curve3::Circle { frame, radius };
        let nurbs = circle.to_nurbs((start_angle, end_angle));
        Ok(self.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ellipse_curve_sync(&mut self, center: EVec3, normal: EVec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad ellipse frame".into()))?;
        Ok(self.register_curve(Curve3::Ellipse { frame, major_radius: semi_major, minor_radius: semi_minor }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn polyline_wire_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let wire = make_polyline_wire(&mut self.body, &pts, false, &mut rec).map_err(map_err)?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rectangle_wire_sync(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut self.body, width, height, &mut rec).map_err(map_err)?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn regular_polygon_wire_sync(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_regular_polygon_wire(&mut self.body, radius, sides, &mut rec).map_err(map_err)?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn interpolate_curve_sync(&mut self, points: &[EVec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        if points.len() < 2 {
            return Err(BrepError::InvalidInput("interpolate_curve needs at least 2 points".into()));
        }
        let controls: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let nurbs = interpolate_curve(&controls, degree, ParamMethod::Centripetal, None, false).ok_or_else(|| BrepError::InvalidInput("interpolate_curve: degenerate or coincident points".into()))?;
        Ok(self.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn approximate_curve_sync(&mut self, points: &[EVec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        if points.len() < 2 {
            return Err(BrepError::InvalidInput("approximate_curve needs at least 2 points".into()));
        }
        let controls: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let target = control_points.clamp(2, controls.len());
        let (nurbs, _err) = approximate_curve_with_count(&controls, degree, target).ok_or_else(|| BrepError::InvalidInput("approximate_curve: degenerate input or infeasible control-point count".into()))?;
        Ok(self.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn helix_curve_sync(&mut self, origin: EVec3, axis: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let mut pts = Vec::new();
        let n = ((turns.abs() * 32.0).ceil() as usize).max(8);
        let axis_v = vec3(axis).normalized().unwrap_or(NativeVec3::Z);
        let frame = Frame3::from_normal(pnt(origin), axis_v).ok_or_else(|| BrepError::InvalidInput("bad helix".into()))?;
        for i in 0..=n {
            let t = i as f64 / n as f64 * turns;
            let ang = t * std::f64::consts::TAU;
            let p = frame.origin + frame.x * (radius * ang.cos()) + frame.y * (radius * ang.sin()) + axis_v * (pitch * t);
            pts.push(evec(p));
        }
        self.polyline_wire_sync(&pts)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn plane_surface_sync(&mut self, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(origin), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad plane".into()))?;
        Ok(self.register_surface(Surface::Plane { frame }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn planar_face_from_points_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let face = make_planar_face_from_points(&mut self.body, &pts, &mut rec).map_err(map_err)?;
        Ok(self.register_face(face))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn planar_face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let w = self.wire_ref(wire)?.clone();
        let origin = self.body.vertices.get(w.vertices[0]).map(|v| v.position).unwrap_or(Pnt3::new(0.0, 0.0, 0.0));
        let mut rec = OpRecorder::new();
        let face = make_planar_face_from_wire(&mut self.body, &w, origin, NativeVec3::Z, &mut rec).map_err(map_err)?;
        Ok(self.register_face(face))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nurbs_surface_from_grid_sync(&mut self, points: &[Vec<EVec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        if points.is_empty() || points[0].is_empty() {
            return Err(BrepError::InvalidInput("nurbs grid requires a non-empty control net".into()));
        }
        let grid: Vec<Vec<Pnt3>> = points.iter().map(|row| row.iter().copied().map(pnt).collect()).collect();
        let surface = interpolate_surface_grid(&grid, degree_u, degree_v).ok_or_else(|| BrepError::InvalidInput("nurbs_surface_from_grid: ragged rows or degenerate net".into()))?;
        Ok(self.register_surface(surface))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn coons_patch_sync(&mut self, curves: &[Vec<EVec3>]) -> Result<GeometryHandle, BrepError> {
        if curves.len() != 4 {
            return Err(BrepError::InvalidInput("coons_patch requires exactly 4 boundary polylines".into()));
        }
        let mut boundary_curves = Vec::with_capacity(4);
        for (idx, curve) in curves.iter().enumerate() {
            if curve.len() < 2 {
                return Err(BrepError::InvalidInput(format!("coons_patch boundary {idx} needs at least 2 points")));
            }
            let pts: Vec<Pnt3> = curve.iter().copied().map(pnt).collect();
            let degree = pts.len().saturating_sub(1).min(3);
            let nurbs = interpolate_curve(&pts, degree, ParamMethod::Centripetal, None, false).ok_or_else(|| BrepError::InvalidInput(format!("coons_patch boundary {idx} is degenerate")))?;
            boundary_curves.push(nurbs);
        }
        let [c0, d1, c1, d0] = [&boundary_curves[0], &boundary_curves[1], &boundary_curves[2], &boundary_curves[3]];
        let surface = coons_patch_nurbs(c0, c1, d0, d1, 1e-6).ok_or_else(|| BrepError::InvalidInput("coons_patch: boundary corners do not match or bases are incompatible".into()))?;
        Ok(self.register_surface(surface))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn offset_face_sync(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let mut rec = OpRecorder::new();
        let out = offset_face(&mut self.body, id, distance, &mut rec).map_err(map_err)?;
        Ok(self.register_face(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn thicken_face_sync(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let mut rec = OpRecorder::new();
        let solid = thicken_face(&mut self.body, id, thickness, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn extrude_wire_sync(&mut self, wire: &GeometryHandle, vector: EVec3) -> Result<GeometryHandle, BrepError> {
        let face = self.planar_face_from_wire_sync(wire)?;
        let dist = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
        let dir = if dist > 1e-15 { [vector[0] / dist, vector[1] / dist, vector[2] / dist] } else { [0.0, 0.0, 1.0] };
        self.extrude_sync(&face, dir, dist)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn extrude_sync(&mut self, face: &GeometryHandle, direction: EVec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let mut rec = OpRecorder::new();
        let solid = extrude_face(&mut self.body, id, vec3(direction), distance, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn revolve_sync(&mut self, face: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let mut rec = OpRecorder::new();
        let solid = revolve_face(&mut self.body, id, pnt(axis_origin), vec3(axis_direction), angle, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn loft_sync(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        let mut faces = Vec::new();
        for p in profiles {
            faces.push(self.face_id(p)?);
        }
        let mut rec = OpRecorder::new();
        let solid = loft_profiles(&mut self.body, &faces, smooth, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sweep_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let wire = self.wire_ref(path)?.clone();
        let mut rec = OpRecorder::new();
        let solid = sweep_along_path(&mut self.body, face, &wire, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn pipe_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let wire = self.wire_ref(path)?.clone();
        let g = match guide {
            Some(h) => Some(self.wire_ref(h)?.clone()),
            None => None,
        };
        let mut rec = OpRecorder::new();
        let solid = pipe(&mut self.body, face, &wire, g.as_ref(), &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn helical_sweep_sync(&mut self, profile: &GeometryHandle, axis_origin: EVec3, axis_dir: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let mut rec = OpRecorder::new();
        let solid = helical_sweep(&mut self.body, face, pnt(axis_origin), vec3(axis_dir), radius, pitch, turns, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fuse_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Unite, 1e-6, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cut_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Cut, 1e-6, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Intersect, 1e-6, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compound_cut_sync(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let t = self.solid_id(target)?;
        let mut ids = Vec::new();
        for tool in tools {
            ids.push(self.solid_id(tool)?);
        }
        let mut rec = OpRecorder::new();
        let solid = compound_cut(&mut self.body, t, &ids, 1e-6, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn translate_sync(&mut self, shape: &GeometryHandle, offset: EVec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let o = vec3(offset);
        self.transform_solid_mesh(solid, |p| p + o)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rotate_sync(&mut self, shape: &GeometryHandle, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(map_err)?;
        let axis_v = vec3(axis).normalized().unwrap_or(NativeVec3::Z);
        let center = Pnt3::new((bb.min.x + bb.max.x) * 0.5, (bb.min.y + bb.max.y) * 0.5, (bb.min.z + bb.max.z) * 0.5);
        self.transform_solid_mesh(solid, |p| rotate_point_around_axis(p, center, axis_v, angle))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn scale_sync(&mut self, shape: &GeometryHandle, factor: f64, center: EVec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let c = pnt(center);
        self.transform_solid_mesh(solid, |p| c + (p - c) * factor)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mirror_sync(&mut self, shape: &GeometryHandle, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let n = vec3(normal).normalized().unwrap_or(NativeVec3::Z);
        let o = pnt(origin);
        self.transform_solid_mesh(solid, |p| {
            let v = p - o;
            p - n * (2.0 * v.dot(n))
        })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn copy_shape_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.translate_sync(shape, [0.0, 0.0, 0.0])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn linear_pattern_sync(&mut self, shape: &GeometryHandle, direction: EVec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        let mut current = shape.clone();
        for i in 1..count.max(1) {
            let off = [direction[0] * spacing * i as f64, direction[1] * spacing * i as f64, direction[2] * spacing * i as f64];
            let next = self.translate_sync(shape, off)?;
            current = self.fuse_sync(&current, &next)?;
        }
        Ok(current)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn circular_pattern_sync(&mut self, shape: &GeometryHandle, axis: EVec3, count: usize) -> Result<GeometryHandle, BrepError> {
        let mut current = shape.clone();
        let n = count.max(1);
        for i in 1..n {
            let ang = std::f64::consts::TAU * i as f64 / n as f64;
            let next = self.rotate_sync(shape, axis, ang)?;
            current = self.fuse_sync(&current, &next)?;
        }
        Ok(current)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn grid_pattern_sync(&mut self, shape: &GeometryHandle, dir_x: EVec3, dir_y: EVec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        let mut current = shape.clone();
        for i in 0..count_x.max(1) {
            for j in 0..count_y.max(1) {
                if i == 0 && j == 0 {
                    continue;
                }
                let off = [dir_x[0] * spacing_x * i as f64 + dir_y[0] * spacing_y * j as f64, dir_x[1] * spacing_x * i as f64 + dir_y[1] * spacing_y * j as f64, dir_x[2] * spacing_x * i as f64 + dir_y[2] * spacing_y * j as f64];
                let next = self.translate_sync(shape, off)?;
                current = self.fuse_sync(&current, &next)?;
            }
        }
        Ok(current)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fillet_sync(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let mut rec = OpRecorder::new();
        let out = fillet_edges(&mut self.body, solid, &edges, radius, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fillet_variable_sync(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let e = *edges.first().ok_or_else(|| BrepError::InvalidInput("no edges".into()))?;
        let mut rec = OpRecorder::new();
        let out = fillet_variable(&mut self.body, solid, e, radius_start, radius_end, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fillet_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut eids = Vec::new();
        for e in edges {
            eids.push(self.edge_id(e)?);
        }
        if eids.is_empty() {
            eids = all_edges(&self.body, solid);
        }
        let mut rec = OpRecorder::new();
        let out = fillet_edges(&mut self.body, solid, &eids, radius, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let mut rec = OpRecorder::new();
        let out = chamfer_edges(&mut self.body, solid, &edges, distance, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_asymmetric_sync(&mut self, shape: &GeometryHandle, d1: f64, _d2: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, d1)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut eids = Vec::new();
        for e in edges {
            eids.push(self.edge_id(e)?);
        }
        if eids.is_empty() {
            eids = all_edges(&self.body, solid);
        }
        let mut rec = OpRecorder::new();
        let out = chamfer_edges(&mut self.body, solid, &eids, distance, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn shell_sync(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let open_ids: Vec<_> = open_faces.iter().filter_map(|h| self.face_id(h).ok()).collect();
        let mut rec = OpRecorder::new();
        let out = shell_solid_with_open_faces(&mut self.body, solid, thickness.abs(), &open_ids, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn draft_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: EVec3, _neutral_point: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let face = if let Some(f) = faces.first() { self.face_id(f)? } else { *self.body.solid_faces(solid).first().ok_or_else(|| BrepError::InvalidInput("no face".into()))? };
        let mut rec = OpRecorder::new();
        let out = draft_angle(&mut self.body, solid, face, angle, vec3(pull_direction), &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn offset_solid_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let out = offset_solid(&mut self.body, solid, distance, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn defeature_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut fids = Vec::new();
        for f in faces {
            fids.push(self.face_id(f)?);
        }
        let mut rec = OpRecorder::new();
        let out = defeature(&mut self.body, solid, &fids, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn section_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<Vec<GeometryHandle>, BrepError> {
        let id = self.solid_id(solid)?;
        let mut rec = OpRecorder::new();
        let faces = section_solid_by_plane(&mut self.body, id, pnt(plane_origin), vec3(plane_normal), 1e-6, &mut rec).map_err(map_err)?;
        Ok(faces.into_iter().map(|f| self.register_face(f)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn split_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        let id = self.solid_id(solid)?;
        let mut rec = OpRecorder::new();
        let (a, b) = split_solid_by_plane(&mut self.body, id, pnt(plane_origin), vec3(plane_normal), 1e-6, &mut rec).map_err(map_err)?;
        Ok((self.register_solid(a), self.register_solid(b)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_curve_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let ca = self.curve_ref(a)?;
        let cb = self.curve_ref(b)?;
        let hits = intersect_curve_curve(ca, cb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_surface_intersect_sync(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let c = self.curve_ref(curve)?;
        let s = self.surface_ref(surface)?;
        let hits = intersect_curve_surface(c, s, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_surface_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let sa = self.surface_ref(a)?;
        let sb = self.surface_ref(b)?;
        let hits = intersect_surface_surface(sa, sb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|hit| self.register_curve(hit.curve3)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_point_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        Ok(evec(self.curve_ref(curve)?.eval(parameter)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_tangent_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        let c = self.curve_ref(curve)?;
        let p0 = c.eval(parameter);
        let p1 = c.eval(parameter + 1e-5);
        let d = p1 - p0;
        Ok([d.x, d.y, d.z])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_domain_sync(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        let c = self.curve_ref(curve)?;
        let (min, max) = c.domain();
        Ok(ParamDomain { min, max })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_curvature_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        Ok(self.curve_ref(curve)?.curvature(parameter))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_point_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.surface_ref(surface)?;
        Ok(evec(s.eval(u, v)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_normal_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.surface_ref(surface)?;
        let n = s.normal(u, v).ok_or_else(|| BrepError::Operation("surface normal undefined".into()))?;
        Ok([n.x, n.y, n.z])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let solid = self.solid_id(shape)?;
        solid_volume(&self.body, solid, 1e-4).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn area_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => solid_surface_area(&self.body, *id, 1e-4).map_err(map_err),
            Entity::Face(id) => face_area(&self.body, *id, 1e-4).map_err(map_err),
            _ => Err(BrepError::InvalidInput("area requires solid or face".into())),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn length_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let edge = self.edge_id(shape)?;
        edge_length(&self.body, edge).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn center_of_mass_sync(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        let solid = self.solid_id(shape)?;
        Ok(evec(solid_center_of_mass(&self.body, solid, 1e-4).map_err(map_err)?))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn bounding_box_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(map_err)?;
        let corners = [
            evec(bb.min),
            evec(Pnt3::new(bb.max.x, bb.min.y, bb.min.z)),
            evec(Pnt3::new(bb.max.x, bb.max.y, bb.min.z)),
            evec(Pnt3::new(bb.min.x, bb.max.y, bb.min.z)),
            evec(Pnt3::new(bb.min.x, bb.min.y, bb.max.z)),
            evec(Pnt3::new(bb.max.x, bb.min.y, bb.max.z)),
            evec(bb.max),
            evec(Pnt3::new(bb.min.x, bb.max.y, bb.max.z)),
        ];
        self.convex_hull_sync(&corners)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn distance_sync(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        distance_solid_solid(&self.body, sa, sb).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn closest_point_sync(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        let solid = self.solid_id(shape)?;
        let (p, d) = closest_point_on_solid(&self.body, solid, pnt(point)).map_err(map_err)?;
        Ok(ClosestPoint { distance: d, point: evec(p), parameter: None, uv: None })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn classify_point_sync(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        let id = self.solid_id(solid)?;
        point_in_solid(&self.body, id, pnt(point), 1e-6).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate_sync(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        let _ = self.solid_id(shape)?;
        let issues = validate_body(&self.body);
        let report = serde_json::json!({
            "ok": issues.is_empty(),
            "issueCount": issues.len(),
            "issues": issues.iter().map(|issue| serde_json::json!({
                "entity": issue.entity,
                "code": issue.code,
                "message": issue.message,
            })).collect::<Vec<_>>(),
        });
        Ok(report.to_string())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn vertex_sync(&mut self, point: EVec3) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let id = make_vertex(&mut self.body, pnt(point), Tol::DEFAULT, &mut rec);
        Ok(self.mint(GeometryKind::Vertex, Entity::Vertex(id)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sew_faces_sync(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mut fids = Vec::new();
        for f in faces {
            fids.push(self.face_id(f)?);
        }
        let mut rec = OpRecorder::new();
        let solid = sew_faces(&mut self.body, &fids, tolerance, &mut rec).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn heal_solid_sync(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let _ = heal_solid(&mut self.body, solid, tolerance, &mut rec).map_err(map_err)?;
        Ok(shape.clone())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn convert_to_nurbs_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let _ = convert_to_nurbs(&mut self.body, solid, &mut rec).map_err(map_err)?;
        Ok(shape.clone())
    }
    /// 🏷️ Deterministic per (label, kind) minting (see [`Brep::mint`]) makes this idempotent: two
    /// calls against the same untouched `shape` walk the same solid→shell/face→coedge structure
    /// and mint against the same [`PersistentLabel`]s each time, so the returned handles are
    /// byte-identical, not merely equal-cardinality.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn deconstruct_sync(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut topo = BrepTopology::default();
        let mut seen_vertices = std::collections::BTreeSet::new();
        let mut seen_edges = std::collections::BTreeSet::new();
        for shell in self.body.solid_shells(solid) {
            topo.shells.push(self.register_shell(shell));
        }
        for face in self.body.solid_faces(solid) {
            topo.faces.push(self.register_face(face));
            for cid in self.body.face_coedges(face) {
                let Some(co) = self.body.coedges.get(cid) else { continue };
                if seen_edges.insert(co.edge.raw_index()) {
                    topo.edges.push(self.mint(GeometryKind::Edge, Entity::Edge(co.edge)));
                }
                if let Some((v0, v1)) = self.body.coedge_endpoints(cid) {
                    for v in [v0, v1] {
                        if seen_vertices.insert(v.raw_index()) {
                            topo.vertices.push(self.mint(GeometryKind::Vertex, Entity::Vertex(v)));
                        }
                    }
                }
            }
        }
        Ok(topo)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_step_sync(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        let mut solids = Vec::new();
        for s in shapes {
            solids.push(self.solid_id(s)?);
        }
        write_step(&self.body, &solids).map_err(map_step)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_stl_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_stl(&self.body, solid, deflection).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_obj_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_obj(&self.body, solid, deflection).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_gltf_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_glb_sync(shapes, deflection)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_glb_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_glb(&self.body, solid, deflection).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_glb_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_glb_to_body(&mut self.body, data, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    /// 🧩️ Merges the imported body into `self.body` (see [`Body::merge`]) rather than replacing
    /// it — handles minted before this call stay resolvable, since nothing already in `self.body`
    /// is renumbered or dropped; only the freshly imported solids get new handles.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_step_sync(&mut self, text: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        let imported = read_step(text).map_err(map_step)?;
        let imported_solid_ids: Vec<_> = imported.solids.ids().collect();
        let map = self.body.merge(&imported);
        Ok(imported_solid_ids.into_iter().map(|id| self.register_solid(map.solids[&id])).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_stl_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_stl_to_body(&mut self.body, data, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_obj_sync(&mut self, text: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_obj_to_body(&mut self.body, text, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_dwg_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_dwg(&self.body, solid, deflection).map_err(map_err)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_dwg_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_dwg_to_body(&mut self.body, data, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn kind_sync(&self, shape: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        Ok(match self.entity(shape)? {
            Entity::Vertex(_) => GeometryKind::Vertex,
            Entity::Edge(_) => GeometryKind::Edge,
            Entity::Wire(_, _) => GeometryKind::Wire,
            Entity::Face(_) => GeometryKind::Face,
            Entity::Shell(_) => GeometryKind::Shell,
            Entity::Solid(_) => GeometryKind::Solid,
            Entity::Compound(_, _) => GeometryKind::Compound,
            Entity::Curve(_, _) => GeometryKind::Curve,
            Entity::Surface(_, _) => GeometryKind::Surface,
        })
    }
    /// 🧠 Face outer/hole loops as position indices into the returned vertex buffer.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn solid_face_loops_sync(&self, shape: &GeometryHandle) -> Result<(Vec<[f32; 3]>, Vec<(Vec<u32>, Vec<Vec<u32>>)>), BrepError> {
        let solid = self.solid_id(shape)?;
        let mut vertex_to_index: HashMap<u32, u32> = HashMap::new();
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut face_loops = Vec::new();
        for face in self.body.solid_faces(solid) {
            let loops = self.body.face_loops(face);
            if loops.is_empty() {
                continue;
            }
            let mut indexed_loops: Vec<Vec<u32>> = Vec::new();
            for loop_id in loops {
                let mut loop_indices = Vec::new();
                for cid in self.body.loop_coedges(loop_id) {
                    let Some((start, _)) = self.body.coedge_endpoints(cid) else { continue };
                    let key = start.raw_index();
                    let index = if let Some(&existing) = vertex_to_index.get(&key) {
                        existing
                    } else {
                        let Some(vertex) = self.body.vertices.get(start) else {
                            return Err(BrepError::MissingHandle(format!("vertex {start}")));
                        };
                        let next = positions.len() as u32;
                        positions.push([vertex.position.x as f32, vertex.position.y as f32, vertex.position.z as f32]);
                        vertex_to_index.insert(key, next);
                        next
                    };
                    loop_indices.push(index);
                }
                if loop_indices.len() >= 3 {
                    indexed_loops.push(loop_indices);
                }
            }
            let mut iter = indexed_loops.into_iter();
            let Some(outer) = iter.next() else { continue };
            face_loops.push((outer, iter.collect()));
        }
        Ok((positions, face_loops))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tessellate_sync(&self, shape: &GeometryHandle, deflection: f64) -> Result<MeshTransfer, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => tessellate_solid(&self.body, *id, deflection).map_err(map_err),
            Entity::Face(id) => tessellate_face(&self.body, *id, deflection).map_err(map_err),
            Entity::Wire(wire, _) => tessellate_wire(&self.body, wire, deflection).map_err(map_err),
            other => Err(BrepError::InvalidInput(format!("cannot tessellate {}", entity_tag(other)))),
        }
    }
    /// ♻️ Reclaims the handle and, if no other live handle still reaches the underlying entity,
    /// runs the arena GC (see [`Brep::compact_unreachable`]) — dispose is not merely a registry
    /// removal, it is the operation that actually frees the topology.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn dispose_sync(&mut self, shape: &GeometryHandle) -> usize {
        let removed = self.live.remove(shape.as_str()).is_some();
        if removed {
            self.compact_unreachable();
        }
        usize::from(removed)
    }
    /// 🏷️ The [`PersistentLabel`] `handle` currently resolves to, or `None` for an unknown handle.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn label_of(&self, handle: &GeometryHandle) -> Option<PersistentLabel> {
        self.live.get(handle.as_str()).and_then(|e| label_of_entity(&self.body, e))
    }
    /// 🏷️ The handle for `label`, searching every arena store in turn — `None` if no live entity
    /// carries that label. Re-mints (deterministically, see [`Brep::mint`]) rather than requiring
    /// the caller to have kept the original handle string around.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn handle_for_label(&mut self, label: PersistentLabel) -> Option<GeometryHandle> {
        if let Some((id, _)) = self.body.vertices.iter().find(|(_, v)| v.label == label) {
            return Some(self.mint(GeometryKind::Vertex, Entity::Vertex(id)));
        }
        if let Some((id, _)) = self.body.edges.iter().find(|(_, e)| e.label == label) {
            return Some(self.mint(GeometryKind::Edge, Entity::Edge(id)));
        }
        if let Some((id, _)) = self.body.faces.iter().find(|(_, f)| f.label == label) {
            return Some(self.mint(GeometryKind::Face, Entity::Face(id)));
        }
        if let Some((id, _)) = self.body.shells.iter().find(|(_, s)| s.label == label) {
            return Some(self.mint(GeometryKind::Shell, Entity::Shell(id)));
        }
        if let Some((id, _)) = self.body.solids.iter().find(|(_, s)| s.label == label) {
            return Some(self.mint(GeometryKind::Solid, Entity::Solid(id)));
        }
        None
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn solid_shells_sync(&mut self, shape: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        let solid = self.solid_id(shape)?;
        Ok(self.body.solid_shells(solid).into_iter().map(|s| self.register_shell(s)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compound_sync(&mut self, solids: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let mut ids = Vec::with_capacity(solids.len());
        for s in solids {
            ids.push(self.solid_id(s)?);
        }
        Ok(self.register_compound(ids))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn explode_sync(&mut self, compound: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        match self.entity(compound)?.clone() {
            Entity::Compound(solids, _) => Ok(solids.into_iter().map(|s| self.register_solid(s)).collect()),
            _ => Err(BrepError::InvalidInput(format!("{} is not a compound", compound.as_str()))),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn all_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut set = std::collections::BTreeSet::new();
    for face in body.solid_faces(solid) {
        for cid in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(cid) {
                set.insert(c.edge);
            }
        }
    }
    set.into_iter().collect()
}

// #endregion 🔖️SyncApi

// #region 🔖️BrepKernelImpl

impl BrepKernel for Brep {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.box_prim_sync(width, depth, height)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.sphere_prim_sync(radius)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cylinder_prim_sync(radius, height)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cone_prim_sync(radius, height)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        self.torus_prim_sync(major, minor)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn convex_hull(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.convex_hull_sync(points)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn line_curve(&mut self, start: EVec3, end: EVec3) -> Result<GeometryHandle, BrepError> {
        self.line_curve_sync(start, end)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn circle_curve(&mut self, center: EVec3, normal: EVec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.circle_curve_sync(center, normal, radius)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn arc_curve(&mut self, center: EVec3, normal: EVec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        self.arc_curve_sync(center, normal, radius, start_angle, end_angle)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn ellipse_curve(&mut self, center: EVec3, normal: EVec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        self.ellipse_curve_sync(center, normal, semi_major, semi_minor)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn polyline_wire(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.polyline_wire_sync(points)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn rectangle_wire(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.rectangle_wire_sync(width, height)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn regular_polygon_wire(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        self.regular_polygon_wire_sync(radius, sides)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn interpolate_curve(&mut self, points: &[EVec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        self.interpolate_curve_sync(points, degree)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn approximate_curve(&mut self, points: &[EVec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        self.approximate_curve_sync(points, degree, control_points)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn helix_curve(&mut self, origin: EVec3, axis: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helix_curve_sync(origin, axis, radius, pitch, turns)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn plane_surface(&mut self, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        self.plane_surface_sync(origin, normal)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn planar_face_from_points(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_points_sync(points)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn planar_face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn nurbs_surface_from_grid(&mut self, points: &[Vec<EVec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        self.nurbs_surface_from_grid_sync(points, degree_u, degree_v)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn coons_patch(&mut self, curves: &[Vec<EVec3>]) -> Result<GeometryHandle, BrepError> {
        self.coons_patch_sync(curves)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn offset_face(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_face_sync(face, distance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn thicken_face(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        self.thicken_face_sync(face, thickness)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn extrude_wire(&mut self, wire: &GeometryHandle, vector: EVec3) -> Result<GeometryHandle, BrepError> {
        self.extrude_wire_sync(wire, vector)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn extrude(&mut self, face: &GeometryHandle, direction: EVec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.extrude_sync(face, direction, distance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn revolve(&mut self, face: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.revolve_sync(face, axis_origin, axis_direction, angle)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn loft(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        self.loft_sync(profiles, smooth)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.sweep_sync(profile, path)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn pipe(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        self.pipe_sync(profile, path, guide)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn helical_sweep(&mut self, profile: &GeometryHandle, axis_origin: EVec3, axis_dir: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helical_sweep_sync(profile, axis_origin, axis_dir, radius, pitch, turns)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.fuse_sync(a, b)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.cut_sync(a, b)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.intersect_sync(a, b)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn compound_cut(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.compound_cut_sync(target, tools)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn translate(&mut self, shape: &GeometryHandle, offset: EVec3) -> Result<GeometryHandle, BrepError> {
        self.translate_sync(shape, offset)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn rotate(&mut self, shape: &GeometryHandle, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.rotate_sync(shape, axis, angle)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: EVec3) -> Result<GeometryHandle, BrepError> {
        self.scale_sync(shape, factor, center)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mirror(&mut self, shape: &GeometryHandle, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        self.mirror_sync(shape, origin, normal)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn copy_shape(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.copy_shape_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn linear_pattern(&mut self, shape: &GeometryHandle, direction: EVec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        self.linear_pattern_sync(shape, direction, spacing, count)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn circular_pattern(&mut self, shape: &GeometryHandle, axis: EVec3, count: usize) -> Result<GeometryHandle, BrepError> {
        self.circular_pattern_sync(shape, axis, count)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn grid_pattern(&mut self, shape: &GeometryHandle, dir_x: EVec3, dir_y: EVec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        self.grid_pattern_sync(shape, dir_x, dir_y, spacing_x, spacing_y, count_x, count_y)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_sync(shape, radius)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fillet_variable(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_variable_sync(shape, radius_start, radius_end)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fillet_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_edges_sync(shape, edges, radius)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, distance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chamfer_asymmetric(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_asymmetric_sync(shape, d1, d2)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chamfer_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_edges_sync(shape, edges, distance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn shell(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.shell_sync(shape, thickness, open_faces)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn draft(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: EVec3, neutral_point: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.draft_sync(shape, faces, pull_direction, neutral_point, angle)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn offset_solid(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_solid_sync(shape, distance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn defeature(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.defeature_sync(shape, faces)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn section(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<Vec<GeometryHandle>, BrepError> {
        self.section_sync(solid, plane_origin, plane_normal)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn split(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        self.split_sync(solid, plane_origin, plane_normal)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_curve_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        self.curve_curve_intersect_sync(a, b, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_surface_intersect(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        self.curve_surface_intersect_sync(curve, surface, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_surface_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        self.surface_surface_intersect_sync(a, b, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_point(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        self.curve_point_sync(curve, parameter)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_tangent(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        self.curve_tangent_sync(curve, parameter)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_domain(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        self.curve_domain_sync(curve)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_curvature(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        self.curve_curvature_sync(curve, parameter)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_point(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        self.surface_point_sync(surface, u, v)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_normal(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        self.surface_normal_sync(surface, u, v)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.volume_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn area(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.area_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn length(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.length_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn center_of_mass(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        self.center_of_mass_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn bounding_box(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.bounding_box_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn distance(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        self.distance_sync(a, b)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn closest_point(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        self.closest_point_sync(shape, point)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn classify_point(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        self.classify_point_sync(solid, point)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validate(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        self.validate_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn vertex(&mut self, point: EVec3) -> Result<GeometryHandle, BrepError> {
        self.vertex_sync(point)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.face_from_wire_sync(wire)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sew_faces(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.sew_faces_sync(faces, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn heal_solid(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.heal_solid_sync(shape, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn convert_to_nurbs(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.convert_to_nurbs_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn deconstruct(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        self.deconstruct_sync(shape)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_step(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        self.export_step_sync(shapes)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_stl_sync(shapes, deflection)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_obj(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        self.export_obj_sync(shapes, deflection)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_gltf(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_gltf_sync(shapes, deflection)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_step(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        self.import_step_sync(data)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_stl(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_stl_sync(data, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_obj(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_obj_sync(data, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn export_dwg(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_dwg_sync(shapes, deflection)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_dwg(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_dwg_sync(data, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        self.kind_sync(handle)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        self.tessellate_sync(handle, tolerance)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dispose(&mut self, handle: &GeometryHandle) {
        let _ = self.dispose_sync(handle);
    }
    /// ♻️ Equivalent to disposing every handle not in `live`, then compacting once — see
    /// [`Brep::compact_unreachable`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn retain(&mut self, live: &std::collections::HashSet<String>) {
        self.live.retain(|k, _| live.contains(k));
        self.compact_unreachable();
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn registry_len(&self) -> usize {
        self.live.len()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn solid_shells(&mut self, solid: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        self.solid_shells_sync(solid)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn compound(&mut self, solids: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.compound_sync(solids)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn explode(&mut self, compound: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        self.explode_sync(compound)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn label(&self, handle: &GeometryHandle) -> Option<u64> {
        self.label_of(handle).map(|l| l.0)
    }
}

// #endregion 🔖️BrepKernelImpl

// #region 🔌️Codecs

/// 🔌️ Format-keyed solid export codec.
pub trait SolidExporter: Send + Sync {
    async fn format_kind(&self) -> &'static str;
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
}

/// 🔌️ Format-keyed solid import codec.
pub trait SolidImporter: Send + Sync {
    async fn format_kind(&self) -> &'static str;
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError>;
}

pub struct StepSolidExporter;
pub struct StepSolidImporter;
pub struct StlSolidExporter;
pub struct StlSolidImporter;
pub struct ObjSolidExporter;
pub struct ObjSolidImporter;
pub struct GlbSolidExporter;
pub struct GlbSolidImporter;

impl SolidExporter for StepSolidExporter {
    async fn format_kind(&self) -> &'static str {
        "step"
    }
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], _deflection: f64) -> Result<Vec<u8>, BrepError> {
        Ok(kernel.export_step_sync(shapes)?.into_bytes())
    }
}
impl SolidImporter for StepSolidImporter {
    async fn format_kind(&self) -> &'static str {
        "step"
    }
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], _tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|e| BrepError::InvalidInput(e.to_string()))?;
        kernel.import_step_sync(text)
    }
}
impl SolidExporter for StlSolidExporter {
    async fn format_kind(&self) -> &'static str {
        "stl"
    }
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_stl_sync(shapes, deflection)
    }
}
impl SolidImporter for StlSolidImporter {
    async fn format_kind(&self) -> &'static str {
        "stl"
    }
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        Ok(vec![kernel.import_stl_sync(bytes, tolerance)?])
    }
}
impl SolidExporter for ObjSolidExporter {
    async fn format_kind(&self) -> &'static str {
        "obj"
    }
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        Ok(kernel.export_obj_sync(shapes, deflection)?.into_bytes())
    }
}
impl SolidImporter for ObjSolidImporter {
    async fn format_kind(&self) -> &'static str {
        "obj"
    }
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|e| BrepError::InvalidInput(e.to_string()))?;
        Ok(vec![kernel.import_obj_sync(text, tolerance)?])
    }
}
impl SolidExporter for GlbSolidExporter {
    async fn format_kind(&self) -> &'static str {
        "glb"
    }
    async fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_glb_sync(shapes, deflection)
    }
}
impl SolidImporter for GlbSolidImporter {
    async fn format_kind(&self) -> &'static str {
        "glb"
    }
    async fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        Ok(vec![kernel.import_glb_sync(bytes, tolerance)?])
    }
}

// #endregion 🔌️Codecs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brep_error_contract_is_owned_and_stable() {
        let errors = [(BrepError::InvalidInput("mesh".into()), "invalid input: mesh"), (BrepError::MissingHandle("a1".into()), "missing handle: a1"), (BrepError::Operation("split".into()), "operation failed: split")];
        for (error, message) in errors {
            assert_eq!(error.to_string(), message);
            assert!(std::error::Error::source(&error).is_none());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn native_box_volume() {
        let mut k = Brep::new();
        let solid = k.box_prim(1.0, 1.0, 1.0).unwrap();
        let v = k.volume(&solid).unwrap();
        assert!((v - 1.0).abs() < 1e-3, "volume {v}");
    }

    #[semio_framework_async_macros::async_test]
    async fn native_fuse_disjoint() {
        let mut k = Brep::new();
        let a = k.box_prim(1.0, 1.0, 1.0).unwrap();
        let b = k.convex_hull(&[[2.0, 0.0, 0.0], [3.0, 0.0, 0.0], [3.0, 1.0, 0.0], [2.0, 1.0, 0.0], [2.0, 0.0, 1.0], [3.0, 0.0, 1.0], [3.0, 1.0, 1.0], [2.0, 1.0, 1.0]]).unwrap();
        let u = k.fuse(&a, &b).unwrap();
        let v = k.volume(&u).unwrap();
        assert!((v - 2.0).abs() < 1e-2, "volume {v}");
    }

    #[semio_framework_async_macros::async_test]
    async fn wire_tessellate_preserves_edge_positions() {
        let mut k = Brep::new();
        let wire = k.rectangle_wire(2.0, 1.5).expect("wire");
        let transfer = k.tessellate_sync(&wire, 0.1).expect("tessellate");
        let data = mesh_data_from_mesh_transfer(&transfer);
        assert!(data.edge_positions.len() >= 24, "edge_positions {}", data.edge_positions.len());
        assert!(data.indices.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn box_shell_produces_positive_volume() {
        let mut k = Brep::new();
        let box_h = k.box_prim(2.0, 2.0, 2.0).expect("box");
        let shelled = k.shell(&box_h, 0.2, &[]).expect("shell");
        let vol = k.volume(&shelled).expect("shell volume");
        assert!(vol > 0.0, "shelled volume {vol}");
        let mesh = k.tessellate_sync(&shelled, 0.1).expect("tessellate shell");
        assert!(!mesh.position.is_empty() || !mesh.edges.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn sphere_torus_cut_produces_preview_mesh() {
        let mut k2 = Brep::new();
        let sphere = k2.sphere_prim(2.2).expect("sphere");
        let torus = k2.torus_prim(2.0, 0.5).expect("torus");
        let tv = k2.volume(&torus).expect("torus volume");
        assert!(tv > 0.5, "torus volume too small: {tv}");
        let tmesh = k2.tessellate_sync(&torus, 0.15).expect("tessellate torus");
        assert!(tmesh.position.len() >= 9 && tmesh.index.len() >= 3, "torus mesh empty");
        let cut = k2.cut(&sphere, &torus).expect("cut");
        let mesh = k2.tessellate_sync(&cut, 0.15).expect("tessellate cut");
        assert!(mesh.position.len() >= 9 && mesh.index.len() >= 3, "cut mesh empty: pos={} idx={}", mesh.position.len(), mesh.index.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn arc_curve_respects_start_end_angles() {
        let mut k = Brep::new();
        let start = 0.0;
        let end = std::f64::consts::FRAC_PI_2;
        let radius = 2.0;
        let arc = k.arc_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], radius, start, end).expect("arc");
        let domain = k.curve_domain_sync(&arc).expect("domain");
        assert!((domain.min - start).abs() < 1e-9 && (domain.max - end).abs() < 1e-9);
        let p0 = k.curve_point_sync(&arc, start).expect("p0");
        let p1 = k.curve_point_sync(&arc, end).expect("p1");
        let r0 = (p0[0] * p0[0] + p0[1] * p0[1] + p0[2] * p0[2]).sqrt();
        let r1 = (p1[0] * p1[0] + p1[1] * p1[1] + p1[2] * p1[2]).sqrt();
        assert!((r0 - radius).abs() < 1e-4, "start radius {r0} from {p0:?}");
        assert!((r1 - radius).abs() < 1e-4, "end radius {r1} from {p1:?}");
        let chord = ((p1[0] - p0[0]).powi(2) + (p1[1] - p0[1]).powi(2) + (p1[2] - p0[2]).powi(2)).sqrt();
        let expected_chord = (2.0 * radius * radius * (1.0 - (end - start).cos())).sqrt();
        assert!((chord - expected_chord).abs() < 1e-3, "chord {chord} expected {expected_chord}");
        // Full circle would land start≈end; a quarter arc must keep endpoints distinct.
        assert!(chord > radius * 0.5);
        let kappa = k.curve_curvature_sync(&arc, (start + end) * 0.5).expect("kappa");
        assert!((kappa - 1.0 / radius).abs() < 5e-2, "kappa {kappa}");
    }

    #[semio_framework_async_macros::async_test]
    async fn solid_face_loops_returns_a_quad_per_box_face() {
        let mut k = Brep::new();
        let solid = k.box_prim_sync(2.0, 3.0, 4.0).expect("box");
        let (positions, face_loops) = k.solid_face_loops_sync(&solid).expect("loops");
        assert_eq!(positions.len(), 8, "a box has 8 distinct vertices");
        assert_eq!(face_loops.len(), 6, "a box has 6 faces");
        for (outer, holes) in &face_loops {
            assert_eq!(outer.len(), 4, "each box face is a quad");
            assert!(holes.is_empty());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_returns_structured_json_report() {
        let mut k = Brep::new();
        let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
        let report = k.validate_sync(&solid).expect("validate");
        let value: serde_json::Value = serde_json::from_str(&report).expect("json");
        assert_eq!(value["ok"], true);
        assert_eq!(value["issueCount"], 0);
        assert!(value["issues"].as_array().unwrap().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn deconstruct_includes_vertices_edges_and_faces() {
        let mut k = Brep::new();
        let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
        let topo = k.deconstruct_sync(&solid).expect("deconstruct");
        assert_eq!(topo.faces.len(), 6);
        assert_eq!(topo.edges.len(), 12);
        assert_eq!(topo.vertices.len(), 8);
    }

    /// 🏷️ Deconstruct must be idempotent: minting from each entity's [`PersistentLabel`] (not a
    /// session counter) means calling it twice on the same untouched shape yields byte-identical
    /// handles, and shells are now included (audit §5.4 — shell was not a first-class handle kind).
    #[semio_framework_async_macros::async_test]
    async fn deconstruct_twice_yields_identical_handles_and_includes_shells() {
        let mut k = Brep::new();
        let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
        let first = k.deconstruct_sync(&solid).expect("first deconstruct");
        let second = k.deconstruct_sync(&solid).expect("second deconstruct");
        assert_eq!(first.shells.len(), 1, "a box has exactly one outer shell");
        assert_eq!(first.vertices, second.vertices, "deconstruct must be idempotent per PersistentLabel");
        assert_eq!(first.edges, second.edges);
        assert_eq!(first.faces, second.faces);
        assert_eq!(first.shells, second.shells);
    }

    /// 🏷️ Registering unrelated geometry between the two calls must not perturb a single handle —
    /// under the old counter-based `mint`, this alone would have changed every handle the second
    /// `deconstruct` produces (audit §5.1: "deconstructing the same body repeatedly can mint new
    /// handles repeatedly").
    #[semio_framework_async_macros::async_test]
    async fn deconstruct_handles_are_unaffected_by_unrelated_registrations_between_calls() {
        let mut k = Brep::new();
        let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
        let before = k.deconstruct_sync(&solid).expect("deconstruct before");
        let _ = k.line_curve_sync([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).expect("unrelated curve");
        let _ = k.sphere_prim_sync(0.5).expect("unrelated sphere");
        let after = k.deconstruct_sync(&solid).expect("deconstruct after");
        assert_eq!(before.vertices, after.vertices);
        assert_eq!(before.edges, after.edges);
        assert_eq!(before.faces, after.faces);
        assert_eq!(before.shells, after.shells);
    }

    /// ♻️ Disposing the only handle reaching a body's geometry must actually free it (audit §5.3:
    /// "dispose is not equivalent to deleting geometry" under the old registry-only dispose).
    #[semio_framework_async_macros::async_test]
    async fn dispose_reclaims_unreferenced_topology() {
        let mut k = Brep::new();
        let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
        k.dispose(&solid);
        assert_eq!(k.registry_len(), 0);
        let counts = k.body.entity_counts();
        assert_eq!(counts.vertices, 0);
        assert_eq!(counts.edges, 0);
        assert_eq!(counts.faces, 0);
        assert_eq!(counts.shells, 0);
        assert_eq!(counts.solids, 0);
    }

    /// ♻️ `retain` is dispose-of-everything-else-then-compact: the dropped solid's own geometry
    /// must be freed while the kept solid stays fully intact and resolvable.
    #[semio_framework_async_macros::async_test]
    async fn retain_compacts_everything_not_kept() {
        let mut k = Brep::new();
        let keep = k.box_prim_sync(1.0, 1.0, 1.0).expect("keep");
        let drop = k.box_prim_sync(2.0, 2.0, 2.0).expect("drop");
        let mut live = std::collections::HashSet::new();
        live.insert(keep.as_str().to_string());
        k.retain(&live);
        assert_eq!(k.registry_len(), 1);
        assert!(k.kind(&keep).is_ok());
        assert!(k.kind(&drop).is_err(), "the dropped handle must no longer resolve");
        let vol = k.volume(&keep).expect("kept solid stays intact");
        assert!((vol - 1.0).abs() < 1e-6, "volume {vol}");
    }

    /// 🧩️ `import_step` merges into the existing body (audit §5.2's required fix) rather than
    /// replacing it: a handle minted before the import must still resolve, and the original
    /// solid's own geometry must be untouched by the merge.
    #[semio_framework_async_macros::async_test]
    async fn import_step_merges_and_keeps_prior_handles_resolvable() {
        let mut k = Brep::new();
        let original = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
        let step_text = k.export_step_sync(std::slice::from_ref(&original)).expect("export");
        let imported = k.import_step_sync(&step_text).expect("import");
        assert!(k.kind(&original).is_ok(), "a handle minted before import must still resolve");
        assert!((k.volume(&original).unwrap() - 1.0).abs() < 1e-6, "the original solid's own geometry must be untouched");
        assert_eq!(imported.len(), 1);
        assert!((k.volume(&imported[0]).unwrap() - 1.0).abs() < 1e-2, "the round-tripped solid should have the same volume");
    }

    /// 🧩️ Shell/compound are first-class handle kinds now (audit §5.4); `explode` is `compound`'s
    /// inverse.
    #[semio_framework_async_macros::async_test]
    async fn solid_shells_compound_and_explode_round_trip() {
        let mut k = Brep::new();
        let a = k.box_prim_sync(1.0, 1.0, 1.0).expect("a");
        let b = k.box_prim_sync(1.0, 1.0, 1.0).expect("b");

        let shells = k.solid_shells(&a).expect("shells");
        assert_eq!(shells.len(), 1);
        assert_eq!(k.kind(&shells[0]).unwrap(), GeometryKind::Shell);

        let compound = k.compound(&[a.clone(), b.clone()]).expect("compound");
        assert_eq!(k.kind(&compound).unwrap(), GeometryKind::Compound);

        let members = k.explode(&compound).expect("explode");
        assert_eq!(members.len(), 2);
        for m in &members {
            assert!((k.volume(m).unwrap() - 1.0).abs() < 1e-6);
        }
    }

    /// 🏷️ `label`/`handle_for_label` are the ephemeral-handle↔persistent-label bridge the audit's
    /// §5.5 required fix asks for.
    #[semio_framework_async_macros::async_test]
    async fn label_and_handle_for_label_round_trip() {
        let mut k = Brep::new();
        let solid = k.box_prim_sync(1.0, 1.0, 1.0).expect("box");
        let label = k.label(&solid).expect("solid must carry a label");
        let via_label = k.handle_for_label(PersistentLabel(label));
        assert_eq!(via_label, Some(solid));
    }
}
