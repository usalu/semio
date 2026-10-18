//! ⛓️ Face chain splits: the face splits by chain, interior curve and interior chain with their chord p-curve and signed area helpers. See [`super`].

use super::*;

// #region 🔖️Imprint
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

/// 🖋️ Imprints a CLOSED imprint expressed PIECEWISE — a cycle of open intersection segments that
/// lies entirely inside `face`'s trim and never touches its outer boundary (the canonical case: a
/// box tool passing clean through a box stock leaves the stock's own cross-section as a rectangle
/// of four line segments on each tool side face, and a square hole on the stock's top and bottom
/// faces). The same two-ring shape as [`split_face_by_interior_curve`] — the cycle becomes a new
/// inner loop (hole) on the original face and the outer ring of a second face on the same surface —
/// only with the ring walked over every member of `chain` instead of one self-closing edge. `chain`
/// must be end-to-end connected and closed (each member's `forward` flag oriented along the walk),
/// as [`crate::boolean`]'s chain assembly emits it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_face_by_interior_chain(body: &mut Body, face: FaceId, chain: &[ParametricEdge], rec: &mut OpRecorder) -> Result<(FaceId, FaceId), KernelError> {
    if chain.len() < 2 {
        return Err(KernelError::Operation(format!("interior chain on face {face} needs at least two members, got {}", chain.len())));
    }
    let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
    let outer = face_data.outer.ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    let (outer_sign, chain_sign) = match body.surfaces.get(face_data.surface).and_then(|surface| surface.normal(0.0, 0.0)) {
        // 🧭️ A ring on a plane (a box face) carries no p-curves, so its UV area reads 0; both windings
        // are measured in 3D about the plane's own normal instead — the same reference for both.
        Some(normal) if matches!(body.surfaces.get(face_data.surface), Some(Surface::Plane { .. })) => {
            (points_signed_area_about(&loop_3d_points(body, outer), normal).signum(), points_signed_area_about(&chain_3d_points(body, chain), normal).signum())
        }
        _ => (loop_uv_signed_area(body, outer).signum(), chain_uv_signed_area(body, chain).signum()),
    };
    let reversed: Vec<ParametricEdge> = chain.iter().rev().map(|&(e, f, pc, pr)| (e, !f, pc, pr)).collect();
    let (hole_members, new_outer_members) = if outer_sign == 0.0 || chain_sign != outer_sign { (chain, reversed.as_slice()) } else { (reversed.as_slice(), chain) };

    let hole_loop = make_loop_pc(body, face, hole_members);
    body.faces.get_mut(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.inners.push(hole_loop);

    let new_outer = make_loop_pc(body, FaceId::from_raw(0, 0), new_outer_members);
    let new_face = add_face(body, face_data.surface, Some(new_outer), vec![], face_data.flipped, face_data.tol, rec);
    body.loops.get_mut(new_outer).ok_or_else(|| KernelError::MissingEntity(format!("loop {new_outer}")))?.face = new_face;

    for (_, shell) in body.shells.iter_mut() {
        if shell.faces.contains(&face) && !shell.faces.contains(&new_face) {
            shell.faces.push(new_face);
        }
    }

    Ok((face, new_face))
}

/// 🖋️ Signed UV area of a closed chain walked in member order — each member sampled along its
/// own p-curve in its `forward` direction, so the sign answers the winding the same way
/// [`loop_uv_signed_area`] does for an existing ring.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chain_uv_signed_area(body: &Body, chain: &[ParametricEdge]) -> f64 {
    const SAMPLES: usize = 8;
    let mut points: Vec<Pnt2> = Vec::new();
    for &(_, forward, pcurve_id, prange) in chain {
        let Some(pcurve) = pcurve_id.and_then(|id| body.curves2.get(id)) else { continue };
        let (t0, t1) = if forward { prange } else { (prange.1, prange.0) };
        for i in 0..SAMPLES {
            points.push(pcurve.eval(t0 + (t1 - t0) * i as f64 / SAMPLES as f64));
        }
    }
    let mut area = 0.0;
    for (i, p) in points.iter().enumerate() {
        let q = points[(i + 1) % points.len()];
        area += p.x * q.y - q.x * p.y;
    }
    area * 0.5
}

/// 🧭️ 3D samples along a ring's coedges, each edge curve walked in that coedge's own direction.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_3d_points(body: &Body, loop_id: LoopId) -> Vec<Pnt3> {
    let members: Vec<(EdgeId, bool)> = body.loop_coedges(loop_id).into_iter().filter_map(|cid| body.coedges.get(cid).map(|co| (co.edge, co.forward))).collect();
    edges_3d_points(body, &members)
}

/// 🧭️ 3D samples along a chain's members, each edge curve walked in the member's own direction.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chain_3d_points(body: &Body, chain: &[ParametricEdge]) -> Vec<Pnt3> {
    let members: Vec<(EdgeId, bool)> = chain.iter().map(|&(e, f, _, _)| (e, f)).collect();
    edges_3d_points(body, &members)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edges_3d_points(body: &Body, members: &[(EdgeId, bool)]) -> Vec<Pnt3> {
    const SAMPLES: usize = 8;
    let mut points = Vec::new();
    for &(edge_id, forward) in members {
        let Some(edge) = body.edges.get(edge_id) else { continue };
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        let (t0, t1) = if forward { edge.range } else { (edge.range.1, edge.range.0) };
        for i in 0..SAMPLES {
            points.push(curve.eval(t0 + (t1 - t0) * i as f64 / SAMPLES as f64));
        }
    }
    points
}

/// 🧭️ Signed area of the closed polygon `points` as seen along `normal` — positive when the walk is
/// counter-clockwise about it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn points_signed_area_about(points: &[Pnt3], normal: Vec3) -> f64 {
    let Some(&first) = points.first() else { return 0.0 };
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, &p) in points.iter().enumerate() {
        let q = points[(i + 1) % points.len()];
        area = area + (p - first).cross(q - first);
    }
    area.dot(normal) * 0.5
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
// #endregion 🔖️Imprint
