//! 📦️ Minimum separation of certified boxes and spheres.

use super::linear_distance::original_linear_distance;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};
use semio_framework_value::{ValueError, ValueRefusalKind};

type Placement = (Pnt3, [Vec3; 3], [f64; 3]);
type Cylinder = (Frame3, f64, f64);

fn vertices((origin, axes, sizes): Placement) -> [Pnt3; 8] {
    std::array::from_fn(|corner| {
        (0..3).filter(|axis| corner & (1 << axis) != 0)
            .fold(origin, |point, axis| point + axes[axis] * sizes[axis])
    })
}

fn point_distance((origin, axes, sizes): Placement, point: Pnt3) -> f64 {
    let outside = |axis: usize| {
        let offset = (point - origin).dot(axes[axis]);
        offset - offset.clamp(0.0, sizes[axis])
    };
    outside(0).hypot(outside(1)).hypot(outside(2))
}

fn certified_gap(distance: f64) -> Result<f64, ValueError> {
    distance.is_finite().then(|| distance.max(0.0)).ok_or_else(|| {
        ValueError::literal(ValueRefusalKind::InvalidValue, "solid distance exceeds finite numeric range")
    })
}

/// 🌐️ Measures the minimum separation between two certified solid spheres.
pub fn sphere_pair_distance((a, ra): (Pnt3, f64), (b, rb): (Pnt3, f64)) -> Result<f64, ValueError> {
    let delta = b - a;
    let distance = delta.x.hypot(delta.y).hypot(delta.z);
    certified_gap(distance - ra.max(rb) - ra.min(rb))
}

/// 🔘️ Converts the signed point distance at a sphere center to material separation.
pub fn sphere_boundary_gap(center_distance: f64, radius: f64) -> Result<f64, ValueError> {
    certified_gap(center_distance - radius)
}

/// 🥫️ Computes the Euclidean separation of two finite cylinders on parallel axes.
pub fn parallel_cylinder_distance((a, ra, ha): Cylinder, (b, rb, hb): Cylinder) -> Result<Option<f64>, ValueError> {
    if a.z != b.z && a.z != -b.z { return Ok(None); }
    let offset = b.origin - a.origin;
    let axial_offset = offset.dot(a.z);
    let radial_offset = offset - a.z * axial_offset;
    let radial = radial_offset.x.hypot(radial_offset.y).hypot(radial_offset.z);
    let radial_gap = certified_gap(radial - ra.max(rb) - ra.min(rb))?;
    let half_a = ha * 0.5;
    let half_b = hb * 0.5;
    let directed_half_b = if a.z == b.z { half_b } else { -half_b };
    let centers = axial_offset + (directed_half_b - half_a);
    let axial_gap = certified_gap(centers.abs() - half_a.max(half_b) - half_a.min(half_b))?;
    certified_gap(radial_gap.hypot(axial_gap)).map(Some)
}

/// 🧭️ Measures a finite cylinder against a box that shares its axial direction.
pub fn axial_cylinder_box_distance((frame, radius, height): Cylinder, (origin, axes, sizes): Placement) -> Result<Option<f64>, ValueError> {
    let Some(axis) = axes.iter().position(|axis| *axis == frame.z || *axis == -frame.z) else { return Ok(None); };
    let offset = frame.origin - origin;
    let start = offset.dot(axes[axis]);
    let end = start + if axes[axis] == frame.z { height } else { -height };
    let axial_gap = certified_gap((start.min(end) - sizes[axis]).max(-start.max(end)))?;
    let transverse = |index: usize| {
        let value = offset.dot(axes[index]);
        value - value.clamp(0.0, sizes[index])
    };
    let radial_gap = certified_gap(transverse((axis + 1) % 3).hypot(transverse((axis + 2) % 3)) - radius)?;
    certified_gap(radial_gap.hypot(axial_gap)).map(Some)
}

/// 🧊️ Measures a certified solid sphere against a certified oriented box.
pub fn sphere_box_distance((center, radius): (Pnt3, f64), placement: Placement) -> Result<f64, ValueError> {
    sphere_boundary_gap(point_distance(placement, center), radius)
}

fn edges((_, axes, sizes): Placement, vertices: &[Pnt3; 8]) -> [(Curve3, f64); 12] {
    std::array::from_fn(|index| {
        let axis = index / 4;
        let offset = index % 4;
        let corner = ((offset & 1) << ((axis + 1) % 3)) | ((offset >> 1) << ((axis + 2) % 3));
        (Curve3::Line { origin: vertices[corner], dir: axes[axis] }, sizes[axis])
    })
}

/// 📏️ Measures two certified separated or touching boxes through their nearest features.
pub fn separated_box_distance(first: Placement, second: Placement) -> Result<f64, ValueError> {
    let offset = second.0 - first.0;
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "box distance exceeds finite numeric range");
    let distance = if first.1 == second.1 {
        let gap = |i: usize| {
            let span = offset.dot(first.1[i]);
            (span - first.2[i]).max(-span - second.2[i]).max(0.0)
        };
        gap(0).hypot(gap(1)).hypot(gap(2))
    } else {
        let origin = Pnt3::from_array([0.0; 3]);
        let a = (origin, first.1, first.2);
        let b = (origin + offset, second.1, second.2);
        let (a_vertices, b_vertices) = (vertices(a), vertices(b));
        let mut best = a_vertices.into_iter().map(|point| point_distance(b, point))
            .chain(b_vertices.into_iter().map(|point| point_distance(a, point)))
            .fold(f64::INFINITY, f64::min);
        let right_edges = edges(b, &b_vertices);
        for (left, left_length) in edges(a, &a_vertices) {
            for (right, right_length) in &right_edges {
                best = best.min(original_linear_distance((&left, Some((0.0, left_length))),
                    (right, Some((0.0, *right_length))))?);
            }
        }
        best
    };
    distance.is_finite().then_some(distance).ok_or_else(invalid)
}
