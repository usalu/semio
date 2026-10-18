//! 🪡️ Boundary splice: the boundary vertex splice by point and by UV with the loop ring index and chain endpoint helpers. See [`super`].

use super::*;

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
pub(super) fn oriented_endpoints(body: &Body, member: ParametricEdge) -> Result<(VertexId, VertexId), KernelError> {
    let edge = body.edges.get(member.0).ok_or_else(|| KernelError::MissingEntity(format!("edge {}", member.0)))?;
    Ok(if member.1 { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) })
}

/// 🖋️ The `(u, v)` a [`ParametricEdge`] traversal starts (`at_start`) or ends at, read from its own
/// p-curve — which is stored in the EDGE's order, so the traversal's own start is `prange.1` when
/// the member runs backwards.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn member_uv(body: &Body, member: ParametricEdge, at_start: bool) -> Option<Pnt2> {
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
pub(super) fn ring_index_at_uv(body: &Body, (verts, members): (&[VertexId], &[ParametricEdge]), target: VertexId, uv: Option<Pnt2>) -> Option<usize> {
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
pub(super) fn splice_boundary_vertex_at_uv(body: &mut Body, loop_id: LoopId, target: VertexId, position: Pnt3, uv: Option<Pnt2>, tol: f64, rec: &mut OpRecorder) -> Result<(), KernelError> {
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
pub(super) fn chain_endpoint_uv(body: &Body, outer: LoopId, chain: &[ParametricEdge]) -> (Option<Pnt2>, Option<Pnt2>) {
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
// #endregion 🔖️Imprint
