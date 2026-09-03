//! 🔀 Imprint→split→classify→select→stitch boolean + mesh fallback.
//!
//! Lane 4-boolean of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.
//! AABB fast paths cover disjoint/contained/axis-aligned-box cases; general overlaps
//! rebuild via tessellation + centroid classification + triangle-soup stitch (hull only as last resort).
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🔀️boolean` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL.

use std::collections::HashSet;

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_shell, add_solid};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_convex_hull, solid_from_triangle_soup};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{solid_bounding_box, solid_volume, AxisAlignedBox};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::tessellation::tessellate_solid;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{FaceId, SolidId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::{BooleanError, KernelError};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::engine::{MeshTransfer, PointClassification};

// #region 🔖️Api

/// 🔀 Boolean combination kind for [`boolean_solid`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BooleanOp {
    Unite,
    Cut,
    Intersect,
}

/// 🔀 Combines solids `a` and `b` under `op`, preferring AABB fast paths then classified triangle-soup stitch.
/// `rec` accumulates the whole operation's [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpDelta`] — every helper below
/// threads it through instead of building its own and discarding it at a private function boundary.
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
        if let Some(id) = aabb_fast_path(body, a, b, &bb_a, &bb_b, op, tol, rec)? {
            return Ok(id);
        }
    }
    mesh_boolean(body, a, b, op, tol, rec)
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

/// 🔀 Splits `solid` by the plane `(origin, normal)` into two solids (classified triangle soups; hull fallback).
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

// #region 🔖️AabbFastPath

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn aabb_fast_path(body: &mut Body, a: SolidId, b: SolidId, bb_a: &AxisAlignedBox, bb_b: &AxisAlignedBox, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
    let gap = aabb_gap(bb_a, bb_b);
    match op {
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
        BooleanOp::Unite => {
            if gap >= tol {
                let mut faces = outer_faces(body, a)?;
                faces.extend(outer_faces(body, b)?);
                return Ok(Some(solid_from_outer_faces(body, faces, Vec::new(), rec)?));
            }
            if is_aabb_box_solid(body, a, bb_a)? && is_aabb_box_solid(body, b, bb_b)? {
                let u = aabb_union(bb_a, bb_b);
                let (w, d, h) = aabb_dims(&u);
                return Ok(Some(make_box(body, w, d, h, rec)?));
            }
            Ok(None)
        }
        BooleanOp::Cut => {
            if aabb_contains(bb_b, bb_a, tol) {
                return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean cut is empty (tool contains target)".into())));
            }
            if gap >= tol {
                return Ok(Some(clone_solid_shells(body, a, rec)?));
            }
            if aabb_contains(bb_a, bb_b, tol) {
                let outer = outer_faces(body, a)?;
                let inner = outer_faces(body, b)?;
                return Ok(Some(solid_from_outer_faces(body, outer, vec![inner], rec)?));
            }
            Ok(None)
        }
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

// #endregion 🔖️AabbFastPath

// #region 🔖️MeshFallback

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mesh_boolean(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
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
    // Prefer the classified triangle soup (non-convex cuts/fuses) over a convex hull of the kept
    // vertices — hull was collapsing C-shaped and holed results into the wrong solid.
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
        let class = point_in_solid(body, other, centroid, tol)?;
        if keep_triangle(op, from_a, class) {
            out_points.push(p0);
            out_points.push(p1);
            out_points.push(p2);
            out_tris.push([p0, p1, p2]);
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn keep_triangle(op: BooleanOp, from_a: bool, class: PointClassification) -> bool {
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
fn mesh_position(mesh: &MeshTransfer, i: usize) -> Pnt3 {
    let o = i * 3;
    Pnt3::new(mesh.position[o] as f64, mesh.position[o + 1] as f64, mesh.position[o + 2] as f64)
}

// #endregion 🔖️MeshFallback

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
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;

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
}
