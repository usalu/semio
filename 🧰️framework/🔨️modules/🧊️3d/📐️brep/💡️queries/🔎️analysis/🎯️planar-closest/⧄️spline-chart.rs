//! ⧄️ Exact affine support and linear trims of a degree-one unit-weight spline face.

use semio_framework_3d_brep_curve::{Curve2, Curve3, bspline::KnotVector};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt3, matrix::Frame3};
use semio_framework_value::{ValueError, ValueRefusalKind};

#[derive(Clone, Copy)]
pub(crate) struct SplineChart {
    pub(crate) frame: Frame3,
    domain: [[f64; 2]; 2],
}

fn unsupported() -> ValueError { ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original affine spline face requires a unit-weight degree-one patch and matching linear trims") }

fn span(knots: &KnotVector) -> Option<[f64; 2]> {
    let &[lo, a, b, hi] = knots.knots.as_slice() else { return None; };
    (knots.degree == 1 && lo == a && hi == b && (hi - lo).is_finite() && lo < hi).then_some([lo, hi])
}

impl SplineChart {
    /// ⧄️ Recovers the affine chart exactly when the original bilinear control net is a parallelogram.
    pub(crate) fn from_surface(surface: &Surface) -> Result<Self, ValueError> {
        let Surface::Nurbs { u_knots, v_knots, controls, weights } = surface else { return Err(unsupported()); };
        let [u0, u1] = span(u_knots).ok_or_else(unsupported)?;
        let [v0, v1] = span(v_knots).ok_or_else(unsupported)?;
        let [first, last] = controls.as_slice() else { return Err(unsupported()); };
        let (&[a, d], &[b, c]) = (first.as_slice(), last.as_slice()) else { return Err(unsupported()); };
        let u = b - a;
        let v = d - a;
        if weights.len() != 2 || weights.iter().any(|row| row.as_slice() != [1.0; 2])
            || c - d != u || c - b != v { return Err(unsupported()); }
        let x = u * (1.0 / (u1 - u0));
        let y = v * (1.0 / (v1 - v0));
        let z = x.cross(y).normalized().ok_or_else(unsupported)?;
        let origin = a - x * u0 - y * v0;
        if !origin.to_array().into_iter().chain([x, y, z].into_iter().flat_map(|axis| axis.to_array())).all(f64::is_finite) {
            return Err(unsupported());
        }
        Ok(Self { frame: Frame3 { origin, x, y, z }, domain: [[u0, u1], [v0, v1]] })
    }

    /// 📎️ Certifies both the original spline edge and its authored pcurve against the same finite span.
    pub(crate) fn segment(&self, curve: &Curve3, trim: &Curve2, range: (f64, f64), vertices: [Pnt3; 2]) -> Result<Curve3, ValueError> {
        let Curve3::Nurbs { knots, controls, weights } = curve else { return Err(unsupported()); };
        let Some([lower, upper]) = span(knots) else { return Err(unsupported()); };
        let [a, b] = vertices;
        if range != (lower, upper) || controls.as_slice() != vertices || weights.as_slice() != [1.0; 2]
            || !matches!(trim, Curve2::Line { .. }) || a == b { return Err(unsupported()); }
        for (t, vertex) in [lower, upper].into_iter().zip(vertices) {
            let uv = trim.eval(t);
            if ![uv.x, uv.y].into_iter().zip(self.domain).all(|(value, [lo, hi])| (lo..=hi).contains(&value)) {
                return Err(unsupported());
            }
            let expected = self.frame.origin + self.frame.x * uv.x + self.frame.y * uv.y;
            let scale = expected.to_array().into_iter().chain(vertex.to_array()).map(f64::abs).fold(0.0, f64::max);
            let error = (expected - vertex).to_array().into_iter().map(f64::abs).fold(0.0, f64::max);
            if !expected.to_array().into_iter().all(f64::is_finite) || error > 128.0 * f64::EPSILON * scale {
                return Err(unsupported());
            }
        }
        Ok(Curve3::Line { origin: a, dir: b - a })
    }
}
