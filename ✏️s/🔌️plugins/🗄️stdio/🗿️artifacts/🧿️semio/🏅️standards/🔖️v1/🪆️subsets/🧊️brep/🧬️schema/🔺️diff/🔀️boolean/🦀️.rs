//! 🔀 Exact imprint→classify→select→stitch boolean pipeline for solids bounded by planes,
//! cylinders, cones, spheres and tori (and NURBS via [`crate::standards::v1::subsets::brep::schema::diff::intersect`]'s marching SSI): face-pair
//! candidates via AABB overlap → [`intersect_surface_surface`] (W2-A) → the SSI curve's domain
//! clipped to both faces' trims → an imprint edge shared by both operands (so stitching needs no
//! fuzzy vertex welding, only shared-edge adjacency) → [`crate::standards::v1::subsets::brep::schema::diff::euler::split_face_by_chain`]/
//! [`crate::standards::v1::subsets::brep::schema::diff::euler::split_face_by_interior_curve`] imprint each side → every resulting piece classified
//! against the OTHER solid via [`crate::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid`] → selected per [`BooleanOp`] → stitched into
//! shell(s)/solid(s) → [`crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body`]. No tessellate→triangle-soup rebuild on this path — the
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

use crate::standards::v1::subsets::brep::schema::diff::euler::{add_shell, add_solid, make_edge, make_vertex, splice_boundary_vertex, split_face_by_chain, split_face_by_interior_curve, split_face_by_seam_crossing, ParametricEdge};
use crate::standards::v1::subsets::brep::schema::diff::intersect::{intersect_curve_surface, intersect_surface_surface, IntCurve};
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_convex_hull, solid_from_triangle_soup};
use crate::standards::v1::subsets::brep::schema::diff::transform::transform_solid;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;
use crate::standards::v1::subsets::brep::schema::engine::{MeshTransfer, PointClassification};
use crate::standards::v1::subsets::brep::schema::inferences::bounding_volume::face_aabb;
use crate::standards::v1::subsets::brep::schema::inferences::classification::{point_in_face_uv, point_in_face_uv_closure};
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::{closest_point_on_solid, shell_signed_volume, solid_bounding_box, solid_volume, AxisAlignedBox};
use crate::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use crate::standards::v1::subsets::brep::schema::inferences::validation_report::BodyValidationJob;
#[cfg(test)]
use crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve2Id, EdgeId, FaceId, LoopId, ShellId, SolidId, VertexId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::closest_parameter;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2;
use crate::standards::v1::subsets::brep::schema::snapshot::error::{BooleanError, KernelError, ValidationIssue};
use crate::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};

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
/// operation's [`crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpDelta`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boolean_solid(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    match boolean_job(body, a, b, op, tol, rec)? {
        BooleanAdmission::Answered(id) => Ok(id),
        BooleanAdmission::Job(job) => job.run_to_completion(body, rec),
    }
}

/// 🔀 Successively cuts `tools` from `target` (folded [`BooleanOp::Cut`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compound_cut(body: &mut Body, target: SolidId, tools: &[SolidId], tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_tol(tol)?;
    require_solid(body, target)?;
    if tools.is_empty() {
        return Err(KernelError::InvalidInput("compound_cut requires at least one tool solid".into()));
    }
    let mut current = target;
    for &tool in tools {
        current = boolean_solid(body, current, tool, BooleanOp::Cut, tol, rec)?;
    }
    Ok(current)
}

/// 🔀 Planar section of `solid` by the plane `(origin, normal)`.
///
/// Collects in-plane vertices and edge/plane hits, then builds one planar face from those points.
/// Exact for polygonal/planar-boundary solids (kept from the prior pass); a genuinely curved-face
/// section (a plane cutting a cylinder/sphere/cone/torus) is not yet routed through the new
/// imprint engine — documented gap, see `📓️w2b-booleans.md`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn section_solid_by_plane(body: &mut Body, solid: SolidId, origin: Pnt3, normal: Vec3, tol: f64, rec: &mut OpRecorder) -> Result<Vec<FaceId>, KernelError> {
    require_tol(tol)?;
    require_solid(body, solid)?;
    let n = plane_normal(normal)?;
    let points = solid_vertex_positions(body, solid);
    let mut section_pts = Vec::new();
    for p in &points {
        if ((*p - origin).dot(n)).abs() <= tol * 10.0 {
            section_pts.push(*p);
        }
    }
    // Also sample edge intersections with the plane.
    let mut edge_ids = HashSet::new();
    for face in body.solid_faces(solid) {
        for loop_id in body.face_loops(face) {
            for cid in body.loop_coedges(loop_id) {
                if let Some(co) = body.coedges.get(cid) {
                    edge_ids.insert(co.edge);
                }
            }
        }
    }
    for edge_id in edge_ids {
        let Some(edge) = body.edges.get(edge_id) else { continue };
        let Some(v0) = body.vertices.get(edge.v0).map(|v| v.position) else { continue };
        let Some(v1) = body.vertices.get(edge.v1).map(|v| v.position) else { continue };
        let d0 = (v0 - origin).dot(n);
        let d1 = (v1 - origin).dot(n);
        if d0 * d1 > 0.0 {
            continue;
        }
        let denom = d0 - d1;
        if denom.abs() <= 1e-15 {
            continue;
        }
        let t = d0 / denom;
        section_pts.push(v0 + (v1 - v0) * t);
    }
    if section_pts.len() < 3 {
        return Ok(Vec::new());
    }
    // Build a planar face from the convex hull of section points in-plane.
    let face = crate::standards::v1::subsets::brep::schema::diff::primitives::make_planar_face_from_points(body, &section_pts, rec)?;
    Ok(vec![face])
}

/// 🔀 Splits `solid` by the plane `(origin, normal)` into two solids (classified triangle soups;
/// hull fallback). Exact for polygonal/planar-boundary solids (kept from the prior pass); a
/// genuinely curved-face split is not yet routed through the new imprint engine — documented gap,
/// see `📓️w2b-booleans.md`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_solid_by_plane(body: &mut Body, solid: SolidId, origin: Pnt3, normal: Vec3, tol: f64, rec: &mut OpRecorder) -> Result<(SolidId, SolidId), KernelError> {
    require_tol(tol)?;
    require_solid(body, solid)?;
    let n = plane_normal(normal)?;
    let mesh = tessellate_solid(body, solid, tol.max(1e-3))?;
    let mut pos_tris: Vec<[Pnt3; 3]> = Vec::new();
    let mut neg_tris: Vec<[Pnt3; 3]> = Vec::new();
    let mut pos_pts = Vec::new();
    let mut neg_pts = Vec::new();
    let npos = mesh.position.len() / 3;
    if mesh.index.len() % 3 != 0 {
        return Err(KernelError::InvalidInput("mesh index length must be a multiple of 3".into()));
    }
    for tri in mesh.index.as_chunks::<3>().0 {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= npos || i1 >= npos || i2 >= npos {
            return Err(KernelError::InvalidInput("mesh index out of range".into()));
        }
        let p0 = mesh_position(&mesh, i0);
        let p1 = mesh_position(&mesh, i1);
        let p2 = mesh_position(&mesh, i2);
        let c = Pnt3::new((p0.x + p1.x + p2.x) / 3.0, (p0.y + p1.y + p2.y) / 3.0, (p0.z + p1.z + p2.z) / 3.0);
        let d = (c - origin).dot(n);
        if d >= -tol {
            pos_tris.push([p0, p1, p2]);
            pos_pts.extend([p0, p1, p2]);
        }
        if d <= tol {
            neg_tris.push([p0, p1, p2]);
            neg_pts.extend([p0, p1, p2]);
        }
    }
    if pos_tris.is_empty() || neg_tris.is_empty() {
        // Fall back to vertex-side hulls when tessellation did not straddle the plane.
        let points = solid_vertex_positions(body, solid);
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        for p in points {
            let d = (p - origin).dot(n);
            if d >= -tol {
                pos.push(p);
            }
            if d <= tol {
                neg.push(p);
            }
        }
        if pos.len() < 4 || neg.len() < 4 {
            return Err(KernelError::Boolean(BooleanError::InvalidResult("split_solid_by_plane: one side has too few points".into())));
        }
        return Ok((make_convex_hull(body, &pos, rec)?, make_convex_hull(body, &neg, rec)?));
    }
    let solid_pos = match solid_from_triangle_soup(body, &pos_tris, rec) {
        Ok(id) => id,
        Err(_) => make_convex_hull(body, &pos_pts, rec)?,
    };
    let solid_neg = match solid_from_triangle_soup(body, &neg_tris, rec) {
        Ok(id) => id,
        Err(_) => make_convex_hull(body, &neg_pts, rec)?,
    };
    Ok((solid_pos, solid_neg))
}

// #endregion 🔖️Api

// #region 🔖️TrivialFastPath

/// 🔀 Checks that don't depend on box-ness at all: disjoint operands (gap ≥ tol) and full
/// containment (one operand's boundary lies entirely inside/outside the other), verified with a
/// real [`point_in_solid`] probe of several boundary points, not just AABB containment (AABB
/// containment alone is necessary but not sufficient — used only to decide whether the probe is
/// worth running).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn trivial_topology_fast_path(body: &mut Body, a: SolidId, b: SolidId, (bb_a, bb_b): (&AxisAlignedBox, &AxisAlignedBox), op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
    let gap = aabb_gap(bb_a, bb_b);
    if gap >= tol {
        return match op {
            BooleanOp::Unite => {
                let mut faces = outer_faces(body, a)?;
                faces.extend(outer_faces(body, b)?);
                Ok(Some(solid_from_outer_faces(body, faces, Vec::new(), rec)?))
            }
            BooleanOp::Cut => Ok(Some(clone_solid_shells(body, a, rec)?)),
            BooleanOp::Intersect => Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty (operands disjoint)".into()))),
        };
    }
    if aabb_contains(bb_b, bb_a, tol) && solid_wholly_inside(body, a, b, tol)? {
        return match op {
            BooleanOp::Unite => Ok(Some(clone_solid_shells(body, b, rec)?)),
            BooleanOp::Intersect => Ok(Some(clone_solid_shells(body, a, rec)?)),
            BooleanOp::Cut => Err(KernelError::Boolean(BooleanError::InvalidResult("boolean cut is empty (tool contains target)".into()))),
        };
    }
    if aabb_contains(bb_a, bb_b, tol) && solid_wholly_inside(body, b, a, tol)? {
        return match op {
            BooleanOp::Unite => Ok(Some(clone_solid_shells(body, a, rec)?)),
            BooleanOp::Intersect => Ok(Some(clone_solid_shells(body, b, rec)?)),
            BooleanOp::Cut => {
                let outer = outer_faces(body, a)?;
                let inner = outer_faces(body, b)?;
                Ok(Some(solid_from_outer_faces(body, outer, vec![inner], rec)?))
            }
        };
    }
    Ok(None)
}

/// 🔀 `true` when every boundary vertex of `inner` classifies as `Inside` or `OnBoundary` against
/// `outer` — a real (if sampling-based) containment proof, not an AABB heuristic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_wholly_inside(body: &Body, inner: SolidId, outer: SolidId, tol: f64) -> Result<bool, KernelError> {
    let points = solid_vertex_positions(body, inner);
    if points.is_empty() {
        return Ok(false);
    }
    for p in points {
        if !matches!(local_point_in_solid(body, outer, p, tol)?, PointClassification::Inside | PointClassification::OnBoundary) {
            return Ok(false);
        }
    }
    Ok(true)
}

// #endregion 🔖️TrivialFastPath

// #region 🔖️BoxFastPath

/// 🔀 When BOTH operands are genuinely axis boxes (6 faces, volume matching their AABB volume),
/// `Intersect` is always another axis box (the AABB intersection), and `Unite` is one too — but
/// ONLY when the two boxes agree on two of the three axis intervals and merely extend each other
/// along the third ([`boxes_union_is_a_box`]); a general partial overlap unions into an L/cross
/// shape, which this shortcut must hand back to the general imprint engine rather than silently
/// returning the (strictly larger) AABB union. `Cut` is likewise left to the general engine.
/// 🐛 The result is built at the union/intersection's own min corner: [`make_box`] always builds
/// at the origin, so the shortcut used to return a correctly-SIZED box in the wrong PLACE
/// whenever either operand was not itself origin-anchored — invisible to a volume-only assertion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn box_fast_path(body: &mut Body, a: SolidId, b: SolidId, (bb_a, bb_b): (&AxisAlignedBox, &AxisAlignedBox), op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
    if !matches!(op, BooleanOp::Unite | BooleanOp::Intersect) {
        return Ok(None);
    }
    if !(is_aabb_box_solid(body, a, bb_a)? && is_aabb_box_solid(body, b, bb_b)?) {
        return Ok(None);
    }
    let target = match op {
        BooleanOp::Unite => {
            if !boxes_union_is_a_box(bb_a, bb_b, tol) {
                return Ok(None);
            }
            aabb_union(bb_a, bb_b)
        }
        BooleanOp::Intersect => {
            let Some(inter) = aabb_intersection(bb_a, bb_b) else {
                return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty".into())));
            };
            let (w, d, h) = aabb_dims(&inter);
            if w <= tol || d <= tol || h <= tol {
                return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty within tolerance".into())));
            }
            inter
        }
        BooleanOp::Cut => unreachable!(),
    };
    let (w, d, h) = aabb_dims(&target);
    let built = make_box(body, w, d, h, rec)?;
    let origin = Vec3::new(target.min.x, target.min.y, target.min.z);
    if origin.norm_sq() <= 0.0 {
        return Ok(Some(built));
    }
    Ok(Some(transform_solid(body, built, &Affine3::translation(origin), rec)?))
}

/// 🔀 `true` when two axis boxes' union is itself an axis box: they must coincide on two of the
/// three axis intervals and overlap or abut along the remaining one. Any other configuration
/// unions into a non-box (L, cross, or two disjoint lumps).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn boxes_union_is_a_box(a: &AxisAlignedBox, b: &AxisAlignedBox, tol: f64) -> bool {
    let spans = [(a.min.x, a.max.x, b.min.x, b.max.x), (a.min.y, a.max.y, b.min.y, b.max.y), (a.min.z, a.max.z, b.min.z, b.max.z)];
    let mut extended = 0usize;
    for (a0, a1, b0, b1) in spans {
        if (a0 - b0).abs() <= tol && (a1 - b1).abs() <= tol {
            continue;
        }
        if gap_1d(a0, a1, b0, b1) > tol {
            return false;
        }
        extended += 1;
    }
    extended <= 1
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_aabb_box_solid(body: &Body, solid: SolidId, bb: &AxisAlignedBox) -> Result<bool, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.len() != 6 {
        return Ok(false);
    }
    let bv = aabb_volume(bb);
    if !(bv.is_finite() && bv > 0.0) {
        return Ok(false);
    }
    let v = solid_volume(body, solid, 1e-6)?;
    Ok((v - bv).abs() <= 1e-6)
}

// #endregion 🔖️BoxFastPath

// #region 🔖️ExactImprintEngine

/// 🔀 One clipped intersection-curve segment queued for imprint on one original face — `edge_id`
/// is shared verbatim between the two originating faces' pending lists (built once, spliced
/// twice), so the two resulting halves are guaranteed to share topology, not just geometry.
struct Pending {
    edge_id: EdgeId,
    pcurve_id: Curve2Id,
    prange: (f64, f64),
    kind: ImprintKind,
}

/// 🔀 Which Euler splice a queued [`Pending`] imprint needs: `Interior` (closed curve bounding a
/// small sub-region — hole + new face), `SeamCrossing` (closed curve spanning a periodic surface's
/// FULL width, so it grazes a doubly-used seam edge at one physical point rather than bounding a
/// sub-region), or `Open` (crosses the boundary at two distinct points — two-chain split).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ImprintKind {
    Interior,
    SeamCrossing,
    Open,
}

/// 🔀 One (face-of-A, face-of-B) imprint unit: intersect the two supports, clip every resulting
/// curve to both trims, and queue the imprint each side owes. The atomic unit of
/// [`BooleanPhase::Imprint`] — extracted verbatim out of the former `exact_imprint_boolean`'s inner
/// loop so the budgeted walk and the unbudgeted one run the SAME code (ticket
/// `26/09/09/PROCEDURAL-3D-END-TO-END`). A pair with nothing to imprint returns `Ok(())`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[allow(clippy::too_many_arguments)]
fn imprint_face_pair(
    body: &mut Body,
    (fa, fb): (FaceId, FaceId),
    (sa, bb_a): (&Surface, &crate::standards::v1::subsets::brep::schema::engine::Aabb),
    tol: f64,
    weld: &mut Vec<(Pnt3, VertexId)>,
    (pending_a, pending_b): (&mut HashMap<FaceId, Vec<Pending>>, &mut HashMap<FaceId, Vec<Pending>>),
    rec: &mut OpRecorder,
) -> Result<(), KernelError> {
    let Ok(bb_b) = face_aabb(body, fb) else { return Ok(()) };
    if !aabb_overlap(bb_a, &bb_b, tol) {
        return Ok(());
    }
    let Some(face_b) = body.faces.get(fb).cloned() else { return Ok(()) };
    let Some(sb) = body.surfaces.get(face_b.surface).cloned() else { return Ok(()) };
    let Ok(curves) = intersect_surface_surface(sa, &sb, tol) else { return Ok(()) };
    for ic in &curves {
        // A near-tangent pair (e.g. two spheres offset by barely more than the sum of
        // their radii) can still produce a genuine but near-zero-radius contact circle
        // from `intersect_surface_surface`'s own tolerant overlap test — a real curve
        // object, not a numerical error, but one whose enclosed area/length is below the
        // tolerance at which its precise topology (which side of the seam it grazes, its
        // own trim gap) is even meaningful. Physically this is single-point external
        // tangency: zero intersection area, nothing to imprint. Skipping it here (rather
        // than imprinting a degenerate sliver) is what keeps the tangent-sphere union's
        // volume the exact, un-carved sum of both spheres.
        let bracket = intcurve_finite_bracket(body, ic, fa, fb);
        if curve3_extent(&ic.curve3, bracket) <= tol {
            continue;
        }
        for (t0, t1, full_period, touches) in clip_intcurve_to_faces(body, ic, fa, fb, tol) {
            if (t1 - t0).abs() < 1e-9 {
                continue;
            }
            let p0 = ic.curve3.eval(t0);
            let p1 = ic.curve3.eval(t1);
            if !full_period && p0.distance(p1) <= tol.max(1e-9) {
                continue; // degenerate near-zero chord — nothing useful to imprint
            }
            let (kind_a, kind_b, t0, t1) = if full_period {
                let outer_a = body.faces.get(fa).and_then(|f| f.outer);
                let outer_b = body.faces.get(fb).and_then(|f| f.outer);
                // `touches` (from `clip_intcurve_to_faces`) are the ACTUAL parameters where
                // the curve grazes a boundary, wherever those really are — empty means
                // neither support's trim showed a gap, so it's genuinely interior on both.
                let touch_pts: Vec<Pnt3> = touches.iter().map(|&t| ic.curve3.eval(t)).collect();
                let ka = if outer_a.is_some_and(|l| touch_pts.iter().any(|&p| point_touches_loop_boundary(body, l, p, tol))) { ImprintKind::SeamCrossing } else { ImprintKind::Interior };
                let kb = if outer_b.is_some_and(|l| touch_pts.iter().any(|&p| point_touches_loop_boundary(body, l, p, tol))) { ImprintKind::SeamCrossing } else { ImprintKind::Interior };
                // A `SeamCrossing` split needs its own imprint edge's (v0==v1) vertex
                // placed exactly AT the physical seam touch — `split_face_by_seam_crossing`
                // finds the vertex on the loop via `splice_boundary_vertex`'s point-on-edge
                // test, which only succeeds if the vertex genuinely lies on the seam edge.
                // The un-anchored `(t0, t1)` range (the clip's own arbitrary domain start,
                // not the touch point) places the vertex at `curve3.eval(t0)` instead —
                // almost never on the seam. Re-anchor the SAME closed period to start at
                // the first detected touch parameter instead (harmless for `Interior`: any
                // start point on a closed loop is topologically equivalent there).
                if (matches!(ka, ImprintKind::SeamCrossing) || matches!(kb, ImprintKind::SeamCrossing)) && !touches.is_empty() {
                    let anchor = touches[0];
                    (ka, kb, anchor, anchor + (t1 - t0))
                } else {
                    (ka, kb, t0, t1)
                }
            } else {
                (ImprintKind::Open, ImprintKind::Open, t0, t1)
            };
            // A segment can run exactly ALONG one operand's pre-existing boundary edge (a
            // plane through a sphere's own centre meets it in that sphere's own seam
            // meridian). Imprinting a second, geometrically identical edge there would
            // leave duplicate topology, so the existing edge is subdivided and REUSED as
            // the shared edge, and that operand queues no imprint of its own — the
            // boundary it needs is already there.
            let along_a = if full_period { None } else { coincident_boundary_edge(body, fa, ic, (t0, t1), tol) };
            let along_b = if full_period || along_a.is_some() { None } else { coincident_boundary_edge(body, fb, ic, (t0, t1), tol) };
            let endpoints = (ic.curve3.eval(t0), ic.curve3.eval(t1));
            let edge_id = match (along_a, along_b) {
                (Some(_), _) => boundary_subedge(body, fa, endpoints, (tol, weld), rec)?,
                (None, Some(_)) => boundary_subedge(body, fb, endpoints, (tol, weld), rec)?,
                (None, None) => build_imprint_edge(body, ic, (t0, t1), full_period, (tol, weld), rec),
            };
            let prange = oriented_prange(body, edge_id, endpoints, (t0, t1));
            let pca = body.curves2.insert(pcurve_for_clip(sa, &ic.curve3, &ic.pcurve_a, (t0, t1), tol));
            let pcb = body.curves2.insert(pcurve_for_clip(&sb, &ic.curve3, &ic.pcurve_b, (t0, t1), tol));
            if along_a.is_none() {
                pending_a.entry(fa).or_default().push(Pending { edge_id, pcurve_id: pca, prange, kind: kind_a });
            }
            if along_b.is_none() {
                pending_b.entry(fb).or_default().push(Pending { edge_id, pcurve_id: pcb, prange, kind: kind_b });
            }
        }
    }
    Ok(())
}

// #endregion 🔖️Imprint

// #region ⏱️ResumableBoolean

/// ⏱️ What a [`BooleanJob`] is currently doing. Phases run in declaration order; `Complete` and
/// `Cancelled` are terminal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BooleanPhase {
    /// ✂️ Imprinting one (face-of-A, face-of-B) pair per unit.
    #[default]
    Imprint,
    /// 🧩 Applying A's queued imprints, one face per unit.
    ApplyA,
    /// 🧩 Applying B's queued imprints, one face per unit.
    ApplyB,
    /// 🎯 Classifying one piece of A against B per unit.
    ClassifyA,
    /// 🎯 Classifying one piece of B against A per unit.
    ClassifyB,
    /// 🧵 Stitching the selected faces into shells/solids and retiring the operands.
    Stitch,
    /// 🩺 Validating the result, one [`BodyValidationJob`] unit per unit.
    Validate,
    Complete,
    Cancelled,
}

impl BooleanPhase {
    /// 🏷️ The stable wire tag every host/extension/UI layer names this phase by.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn tag(self) -> &'static str {
        match self {
            Self::Imprint => "imprint",
            Self::ApplyA => "applyA",
            Self::ApplyB => "applyB",
            Self::ClassifyA => "classifyA",
            Self::ClassifyB => "classifyB",
            Self::Stitch => "stitch",
            Self::Validate => "validate",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
        }
    }
}

/// 📈 Progress of one resumable boolean. `units_done` never decreases. `units_total` is the plan
/// known SO FAR and is revised upward exactly once — when the stitch lands and the result's
/// validation plan (how many entities there are to check) can finally be read; nothing about the
/// post-stitch body is knowable before the stitch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BooleanProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub faces_done: usize,
    pub faces_total: usize,
    pub phase: BooleanPhase,
}

/// ⏱️ Outcome of one budgeted [`BooleanJob::step`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanStep {
    /// 🔁 Budget spent, work remains — call `step` again.
    Working(BooleanProgress),
    /// ✅ Terminal: the result solid is live in the body.
    Done(SolidId),
    /// 🛑 Terminal: [`BooleanJob::cancel`] retired the job; no solid is produced.
    Cancelled(BooleanProgress),
}

/// 🔀 What [`boolean_job`] admitted: either a fast path already answered (microseconds, nothing to
/// resume), or the general engine owes a resumable job.
pub enum BooleanAdmission {
    /// ⚡️ A trivial/box fast path answered exactly, in place.
    Answered(SolidId),
    /// ⏱️ The general exact engine, as a budgetable job.
    Job(BooleanJob),
}

/// ⏱️ The general exact imprint→classify→select→stitch→validate pipeline split into budgetable
/// units, so a host can run it inside an interactive step ceiling across many turns, report
/// progress, and cancel it — the boolean twin of
/// [`crate::standards::v1::subsets::brep::schema::inferences::tessellation::TessellationJob`].
///
/// One unit is one face PAIR (imprint), one face (apply/classify) or one
/// [`BodyValidationJob`] unit (validate). Measured on `🍩️sphere-cut-with-torus` in a native debug
/// build the whole operation costs 4.5 s, of which the stitch is 1.5 s and the validation 3.0 s —
/// so the validation's per-face units are the ones that actually make this preemptible, and the
/// stitch (one unit, group + signed-volume probe + containment) is the coarsest step this design
/// admits (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
///
/// 🚧️ A cancelled or failed job leaves the imprints it already applied in the body — exactly what
/// a boolean that returns `Err` mid-pipeline has always left. The operand handles stay valid (a
/// split face covers the same surface region); nothing is rolled back, because the kernel has no
/// transaction.
pub struct BooleanJob {
    a: SolidId,
    b: SolidId,
    op: BooleanOp,
    tol: f64,
    pre_existing_solids: HashSet<SolidId>,
    coincident_a: HashSet<FaceId>,
    faces_a: Vec<FaceId>,
    faces_b: Vec<FaceId>,
    /// 🔗 Per-face-of-A support cache: the surface and AABB the whole `fb` row is tested against,
    /// read once per row instead of once per pair.
    row: Option<(Surface, crate::standards::v1::subsets::brep::schema::engine::Aabb)>,
    weld: Vec<(Pnt3, VertexId)>,
    pending_a: HashMap<FaceId, Vec<Pending>>,
    pending_b: HashMap<FaceId, Vec<Pending>>,
    pieces_a: Vec<FaceId>,
    pieces_b: Vec<FaceId>,
    selected: Vec<FaceId>,
    result: Option<SolidId>,
    validation: Option<BodyValidationJob>,
    cursor_a: usize,
    cursor_b: usize,
    apply_a: usize,
    apply_b: usize,
    classify_a: usize,
    classify_b: usize,
    units_done: usize,
    units_total: usize,
    phase: BooleanPhase,
}

impl BooleanJob {
    /// 🔀 Plans the general exact boolean of `a` and `b`. Every plan input (which faces exist,
    /// which pairs are coincident, the weld table) is read ONCE here, so the walk below is a pure
    /// cursor advance.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn new(body: &Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64) -> Self {
        let pre_existing_solids: HashSet<SolidId> = body.solids.iter().map(|(id, _)| id).collect();
        let faces_a_all = body.solid_faces(a);
        let faces_b_all = body.solid_faces(b);
        let coincident = find_coincident_face_pairs(body, &faces_a_all, &faces_b_all, tol);
        let coincident_b: HashSet<FaceId> = coincident.iter().map(|&(_, fb)| fb).collect();
        let coincident_a: HashSet<FaceId> = coincident.iter().map(|&(fa, _)| fa).collect();
        let faces_b: Vec<FaceId> = faces_b_all.iter().copied().filter(|f| !coincident_b.contains(f)).collect();
        let faces_a: Vec<FaceId> = faces_a_all;
        // 🔗 Imprint endpoints are welded against each other AND against both operands' existing
        // boundary vertices: an intersection segment that ends exactly on a pole (a plane through a
        // sphere's centre ends its arc there) must reuse that pole's vertex, otherwise the chain's
        // end is a fresh id that `splice_boundary_vertex` cannot find on the ring.
        let mut weld: Vec<(Pnt3, VertexId)> = Vec::new();
        for &face in faces_a.iter().chain(faces_b_all.iter()) {
            for coedge_id in body.face_coedges(face) {
                let Some((vertex_id, _)) = body.coedge_endpoints(coedge_id) else { continue };
                let Some(vertex) = body.vertices.get(vertex_id) else { continue };
                if !weld.iter().any(|&(_, existing)| existing == vertex_id) {
                    weld.push((vertex.position, vertex_id));
                }
            }
        }
        let units_total = faces_a.len() * faces_b.len() + 2 * (faces_a.len() + faces_b.len()) + 1;
        Self {
            a,
            b,
            op,
            tol,
            pre_existing_solids,
            coincident_a,
            faces_a,
            faces_b,
            row: None,
            weld,
            pending_a: HashMap::new(),
            pending_b: HashMap::new(),
            pieces_a: Vec::new(),
            pieces_b: Vec::new(),
            selected: Vec::new(),
            result: None,
            validation: None,
            cursor_a: 0,
            cursor_b: 0,
            apply_a: 0,
            apply_b: 0,
            classify_a: 0,
            classify_b: 0,
            units_done: 0,
            units_total,
            phase: BooleanPhase::Imprint,
        }
    }

    /// 📈 This job's progress right now — safe to read between steps and after termination.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn progress(&self) -> BooleanProgress {
        BooleanProgress {
            units_done: self.units_done,
            units_total: self.units_total.max(self.units_done),
            faces_done: self.apply_a + self.apply_b + self.classify_a + self.classify_b,
            faces_total: 2 * (self.faces_a.len() + self.faces_b.len()),
            phase: self.phase,
        }
    }

    /// 🛑 Retires this job at the next observable boundary. A completed job is never cancelled —
    /// supersession may only retire work still in flight, never destroy a result already paid for
    /// (the same law `TessellationJob::cancel` states).
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn cancel(&mut self) {
        if matches!(self.phase, BooleanPhase::Complete) {
            return;
        }
        self.phase = BooleanPhase::Cancelled;
        self.pending_a.clear();
        self.pending_b.clear();
        self.weld.clear();
        self.validation = None;
    }

    /// ✅ True once the job reached a terminal phase.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn is_terminal(&self) -> bool {
        matches!(self.phase, BooleanPhase::Complete | BooleanPhase::Cancelled)
    }

    /// ♾️ Runs every remaining unit in one call — the unbudgeted façade [`boolean_solid`] is.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn run_to_completion(mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
        loop {
            match self.step(body, rec, usize::MAX)? {
                BooleanStep::Done(id) => return Ok(id),
                BooleanStep::Cancelled(_) => return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean cancelled".into()))),
                BooleanStep::Working(_) => continue,
            }
        }
    }

    /// ⏱️ Advances by at most `budget` units. A `budget` of zero is a legal progress probe that
    /// performs no work.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn step(&mut self, body: &mut Body, rec: &mut OpRecorder, budget: usize) -> Result<BooleanStep, KernelError> {
        match self.phase {
            BooleanPhase::Complete => return Ok(BooleanStep::Done(self.result.ok_or_else(|| KernelError::Boolean(BooleanError::InvalidResult("completed boolean carries no solid".into())))?)),
            BooleanPhase::Cancelled => return Ok(BooleanStep::Cancelled(self.progress())),
            _ => {}
        }
        let mut spent = 0usize;
        while spent < budget {
            match self.phase {
                BooleanPhase::Imprint => {
                    if self.cursor_a >= self.faces_a.len() {
                        self.phase = BooleanPhase::ApplyA;
                        continue;
                    }
                    let fa = self.faces_a[self.cursor_a];
                    if self.coincident_a.contains(&fa) || self.faces_b.is_empty() {
                        self.cursor_a += 1;
                        self.cursor_b = 0;
                        self.row = None;
                        continue;
                    }
                    if self.row.is_none() {
                        let Some(face_a) = body.faces.get(fa).cloned() else {
                            self.cursor_a += 1;
                            self.cursor_b = 0;
                            continue;
                        };
                        let Some(sa) = body.surfaces.get(face_a.surface).cloned() else {
                            self.cursor_a += 1;
                            self.cursor_b = 0;
                            continue;
                        };
                        let Ok(bb_a) = face_aabb(body, fa) else {
                            self.cursor_a += 1;
                            self.cursor_b = 0;
                            continue;
                        };
                        self.row = Some((sa, bb_a));
                    }
                    if self.cursor_b >= self.faces_b.len() {
                        self.cursor_a += 1;
                        self.cursor_b = 0;
                        self.row = None;
                        continue;
                    }
                    let fb = self.faces_b[self.cursor_b];
                    let (sa, bb_a) = self.row.take().expect("row cached above");
                    let outcome = imprint_face_pair(body, (fa, fb), (&sa, &bb_a), self.tol, &mut self.weld, (&mut self.pending_a, &mut self.pending_b), rec);
                    self.row = Some((sa, bb_a));
                    outcome?;
                    self.cursor_b += 1;
                }
                BooleanPhase::ApplyA => {
                    if self.apply_a >= self.faces_a.len() {
                        self.phase = BooleanPhase::ApplyB;
                        continue;
                    }
                    let fa = self.faces_a[self.apply_a];
                    if self.coincident_a.contains(&fa) {
                        self.pieces_a.push(fa);
                    } else {
                        match self.pending_a.remove(&fa) {
                            Some(list) => {
                                let pieces = apply_pending_imprints(body, fa, list, self.tol, rec)?;
                                self.pieces_a.extend(pieces);
                            }
                            None => self.pieces_a.push(fa),
                        }
                    }
                    self.apply_a += 1;
                }
                BooleanPhase::ApplyB => {
                    if self.apply_b >= self.faces_b.len() {
                        self.phase = BooleanPhase::ClassifyA;
                        continue;
                    }
                    let fb = self.faces_b[self.apply_b];
                    match self.pending_b.remove(&fb) {
                        Some(list) => {
                            let pieces = apply_pending_imprints(body, fb, list, self.tol, rec)?;
                            self.pieces_b.extend(pieces);
                        }
                        None => self.pieces_b.push(fb),
                    }
                    self.apply_b += 1;
                }
                BooleanPhase::ClassifyA => {
                    if self.classify_a >= self.pieces_a.len() {
                        self.phase = BooleanPhase::ClassifyB;
                        continue;
                    }
                    let f = self.pieces_a[self.classify_a];
                    if self.coincident_a.contains(&f) {
                        if matches!(self.op, BooleanOp::Unite | BooleanOp::Intersect) {
                            self.selected.push(f);
                        }
                    } else {
                        let class = classify_face_against_solid(body, f, self.b, self.tol)?;
                        if keep_face(self.op, true, class) {
                            self.selected.push(f);
                        }
                    }
                    self.classify_a += 1;
                }
                BooleanPhase::ClassifyB => {
                    if self.classify_b >= self.pieces_b.len() {
                        self.phase = BooleanPhase::Stitch;
                        continue;
                    }
                    let f = self.pieces_b[self.classify_b];
                    let class = classify_face_against_solid(body, f, self.a, self.tol)?;
                    if keep_face(self.op, false, class) {
                        if matches!(self.op, BooleanOp::Cut) {
                            flip_face(body, f);
                        }
                        self.selected.push(f);
                    }
                    self.classify_b += 1;
                }
                BooleanPhase::Stitch => {
                    if self.selected.is_empty() {
                        return Err(KernelError::Boolean(BooleanError::InvalidResult("exact boolean selection kept no faces".into())));
                    }
                    let selected_set: HashSet<FaceId> = self.selected.iter().copied().collect();
                    let result = stitch_selected_faces(body, &self.selected, self.tol, rec)?;
                    remove_solid_and_orphans(body, self.a, &selected_set, rec);
                    remove_solid_and_orphans(body, self.b, &selected_set, rec);
                    gc_orphan_edges_and_vertices(body, rec);
                    self.result = Some(result);
                    let validation = BodyValidationJob::new(body);
                    self.units_total = self.units_done + 1 + validation.progress().units_total;
                    self.validation = Some(validation);
                    self.phase = BooleanPhase::Validate;
                    self.units_done += 1;
                    spent += 1;
                    continue;
                }
                BooleanPhase::Validate => {
                    let Some(validation) = self.validation.as_mut() else {
                        self.phase = BooleanPhase::Complete;
                        continue;
                    };
                    if validation.is_complete() {
                        let issues = issues_scoped_to_new_solids(body, &self.pre_existing_solids, self.validation.take().expect("checked above").into_issues());
                        if !issues.is_empty() {
                            let listed: Vec<String> = issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect();
                            return Err(KernelError::Boolean(BooleanError::InvalidResult(format!("exact boolean result failed validation: {} issue(s): {}", issues.len(), listed.join(" | ")))));
                        }
                        self.phase = BooleanPhase::Complete;
                        continue;
                    }
                    let before = validation.progress().units_done;
                    let after = validation.step(body, budget - spent).units_done;
                    let advanced = after.saturating_sub(before).max(1);
                    self.units_done += advanced;
                    spent += advanced;
                    continue;
                }
                BooleanPhase::Complete => return Ok(BooleanStep::Done(self.result.ok_or_else(|| KernelError::Boolean(BooleanError::InvalidResult("completed boolean carries no solid".into())))?)),
                BooleanPhase::Cancelled => return Ok(BooleanStep::Cancelled(self.progress())),
            }
            self.units_done += 1;
            spent += 1;
        }
        match self.phase {
            BooleanPhase::Complete => Ok(BooleanStep::Done(self.result.ok_or_else(|| KernelError::Boolean(BooleanError::InvalidResult("completed boolean carries no solid".into())))?)),
            BooleanPhase::Cancelled => Ok(BooleanStep::Cancelled(self.progress())),
            _ => Ok(BooleanStep::Working(self.progress())),
        }
    }
}

/// 🔀 Admits a boolean: runs the microsecond-cheap trivial/box fast paths in place and, when
/// neither applies, hands back the general engine as a resumable [`BooleanJob`]. This is the
/// entry point an interactive host drives; [`boolean_solid`] is the unbudgeted façade over it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boolean_job(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<BooleanAdmission, KernelError> {
    require_tol(tol)?;
    require_solid(body, a)?;
    require_solid(body, b)?;
    if a == b {
        return Err(KernelError::InvalidInput("boolean operands must be distinct solids".into()));
    }
    let bb_a = solid_bounding_box(body, a)?;
    let bb_b = solid_bounding_box(body, b)?;
    if aabb_finite(&bb_a) && aabb_finite(&bb_b) {
        if let Some(id) = trivial_topology_fast_path(body, a, b, (&bb_a, &bb_b), op, tol, rec)? {
            return Ok(BooleanAdmission::Answered(id));
        }
        if let Some(id) = box_fast_path(body, a, b, (&bb_a, &bb_b), op, tol, rec)? {
            return Ok(BooleanAdmission::Answered(id));
        }
    }
    Ok(BooleanAdmission::Job(BooleanJob::new(body, a, b, op, tol)))
}

// #endregion ⏱️ResumableBoolean

// #region 🔖️Scoping
/// 🔀 Filters `issues` down to the ones attributable to solids the boolean just CREATED — a
/// pre-existing operand may already have carried invalid topology, and the boolean must not be
/// blamed for it. Resolves each issue's `entity` string against a forward map of every string
/// [`validate_body`]'s checks can emit for something owned by a `pre_existing_solids` member
/// (built by [`pre_existing_entity_strings`]) rather than inverting `raw_index()` back into an id,
/// which would silently desync the moment either side's format changes independently. An entity
/// that resolves to neither a pre-existing nor a new solid (orphaned, or unparseable) is kept —
/// conservative, since a boolean that leaves an unattributable issue is a real failure.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn issues_scoped_to_new_solids(body: &Body, pre_existing_solids: &HashSet<SolidId>, issues: Vec<ValidationIssue>) -> Vec<ValidationIssue> {
    let stale = pre_existing_entity_strings(body, pre_existing_solids);
    issues.into_iter().filter(|issue| !stale.contains(&issue.entity)).collect()
}

/// 🔀 Every `entity` string [`validate_body`]'s checks can emit for something owned — directly, or
/// via shell/face/loop/coedge containment down to the edge and vertex level — by one of `solids`.
/// See [`issues_scoped_to_new_solids`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pre_existing_entity_strings(body: &Body, solids: &HashSet<SolidId>) -> HashSet<String> {
    let mut owned = HashSet::new();
    for &solid_id in solids {
        owned.insert(format!("solid-{}", solid_id.raw_index()));
        let Some(solid) = body.solids.get(solid_id) else { continue };
        for &void_shell in &solid.inners {
            owned.insert(format!("solid-{}-void-shell-{}", solid_id.raw_index(), void_shell.raw_index()));
        }
        let faces = body.solid_faces(solid_id);
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                owned.insert(format!("face-{}-face-{}", faces[i].raw_index(), faces[j].raw_index()));
            }
        }
        for &face_id in &faces {
            owned.insert(format!("face-{}", face_id.raw_index()));
            for loop_id in body.face_loops(face_id) {
                owned.insert(format!("loop-{}", loop_id.raw_index()));
                for coedge_id in body.loop_coedges(loop_id) {
                    owned.insert(format!("coedge-{}", coedge_id.raw_index()));
                    let Some(coedge) = body.coedges.get(coedge_id) else { continue };
                    owned.insert(format!("edge-{}", coedge.edge.raw_index()));
                    let Some(edge) = body.edges.get(coedge.edge) else { continue };
                    owned.insert(format!("vertex-{}", edge.v0.raw_index()));
                    owned.insert(format!("vertex-{}", edge.v1.raw_index()));
                }
            }
        }
        for shell_id in body.solid_shells(solid_id) {
            for face_id in body.shell_faces(shell_id) {
                for coedge_id in body.face_coedges(face_id) {
                    let Some(coedge) = body.coedges.get(coedge_id) else { continue };
                    owned.insert(format!("shell-{}-edge-{}", shell_id.raw_index(), coedge.edge.raw_index()));
                }
            }
        }
    }
    owned
}

// #region 🔖️Clip

/// 🔀 Clips `ic`'s shared parameter domain to the sub-ranges where BOTH `pcurve_a` (on `face_a`)
/// and `pcurve_b` (on `face_b`) land inside their own face's trim, sampling at a fixed resolution
/// (matching [`crate::standards::v1::subsets::brep::schema::diff::intersect::surface_surface::general_marching`]'s own documented-simplification sampling style).
/// A periodic curve whose valid runs cover (within 0.1% of the period) the WHOLE domain is
/// returned as ONE fully-closed range (`closed = true`) — this covers both "never touches either
/// boundary" and "touches a seam at exactly one physical point," which reads as a narrow gap
/// splitting one closed loop into two adjacent open runs (a latitude circle's seam touch can land
/// at any phase, not just the circle's own `t=0`/`t=period` ends). Otherwise every maximal valid
/// sub-run becomes an open range with its boundary refined by bisection (`closed = false`). Does
/// not handle a genuinely PARTIAL run that wraps across the periodic domain's own `t=lo`/`t=hi`
/// seam (documented gap — not hit by the analytic primitive pairs this wave targets).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn clip_intcurve_to_faces(body: &Body, ic: &IntCurve, face_a: FaceId, face_b: FaceId, tol: f64) -> Vec<(f64, f64, bool, Vec<f64>)> {
    let (lo, hi) = intcurve_finite_bracket(body, ic, face_a, face_b);
    if hi.partial_cmp(&lo) != Some(std::cmp::Ordering::Greater) {
        return Vec::new();
    }
    let periodic = ic.curve3.is_periodic() && ic.curve3.period().is_some_and(|p| (hi - lo - p).abs() < 1e-6 * p.max(1.0));
    const N: usize = 64;
    // The trim's CLOSURE, not its interior: a clip that treats "on the boundary" as outside stops
    // one sample short of a sphere's pole (which IS its `v = ±π/2` boundary), never reaches a
    // periodic seam, and shreds an arc that runs exactly along an operand's own edge into
    // borderline fragments — all three of the procedural examples' failure modes. See
    // `classification::point_in_face_uv_closure`.
    let valid = |t: f64| -> bool {
        let ua = wrap_uv_for_surface(body, face_a, ic.pcurve_a.eval(t));
        let ub = wrap_uv_for_surface(body, face_b, ic.pcurve_b.eval(t));
        point_in_face_uv_closure_periodic(body, face_a, ua, tol) && point_in_face_uv_closure_periodic(body, face_b, ub, tol)
    };
    let mut inside = [false; N + 1];
    for (i, slot) in inside.iter_mut().enumerate() {
        let t = lo + (hi - lo) * (i as f64 / N as f64);
        *slot = valid(t);
    }
    let cell = (hi - lo) / N as f64;
    let reach = (cell, curve3_extent(&ic.curve3, (lo, lo + cell)).max(1e-12) * 2.0);
    let mut runs: Vec<(f64, f64)> = Vec::new();
    let mut i = 0usize;
    while i <= N {
        if inside[i] {
            let start = i;
            while i <= N && inside[i] {
                i += 1;
            }
            let end = i - 1;
            let t_at = |k: usize| lo + (hi - lo) * (k as f64 / N as f64);
            let t0 = if start > 0 { snap_clip_endpoint(body, ic, (face_a, face_b), refine_boundary(&valid, t_at(start - 1), t_at(start)), reach, tol) } else { t_at(start) };
            let t1 = if end < N { snap_clip_endpoint(body, ic, (face_a, face_b), refine_boundary(&valid, t_at(end + 1), t_at(end)), reach, tol) } else { t_at(end) };
            if t1 > t0 {
                runs.push((t0, t1));
            }
        } else {
            i += 1;
        }
    }
    if periodic && !runs.is_empty() {
        // A periodic curve whose valid runs account for (essentially) the whole period never
        // leaves either trim: it is ONE closed loop, not the two adjacent open runs a seam touch
        // used to read as. Under the closed trim test a seam touch leaves no gap at all, so the
        // touch parameters can no longer be read off the gaps and are solved for directly
        // ([`boundary_touch_parameters`]) — which is also strictly more accurate, since a gap's
        // midpoint was only ever as good as the sampling that produced the gap.
        let covered: f64 = runs.iter().map(|&(t0, t1)| t1 - t0).sum();
        if (hi - lo - covered) < (hi - lo) * 1e-3 {
            return vec![(lo, hi, true, boundary_touch_parameters(body, ic, (face_a, face_b), (lo, hi), tol))];
        }
    }
    runs.into_iter().map(|(t0, t1)| (t0, t1, false, Vec::new())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refine_boundary(valid: &impl Fn(f64) -> bool, mut outside: f64, mut inside: f64) -> f64 {
    for _ in 0..40 {
        let mid = 0.5 * (outside + inside);
        if valid(mid) {
            inside = mid;
        } else {
            outside = mid;
        }
    }
    inside
}

/// 🔀 [`IntCurve::domain`] is infinite for an unbounded [`crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Line`] (e.g. plane/plane,
/// coincident-cylinder-axis lines); windows it around the two faces' combined AABB so sampling
/// stays finite and relevant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn intcurve_finite_bracket(body: &Body, ic: &IntCurve, face_a: FaceId, face_b: FaceId) -> (f64, f64) {
    let (lo, hi) = (ic.domain.min, ic.domain.max);
    if lo.is_finite() && hi.is_finite() {
        return (lo, hi);
    }
    let mut radius = 100.0;
    let mut center = 0.0;
    if let (Ok(ba), Ok(bb)) = (face_aabb(body, face_a), face_aabb(body, face_b)) {
        let dx = (ba.max[0] - ba.min[0]).max(bb.max[0] - bb.min[0]);
        let dy = (ba.max[1] - ba.min[1]).max(bb.max[1] - bb.min[1]);
        let dz = (ba.max[2] - ba.min[2]).max(bb.max[2] - bb.min[2]);
        radius = (dx * dx + dy * dy + dz * dz).sqrt().max(1.0) * 4.0;
        let centroid = Pnt3::new((ba.min[0] + ba.max[0] + bb.min[0] + bb.max[0]) * 0.25, (ba.min[1] + ba.max[1] + bb.min[1] + bb.max[1]) * 0.25, (ba.min[2] + ba.max[2] + bb.min[2] + bb.max[2]) * 0.25);
        if let crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Line { origin, dir } = &ic.curve3 {
            let n2 = dir.norm_sq();
            if n2 > 1e-30 {
                center = dir.dot(centroid - *origin) / n2;
            }
        }
    }
    (if lo.is_finite() { lo } else { center - radius }, if hi.is_finite() { hi } else { center + radius })
}

/// 🔀 Diameter (max pairwise distance) of 16 samples of `curve` across `domain` — a cheap,
/// curve-kind-agnostic stand-in for "how big is this intersection curve", used to distinguish a
/// genuine contact circle/ellipse from a near-zero-radius numerical artifact of a tolerant
/// overlap test (see the near-tangent-spheres skip at this function's call site).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn curve3_extent(curve: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3, domain: (f64, f64)) -> f64 {
    const K: usize = 16;
    let (lo, hi) = domain;
    if hi.partial_cmp(&lo) != Some(std::cmp::Ordering::Greater) {
        return 0.0;
    }
    let pts: Vec<Pnt3> = (0..=K).map(|i| curve.eval(lo + (hi - lo) * (i as f64 / K as f64))).collect();
    let mut max_d = 0.0f64;
    for i in 0..pts.len() {
        for j in (i + 1)..pts.len() {
            max_d = max_d.max(pts[i].distance(pts[j]));
        }
    }
    max_d
}

/// 🔀 `true` when `point` lies within `tol` of `loop_id`'s own boundary edges — distinguishes a
/// closed imprint curve that genuinely bounds a small interior sub-region from one that grazes a
/// doubly-used seam edge (see [`ImprintKind::SeamCrossing`]). Uses [`closest_parameter`]'s
/// certified/analytic projection (exact for `Line`/`Circle`/`Ellipse`, Newton-refined for
/// `Nurbs`), not a fixed sample grid: 24 discrete samples per edge (the previous approach) are
/// almost always MORE than `tol` away from the true closest point on the curve even when `point`
/// lies exactly ON it — e.g. a great-circle seam edge sampled every ~0.13 rad has no reason to
/// land within `1e-6` of an arbitrary touch parameter — silently misclassifying every genuine
/// seam touch as `Interior`. 🐛 Found live: a sphere/sphere lens's touch point at the seam's own
/// v≈0.6435 rad sampled false under the old 24-point check, misrouting the split to `Interior`,
/// whose resulting polar-cap face then has a UV boundary that is a straight line across the full
/// periodic `u` domain (zero enclosed area in flat UV) — `interior_point_of_face`'s grid scan
/// then correctly, but unhelpfully, finds no inside point at all.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_touches_loop_boundary(body: &Body, loop_id: LoopId, point: Pnt3, tol: f64) -> bool {
    let linear = tol.max(1e-9);
    for cid in body.loop_coedges(loop_id) {
        let Some(co) = body.coedges.get(cid) else { continue };
        let Some(edge) = body.edges.get(co.edge) else { continue };
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        let cp = closest_parameter(curve, edge.range, point, linear);
        if cp.distance <= linear {
            return true;
        }
    }
    false
}

/// 🔀 Wraps a p-curve sample into the face's own surface's natural periodic domain before a trim
/// test — a periodic curve's own parametrization can legitimately produce `u`/`v` outside
/// `[0, 2π)` (e.g. a phase-shifted azimuthal line), which is still the same physical point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_uv_for_surface(body: &Body, face: FaceId, uv: Pnt2) -> Pnt2 {
    let Some(face_data) = body.faces.get(face) else { return uv };
    let Some(surface) = body.surfaces.get(face_data.surface) else { return uv };
    let mut out = uv;
    if surface.is_u_periodic() {
        out.x = out.x.rem_euclid(std::f64::consts::TAU);
    }
    if surface.is_v_periodic() {
        out.y = out.y.rem_euclid(std::f64::consts::TAU);
    }
    out
}

/// 🔀 `point_in_face_uv`, but tolerant of a periodic loop's own UV representation having drifted
/// by any whole number of periods from `wrap_uv_for_surface`'s canonical `[0, 2π)` window.
/// `classification::loop_uv_polygon_sampled` unwraps each face's OWN boundary polygon
/// continuously starting from its own first sample (chaining `unwrap_angle` across coedges), so
/// two DIFFERENT faces' polygons can legitimately end up centered on different multiples of `2π`
/// even for the "same" physical seam — comparing a single canonically-wrapped query point against
/// that polygon is not reliable. Instead of assuming any particular offset, tries the query at
/// every small integer shift and accepts the first that lands inside — cheap (a handful of extra
/// `point_in_face_uv` calls) and correct regardless of which offset either polygon settled on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_face_uv_periodic(body: &Body, face: FaceId, uv: Pnt2, tol: f64) -> bool {
    uv_probe_over_period_shifts(body, face, uv, |candidate| point_in_face_uv(body, face, candidate, tol).unwrap_or(false))
}

/// 🔀 [`point_in_face_uv_periodic`] against the trim's CLOSURE — see
/// [`crate::standards::v1::subsets::brep::schema::inferences::classification::point_in_face_uv_closure`]
/// for why a clip must use the closed test (poles, seams and coincident arcs all live exactly on
/// the boundary and are invisible to the open one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_face_uv_closure_periodic(body: &Body, face: FaceId, uv: Pnt2, tol: f64) -> bool {
    uv_probe_over_period_shifts(body, face, uv, |candidate| point_in_face_uv_closure(body, face, candidate, tol).unwrap_or(false))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn uv_probe_over_period_shifts(body: &Body, face: FaceId, uv: Pnt2, probe: impl Fn(Pnt2) -> bool) -> bool {
    let Some(face_data) = body.faces.get(face) else { return false };
    let Some(surface) = body.surfaces.get(face_data.surface) else { return false };
    let tau = std::f64::consts::TAU;
    let u_shifts: &[f64] = if surface.is_u_periodic() { &[0.0, tau, -tau, 2.0 * tau, -2.0 * tau, 3.0 * tau, -3.0 * tau] } else { &[0.0] };
    let v_shifts: &[f64] = if surface.is_v_periodic() { &[0.0, tau, -tau, 2.0 * tau, -2.0 * tau] } else { &[0.0] };
    for &du in u_shifts {
        for &dv in v_shifts {
            if probe(Pnt2::new(uv.x + du, uv.y + dv)) {
                return true;
            }
        }
    }
    false
}

/// 🔀 Every boundary curve (3D curve + its edge's own range) of `face`'s outer ring — the exact
/// geometry a clip endpoint has to land ON, and the geometry a closed imprint curve grazes when it
/// crosses a seam.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn outer_boundary_curves(body: &Body, face: FaceId) -> Vec<(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3, (f64, f64))> {
    let mut out = Vec::new();
    let Some(outer) = body.faces.get(face).and_then(|f| f.outer) else { return out };
    for coedge_id in body.loop_coedges(outer) {
        let Some(coedge) = body.coedges.get(coedge_id) else { continue };
        let Some(edge) = body.edges.get(coedge.edge) else { continue };
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        out.push((curve.clone(), edge.range));
    }
    out
}

/// 🔀 Distance from `point` to the nearest of `curves`, and that curve's index — [`closest_parameter`]
/// is analytic for `Line`/`Circle`/`Ellipse` (including a POINT edge, whose zero-direction line
/// projects to its own origin) and Newton-refined for `Nurbs`, so this is the certified distance,
/// not a sampled estimate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nearest_boundary_curve(curves: &[(crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3, (f64, f64))], point: Pnt3, tol: f64) -> Option<(usize, f64)> {
    let mut best: Option<(usize, f64)> = None;
    for (index, (curve, range)) in curves.iter().enumerate() {
        let d = closest_parameter(curve, *range, point, tol).distance;
        if best.is_none_or(|(_, bd)| d < bd) {
            best = Some((index, d));
        }
    }
    best
}

/// 🔀 Pulls a bisection-refined clip endpoint onto the trim boundary it approximates, EXACTLY.
///
/// [`refine_boundary`] can only ever locate the crossing to wherever the (tolerant) trim predicate
/// flips, i.e. to within `tol` of the truth — and the two faces of a pair flip in different
/// directions, so the SAME physical corner comes out of two different face pairs up to `2·tol`
/// apart, which is outside the imprint weld radius and leaves two arcs that physically meet as an
/// unconnected pair. Alternating projection fixes that at the source: project the endpoint onto the
/// boundary curve it is closest to, re-solve the intersection curve's own parameter for that
/// projected point, repeat. For the transversal crossings a clip produces this is a contraction
/// and converges to the true curve/curve intersection in a handful of steps, so both face pairs
/// land on the identical point and the weld — and therefore the chain — succeeds.
///
/// Returns `t_guess` unchanged when no boundary curve is near enough to be the crossing (the run
/// ended at the sampled domain's own edge, not at a trim), so a genuinely unbounded run is left
/// alone.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn snap_clip_endpoint(body: &Body, ic: &IntCurve, (face_a, face_b): (FaceId, FaceId), t_guess: f64, (param_reach, space_reach): (f64, f64), tol: f64) -> f64 {
    let linear = tol.max(1e-12);
    let mut curves = outer_boundary_curves(body, face_a);
    curves.extend(outer_boundary_curves(body, face_b));
    if curves.is_empty() {
        return t_guess;
    }
    let mut point = ic.curve3.eval(t_guess);
    let Some((index, distance)) = nearest_boundary_curve(&curves, point, linear) else { return t_guess };
    if distance > space_reach {
        return t_guess;
    }
    let (curve, range) = &curves[index];
    let mut t = t_guess;
    for _ in 0..8 {
        let projected = closest_parameter(curve, *range, point, linear).point;
        let solved = closest_parameter(&ic.curve3, (t_guess - param_reach, t_guess + param_reach), projected, linear);
        if (solved.t - t).abs() <= 1e-15 {
            return solved.t;
        }
        t = solved.t;
        point = solved.point;
    }
    t
}

/// 🔀 Parameters at which a CLOSED imprint curve grazes either support face's own boundary ring —
/// the seam touches that decide [`ImprintKind::SeamCrossing`] and anchor its imprint vertex.
///
/// Derived from the geometry directly rather than from gaps in the trim sampling: once the clip
/// uses the trim's CLOSURE (as it must, so a curve reaching a pole or running along a seam is not
/// truncated), a seam touch leaves NO gap to read it off. Scans the distance to the nearest
/// boundary curve, keeps every local minimum that actually reaches within tolerance, and refines
/// each by golden-section search so the anchor lands on the seam edge itself — which is exactly
/// what `splice_boundary_vertex`'s point-on-edge test requires. A curve that is within tolerance of
/// a boundary over a large fraction of its length is not grazing it but COINCIDENT with it
/// ([`coincident_boundary_edge`]'s case), and yields no touches here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn boundary_touch_parameters(body: &Body, ic: &IntCurve, (face_a, face_b): (FaceId, FaceId), (lo, hi): (f64, f64), tol: f64) -> Vec<f64> {
    const N: usize = 512;
    let linear = tol.max(1e-9);
    let mut curves = outer_boundary_curves(body, face_a);
    curves.extend(outer_boundary_curves(body, face_b));
    if curves.is_empty() || hi <= lo {
        return Vec::new();
    }
    let step = (hi - lo) / N as f64;
    let at = |i: usize| lo + step * i as f64;
    let distance = |t: f64| nearest_boundary_curve(&curves, ic.curve3.eval(t), linear).map_or(f64::INFINITY, |(_, d)| d);
    let samples: Vec<f64> = (0..N).map(|i| distance(at(i))).collect();
    if samples.iter().filter(|&&d| d <= linear).count() * 4 > N {
        return Vec::new();
    }
    let mut touches = Vec::new();
    for i in 0..N {
        let prev = samples[(i + N - 1) % N];
        let next = samples[(i + 1) % N];
        if !(samples[i] <= prev && samples[i] < next) {
            continue;
        }
        let refined = golden_section_minimum(&distance, at(i) - step, at(i) + step);
        if distance(refined) <= linear {
            touches.push(refined);
        }
    }
    touches
}

/// 🔀 The p-curve to imprint over the CLIPPED sub-range `(t0, t1)`, repaired if the one
/// `intersect_surface_surface` supplied does not actually hold there.
///
/// An SSI p-curve is certified against the intersection curve's FULL natural domain, and where no
/// closed form exists it is a global interpolation of 33 inversion samples. A boolean never uses
/// the full domain — it imprints a clipped sub-range — and a global fit can be arbitrarily wrong on
/// one: a great circle through a sphere's POLES has a `u` that jumps by π there, which no single
/// interpolation can represent, so the fit oscillates (measured: 0.14 on a r=1.2 sphere) while
/// still passing through every one of its own nodes, i.e. while still reporting a ~1e-16 error to
/// the sampling that produced it. `validate_body`'s same-parameter check then rejects the finished
/// boolean.
///
/// Measured on the range actually used, and rebuilt there when it misses: the sub-range's inversion
/// samples (exact via [`closest_uv`], with a pole's undefined `u` carried from its neighbour and
/// periodic directions unwrapped) are fitted AFFINELY, which is not a simplification but the exact
/// answer for every sub-arc this stage produces — a latitude circle (`v` constant), a meridian
/// branch (`u` constant, `v` affine), a ruling, or any arc of them. A sub-range that is genuinely
/// not affine keeps the original p-curve rather than trading one wrong answer for another.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pcurve_for_clip(surface: &Surface, curve3: &crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3, pcurve: &Curve2, (t0, t1): (f64, f64), tol: f64) -> Curve2 {
    const K: usize = 33;
    let at = |i: usize| t0 + (t1 - t0) * i as f64 / (K - 1) as f64;
    let deviation = |candidate: &Curve2| (0..K).map(|i| { let uv = candidate.eval(at(i)); surface.eval(uv.x, uv.y).distance(curve3.eval(at(i))) }).fold(0.0f64, f64::max);
    if deviation(pcurve) <= tol {
        return pcurve.clone();
    }
    let domain = surface.domain();
    let mut samples: Vec<Pnt2> = Vec::with_capacity(K);
    let mut degenerate: Vec<bool> = Vec::with_capacity(K);
    for i in 0..K {
        let found = closest_uv(surface, domain, curve3.eval(at(i)), tol.max(1e-12));
        samples.push(Pnt2::new(found.u, found.v));
        degenerate.push(surface.is_degenerate_uv(found.u, found.v));
    }
    let Some(anchor) = (0..K).find(|&i| !degenerate[i]) else { return pcurve.clone() };
    for i in (0..anchor).rev() {
        samples[i].x = samples[i + 1].x;
    }
    for i in (anchor + 1)..K {
        if degenerate[i] {
            samples[i].x = samples[i - 1].x;
        }
    }
    let mut previous = samples[anchor];
    for i in 0..K {
        if surface.is_u_periodic() {
            samples[i].x = unwrap_to(previous.x, samples[i].x);
        }
        if surface.is_v_periodic() {
            samples[i].y = unwrap_to(previous.y, samples[i].y);
        }
        previous = samples[i];
    }
    let fitted = affine_pcurve_through(&samples, (t0, t1));
    if deviation(&fitted) <= tol {
        fitted
    } else {
        pcurve.clone()
    }
}

/// 🔀 Least-squares affine `Curve2::Line` through UV `samples` taken at evenly spaced parameters
/// across `(t0, t1)`, in the SAME absolute parametrization the samples were taken in (so the
/// result is a drop-in replacement for the p-curve it repairs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn affine_pcurve_through(samples: &[Pnt2], (t0, t1): (f64, f64)) -> Curve2 {
    let n = samples.len();
    let at = |i: usize| t0 + (t1 - t0) * i as f64 / (n - 1) as f64;
    let mean_t: f64 = (0..n).map(at).sum::<f64>() / n as f64;
    let mean_u: f64 = samples.iter().map(|p| p.x).sum::<f64>() / n as f64;
    let mean_v: f64 = samples.iter().map(|p| p.y).sum::<f64>() / n as f64;
    let mut stt = 0.0;
    let mut stu = 0.0;
    let mut stv = 0.0;
    for (i, sample) in samples.iter().enumerate() {
        let dt = at(i) - mean_t;
        stt += dt * dt;
        stu += dt * (sample.x - mean_u);
        stv += dt * (sample.y - mean_v);
    }
    let (du, dv) = if stt > 1e-30 { (stu / stt, stv / stt) } else { (0.0, 0.0) };
    Curve2::Line { origin: Pnt2::new(mean_u - du * mean_t, mean_v - dv * mean_t), dir: Vec2::new(du, dv) }
}

/// 🔀 Parameter minimizing `f` on `[lo, hi]` by golden-section search — used to pin a seam touch
/// (a smooth, strictly unimodal distance minimum within one sampling cell) to full precision.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn golden_section_minimum(f: &impl Fn(f64) -> f64, mut lo: f64, mut hi: f64) -> f64 {
    const INV_PHI: f64 = 0.618_033_988_749_894_9;
    let (mut c, mut d) = (hi - (hi - lo) * INV_PHI, lo + (hi - lo) * INV_PHI);
    let (mut fc, mut fd) = (f(c), f(d));
    for _ in 0..80 {
        if fc < fd {
            hi = d;
            d = c;
            fd = fc;
            c = hi - (hi - lo) * INV_PHI;
            fc = f(c);
        } else {
            lo = c;
            c = d;
            fc = fd;
            d = lo + (hi - lo) * INV_PHI;
            fd = f(d);
        }
    }
    0.5 * (lo + hi)
}

// #endregion 🔖️Clip

// #region 🔖️Imprint

/// 🔀 A `closed` imprint vertex gets spliced onto a PRE-EXISTING boundary edge (e.g. a sphere's
/// own seam, built by `primitives` at `Tol::DEFAULT`) — `validate_body` requires a vertex's own
/// tolerance to never exceed any edge that references it, so the vertex can't just inherit the
/// caller's (possibly looser) `tol`; it is clamped to at least as tight as the kernel's own
/// baseline (`Tol::DEFAULT`) so it never exceeds ANY edge it might get spliced into.
/// 🔀 Mints — or REUSES — the imprint vertex at `position`. Two intersection segments that meet at
/// one physical point (three planes cutting one sphere give quarter arcs that share their
/// endpoints) must share one vertex id, otherwise the segments never read as a connected chain and
/// each half-open segment fails the two-crossing chord rule on its own.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn welded_imprint_vertex(body: &mut Body, weld: &mut Vec<(Pnt3, VertexId)>, position: Pnt3, tol_v: Tol, tol: f64, rec: &mut OpRecorder) -> VertexId {
    let radius = tol.max(1e-9);
    if let Some(&(_, existing)) = weld.iter().find(|(p, _)| p.distance(position) <= radius) {
        return existing;
    }
    let created = make_vertex(body, position, tol_v, rec);
    weld.push((position, created));
    created
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_imprint_edge(body: &mut Body, ic: &IntCurve, (t0, t1): (f64, f64), closed: bool, (tol, weld): (f64, &mut Vec<(Pnt3, VertexId)>), rec: &mut OpRecorder) -> EdgeId {
    let curve_id = body.curves3.insert(ic.curve3.clone());
    let tol_v = Tol::new(tol.min(Tol::DEFAULT.value()));
    if closed {
        let v = welded_imprint_vertex(body, weld, ic.curve3.eval(t0), tol_v, tol, rec);
        make_edge(body, curve_id, (t0, t1), v, v, tol_v, rec)
    } else {
        let va = welded_imprint_vertex(body, weld, ic.curve3.eval(t0), tol_v, tol, rec);
        let vb = welded_imprint_vertex(body, weld, ic.curve3.eval(t1), tol_v, tol, rec);
        make_edge(body, curve_id, (t0, t1), va, vb, tol_v, rec)
    }
}

/// 🔀 Locates which of the currently live `active` pieces of one original face a pending imprint
/// belongs in, by sampling its own p-curve. The exact midpoint of `prange` can coincidentally land
/// ON a `SeamCrossing` curve's own touch point (e.g. by construction/symmetry of the analytic
/// circle frame), so several fractions along the range are tried rather than only `s = 0.5`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn locate_active_piece(body: &Body, active: &[FaceId], pcurve_id: Curve2Id, prange: (f64, f64), tol: f64) -> Option<usize> {
    let pc = body.curves2.get(pcurve_id)?.clone();
    for &s in &[0.5, 0.3, 0.7, 0.15, 0.85, 0.05, 0.95] {
        let t = prange.0 + (prange.1 - prange.0) * s;
        let uv = wrap_uv_for_surface(body, active[0], pc.eval(t));
        if let Some(index) = active.iter().position(|&f| point_in_face_uv_periodic(body, f, uv, tol)) {
            return Some(index);
        }
    }
    None
}

/// 🔀 Assembles the `Open` pendings of one face into maximal END-TO-END chains. Imprint vertices
/// are welded across segments ([`welded_imprint_vertex`]), so two arcs that meet physically share
/// one vertex id and this is a plain path walk over that adjacency. Each chain is emitted as
/// [`crate::standards::v1::subsets::brep::schema::diff::euler::split_face_by_chain`]'s member list,
/// oriented from one free end to the other. A component in which every vertex has degree 2 is a
/// CYCLE of open segments — a closed imprint expressed piecewise, which belongs to the interior /
/// seam-crossing splitters and is reported rather than silently mis-split.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chain_open_pendings(body: &Body, pendings: &[Pending]) -> Result<Vec<Vec<ParametricEdge>>, KernelError> {
    let mut ends: HashMap<VertexId, Vec<usize>> = HashMap::new();
    for (index, p) in pendings.iter().enumerate() {
        let edge = body.edges.get(p.edge_id).ok_or_else(|| KernelError::MissingEntity(format!("edge {}", p.edge_id)))?;
        ends.entry(edge.v0).or_default().push(index);
        ends.entry(edge.v1).or_default().push(index);
    }
    let mut used = vec![false; pendings.len()];
    let mut chains = Vec::new();
    let mut starts: Vec<VertexId> = ends.iter().filter(|(_, uses)| uses.len() == 1).map(|(&v, _)| v).collect();
    starts.sort_by_key(|v| v.raw_index());
    for start in starts {
        let Some(&seed) = ends.get(&start).and_then(|uses| uses.first()) else { continue };
        if used[seed] {
            continue;
        }
        let mut chain = Vec::new();
        let mut cursor = start;
        let mut next = Some(seed);
        while let Some(index) = next {
            used[index] = true;
            let p = &pendings[index];
            let edge = body.edges.get(p.edge_id).ok_or_else(|| KernelError::MissingEntity(format!("edge {}", p.edge_id)))?;
            let forward = edge.v0 == cursor;
            chain.push((p.edge_id, forward, Some(p.pcurve_id), p.prange));
            cursor = if forward { edge.v1 } else { edge.v0 };
            next = ends.get(&cursor).into_iter().flatten().copied().find(|&candidate| !used[candidate]);
        }
        chains.push(chain);
    }
    if used.iter().any(|&u| !u) {
        return Err(KernelError::Boolean(BooleanError::ImprintFailed("open imprint segments form a cycle with no free end — a piecewise-closed imprint is not supported".into())));
    }
    Ok(chains)
}

/// 🔀 The pre-existing boundary edge of `face` that the clipped segment `ic[t0, t1]` runs ALONG
/// (every sample within tolerance of that edge's own curve), if there is one. This is the
/// "imprint lands exactly on existing topology" case: the boundary the boolean needs on this side
/// already exists, and minting a second, geometrically identical edge for it would leave the
/// result with duplicate — and therefore non-manifold — topology.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coincident_boundary_edge(body: &Body, face: FaceId, ic: &IntCurve, (t0, t1): (f64, f64), tol: f64) -> Option<EdgeId> {
    const SAMPLES: usize = 8;
    let linear = tol.max(1e-9);
    let samples: Vec<Pnt3> = (0..=SAMPLES).map(|i| ic.curve3.eval(t0 + (t1 - t0) * i as f64 / SAMPLES as f64)).collect();
    for coedge_id in body.face_coedges(face) {
        let Some(coedge) = body.coedges.get(coedge_id) else { continue };
        let Some(edge) = body.edges.get(coedge.edge) else { continue };
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        if samples.iter().all(|&p| closest_parameter(curve, edge.range, p, linear).distance <= linear * 10.0) {
            return Some(coedge.edge);
        }
    }
    None
}

/// 🔀 The vertex of `face`'s outer ring at `position` — an existing ring vertex when one is
/// already there, otherwise the welded imprint vertex for that point, spliced in (which subdivides
/// whichever boundary edge carries it, updating every occurrence of that edge in the ring).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn boundary_vertex_at(body: &mut Body, face: FaceId, position: Pnt3, (tol, weld): (f64, &mut Vec<(Pnt3, VertexId)>), rec: &mut OpRecorder) -> Result<VertexId, KernelError> {
    let outer = body.faces.get(face).and_then(|f| f.outer).ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    let linear = tol.max(1e-9);
    for coedge_id in body.loop_coedges(outer) {
        let Some((start, _)) = body.coedge_endpoints(coedge_id) else { continue };
        if body.vertices.get(start).is_some_and(|v| v.position.distance(position) <= linear) {
            if !weld.iter().any(|&(_, existing)| existing == start) {
                weld.push((position, start));
            }
            return Ok(start);
        }
    }
    let vertex = welded_imprint_vertex(body, weld, position, Tol::new(tol.min(Tol::DEFAULT.value())), tol, rec);
    splice_boundary_vertex(body, outer, vertex, position, tol, rec)?;
    Ok(vertex)
}

/// 🔀 Subdivides `face`'s coincident boundary so exactly one edge spans `endpoints`, and returns
/// it — the shared edge the OTHER operand imprints against. See [`coincident_boundary_edge`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn boundary_subedge(body: &mut Body, face: FaceId, (p0, p1): (Pnt3, Pnt3), (tol, weld): (f64, &mut Vec<(Pnt3, VertexId)>), rec: &mut OpRecorder) -> Result<EdgeId, KernelError> {
    let va = boundary_vertex_at(body, face, p0, (tol, weld), rec)?;
    let vb = boundary_vertex_at(body, face, p1, (tol, weld), rec)?;
    if va == vb {
        return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("coincident imprint on face {face} collapsed to a single boundary vertex"))));
    }
    let outer = body.faces.get(face).and_then(|f| f.outer).ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    for coedge_id in body.loop_coedges(outer) {
        let Some(coedge) = body.coedges.get(coedge_id) else { continue };
        let Some(edge) = body.edges.get(coedge.edge) else { continue };
        if (edge.v0 == va && edge.v1 == vb) || (edge.v0 == vb && edge.v1 == va) {
            return Ok(coedge.edge);
        }
    }
    Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("coincident imprint on face {face} did not resolve to one boundary edge between its endpoints"))))
}

/// 🔀 Orders the shared edge's p-curve range to match the edge's own `v0 → v1` direction, so the
/// linear `prange → edge.range` map the same-parameter check applies stays consistent whether the
/// edge was freshly built (already in that order) or reused from an operand's own boundary
/// (whose direction is whatever the primitive chose).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn oriented_prange(body: &Body, edge_id: EdgeId, (p0, p1): (Pnt3, Pnt3), (t0, t1): (f64, f64)) -> (f64, f64) {
    let Some(edge) = body.edges.get(edge_id) else { return (t0, t1) };
    let Some(start) = body.vertices.get(edge.v0) else { return (t0, t1) };
    if start.position.distance(p1) < start.position.distance(p0) {
        (t1, t0)
    } else {
        (t0, t1)
    }
}

/// 🔀 Applies every pending imprint queued for one original face, tracking the growing set of
/// live pieces so a second (or third) pending curve on the same original face is spliced into
/// whichever current piece its own midpoint actually falls inside. Closed curves are spliced one
/// at a time; open segments are first assembled into chains ([`chain_open_pendings`]) because an
/// individual segment need not reach the face's boundary on its own.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pending_imprints(body: &mut Body, original: FaceId, pending: Vec<Pending>, tol: f64, rec: &mut OpRecorder) -> Result<Vec<FaceId>, KernelError> {
    let mut active = vec![original];
    let (open, closed): (Vec<Pending>, Vec<Pending>) = pending.into_iter().partition(|p| matches!(p.kind, ImprintKind::Open));
    for mut p in closed {
        let Some(idx) = locate_active_piece(body, &active, p.pcurve_id, p.prange, tol) else {
            return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("imprint segment midpoint not found inside any active piece of face {original}"))));
        };
        p.pcurve_id = align_pcurve_branch(body, active[idx], &[(p.pcurve_id, p.prange)]).first().copied().unwrap_or(p.pcurve_id);
        let target = active[idx];
        let (fa, fb) = match p.kind {
            ImprintKind::Interior => split_face_by_interior_curve(body, target, p.edge_id, p.pcurve_id, p.prange, rec)?,
            ImprintKind::SeamCrossing => split_face_by_seam_crossing(body, target, p.edge_id, p.pcurve_id, p.prange, tol, rec)?,
            ImprintKind::Open => unreachable!("open pendings were partitioned out"),
        };
        active[idx] = fa;
        active.push(fb);
    }
    for chain in chain_open_pendings(body, &open)? {
        let probe = chain[chain.len() / 2];
        let Some(pcurve_id) = probe.2 else {
            return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("imprint chain member on face {original} carries no p-curve"))));
        };
        let Some(idx) = locate_active_piece(body, &active, pcurve_id, probe.3, tol) else {
            return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("imprint segment midpoint not found inside any active piece of face {original}"))));
        };
        let target = active[idx];
        let (fa, fb) = split_face_by_chain(body, target, &chain, tol, rec)?;
        active[idx] = fa;
        active.push(fb);
    }
    Ok(active)
}

/// 🔀 Re-expresses the imprint p-curves `carried` in the same periodic BRANCH as `face`'s own
/// boundary ring, returning one (possibly new) curve id per input.
///
/// A p-curve on a periodic surface is only pinned down modulo whole periods, and nothing forces the
/// branch a surface inversion returns to match the branch the face's ring was written in. Imprint
/// one into the other unchanged and the resulting face's UV polygon has its pieces sitting `2π`
/// apart — a shape with no interior at all, which `interior_point_of_face` then correctly (and
/// unhelpfully) refuses to sample. Found live on `🍩️sphere-cut-with-torus`, whose second
/// intersection circle arrived at `u ∈ [2π, 4π]`, `v = −1.271` against a ring written on
/// `u ∈ [0, 2π]`, `v ∈ [0, 2π]`.
///
/// One shift for the whole group, chosen by putting the group's own UV centroid nearest the ring's,
/// and only along directions the surface is actually periodic in. Applied to the CLOSED imprints
/// (one whole curve, one branch); an open chain's members can legitimately have been born in
/// different branches from each other, so those are aligned per traversal at split time instead
/// (`euler::chain_endpoint_uv`) rather than dragged onto a common one here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn align_pcurve_branch(body: &mut Body, face: FaceId, carried: &[(Curve2Id, (f64, f64))]) -> Vec<Curve2Id> {
    let existing: Vec<Curve2Id> = carried.iter().map(|&(id, _)| id).collect();
    let Some(surface) = body.faces.get(face).and_then(|f| body.surfaces.get(f.surface)) else { return existing };
    let (u_periodic, v_periodic) = (surface.is_u_periodic(), surface.is_v_periodic());
    if !(u_periodic || v_periodic) {
        return existing;
    }
    let sample = |body: &Body, id: Curve2Id, range: (f64, f64)| -> Vec<Pnt2> {
        body.curves2.get(id).map_or_else(Vec::new, |pc| (0..=8).map(|i| pc.eval(range.0 + (range.1 - range.0) * f64::from(i) / 8.0)).collect())
    };
    let mut group: Vec<Pnt2> = Vec::new();
    for &(id, range) in carried {
        group.extend(sample(body, id, range));
    }
    let mut ring: Vec<Pnt2> = Vec::new();
    for coedge_id in body.face_coedges(face) {
        let Some(coedge) = body.coedges.get(coedge_id).cloned() else { continue };
        let Some(pcurve) = coedge.pcurve else { continue };
        ring.extend(sample(body, pcurve, coedge.prange));
    }
    if group.is_empty() || ring.is_empty() {
        return existing;
    }
    let centroid = |points: &[Pnt2]| Pnt2::new(points.iter().map(|p| p.x).sum::<f64>() / points.len() as f64, points.iter().map(|p| p.y).sum::<f64>() / points.len() as f64);
    let (group_center, ring_center) = (centroid(&group), centroid(&ring));
    let tau = std::f64::consts::TAU;
    let du = if u_periodic { ((ring_center.x - group_center.x) / tau).round() * tau } else { 0.0 };
    let dv = if v_periodic { ((ring_center.y - group_center.y) / tau).round() * tau } else { 0.0 };
    if du == 0.0 && dv == 0.0 {
        return existing;
    }
    let delta = Vec2::new(du, dv);
    existing.into_iter().map(|id| body.curves2.get(id).map(|pc| pc.translated(delta)).map_or(id, |shifted| body.curves2.insert(shifted))).collect()
}

// #endregion 🔖️Imprint

// #region 🔖️Classify

/// 🔀 A UV point inside `face`'s own trim (outer minus every hole), for use as a representative
/// 3D sample when classifying the whole piece: the outer loop's sampled centroid first, then a
/// coarse grid scan of the loop's UV bounding box.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn interior_point_of_face(body: &Body, face: FaceId, tol: f64) -> Option<Pnt3> {
    let face_data = body.faces.get(face)?;
    let surface = body.surfaces.get(face_data.surface)?.clone();
    let outer = face_data.outer?;
    let pts = sample_loop_uv(body, outer, &surface, 12);
    if pts.is_empty() {
        return None;
    }
    let cx = pts.iter().map(|p| p.x).sum::<f64>() / pts.len() as f64;
    let cy = pts.iter().map(|p| p.y).sum::<f64>() / pts.len() as f64;
    let (mut umin, mut umax, mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    for p in &pts {
        umin = umin.min(p.x);
        umax = umax.max(p.x);
        vmin = vmin.min(p.y);
        vmax = vmax.max(p.y);
    }
    if point_in_face_uv_periodic(body, face, Pnt2::new(cx, cy), tol) {
        return Some(surface.eval(cx, cy));
    }
    const STEPS: usize = 16;
    for i in 0..STEPS {
        for j in 0..STEPS {
            let u = umin + (umax - umin) * (i as f64 + 0.5) / STEPS as f64;
            let v = vmin + (vmax - vmin) * (j as f64 + 0.5) / STEPS as f64;
            if point_in_face_uv_periodic(body, face, Pnt2::new(u, v), tol) {
                return Some(surface.eval(u, v));
            }
        }
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_loop_uv(body: &Body, loop_id: LoopId, surface: &Surface, per_edge: usize) -> Vec<Pnt2> {
    let mut pts = Vec::new();
    let mut prev_u: Option<f64> = None;
    let mut prev_v: Option<f64> = None;
    for cid in body.loop_coedges(loop_id) {
        let Some(co) = body.coedges.get(cid) else { continue };
        let Some(pc_id) = co.pcurve else { continue };
        let Some(pc) = body.curves2.get(pc_id) else { continue };
        let (t0, t1) = if co.forward { co.prange } else { (co.prange.1, co.prange.0) };
        for k in 0..per_edge {
            let s = k as f64 / per_edge as f64;
            let mut p = pc.eval(t0 + (t1 - t0) * s);
            // Unwrap continuously across coedges (matching `classification::loop_uv_polygon_sampled`'s
            // own periodic handling) so a loop that grazes a periodic seam still yields a compact,
            // consistently-signed UV bounding box instead of a bogus near-full-domain one.
            if surface.is_u_periodic() {
                if let Some(pu) = prev_u {
                    p.x = unwrap_to(pu, p.x);
                }
                prev_u = Some(p.x);
            }
            if surface.is_v_periodic() {
                if let Some(pv) = prev_v {
                    p.y = unwrap_to(pv, p.y);
                }
                prev_v = Some(p.y);
            }
            pts.push(p);
        }
    }
    pts
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_to(prev: f64, u: f64) -> f64 {
    let tau = std::f64::consts::TAU;
    let diff = u - prev;
    prev + diff - tau * ((diff + std::f64::consts::PI) / tau).floor()
}

/// 🔀 Six fixed irrational-ish ray directions for [`local_point_in_solid`]'s retry consensus —
/// same role as `classification.rs`'s own private `RAY_RETRY_DIRS` (kept independent since that
/// one isn't `pub`).
/// 🐛 All six used to have a POSITIVE `z`, so every retry left the query point through the same
/// hemisphere: one damaged face in that hemisphere loses every vote at once, and the classifier
/// reports `Outside` unanimously rather than disagreeing. Half of them are negated here so the set
/// spans both hemispheres and no single face can shadow the whole consensus.
const LOCAL_RAY_RETRY_DIRS: [[f64; 3]; 6] = [
    [0.573_576_436_351_046, 0.740_535_693_464_567_5, 0.350_889_803_483_932_2],
    [0.350_889_803_483_932_2, -0.573_576_436_351_046, -0.740_535_693_464_567_5],
    [0.267_261_241_941_149_4, 0.534_522_483_882_298_8, 0.801_783_725_737_219],
    [-0.577_350_269_189_625_8, -0.577_350_269_189_625_8, -0.577_350_269_189_625_7],
    [0.308_608_313_448_298, 0.904_511_432_523_735, 0.293_892_626_045_885],
    [-0.843_391_445_261_857, -0.214_298_755_144_806, -0.491_975_172_042_98],
];

/// 🔀 Local replacement for `classification::point_in_solid`, needed to work around a confirmed
/// bug there rather than editing that file (outside this file's ownership per the ticket brief —
/// see the report for the exact diff `classification.rs` would need instead).
/// `point_in_face_trim_status`'s `Surface::Sphere` branch builds its own trim "polygon" from the
/// face's real 3D VERTICES (`face_boundary_points`) — correct for the pre-this-ticket
/// two-hemisphere sphere topology, but wrong for W1-E's new single-face/one-seam sphere (only 2
/// real vertices: the two poles), where that "polygon" degenerates to two collinear points, so
/// `point_in_polygon_3d` returns `false` for every hit and EVERY ray reports zero crossings.
/// Confirmed directly (not assumed): `point_in_solid` on a bare, unsplit unit sphere returns
/// `Outside` for `(0,0,0)` (its own center) and for `(2,0,0)` (clearly outside) identically. This
/// reimplements the same multi-ray-parity vote, but routes every face's trim test through
/// `classification::point_in_face_uv` (the UV-sampled path, unaffected by the bug above) using the
/// `(u, v)` [`intersect_curve_surface`] already returns per hit, instead of the broken 3D-vertex
/// shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn local_point_in_solid(body: &Body, solid: SolidId, point: Pnt3, tol: f64) -> Result<PointClassification, KernelError> {
    let (_, dist) = closest_point_on_solid(body, solid, point)?;
    if dist <= tol {
        return Ok(PointClassification::OnBoundary);
    }
    let faces = body.solid_faces(solid);
    let mut inside_votes = 0u32;
    let mut outside_votes = 0u32;
    for raw_dir in LOCAL_RAY_RETRY_DIRS {
        let dir = Vec3::new(raw_dir[0], raw_dir[1], raw_dir[2]);
        let ray = crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Line { origin: point, dir };
        let mut grazing = false;
        let mut hits: Vec<f64> = Vec::new();
        for &face in &faces {
            let Some(face_data) = body.faces.get(face) else { continue };
            let Some(surface) = body.surfaces.get(face_data.surface) else { continue };
            let face_hits = match intersect_curve_surface(&ray, surface, tol) {
                Ok(h) => h,
                Err(_) => {
                    grazing = true;
                    break;
                }
            };
            for h in face_hits {
                if h.t <= 1e-9 {
                    continue;
                }
                if let Some(n) = surface.normal(h.u, h.v) {
                    if let Some(nn) = n.normalized() {
                        if nn.dot(dir).abs() < 1e-6 {
                            grazing = true;
                            break;
                        }
                    }
                }
                if point_in_face_uv(body, face, wrap_uv_for_surface(body, face, Pnt2::new(h.u, h.v)), tol).unwrap_or(false) {
                    hits.push(h.t);
                }
            }
            if grazing {
                break;
            }
        }
        if grazing {
            continue;
        }
        hits.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let merge_tol = tol * 10.0;
        let mut crossings = 0u32;
        let mut last: Option<f64> = None;
        for t in hits {
            if last.is_none_or(|l| (t - l).abs() > merge_tol) {
                crossings += 1;
                last = Some(t);
            }
        }
        if crossings % 2 == 1 {
            inside_votes += 1;
        } else {
            outside_votes += 1;
        }
        if inside_votes >= 2 {
            return Ok(PointClassification::Inside);
        }
        if outside_votes >= 2 {
            return Ok(PointClassification::Outside);
        }
    }
    if inside_votes > outside_votes {
        Ok(PointClassification::Inside)
    } else if outside_votes > inside_votes {
        Ok(PointClassification::Outside)
    } else {
        Err(KernelError::Operation("local point classification: every retry direction was grazing or degenerate".into()))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn classify_face_against_solid(body: &Body, face: FaceId, other: SolidId, tol: f64) -> Result<PointClassification, KernelError> {
    let Some(p) = interior_point_of_face(body, face, tol) else {
        return Err(KernelError::Boolean(BooleanError::ClassificationAmbiguous(format!("no interior UV sample found for face {face}"))));
    };
    local_point_in_solid(body, other, p, tol)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn keep_face(op: BooleanOp, from_a: bool, class: PointClassification) -> bool {
    match op {
        BooleanOp::Unite => matches!(class, PointClassification::Outside | PointClassification::OnBoundary),
        BooleanOp::Intersect => matches!(class, PointClassification::Inside | PointClassification::OnBoundary),
        BooleanOp::Cut => {
            if from_a {
                matches!(class, PointClassification::Outside | PointClassification::OnBoundary)
            } else {
                matches!(class, PointClassification::Inside)
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn flip_face(body: &mut Body, face: FaceId) {
    if let Some(f) = body.faces.get_mut(face) {
        f.flipped = !f.flipped;
    }
}

// #endregion 🔖️Classify

// #region 🔖️Coincident

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_coincident_face_pairs(body: &Body, faces_a: &[FaceId], faces_b: &[FaceId], tol: f64) -> Vec<(FaceId, FaceId)> {
    let mut out = Vec::new();
    for &fa in faces_a {
        let Some(face_a) = body.faces.get(fa) else { continue };
        let Some(outer_a) = face_a.outer else { continue };
        let Some(sa) = body.surfaces.get(face_a.surface) else { continue };
        for &fb in faces_b {
            let Some(face_b) = body.faces.get(fb) else { continue };
            let Some(outer_b) = face_b.outer else { continue };
            let Some(sb) = body.surfaces.get(face_b.surface) else { continue };
            if surfaces_equal(sa, sb, tol) && loops_coincide(body, outer_a, outer_b, tol) {
                out.push((fa, fb));
            }
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surfaces_equal(sa: &Surface, sb: &Surface, tol: f64) -> bool {
    match (sa, sb) {
        (Surface::Plane { frame: fa }, Surface::Plane { frame: fb }) => fa.z.dot(fb.z).abs() > 1.0 - 1e-9 && (fa.origin - fb.origin).dot(fa.z).abs() < tol,
        (Surface::Cylinder { frame: fa, radius: ra }, Surface::Cylinder { frame: fb, radius: rb }) => (ra - rb).abs() < tol && fa.z.cross(fb.z).norm() < 1e-9 && (fa.origin - fb.origin).cross(fa.z).norm() < tol,
        (Surface::Sphere { frame: fa, radius: ra }, Surface::Sphere { frame: fb, radius: rb }) => (ra - rb).abs() < tol && (fa.origin - fb.origin).norm() < tol,
        _ => false,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_positions(body: &Body, loop_id: LoopId) -> Vec<Pnt3> {
    body.loop_coedges(loop_id).into_iter().filter_map(|c| body.coedge_endpoints(c)).filter_map(|(v, _)| body.vertices.get(v).map(|x| x.position)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loops_coincide(body: &Body, la: LoopId, lb: LoopId, tol: f64) -> bool {
    let pa = loop_positions(body, la);
    let pb = loop_positions(body, lb);
    if pa.is_empty() || pa.len() != pb.len() {
        return false;
    }
    pa.iter().all(|p| pb.iter().any(|q| p.distance(*q) <= tol))
}

// #endregion 🔖️Coincident

// #region 🔖️Stitch

/// 🔀 Groups `selected` faces into connected shells via shared edges, orients every group outward
/// (flipping the whole group when its net signed volume comes out negative), then assembles ONE
/// solid out of all of them: a group that a containment test ([`solid_wholly_inside`]) places
/// inside another becomes an inverted VOID shell, and every remaining group joins the outer shell.
/// 🐛 This used to return only the largest-volume group and leave the rest live-but-unreturned,
/// which silently dropped every other lump: two barely-tangent spheres united to the volume of ONE
/// sphere. Disjoint lumps sharing one shell is the same representation
/// [`trivial_topology_fast_path`]'s own disjoint-union branch already produces.
/// 🐛 The sign probe's own `chord_tol` used to be `1e-6`: for a full-circle imprint edge that made
/// `segments_for_chord_deviation` sample ~1700+ boundary points, and `ear_clip`'s O(n³) worst case
/// over that many points took minutes. Only the SIGN of this volume is used here (to decide
/// whether to flip a group's faces), so it now matches `validation_report`'s own sign-probe
/// tolerance (`PROBE_TOL = 1e-3`) instead of a precision this call never needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stitch_selected_faces(body: &mut Body, selected: &[FaceId], tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let groups = group_shells(body, selected);
    if groups.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult("stitch produced no shells".into())));
    }
    let mut lumps: Vec<(Vec<FaceId>, ShellId, SolidId)> = Vec::with_capacity(groups.len());
    for group in groups {
        let shell = add_shell(body, group.clone(), rec);
        if shell_signed_volume(body, shell, 1e-3).unwrap_or(0.0) < 0.0 {
            for &f in &group {
                flip_face(body, f);
            }
        }
        let solid = add_solid(body, shell, Vec::new(), rec);
        lumps.push((group, shell, solid));
    }
    let mut void_of: Vec<Option<usize>> = vec![None; lumps.len()];
    for inner in 0..lumps.len() {
        for outer in 0..lumps.len() {
            if inner == outer || void_of[outer] == Some(inner) {
                continue;
            }
            if solid_wholly_inside(body, lumps[inner].2, lumps[outer].2, tol)? {
                void_of[inner] = Some(outer);
                break;
            }
        }
    }
    let mut outer_faces: Vec<FaceId> = Vec::new();
    let mut void_shells: Vec<ShellId> = Vec::new();
    for (index, (group, shell, _)) in lumps.iter().enumerate() {
        if void_of[index].is_some() {
            for &f in group {
                flip_face(body, f);
            }
            void_shells.push(*shell);
        } else {
            outer_faces.extend(group.iter().copied());
        }
    }
    for (index, (_, shell, solid)) in lumps.iter().enumerate() {
        if let Some(label) = body.solids.get(*solid).map(|s| s.label) {
            rec.record_deleted(label);
        }
        body.solids.remove(*solid);
        if void_of[index].is_none() {
            if let Some(label) = body.shells.get(*shell).map(|s| s.label) {
                rec.record_deleted(label);
            }
            body.shells.remove(*shell);
        }
    }
    if outer_faces.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult("stitch found only enclosed shells and no outer boundary".into())));
    }
    let outer = add_shell(body, outer_faces, rec);
    Ok(add_solid(body, outer, void_shells, rec))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn group_shells(body: &Body, selected: &[FaceId]) -> Vec<Vec<FaceId>> {
    let mut edge_to_faces: HashMap<EdgeId, Vec<FaceId>> = HashMap::new();
    for &f in selected {
        for cid in body.face_coedges(f) {
            if let Some(co) = body.coedges.get(cid) {
                edge_to_faces.entry(co.edge).or_default().push(f);
            }
        }
    }
    let mut adjacency: HashMap<FaceId, HashSet<FaceId>> = HashMap::new();
    for &f in selected {
        adjacency.entry(f).or_default();
    }
    for faces in edge_to_faces.values() {
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                adjacency.entry(faces[i]).or_default().insert(faces[j]);
                adjacency.entry(faces[j]).or_default().insert(faces[i]);
            }
        }
    }
    let mut visited: HashSet<FaceId> = HashSet::new();
    let mut groups = Vec::new();
    for &f in selected {
        if visited.contains(&f) {
            continue;
        }
        let mut stack = vec![f];
        visited.insert(f);
        let mut comp = Vec::new();
        while let Some(cur) = stack.pop() {
            comp.push(cur);
            if let Some(neighbors) = adjacency.get(&cur) {
                for &n in neighbors {
                    if visited.insert(n) {
                        stack.push(n);
                    }
                }
            }
        }
        groups.push(comp);
    }
    groups
}

// #endregion 🔖️Stitch

// #region 🔖️Cleanup

/// 🔀 Removes every face of `solid`'s original shells that is NOT in `keep` (a face that WAS kept
/// survives as a floating face until [`stitch_selected_faces`]'s new shell picks it back up), then
/// drops the now-superseded shell(s)/solid wrapper itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remove_solid_and_orphans(body: &mut Body, solid: SolidId, keep: &HashSet<FaceId>, rec: &mut OpRecorder) {
    let Some(data) = body.solids.get(solid).cloned() else { return };
    let shells: Vec<ShellId> = std::iter::once(data.outer).chain(data.inners.iter().copied()).collect();
    for shell_id in shells {
        let Some(shell) = body.shells.get(shell_id).cloned() else { continue };
        for f in shell.faces {
            if !keep.contains(&f) {
                remove_face(body, f, rec);
            }
        }
        if let Some(label) = body.shells.get(shell_id).map(|s| s.label) {
            rec.record_deleted(label);
        }
        body.shells.remove(shell_id);
    }
    if let Some(label) = body.solids.get(solid).map(|s| s.label) {
        rec.record_deleted(label);
    }
    body.solids.remove(solid);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remove_face(body: &mut Body, face: FaceId, rec: &mut OpRecorder) {
    let Some(data) = body.faces.get(face).cloned() else { return };
    let mut loops = Vec::new();
    if let Some(o) = data.outer {
        loops.push(o);
    }
    loops.extend(data.inners.iter().copied());
    for loop_id in loops {
        for cid in body.loop_coedges(loop_id) {
            body.coedges.remove(cid);
        }
        body.loops.remove(loop_id);
    }
    rec.record_deleted(data.label);
    body.faces.remove(face);
}

/// 🔀 Drops every edge/vertex no longer referenced by any live coedge/edge — the imprint pipeline
/// creates edges/vertices speculatively (both faces of a pair queue the same shared edge even when
/// only one side ends up selected) and `remove_face` only clears coedges, not the shared geometry
/// underneath them.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gc_orphan_edges_and_vertices(body: &mut Body, rec: &mut OpRecorder) {
    let used_edges: HashSet<EdgeId> = body.coedges.iter().map(|(_, c)| c.edge).collect();
    let dead_edges: Vec<EdgeId> = body.edges.iter().map(|(id, _)| id).filter(|id| !used_edges.contains(id)).collect();
    for e in dead_edges {
        if let Some(data) = body.edges.get(e) {
            rec.record_deleted(data.label);
        }
        body.edges.remove(e);
    }
    let mut used_verts: HashSet<VertexId> = HashSet::new();
    for (_, e) in body.edges.iter() {
        used_verts.insert(e.v0);
        used_verts.insert(e.v1);
    }
    let dead_verts: Vec<VertexId> = body.vertices.iter().map(|(id, _)| id).filter(|id| !used_verts.contains(id)).collect();
    for v in dead_verts {
        if let Some(data) = body.vertices.get(v) {
            rec.record_deleted(data.label);
        }
        body.vertices.remove(v);
    }
}

// #endregion 🔖️Cleanup

// #endregion 🔖️ExactImprintEngine

// #region 🔖️MeshPreview

/// 🔀 The pre-rewrite tessellate→centroid-classify→triangle-soup boolean, kept as an explicit
/// opt-in for callers that pass `OpQuality::MeshDerivedBRep` — `boolean_solid` itself never calls
/// this.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boolean_solid_mesh_preview(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_tol(tol)?;
    require_solid(body, a)?;
    require_solid(body, b)?;
    let deflection = tol.max(1e-3);
    let mesh_a = tessellate_solid(body, a, deflection)?;
    let mesh_b = tessellate_solid(body, b, deflection)?;
    let mut points = Vec::new();
    let mut triangles: Vec<[Pnt3; 3]> = Vec::new();
    append_kept_triangles(body, &mesh_a, b, op, true, tol, (&mut points, &mut triangles))?;
    append_kept_triangles(body, &mesh_b, a, op, false, tol, (&mut points, &mut triangles))?;
    if triangles.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult("mesh boolean produced no triangles".into())));
    }
    match solid_from_triangle_soup(body, &triangles, rec) {
        Ok(id) => Ok(id),
        Err(_) => make_convex_hull(body, &points, rec).map_err(|e| match e {
            KernelError::InvalidInput(msg) => KernelError::Boolean(BooleanError::InvalidResult(msg)),
            other => other,
        }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn append_kept_triangles(body: &Body, mesh: &MeshTransfer, other: SolidId, op: BooleanOp, from_a: bool, tol: f64, (out_points, out_tris): (&mut Vec<Pnt3>, &mut Vec<[Pnt3; 3]>)) -> Result<(), KernelError> {
    let npos = mesh.position.len() / 3;
    if !mesh.index.len().is_multiple_of(3) {
        return Err(KernelError::InvalidInput("mesh index length must be a multiple of 3".into()));
    }
    for tri in mesh.index.as_chunks::<3>().0 {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= npos || i1 >= npos || i2 >= npos {
            return Err(KernelError::InvalidInput("mesh index out of range".into()));
        }
        let p0 = mesh_position(mesh, i0);
        let p1 = mesh_position(mesh, i1);
        let p2 = mesh_position(mesh, i2);
        let centroid = Pnt3::new((p0.x + p1.x + p2.x) / 3.0, (p0.y + p1.y + p2.y) / 3.0, (p0.z + p1.z + p2.z) / 3.0);
        let class = local_point_in_solid(body, other, centroid, tol)?;
        if keep_face(op, from_a, class) {
            out_points.push(p0);
            out_points.push(p1);
            out_points.push(p2);
            out_tris.push([p0, p1, p2]);
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mesh_position(mesh: &MeshTransfer, i: usize) -> Pnt3 {
    let o = i * 3;
    Pnt3::new(mesh.position[o] as f64, mesh.position[o + 1] as f64, mesh.position[o + 2] as f64)
}

// #endregion 🔖️MeshPreview

// #region 🔖️ShellHelpers

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_from_outer_faces(body: &mut Body, outer_faces: Vec<FaceId>, inner_face_sets: Vec<Vec<FaceId>>, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if outer_faces.is_empty() {
        return Err(KernelError::InvalidInput("outer shell requires at least one face".into()));
    }
    let outer = add_shell(body, outer_faces, rec);
    let mut inners = Vec::with_capacity(inner_face_sets.len());
    for faces in inner_face_sets {
        if faces.is_empty() {
            return Err(KernelError::InvalidInput("inner shell requires at least one face".into()));
        }
        inners.push(add_shell(body, faces, rec));
    }
    Ok(add_solid(body, outer, inners, rec))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn clone_solid_shells(body: &mut Body, solid: SolidId, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let data = body.solids.get(solid).ok_or_else(|| KernelError::MissingEntity(format!("solid {solid}")))?.clone();
    let outer = outer_faces(body, solid)?;
    let mut inners = Vec::new();
    for shell_id in data.inners {
        let faces = body.shells.get(shell_id).ok_or_else(|| KernelError::MissingEntity(format!("shell {shell_id}")))?.faces.clone();
        inners.push(faces);
    }
    solid_from_outer_faces(body, outer, inners, rec)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn outer_faces(body: &Body, solid: SolidId) -> Result<Vec<FaceId>, KernelError> {
    let data = body.solids.get(solid).ok_or_else(|| KernelError::MissingEntity(format!("solid {solid}")))?;
    let shell = body.shells.get(data.outer).ok_or_else(|| KernelError::MissingEntity(format!("shell {}", data.outer)))?;
    Ok(shell.faces.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_vertex_positions(body: &Body, solid: SolidId) -> Vec<Pnt3> {
    let mut seen: HashSet<VertexId> = HashSet::new();
    let mut points = Vec::new();
    for face in body.solid_faces(solid) {
        let Some(face_ent) = body.faces.get(face) else {
            continue;
        };
        let mut loops = Vec::new();
        if let Some(outer) = face_ent.outer {
            loops.push(outer);
        }
        loops.extend(face_ent.inners.iter().copied());
        for loop_id in loops {
            let Some(loop_ent) = body.loops.get(loop_id) else {
                continue;
            };
            let start = loop_ent.first;
            let mut cur = start;
            loop {
                if let Some((v0, _)) = body.coedge_endpoints(cur) {
                    if seen.insert(v0) {
                        if let Some(v) = body.vertices.get(v0) {
                            points.push(v.position);
                        }
                    }
                }
                let Some(coedge) = body.coedges.get(cur) else {
                    break;
                };
                cur = coedge.next;
                if cur == start {
                    break;
                }
            }
        }
    }
    points
}

// #endregion 🔖️ShellHelpers

// #region 🔖️AabbMath

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_finite(bb: &AxisAlignedBox) -> bool {
    bb.min.x.is_finite() && bb.min.y.is_finite() && bb.min.z.is_finite() && bb.max.x.is_finite() && bb.max.y.is_finite() && bb.max.z.is_finite()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_dims(bb: &AxisAlignedBox) -> (f64, f64, f64) {
    (bb.max.x - bb.min.x, bb.max.y - bb.min.y, bb.max.z - bb.min.z)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_volume(bb: &AxisAlignedBox) -> f64 {
    let (w, d, h) = aabb_dims(bb);
    w * d * h
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_gap(a: &AxisAlignedBox, b: &AxisAlignedBox) -> f64 {
    let dx = gap_1d(a.min.x, a.max.x, b.min.x, b.max.x);
    let dy = gap_1d(a.min.y, a.max.y, b.min.y, b.max.y);
    let dz = gap_1d(a.min.z, a.max.z, b.min.z, b.max.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gap_1d(a0: f64, a1: f64, b0: f64, b1: f64) -> f64 {
    if a1 < b0 {
        b0 - a1
    } else if b1 < a0 {
        a0 - b1
    } else {
        0.0
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_intersection(a: &AxisAlignedBox, b: &AxisAlignedBox) -> Option<AxisAlignedBox> {
    let min = Pnt3::new(a.min.x.max(b.min.x), a.min.y.max(b.min.y), a.min.z.max(b.min.z));
    let max = Pnt3::new(a.max.x.min(b.max.x), a.max.y.min(b.max.y), a.max.z.min(b.max.z));
    if min.x < max.x && min.y < max.y && min.z < max.z {
        Some(AxisAlignedBox { min, max })
    } else {
        None
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_union(a: &AxisAlignedBox, b: &AxisAlignedBox) -> AxisAlignedBox {
    AxisAlignedBox { min: Pnt3::new(a.min.x.min(b.min.x), a.min.y.min(b.min.y), a.min.z.min(b.min.z)), max: Pnt3::new(a.max.x.max(b.max.x), a.max.y.max(b.max.y), a.max.z.max(b.max.z)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_contains(outer: &AxisAlignedBox, inner: &AxisAlignedBox, tol: f64) -> bool {
    outer.min.x <= inner.min.x + tol && outer.min.y <= inner.min.y + tol && outer.min.z <= inner.min.z + tol && outer.max.x + tol >= inner.max.x && outer.max.y + tol >= inner.max.y && outer.max.z + tol >= inner.max.z
}

/// 🔀 Overlap test for the small `engine::Aabb` shape ([`face_aabb`]'s return type) rather than
/// [`AxisAlignedBox`] — inflated by `tol` on every side so a face pair whose supports are exactly
/// tangent still gets a chance to intersect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_overlap(a: &crate::standards::v1::subsets::brep::schema::engine::Aabb, b: &crate::standards::v1::subsets::brep::schema::engine::Aabb, tol: f64) -> bool {
    (0..3).all(|i| a.min[i] - tol <= b.max[i] + tol && b.min[i] - tol <= a.max[i] + tol)
}

// #endregion 🔖️AabbMath

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
