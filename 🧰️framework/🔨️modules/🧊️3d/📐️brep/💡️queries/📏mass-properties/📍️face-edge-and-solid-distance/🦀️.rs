//! 📍️ Face, edge and solid distance: face area, edge length, the solid to solid distance and overlap tests and the closest point on a solid. See [`super`].

use super::*;

// #region 🔖️FaceEdge
/// 📐 Area of one face (`outer` minus `inner` loops).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn face_area(body: &Body, face: FaceId, chord_tol: f64) -> Result<f64, KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut area = 0.0;
    if let Some(outer) = face_ent.outer {
        area += loop_area(body, face, outer, chord_tol)?;
    }
    for &inner in &face_ent.inners {
        area -= loop_area(body, face, inner, chord_tol)?;
    }
    Ok(area.abs())
}

/// 📐 Arc length of an edge over its trimmed parameter range.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn edge_length(body: &Body, edge: EdgeId) -> Result<f64, KernelError> {
    let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    Ok(curve_ops::arc_length(curve, edge_ent.range.0, edge_ent.range.1, 1e-9))
}
// #endregion 🔖️FaceEdge

// #region 🔖️Distance
/// 📏 Minimum distance between two closed solids — `0.0` only when a real point-in-solid
/// classification (not a coarse sample-point-happens-to-be-close heuristic) finds one solid
/// genuinely containing a point of the other; otherwise the TRUE minimum via vertex/face, face
/// sample point/face, and edge/edge candidates (§6.11: "overlapping-solid distance can sample face
/// points and miss edge-edge/interior extrema").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn distance_solid_solid(body: &Body, a: SolidId, b: SolidId) -> Result<f64, KernelError> {
    let bb_a = solid_bounding_box(body, a)?;
    let bb_b = solid_bounding_box(body, b)?;
    let separated = axis_aligned_box_distance(&bb_a, &bb_b);
    if separated > 1e-9 {
        return Ok(separated);
    }
    if solids_overlap(body, a, b)? {
        return Ok(0.0);
    }
    let mut best = f64::INFINITY;
    for face in body.solid_faces(a) {
        for p in face_sample_points(body, face)? {
            let (_, d) = closest_point_on_solid(body, b, p)?;
            best = best.min(d);
        }
    }
    for face in body.solid_faces(b) {
        for p in face_sample_points(body, face)? {
            let (_, d) = closest_point_on_solid(body, a, p)?;
            best = best.min(d);
        }
    }
    if let Some(d) = edge_edge_closest_distance(body, a, b) {
        best = best.min(d);
    }
    if !best.is_finite() {
        return Err(KernelError::Operation("distance_solid_solid: no finite candidate distance found".into()));
    }
    Ok(best)
}

/// 📏 `true` when a sample point on either solid's boundary is genuinely `Inside` the other, per
/// the one authoritative classifier — not a bounding-box or sample-distance proxy for overlap.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn solids_overlap(body: &Body, a: SolidId, b: SolidId) -> Result<bool, KernelError> {
    use semio_framework_3d_brep_representation::contract::PointClassification;
    use crate::classification::point_in_solid;
    for face in body.solid_faces(a) {
        for p in face_sample_points(body, face)? {
            if matches!(point_in_solid(body, b, p, 1e-9)?, PointClassification::Inside) {
                return Ok(true);
            }
        }
    }
    for face in body.solid_faces(b) {
        for p in face_sample_points(body, face)? {
            if matches!(point_in_solid(body, a, p, 1e-9)?, PointClassification::Inside) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn solid_unique_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut seen = std::collections::HashSet::new();
    for face in body.solid_faces(solid) {
        for coedge in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(coedge) {
                seen.insert(c.edge);
            }
        }
    }
    seen.into_iter().collect()
}

/// 📏 Nearest approach between every edge of `a` and every edge of `b`, sampling one curve against
/// the other's `curve_ops::closest_parameter` in both directions — a practical (sampling-refined,
/// not a certified global continuous optimum) edge/edge extremum candidate the old face-sample-only
/// distance missed entirely.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_edge_closest_distance(body: &Body, a: SolidId, b: SolidId) -> Option<f64> {
    let edges_a = solid_unique_edges(body, a);
    let edges_b = solid_unique_edges(body, b);
    let mut best: Option<f64> = None;
    for &ea in &edges_a {
        for &eb in &edges_b {
            if let Some(d) = edge_edge_min_distance(body, ea, eb) {
                best = Some(best.map_or(d, |b: f64| b.min(d)));
            }
        }
    }
    best
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_edge_min_distance(body: &Body, ea: EdgeId, eb: EdgeId) -> Option<f64> {
    let edge_a = body.edges.get(ea)?;
    let edge_b = body.edges.get(eb)?;
    let curve_a = body.curves3.get(edge_a.curve)?;
    let curve_b = body.curves3.get(edge_b.curve)?;
    const SAMPLES: usize = 12;
    let mut best = f64::INFINITY;
    for i in 0..=SAMPLES {
        let s = i as f64 / SAMPLES as f64;
        let t = edge_a.range.0 + (edge_a.range.1 - edge_a.range.0) * s;
        let closest = curve_ops::closest_parameter(curve_b, edge_b.range, curve_a.eval(t), 1e-9);
        best = best.min(closest.distance);
    }
    for i in 0..=SAMPLES {
        let s = i as f64 / SAMPLES as f64;
        let t = edge_b.range.0 + (edge_b.range.1 - edge_b.range.0) * s;
        let closest = curve_ops::closest_parameter(curve_a, edge_a.range, curve_b.eval(t), 1e-9);
        best = best.min(closest.distance);
    }
    Some(best)
}

/// 📏 Closest point on `solid` to `point` and the Euclidean distance.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn closest_point_on_solid(body: &Body, solid: SolidId, point: Pnt3) -> Result<(Pnt3, f64), KernelError> {
    let faces = body.solid_faces(solid);
    if faces.is_empty() {
        return Err(KernelError::MissingEntity("solid has no faces".into()));
    }
    let mut best_p = point;
    let mut best_d = f64::INFINITY;
    for face in faces {
        let (p, d) = closest_point_on_face(body, face, point)?;
        if d < best_d {
            best_d = d;
            best_p = p;
        }
    }
    Ok((best_p, best_d))
}
// #endregion 🔖️Distance

// #region 🔖️Loops
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn axis_aligned_box_distance(a: &AxisAlignedBox, b: &AxisAlignedBox) -> f64 {
    let dx = gap_1d(a.min.x, a.max.x, b.min.x, b.max.x);
    let dy = gap_1d(a.min.y, a.max.y, b.min.y, b.max.y);
    let dz = gap_1d(a.min.z, a.max.z, b.min.z, b.max.z);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gap_1d(a0: f64, a1: f64, b0: f64, b1: f64) -> f64 {
    if a1 < b0 {
        b0 - a1
    } else if b1 < a0 {
        a0 - b1
    } else {
        0.0
    }
}
// #endregion 🔖️Loops
