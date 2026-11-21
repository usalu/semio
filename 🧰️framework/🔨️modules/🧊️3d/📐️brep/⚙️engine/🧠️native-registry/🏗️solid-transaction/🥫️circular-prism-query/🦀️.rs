//! 🥫️ Exact distance to the finite profiles and caps of retained cylindrical Booleans.

use semio_framework_3d_brep_construction::circular_prism::{CircleSupports, DiskPairOperation};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::Vec3;
use std::f64::consts::TAU;
use super::*;

fn length(vector: Vec3) -> f64 {
    vector.x.hypot(vector.y).hypot(vector.z)
}

/// 📍️ Measures the signed distance to a bounded disk-pair or disk-rectangle extrusion.
pub fn circular_prism_distance(shape: CircularPrism, point: Pnt3) -> Option<f64> {
    let supports = shape.circle_supports();
    if matches!(supports, CircleSupports::Shared { center: None, .. }) { return None; }
    let delta = point - shape.center;
    let axial = delta.dot(shape.axis);
    let in_plane = delta - shape.axis * axial;
    if !axial.is_finite() || !in_plane.to_array().into_iter().all(f64::is_finite) { return Some(f64::INFINITY); }
    let relative = |base| in_plane - (base - shape.center);
    let mut planar_distance = f64::INFINITY;
    let mut inside_lines = true;
    for i in 0..shape.count {
        let edge = shape.edge(i);
        let distance = match edge.curve {
            Curve3::Circle { frame, radius } => {
                let delta = relative(frame.origin);
                let (x, y) = (delta.dot(frame.x), delta.dot(frame.y));
                let angle = y.atan2(x).rem_euclid(TAU);
                if angle <= edge.range.1 {
                    (x.hypot(y) - radius).abs()
                } else {
                    let [first, last] = edge.vertices.map(|vertex| length(relative(shape.vertex(vertex))));
                    first.min(last)
                }
            }
            Curve3::Line { origin, dir } => {
                let size = length(dir);
                let tangent = dir * (1.0 / size);
                let delta = relative(origin);
                inside_lines &= tangent.cross(shape.axis).dot(delta) <= 0.0;
                let along = delta.dot(tangent).clamp(0.0, size);
                length(delta - tangent * along)
            }
            _ => return None,
        };
        planar_distance = planar_distance.min(distance);
    }
    let within = |center, radius| length(relative(center)) <= radius;
    let inside = match supports {
        CircleSupports::Shared { radius, center } => inside_lines && within(center?, radius),
        CircleSupports::Pair { circles, operation } => {
            let [a, b] = circles.map(|(center, radius)| within(center, radius));
            match operation {
                DiskPairOperation::Intersect => a && b,
                DiskPairOperation::Unite => a || b,
                DiskPairOperation::Cut => a && !b,
            }
        }
    };
    let planar = if inside { -planar_distance } else { planar_distance };
    let axial = (-axial).max(axial - shape.length);
    Some(planar.max(0.0).hypot(axial.max(0.0)) + planar.max(axial).min(0.0))
}
