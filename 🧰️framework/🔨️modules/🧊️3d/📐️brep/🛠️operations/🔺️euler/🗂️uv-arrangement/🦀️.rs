//! 🗂️ UV arrangement: the boundary hit record, the UV segment and line intersection, the edge parameter lookup and the loop walks with their p-curve chains. See [`super`].

use super::*;

// #region 🔖️UvArrange
#[derive(Clone, Copy)]
pub(super) struct BoundaryHit {
    pub(super) edge: EdgeId,
    pub(super) curve_t: f64,
    pub(super) point: Pnt3,
    pub(super) vertex_hint: Option<VertexId>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn project_uv(frame: &Frame3, p: Pnt3) -> Pnt2 {
    let local = frame.to_local(p);
    Pnt2::new(local.x, local.y)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn intersect_segment_line_uv(a: Pnt2, b: Pnt2, p0: Pnt2, p1: Pnt2, tol: f64) -> Option<(f64, Pnt2)> {
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
pub(super) fn edge_param_at_point(body: &Body, edge: &Edge, point: Pnt3, tol: f64) -> Result<f64, KernelError> {
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
pub(super) fn endpoint_vertex(edge: &Edge, curve_t: f64, linear: f64) -> Option<VertexId> {
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
pub(super) fn loop_walk(body: &Body, loop_id: LoopId) -> Result<LoopWalk, KernelError> {
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
pub(super) fn loop_walk_pc(body: &Body, loop_id: LoopId) -> Result<(Vec<VertexId>, Vec<ParametricEdge>), KernelError> {
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
pub(super) fn member_chain_pc(members: &[ParametricEdge], from: usize, to: usize) -> Vec<ParametricEdge> {
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
pub(super) fn make_loop_pc(body: &mut Body, face: FaceId, members: &[ParametricEdge]) -> LoopId {
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
pub(super) fn resolve_edge_containing_param(body: &Body, e1: EdgeId, e2: EdgeId, t: f64) -> Result<EdgeId, KernelError> {
    for edge_id in [e1, e2] {
        let Some(edge) = body.edges.get(edge_id) else { continue };
        if t > edge.range.0 + 1e-14 && t < edge.range.1 - 1e-14 {
            return Ok(edge_id);
        }
    }
    Err(KernelError::Operation("same-edge double hit: second parameter not found on either survivor".into()))
}
