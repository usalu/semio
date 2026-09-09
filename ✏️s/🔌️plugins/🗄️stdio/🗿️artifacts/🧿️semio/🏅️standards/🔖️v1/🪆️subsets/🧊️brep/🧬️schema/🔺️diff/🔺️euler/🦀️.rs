//! ✂️ Checked topology editors (the *only* functions permitted to mutate a `Body`) plus UV planar
//! arrangement and face split via those editors (imprint). Shares one compute subdir per the
//! `✂️intersect`-style "one compute subdir, not a 1:1 file mapping" precedent this ticket's wave
//! PEEL established — no dedicated `🖋️imprint` facet was pre-mounted, and the primitive editors
//! and the one operation built on them belong together. Framework-3d's own `🔺️euler` module
//! (`make_vertex`/`make_edge`/`make_loop`/`add_face`/`add_shell`/`add_solid`/`split_edge`) moved
//! in HERE, alongside the imprint code already resident from wave PEEL, in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3.
//!
//! Minimum viable imprint: cut a planar face with a line through two points, splitting the outer
//! loop into two faces that share a new chord edge. Lane 3-imprint of ticket
//! `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`. Moved from
//! `🧰️framework/🔨️modules/🧊️3d/📐️brep/🖋️imprint` (imprint) and
//! `🧰️framework/🔨️modules/🧊️3d/📐️brep/🔺️euler` (editors) in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, waves PEEL and PEEL3.
/// 🔗 Oriented edge with optional parameter curve and its range.
pub type ParametricEdge = (EdgeId, bool, Option<Curve2Id>, (f64, f64));

/// 🚶 Ordered vertices and oriented edges around a loop.
pub type LoopWalk = (Vec<VertexId>, Vec<(EdgeId, bool)>);

use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, SurfaceId, VertexId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::closest_parameter;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3};
use crate::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Coedge, Edge, Face, Loop, Shell, Solid, Vertex};
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
#[cfg(test)]
use crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3};

// #region 🔖️Make

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dummy_coedge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}

/// ✂️ Creates a new vertex, recording it as generated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_vertex(body: &mut Body, position: Pnt3, tol: Tol, rec: &mut OpRecorder) -> VertexId {
    let label = body.new_label();
    rec.record_generated(label);
    body.vertices.insert(Vertex { position, tol, label })
}

/// ✂️ Creates a new edge referencing shared curve geometry, recording it as generated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_edge(body: &mut Body, curve: Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    let label = body.new_label();
    rec.record_generated(label);
    body.edges.insert(Edge { curve, range, v0, v1, tol, label })
}

/// ✂️ Builds a closed coedge ring from `members` (one `(edge, forward)` pair per coedge, in ring
/// order) and links it into a new [`Loop`]. Loops/coedges have no [`crate::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]
/// of their own (they are structural, not independently document-nameable), so nothing is recorded.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn make_loop(body: &mut Body, face: FaceId, members: &[(EdgeId, bool)]) -> LoopId {
    let loop_id = body.loops.insert(Loop { first: dummy_coedge(), face });
    let coedge_ids: Vec<CoedgeId> = members.iter().map(|&(edge, forward)| body.coedges.insert(Coedge { edge, forward, pcurve: None, prange: (0.0, 0.0), loop_id, next: dummy_coedge(), prev: dummy_coedge() })).collect();
    let n = coedge_ids.len();
    for i in 0..n {
        let coedge = body.coedges.get_mut(coedge_ids[i]).unwrap();
        coedge.next = coedge_ids[(i + 1) % n];
        coedge.prev = coedge_ids[(i + n - 1) % n];
    }
    body.loops.get_mut(loop_id).unwrap().first = coedge_ids[0];
    loop_id
}

/// ✂️ Creates a new face, recording it as generated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn add_face(body: &mut Body, surface: SurfaceId, outer: Option<LoopId>, inners: Vec<LoopId>, flipped: bool, tol: Tol, rec: &mut OpRecorder) -> FaceId {
    let label = body.new_label();
    rec.record_generated(label);
    body.faces.insert(Face { surface, outer, inners, flipped, tol, label })
}

/// ✂️ Creates a new shell, recording it as generated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn add_shell(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> ShellId {
    let label = body.new_label();
    rec.record_generated(label);
    body.shells.insert(Shell { faces, label })
}

/// ✂️ Creates a new solid, recording it as generated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn add_solid(body: &mut Body, outer: ShellId, inners: Vec<ShellId>, rec: &mut OpRecorder) -> SolidId {
    let label = body.new_label();
    rec.record_generated(label);
    body.solids.insert(Solid { outer, inners, label })
}

// #endregion 🔖️Make

// #region 🔖️SplitJoin

/// ✂️ Splits `edge_id` at curve parameter `t` (which must lie strictly within the edge's current
/// range) into two edges sharing the same underlying curve, joined by a new vertex at `position`.
/// Every coedge that used the old edge is replaced by two coedges spliced into the same ring in
/// the correct order — including the degenerate case of a single self-referential coedge (a full
/// periodic edge, e.g. a closed circle, forming a one-coedge loop). Returns
/// `(first_half, second_half, new_vertex)`, where "first"/"second" are relative to the edge's own
/// `v0 → v1` direction (not any particular coedge's orientation).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_edge(body: &mut Body, edge_id: EdgeId, t: f64, position: Pnt3, rec: &mut OpRecorder) -> (EdgeId, EdgeId, VertexId) {
    let old_edge = body.edges.get(edge_id).expect("split_edge requires a live edge id").clone();
    let new_vertex = make_vertex(body, position, old_edge.tol, rec);
    let (e1, e2) = split_edge_with_vertex(body, edge_id, t, new_vertex, rec);
    (e1, e2, new_vertex)
}

/// ✂️ Like [`split_edge`], but binds a caller-provided (already-created) vertex at the split point
/// instead of minting a fresh one — the imprint layer uses this so both sides of a shared boolean
/// boundary vertex end up referencing the identical [`VertexId`]. Every affected coedge's own
/// p-curve (when present) is split proportionally alongside the 3D curve, using the SAME
/// normalized split fraction on both — pcurve `prange` and edge `range` share one parametrization
/// convention (`t = edge.range.0 + (edge.range.1 - edge.range.0)·s`, `p = prange.0 +
/// (prange.1-prange.0)·s`, always in the edge's own curve order per [`crate::standards::v1::subsets::brep::schema::diff::primitives`]'s
/// documented convention) — this was the previous version's own latent bug (it dropped every
/// split coedge's pcurve to `None`, silently breaking validation's `missing-pcurve` check on any
/// face that had ever been imprinted).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_edge_with_vertex(body: &mut Body, edge_id: EdgeId, t: f64, vertex: VertexId, rec: &mut OpRecorder) -> (EdgeId, EdgeId) {
    let old_edge = body.edges.get(edge_id).expect("split_edge_with_vertex requires a live edge id").clone();
    debug_assert!(t > old_edge.range.0 && t < old_edge.range.1, "split parameter must lie strictly within the edge's range");
    let e1 = make_edge(body, old_edge.curve, (old_edge.range.0, t), old_edge.v0, vertex, old_edge.tol, rec);
    let e2 = make_edge(body, old_edge.curve, (t, old_edge.range.1), vertex, old_edge.v1, old_edge.tol, rec);
    let span = old_edge.range.1 - old_edge.range.0;
    let s = if span.abs() > 1e-30 { (t - old_edge.range.0) / span } else { 0.5 };
    let affected: Vec<CoedgeId> = body.edge_coedges(edge_id);
    for coedge_id in affected {
        let coedge = body.coedges.get(coedge_id).unwrap().clone();
        let self_loop = coedge.prev == coedge_id && coedge.next == coedge_id;
        let (pcurve1, prange1, pcurve2, prange2) = match coedge.pcurve {
            Some(pc) => {
                let mid_p = coedge.prange.0 + (coedge.prange.1 - coedge.prange.0) * s;
                (Some(pc), (coedge.prange.0, mid_p), Some(pc), (mid_p, coedge.prange.1))
            }
            None => (None, (0.0, 0.0), None, (0.0, 0.0)),
        };
        // `coedge_e1`/`coedge_e2` reference `e1`/`e2` respectively — always the (range.0..t) and
        // (t..range.1) halves in the edge's OWN curve order, independent of `forward`.
        let coedge_e1 = body.coedges.insert(Coedge { edge: e1, forward: coedge.forward, pcurve: pcurve1, prange: prange1, loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        let coedge_e2 = body.coedges.insert(Coedge { edge: e2, forward: coedge.forward, pcurve: pcurve2, prange: prange2, loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        // Ring order follows traversal direction: forward visits e1 then e2; reversed visits e2 then e1.
        let (c1, c2) = if coedge.forward { (coedge_e1, coedge_e2) } else { (coedge_e2, coedge_e1) };
        if self_loop {
            body.coedges.get_mut(c1).unwrap().prev = c2;
            body.coedges.get_mut(c1).unwrap().next = c2;
            body.coedges.get_mut(c2).unwrap().prev = c1;
            body.coedges.get_mut(c2).unwrap().next = c1;
        } else {
            let prev_id = coedge.prev;
            let next_id = coedge.next;
            body.coedges.get_mut(c1).unwrap().prev = prev_id;
            body.coedges.get_mut(c1).unwrap().next = c2;
            body.coedges.get_mut(c2).unwrap().prev = c1;
            body.coedges.get_mut(c2).unwrap().next = next_id;
            body.coedges.get_mut(prev_id).unwrap().next = c1;
            body.coedges.get_mut(next_id).unwrap().prev = c2;
        }
        if let Some(lp) = body.loops.get_mut(coedge.loop_id) {
            if lp.first == coedge_id {
                lp.first = c1;
            }
        }
        body.coedges.remove(coedge_id);
    }
    body.edges.remove(edge_id);
    rec.record_deleted(old_edge.label);
    (e1, e2)
}

/// ✂️ Splits `edge_id` at every parameter in `params` (need not be sorted), returning the new
/// interior vertices in ascending-parameter order. Each split narrows the remaining "upper" half,
/// so later parameters are located relative to the surviving edge, not the original.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_edge_at_params(body: &mut Body, edge_id: EdgeId, params: &[f64], rec: &mut OpRecorder) -> Vec<VertexId> {
    let mut sorted: Vec<f64> = params.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut current = edge_id;
    let mut verts = Vec::with_capacity(sorted.len());
    for t in sorted {
        let Some(edge) = body.edges.get(current) else { break };
        if !(t > edge.range.0 && t < edge.range.1) {
            continue;
        }
        let curve = edge.curve;
        let position = body.curves3.get(curve).expect("edge curve").eval(t);
        let (_e1, e2, v) = split_edge(body, current, t, position, rec);
        verts.push(v);
        current = e2;
    }
    verts
}

// #endregion 🔖️SplitJoin

// #region 🔖️Api

/// 🖋️ Split a planar face along the line through `p0` and `p1`.
///
/// Intersects the infinite line with the outer boundary in the face UV plane, splits the two hit
/// edges (or reuses existing vertices when the hit lands on a corner), inserts a chord edge, and
/// rebuilds two outer loops via Euler editors. Returns `(original_face, new_face)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_planar_face_by_line(body: &mut Body, face: FaceId, p0: Pnt3, p1: Pnt3) -> Result<(FaceId, FaceId), KernelError> {
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
    let outer = face_data.outer.ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    if !face_data.inners.is_empty() {
        return Err(KernelError::Operation("split_planar_face_by_line does not support faces with inner loops yet".into()));
    }
    let surface = body.surfaces.get(face_data.surface).ok_or_else(|| KernelError::MissingEntity(format!("surface {}", face_data.surface)))?.clone();
    let Surface::Plane { frame } = surface else {
        return Err(KernelError::InvalidInput("split_planar_face_by_line requires a planar face".into()));
    };
    let tol = face_data.tol;
    let linear = tol.value().max(1e-12);
    if (p1 - p0).norm() <= linear {
        return Err(KernelError::InvalidInput("cutting line endpoints must be distinct".into()));
    }
    let uv0 = project_uv(&frame, p0);
    let uv1 = project_uv(&frame, p1);
    if (uv1 - uv0).norm() <= linear {
        return Err(KernelError::InvalidInput("cutting line is orthogonal to the face plane".into()));
    }

    let coedges = body.loop_coedges(outer);
    if coedges.len() < 3 {
        return Err(KernelError::Operation(format!("face {face} outer loop needs at least 3 coedges")));
    }

    let mut hits: Vec<BoundaryHit> = Vec::new();
    for &cid in &coedges {
        let coedge = body.coedges.get(cid).ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid}")))?;
        let edge_id = coedge.edge;
        if hits.iter().any(|h| h.edge == edge_id) {
            continue;
        }
        let edge = body.edges.get(edge_id).ok_or_else(|| KernelError::MissingEntity(format!("edge {edge_id}")))?.clone();
        let v0 = body.vertices.get(edge.v0).ok_or_else(|| KernelError::MissingEntity(format!("vertex {}", edge.v0)))?.position;
        let v1 = body.vertices.get(edge.v1).ok_or_else(|| KernelError::MissingEntity(format!("vertex {}", edge.v1)))?.position;
        let a = project_uv(&frame, v0);
        let b = project_uv(&frame, v1);
        let Some((_t_seg, point_uv)) = intersect_segment_line_uv(a, b, uv0, uv1, linear) else {
            continue;
        };
        let point = frame.to_world(Pnt3::new(point_uv.x, point_uv.y, 0.0));
        let curve_t = edge_param_at_point(body, &edge, point, linear)?;
        if let Some(existing) = endpoint_vertex(&edge, curve_t, linear) {
            if !hits.iter().any(|h| h.vertex_hint == Some(existing) || h.point.distance(point) <= linear) {
                hits.push(BoundaryHit { edge: edge_id, curve_t, point, vertex_hint: Some(existing) });
            }
            continue;
        }
        if !(curve_t > edge.range.0 + 1e-14 && curve_t < edge.range.1 - 1e-14) {
            continue;
        }
        if hits.iter().any(|h| h.point.distance(point) <= linear) {
            continue;
        }
        hits.push(BoundaryHit { edge: edge_id, curve_t, point, vertex_hint: None });
    }

    if hits.len() != 2 {
        return Err(KernelError::Operation(format!("cutting line must cross the outer boundary at exactly two places (got {})", hits.len())));
    }
    // Same-edge double hits are supported: split higher curve_t first so the lower parameter stays valid.
    if hits[0].edge == hits[1].edge {
        hits.sort_by(|a, b| b.curve_t.partial_cmp(&a.curve_t).unwrap_or(std::cmp::Ordering::Equal));
    }

    let mut rec = OpRecorder::new();
    let mut split_vertices = [VertexId::from_raw(0, 0); 2];
    let mut surviving_same_edge: Option<(EdgeId, EdgeId)> = None;
    for (i, hit) in hits.iter().enumerate() {
        if let Some(v) = hit.vertex_hint {
            split_vertices[i] = v;
            continue;
        }
        let edge_for_split = if i == 1 && hits[0].edge == hits[1].edge {
            let (e1, e2) = surviving_same_edge.ok_or_else(|| KernelError::Operation("same-edge double hit missing first split survivors".into()))?;
            resolve_edge_containing_param(body, e1, e2, hit.curve_t)?
        } else {
            hit.edge
        };
        let (e1, e2, v) = split_edge(body, edge_for_split, hit.curve_t, hit.point, &mut rec);
        split_vertices[i] = v;
        if i == 0 && hits[0].edge == hits[1].edge {
            surviving_same_edge = Some((e1, e2));
        }
    }
    let va = split_vertices[0];
    let vb = split_vertices[1];
    if va == vb {
        return Err(KernelError::Operation("cutting line degenerates to a single boundary vertex".into()));
    }

    let (verts, members) = loop_walk_pc(body, outer)?;
    let ia = verts.iter().position(|&v| v == va).ok_or_else(|| KernelError::Operation("split vertex A missing from outer loop".into()))?;
    let ib = verts.iter().position(|&v| v == vb).ok_or_else(|| KernelError::Operation("split vertex B missing from outer loop".into()))?;
    let chain_ab = member_chain_pc(&members, ia, ib);
    let chain_ba = member_chain_pc(&members, ib, ia);
    if chain_ab.is_empty() || chain_ba.is_empty() {
        return Err(KernelError::Operation("cutting line does not partition the outer loop into two non-empty chains".into()));
    }

    let pa = body.vertices.get(va).expect("vertex A").position;
    let pb = body.vertices.get(vb).expect("vertex B").position;
    let cut_curve = body.curves3.insert(Curve3::Line { origin: pa, dir: pb - pa });
    let cut = make_edge(body, cut_curve, (0.0, 1.0), va, vb, tol, &mut rec);
    // Same planar face on both sides of the chord, so the chord's own p-curve is a straight line
    // in UV, in the edge's own curve order (`p = uv_a + (uv_b - uv_a) * t`, matching `t ∈ (0, 1)`
    // on the 3D curve above) — never reparametrized per coedge, per the p-curve convention.
    let uv_a = project_uv(&frame, pa);
    let uv_b = project_uv(&frame, pb);
    let cut_pcurve = body.curves2.insert(Curve2::Line { origin: uv_a, dir: uv_b - uv_a });

    let mut members_a = chain_ab;
    members_a.push((cut, false, Some(cut_pcurve), (0.0, 1.0)));
    let mut members_b = chain_ba;
    members_b.push((cut, true, Some(cut_pcurve), (0.0, 1.0)));

    for cid in body.loop_coedges(outer) {
        body.coedges.remove(cid);
    }
    body.loops.remove(outer);

    let loop_a = make_loop_pc(body, face, &members_a);
    {
        let f = body.faces.get_mut(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?;
        f.outer = Some(loop_a);
        f.inners.clear();
    }

    let loop_b = make_loop_pc(body, FaceId::from_raw(0, 0), &members_b);
    let face_b = add_face(body, face_data.surface, Some(loop_b), vec![], face_data.flipped, tol, &mut rec);
    body.loops.get_mut(loop_b).ok_or_else(|| KernelError::MissingEntity(format!("loop {loop_b}")))?.face = face_b;

    for (_, shell) in body.shells.iter_mut() {
        if shell.faces.contains(&face) && !shell.faces.contains(&face_b) {
            shell.faces.push(face_b);
        }
    }

    Ok((face, face_b))
}

// #endregion 🔖️Api

// #region 🔖️Imprint

/// 🖋️ Ensures `target` is a vertex on `loop_id`'s ring: a no-op if it already is (e.g. a sibling
/// imprint already spliced it in), otherwise splits whichever boundary edge `position` lies on
/// (via [`split_edge_with_vertex`], preserving every affected coedge's p-curve). Returns an error
/// when `position` touches neither an existing ring vertex nor any ring edge's span — the caller's
/// imprint curve was not actually clipped to this loop's trim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn splice_boundary_vertex(body: &mut Body, loop_id: LoopId, target: VertexId, position: Pnt3, tol: f64, rec: &mut OpRecorder) -> Result<(), KernelError> {
    let linear = tol.max(1e-12);
    for cid in body.loop_coedges(loop_id) {
        let Some((v0, v1)) = body.coedge_endpoints(cid) else { continue };
        if v0 == target || v1 == target {
            return Ok(());
        }
    }
    for cid in body.loop_coedges(loop_id) {
        let Some(coedge) = body.coedges.get(cid) else { continue };
        let edge_id = coedge.edge;
        let edge = body.edges.get(edge_id).ok_or_else(|| KernelError::MissingEntity(format!("edge {edge_id}")))?.clone();
        let p0 = body.vertices.get(edge.v0).map(|v| v.position);
        let p1 = body.vertices.get(edge.v1).map(|v| v.position);
        if p0.is_some_and(|p| p.distance(position) <= linear) || p1.is_some_and(|p| p.distance(position) <= linear) {
            // an endpoint coincides with `position` but under a *different* vertex id than
            // `target` — a genuine coordinate collision the caller should resolve upstream
            // (reuse the existing id instead of minting `target`); nothing to splice here.
            continue;
        }
        let Ok(curve_t) = edge_param_at_point(body, &edge, position, linear) else { continue };
        if curve_t > edge.range.0 + 1e-14 && curve_t < edge.range.1 - 1e-14 {
            split_edge_with_vertex(body, edge_id, curve_t, target, rec);
            return Ok(());
        }
    }
    Err(KernelError::Operation("imprint point does not lie on the face's boundary loop".into()))
}

/// 🖋️ The oriented start/end vertices of one [`ParametricEdge`] traversal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn oriented_endpoints(body: &Body, member: ParametricEdge) -> Result<(VertexId, VertexId), KernelError> {
    let edge = body.edges.get(member.0).ok_or_else(|| KernelError::MissingEntity(format!("edge {}", member.0)))?;
    Ok(if member.1 { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) })
}

/// 🖋️ The `(u, v)` a [`ParametricEdge`] traversal starts (`at_start`) or ends at, read from its own
/// p-curve — which is stored in the EDGE's order, so the traversal's own start is `prange.1` when
/// the member runs backwards.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn member_uv(body: &Body, member: ParametricEdge, at_start: bool) -> Option<Pnt2> {
    let pcurve = body.curves2.get(member.2?)?;
    let (t0, t1) = member.3;
    Some(pcurve.eval(if at_start == member.1 { t0 } else { t1 }))
}

/// 🖋️ Ring position (index into [`loop_walk_pc`]'s walk) at which the boundary is at `uv`.
///
/// Matching by VERTEX id alone is not enough on a periodic face: the seam vertex of a sphere,
/// cylinder, cone or torus legitimately appears TWICE on its own ring — once at `u = 0`, once at
/// `u = 2π` — and a pole vertex likewise. Taking the first occurrence (what this used to do) picks
/// one of those at random, and half the time partitions the face the wrong way round: the piece on
/// the `u > 0` side gets handed the `u = 2π` boundary, so it comes out with the reversed traversal
/// and every edge it shares with the other operand reads `orientation-inconsistent`. The p-curve
/// says unambiguously which occurrence the chord actually meets.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ring_index_at_uv(body: &Body, (verts, members): (&[VertexId], &[ParametricEdge]), target: VertexId, uv: Option<Pnt2>) -> Option<usize> {
    let candidates: Vec<usize> = (0..verts.len()).filter(|&i| verts[i] == target).collect();
    match (candidates.len(), uv) {
        (0, _) => None,
        (1, _) | (_, None) => candidates.first().copied(),
        (_, Some(uv)) => candidates.into_iter().min_by(|&left, &right| {
            let d = |i: usize| member_uv(body, members[i], true).map_or(f64::INFINITY, |p| (p.x - uv.x).hypot(p.y - uv.y));
            d(left).partial_cmp(&d(right)).unwrap_or(std::cmp::Ordering::Equal)
        }),
    }
}

/// 🖋️ Splits whichever ring coedge's p-curve passes through `uv` so the boundary has a vertex
/// THERE, then defers to [`splice_boundary_vertex`] when no p-curve match is found.
///
/// The 3D-only splice cannot express a chord that ends on a DEGENERATE edge — a sphere's pole,
/// where the whole `u` range collapses to one point. Every point of that edge is the same
/// position, so `edge_param_at_point` has nothing to solve and `splice_boundary_vertex` reports the
/// vertex "already on the ring" and stops, leaving the chord's end at whichever of the pole's two
/// ring occurrences happens to come first — neither of which is where the chord actually arrives.
/// In `(u, v)` the pole edge is an ordinary segment and the arrival point is an ordinary interior
/// parameter on it, so splitting there is well defined and gives the chord its own ring position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn splice_boundary_vertex_at_uv(body: &mut Body, loop_id: LoopId, target: VertexId, position: Pnt3, uv: Option<Pnt2>, tol: f64, rec: &mut OpRecorder) -> Result<(), KernelError> {
    let Some(uv) = uv else { return splice_boundary_vertex(body, loop_id, target, position, tol, rec) };
    const SAMPLES: usize = 256;
    let mut best: Option<(CoedgeId, f64, f64)> = None;
    for cid in body.loop_coedges(loop_id) {
        let Some(coedge) = body.coedges.get(cid).cloned() else { continue };
        let Some(pcurve) = coedge.pcurve.and_then(|id| body.curves2.get(id)).cloned() else { continue };
        for i in 0..=SAMPLES {
            let s = i as f64 / SAMPLES as f64;
            let sample = pcurve.eval(coedge.prange.0 + (coedge.prange.1 - coedge.prange.0) * s);
            let distance = (sample.x - uv.x).hypot(sample.y - uv.y);
            if best.is_none_or(|(_, _, d)| distance < d) {
                best = Some((cid, s, distance));
            }
        }
    }
    let Some((cid, s, distance)) = best else { return splice_boundary_vertex(body, loop_id, target, position, tol, rec) };
    let Some(coedge) = body.coedges.get(cid).cloned() else { return splice_boundary_vertex(body, loop_id, target, position, tol, rec) };
    let Some(edge) = body.edges.get(coedge.edge).cloned() else { return splice_boundary_vertex(body, loop_id, target, position, tol, rec) };
    let span = edge.range.1 - edge.range.0;
    let curve_t = edge.range.0 + span * s;
    if distance > tol.max(1e-9) * 1e3 || !(curve_t > edge.range.0 + 1e-12 && curve_t < edge.range.1 - 1e-12) {
        return splice_boundary_vertex(body, loop_id, target, position, tol, rec);
    }
    if (edge.v0 == target || edge.v1 == target) && !is_point_edge_geometry(body, &edge) {
        return splice_boundary_vertex(body, loop_id, target, position, tol, rec);
    }
    split_edge_with_vertex(body, coedge.edge, curve_t, target, rec);
    Ok(())
}

/// 🖋️ The chord chain's own two end `(u, v)`, shifted into the same periodic branch the face's own
/// ring is expressed in.
///
/// A p-curve on a periodic surface is pinned down only modulo whole periods, and the branch a
/// p-curve was born in (wherever the surface inversion behind it happened to land) need not be the
/// branch the ring was written in — comparing the two unshifted picks the wrong ring occurrence and
/// partitions the face the wrong way round. One shift for the chain, chosen by putting its own
/// `u`/`v` centroid nearest the ring's, resolves that without ever having to guess an INDIVIDUAL
/// endpoint's branch, which is the part that genuinely cannot be guessed: `u = 0` and `u = 2π` are
/// the same point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chain_endpoint_uv(body: &Body, outer: LoopId, chain: &[ParametricEdge]) -> (Option<Pnt2>, Option<Pnt2>) {
    let (Some(&first), Some(&last)) = (chain.first(), chain.last()) else { return (None, None) };
    let (Some(start), Some(end)) = (member_uv(body, first, true), member_uv(body, last, false)) else { return (None, None) };
    let surface = body.loops.get(outer).and_then(|l| body.faces.get(l.face)).and_then(|f| body.surfaces.get(f.surface));
    let (u_periodic, v_periodic) = surface.map_or((false, false), |s| (s.is_u_periodic(), s.is_v_periodic()));
    let Ok((_, members)) = loop_walk_pc(body, outer) else { return (Some(start), Some(end)) };
    let gather = |group: &[ParametricEdge]| -> Vec<Pnt2> { group.iter().flat_map(|&m| [member_uv(body, m, true), member_uv(body, m, false)]).flatten().collect() };
    let (chain_uv, ring_uv) = (gather(chain), gather(&members));
    if chain_uv.is_empty() || ring_uv.is_empty() || !(u_periodic || v_periodic) {
        return (Some(start), Some(end));
    }
    let centroid = |points: &[Pnt2]| Pnt2::new(points.iter().map(|p| p.x).sum::<f64>() / points.len() as f64, points.iter().map(|p| p.y).sum::<f64>() / points.len() as f64);
    let (chain_center, ring_center) = (centroid(&chain_uv), centroid(&ring_uv));
    let tau = std::f64::consts::TAU;
    let du = if u_periodic { ((ring_center.x - chain_center.x) / tau).round() * tau } else { 0.0 };
    let dv = if v_periodic { ((ring_center.y - chain_center.y) / tau).round() * tau } else { 0.0 };
    (Some(Pnt2::new(start.x + du, start.y + dv)), Some(Pnt2::new(end.x + du, end.y + dv)))
}

/// 🖋️ `true` for an edge whose whole 3D range collapses to one point (a sphere's pole closer) —
/// the only kind [`splice_boundary_vertex_at_uv`] may split at a parameter its own endpoints
/// already carry the vertex for.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_point_edge_geometry(body: &Body, edge: &Edge) -> bool {
    if edge.v0 != edge.v1 {
        return false;
    }
    let Some(curve) = body.curves3.get(edge.curve) else { return false };
    let anchor = curve.eval(edge.range.0);
    (0..=8).all(|i| curve.eval(edge.range.0 + (edge.range.1 - edge.range.0) * f64::from(i) / 8.0).distance(anchor) <= edge.tol.value())
}

/// 🖋️ Generalizes [`split_planar_face_by_line`] to any surface/curve pair AND to a multi-segment
/// chord: splices the chain's two END vertices into `face`'s outer boundary and rebuilds the face
/// as two rings that share the whole chain, using 3D point-on-boundary matching (any curve kind,
/// via [`splice_boundary_vertex`]) instead of the planar line-projection the original used. Each
/// member carries THIS face's own p-curve for its edge, already in that edge's own curve order —
/// set on both traversals verbatim (never reversed, per the p-curve convention).
///
/// A chain rather than a single chord because one intersection segment need not reach the boundary
/// on its own: three planes cutting one sphere meet it in quarter arcs that touch the face
/// boundary at one end and EACH OTHER at the other, so only their concatenation is a legal chord.
/// The caller assembles maximal chains (see `diff::boolean`'s `chain_open_pendings`); a chain whose
/// two ends coincide is closed and belongs to [`split_face_by_interior_curve`] or
/// [`split_face_by_seam_crossing`] instead. Returns `(original_face, new_face)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_face_by_chain(body: &mut Body, face: FaceId, chain: &[ParametricEdge], tol: f64, rec: &mut OpRecorder) -> Result<(FaceId, FaceId), KernelError> {
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
    let outer = face_data.outer.ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    if !face_data.inners.is_empty() {
        return Err(KernelError::Operation("split_face_by_chain does not support faces with inner loops yet".into()));
    }
    let (Some(&first), Some(&last)) = (chain.first(), chain.last()) else {
        return Err(KernelError::Operation("imprint chain is empty".into()));
    };
    let (va, _) = oriented_endpoints(body, first)?;
    let (_, vb) = oriented_endpoints(body, last)?;
    if va == vb {
        return Err(KernelError::Operation("imprint chord has coincident endpoints".into()));
    }
    for pair in chain.windows(2) {
        let (_, end) = oriented_endpoints(body, pair[0])?;
        let (start, _) = oriented_endpoints(body, pair[1])?;
        if end != start {
            return Err(KernelError::Operation("imprint chain members are not end-to-end connected".into()));
        }
    }
    let pa = body.vertices.get(va).ok_or_else(|| KernelError::MissingEntity(format!("vertex {va}")))?.position;
    let pb = body.vertices.get(vb).ok_or_else(|| KernelError::MissingEntity(format!("vertex {vb}")))?.position;
    let (uv_a, uv_b) = chain_endpoint_uv(body, outer, chain);
    splice_boundary_vertex_at_uv(body, outer, va, pa, uv_a, tol, rec)?;
    splice_boundary_vertex_at_uv(body, outer, vb, pb, uv_b, tol, rec)?;

    let (verts, members) = loop_walk_pc(body, outer)?;
    let ia = ring_index_at_uv(body, (&verts, &members), va, uv_a).ok_or_else(|| KernelError::Operation("chord endpoint A missing from outer loop after splice".into()))?;
    let ib = ring_index_at_uv(body, (&verts, &members), vb, uv_b).ok_or_else(|| KernelError::Operation("chord endpoint B missing from outer loop after splice".into()))?;
    let chain_ab = member_chain_pc(&members, ia, ib);
    let chain_ba = member_chain_pc(&members, ib, ia);
    if chain_ab.is_empty() || chain_ba.is_empty() {
        return Err(KernelError::Operation("imprint edge does not partition the outer loop into two non-empty chains".into()));
    }

    let mut members_a = chain_ab; // va -> vb along the boundary
    members_a.extend(chain.iter().rev().map(|&(edge, forward, pcurve, prange)| (edge, !forward, pcurve, prange))); // vb -> va along the chain
    let mut members_b = chain_ba; // vb -> va along the boundary
    members_b.extend_from_slice(chain); // va -> vb along the chain

    for cid in body.loop_coedges(outer) {
        body.coedges.remove(cid);
    }
    body.loops.remove(outer);

    let loop_a = make_loop_pc(body, face, &members_a);
    {
        let f = body.faces.get_mut(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?;
        f.outer = Some(loop_a);
        f.inners.clear();
    }

    let loop_b = make_loop_pc(body, FaceId::from_raw(0, 0), &members_b);
    let face_b = add_face(body, face_data.surface, Some(loop_b), vec![], face_data.flipped, face_data.tol, rec);
    body.loops.get_mut(loop_b).ok_or_else(|| KernelError::MissingEntity(format!("loop {loop_b}")))?.face = face_b;

    for (_, shell) in body.shells.iter_mut() {
        if shell.faces.contains(&face) && !shell.faces.contains(&face_b) {
            shell.faces.push(face_b);
        }
    }

    Ok((face, face_b))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn set_chord_pcurve(body: &mut Body, loop_id: LoopId, edge_id: EdgeId, pcurve: Curve2Id, prange: (f64, f64)) {
    for cid in body.loop_coedges(loop_id) {
        if let Some(co) = body.coedges.get_mut(cid) {
            if co.edge == edge_id {
                co.pcurve = Some(pcurve);
                co.prange = prange;
            }
        }
    }
}

/// 🖋️ Imprints a CLOSED intersection curve that lies entirely inside `face`'s trim (never touches
/// the outer boundary): `edge_id` is a full-period self-referential edge (`v0 == v1`, one vertex,
/// the sphere-pole/torus-seam pattern). Adds it as a new inner loop (hole) on the original face
/// and builds a second face on the SAME surface whose outer loop is the same edge reversed — the
/// region the loop bounds. Orientation of the two new rings is chosen from the signed UV area of
/// `pcurve` so the hole is a genuine subtraction (opposite winding from the original outer loop)
/// and the new face's own outer ring is a normal positively-wound boundary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_face_by_interior_curve(body: &mut Body, face: FaceId, edge_id: EdgeId, pcurve_id: Curve2Id, prange: (f64, f64), rec: &mut OpRecorder) -> Result<(FaceId, FaceId), KernelError> {
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
    let outer = face_data.outer.ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    let pcurve = body.curves2.get(pcurve_id).ok_or_else(|| KernelError::MissingEntity(format!("curve2 {pcurve_id}")))?.clone();
    let outer_sign = loop_uv_signed_area(body, outer).signum();
    let loop_sign = |forward: bool| -> f64 {
        let n = 24usize;
        let (t0, t1) = if forward { prange } else { (prange.1, prange.0) };
        let mut area = 0.0;
        let mut prev = pcurve.eval(t0);
        for i in 1..=n {
            let s = i as f64 / n as f64;
            let cur = pcurve.eval(t0 + (t1 - t0) * s);
            area += prev.x * cur.y - cur.x * prev.y;
            prev = cur;
        }
        area
    };
    let hole_forward = outer_sign == 0.0 || loop_sign(true).signum() != outer_sign;

    let hole_loop = make_loop(body, face, &[(edge_id, hole_forward)]);
    set_chord_pcurve(body, hole_loop, edge_id, pcurve_id, prange);
    body.faces.get_mut(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.inners.push(hole_loop);

    let new_outer = make_loop(body, FaceId::from_raw(0, 0), &[(edge_id, !hole_forward)]);
    set_chord_pcurve(body, new_outer, edge_id, pcurve_id, prange);
    let new_face = add_face(body, face_data.surface, Some(new_outer), vec![], face_data.flipped, face_data.tol, rec);
    body.loops.get_mut(new_outer).ok_or_else(|| KernelError::MissingEntity(format!("loop {new_outer}")))?.face = new_face;

    for (_, shell) in body.shells.iter_mut() {
        if shell.faces.contains(&face) && !shell.faces.contains(&new_face) {
            shell.faces.push(new_face);
        }
    }

    Ok((face, new_face))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_uv_signed_area(body: &Body, loop_id: LoopId) -> f64 {
    let mut area = 0.0;
    let mut prev: Option<Pnt2> = None;
    let mut first: Option<Pnt2> = None;
    for cid in body.loop_coedges(loop_id) {
        let Some(co) = body.coedges.get(cid) else { continue };
        let Some(pc_id) = co.pcurve else { continue };
        let Some(pc) = body.curves2.get(pc_id) else { continue };
        let (t0, t1) = if co.forward { co.prange } else { (co.prange.1, co.prange.0) };
        let p0 = pc.eval(t0);
        if first.is_none() {
            first = Some(p0);
        }
        if let Some(pr) = prev {
            area += pr.x * p0.y - p0.x * pr.y;
        }
        prev = Some(pc.eval(t1));
    }
    if let (Some(pr), Some(f0)) = (prev, first) {
        area += pr.x * f0.y - f0.x * pr.y;
    }
    area * 0.5
}

/// 🖋️ Imprints a CLOSED periodic curve that grazes a boundary edge used TWICE within the same
/// loop at the SAME physical point (the canonical case: a full latitude circle on a single-seam
/// cylinder/cone/sphere/torus face, which spans the entire periodic width and so touches the seam
/// edge's both occurrences at once) — genuinely different from [`split_face_by_interior_curve`]'s
/// "small hole in the middle of the face" shape: the curve doesn't bound a sub-region, it
/// separates the WHOLE face into two pieces along the periodic direction, exactly like
/// [`split_face_by_chain`] except both chord endpoints are the SAME vertex, appearing twice in the
/// ring once spliced in. `edge_id` must be a closed (`v0 == v1`) edge (same shape
/// [`split_face_by_interior_curve`] uses); [`splice_boundary_vertex`] inserts that one vertex into
/// the loop — because it splits by EDGE id, not by coedge, one call updates BOTH occurrences of
/// the doubly-used boundary edge in a single pass — and the vertex then appears at exactly two
/// positions in the ring, giving the two chains this function splits between.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_face_by_seam_crossing(body: &mut Body, face: FaceId, edge_id: EdgeId, pcurve: Curve2Id, prange: (f64, f64), tol: f64, rec: &mut OpRecorder) -> Result<(FaceId, FaceId), KernelError> {
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
    let outer = face_data.outer.ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    if !face_data.inners.is_empty() {
        return Err(KernelError::Operation("split_face_by_seam_crossing does not support faces with inner loops yet".into()));
    }
    let new_edge = body.edges.get(edge_id).ok_or_else(|| KernelError::MissingEntity(format!("edge {edge_id}")))?.clone();
    let v = new_edge.v0;
    if new_edge.v1 != v {
        return Err(KernelError::Operation("split_face_by_seam_crossing requires a closed (v0 == v1) imprint edge".into()));
    }
    let position = body.vertices.get(v).ok_or_else(|| KernelError::MissingEntity(format!("vertex {v}")))?.position;
    splice_boundary_vertex(body, outer, v, position, tol, rec)?;

    let (verts, members) = loop_walk_pc(body, outer)?;
    let occurrences: Vec<usize> = verts.iter().enumerate().filter(|&(_, &vv)| vv == v).map(|(i, _)| i).collect();
    if occurrences.len() != 2 {
        return Err(KernelError::Operation(format!("seam-crossing split expected the imprint vertex to appear exactly twice on the boundary after splicing (got {})", occurrences.len())));
    }
    let (ia, ib) = (occurrences[0], occurrences[1]);
    let chain_ab = member_chain_pc(&members, ia, ib);
    let chain_ba = member_chain_pc(&members, ib, ia);
    if chain_ab.is_empty() || chain_ba.is_empty() {
        return Err(KernelError::Operation("seam-crossing edge does not partition the outer loop into two non-empty chains".into()));
    }

    // Which way round the closed imprint edge runs is NOT free: it has to continue from where the
    // boundary chain it closes left off. Topology cannot say — a closed edge has `v0 == v1`, so
    // both senses "connect" — but the p-curve can, and it must, because getting it wrong hands one
    // of the two pieces a reversed ring, which then traverses every edge it shares with the other
    // operand the same way that operand does (`orientation-inconsistent`) and leaves a UV boundary
    // that runs backwards through itself. Fixing it to `false`/`true` was right half the time.
    let junction = member_uv(body, members[ib], true);
    let forward_a = match (junction, body.curves2.get(pcurve)) {
        (Some(uv), Some(pc)) => {
            let reach = |t: f64| { let p = pc.eval(t); (p.x - uv.x).hypot(p.y - uv.y) };
            reach(prange.0) <= reach(prange.1)
        }
        _ => false,
    };
    let mut members_a = chain_ab;
    members_a.push((edge_id, forward_a, Some(pcurve), prange));
    let mut members_b = chain_ba;
    members_b.push((edge_id, !forward_a, Some(pcurve), prange));

    for cid in body.loop_coedges(outer) {
        body.coedges.remove(cid);
    }
    body.loops.remove(outer);

    let loop_a = make_loop_pc(body, face, &members_a);
    {
        let f = body.faces.get_mut(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?;
        f.outer = Some(loop_a);
        f.inners.clear();
    }

    let loop_b = make_loop_pc(body, FaceId::from_raw(0, 0), &members_b);
    let face_b = add_face(body, face_data.surface, Some(loop_b), vec![], face_data.flipped, face_data.tol, rec);
    body.loops.get_mut(loop_b).ok_or_else(|| KernelError::MissingEntity(format!("loop {loop_b}")))?.face = face_b;

    for (_, shell) in body.shells.iter_mut() {
        if shell.faces.contains(&face) && !shell.faces.contains(&face_b) {
            shell.faces.push(face_b);
        }
    }

    Ok((face, face_b))
}

/// 🖋️ Merges two faces sharing exactly one edge `edge_id` (each using it once, on their own outer
/// loop, in opposite `forward` sense — the classic post-boolean "coincident/adjacent faces on the
/// same surface become one" merge) into a single face: splices the two rings together, dropping
/// the shared edge, and removes the second face (`face_b`) from every shell that references it.
/// Returns the surviving face id (`face_a`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kill_edge_merge_faces(body: &mut Body, face_a: FaceId, face_b: FaceId, edge_id: EdgeId, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    let coedges = body.edge_coedges(edge_id);
    if coedges.len() != 2 {
        return Err(KernelError::Operation(format!("kill_edge_merge_faces requires an edge used by exactly 2 coedges (got {})", coedges.len())));
    }
    let outer_a = body.faces.get(face_a).ok_or_else(|| KernelError::MissingEntity(format!("face {face_a}")))?.outer.ok_or_else(|| KernelError::Operation("face_a has no outer loop".into()))?;
    let face_b_data = body.faces.get(face_b).ok_or_else(|| KernelError::MissingEntity(format!("face {face_b}")))?.clone();
    let outer_b = face_b_data.outer.ok_or_else(|| KernelError::Operation("face_b has no outer loop".into()))?;
    let ca = coedges.iter().copied().find(|&c| body.coedges.get(c).map(|co| co.loop_id) == Some(outer_a)).ok_or_else(|| KernelError::Operation("edge not on face_a's outer loop".into()))?;
    let cb = coedges.iter().copied().find(|&c| body.coedges.get(c).map(|co| co.loop_id) == Some(outer_b)).ok_or_else(|| KernelError::Operation("edge not on face_b's outer loop".into()))?;
    let coa = body.coedges.get(ca).unwrap().clone();
    let cob = body.coedges.get(cb).unwrap().clone();
    if coa.forward == cob.forward {
        return Err(KernelError::Operation("kill_edge_merge_faces requires opposite-orientation coedges".into()));
    }

    let (_, members_a) = loop_walk(body, outer_a)?;
    let (_, members_b) = loop_walk(body, outer_b)?;
    let ia = members_a.iter().position(|&(e, _)| e == edge_id).ok_or_else(|| KernelError::Operation("edge missing from face_a walk".into()))?;
    let ib = members_b.iter().position(|&(e, _)| e == edge_id).ok_or_else(|| KernelError::Operation("edge missing from face_b walk".into()))?;
    let na = members_a.len();
    let nb = members_b.len();
    let mut merged: Vec<(EdgeId, bool)> = Vec::with_capacity(na + nb - 2);
    for k in 1..na {
        merged.push(members_a[(ia + k) % na]);
    }
    for k in 1..nb {
        merged.push(members_b[(ib + k) % nb]);
    }
    if merged.len() < 3 {
        return Err(KernelError::Operation("merged ring has fewer than 3 sides".into()));
    }

    for cid in body.loop_coedges(outer_a) {
        body.coedges.remove(cid);
    }
    body.loops.remove(outer_a);
    for cid in body.loop_coedges(outer_b) {
        body.coedges.remove(cid);
    }
    body.loops.remove(outer_b);
    body.edges.remove(edge_id);
    rec.record_deleted(face_b_data.label);

    let merged_loop = make_loop(body, face_a, &merged);
    let f = body.faces.get_mut(face_a).ok_or_else(|| KernelError::MissingEntity(format!("face {face_a}")))?;
    f.outer = Some(merged_loop);

    for (_, shell) in body.shells.iter_mut() {
        shell.faces.retain(|&f| f != face_b);
    }
    body.faces.remove(face_b);
    Ok(face_a)
}

// #endregion 🔖️Imprint

// #region 🔖️UvArrange

#[derive(Clone, Copy)]
struct BoundaryHit {
    edge: EdgeId,
    curve_t: f64,
    point: Pnt3,
    vertex_hint: Option<VertexId>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn project_uv(frame: &Frame3, p: Pnt3) -> Pnt2 {
    let local = frame.to_local(p);
    Pnt2::new(local.x, local.y)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn intersect_segment_line_uv(a: Pnt2, b: Pnt2, p0: Pnt2, p1: Pnt2, tol: f64) -> Option<(f64, Pnt2)> {
    let r = b - a;
    let s = p1 - p0;
    let rxs = r.cross(s);
    if rxs.abs() <= tol * tol.max(1.0) {
        return None;
    }
    let q = p0 - a;
    let t = q.cross(s) / rxs;
    let edge_len = r.norm();
    if edge_len <= tol {
        return None;
    }
    let param_tol = (tol / edge_len).clamp(1e-14, 0.25);
    if t < -param_tol || t > 1.0 + param_tol {
        return None;
    }
    let t_clamped = t.clamp(0.0, 1.0);
    let point = a + r * t_clamped;
    Some((t_clamped, point))
}

/// ✂️ Curve parameter at `point` (assumed to already lie on `edge`'s curve, within `tol`).
/// `Line` uses an exact projection; every other kind uses [`closest_parameter`]'s
/// certified/analytic closest-point (exact for `Circle`/`Ellipse`, Newton-refined for `Nurbs`).
/// 🐛 Previously a fixed 32-sample uniform scan gated at `tol*10`: exact for a straight-edge
/// chord, but a great-circle/ellipse arc's own 32-sample spacing (~0.1 rad on a unit-radius
/// meridian) is orders of magnitude coarser than any reasonable boolean tolerance (`1e-6`),
/// silently rejecting genuine on-curve points — confirmed live via a sphere/sphere seam-touch
/// vertex, exactly on the seam's own [`Curve3::Circle`], failing this check every time.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_param_at_point(body: &Body, edge: &Edge, point: Pnt3, tol: f64) -> Result<f64, KernelError> {
    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve {}", edge.curve)))?;
    match curve {
        Curve3::Line { origin, dir } => {
            let denom = dir.norm_sq();
            if denom <= tol * tol {
                return Err(KernelError::Operation("degenerate edge line".into()));
            }
            // 🐛 The projection parameter alone says only WHERE on the infinite line the point
            // projects, never whether it is on that line at all — and `splice_boundary_vertex`
            // splits the first ring edge whose parameter lands strictly inside. A sphere's north
            // pole projects onto the box edge on the far side of the same face at an interior
            // parameter, so the imprint vertex was spliced into a face it is nowhere near (found
            // live in the sphere-box fuse: the pole landed on the `x = 1.5` face's own edge). The
            // non-line branch below has always checked its distance; this one must too.
            let t = dir.dot(point - *origin) / denom;
            if (*origin + *dir * t).distance(point) > tol.max(1e-9) * 10.0 {
                return Err(KernelError::Operation("imprint hit does not lie on this line edge".into()));
            }
            Ok(t)
        }
        _ => {
            let cp = closest_parameter(curve, edge.range, point, tol);
            if cp.distance > tol.max(1e-9) * 10.0 {
                return Err(KernelError::Operation("could not locate imprint hit on non-line edge".into()));
            }
            Ok(cp.t)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn endpoint_vertex(edge: &Edge, curve_t: f64, linear: f64) -> Option<VertexId> {
    let (t0, t1) = edge.range;
    let span = (t1 - t0).abs().max(1e-30);
    let param_tol = (linear / span).clamp(1e-14, 0.25);
    if (curve_t - t0).abs() <= param_tol * span || (curve_t - t0).abs() <= 1e-12 {
        return Some(edge.v0);
    }
    if (curve_t - t1).abs() <= param_tol * span || (curve_t - t1).abs() <= 1e-12 {
        return Some(edge.v1);
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_walk(body: &Body, loop_id: LoopId) -> Result<LoopWalk, KernelError> {
    let coedges = body.loop_coedges(loop_id);
    let mut verts = Vec::with_capacity(coedges.len());
    let mut members = Vec::with_capacity(coedges.len());
    for cid in coedges {
        let (start, _) = body.coedge_endpoints(cid).ok_or_else(|| KernelError::Operation(format!("coedge {cid} missing endpoints")))?;
        let coedge = body.coedges.get(cid).ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid}")))?;
        verts.push(start);
        members.push((coedge.edge, coedge.forward));
    }
    Ok((verts, members))
}

/// 🖋️ Like [`loop_walk`], but also captures each coedge's own `(pcurve, prange)` — needed by any
/// splitter that rebuilds a loop's ring via [`make_loop`] (which always mints fresh coedges with
/// `pcurve: None`): without this, a pre-existing chain member (e.g. a cylinder's own `e_bot`/
/// `e_top`/half-seam pieces, already carrying real p-curves from `🧱️primitives`) would silently
/// lose them on every split, later failing `check_missing_pcurves`/trim tests for no visible
/// reason. 🐛 This was a real bug here (not hypothetical): `split_face_by_chain`/
/// `split_face_by_seam_crossing` used to call plain `loop_walk` + `make_loop`, discarding every
/// chain member's p-curve — confirmed via live debug instrumentation (a post-split cylinder
/// lateral piece's own `sample_loop_uv` showed only ONE coedge surviving, because the other three
/// had silently gone pcurve-`None` and were skipped by every p-curve-only consumer).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_walk_pc(body: &Body, loop_id: LoopId) -> Result<(Vec<VertexId>, Vec<ParametricEdge>), KernelError> {
    let coedges = body.loop_coedges(loop_id);
    let mut verts = Vec::with_capacity(coedges.len());
    let mut members = Vec::with_capacity(coedges.len());
    for cid in coedges {
        let (start, _) = body.coedge_endpoints(cid).ok_or_else(|| KernelError::Operation(format!("coedge {cid} missing endpoints")))?;
        let coedge = body.coedges.get(cid).ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid}")))?;
        verts.push(start);
        members.push((coedge.edge, coedge.forward, coedge.pcurve, coedge.prange));
    }
    Ok((verts, members))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn member_chain_pc(members: &[ParametricEdge], from: usize, to: usize) -> Vec<ParametricEdge> {
    let n = members.len();
    let mut out = Vec::new();
    let mut i = from;
    while i != to {
        out.push(members[i]);
        i = (i + 1) % n;
        if out.len() > n {
            break;
        }
    }
    out
}

/// 🖋️ [`make_loop`] plus restoring each member's own `(pcurve, prange)` — see [`loop_walk_pc`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn make_loop_pc(body: &mut Body, face: FaceId, members: &[ParametricEdge]) -> LoopId {
    let plain: Vec<(EdgeId, bool)> = members.iter().map(|&(e, f, _, _)| (e, f)).collect();
    let loop_id = make_loop(body, face, &plain);
    for (cid, &(_, _, pc, pr)) in body.loop_coedges(loop_id).into_iter().zip(members.iter()) {
        if let Some(co) = body.coedges.get_mut(cid) {
            co.pcurve = pc;
            co.prange = pr;
        }
    }
    loop_id
}

// #endregion 🔖️UvArrange

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_edge_containing_param(body: &Body, e1: EdgeId, e2: EdgeId, t: f64) -> Result<EdgeId, KernelError> {
    for edge_id in [e1, e2] {
        let Some(edge) = body.edges.get(edge_id) else { continue };
        if t > edge.range.0 + 1e-14 && t < edge.range.1 - 1e-14 {
            return Ok(edge_id);
        }
    }
    Err(KernelError::Operation("same-edge double hit: second parameter not found on either survivor".into()))
}

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
