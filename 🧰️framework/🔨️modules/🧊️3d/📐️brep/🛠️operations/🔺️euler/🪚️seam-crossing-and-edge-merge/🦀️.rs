//! 🪚️ Seam crossing and edge merge: the face split by seam crossing and the edge kill that merges the two faces. See [`super`].

use super::*;

// #region 🔖️Imprint
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
