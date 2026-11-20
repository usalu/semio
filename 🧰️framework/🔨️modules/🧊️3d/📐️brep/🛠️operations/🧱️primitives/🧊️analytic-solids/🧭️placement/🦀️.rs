//! 🧭️ Exact affine placement of certified primitive recipes.

use super::{Frame3, Pnt3, SolidPrimitive, Tol, Vec3};
use semio_framework_3d_brep_vector::matrix::Affine3;

fn same_scale(first: f64, last: f64) -> bool {
    (first - last).abs() <= first.max(last) * 1e-10
}

fn mapped_axes(frame: Frame3, map: &Affine3) -> Option<(Frame3, f64, f64)> {
    let [x, y, z] = [frame.x, frame.y, frame.z].map(|axis| map.apply_vector(axis));
    let [sx, sy, sz] = [x.norm(), y.norm(), z.norm()];
    if ![sx, sy, sz].into_iter().all(|length| length.is_finite() && length > 0.0) || !same_scale(sx, sy) {
        return None;
    }
    let [x, y, z] = [x.normalized()?, y.normalized()?, z.normalized()?];
    if x.dot(y).abs() > 1e-10 || x.dot(z).abs() > 1e-10 || y.dot(z).abs() > 1e-10 { return None; }
    let frame = Frame3::from_x_z(map.apply_point(frame.origin), x, z)?;
    Some((frame, sx, sz))
}

fn mapped_box(origin: Pnt3, size: [f64; 3], map: &Affine3) -> Option<SolidPrimitive> {
    let mut lower = map.apply_point(origin).to_array();
    let mut extents = [0.0; 3];
    for (direction, length) in [Vec3::X, Vec3::Y, Vec3::Z].into_iter().zip(size) {
        let sides = map.apply_vector(direction * length).to_array();
        if !sides.into_iter().all(f64::is_finite) { return None; }
        let (axis, span) = sides.into_iter().enumerate().max_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))?;
        if extents[axis] != 0.0 || span.abs() <= Tol::DEFAULT.value() || sides.into_iter().enumerate()
            .any(|(other, value)| other != axis && value.abs() > Tol::DEFAULT.value()) {
            return None;
        }
        lower[axis] += span.min(0.0);
        extents[axis] = span.abs();
    }
    let [width, depth, height] = extents;
    Some(SolidPrimitive::Box { origin: Pnt3::from_array(lower), width, depth, height })
}

impl SolidPrimitive {
    /// 🧲️ Reissues the primitive only where its affine image remains in the same analytic family.
    pub fn affinely_transformed(self, map: &Affine3) -> Option<Self> {
        match self {
            Self::Box { origin, width, depth, height } => mapped_box(origin, [width, depth, height], map),
            Self::Sphere { frame, radius } => {
                let (frame, radial, axial) = mapped_axes(frame, map)?;
                same_scale(radial, axial).then_some(Self::Sphere { frame, radius: radius * radial })
            }
            Self::Cylinder { frame, radius, height } => {
                let (frame, radial, axial) = mapped_axes(frame, map)?;
                Some(Self::Cylinder { frame, radius: radius * radial, height: height * axial })
            }
            Self::Cone { frame, radius, height } => {
                let (frame, radial, axial) = mapped_axes(frame, map)?;
                Some(Self::Cone { frame, radius: radius * radial, height: height * axial })
            }
            Self::Torus { frame, major, minor } => {
                let (frame, radial, axial) = mapped_axes(frame, map)?;
                same_scale(radial, axial).then_some(Self::Torus { frame, major: major * radial, minor: minor * radial })
            }
        }
    }
}
