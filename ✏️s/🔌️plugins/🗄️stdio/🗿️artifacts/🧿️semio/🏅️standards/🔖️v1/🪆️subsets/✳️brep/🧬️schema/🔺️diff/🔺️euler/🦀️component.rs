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

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, CoedgeId, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, SurfaceId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Coedge, Edge, Face, Loop, Shell, Solid, Vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3};
#[cfg(test)]
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3;

// #region 🔖️Make

async fn dummy_coedge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}

/// ✂️ Creates a new vertex, recording it as generated.
pub async fn make_vertex(body: &mut Body, position: Pnt3, tol: Tol, rec: &mut OpRecorder) -> VertexId {
    let label = body.new_label();
    rec.record_generated(label);
    body.vertices.insert(Vertex { position, tol, label })
}

/// ✂️ Creates a new edge referencing shared curve geometry, recording it as generated.
pub async fn make_edge(body: &mut Body, curve: Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId, tol: Tol, rec: &mut OpRecorder) -> EdgeId {
    let label = body.new_label();
    rec.record_generated(label);
    body.edges.insert(Edge { curve, range, v0, v1, tol, label })
}

/// ✂️ Builds a closed coedge ring from `members` (one `(edge, forward)` pair per coedge, in ring
/// order) and links it into a new [`Loop`]. Loops/coedges have no [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::PersistentLabel`]
/// of their own (they are structural, not independently document-nameable), so nothing is recorded.
pub async fn make_loop(body: &mut Body, face: FaceId, members: &[(EdgeId, bool)]) -> LoopId {
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
pub async fn add_face(body: &mut Body, surface: SurfaceId, outer: Option<LoopId>, inners: Vec<LoopId>, flipped: bool, tol: Tol, rec: &mut OpRecorder) -> FaceId {
    let label = body.new_label();
    rec.record_generated(label);
    body.faces.insert(Face { surface, outer, inners, flipped, tol, label })
}

/// ✂️ Creates a new shell, recording it as generated.
pub async fn add_shell(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> ShellId {
    let label = body.new_label();
    rec.record_generated(label);
    body.shells.insert(Shell { faces, label })
}

/// ✂️ Creates a new solid, recording it as generated.
pub async fn add_solid(body: &mut Body, outer: ShellId, inners: Vec<ShellId>, rec: &mut OpRecorder) -> SolidId {
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
pub async fn split_edge(body: &mut Body, edge_id: EdgeId, t: f64, position: Pnt3, rec: &mut OpRecorder) -> (EdgeId, EdgeId, VertexId) {
    let old_edge = body.edges.get(edge_id).expect("split_edge requires a live edge id").clone();
    debug_assert!(t > old_edge.range.0 && t < old_edge.range.1, "split parameter must lie strictly within the edge's range");
    let new_vertex = make_vertex(body, position, old_edge.tol, rec);
    let e1 = make_edge(body, old_edge.curve, (old_edge.range.0, t), old_edge.v0, new_vertex, old_edge.tol, rec);
    let e2 = make_edge(body, old_edge.curve, (t, old_edge.range.1), new_vertex, old_edge.v1, old_edge.tol, rec);
    let affected: Vec<CoedgeId> = body.edge_coedges(edge_id);
    for coedge_id in affected {
        let coedge = body.coedges.get(coedge_id).unwrap().clone();
        let (first_edge, second_edge) = if coedge.forward { (e1, e2) } else { (e2, e1) };
        let self_loop = coedge.prev == coedge_id && coedge.next == coedge_id;
        let c1 = body.coedges.insert(Coedge { edge: first_edge, forward: coedge.forward, pcurve: None, prange: (0.0, 0.0), loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        let c2 = body.coedges.insert(Coedge { edge: second_edge, forward: coedge.forward, pcurve: None, prange: (0.0, 0.0), loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
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
    (e1, e2, new_vertex)
}

// #endregion 🔖️SplitJoin

// #region 🔖️Api

/// 🖋️ Split a planar face along the line through `p0` and `p1`.
///
/// Intersects the infinite line with the outer boundary in the face UV plane, splits the two hit
/// edges (or reuses existing vertices when the hit lands on a corner), inserts a chord edge, and
/// rebuilds two outer loops via Euler editors. Returns `(original_face, new_face)`.
pub async fn split_planar_face_by_line(body: &mut Body, face: FaceId, p0: Pnt3, p1: Pnt3) -> Result<(FaceId, FaceId), KernelError> {
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

    let (verts, members) = loop_walk(body, outer)?;
    let ia = verts.iter().position(|&v| v == va).ok_or_else(|| KernelError::Operation("split vertex A missing from outer loop".into()))?;
    let ib = verts.iter().position(|&v| v == vb).ok_or_else(|| KernelError::Operation("split vertex B missing from outer loop".into()))?;
    let chain_ab = member_chain(&members, ia, ib);
    let chain_ba = member_chain(&members, ib, ia);
    if chain_ab.is_empty() || chain_ba.is_empty() {
        return Err(KernelError::Operation("cutting line does not partition the outer loop into two non-empty chains".into()));
    }

    let pa = body.vertices.get(va).expect("vertex A").position;
    let pb = body.vertices.get(vb).expect("vertex B").position;
    let cut_curve = body.curves3.insert(Curve3::Line { origin: pa, dir: pb - pa });
    let cut = make_edge(body, cut_curve, (0.0, 1.0), va, vb, tol, &mut rec);

    let mut members_a = chain_ab;
    members_a.push((cut, false));
    let mut members_b = chain_ba;
    members_b.push((cut, true));

    for cid in body.loop_coedges(outer) {
        body.coedges.remove(cid);
    }
    body.loops.remove(outer);

    let loop_a = make_loop(body, face, &members_a);
    {
        let f = body.faces.get_mut(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?;
        f.outer = Some(loop_a);
        f.inners.clear();
    }

    let loop_b = make_loop(body, FaceId::from_raw(0, 0), &members_b);
    let face_b = add_face(body, face_data.surface, Some(loop_b), vec![], face_data.flipped, tol, &mut rec);
    body.loops.get_mut(loop_b).ok_or_else(|| KernelError::MissingEntity(format!("loop {loop_b}")))?.face = face_b;

    for (_, shell) in body.shells.iter_mut() {
        if shell.faces.iter().any(|&f| f == face) && !shell.faces.iter().any(|&f| f == face_b) {
            shell.faces.push(face_b);
        }
    }

    Ok((face, face_b))
}

// #endregion 🔖️Api

// #region 🔖️UvArrange

#[derive(Clone, Copy)]
struct BoundaryHit {
    edge: EdgeId,
    curve_t: f64,
    point: Pnt3,
    vertex_hint: Option<VertexId>,
}

async fn project_uv(frame: &Frame3, p: Pnt3) -> Pnt2 {
    let local = frame.to_local(p);
    Pnt2::new(local.x, local.y)
}

async fn intersect_segment_line_uv(a: Pnt2, b: Pnt2, p0: Pnt2, p1: Pnt2, tol: f64) -> Option<(f64, Pnt2)> {
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

async fn edge_param_at_point(body: &Body, edge: &Edge, point: Pnt3, tol: f64) -> Result<f64, KernelError> {
    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve {}", edge.curve)))?;
    match curve {
        Curve3::Line { origin, dir } => {
            let denom = dir.norm_sq();
            if denom <= tol * tol {
                return Err(KernelError::Operation("degenerate edge line".into()));
            }
            Ok(dir.dot(point - *origin) / denom)
        }
        _ => {
            let (t0, t1) = edge.range;
            let mut best_t = 0.5 * (t0 + t1);
            let mut best_d = f64::INFINITY;
            for i in 0..=32 {
                let u = i as f64 / 32.0;
                let t = t0 + (t1 - t0) * u;
                let d = curve.eval(t).distance(point);
                if d < best_d {
                    best_d = d;
                    best_t = t;
                }
            }
            if best_d > tol * 10.0 {
                return Err(KernelError::Operation("could not locate imprint hit on non-line edge".into()));
            }
            Ok(best_t)
        }
    }
}

async fn endpoint_vertex(edge: &Edge, curve_t: f64, linear: f64) -> Option<VertexId> {
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

async fn loop_walk(body: &Body, loop_id: LoopId) -> Result<(Vec<VertexId>, Vec<(EdgeId, bool)>), KernelError> {
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

async fn member_chain(members: &[(EdgeId, bool)], from: usize, to: usize) -> Vec<(EdgeId, bool)> {
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

// #endregion 🔖️UvArrange

async fn resolve_edge_containing_param(body: &Body, e1: EdgeId, e2: EdgeId, t: f64) -> Result<EdgeId, KernelError> {
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::{make_planar_face_from_wire, make_rectangle_wire};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;

    /// ✂️ Builds the topology of a unit tetrahedron (4 vertices, 6 edges, 4 triangular faces, 1
    /// shell, 1 solid) purely through the checked editors above — the flagship "assemble a real
    /// closed solid from scratch" gate for this phase.
    async fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
        let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
        let vertices: Vec<VertexId> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
        let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
        let mut edges = std::collections::HashMap::new();
        for &(a, b) in &edge_pairs {
            let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
            let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
            edges.insert((a, b), edge);
            edges.insert((b, a), edge);
        }
        // Four triangular faces of a tetrahedron with vertex indices 0,1,2,3.
        let face_defs = [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]];
        let mut faces = Vec::new();
        for tri in face_defs {
            let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
            let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
            let surface = body.surfaces.insert(Surface::Plane { frame });
            let members: Vec<(EdgeId, bool)> = (0..3)
                .map(|i| {
                    let a = tri[i];
                    let b = tri[(i + 1) % 3];
                    let edge = edges[&(a, b)];
                    let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                    (edge, forward)
                })
                .collect();
            let outer = make_loop(body, FaceId::from_raw(0, 0), &members);
            let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
            body.loops.get_mut(outer).unwrap().face = face;
            faces.push(face);
        }
        let shell = add_shell(body, faces, rec);
        add_solid(body, shell, vec![], rec)
    }

    #[semio_framework_async_macros::async_test]
    async fn tetrahedron_satisfies_euler_poincare_formula() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let vertex_count = body.vertices.len() as i64;
        let edge_count = body.edges.len() as i64;
        let face_count = body.solid_faces(solid).len() as i64;
        assert_eq!(vertex_count, 4);
        assert_eq!(edge_count, 6);
        assert_eq!(face_count, 4);
        assert_eq!(vertex_count - edge_count + face_count, 2, "V - E + F must equal 2 for a genus-0 closed solid");
    }

    #[semio_framework_async_macros::async_test]
    async fn tetrahedron_build_records_every_entity_as_generated() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        build_tetrahedron(&mut body, &mut rec);
        let delta = rec.into_delta();
        assert_eq!(delta.generated.len(), 4 + 6 + 4 + 1 + 1, "vertices + edges + faces + shell + solid");
        assert!(delta.deleted.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn each_face_loop_is_a_closed_ring_of_three_coedges() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        for face in body.solid_faces(solid) {
            let outer = body.faces.get(face).unwrap().outer.unwrap();
            assert_eq!(body.loop_coedges(outer).len(), 3);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn split_edge_on_a_free_edge_creates_two_edges_and_a_vertex() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let v1 = make_vertex(&mut body, Pnt3::new(4.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
        let edge = make_edge(&mut body, curve, (0.0, 4.0), v0, v1, Tol::DEFAULT, &mut rec);
        let (e1, e2, new_vertex) = split_edge(&mut body, edge, 1.5, Pnt3::new(1.5, 0.0, 0.0), &mut rec);
        assert!(!body.edges.contains(edge));
        assert_eq!(body.edges.get(e1).unwrap().v0, v0);
        assert_eq!(body.edges.get(e1).unwrap().v1, new_vertex);
        assert_eq!(body.edges.get(e2).unwrap().v0, new_vertex);
        assert_eq!(body.edges.get(e2).unwrap().v1, v1);
        assert_eq!(body.edges.get(e1).unwrap().range, (0.0, 1.5));
        assert_eq!(body.edges.get(e2).unwrap().range, (1.5, 4.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn split_edge_within_a_loop_ring_preserves_ring_validity() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_tetrahedron(&mut body, &mut rec);
        let face = body.solid_faces(solid)[0];
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let coedges_before = body.loop_coedges(outer);
        let target_coedge = coedges_before[0];
        let target_edge = body.coedges.get(target_coedge).unwrap().edge;
        let (t0, t1) = body.edges.get(target_edge).unwrap().range;
        let mid = 0.5 * (t0 + t1);
        let curve = body.edges.get(target_edge).unwrap().curve;
        let position = body.curves3.get(curve).unwrap().eval(mid);
        split_edge(&mut body, target_edge, mid, position, &mut rec);
        let coedges_after = body.loop_coedges(outer);
        assert_eq!(coedges_after.len(), coedges_before.len() + 1, "the ring gains exactly one coedge");
        // The ring must still be a single closed cycle covering every live coedge in the loop.
        let mut seen = std::collections::HashSet::new();
        for c in &coedges_after {
            assert!(seen.insert(*c), "ring must not repeat a coedge");
        }
        for c in &coedges_after {
            let co = body.coedges.get(*c).unwrap();
            assert!(coedges_after.contains(&co.next));
            assert!(coedges_after.contains(&co.prev));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn split_edge_on_a_self_referential_single_coedge_loop_produces_a_valid_two_coedge_ring() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let curve = body.curves3.insert(Curve3::Circle { frame, radius: 1.0 });
        let v = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let edge = make_edge(&mut body, curve, (0.0, std::f64::consts::TAU), v, v, Tol::DEFAULT, &mut rec);
        let loop_id = make_loop(&mut body, FaceId::from_raw(0, 0), &[(edge, true)]);
        let coedges_before = body.loop_coedges(loop_id);
        assert_eq!(coedges_before.len(), 1);
        let single = coedges_before[0];
        assert_eq!(body.coedges.get(single).unwrap().next, single);
        assert_eq!(body.coedges.get(single).unwrap().prev, single);
        split_edge(&mut body, edge, std::f64::consts::PI, Pnt3::new(-1.0, 0.0, 0.0), &mut rec);
        let coedges_after = body.loop_coedges(loop_id);
        assert_eq!(coedges_after.len(), 2);
        let a = body.coedges.get(coedges_after[0]).unwrap();
        let b = body.coedges.get(coedges_after[1]).unwrap();
        assert_eq!(a.next, coedges_after[1]);
        assert_eq!(b.next, coedges_after[0]);
        assert_eq!(a.prev, coedges_after[1]);
        assert_eq!(b.prev, coedges_after[0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn split_rectangle_face_into_two() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut body, 2.0, 2.0, &mut rec).unwrap();
        let face = make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).unwrap();
        let (f0, f1) = split_planar_face_by_line(&mut body, face, Pnt3::new(1.0, -1.0, 0.0), Pnt3::new(1.0, 3.0, 0.0)).unwrap();
        assert_ne!(f0, f1);
        assert!(body.faces.contains(f0));
        assert!(body.faces.contains(f1));
        let loop0 = body.faces.get(f0).unwrap().outer.unwrap();
        let loop1 = body.faces.get(f1).unwrap().outer.unwrap();
        assert_eq!(body.loop_coedges(loop0).len(), 4);
        assert_eq!(body.loop_coedges(loop1).len(), 4);
        assert_eq!(body.faces.len(), 2);
        assert_eq!(body.edges.len(), 7);
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
    }

    #[semio_framework_async_macros::async_test]
    async fn split_rejects_non_cutting_line() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut body, 2.0, 2.0, &mut rec).unwrap();
        let face = make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).unwrap();
        let err = split_planar_face_by_line(&mut body, face, Pnt3::new(3.0, 0.0, 0.0), Pnt3::new(3.0, 2.0, 0.0)).unwrap_err();
        assert!(matches!(err, KernelError::Operation(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn split_rejects_missing_face() {
        let mut body = Body::new();
        let err = split_planar_face_by_line(&mut body, FaceId::from_raw(0, 0), Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0)).unwrap_err();
        assert!(matches!(err, KernelError::MissingEntity(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn resolve_edge_containing_param_picks_survivor_after_split() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let face =
            crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(4.0, 0.0, 0.0), Pnt3::new(4.0, 2.0, 0.0), Pnt3::new(0.0, 2.0, 0.0)], &mut rec)
                .expect("rect face");
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        let edge = body.coedges.get(body.loop_coedges(outer)[0]).unwrap().edge;
        let e = body.edges.get(edge).unwrap().clone();
        let mid_t = (e.range.0 + e.range.1) * 0.5;
        let mid_p = body.curves3.get(e.curve).unwrap().eval(mid_t);
        let (e1, e2, _) = split_edge(&mut body, edge, mid_t, mid_p, &mut rec);
        let low_t = e.range.0 + (e.range.1 - e.range.0) * 0.25;
        let high_t = e.range.0 + (e.range.1 - e.range.0) * 0.75;
        let low_edge = resolve_edge_containing_param(&body, e1, e2, low_t).expect("low");
        let high_edge = resolve_edge_containing_param(&body, e1, e2, high_t).expect("high");
        assert_ne!(low_edge, high_edge);
    }
}

// #endregion 🔖️Tests
