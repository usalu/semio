//! 🤝 Contract of the 3D kernel's B-Rep interface: the geometry handle and error vocabulary and the model-free synchronous
//! [`BrepKernel`] trait that the arena-backed engine implements.
//!
//! Crate cut of `semio-framework-3d-brep-engine` (carve FB, `🪓️monolith-split/RESULT-CRATE-CARVE-FB.md`).

extern crate semio_framework_value_derive as value_derive;
/// 🧱 Vertex positions with outer and inner vertex loops for each face.
pub type SolidFaceLoops = (Vec<[f32; 3]>, Vec<(Vec<u32>, Vec<Vec<u32>>)>);

use semio_framework_3d_brep_representation::contract::*;
use semio_framework_3d_brep_representation::error::KernelError;

// #region 🔖️ContractTypes

/// 🧭️ Geometry entity kind carried by a handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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

/// 🧭️ Opaque kind-and-label geometry identity within the owning kernel.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
#[value(transparent)]
pub struct GeometryHandle(pub String);

impl semio_framework_value::retirement::RetireOwned for GeometryHandle {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.0)}
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)}
    fn controlled_retirement_supported()->bool{true}
}

impl GeometryHandle {
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
#[value(crate = "::semio_framework_value")]
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
    fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError>;
    fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError>;
    fn convex_hull(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError>;
    // #endregion Primitives

    // #region Curves
    fn line_curve(&mut self, start: Vec3, end: Vec3) -> Result<GeometryHandle, BrepError>;
    fn circle_curve(&mut self, center: Vec3, normal: Vec3, radius: f64) -> Result<GeometryHandle, BrepError>;
    fn arc_curve(&mut self, center: Vec3, normal: Vec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError>;
    fn ellipse_curve(&mut self, center: Vec3, normal: Vec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError>;
    fn polyline_wire(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError>;
    fn rectangle_wire(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError>;
    fn regular_polygon_wire(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError>;
    fn interpolate_curve(&mut self, points: &[Vec3], degree: usize) -> Result<GeometryHandle, BrepError>;
    fn approximate_curve(&mut self, points: &[Vec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError>;
    fn helix_curve(&mut self, origin: Vec3, axis: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError>;
    // #endregion Curves

    // #region Surfaces
    fn plane_surface(&mut self, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError>;
    fn planar_face_from_points(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError>;
    fn planar_face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn nurbs_surface_from_grid(&mut self, points: &[Vec<Vec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError>;
    fn coons_patch(&mut self, curves: &[Vec<Vec3>]) -> Result<GeometryHandle, BrepError>;
    fn offset_face(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError>;
    fn thicken_face(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError>;
    // #endregion Surfaces

    // #region Sweeps
    fn extrude_wire(&mut self, wire: &GeometryHandle, vector: Vec3) -> Result<GeometryHandle, BrepError>;
    fn extrude(&mut self, face: &GeometryHandle, direction: Vec3, distance: f64) -> Result<GeometryHandle, BrepError>;
    fn revolve(&mut self, face: &GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    fn loft(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError>;
    fn sweep(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn pipe(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError>;
    fn helical_sweep(&mut self, profile: &GeometryHandle, axis_origin: Vec3, axis_dir: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError>;
    // #endregion Sweeps

    // #region Booleans
    fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn compound_cut(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    // #endregion Booleans

    // #region Transforms
    fn translate(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError>;
    fn rotate(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    /// 🔁 Rotates about an explicit `origin` — [`Self::rotate`] has no origin parameter and always
    /// rotates about the world origin (`(0,0,0)`), never a shape's bounding-box center.
    fn rotate_about(&mut self, shape: &GeometryHandle, origin: Vec3, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError>;
    /// 📏️ Scales geometry independently along the world axes around an explicit center.
    fn scale_axes(&mut self, shape: &GeometryHandle, factors: Vec3, center: Vec3) -> Result<GeometryHandle, BrepError>;
    fn mirror(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError>;
    fn copy_shape(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn linear_pattern(&mut self, shape: &GeometryHandle, direction: Vec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError>;
    fn circular_pattern(&mut self, shape: &GeometryHandle, axis: Vec3, count: usize) -> Result<GeometryHandle, BrepError>;
    #[allow(clippy::too_many_arguments, reason = "grid pattern needs an independent spacing/count pair per axis; grouping into a params struct would ripple through every out-of-scope BrepKernel implementor and caller")]
    fn grid_pattern(&mut self, shape: &GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError>;
    // #endregion Transforms

    // #region Features
    fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError>;
    fn fillet_variable(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError>;
    /// 🎯️ Fillets only the given edges instead of every edge of the solid — avoids paying the
    /// cost of a full-solid fillet when only a handful of edges are actually selected.
    fn fillet_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError>;
    fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError>;
    fn chamfer_asymmetric(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError>;
    /// 🎯️ Chamfers only the given edges instead of every edge of the solid.
    fn chamfer_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError>;
    fn shell(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    fn draft(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: Vec3, neutral_point: Vec3, angle: f64) -> Result<GeometryHandle, BrepError>;
    fn offset_solid(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError>;
    fn defeature(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    // #endregion Features

    // #region Intersect
    fn section(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<Vec<GeometryHandle>, BrepError>;
    fn split(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<(GeometryHandle, GeometryHandle), BrepError>;
    fn curve_curve_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError>;
    fn curve_surface_intersect(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError>;
    fn surface_surface_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError>;
    // #endregion Intersect

    // #region Evaluate
    fn curve_point(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError>;
    fn curve_tangent(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError>;
    fn curve_domain(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError>;
    fn curve_curvature(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError>;
    fn surface_point(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError>;
    fn surface_normal(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError>;
    /// 🧭️ Certified closest parameter on `curve` to `point`: `(t, point, distance)`. Analytic
    /// closed forms for line/circle/ellipse, Bézier-span subdivision + Newton for NURBS.
    fn curve_closest_parameter(&self, curve: &GeometryHandle, point: Vec3) -> Result<(f64, Vec3, f64), BrepError>;
    /// 🧭️ Certified closest `(u, v)` on `surface` to `point`: `(u, v, point, distance)`. Analytic
    /// closed forms for every analytic surface kind (poles/apex/both torus periods handled),
    /// Bézier-patch subdivision + damped 2D Newton for NURBS.
    fn surface_closest_uv(&self, surface: &GeometryHandle, point: Vec3) -> Result<(f64, f64, Vec3, f64), BrepError>;
    // #endregion Evaluate

    // #region Measure
    fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError>;
    fn area(&self, shape: &GeometryHandle) -> Result<f64, BrepError>;
    fn length(&self, shape: &GeometryHandle) -> Result<f64, BrepError>;
    fn center_of_mass(&self, shape: &GeometryHandle) -> Result<Vec3, BrepError>;
    fn bounding_box(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn distance(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError>;
    fn closest_point(&self, shape: &GeometryHandle, point: Vec3) -> Result<ClosestPoint, BrepError>;
    fn classify_point(&self, solid: &GeometryHandle, point: Vec3) -> Result<PointClassification, BrepError>;
    fn validate(&self, shape: &GeometryHandle) -> Result<String, BrepError>;
    // #endregion Measure

    // #region Construct
    fn vertex(&mut self, point: Vec3) -> Result<GeometryHandle, BrepError>;
    fn face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn sew_faces(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError>;
    fn heal_solid(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError>;
    fn convert_to_nurbs(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError>;
    fn deconstruct(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError>;
    // #endregion Construct

    // #region IO

    fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
    fn export_obj(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError>;
    fn export_gltf(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;

    fn import_stl(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError>;
    fn import_obj(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError>;
    fn export_mesh(&self, shapes: &[GeometryHandle], deflection: f64, exporter: &dyn semio_framework_mesh_engine::io::MeshExporter) -> Result<Vec<u8>, BrepError>;
    fn import_mesh(&mut self, data: &[u8], tolerance: f64, importer: &dyn semio_framework_mesh_engine::io::MeshImporter) -> Result<GeometryHandle, BrepError>;
    // #endregion IO

    // #region Core
    fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError>;
    fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError>;
    fn dispose(&mut self, handle: &GeometryHandle);
    /// 🧹️ Drops every registry entry whose handle isn't in `live`.
    fn retain(&mut self, live: &std::collections::HashSet<String>);
    /// 📊️ Number of geometry handles currently held by the kernel's registry.
    fn registry_len(&self) -> usize;
    /// 🧩️ Every shell of `solid` as its own first-class handle (see `GeometryKind::Shell`).
    fn solid_shells(&mut self, solid: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError>;
    /// 🧩️ Bundles `solids` behind one `GeometryKind::Compound` handle — the collection identity
    /// patterns and booleans return when the operation naturally yields more than one
    /// solid, mirroring [`Self::explode`]'s inverse.
    fn compound(&mut self, solids: &[GeometryHandle]) -> Result<GeometryHandle, BrepError>;
    /// 🧩️ The inverse of [`Self::compound`]: the member solids' own handles.
    fn explode(&mut self, compound: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError>;
    /// 🏷️ The document-scoped [`PersistentLabel`] a handle currently resolves to — stable across
    /// a `dispose`d-and-regranted handle for the same entity, unlike the handle string itself.
    fn label(&self, handle: &GeometryHandle) -> Option<u64>;
    // #endregion Core
}
// #endregion 🔖️Kernel

// #region 🧮Convert

/// 🧭️ Reports a kernel error the way every one-shot operation of the engine always has: as [`BrepError::Operation`] carrying its message.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn map_err(e: &KernelError) -> BrepError {
    BrepError::Operation(e.to_string())
}

// #endregion 🧮Convert

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
