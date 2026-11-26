//! 🔗️ Native trims shared by the spherical and planar faces of a corner union.

use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim, planar_pcurve};
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_surface::curved_boolean::SphericalCornerBox;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt2, Vec2};
use std::f64::consts::FRAC_PI_2;

pub fn edge(shape: SphericalCornerBox, index: usize) -> PrimitiveEdge {
    let (curve, range, vertices) = shape.edge(index);
    PrimitiveEdge { curve, range, vertices }
}

pub fn surface(shape: SphericalCornerBox, face: usize) -> Surface {
    if face >= SphericalCornerBox::SPHERE_FACES {
        return Surface::Plane { frame: shape.plane(face - SphericalCornerBox::SPHERE_FACES) };
    }
    let (quadrant, _) = SphericalCornerBox::sphere_chart(face);
    let frame = match shape.edge(quadrant).0 {
        semio_framework_3d_brep_curve::Curve3::Circle { frame, .. } => frame,
        _ => unreachable!("spherical corner box equator"),
    };
    Surface::Sphere { frame, radius: shape.radius }
}

pub fn trim(shape: SphericalCornerBox, face: usize, member: usize) -> PrimitiveTrim {
    if face >= SphericalCornerBox::SPHERE_FACES {
        let face = face - SphericalCornerBox::SPHERE_FACES;
        let (edge, forward) = shape.plane_edge(face, member);
        let (curve, range, _) = shape.edge(edge);
        return PrimitiveTrim { edge, forward, range, curve: planar_pcurve(&curve, shape.plane(face)) };
    }
    let (quadrant, upper) = SphericalCornerBox::sphere_chart(face);
    let (start, end, lower, higher, seams) = if upper {
        (0., FRAC_PI_2, quadrant, 12 + face, 8)
    } else {
        (-FRAC_PI_2, 0., 12 + face, quadrant, 4)
    };
    let (edge, forward, u, v, du, dv) = [
        (lower, true, 0., start, 1., 0.),
        (seams + (quadrant + 1) % 4, true, FRAC_PI_2, 0., 0., 1.),
        (higher, false, 0., end, 1., 0.),
        (seams + quadrant, false, 0., 0., 0., 1.),
    ][member];
    PrimitiveTrim {
        edge, forward, range: if member % 2 == 0 {(0., FRAC_PI_2)} else {(start, end)},
        curve: Curve2::Line { origin: Pnt2::new(u,v), dir: Vec2::new(du,dv) },
    }
}
