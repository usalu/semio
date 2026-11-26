//! 🪢️ Shared circular trims for the spherical torus cut.

use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim};
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_surface::curved_boolean::SphericalTorusCut;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt2, Vec2};
use std::f64::consts::TAU;

pub fn edge(shape: SphericalTorusCut, index: usize) -> PrimitiveEdge {
    let (curve, range, vertices) = shape.edge(index);
    PrimitiveEdge { curve, range, vertices }
}

pub fn surface(shape: SphericalTorusCut, face: usize) -> Surface {
    if face == 2 {
        Surface::Torus { frame: shape.frame, major_radius: shape.major, minor_radius: shape.minor }
    } else {
        Surface::Sphere { frame: shape.frame, radius: shape.radius }
    }
}

pub fn trim(shape: SphericalTorusCut, face: usize, member: usize) -> PrimitiveTrim {
    let [lower, upper, seam] = [[0, 1, 4], [2, 3, 5], [2, 1, 6]][face];
    let [start, end] = shape.band(face);
    let (edge, forward, u, v, du, dv) = [
        (lower, true, 0., start, 1., 0.),
        (seam, true, TAU, 0., 0., 1.),
        (upper, false, 0., end, 1., 0.),
        (seam, false, 0., 0., 0., 1.),
    ][member];
    PrimitiveTrim {
        edge, forward, range: (if member % 2 == 0 { (0., TAU) } else { (start, end) }),
        curve: Curve2::Line { origin: Pnt2::new(u, v), dir: Vec2::new(du, dv) },
    }
}
