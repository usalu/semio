//! 🔗️ Imprint vertices and edges: the welded imprint vertex, the imprint edge, the coincident boundary edge and the boundary vertex and sub-edge helpers. See [`super`].

use super::*;

// #region 🔖️Scoping
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
pub(super) fn build_imprint_edge(body: &mut Body, ic: &IntCurve, (t0, t1): (f64, f64), closed: bool, (tol, weld): (f64, &mut Vec<(Pnt3, VertexId)>), rec: &mut OpRecorder) -> EdgeId {
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

/// 🔀 The pre-existing boundary edge of `face` that the clipped segment `ic[t0, t1]` runs ALONG
/// (every sample within tolerance of that edge's own curve), if there is one. This is the
/// "imprint lands exactly on existing topology" case: the boundary the boolean needs on this side
/// already exists, and minting a second, geometrically identical edge for it would leave the
/// result with duplicate — and therefore non-manifold — topology.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn coincident_boundary_edge(body: &Body, face: FaceId, ic: &IntCurve, (t0, t1): (f64, f64), tol: f64) -> Option<EdgeId> {
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
pub(super) fn boundary_subedge(body: &mut Body, face: FaceId, (p0, p1): (Pnt3, Pnt3), (tol, weld): (f64, &mut Vec<(Pnt3, VertexId)>), rec: &mut OpRecorder) -> Result<EdgeId, KernelError> {
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

/// 🔗️ Retains one existing seam vertex for geometrically coincident closed operand boundaries.
pub(super) fn unify_closed_boundary_vertices(body: &mut Body, from: EdgeId, to: EdgeId, tol: f64, weld: &mut [(Pnt3, VertexId)]) -> Result<(), KernelError> {
    let from_vertex = body.edges.get(from).map(|edge| edge.v0).ok_or_else(|| KernelError::MissingEntity(format!("edge {from}")))?;
    let to_vertex = body.edges.get(to).map(|edge| edge.v0).ok_or_else(|| KernelError::MissingEntity(format!("edge {to}")))?;
    let a = body.vertices.get(from_vertex).ok_or_else(|| KernelError::MissingEntity(format!("vertex {from_vertex}")))?.position;
    let b = body.vertices.get(to_vertex).ok_or_else(|| KernelError::MissingEntity(format!("vertex {to_vertex}")))?.position;
    if a.distance(b) > tol { return Err(KernelError::Boolean(BooleanError::ImprintFailed("closed coincident boundaries have distinct seam anchors".into()))); }
    if from_vertex != to_vertex {
        for (_, edge) in body.edges.iter_mut() {
            if edge.v0 == from_vertex { edge.v0 = to_vertex; }
            if edge.v1 == from_vertex { edge.v1 = to_vertex; }
        }
        for (_, vertex) in weld.iter_mut() { if *vertex == from_vertex { *vertex = to_vertex; } }
    }
    Ok(())
}

/// 🔀 Re-points every coedge of `from` at `to` — the same physical segment, already bounded by the
/// same welded vertices on both operands — flipping the coedge's sense when the two edges run in
/// opposite vertex order. A coedge's p-curve is parameterised along its own edge's curve, so one
/// carried over from `from` is dropped (a planar ring needs none; a periodic one is rebuilt by the
/// splitters from the shared edge). `from` is left without coedges for the job's orphan sweep.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn unify_boundary_edges(body: &mut Body, from: EdgeId, to: EdgeId) -> Result<(), KernelError> {
    let (from_v0, from_v1) = body.edges.get(from).map(|edge| (edge.v0, edge.v1)).ok_or_else(|| KernelError::MissingEntity(format!("edge {from}")))?;
    let (to_v0, to_v1) = body.edges.get(to).map(|edge| (edge.v0, edge.v1)).ok_or_else(|| KernelError::MissingEntity(format!("edge {to}")))?;
    let same_order = if from_v0 == from_v1 && to_v0 == to_v1 {
        body.edges.get(from).and_then(|edge| body.curves3.get(edge.curve).and_then(|curve| curve.tangent(edge.range.0)))
            .zip(body.edges.get(to).and_then(|edge| body.curves3.get(edge.curve).and_then(|curve| curve.tangent(edge.range.0))))
            .is_some_and(|(a, b)| a.dot(b) >= 0.0)
    } else { from_v0 == to_v0 && from_v1 == to_v1 };
    if !same_order && !(from_v0 == to_v1 && from_v1 == to_v0) {
        return Err(KernelError::Boolean(BooleanError::ImprintFailed(format!("boundary edges {from} and {to} do not span the same welded vertices"))));
    }
    let coedges: Vec<CoedgeId> = body.coedges.iter().filter(|(_, co)| co.edge == from).map(|(id, _)| id).collect();
    for cid in coedges {
        if let Some(co) = body.coedges.get_mut(cid) {
            co.edge = to;
            if !same_order {
                co.forward = !co.forward;
            }
            co.pcurve = None;
        }
    }
    Ok(())
}

/// 🔀 Orders the shared edge's p-curve range to match the edge's own `v0 → v1` direction, so the
/// linear `prange → edge.range` map the same-parameter check applies stays consistent whether the
/// edge was freshly built (already in that order) or reused from an operand's own boundary
/// (whose direction is whatever the primitive chose).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn oriented_prange(body: &Body, edge_id: EdgeId, (p0, p1): (Pnt3, Pnt3), (t0, t1): (f64, f64)) -> (f64, f64) {
    let Some(edge) = body.edges.get(edge_id) else { return (t0, t1) };
    let Some(start) = body.vertices.get(edge.v0) else { return (t0, t1) };
    if start.position.distance(p1) < start.position.distance(p0) {
        (t1, t0)
    } else {
        (t0, t1)
    }
}
// #endregion 🔖️Imprint
// #endregion 🔖️ExactImprintEngine
