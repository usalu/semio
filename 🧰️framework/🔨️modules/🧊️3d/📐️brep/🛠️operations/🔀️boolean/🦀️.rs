//! 🔀 Exact imprint→classify→select→stitch boolean pipeline for solids bounded by planes,
//! cylinders, cones, spheres and tori (and NURBS via [`semio_framework_3d_brep_intersect`]'s marching SSI): face-pair
//! candidates via AABB overlap → [`intersect_surface_surface`] (W2-A) → the SSI curve's domain
//! clipped to both faces' trims → an imprint edge shared by both operands (so stitching needs no
//! fuzzy vertex welding, only shared-edge adjacency) → [`semio_framework_3d_brep_construction::euler::split_face_by_chain`]/
//! [`semio_framework_3d_brep_construction::euler::split_face_by_interior_curve`] imprint each side → every resulting piece classified
//! against the OTHER solid via [`semio_framework_3d_brep_queries::classification::point_in_solid`] → selected per [`BooleanOp`] → stitched into
//! shell(s)/solid(s) → [`semio_framework_3d_brep_validation::validate_body`]. No tessellate→triangle-soup rebuild on this path — the
//! old mesh pipeline survives only as the explicit opt-in [`boolean_solid_mesh_preview`]. A
//! trivial disjoint/contained fast path and a box-specific exact-analytic fast path (both still
//! genuinely exact, not mesh-derived) run before the general engine when they apply.
//!
//! Documented scope (see `📓️w2b-booleans.md` for the full account, and this ticket's
//! `📓️boolean-kernel-2026-09-09.md` for what the 2026-09-09 pass changed): the general imprint
//! engine handles curves that are fully interior to both faces (closed loop → hole + new face),
//! that graze a periodic seam at one point (seam crossing), or that reach each face's boundary at
//! two points — where "a curve" now means a CHAIN of segments assembled by [`chain_open_pendings`],
//! so a segment that ends on ANOTHER segment rather than on the boundary is legal as long as the
//! chain as a whole reaches it. An imprint that lands exactly along pre-existing topology
//! subdivides and REUSES that boundary edge ([`coincident_boundary_edge`]) instead of minting a
//! duplicate. Coincident/adjacent duplicate faces on the same surface (e.g. two operands sharing a
//! face exactly) are detected and merged into one rather than kept twice.
//!
//! ♻️ Input lifetime: a boolean OWNS everything it answers with and frees nothing it did not
//! create. The general engine plans on deep copies of both operands ([`BooleanJob::new`]), every
//! fast-path branch that "returns an operand" returns a [`copy_solid`] of it, and the post-stitch
//! orphan sweep is scoped to the entities the job itself minted. The operands are read-only
//! throughout, because the evaluator above this kernel serves an unchanged input node from cache
//! and re-presents the SAME handle on the next evaluation — a boolean that consumed its inputs made
//! `sphere-box-fuse`/`sphere-cut-with-torus` answer `missing handle: <digest>` from the second
//! evaluation onwards (`📓️brep-boolean-input-lifetime-2026-09-15.md`).
//!
//! Still out of scope, surfacing as a `BooleanError` rather than a silently wrong result: a set of
//! open segments that closes into a cycle with no free end, and a clip whose endpoints fall on a
//! pole or exactly along a trim boundary — `refine_boundary` locates those only to within the
//! caller's tolerance, which is why the two procedural-3d boolean examples still fail (see the
//! ticket report's §4.3).
//!
//! Lane 4-boolean of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`, rewritten from
//! the tessellate-and-classify pipeline in `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 2
//! (W2-B) — see `📓️w2b-booleans.md`.

use std::collections::{HashMap, HashSet};

use semio_framework_3d_brep_construction::euler::{add_shell, add_solid, make_edge, make_vertex, splice_boundary_vertex};
use semio_framework_3d_brep_intersect::{intersect_curve_surface, intersect_surface_surface, IntCurve};
use semio_framework_3d_brep_construction::primitives::{make_box, make_convex_hull, solid_from_triangle_soup};
use semio_framework_3d_brep_construction::staged::{drive, drive_solid, StageOutput, StageProgress, StageStep, StagedOperation};
use semio_framework_3d_brep_construction::transform::{copy_solid, transform_solid};
use semio_framework_3d_brep_vector::matrix::Affine3;
use semio_framework_3d_brep_representation::contract::{MeshTransfer, PointClassification};
use semio_framework_3d_brep_queries::bounding_volume::face_aabb;
use semio_framework_3d_brep_queries::classification::{point_in_face_uv, point_in_face_uv_closure};
use semio_framework_3d_brep_queries::mass_properties::{closest_point_on_solid, shell_signed_volume, solid_bounding_box, solid_volume, AxisAlignedBox};
use semio_framework_3d_brep_tessellation::tessellate_solid;
use semio_framework_3d_brep_validation::BodyValidationJob;
#[cfg(test)]
use semio_framework_3d_brep_validation::validate_body;
use semio_framework_3d_brep_representation::arena::{ArenaId, CoedgeId, Curve2Id, EdgeId, FaceId, LoopId, ShellId, SolidId, VertexId};
use semio_framework_3d_brep_curve::curve_ops::closest_parameter;
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_representation::error::{BooleanError, KernelError, ValidationIssue};
use semio_framework_3d_brep_surface::surface_ops::closest_uv;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, Vec3};

// #region 🔖️Api

/// 🔀 Boolean combination kind for [`boolean_solid`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BooleanOp {
    Unite,
    Cut,
    Intersect,
}

/// 🔀 Combines solids `a` and `b` under `op`: a trivial disjoint/contained check, then a
/// box-specific exact-analytic shortcut when both operands genuinely are axis boxes, then the
/// general exact imprint→classify→select→stitch engine — never the mesh path (see
/// [`boolean_solid_mesh_preview`] for that, behind explicit opt-in). `rec` accumulates the whole
/// operation's [`semio_framework_3d_brep_representation::topology::history::OpDelta`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boolean_solid(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    match boolean_job(body, a, b, op, tol, rec)? {
        BooleanAdmission::Answered(id) => Ok(id),
        BooleanAdmission::Job(job) => job.run_to_completion(body, rec),
    }
}

/// 🔀 Successively cuts `tools` from `target` (folded [`BooleanOp::Cut`]). Every intermediate is a
/// solid this fold minted and nobody else can name, so it is dropped as soon as the next round
/// supersedes it — `target` and the tools, which the caller DOES name, are never touched (see
/// [`BooleanJob::new`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compound_cut(body: &mut Body, target: SolidId, tools: &[SolidId], tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    drive_solid(&mut BooleanFoldJob::compound_cut(body, target, tools, tol)?, body, rec)
}

/// 🔀 Planar section of `solid` by the plane `(origin, normal)`.
///
/// Collects in-plane vertices and edge/plane hits, then builds one planar face from those points.
/// Exact for polygonal/planar-boundary solids (kept from the prior pass); a genuinely curved-face
/// section (a plane cutting a cylinder/sphere/cone/torus) is not yet routed through the new
/// imprint engine — documented gap, see `📓️w2b-booleans.md`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn section_solid_by_plane(body: &mut Body, solid: SolidId, origin: Pnt3, normal: Vec3, tol: f64, rec: &mut OpRecorder) -> Result<Vec<FaceId>, KernelError> {
    match drive(&mut SectionJob::new(body, solid, origin, normal, tol)?, body, rec)? {
        StageOutput::Faces(faces) => Ok(faces),
        other => Err(KernelError::Operation(format!("section did not yield faces: {other:?}"))),
    }
}

/// 🔀 Splits `solid` by the plane `(origin, normal)` into two solids (classified triangle soups;
/// hull fallback). Exact for polygonal/planar-boundary solids (kept from the prior pass); a
/// genuinely curved-face split is not yet routed through the new imprint engine — documented gap,
/// see `📓️w2b-booleans.md`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_solid_by_plane(body: &mut Body, solid: SolidId, origin: Pnt3, normal: Vec3, tol: f64, rec: &mut OpRecorder) -> Result<(SolidId, SolidId), KernelError> {
    match drive(&mut SplitJob::new(body, solid, origin, normal, tol)?, body, rec)? {
        StageOutput::Solids(solids) if solids.len() == 2 => Ok((solids[0], solids[1])),
        other => Err(KernelError::Operation(format!("split did not yield two solids: {other:?}"))),
    }
}

// #endregion 🔖️Api

#[path = "⚡️fast-paths/🦀️.rs"]
mod fast_paths;
use fast_paths::{box_fast_path, solid_wholly_inside, trivial_topology_fast_path};

#[path = "🖇️imprint-face-pairs/🦀️.rs"]
mod imprint_face_pairs;
use imprint_face_pairs::{Pending, imprint_face_pair};

#[path = "⏳️resumable-boolean-job/🦀️.rs"]
mod resumable_boolean_job;
pub use resumable_boolean_job::{BooleanAdmission, BooleanJob, BooleanPhase, BooleanProgress, BooleanStep, boolean_job};

#[path = "✂️scoping-and-clip-curves/🦀️.rs"]
mod scoping_and_clip_curves;
use scoping_and_clip_curves::{clip_intcurve_to_faces, curve3_extent, intcurve_finite_bracket, issues_scoped_to_new_solids, nearest_boundary_curve, nearest_transversal_boundary_curve, outer_boundary_curves, point_in_face_uv_periodic};

#[path = "📌️clip-endpoint-snapping/🦀️.rs"]
mod clip_endpoint_snapping;
use clip_endpoint_snapping::{boundary_touch_parameters, pcurve_for_clip, snap_clip_endpoint};

#[path = "🔗️imprint-vertices-and-edges/🦀️.rs"]
mod imprint_vertices_and_edges;
use imprint_vertices_and_edges::{boundary_subedge, build_imprint_edge, coincident_boundary_edge, oriented_prange, unify_boundary_edges, unify_closed_boundary_vertices};

#[path = "🏷️classification-and-coincidence/🦀️.rs"]
mod classification_and_coincidence;
use classification_and_coincidence::{classify_face_against_solid, find_coincident_face_pairs, flip_face, keep_coincident_boundary, keep_face, local_point_in_solid, sample_loop_uv, unwrap_to};

#[path = "🧵️stitch-and-cleanup/🦀️.rs"]
mod stitch_and_cleanup;
use stitch_and_cleanup::{gc_orphan_edges_and_vertices, remove_solid_and_orphans, stitch_selected_faces};

#[path = "👁️mesh-preview-and-shell-helpers/🦀️.rs"]
mod mesh_preview_and_shell_helpers;
use mesh_preview_and_shell_helpers::{detached_copy_of_outer_faces, mesh_position, outer_faces, solid_from_outer_faces, solid_probe_points, solid_vertex_positions};
pub use mesh_preview_and_shell_helpers::boolean_solid_mesh_preview;

#[path = "📏️aabb-math-and-stage/🦀️.rs"]
mod aabb_math_and_stage;
use aabb_math_and_stage::{aabb_contains, aabb_dims, aabb_finite, aabb_gap, aabb_intersection, aabb_overlap, aabb_union, aabb_volume, gap_1d};
pub use aabb_math_and_stage::BooleanStage;

// #region 🔖️Jobs

#[path = "⏱️jobs/🦀️.rs"]
mod jobs;
#[path = "🧭️arrangement/🦀️.rs"]
mod arrangement;
pub use jobs::{BooleanFoldJob, SectionJob, SplitJob};

// #endregion 🔖️Jobs

// #region 🔖️Validate

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_tol(tol: f64) -> Result<(), KernelError> {
    if tol.is_finite() && tol > 0.0 {
        Ok(())
    } else {
        Err(KernelError::InvalidInput("tolerance must be positive and finite".into()))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn require_solid(body: &Body, solid: SolidId) -> Result<(), KernelError> {
    if body.solids.get(solid).is_some() {
        Ok(())
    } else {
        Err(KernelError::MissingEntity(format!("solid {solid}")))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn plane_normal(normal: Vec3) -> Result<Vec3, KernelError> {
    normal.normalized().ok_or_else(|| KernelError::InvalidInput("plane normal must be non-zero".into()))
}

// #endregion 🔖️Validate

#[cfg(test)]
pub(crate) use semio_framework_3d_testing_support as test_support;

#[cfg(test)]
#[path = "🧪️tests/🔬️differential/🦀️.rs"]
mod differential;

#[cfg(any(test, feature = "testkit"))]
pub mod testkit {
    use super::*;
    pub fn local_point_in_solid(body: &Body, solid: SolidId, point: Pnt3, tol: f64) -> Result<PointClassification, KernelError> {
        super::local_point_in_solid(body, solid, point, tol)
    }
    pub fn keep_coincident_boundary(op: BooleanOp, aligned: bool) -> bool {
        super::keep_coincident_boundary(op, aligned)
    }
    pub fn find_coincident_face_pairs(body: &Body, faces_a: &[FaceId], faces_b: &[FaceId], tol: f64) -> Vec<(FaceId, FaceId, bool)> {
        super::find_coincident_face_pairs(body, faces_a, faces_b, tol)
    }
}
