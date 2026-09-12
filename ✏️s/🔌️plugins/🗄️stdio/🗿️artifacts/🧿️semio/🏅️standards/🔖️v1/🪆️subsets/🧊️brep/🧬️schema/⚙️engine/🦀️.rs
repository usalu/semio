//! 🧠 Native B-Rep kernel: consumer contract (`BrepKernel`, `GeometryHandle`, `GeometryKind`,
//! `block_on`) plus its sole implementor `Brep`, which delegates every operation to
//! `semio_framework_3d::engine::*`'s pure algorithm modules over a `&mut Body`/`&Body` arena.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/{⚙️engine,🧰️kernel}` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave G5 ("the brep flip") — this is
//! the temporary forward edge (`stdio → semio-framework-3d`) that lets the other framework-3d
//! brep subdirs (arena, topology, boolean, tessellate, euler, …) peel into `🧊️brep`'s compute
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
//! 📦️ `mesh_io` converts between kernel topology and meshes through supplied codec interfaces.
//! Format-specific implementations belong to the artifact I/O tree.
//!
//! 📄️ `🟫️step` (below) moved IN wave PEEL3: framework-3d's hand-rolled ISO 10303-21 Part-21
//! reader/writer, the last thing this file's `export_step`/`import_step` needed from the
//! framework side. Known pre-existing duplicate of stdio's own, separately-complete AP214
//! `SemioBrepToStep`/`SemioBrepFromStep` walk (`🧊️brep/🚪️io`) — reconciling the two requires
//! rewiring this file's `BrepKernel` impl, which is out of scope here (see `📌️important.md`,
//! "BrepKernel — do NOT attempt"). Relocated verbatim only, to satisfy the crate-direction law
//! now that arena/topo/euler/history/primitives moved into this same crate too.
/// 🧱 Vertex positions with outer and inner vertex loops for each face.
pub type SolidFaceLoops = (Vec<[f32; 3]>, Vec<(Vec<u32>, Vec<Vec<u32>>)>);

use std::collections::HashMap;

#[path = "📦️mesh-io/🦀️.rs"]
mod mesh_io;

#[path = "🟫️step/🦀️.rs"]
mod step;

#[path = "🔖️contract/🦀️.rs"]
pub mod contract;
pub use contract::*;

use crate::standards::v1::subsets::brep::schema::diff::blend::{chamfer_edges, fillet_edges, fillet_variable};
use crate::standards::v1::subsets::brep::schema::diff::boolean::{boolean_job, boolean_solid, compound_cut, section_solid_by_plane, split_solid_by_plane, BooleanAdmission, BooleanJob, BooleanOp, BooleanProgress, BooleanStep};
use crate::standards::v1::subsets::brep::schema::diff::euler::make_vertex;
use crate::standards::v1::subsets::brep::schema::diff::intersect::intersect_curve_curve;
use crate::standards::v1::subsets::brep::schema::diff::intersect::intersect_curve_surface;
use crate::standards::v1::subsets::brep::schema::diff::intersect::intersect_surface_surface;
use crate::standards::v1::subsets::brep::schema::diff::offset::{draft_angle, offset_face, offset_solid, shell_solid_with_open_faces, thicken_face};
use crate::standards::v1::subsets::brep::schema::diff::primitives::{
    make_box, make_cone, make_convex_hull, make_cylinder, make_planar_face_from_points, make_planar_face_from_wire, make_polyline_wire, make_rectangle_wire, make_regular_polygon_wire, make_sphere, make_torus, Wire,
};
use crate::standards::v1::subsets::brep::schema::diff::sew::{convert_to_nurbs, defeature, heal_solid, sew_faces};
use crate::standards::v1::subsets::brep::schema::diff::sweep::{extrude_face, helical_sweep, loft_profiles, pipe, revolve_face, sweep_along_path};
use crate::standards::v1::subsets::brep::schema::diff::transform::{copy_solid, transform_face, transform_solid, transform_wire};
use crate::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid;
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::{closest_point_on_solid, distance_solid_solid, edge_length, face_area, solid_bounding_box, solid_center_of_mass, solid_surface_area, solid_volume};
use crate::standards::v1::subsets::brep::schema::inferences::tessellation::{tessellate_face, tessellate_solid, tessellate_wire, TessellationJob};
use crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, EdgeId, FaceId, SolidId, VertexId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::{approximate_curve_with_count, closest_parameter as curve_closest_parameter_fn, coons_patch_nurbs, interpolate_curve, interpolate_surface_grid, ParamMethod};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::standards::v1::subsets::brep::schema::snapshot::error::{KernelError, ValidationIssue};
use crate::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv as surface_closest_uv_fn;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::{Affine3, Frame3};
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3 as NativeVec3};
use contract::Vec3 as EVec3;
use mesh_io::{export_solid_glb, export_solid_mesh, export_solid_obj, export_solid_stl, import_glb_to_body, import_mesh_to_body, import_obj_to_body, import_stl_to_body, mesh_to_mesh_data, triangle_mesh_from_transfer};
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
    /// 🔁 Rotates about an explicit `origin` — [`Self::rotate`] has no origin parameter and always
    /// rotates about the world origin (`(0,0,0)`), never a shape's bounding-box center.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn rotate_about(&mut self, shape: &GeometryHandle, origin: Vec3, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
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
    /// 🧭️ Certified closest parameter on `curve` to `point`: `(t, point, distance)`. Analytic
    /// closed forms for line/circle/ellipse, Bézier-span subdivision + Newton for NURBS.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve_closest_parameter(&self, curve: &GeometryHandle, point: Vec3) -> Result<(f64, Vec3, f64), BrepError>;
    /// 🧭️ Certified closest `(u, v)` on `surface` to `point`: `(u, v, point, distance)`. Analytic
    /// closed forms for every analytic surface kind (poles/apex/both torus periods handled),
    /// Bézier-patch subdivision + damped 2D Newton for NURBS.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_closest_uv(&self, surface: &GeometryHandle, point: Vec3) -> Result<(f64, f64, Vec3, f64), BrepError>;
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
    fn export_mesh(&self, shapes: &[GeometryHandle], deflection: f64, exporter: &dyn semio_framework_mesh_engine::MeshExporter) -> Result<Vec<u8>, BrepError>;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_mesh(&mut self, data: &[u8], tolerance: f64, importer: &dyn semio_framework_mesh_engine::MeshImporter) -> Result<GeometryHandle, BrepError>;
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

use crate::standards::v1::subsets::brep::schema::snapshot::arena::ShellId;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::EntityRef;

/// 🧠 One live registry entry. Vertex/Edge/Face/🐚️Shell/Solid wrap the arena id whose own
/// [`crate::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]
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

/// ⏱️ A retained resumable boolean plus the operation recorder its whole run accumulates into —
/// the recorder must outlive every step, which is why the job owns it rather than borrowing the
/// caller's (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
pub struct BrepBooleanJob {
    job: BooleanJob,
    rec: OpRecorder,
}

impl BrepBooleanJob {
    /// 📈️ Progress right now — safe to read between steps and after termination.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn progress(&self) -> BooleanProgress {
        self.job.progress()
    }

    /// 🛑️ Retires this boolean at the next observable boundary.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cancel(&mut self) {
        self.job.cancel();
    }

    /// ✅️ True once the job reached a terminal phase.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_terminal(&self) -> bool {
        self.job.is_terminal()
    }
}

/// 🔀️ What [`Brep::boolean_job_sync`] admitted.
pub enum BrepBooleanAdmission {
    /// ⚡️ A fast path answered exactly and in place; there is nothing to resume.
    Answered(GeometryHandle),
    /// ⏱️ The general exact engine, as a budgetable job.
    Job(BrepBooleanJob),
}

/// ⏱️ Outcome of one [`Brep::step_boolean_job_sync`].
pub enum BrepBooleanStep {
    /// 🔁 Budget spent, work remains.
    Working(BooleanProgress),
    /// ✅️ Terminal: the result solid, registered as a live handle.
    Ready(GeometryHandle),
    /// 🛑️ Terminal: the job was cancelled; nothing is produced.
    Cancelled(BooleanProgress),
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
fn map_err(e: &KernelError) -> BrepError {
    BrepError::Operation(e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn map_step(e: &crate::standards::v1::subsets::brep::schema::snapshot::error::StepError) -> BrepError {
    BrepError::Operation(e.to_string())
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
/// handle from. Vertex/Edge/Face/🐚️Shell/Solid read it back out of the `Body` arena entry they wrap
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

    /// ♻️ [`crate::standards::v1::subsets::brep::schema::snapshot::topology::EntityRef`] roots for every entity currently kept alive by a live
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
        let solid = make_box(&mut self.body, width, depth, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sphere_prim_sync(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_sphere(&mut self.body, radius, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cylinder_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cylinder(&mut self.body, radius, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cone_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cone(&mut self.body, radius, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn torus_prim_sync(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_torus(&mut self.body, major, minor, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn convex_hull_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let solid = make_convex_hull(&mut self.body, &pts, &mut rec).map_err(|error| map_err(&error))?;
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
        let wire = make_polyline_wire(&mut self.body, &pts, false, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rectangle_wire_sync(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut self.body, width, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn regular_polygon_wire_sync(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_regular_polygon_wire(&mut self.body, radius, sides, &mut rec).map_err(|error| map_err(&error))?;
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
        let face = make_planar_face_from_points(&mut self.body, &pts, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_face(face))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn planar_face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let w = self.wire_ref(wire)?.clone();
        let origin = self.body.vertices.get(w.vertices[0]).map_or(Pnt3::new(0.0, 0.0, 0.0), |v| v.position);
        let mut rec = OpRecorder::new();
        let face = make_planar_face_from_wire(&mut self.body, &w, origin, NativeVec3::Z, &mut rec).map_err(|error| map_err(&error))?;
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
        let out = offset_face(&mut self.body, id, distance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_face(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn thicken_face_sync(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let mut rec = OpRecorder::new();
        let solid = thicken_face(&mut self.body, id, thickness, &mut rec).map_err(|error| map_err(&error))?;
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
        let solid = extrude_face(&mut self.body, id, vec3(direction), distance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn revolve_sync(&mut self, face: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let mut rec = OpRecorder::new();
        let solid = revolve_face(&mut self.body, id, pnt(axis_origin), vec3(axis_direction), angle, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn loft_sync(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        let mut faces = Vec::new();
        for p in profiles {
            faces.push(self.face_id(p)?);
        }
        let mut rec = OpRecorder::new();
        let solid = loft_profiles(&mut self.body, &faces, smooth, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sweep_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let wire = self.wire_ref(path)?.clone();
        let mut rec = OpRecorder::new();
        let solid = sweep_along_path(&mut self.body, face, &wire, &mut rec).map_err(|error| map_err(&error))?;
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
        let solid = pipe(&mut self.body, face, &wire, g.as_ref(), &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn helical_sweep_sync(&mut self, profile: &GeometryHandle, axis_origin: EVec3, axis_dir: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let mut rec = OpRecorder::new();
        let solid = helical_sweep(&mut self.body, face, (pnt(axis_origin), vec3(axis_dir)), radius, pitch, turns, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fuse_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Unite, 1e-6, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cut_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Cut, 1e-6, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }

    /// 🔀️ The one-shot set operation, over the SAME road [`Brep::boolean_job_sync`] takes — so a
    /// caller that cannot afford a job and a caller that drives one cannot diverge.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn boolean_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, op: BooleanOp) -> Result<GeometryHandle, BrepError> {
        match self.boolean_job_sync(a, b, op)? {
            BrepBooleanAdmission::Answered(handle) => Ok(handle),
            BrepBooleanAdmission::Job(mut job) => loop {
                match self.step_boolean_job_sync(&mut job, usize::MAX)? {
                    BrepBooleanStep::Ready(handle) => return Ok(handle),
                    BrepBooleanStep::Cancelled(_) => return Err(BrepError::Operation("boolean cancelled".to_string())),
                    BrepBooleanStep::Working(_) => continue,
                }
            },
        }
    }

    /// ⏱️ A resumable, budgetable boolean of `a` and `b` — the interactive twin of
    /// [`Brep::cut_sync`]/[`Brep::fuse_sync`]/[`Brep::intersect_sync`], and the exact analogue of
    /// [`Brep::tessellate_job_sync`]. The microsecond-cheap fast paths are taken here and answer
    /// immediately; everything else comes back as a job the caller drives with
    /// [`Brep::step_boolean_job_sync`] so no single call outruns an interactive step ceiling
    /// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn boolean_job_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, op: BooleanOp) -> Result<BrepBooleanAdmission, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        match boolean_job(&mut self.body, sa, sb, op, 1e-6, &mut rec).map_err(|error| map_err(&error))? {
            BooleanAdmission::Answered(solid) => Ok(BrepBooleanAdmission::Answered(self.register_solid(solid))),
            BooleanAdmission::Job(job) => Ok(BrepBooleanAdmission::Job(BrepBooleanJob { job, rec })),
        }
    }

    /// ⏱️ Advances `job` by at most `budget` units against THIS kernel's topology. The job borrows
    /// nothing, so a host may retain it across turns and re-present the kernel on every step — the
    /// same contract [`Brep::tessellation_body`] states for tessellation.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn step_boolean_job_sync(&mut self, job: &mut BrepBooleanJob, budget: usize) -> Result<BrepBooleanStep, BrepError> {
        match job.job.step(&mut self.body, &mut job.rec, budget).map_err(|error| map_err(&error))? {
            BooleanStep::Working(progress) => Ok(BrepBooleanStep::Working(progress)),
            BooleanStep::Cancelled(progress) => Ok(BrepBooleanStep::Cancelled(progress)),
            BooleanStep::Done(solid) => Ok(BrepBooleanStep::Ready(self.register_solid(solid))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let mut rec = OpRecorder::new();
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Intersect, 1e-6, &mut rec).map_err(|error| map_err(&error))?;
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
        let solid = compound_cut(&mut self.body, t, &ids, 1e-6, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    /// 🔁 Dispatches an exact affine transform to whichever geometry kind `shape` resolves to: a
    /// solid (topology deep copy, [`transform_solid`]), a face ([`transform_face`]), a wire
    /// ([`transform_wire`]), or a bare curve/surface ([`Curve3`]/[`Surface::transformed`] directly
    /// — cheap, since neither lives in `self.body`). Vertex/Edge/🐚️Shell/Compound handles are out of
    /// this ticket's scope (audit §6.2 only covers solids/faces/wires/curves/surfaces).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transform_shape_sync(&mut self, shape: &GeometryHandle, map: &Affine3) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        if let Ok(id) = self.solid_id(shape) {
            let out = transform_solid(&mut self.body, id, map, &mut rec).map_err(|error| map_err(&error))?;
            return Ok(self.register_solid(out));
        }
        if let Ok(id) = self.face_id(shape) {
            let out = transform_face(&mut self.body, id, map, &mut rec).map_err(|error| map_err(&error))?;
            return Ok(self.register_face(out));
        }
        if let Ok(wire) = self.wire_ref(shape) {
            let wire = wire.clone();
            let out = transform_wire(&mut self.body, &wire, map, &mut rec);
            return Ok(self.register_wire(out));
        }
        if let Ok(curve) = self.curve_ref(shape) {
            let out = curve.transformed(map);
            return Ok(self.register_curve(out));
        }
        if let Ok(surface) = self.surface_ref(shape) {
            let out = surface.transformed(map);
            return Ok(self.register_surface(out));
        }
        Err(BrepError::InvalidInput(format!("{} is not a solid/face/wire/curve/surface", shape.as_str())))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn translate_sync(&mut self, shape: &GeometryHandle, offset: EVec3) -> Result<GeometryHandle, BrepError> {
        self.transform_shape_sync(shape, &Affine3::translation(vec3(offset)))
    }
    /// 🔁 Rotates about the WORLD origin, not `shape`'s bounding-box center — the trait's `rotate`
    /// carries no explicit origin. Use [`Self::rotate_about_sync`] for rotation about an arbitrary
    /// point.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rotate_sync(&mut self, shape: &GeometryHandle, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.transform_shape_sync(shape, &Affine3::rotation_axis_angle(vec3(axis), angle))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rotate_about_sync(&mut self, shape: &GeometryHandle, origin: EVec3, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.transform_shape_sync(shape, &Affine3::rotation_about(pnt(origin), vec3(axis), angle))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn scale_sync(&mut self, shape: &GeometryHandle, factor: f64, center: EVec3) -> Result<GeometryHandle, BrepError> {
        self.transform_shape_sync(shape, &Affine3::scaling(pnt(center), NativeVec3::new(factor, factor, factor)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mirror_sync(&mut self, shape: &GeometryHandle, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        self.transform_shape_sync(shape, &Affine3::mirror(pnt(origin), vec3(normal)))
    }

    /// 🔁 A solid copies via [`copy_solid`] (its dedicated identity-transform entry point); every
    /// other supported kind copies via [`Self::transform_shape_sync`] under [`Affine3::IDENTITY`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn copy_shape_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        if let Ok(id) = self.solid_id(shape) {
            let mut rec = OpRecorder::new();
            let out = copy_solid(&mut self.body, id, &mut rec).map_err(|error| map_err(&error))?;
            return Ok(self.register_solid(out));
        }
        self.transform_shape_sync(shape, &Affine3::IDENTITY)
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
    pub fn grid_pattern_sync(&mut self, shape: &GeometryHandle, dir_x: EVec3, dir_y: EVec3, (spacing_x, spacing_y): (f64, f64), count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
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
        let out = fillet_edges(&mut self.body, solid, &edges, radius, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fillet_variable_sync(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let e = *edges.first().ok_or_else(|| BrepError::InvalidInput("no edges".into()))?;
        let mut rec = OpRecorder::new();
        let out = fillet_variable(&mut self.body, solid, e, radius_start, radius_end, &mut rec).map_err(|error| map_err(&error))?;
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
        let out = fillet_edges(&mut self.body, solid, &eids, radius, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let mut rec = OpRecorder::new();
        let out = chamfer_edges(&mut self.body, solid, &edges, distance, distance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_asymmetric_sync(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let mut rec = OpRecorder::new();
        let out = chamfer_edges(&mut self.body, solid, &edges, d1, d2, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
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
        let out = chamfer_edges(&mut self.body, solid, &eids, distance, distance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn shell_sync(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let open_ids: Vec<_> = open_faces.iter().filter_map(|h| self.face_id(h).ok()).collect();
        let mut rec = OpRecorder::new();
        let out = shell_solid_with_open_faces(&mut self.body, solid, thickness.abs(), &open_ids, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn draft_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: EVec3, neutral_point: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut fids = Vec::new();
        for f in faces {
            fids.push(self.face_id(f)?);
        }
        if fids.is_empty() {
            fids = self.body.solid_faces(solid);
        }
        let mut rec = OpRecorder::new();
        let out = draft_angle(&mut self.body, solid, &fids, vec3(pull_direction), (pnt(neutral_point), vec3(pull_direction)), angle, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn offset_solid_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let out = offset_solid(&mut self.body, solid, distance, &mut rec).map_err(|error| map_err(&error))?;
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
        let out = defeature(&mut self.body, solid, &fids, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(out))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn section_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<Vec<GeometryHandle>, BrepError> {
        let id = self.solid_id(solid)?;
        let mut rec = OpRecorder::new();
        let faces = section_solid_by_plane(&mut self.body, id, pnt(plane_origin), vec3(plane_normal), 1e-6, &mut rec).map_err(|error| map_err(&error))?;
        Ok(faces.into_iter().map(|f| self.register_face(f)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn split_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        let id = self.solid_id(solid)?;
        let mut rec = OpRecorder::new();
        let (a, b) = split_solid_by_plane(&mut self.body, id, pnt(plane_origin), vec3(plane_normal), 1e-6, &mut rec).map_err(|error| map_err(&error))?;
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
    /// 🧭️ Certified closest parameter on `curve` to `point` — see [`BrepKernel::curve_closest_parameter`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_closest_parameter_sync(&self, curve: &GeometryHandle, point: EVec3) -> Result<(f64, EVec3, f64), BrepError> {
        let c = self.curve_ref(curve)?;
        let cp = curve_closest_parameter_fn(c, c.domain(), pnt(point), 1e-9);
        Ok((cp.t, evec(cp.point), cp.distance))
    }
    /// 🧭️ Certified closest `(u, v)` on `surface` to `point` — see [`BrepKernel::surface_closest_uv`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_closest_uv_sync(&self, surface: &GeometryHandle, point: EVec3) -> Result<(f64, f64, EVec3, f64), BrepError> {
        let s = self.surface_ref(surface)?;
        let cp = surface_closest_uv_fn(s, s.domain(), pnt(point), 1e-9);
        Ok((cp.u, cp.v, evec(cp.point), cp.distance))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let solid = self.solid_id(shape)?;
        solid_volume(&self.body, solid, 1e-4).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn area_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => solid_surface_area(&self.body, *id, 1e-4).map_err(|error| map_err(&error)),
            Entity::Face(id) => face_area(&self.body, *id, 1e-4).map_err(|error| map_err(&error)),
            _ => Err(BrepError::InvalidInput("area requires solid or face".into())),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn length_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let edge = self.edge_id(shape)?;
        edge_length(&self.body, edge).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn center_of_mass_sync(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        let solid = self.solid_id(shape)?;
        Ok(evec(solid_center_of_mass(&self.body, solid, 1e-4).map_err(|error| map_err(&error))?))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn bounding_box_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(|error| map_err(&error))?;
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
        distance_solid_solid(&self.body, sa, sb).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn closest_point_sync(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        match self.entity(shape)? {
            Entity::Curve(_, _) => {
                let (t, p, d) = self.curve_closest_parameter_sync(shape, point)?;
                Ok(ClosestPoint { distance: d, point: p, parameter: Some(t), uv: None })
            }
            Entity::Surface(_, _) => {
                let (u, v, p, d) = self.surface_closest_uv_sync(shape, point)?;
                Ok(ClosestPoint { distance: d, point: p, parameter: None, uv: Some([u, v]) })
            }
            _ => {
                let solid = self.solid_id(shape)?;
                let (p, d) = closest_point_on_solid(&self.body, solid, pnt(point)).map_err(|error| map_err(&error))?;
                Ok(ClosestPoint { distance: d, point: evec(p), parameter: None, uv: None })
            }
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn classify_point_sync(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        let id = self.solid_id(solid)?;
        point_in_solid(&self.body, id, pnt(point), 1e-6).map_err(|error| map_err(&error))
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
        let solid = sew_faces(&mut self.body, &fids, tolerance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn heal_solid_sync(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let _ = heal_solid(&mut self.body, solid, tolerance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(shape.clone())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn convert_to_nurbs_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut rec = OpRecorder::new();
        let _ = convert_to_nurbs(&mut self.body, solid, &mut rec).map_err(|error| map_err(&error))?;
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
        write_step(&self.body, &solids).map_err(|error| map_step(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_stl_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_stl(&self.body, solid, deflection).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_obj_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_obj(&self.body, solid, deflection).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_gltf_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_glb_sync(shapes, deflection)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_glb_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_glb(&self.body, solid, deflection).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_glb_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_glb_to_body(&mut self.body, data, tolerance).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    /// 🧩️ Merges the imported body into `self.body` (see [`Body::merge`]) rather than replacing
    /// it — handles minted before this call stay resolvable, since nothing already in `self.body`
    /// is renumbered or dropped; only the freshly imported solids get new handles.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_step_sync(&mut self, text: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        let imported = read_step(text).map_err(|error| map_step(&error))?;
        let imported_solid_ids: Vec<_> = imported.solids.ids().collect();
        let map = self.body.merge(&imported);
        Ok(imported_solid_ids.into_iter().map(|id| self.register_solid(map.solids[&id])).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_stl_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_stl_to_body(&mut self.body, data, tolerance).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_obj_sync(&mut self, text: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_obj_to_body(&mut self.body, text, tolerance).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_mesh_sync(&self, shapes: &[GeometryHandle], deflection: f64, exporter: &dyn semio_framework_mesh_engine::MeshExporter) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_mesh(&self.body, solid, deflection, exporter).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_mesh_sync(&mut self, data: &[u8], tolerance: f64, importer: &dyn semio_framework_mesh_engine::MeshImporter) -> Result<GeometryHandle, BrepError> {
        let solid = import_mesh_to_body(&mut self.body, data, tolerance, importer).map_err(|error| map_err(&error))?;
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
    pub fn solid_face_loops_sync(&self, shape: &GeometryHandle) -> Result<SolidFaceLoops, BrepError> {
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
            Entity::Solid(id) => tessellate_solid(&self.body, *id, deflection).map_err(|error| map_err(&error)),
            Entity::Face(id) => tessellate_face(&self.body, *id, deflection).map_err(|error| map_err(&error)),
            Entity::Wire(wire, _) => tessellate_wire(&self.body, wire, deflection).map_err(|error| map_err(&error)),
            other => Err(BrepError::InvalidInput(format!("cannot tessellate {}", entity_tag(other)))),
        }
    }
    /// ⏱️ A resumable, budgetable tessellation of `shape` — the interactive twin of
    /// [`Brep::tessellate_sync`]. The caller drives it with `TessellationJob::step(body, budget)`
    /// (see [`Brep::tessellation_body`]) so no single call outruns an interactive step ceiling,
    /// reads `progress()` between steps, and `cancel()`s it when a newer request supersedes it.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tessellate_job_sync(&self, shape: &GeometryHandle, deflection: f64) -> Result<TessellationJob, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => TessellationJob::for_solid(&self.body, *id, deflection).map_err(|error| map_err(&error)),
            Entity::Face(id) => TessellationJob::for_face(&self.body, *id, deflection).map_err(|error| map_err(&error)),
            Entity::Wire(wire, _) => Ok(TessellationJob::for_wire(wire, deflection)),
            other => Err(BrepError::InvalidInput(format!("cannot tessellate {}", entity_tag(other)))),
        }
    }

    /// 🧬 The topology a [`TessellationJob`] steps against — the job borrows nothing, so a host can
    /// retain it across turns and re-present the body on every step.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tessellation_body(&self) -> &Body {
        &self.body
    }

    /// 🩺️ The structured validation gate every preview tessellation runs first: `Ok(())` when the
    /// shape carries no ERROR-class issue, `Err(issues)` otherwise. Advisory `warning-` codes never
    /// block. Unlike [`Brep::validate_sync`] this returns the typed issues instead of a JSON string,
    /// so a host can route them into a typed diagnostic rather than re-parsing prose.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate_gate_sync(&self, shape: &GeometryHandle) -> Result<(), Vec<ValidationIssue>> {
        let Ok(entity) = self.entity(shape) else {
            return Err(vec![ValidationIssue { entity: shape.as_str().to_string(), code: "unknown-handle", message: format!("handle {} is not live in this kernel", shape.as_str()) }]);
        };
        if !matches!(entity, Entity::Solid(_) | Entity::Shell(_) | Entity::Compound(_, _)) {
            return Ok(());
        }
        let blocking: Vec<ValidationIssue> = validate_body(&self.body).into_iter().filter(|issue| !issue.code.starts_with("warning-")).collect();
        if blocking.is_empty() {
            Ok(())
        } else {
            Err(blocking)
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
        let vertex = self.body.vertices.iter().find(|(_, v)| v.label == label).map(|(id, _)| id);
        if let Some(id) = vertex {
            return Some(self.mint(GeometryKind::Vertex, Entity::Vertex(id)));
        }
        let edge = self.body.edges.iter().find(|(_, e)| e.label == label).map(|(id, _)| id);
        if let Some(id) = edge {
            return Some(self.mint(GeometryKind::Edge, Entity::Edge(id)));
        }
        let face = self.body.faces.iter().find(|(_, f)| f.label == label).map(|(id, _)| id);
        if let Some(id) = face {
            return Some(self.mint(GeometryKind::Face, Entity::Face(id)));
        }
        let shell = self.body.shells.iter().find(|(_, s)| s.label == label).map(|(id, _)| id);
        if let Some(id) = shell {
            return Some(self.mint(GeometryKind::Shell, Entity::Shell(id)));
        }
        let solid = self.body.solids.iter().find(|(_, s)| s.label == label).map(|(id, _)| id);
        if let Some(id) = solid {
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
    fn rotate_about(&mut self, shape: &GeometryHandle, origin: EVec3, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.rotate_about_sync(shape, origin, axis, angle)
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
        self.grid_pattern_sync(shape, dir_x, dir_y, (spacing_x, spacing_y), count_x, count_y)
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
    fn curve_closest_parameter(&self, curve: &GeometryHandle, point: EVec3) -> Result<(f64, EVec3, f64), BrepError> {
        self.curve_closest_parameter_sync(curve, point)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface_closest_uv(&self, surface: &GeometryHandle, point: EVec3) -> Result<(f64, f64, EVec3, f64), BrepError> {
        self.surface_closest_uv_sync(surface, point)
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
    fn export_mesh(&self, shapes: &[GeometryHandle], deflection: f64, exporter: &dyn semio_framework_mesh_engine::MeshExporter) -> Result<Vec<u8>, BrepError> {
        self.export_mesh_sync(shapes, deflection, exporter)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn import_mesh(&mut self, data: &[u8], tolerance: f64, importer: &dyn semio_framework_mesh_engine::MeshImporter) -> Result<GeometryHandle, BrepError> {
        self.import_mesh_sync(data, tolerance, importer)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
