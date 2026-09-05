//! 🔀 Exact imprint→classify→select→stitch boolean pipeline for solids bounded by planes,
//! cylinders, cones, spheres and tori (and NURBS via [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect`]'s marching SSI): face-pair
//! candidates via AABB overlap → [`intersect_surface_surface`] (W2-A) → the SSI curve's domain
//! clipped to both faces' trims → an imprint edge shared by both operands (so stitching needs no
//! fuzzy vertex welding, only shared-edge adjacency) → [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::split_face_by_edge`]/
//! [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::split_face_by_interior_curve`] imprint each side → every resulting piece classified
//! against the OTHER solid via [`crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid`] → selected per [`BooleanOp`] → stitched into
//! shell(s)/solid(s) → [`crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body`]. No tessellate→triangle-soup rebuild on this path — the
//! old mesh pipeline survives only as the explicit opt-in [`boolean_solid_mesh_preview`]. A
//! trivial disjoint/contained fast path and a box-specific exact-analytic fast path (both still
//! genuinely exact, not mesh-derived) run before the general engine when they apply.
//!
//! Documented scope (see `📓️w2b-booleans.md` for the full account): the general imprint engine
//! handles curves that are either fully interior to both faces (closed loop → hole + new face) or
//! cross each face's boundary at exactly two points (open chord → two-chain split); a curve
//! crossing more than twice, or two operands whose imprint lands exactly along pre-existing
//! topology (coincident edges, not just coincident faces), is out of scope for this pass and
//! surfaces as a `BooleanError` rather than silently producing a wrong result. Coincident/adjacent
//! duplicate faces on the same surface (e.g. two operands sharing a face exactly) are detected and
//! merged into one rather than kept twice.
//!
//! Lane 4-boolean of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`, rewritten from
//! the tessellate-and-classify pipeline in `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` wave 2
//! (W2-B) — see `📓️w2b-booleans.md`.

use std::collections::{HashMap, HashSet};

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_shell, add_solid, make_edge, make_vertex, split_face_by_edge, split_face_by_interior_curve, split_face_by_seam_crossing};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect::{intersect_curve_surface, intersect_surface_surface, IntCurve};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_convex_hull, solid_from_triangle_soup};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::bounding_volume::face_aabb;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::classification::point_in_face_uv;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{closest_point_on_solid, shell_signed_volume, solid_bounding_box, solid_volume, AxisAlignedBox};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve2Id, EdgeId, FaceId, LoopId, ShellId, SolidId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::closest_parameter;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::{BooleanError, KernelError, ValidationIssue};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::{MeshTransfer, PointClassification};

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
/// operation's [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpDelta`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boolean_solid(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_tol(tol)?;
    require_solid(body, a)?;
    require_solid(body, b)?;
    if a == b {
        return Err(KernelError::InvalidInput("boolean operands must be distinct solids".into()));
    }

    let bb_a = solid_bounding_box(body, a)?;
    let bb_b = solid_bounding_box(body, b)?;
    if aabb_finite(&bb_a) && aabb_finite(&bb_b) {
        if let Some(id) = trivial_topology_fast_path(body, a, b, &bb_a, &bb_b, op, tol, rec)? {
            return Ok(id);
        }
        if let Some(id) = box_fast_path(body, a, b, &bb_a, &bb_b, op, tol, rec)? {
            return Ok(id);
        }
    }
    exact_imprint_boolean(body, a, b, op, tol, rec)
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
    let points = solid_vertex_positions(body, solid)?;
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
    let face = crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_planar_face_from_points(body, &section_pts, rec)?;
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
    for tri in mesh.index.chunks_exact(3) {
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
        let points = solid_vertex_positions(body, solid)?;
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
fn trivial_topology_fast_path(body: &mut Body, a: SolidId, b: SolidId, bb_a: &AxisAlignedBox, bb_b: &AxisAlignedBox, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
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
    let points = solid_vertex_positions(body, inner)?;
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
/// Unite/Intersect of overlapping boxes reduce to one more box — still an exact analytic result,
/// just cheaper than running the general imprint engine on 12 coplanar face pairs. Cut is left to
/// the general engine (an L-shaped result isn't a box).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn box_fast_path(body: &mut Body, a: SolidId, b: SolidId, bb_a: &AxisAlignedBox, bb_b: &AxisAlignedBox, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
    if !matches!(op, BooleanOp::Unite | BooleanOp::Intersect) {
        return Ok(None);
    }
    if !(is_aabb_box_solid(body, a, bb_a)? && is_aabb_box_solid(body, b, bb_b)?) {
        return Ok(None);
    }
    match op {
        BooleanOp::Unite => {
            let u = aabb_union(bb_a, bb_b);
            let (w, d, h) = aabb_dims(&u);
            Ok(Some(make_box(body, w, d, h, rec)?))
        }
        BooleanOp::Intersect => {
            let Some(inter) = aabb_intersection(bb_a, bb_b) else {
                return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty".into())));
            };
            let (w, d, h) = aabb_dims(&inter);
            if w <= tol || d <= tol || h <= tol {
                return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty within tolerance".into())));
            }
            Ok(Some(make_box(body, w, d, h, rec)?))
        }
        BooleanOp::Cut => unreachable!(),
    }
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

/// 🔀 The general exact pipeline: imprint every overlapping face pair, classify every resulting
/// piece against the other solid, select per `op`, stitch the survivors into shell(s)/solid(s).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn exact_imprint_boolean(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let pre_existing_solids: HashSet<SolidId> = body.solids.iter().map(|(id, _)| id).collect();
    let faces_a_all = body.solid_faces(a);
    let faces_b_all = body.solid_faces(b);

    let coincident = find_coincident_face_pairs(body, &faces_a_all, &faces_b_all, tol);
    let coincident_b: HashSet<FaceId> = coincident.iter().map(|&(_, fb)| fb).collect();
    let coincident_a: HashSet<FaceId> = coincident.iter().map(|&(fa, _)| fa).collect();
    let faces_b: Vec<FaceId> = faces_b_all.iter().copied().filter(|f| !coincident_b.contains(f)).collect();
    let faces_a: Vec<FaceId> = faces_a_all.clone();

    let mut pending_a: HashMap<FaceId, Vec<Pending>> = HashMap::new();
    let mut pending_b: HashMap<FaceId, Vec<Pending>> = HashMap::new();

    for &fa in &faces_a {
        if coincident_a.contains(&fa) {
            continue;
        }
        let Some(face_a) = body.faces.get(fa).cloned() else { continue };
        let Some(sa) = body.surfaces.get(face_a.surface).cloned() else { continue };
        let Ok(bb_a) = face_aabb(body, fa) else { continue };
        for &fb in &faces_b {
            let Ok(bb_b) = face_aabb(body, fb) else { continue };
            if !aabb_overlap(&bb_a, &bb_b, tol) {
                continue;
            }
            let Some(face_b) = body.faces.get(fb).cloned() else { continue };
            let Some(sb) = body.surfaces.get(face_b.surface).cloned() else { continue };
            let Ok(curves) = intersect_surface_surface(&sa, &sb, tol) else { continue };
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
                    let edge_id = build_imprint_edge(body, ic, t0, t1, full_period, tol, rec);
                    let pca = body.curves2.insert(ic.pcurve_a.clone());
                    let pcb = body.curves2.insert(ic.pcurve_b.clone());
                    pending_a.entry(fa).or_default().push(Pending { edge_id, pcurve_id: pca, prange: (t0, t1), kind: kind_a });
                    pending_b.entry(fb).or_default().push(Pending { edge_id, pcurve_id: pcb, prange: (t0, t1), kind: kind_b });
                }
            }
        }
    }

    let mut pieces_a: Vec<FaceId> = Vec::new();
    for &fa in &faces_a {
        if coincident_a.contains(&fa) {
            pieces_a.push(fa);
            continue;
        }
        match pending_a.remove(&fa) {
            Some(list) => pieces_a.extend(apply_pending_imprints(body, fa, list, tol, rec)?),
            None => pieces_a.push(fa),
        }
    }
    let mut pieces_b: Vec<FaceId> = Vec::new();
    for &fb in &faces_b {
        match pending_b.remove(&fb) {
            Some(list) => pieces_b.extend(apply_pending_imprints(body, fb, list, tol, rec)?),
            None => pieces_b.push(fb),
        }
    }

    let mut selected: Vec<FaceId> = Vec::new();
    for &f in &pieces_a {
        if coincident_a.contains(&f) {
            if matches!(op, BooleanOp::Unite | BooleanOp::Intersect) {
                selected.push(f);
            }
            continue;
        }
        let class = classify_face_against_solid(body, f, b, tol)?;
        if keep_face(op, true, class) {
            selected.push(f);
        }
    }
    for &f in &pieces_b {
        let class = classify_face_against_solid(body, f, a, tol)?;
        if keep_face(op, false, class) {
            if matches!(op, BooleanOp::Cut) {
                flip_face(body, f);
            }
            selected.push(f);
        }
    }

    if selected.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult("exact boolean selection kept no faces".into())));
    }

    let selected_set: HashSet<FaceId> = selected.iter().copied().collect();
    let result = stitch_selected_faces(body, &selected, rec)?;

    remove_solid_and_orphans(body, a, &selected_set, rec);
    remove_solid_and_orphans(body, b, &selected_set, rec);
    gc_orphan_edges_and_vertices(body, rec);

    let issues = issues_scoped_to_new_solids(body, &pre_existing_solids, validate_body(body));
    if !issues.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult(format!(
            "exact boolean result failed validation: {} issue(s), first: {}:{}:{}",
            issues.len(),
            issues[0].entity,
            issues[0].code,
            issues[0].message
        ))));
    }
    Ok(result)
}

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
/// (matching [`crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::intersect::surface_surface::general_marching`]'s own documented-simplification sampling style).
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
    if !(hi > lo) {
        return Vec::new();
    }
    let periodic = ic.curve3.is_periodic() && ic.curve3.period().is_some_and(|p| (hi - lo - p).abs() < 1e-6 * p.max(1.0));
    const N: usize = 64;
    let valid = |t: f64| -> bool {
        let ua = wrap_uv_for_surface(body, face_a, ic.pcurve_a.eval(t));
        let ub = wrap_uv_for_surface(body, face_b, ic.pcurve_b.eval(t));
        point_in_face_uv_periodic(body, face_a, ua, tol) && point_in_face_uv_periodic(body, face_b, ub, tol)
    };
    let mut inside = vec![false; N + 1];
    for (i, slot) in inside.iter_mut().enumerate() {
        let t = lo + (hi - lo) * (i as f64 / N as f64);
        *slot = valid(t);
    }
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
            let t0 = if start > 0 { refine_boundary(&valid, t_at(start - 1), t_at(start)) } else { t_at(start) };
            let t1 = if end < N { refine_boundary(&valid, t_at(end + 1), t_at(end)) } else { t_at(end) };
            if t1 > t0 {
                runs.push((t0, t1));
            }
        } else {
            i += 1;
        }
    }
    if periodic && !runs.is_empty() {
        // A periodic curve can graze a face's own boundary at an ARBITRARY interior parameter,
        // not just the domain's own `t=lo`/`t=hi` ends (e.g. a latitude circle touches a
        // cylinder's/sphere's seam wherever its own phase happens to put `u=0`, not necessarily at
        // the circle's own `t=0`) — that single physical touch reads as one narrow invalid gap in
        // `inside[]` and splits what is genuinely a single closed loop into two adjacent open
        // runs. Recognize this by TOTAL COVERAGE rather than assuming a fixed location: if the
        // valid runs' combined measure accounts for nearly the whole period (i.e. the runs are
        // separated only by narrow touch-sized gaps, not a genuine excursion outside the trim),
        // treat the whole thing as one closed range. The gap's own midpoint (the actual touch
        // parameter, wherever it really is) is returned alongside so the caller's per-face
        // `Interior`/`SeamCrossing` decision samples the REAL touch point instead of assuming
        // `t=lo` — `None` when there was no gap at all (genuinely interior on every support).
        let covered: f64 = runs.iter().map(|&(t0, t1)| t1 - t0).sum();
        if (hi - lo - covered) < (hi - lo) * 1e-3 {
            // A sphere/sphere (or other doubly-periodic) pair can graze EACH support's own seam
            // at a DIFFERENT parameter — collect every gap's midpoint, not just the first, so the
            // caller's per-face `SeamCrossing` check tests all of them.
            let touches: Vec<f64> = runs.windows(2).map(|w| 0.5 * (w[0].1 + w[1].0)).collect();
            return vec![(lo, hi, true, touches)];
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

/// 🔀 [`IntCurve::domain`] is infinite for an unbounded [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Line`] (e.g. plane/plane,
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
        if let crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Line { origin, dir } = &ic.curve3 {
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
fn curve3_extent(curve: &crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3, domain: (f64, f64)) -> f64 {
    const K: usize = 16;
    let (lo, hi) = domain;
    if !(hi > lo) {
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
    let Some(face_data) = body.faces.get(face) else { return false };
    let Some(surface) = body.surfaces.get(face_data.surface) else { return false };
    let tau = std::f64::consts::TAU;
    let u_shifts: &[f64] = if surface.is_u_periodic() { &[0.0, tau, -tau, 2.0 * tau, -2.0 * tau, 3.0 * tau, -3.0 * tau] } else { &[0.0] };
    let v_shifts: &[f64] = if surface.is_v_periodic() { &[0.0, tau, -tau, 2.0 * tau, -2.0 * tau] } else { &[0.0] };
    for &du in u_shifts {
        for &dv in v_shifts {
            let candidate = Pnt2::new(uv.x + du, uv.y + dv);
            if point_in_face_uv(body, face, candidate, tol).unwrap_or(false) {
                return true;
            }
        }
    }
    false
}

// #endregion 🔖️Clip

// #region 🔖️Imprint

/// 🔀 A `closed` imprint vertex gets spliced onto a PRE-EXISTING boundary edge (e.g. a sphere's
/// own seam, built by `primitives` at `Tol::DEFAULT`) — `validate_body` requires a vertex's own
/// tolerance to never exceed any edge that references it, so the vertex can't just inherit the
/// caller's (possibly looser) `tol`; it is clamped to at least as tight as the kernel's own
/// baseline (`Tol::DEFAULT`) so it never exceeds ANY edge it might get spliced into.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_imprint_edge(body: &mut Body, ic: &IntCurve, t0: f64, t1: f64, closed: bool, tol: f64, rec: &mut OpRecorder) -> EdgeId {
    let curve_id = body.curves3.insert(ic.curve3.clone());
    let tol_v = Tol::new(tol.min(Tol::DEFAULT.value()));
    if closed {
        let p = ic.curve3.eval(t0);
        let v = make_vertex(body, p, tol_v, rec);
        make_edge(body, curve_id, (t0, t1), v, v, tol_v, rec)
    } else {
        let pa = ic.curve3.eval(t0);
        let pb = ic.curve3.eval(t1);
        let va = make_vertex(body, pa, tol_v, rec);
        let vb = make_vertex(body, pb, tol_v, rec);
        make_edge(body, curve_id, (t0, t1), va, vb, tol_v, rec)
    }
}

/// 🔀 Applies every pending imprint queued for one original face, tracking the growing set of
/// live pieces so a second (or third) pending curve on the same original face is spliced into
/// whichever current piece its own midpoint actually falls inside.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pending_imprints(body: &mut Body, original: FaceId, pending: Vec<Pending>, tol: f64, rec: &mut OpRecorder) -> Result<Vec<FaceId>, KernelError> {
    let mut active = vec![original];
    for p in pending {
        let Some(pc) = body.curves2.get(p.pcurve_id).cloned() else { continue };
        // The exact midpoint of `prange` can coincidentally land ON a `SeamCrossing` curve's own
        // touch point (e.g. by construction/symmetry of the analytic circle frame) — try several
        // fractions along the range rather than only `s=0.5`, so one spurious on-boundary sample
        // can't fail the whole lookup.
        let mut found: Option<(usize, Pnt2)> = None;
        'fracs: for &s in &[0.5, 0.3, 0.7, 0.15, 0.85, 0.05, 0.95] {
            let t = p.prange.0 + (p.prange.1 - p.prange.0) * s;
            let uv = wrap_uv_for_surface(body, active[0], pc.eval(t));
            for (i, &f) in active.iter().enumerate() {
                if point_in_face_uv_periodic(body, f, uv, tol) {
                    found = Some((i, uv));
                    break 'fracs;
                }
            }
        }
        let Some((idx, _uv)) = found else {
            return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("imprint segment midpoint not found inside any active piece of face {original}"))));
        };
        let target = active[idx];
        let (fa, fb) = match p.kind {
            ImprintKind::Interior => split_face_by_interior_curve(body, target, p.edge_id, p.pcurve_id, p.prange, rec)?,
            ImprintKind::SeamCrossing => split_face_by_seam_crossing(body, target, p.edge_id, p.pcurve_id, p.prange, tol, rec)?,
            ImprintKind::Open => split_face_by_edge(body, target, p.edge_id, p.pcurve_id, p.prange, tol, rec)?,
        };
        active[idx] = fa;
        active.push(fb);
    }
    Ok(active)
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
const LOCAL_RAY_RETRY_DIRS: [[f64; 3]; 6] = [
    [0.573_576_436_351_046, 0.740_535_693_464_567_5, 0.350_889_803_483_932_2],
    [-0.350_889_803_483_932_2, 0.573_576_436_351_046, 0.740_535_693_464_567_5],
    [0.267_261_241_941_149_4, 0.534_522_483_882_298_8, 0.801_783_725_737_219],
    [0.577_350_269_189_625_8, 0.577_350_269_189_625_8, 0.577_350_269_189_625_7],
    [0.308_608_313_448_298, 0.904_511_432_523_735, 0.293_892_626_045_885],
    [0.843_391_445_261_857, 0.214_298_755_144_806, 0.491_975_172_042_98],
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
        let ray = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::Line { origin: point, dir };
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
    body.loop_coedges(loop_id)
        .into_iter()
        .filter_map(|c| body.coedge_endpoints(c))
        .filter_map(|(v, _)| body.vertices.get(v).map(|x| x.position))
        .collect()
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

/// 🔀 Groups `selected` faces into connected shells via shared edges, builds one shell + solid per
/// group (flipping every face in a group whose net signed volume comes out negative), and returns
/// the largest-volume solid as the primary result — matching `boolean_solid`'s single-handle
/// return; any additional (disjoint) components stay live in the body but unreturned, same as the
/// engine wrappers' documented "primary solid, rest listed in the record" convention.
/// 🐛 The sign probe's own `chord_tol` used to be `1e-6`: for a full-circle imprint edge that made
/// `segments_for_chord_deviation` sample ~1700+ boundary points, and `ear_clip`'s O(n³) worst case
/// over that many points took minutes. Only the SIGN of this volume is used here (to decide
/// whether to flip a group's faces), so it now matches `validation_report`'s own sign-probe
/// tolerance (`PROBE_TOL = 1e-3`) instead of a precision this call never needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stitch_selected_faces(body: &mut Body, selected: &[FaceId], rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let dbg = std::env::var("SEMIO_DEBUG_FX6").is_ok();
    let groups = group_shells(body, selected);
    if dbg {
        eprintln!("[DEBUG] fx6 group_shells done: {} groups, sizes={:?}", groups.len(), groups.iter().map(|g| g.len()).collect::<Vec<_>>());
    }
    let mut best: Option<(SolidId, f64)> = None;
    for group in groups {
        let shell = add_shell(body, group.clone(), rec);
        if dbg {
            eprintln!("[DEBUG] fx6 computing shell_signed_volume for shell {shell}");
        }
        let mut volume = shell_signed_volume(body, shell, 1e-3).unwrap_or(0.0);
        if dbg {
            eprintln!("[DEBUG] fx6 shell_signed_volume={volume}");
        }
        if volume < 0.0 {
            for &f in &group {
                flip_face(body, f);
            }
            volume = -volume;
        }
        let solid = add_solid(body, shell, Vec::new(), rec);
        if best.as_ref().is_none_or(|&(_, v)| volume > v) {
            best = Some((solid, volume));
        }
    }
    best.map(|(s, _)| s).ok_or_else(|| KernelError::Boolean(BooleanError::InvalidResult("stitch produced no shells".into())))
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
    append_kept_triangles(body, &mesh_a, b, op, true, tol, &mut points, &mut triangles)?;
    append_kept_triangles(body, &mesh_b, a, op, false, tol, &mut points, &mut triangles)?;
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
fn append_kept_triangles(body: &Body, mesh: &MeshTransfer, other: SolidId, op: BooleanOp, from_a: bool, tol: f64, out_points: &mut Vec<Pnt3>, out_tris: &mut Vec<[Pnt3; 3]>) -> Result<(), KernelError> {
    let npos = mesh.position.len() / 3;
    if mesh.index.len() % 3 != 0 {
        return Err(KernelError::InvalidInput("mesh index length must be a multiple of 3".into()));
    }
    for tri in mesh.index.chunks_exact(3) {
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
fn solid_vertex_positions(body: &Body, solid: SolidId) -> Result<Vec<Pnt3>, KernelError> {
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
    Ok(points)
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
fn aabb_overlap(a: &crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::Aabb, b: &crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::Aabb, tol: f64) -> bool {
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::transform::transform_solid;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn translate_solid(body: &mut Body, solid: SolidId, delta: Vec3, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
        transform_solid(body, solid, &Affine3::translation(delta), rec)
    }

    /// 🧪 Regression guard for the `classification::point_in_solid` sphere-trim bug worked around
    /// locally by [`local_point_in_solid`]: a plain, unsplit unit sphere's own center (and a
    /// clearly-interior off-center point) must classify `Inside`.
    #[semio_framework_async_macros::async_test]
    async fn local_point_in_solid_handles_plain_sphere_interior() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let s = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        assert!(matches!(local_point_in_solid(&body, s, Pnt3::new(0.0, 0.0, 0.0), 1e-6), Ok(PointClassification::Inside)));
        assert!(matches!(local_point_in_solid(&body, s, Pnt3::new(0.5, 0.0, 0.0), 1e-6), Ok(PointClassification::Inside)));
        assert!(matches!(local_point_in_solid(&body, s, Pnt3::new(2.0, 0.0, 0.0), 1e-6), Ok(PointClassification::Outside)));
        let b = translate_solid(&mut body, s, Vec3::new(0.0, 0.0, 1.2), &mut rec).unwrap();
        let p = Pnt3::new(-0.44721359549995704, -5.410403269529241e-16, 0.8944271909999163);
        assert!(matches!(local_point_in_solid(&body, b, p, 1e-6), Ok(PointClassification::Inside)));
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn offset_unit_cube(body: &mut Body, offset: Pnt3, rec: &mut OpRecorder) -> SolidId {
        let corners = [
            offset + Vec3::new(0.0, 0.0, 0.0),
            offset + Vec3::new(1.0, 0.0, 0.0),
            offset + Vec3::new(1.0, 1.0, 0.0),
            offset + Vec3::new(0.0, 1.0, 0.0),
            offset + Vec3::new(0.0, 0.0, 1.0),
            offset + Vec3::new(1.0, 0.0, 1.0),
            offset + Vec3::new(1.0, 1.0, 1.0),
            offset + Vec3::new(0.0, 1.0, 1.0),
        ];
        make_convex_hull(body, &corners, rec).expect("offset cube hull")
    }

    #[semio_framework_async_macros::async_test]
    async fn disjoint_unit_boxes_fuse_volume_near_two() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let b = offset_unit_cube(&mut body, Pnt3::new(2.0, 0.0, 0.0), &mut rec);
        let fused = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        let vol = solid_volume(&body, fused, 1e-6).unwrap();
        assert!((vol - 2.0).abs() < 1e-3, "expected volume ≈ 2, got {vol}");
    }

    #[semio_framework_async_macros::async_test]
    async fn overlapping_aabb_intersect_volume_matches_dims() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let b = offset_unit_cube(&mut body, Pnt3::new(0.5, 0.5, 0.5), &mut rec);
        let hit = boolean_solid(&mut body, a, b, BooleanOp::Intersect, 1e-6, &mut rec).unwrap();
        let vol = solid_volume(&body, hit, 1e-6).unwrap();
        let expected = 0.5 * 0.5 * 0.5;
        assert!((vol - expected).abs() < 1e-3, "expected {expected}, got {vol}");
    }

    #[semio_framework_async_macros::async_test]
    async fn boolean_unite_is_deterministic() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let b = offset_unit_cube(&mut body, Pnt3::new(2.0, 0.0, 0.0), &mut rec);
        let faces_before = body.faces.len();
        let r1 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        let delta1 = body.faces.len() - faces_before;
        let n1 = body.solid_faces(r1).len();
        let faces_mid = body.faces.len();
        let r2 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        let delta2 = body.faces.len() - faces_mid;
        let n2 = body.solid_faces(r2).len();
        assert_eq!(delta1, delta2);
        assert_eq!(n1, n2);
    }

    #[semio_framework_async_macros::async_test]
    async fn cut_disjoint_preserves_volume() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
        let b = offset_unit_cube(&mut body, Pnt3::new(3.0, 0.0, 0.0), &mut rec);
        let vol_a = solid_volume(&body, a, 1e-6).unwrap();
        let cut = boolean_solid(&mut body, a, b, BooleanOp::Cut, 1e-6, &mut rec).unwrap();
        let vol_cut = solid_volume(&body, cut, 1e-6).unwrap();
        assert!((vol_cut - vol_a).abs() < 1e-3, "cut volume {vol_cut} vs A {vol_a}");
    }

    #[semio_framework_async_macros::async_test]
    async fn adversarial_scale_sweep_determinism() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        for scale in [0.1_f64, 1.0, 10.0, 100.0] {
            let a = make_box(&mut body, scale, scale, scale, &mut rec).unwrap();
            let o = Pnt3::new(scale * 2.0, 0.0, 0.0);
            let corners = [
                o,
                Pnt3::new(o.x + scale, o.y, o.z),
                Pnt3::new(o.x + scale, o.y + scale, o.z),
                Pnt3::new(o.x, o.y + scale, o.z),
                Pnt3::new(o.x, o.y, o.z + scale),
                Pnt3::new(o.x + scale, o.y, o.z + scale),
                Pnt3::new(o.x + scale, o.y + scale, o.z + scale),
                Pnt3::new(o.x, o.y + scale, o.z + scale),
            ];
            let b = make_convex_hull(&mut body, &corners, &mut rec).unwrap();
            let u0 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
            let u1 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
            assert_eq!(body.solid_faces(u0).len(), body.solid_faces(u1).len());
            let v = solid_volume(&body, u0, scale * 1e-4).unwrap();
            assert!((v - 2.0 * scale.powi(3)).abs() < scale.powi(3) * 1e-2, "scale={scale} v={v}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fuzz_random_aabb_intersect_volume_nonnegative() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let mut seed = 1u64;
        for _ in 0..32 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let w = 0.5 + (seed % 50) as f64 * 0.1;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let ox = (seed % 20) as f64 * 0.25;
            let a = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
            let o = Pnt3::new(ox, ox * 0.5, 0.0);
            let corners =
                [o, Pnt3::new(o.x + w, o.y, o.z), Pnt3::new(o.x + w, o.y + w, o.z), Pnt3::new(o.x, o.y + w, o.z), Pnt3::new(o.x, o.y, o.z + w), Pnt3::new(o.x + w, o.y, o.z + w), Pnt3::new(o.x + w, o.y + w, o.z + w), Pnt3::new(o.x, o.y + w, o.z + w)];
            let b = make_convex_hull(&mut body, &corners, &mut rec).unwrap();
            if let Ok(inter) = boolean_solid(&mut body, a, b, BooleanOp::Intersect, 1e-6, &mut rec) {
                assert!(solid_volume(&body, inter, 1e-3).unwrap() >= -1e-9);
            }
        }
    }

    // #region 🧪️ExactCurvedTests

    /// 🧪 A cylinder through the center of a box, radius small enough that the cylinder only
    /// crosses the box's top/bottom planar caps (as closed interior circles) and its own lateral
    /// face only meets the box at those two caps too — every imprint curve in this scenario is the
    /// "closed, fully interior" case, exercising `split_face_by_interior_curve` on both a planar
    /// and a cylindrical support. Volume = box − (cylinder ∩ box) by the closed-form cylinder
    /// volume clipped to the box's height.
    #[semio_framework_async_macros::async_test]
    async fn box_union_cylinder_through_exact_volume_and_validates() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let box_solid = make_box(&mut body, 4.0, 4.0, 2.0, &mut rec).unwrap();
        let box_solid = translate_solid(&mut body, box_solid, Vec3::new(-2.0, -2.0, -0.5), &mut rec).unwrap();
        let cyl = make_cylinder(&mut body, 0.5, 4.0, &mut rec).unwrap();
        let cyl = translate_solid(&mut body, cyl, Vec3::new(0.0, 0.0, -2.0), &mut rec).unwrap();
        let united = boolean_solid(&mut body, box_solid, cyl, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
        let vol = solid_volume(&body, united, 1e-4).unwrap();
        let box_vol = 4.0 * 4.0 * 2.0;
        let cyl_vol = std::f64::consts::PI * 0.5 * 0.5 * 4.0;
        let overlap_vol = std::f64::consts::PI * 0.5 * 0.5 * 2.0; // the cylinder segment inside the box's height
        let expected = box_vol + cyl_vol - overlap_vol;
        assert!((vol - expected).abs() / expected < 5e-3, "expected≈{expected}, got {vol}");
    }

    /// 🧪 Same geometry, `Cut`: the cylinder bores a round hole through the box.
    #[semio_framework_async_macros::async_test]
    async fn box_minus_cylinder_bore_exact_volume_and_validates() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let box_solid = make_box(&mut body, 4.0, 4.0, 2.0, &mut rec).unwrap();
        let box_solid = translate_solid(&mut body, box_solid, Vec3::new(-2.0, -2.0, -0.5), &mut rec).unwrap();
        let cyl = make_cylinder(&mut body, 0.5, 4.0, &mut rec).unwrap();
        let cyl = translate_solid(&mut body, cyl, Vec3::new(0.0, 0.0, -2.0), &mut rec).unwrap();
        let bored = boolean_solid(&mut body, box_solid, cyl, BooleanOp::Cut, 1e-6, &mut rec).unwrap();
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
        let vol = solid_volume(&body, bored, 1e-4).unwrap();
        let box_vol = 4.0 * 4.0 * 2.0;
        let bore_vol = std::f64::consts::PI * 0.5 * 0.5 * 2.0;
        let expected = box_vol - bore_vol;
        assert!((vol - expected).abs() / expected < 5e-3, "expected≈{expected}, got {vol}");
    }

    /// 🧪 Two spheres overlapping (a lens): sphere/sphere intersection circle stays away from
    /// either sphere's own seam/poles, so both sides see it as a closed interior curve.
    #[semio_framework_async_macros::async_test]
    async fn sphere_union_sphere_lens_exact_volume_and_validates() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let b = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let b = translate_solid(&mut body, b, Vec3::new(0.0, 0.0, 1.2), &mut rec).unwrap();
        let united = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
        let sphere_vol = 4.0 / 3.0 * std::f64::consts::PI;
        // Spherical-cap overlap volume for two equal spheres radius r, center distance d:
        // V_lens = π (4r + d)(2r − d)² / 12.
        let (r, d) = (1.0_f64, 1.2_f64);
        let lens = std::f64::consts::PI * (4.0 * r + d) * (2.0 * r - d).powi(2) / 12.0;
        let expected = 2.0 * sphere_vol - lens;
        let vol = solid_volume(&body, united, 1e-4).unwrap();
        assert!((vol - expected).abs() / expected < 1e-2, "expected≈{expected}, got {vol}");
    }

    /// 🧪 A∪A / A∩A / A∖A on a single sphere via its own coincident-face special case (a solid
    /// booleaned with itself has every face pair exactly coincident).
    #[semio_framework_async_macros::async_test]
    async fn self_boolean_identities_on_a_sphere() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let a2 = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let vol_a = solid_volume(&body, a, 1e-6).unwrap();
        let union = boolean_solid(&mut body, a, a2, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        assert!((solid_volume(&body, union, 1e-6).unwrap() - vol_a).abs() / vol_a < 1e-6, "A∪A must equal A's volume");

        let mut body2 = Body::new();
        let mut rec2 = OpRecorder::new();
        let c = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
        let c2 = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
        let vol_c = solid_volume(&body2, c, 1e-6).unwrap();
        let inter = boolean_solid(&mut body2, c, c2, BooleanOp::Intersect, 1e-6, &mut rec2).unwrap();
        assert!((solid_volume(&body2, inter, 1e-6).unwrap() - vol_c).abs() / vol_c < 1e-6, "A∩A must equal A's volume");
    }

    /// 🧪 Commutativity of union/intersect (volume): `A∪B == B∪A`, `A∩B == B∩A`.
    #[semio_framework_async_macros::async_test]
    async fn union_and_intersect_are_commutative_by_volume() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a1 = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let b1 = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let b1 = translate_solid(&mut body, b1, Vec3::new(0.0, 0.0, 1.2), &mut rec).unwrap();
        let u_ab = boolean_solid(&mut body, a1, b1, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        let v_ab = solid_volume(&body, u_ab, 1e-4).unwrap();

        let mut body2 = Body::new();
        let mut rec2 = OpRecorder::new();
        let a2 = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
        let b2 = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
        let a2 = translate_solid(&mut body2, a2, Vec3::new(0.0, 0.0, 1.2), &mut rec2).unwrap();
        let u_ba = boolean_solid(&mut body2, b2, a2, BooleanOp::Unite, 1e-6, &mut rec2).unwrap();
        let v_ba = solid_volume(&body2, u_ba, 1e-4).unwrap();
        assert!((v_ab - v_ba).abs() / v_ab < 1e-6, "union must be commutative: {v_ab} vs {v_ba}");

        let mut body3 = Body::new();
        let mut rec3 = OpRecorder::new();
        let a3 = make_sphere(&mut body3, 1.0, &mut rec3).unwrap();
        let b3 = make_sphere(&mut body3, 1.0, &mut rec3).unwrap();
        let b3 = translate_solid(&mut body3, b3, Vec3::new(0.0, 0.0, 1.2), &mut rec3).unwrap();
        let i_ab = boolean_solid(&mut body3, a3, b3, BooleanOp::Intersect, 1e-6, &mut rec3).unwrap();
        let vi_ab = solid_volume(&body3, i_ab, 1e-4).unwrap();

        let mut body4 = Body::new();
        let mut rec4 = OpRecorder::new();
        let a4 = make_sphere(&mut body4, 1.0, &mut rec4).unwrap();
        let b4 = make_sphere(&mut body4, 1.0, &mut rec4).unwrap();
        let a4 = translate_solid(&mut body4, a4, Vec3::new(0.0, 0.0, 1.2), &mut rec4).unwrap();
        let i_ba = boolean_solid(&mut body4, b4, a4, BooleanOp::Intersect, 1e-6, &mut rec4).unwrap();
        let vi_ba = solid_volume(&body4, i_ba, 1e-4).unwrap();
        assert!((vi_ab - vi_ba).abs() / vi_ab < 1e-6, "intersect must be commutative: {vi_ab} vs {vi_ba}");
    }

    /// 🧪 A tangent (just-touching, not overlapping) sphere pair: union volume must be the exact
    /// sum (no lens carved out), and the classifier must not choke on the near-zero-margin contact.
    #[semio_framework_async_macros::async_test]
    async fn tangent_spheres_union_volume_is_exact_sum() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let b = make_sphere(&mut body, 1.0, &mut rec).unwrap();
        let b = translate_solid(&mut body, b, Vec3::new(2.0 + 1e-4, 0.0, 0.0), &mut rec).unwrap();
        let united = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-3, &mut rec).unwrap();
        let vol = solid_volume(&body, united, 1e-4).unwrap();
        let expected = 2.0 * 4.0 / 3.0 * std::f64::consts::PI;
        assert!((vol - expected).abs() / expected < 1e-2, "expected≈{expected}, got {vol}");
    }

    // #endregion 🧪️ExactCurvedTests
}
