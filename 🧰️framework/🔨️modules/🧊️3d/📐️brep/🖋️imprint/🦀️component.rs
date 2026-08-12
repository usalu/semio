//! 🖋️ UV planar arrangement and face split via Euler ops.
//!
//! Minimum viable imprint: cut a planar face with a line through two points, splitting the outer
//! loop into two faces that share a new chord edge. Lane 3-imprint of ticket
//! `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

use crate::brep::arena::{ArenaId, EdgeId, FaceId, LoopId, VertexId};
use crate::brep::curve::Curve3;
use crate::brep::error::KernelError;
use crate::brep::euler::{add_face, make_edge, make_loop, split_edge};
use crate::brep::history::OpRecorder;
use crate::brep::mat::Frame3;
use crate::brep::surface::Surface;
use crate::brep::topo::{Body, Edge};
use crate::brep::vec::{Pnt2, Pnt3, Vec3};

// #region 🔖️Api

/// 🖋️ Split a planar face along the line through `p0` and `p1`.
///
/// Intersects the infinite line with the outer boundary in the face UV plane, splits the two hit
/// edges (or reuses existing vertices when the hit lands on a corner), inserts a chord edge, and
/// rebuilds two outer loops via Euler editors. Returns `(original_face, new_face)`.
pub fn split_planar_face_by_line(
    body: &mut Body,
    face: FaceId,
    p0: Pnt3,
    p1: Pnt3,
) -> Result<(FaceId, FaceId), KernelError> {
    let face_data = body
        .faces
        .get(face)
        .ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?
        .clone();
    let outer = face_data
        .outer
        .ok_or_else(|| KernelError::Operation(format!("face {face} has no outer loop")))?;
    if !face_data.inners.is_empty() {
        return Err(KernelError::Operation(
            "split_planar_face_by_line does not support faces with inner loops yet".into(),
        ));
    }
    let surface = body
        .surfaces
        .get(face_data.surface)
        .ok_or_else(|| KernelError::MissingEntity(format!("surface {}", face_data.surface)))?
        .clone();
    let Surface::Plane { frame } = surface else {
        return Err(KernelError::InvalidInput(
            "split_planar_face_by_line requires a planar face".into(),
        ));
    };
    let tol = face_data.tol;
    let linear = tol.value().max(1e-12);
    if (p1 - p0).norm() <= linear {
        return Err(KernelError::InvalidInput(
            "cutting line endpoints must be distinct".into(),
        ));
    }
    let uv0 = project_uv(&frame, p0);
    let uv1 = project_uv(&frame, p1);
    if (uv1 - uv0).norm() <= linear {
        return Err(KernelError::InvalidInput(
            "cutting line is orthogonal to the face plane".into(),
        ));
    }

    let coedges = body.loop_coedges(outer);
    if coedges.len() < 3 {
        return Err(KernelError::Operation(format!(
            "face {face} outer loop needs at least 3 coedges"
        )));
    }

    let mut hits: Vec<BoundaryHit> = Vec::new();
    for &cid in &coedges {
        let coedge = body
            .coedges
            .get(cid)
            .ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid}")))?;
        let edge_id = coedge.edge;
        if hits.iter().any(|h| h.edge == edge_id) {
            continue;
        }
        let edge = body
            .edges
            .get(edge_id)
            .ok_or_else(|| KernelError::MissingEntity(format!("edge {edge_id}")))?
            .clone();
        let v0 = body
            .vertices
            .get(edge.v0)
            .ok_or_else(|| KernelError::MissingEntity(format!("vertex {}", edge.v0)))?
            .position;
        let v1 = body
            .vertices
            .get(edge.v1)
            .ok_or_else(|| KernelError::MissingEntity(format!("vertex {}", edge.v1)))?
            .position;
        let a = project_uv(&frame, v0);
        let b = project_uv(&frame, v1);
        let Some((_t_seg, point_uv)) = intersect_segment_line_uv(a, b, uv0, uv1, linear) else {
            continue;
        };
        let point = frame.to_world(Pnt3::new(point_uv.x, point_uv.y, 0.0));
        let curve_t = edge_param_at_point(body, &edge, point, linear)?;
        if let Some(existing) = endpoint_vertex(&edge, curve_t, linear) {
            if !hits.iter().any(|h| {
                h.vertex_hint == Some(existing) || h.point.distance(point) <= linear
            }) {
                hits.push(BoundaryHit {
                    edge: edge_id,
                    curve_t,
                    point,
                    vertex_hint: Some(existing),
                });
            }
            continue;
        }
        if !(curve_t > edge.range.0 + 1e-14 && curve_t < edge.range.1 - 1e-14) {
            continue;
        }
        if hits.iter().any(|h| h.point.distance(point) <= linear) {
            continue;
        }
        hits.push(BoundaryHit {
            edge: edge_id,
            curve_t,
            point,
            vertex_hint: None,
        });
    }

    if hits.len() != 2 {
        return Err(KernelError::Operation(format!(
            "cutting line must cross the outer boundary at exactly two places (got {})",
            hits.len()
        )));
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
            let (e1, e2) = surviving_same_edge.ok_or_else(|| {
                KernelError::Operation("same-edge double hit missing first split survivors".into())
            })?;
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
        return Err(KernelError::Operation(
            "cutting line degenerates to a single boundary vertex".into(),
        ));
    }

    let (verts, members) = loop_walk(body, outer)?;
    let ia = verts
        .iter()
        .position(|&v| v == va)
        .ok_or_else(|| KernelError::Operation("split vertex A missing from outer loop".into()))?;
    let ib = verts
        .iter()
        .position(|&v| v == vb)
        .ok_or_else(|| KernelError::Operation("split vertex B missing from outer loop".into()))?;
    let chain_ab = member_chain(&members, ia, ib);
    let chain_ba = member_chain(&members, ib, ia);
    if chain_ab.is_empty() || chain_ba.is_empty() {
        return Err(KernelError::Operation(
            "cutting line does not partition the outer loop into two non-empty chains".into(),
        ));
    }

    let pa = body.vertices.get(va).expect("vertex A").position;
    let pb = body.vertices.get(vb).expect("vertex B").position;
    let cut_curve = body.curves3.insert(Curve3::Line {
        origin: pa,
        dir: pb - pa,
    });
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
        let f = body
            .faces
            .get_mut(face)
            .ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?;
        f.outer = Some(loop_a);
        f.inners.clear();
    }

    let loop_b = make_loop(body, FaceId::from_raw(0, 0), &members_b);
    let face_b = add_face(
        body,
        face_data.surface,
        Some(loop_b),
        vec![],
        face_data.flipped,
        tol,
        &mut rec,
    );
    body.loops
        .get_mut(loop_b)
        .ok_or_else(|| KernelError::MissingEntity(format!("loop {loop_b}")))?
        .face = face_b;

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

fn project_uv(frame: &Frame3, p: Pnt3) -> Pnt2 {
    let local = frame.to_local(p);
    Pnt2::new(local.x, local.y)
}

fn intersect_segment_line_uv(
    a: Pnt2,
    b: Pnt2,
    p0: Pnt2,
    p1: Pnt2,
    tol: f64,
) -> Option<(f64, Pnt2)> {
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

fn edge_param_at_point(body: &Body, edge: &Edge, point: Pnt3, tol: f64) -> Result<f64, KernelError> {
    let curve = body
        .curves3
        .get(edge.curve)
        .ok_or_else(|| KernelError::MissingEntity(format!("curve {}", edge.curve)))?;
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
                return Err(KernelError::Operation(
                    "could not locate imprint hit on non-line edge".into(),
                ));
            }
            Ok(best_t)
        }
    }
}

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

fn loop_walk(body: &Body, loop_id: LoopId) -> Result<(Vec<VertexId>, Vec<(EdgeId, bool)>), KernelError> {
    let coedges = body.loop_coedges(loop_id);
    let mut verts = Vec::with_capacity(coedges.len());
    let mut members = Vec::with_capacity(coedges.len());
    for cid in coedges {
        let (start, _) = body
            .coedge_endpoints(cid)
            .ok_or_else(|| KernelError::Operation(format!("coedge {cid} missing endpoints")))?;
        let coedge = body
            .coedges
            .get(cid)
            .ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid}")))?;
        verts.push(start);
        members.push((coedge.edge, coedge.forward));
    }
    Ok((verts, members))
}

fn member_chain(members: &[(EdgeId, bool)], from: usize, to: usize) -> Vec<(EdgeId, bool)> {
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


fn resolve_edge_containing_param(
    body: &Body,
    e1: EdgeId,
    e2: EdgeId,
    t: f64,
) -> Result<EdgeId, KernelError> {
    for edge_id in [e1, e2] {
        let Some(edge) = body.edges.get(edge_id) else { continue };
        if t > edge.range.0 + 1e-14 && t < edge.range.1 - 1e-14 {
            return Ok(edge_id);
        }
    }
    Err(KernelError::Operation(
        "same-edge double hit: second parameter not found on either survivor".into(),
    ))
}

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::primitives::{make_planar_face_from_wire, make_rectangle_wire};
    use crate::brep::validate::validate_body;

    #[test]
    fn split_rectangle_face_into_two() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut body, 2.0, 2.0, &mut rec).unwrap();
        let face = make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec)
            .unwrap();
        let (f0, f1) = split_planar_face_by_line(
            &mut body,
            face,
            Pnt3::new(1.0, -1.0, 0.0),
            Pnt3::new(1.0, 3.0, 0.0),
        )
        .unwrap();
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
        assert!(
            issues.is_empty(),
            "validate_body issues: {:?}",
            issues
                .iter()
                .map(|i| format!("{}:{}:{}", i.entity, i.code, i.message))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn split_rejects_non_cutting_line() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut body, 2.0, 2.0, &mut rec).unwrap();
        let face = make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec)
            .unwrap();
        let err = split_planar_face_by_line(
            &mut body,
            face,
            Pnt3::new(3.0, 0.0, 0.0),
            Pnt3::new(3.0, 2.0, 0.0),
        )
        .unwrap_err();
        assert!(matches!(err, KernelError::Operation(_)));
    }

    #[test]
    fn split_rejects_missing_face() {
        let mut body = Body::new();
        let err = split_planar_face_by_line(
            &mut body,
            FaceId::from_raw(0, 0),
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 0.0),
        )
        .unwrap_err();
        assert!(matches!(err, KernelError::MissingEntity(_)));
    }
}


    #[test]
    fn resolve_edge_containing_param_picks_survivor_after_split() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let face = crate::brep::primitives::make_planar_face_from_points(
            &mut body,
            &[
                Pnt3::new(0.0, 0.0, 0.0),
                Pnt3::new(4.0, 0.0, 0.0),
                Pnt3::new(4.0, 2.0, 0.0),
                Pnt3::new(0.0, 2.0, 0.0),
            ],
            &mut rec,
        )
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

// #endregion 🔖️Tests
