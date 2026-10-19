//! 🏗️ Topology editors: the vertex, edge, face, shell and solid makers, the solid scaffold retirement, the edge splits and the planar face split by a line. See [`super`].

use super::*;

// #region 🔖️Make
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

/// ✂️ Retires a solid and its shells while every face they carried lives on — the exact inverse of
/// [`add_solid`]/[`add_shell`] for an operator that built a solid only as a SCAFFOLD to carry
/// faces it then hands to a different solid. Faces, loops, edges and vertices are untouched: the
/// caller owns them now.
///
/// A scaffold left behind is not inert. It is a second, independent solid that
/// [`semio_framework_3d_brep_validation::validate_body`]
/// walks like any other — so when `↔️offset::shell_solid` flips every one of its faces to build
/// the cavity, the scaffold's own outer shell integrates to −V and the body carries a permanent
/// `shell-orientation-inward` issue. `Brep::validate_gate_sync` validates the WHOLE body, so that
/// one orphan refuses every preview taken from that kernel afterwards, not just the shelled one
/// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, `🐚️box-shell-preview`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn retire_solid_scaffold(body: &mut Body, solid: SolidId, rec: &mut OpRecorder) {
    let Some(data) = body.solids.get(solid).cloned() else { return };
    for shell in std::iter::once(data.outer).chain(data.inners.iter().copied()) {
        if let Some(label) = body.shells.get(shell).map(|s| s.label) {
            rec.record_deleted(label);
        }
        body.shells.remove(shell);
    }
    rec.record_deleted(data.label);
    body.solids.remove(solid);
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
/// (prange.1-prange.0)·s`, always in the edge's own curve order per [`crate::primitives`]'s
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
        let coedge_e1 = body.coedges.insert(Coedge { edge: e1, forward: coedge.forward, pcurve: pcurve1, prange: prange1, loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
        let coedge_e2 = body.coedges.insert(Coedge { edge: e2, forward: coedge.forward, pcurve: pcurve2, prange: prange2, loop_id: coedge.loop_id, next: dummy_coedge(), prev: dummy_coedge() });
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
