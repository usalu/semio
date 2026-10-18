//! 🔪️ Face surgery: the patch trimming, the carried edge minting, the original face rebuild and the patch and corner face assembly. See [`super`].

use super::*;

// #region 🔖️Surgery
/// 🎨️ Orders the patch's two ends by its own run parameter and derives the end trims: an iso-line
/// wherever both faces stop at the same station (every constant-radius blend, whose cap plane is
/// perpendicular to its axis), the exact conic otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn trim_patch(body: &Body, edge: EdgeId, patch: &mut Patch, corners: &BTreeMap<VertexId, Corner>) -> Result<(), KernelError> {
    if patch.closed {
        patch.run = [(0.0, TAU); 2];
        patch.ends = [EndTrim::Iso(0.0), EndTrim::Iso(TAU)];
        return Ok(());
    }
    let faces = patch.faces;
    let mut ends = patch.end_vertex;
    let at = |ends: &[VertexId; 2], slot: usize, i: usize| corners[&ends[slot]].runs[&(edge, faces[i])];
    if at(&ends, 0, 0) > at(&ends, 1, 0) {
        ends.swap(0, 1);
    }
    let mut run = [(0.0, 0.0); 2];
    for (i, entry) in run.iter_mut().enumerate() {
        let (lo, hi) = (at(&ends, 0, i), at(&ends, 1, i));
        if hi - lo <= 1e-9 {
            return Err(KernelError::Operation("blend: the blend consumes its whole edge — reduce the radius".into()));
        }
        *entry = (lo, hi);
    }
    patch.end_vertex = ends;
    patch.run = run;
    for slot in 0..2 {
        let (a, b) = (at(&ends, slot, 0), at(&ends, slot, 1));
        let Some(cap) = corners[&ends[slot]].cap else {
            patch.ends[slot] = EndTrim::Iso(a);
            continue;
        };
        let (point, normal) = face_plane(body, cap)?;
        // 🧭️ Agreeing on ONE station is necessary but not sufficient: a variable fillet's cap
        // plane is symmetric about the dihedral's bisector, so both rulings meet it at the same
        // distance from the apex while the cross-section circle between them bulges straight
        // out of that plane. The trim is an iso-line only when the iso-line itself lies IN the
        // cap — which is exactly the constant-radius case, whose axis the cap is perpendicular to.
        patch.ends[slot] = if (a - b).abs() <= 1e-9 && iso_lies_in_plane(patch, a, point, normal) {
            EndTrim::Iso(a)
        } else {
            EndTrim::Conic(conic_end_trim(patch, point, normal)?)
        };
    }
    Ok(())
}

/// 🎨️ Whether the patch's own cross-section at run parameter `w` lies IN the given plane — the
/// precondition for trimming that end with an iso-line instead of the plane's own conic section.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn iso_lies_in_plane(patch: &Patch, w: f64, plane_point: Pnt3, plane_normal: Vec3) -> bool {
    let curve = patch.surface.isocurve(patch.run_dir(), w);
    let (lo, hi) = (patch.cross[0], patch.cross[1]);
    (0..=8).all(|i| {
        let t = lo + (hi - lo) * f64::from(i) / 8.0;
        (curve.eval(t) - plane_point).dot(plane_normal).abs() <= 1e-9
    })
}

/// 🎨️ Carries one unblended edge over into the result, shortened wherever a corner pulled one of
/// its ends back, with fresh vertices so the input solid stays intact.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn mint_carried_edge(body: &mut Body, edge: EdgeId, corners: &BTreeMap<VertexId, Corner>, solid_faces: &BTreeSet<FaceId>, minted: &mut Minted, rec: &mut OpRecorder) -> Result<(), KernelError> {
    let ent = body.edges.get(edge).unwrap().clone();
    let curve = body.curves3.get(ent.curve).unwrap().clone();
    let originals = [ent.v0, ent.v1];
    let mut params = [ent.range.0, ent.range.1];
    let mut verts = originals;
    for slot in 0..2 {
        if let Some(corner) = corners.get(&originals[slot]) {
            let face = blended_face_of(body, edge, corner, solid_faces)?;
            let point = corner.points[&face];
            params[slot] = curve_param_at(&curve, point, ent.range).ok_or_else(|| KernelError::Operation("blend: a shortened endpoint is not on its own edge".into()))?;
            verts[slot] = corner.verts[&face];
            continue;
        }
        if !minted.vertices.contains_key(&originals[slot]) {
            let position = body.vertices.get(originals[slot]).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
            let fresh = make_vertex(body, position, Tol::DEFAULT, rec);
            minted.vertices.insert(originals[slot], fresh);
        }
        verts[slot] = minted.vertices[&originals[slot]];
    }
    let range = (params[0], params[1]);
    let fresh = make_edge(body, ent.curve, range, verts[0], verts[1], Tol::DEFAULT, rec);
    minted.edges.insert(edge, fresh);
    minted.ranges.insert(edge, range);
    Ok(())
}

/// 🎨️ The single blended face an unblended edge touches at a corner — the face whose own boundary
/// pulled that edge's endpoint back to the corner point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn blended_face_of(body: &Body, edge: EdgeId, corner: &Corner, solid_faces: &BTreeSet<FaceId>) -> Result<FaceId, KernelError> {
    let hits: Vec<FaceId> = edge_faces(body, solid_faces, edge).into_iter().filter(|f| corner.points.contains_key(f)).collect();
    match hits.len() {
        1 => Ok(hits[0]),
        _ => Err(KernelError::Operation("blend: an unblended edge at a blended vertex touches no single blended face".into())),
    }
}

/// 🎨️ Rebuilds one original face: every selected edge becomes that face's own tangency curve, every
/// other edge is carried over (its p-curve reused verbatim, its parameter range remapped where a
/// corner shortened it), and a partially blended vertex splices in the blend's end curve.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn rebuild_face(body: &mut Body, face: FaceId, patches: &BTreeMap<EdgeId, Patch>, corners: &BTreeMap<VertexId, Corner>, minted: &Minted, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    let fd = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?.clone();
    let surface = body.surfaces.get(fd.surface).unwrap().clone();
    let outer = fd.outer.ok_or_else(|| KernelError::Operation("blend: face has no outer loop".into()))?;
    if !fd.inners.is_empty() {
        return Err(KernelError::Operation("blend: faces with holes are not supported".into()));
    }
    let ring = body.loop_coedges(outer);
    let mut members: Vec<Member> = Vec::with_capacity(ring.len() + 2);
    for &cid in &ring {
        let coedge = body.coedges.get(cid).unwrap().clone();
        let end_vertex = body.coedge_endpoints(cid).map(|(_, b)| b).ok_or_else(|| KernelError::Operation("blend: broken coedge".into()))?;
        if let Some(patch) = patches.get(&coedge.edge) {
            let i = patch.faces.iter().position(|f| *f == face).ok_or_else(|| KernelError::Operation("blend: a patch does not know its own face".into()))?;
            let curve = patch.tangency(i);
            let pcurve = blend_pcurve(&surface, &curve, patch.run[i])?;
            members.push(Member { edge: minted.tangency[&(patch.edge, i)], forward: !patch_tangency_forward(patch, i), pcurve, prange: patch.run[i] });
        } else {
            let range = minted.ranges[&coedge.edge];
            let original = body.edges.get(coedge.edge).map(|e| e.range).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
            let pcurve_id = coedge.pcurve.ok_or_else(|| KernelError::Operation("blend: the input solid has a coedge without a p-curve".into()))?;
            let pcurve = body.curves2.get(pcurve_id).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?.clone();
            members.push(Member { edge: minted.edges[&coedge.edge], forward: coedge.forward, pcurve, prange: remap_prange(coedge.prange, original, range) });
        }
        if let Some(corner) = corners.get(&end_vertex) {
            if corner.cap == Some(face) {
                let patch = &patches[&corner.corner_edges[0]];
                let slot = usize::from(patch.end_vertex[1] == end_vertex);
                let (curve, range) = patch.end_curve(slot);
                let pcurve = blend_pcurve(&surface, &curve, range)?;
                members.push(Member { edge: minted.ends[&(patch.edge, slot)], forward: !patch_end_forward(patch, slot), pcurve, prange: range });
            }
        }
    }
    Ok(attach_with_pcurves(body, fd.surface, &mut members, fd.flipped, rec))
}

/// 🎨️ A p-curve's own sub-range for a shortened edge — the same affine `s` the same-parameter law
/// maps `range` onto `prange` with.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remap_prange(prange: (f64, f64), original: (f64, f64), trimmed: (f64, f64)) -> (f64, f64) {
    let span = original.1 - original.0;
    if span.abs() <= 1e-15 {
        return prange;
    }
    let at = |t: f64| prange.0 + (prange.1 - prange.0) * (t - original.0) / span;
    (at(trimmed.0), at(trimmed.1))
}

/// 🎨️ How the patch itself traverses its own tangency curve `i`: the ring is
/// `[tangency 0, end@hi, tangency 1, end@lo]` when that walks `(u, v)` counter-clockwise and its
/// reverse otherwise. Every ADJACENT face traverses the same edge the other way round, which is
/// exactly what a closed, coherently oriented shell means — so no face outside the patch needs its
/// own winding derivation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn patch_tangency_forward(patch: &Patch, i: usize) -> bool {
    patch.ring_is_ccw() == (i == 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn patch_end_forward(patch: &Patch, slot: usize) -> bool {
    patch.ring_is_ccw() == (slot == 1)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_patch_face(body: &mut Body, edge: EdgeId, patch: &Patch, minted: &Minted, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    let surface = patch.surface.clone();
    let surface_id = body.surfaces.insert(surface.clone());
    let tangency = |i: usize| (minted.tangency[&(edge, i)], patch_tangency_forward(patch, i), patch.tangency(i), patch.run[i]);
    let end = |slot: usize| {
        let (curve, range) = patch.end_curve(slot);
        (minted.ends[&(edge, slot)], patch_end_forward(patch, slot), curve, range)
    };
    let entries = if patch.ring_is_ccw() { [tangency(0), end(1), tangency(1), end(0)] } else { [end(0), tangency(1), end(1), tangency(0)] };
    let mut members = Vec::with_capacity(4);
    for (edge_id, forward, curve, range) in entries {
        let pcurve = blend_pcurve(&surface, &curve, range)?;
        members.push(Member { edge: edge_id, forward, pcurve, prange: range });
    }
    Ok(attach_with_pcurves(body, surface_id, &mut members, false, rec))
}

/// 🎨️ Builds the corner face from the three end curves meeting at a fully blended vertex, chaining
/// them into a ring by their shared vertices — the traversal sense is forced by shell closure (the
/// opposite of each patch's own), so the ring's winding needs no independent derivation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_corner_face(body: &mut Body, vertex: VertexId, corner: &Corner, patches: &BTreeMap<EdgeId, Patch>, minted: &Minted, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    let surface = corner.corner_surface.clone().expect("corner face has a surface");
    let surface_id = body.surfaces.insert(surface.clone());
    let mut pending: Vec<(EdgeId, bool, Curve3, (f64, f64), VertexId, VertexId)> = Vec::new();
    for &edge in &corner.corner_edges {
        let patch = &patches[&edge];
        let slot = usize::from(patch.end_vertex[1] == vertex);
        let id = minted.ends[&(edge, slot)];
        let forward = !patch_end_forward(patch, slot);
        let ent = body.edges.get(id).unwrap();
        let (from, to) = if forward { (ent.v0, ent.v1) } else { (ent.v1, ent.v0) };
        let (curve, range) = patch.end_curve(slot);
        pending.push((id, forward, curve, range, from, to));
    }
    let mut ordered = vec![pending.remove(0)];
    while !pending.is_empty() {
        let tail = ordered.last().unwrap().5;
        let index = pending.iter().position(|entry| entry.4 == tail).ok_or_else(|| KernelError::Operation("blend: the corner's end curves do not form a ring".into()))?;
        ordered.push(pending.remove(index));
    }
    if ordered.first().unwrap().4 != ordered.last().unwrap().5 {
        return Err(KernelError::Operation("blend: the corner ring does not close".into()));
    }
    let mut members = Vec::with_capacity(ordered.len());
    for (id, forward, curve, range, _, _) in ordered {
        let pcurve = blend_pcurve(&surface, &curve, range)?;
        members.push(Member { edge: id, forward, pcurve, prange: range });
    }
    Ok(attach_with_pcurves(body, surface_id, &mut members, false, rec))
}
// #endregion 🔖️Surgery
