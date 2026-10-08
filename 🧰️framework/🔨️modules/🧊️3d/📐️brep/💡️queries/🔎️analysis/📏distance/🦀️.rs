//! 📏 Minimum distance between two shapes and between a point and a shape, with the witness points.
//!
//! Point-to-shape distance is exact per face (analytic closest point with the trim test) and prunes faces
//! by the lower bound their bounding boxes give. Shape-to-shape distance is the minimum over closest
//! points of one shape's sample points against the other's faces, both ways, and over edge-to-edge
//! nearest approaches, then polished by alternating exact projections between the shapes (which converges
//! to the true pair for convex shapes); it is `0` when the shapes' solids overlap. For non-convex shapes it
//! is certified only up to its sampling, like `distance_solid_solid`.

use super::{ScopeMembers, ShapeKind, ShapeScope};
use crate::brep::queries::bounding_volume::face_aabb;
use crate::brep::queries::classification::point_in_solid;
use crate::brep::queries::mass_properties::{closest_point_on_face, solids_overlap};
use crate::brep::representation::arena::{EdgeId, FaceId};
use crate::brep::representation::curve::curve_ops;
use crate::brep::representation::error::KernelError;
use crate::brep::representation::topology::Body;
use crate::brep::representation::vector::Pnt3;

/// 📍 The nearest pair of points between two shapes.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ClosestPair {
    pub distance: f64,
    pub point_a: [f64; 3],
    pub point_b: [f64; 3],
}

/// 🧲 The nearest point of a shape to a query point; `signed` is negative inside a solid or compound.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct PointDistance {
    pub distance: f64,
    pub closest: [f64; 3],
    pub signed: Option<f64>,
}

const EDGE_SAMPLES: usize = 12;
const REFINE_STEPS: usize = 200;

fn array(p: Pnt3) -> [f64; 3] {
    [p.x, p.y, p.z]
}

fn box_gap(min: [f64; 3], max: [f64; 3], p: Pnt3) -> f64 {
    let gap = |axis: usize, value: f64| (min[axis] - value).max(value - max[axis]).max(0.0);
    (gap(0, p.x).powi(2) + gap(1, p.y).powi(2) + gap(2, p.z).powi(2)).sqrt()
}

fn nearest_on_faces(body: &Body, faces: &[FaceId], p: Pnt3) -> Result<Option<(Pnt3, f64)>, KernelError> {
    let mut ranked = Vec::with_capacity(faces.len());
    for &face in faces {
        let bound = face_aabb(body, face).map_or(0.0, |b| box_gap(b.min, b.max, p));
        ranked.push((bound, face));
    }
    ranked.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut best: Option<(Pnt3, f64)> = None;
    for (bound, face) in ranked {
        if best.is_some_and(|(_, d)| bound >= d) {
            break;
        }
        let (point, distance) = closest_point_on_face(body, face, p)?;
        if best.is_none_or(|(_, d)| distance < d) {
            best = Some((point, distance));
        }
    }
    Ok(best)
}

fn nearest_on_wires(body: &Body, members: &ScopeMembers, p: Pnt3) -> Option<(Pnt3, f64)> {
    let mut best: Option<(Pnt3, f64)> = None;
    for &edge in &members.edges {
        let Some(edge_ent) = body.edges.get(edge) else { continue };
        let Some(curve) = body.curves3.get(edge_ent.curve) else { continue };
        let closest = curve_ops::closest_parameter(curve, edge_ent.range, p, 1e-9);
        if best.is_none_or(|(_, d)| closest.distance < d) {
            best = Some((closest.point, closest.distance));
        }
    }
    for &vertex in &members.vertices {
        let Some(v) = body.vertices.get(vertex) else { continue };
        let distance = (v.position - p).norm();
        if best.is_none_or(|(_, d)| distance < d) {
            best = Some((v.position, distance));
        }
    }
    best
}

fn nearest(body: &Body, members: &ScopeMembers, p: Pnt3) -> Result<Option<(Pnt3, f64)>, KernelError> {
    if members.faces.is_empty() { Ok(nearest_on_wires(body, members, p)) } else { nearest_on_faces(body, &members.faces, p) }
}

/// 📌 Nearest point of `scope` to `point`, and the signed distance for solids and compounds.
pub fn point_distance(body: &Body, scope: &ShapeScope, point: Pnt3) -> Result<PointDistance, KernelError> {
    let members = scope.members(body)?;
    let (closest, distance) = nearest(body, &members, point)?.ok_or_else(|| KernelError::InvalidInput("shape has no geometry to measure against".into()))?;
    let signed = if matches!(scope.kind, ShapeKind::Solid | ShapeKind::Compound) {
        let mut inside = false;
        for &solid in &members.solids {
            if matches!(point_in_solid(body, solid, point, 1e-9)?, crate::brep::engine::PointClassification::Inside) {
                inside = true;
            }
        }
        Some(if inside { -distance } else { distance })
    } else {
        None
    };
    Ok(PointDistance { distance, closest: array(closest), signed })
}

fn samples(body: &Body, members: &ScopeMembers) -> Vec<Pnt3> {
    let mut points: Vec<Pnt3> = members.vertices.iter().filter_map(|&v| body.vertices.get(v)).map(|v| v.position).collect();
    for &edge in &members.edges {
        let Some(edge_ent) = body.edges.get(edge) else { continue };
        let Some(curve) = body.curves3.get(edge_ent.curve) else { continue };
        for i in 0..=EDGE_SAMPLES {
            points.push(curve.eval(edge_ent.range.0 + (edge_ent.range.1 - edge_ent.range.0) * i as f64 / EDGE_SAMPLES as f64));
        }
    }
    points
}

fn edge_pair(body: &Body, a: EdgeId, b: EdgeId) -> Option<(Pnt3, Pnt3, f64)> {
    let (ea, eb) = (body.edges.get(a)?, body.edges.get(b)?);
    let (ca, cb) = (body.curves3.get(ea.curve)?, body.curves3.get(eb.curve)?);
    let mut best: Option<(Pnt3, Pnt3, f64)> = None;
    for i in 0..=EDGE_SAMPLES {
        let p = ca.eval(ea.range.0 + (ea.range.1 - ea.range.0) * i as f64 / EDGE_SAMPLES as f64);
        let q = curve_ops::closest_parameter(cb, eb.range, p, 1e-9);
        if best.is_none_or(|(_, _, d)| q.distance < d) {
            best = Some((p, q.point, q.distance));
        }
    }
    for i in 0..=EDGE_SAMPLES {
        let q = cb.eval(eb.range.0 + (eb.range.1 - eb.range.0) * i as f64 / EDGE_SAMPLES as f64);
        let p = curve_ops::closest_parameter(ca, ea.range, q, 1e-9);
        if best.is_none_or(|(_, _, d)| p.distance < d) {
            best = Some((p.point, q, p.distance));
        }
    }
    best
}

/// 📏 Minimum distance between `a` and `b` with the nearest point on each.
///
/// Fails when either shape has no geometry. Overlapping solids give `0` with `point_a == point_b`.
pub fn shape_distance(body: &Body, a: &ShapeScope, b: &ShapeScope) -> Result<ClosestPair, KernelError> {
    let (ma, mb) = (a.members(body)?, b.members(body)?);
    let mut best: Option<(Pnt3, Pnt3, f64)> = None;
    let mut consider = |pa: Pnt3, pb: Pnt3, distance: f64| {
        if best.is_none_or(|(_, _, d)| distance < d) {
            best = Some((pa, pb, distance));
        }
    };
    for &sa in &ma.solids {
        for &sb in &mb.solids {
            if solids_overlap(body, sa, sb)? {
                let witness = body.vertices.get(*ma.vertices.first().ok_or_else(|| KernelError::MissingEntity("vertex".into()))?).map(|v| v.position).unwrap_or(Pnt3::new(0.0, 0.0, 0.0));
                consider(witness, witness, 0.0);
            }
        }
    }
    for p in samples(body, &ma) {
        if let Some((q, d)) = nearest(body, &mb, p)? {
            consider(p, q, d);
        }
    }
    for q in samples(body, &mb) {
        if let Some((p, d)) = nearest(body, &ma, q)? {
            consider(p, q, d);
        }
    }
    for &ea in &ma.edges {
        for &eb in &mb.edges {
            if let Some((p, q, d)) = edge_pair(body, ea, eb) {
                consider(p, q, d);
            }
        }
    }
    let (mut pa, mut pb, mut distance) = best.ok_or_else(|| KernelError::InvalidInput("a shape has no geometry to measure".into()))?;
    for _ in 0..REFINE_STEPS {
        if distance <= 1e-14 {
            break;
        }
        let Some((next_a, _)) = nearest(body, &ma, pb)? else { break };
        let Some((next_b, next_distance)) = nearest(body, &mb, next_a)? else { break };
        if next_distance >= distance {
            break;
        }
        let converged = distance - next_distance <= 1e-14 * distance.max(1.0);
        (pa, pb, distance) = (next_a, next_b, next_distance);
        if converged {
            break;
        }
    }
    Ok(ClosestPair { distance, point_a: array(pa), point_b: array(pb) })
}
