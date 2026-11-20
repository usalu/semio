//! 🪞️ An exact planar pcurve for a certified rational quadratic boundary.

use semio_framework_3d_brep_curve::{Curve2, Curve3, bspline::KnotVector};
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, matrix::Frame3, predicates::{Orient, orient2d}};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::mem::size_of;

/// 🧾️ Holds the fixed-size projected source recipe before allocating its exact pcurve.
pub struct QuadraticPcurve {
    knots: [f64; 6],
    trim: [f64; 2],
    controls: [Pnt2; 3],
    weights: [f64; 3],
}

impl QuadraticPcurve {
    pub const STORAGE: usize = 9 * size_of::<f64>() + 3 * size_of::<Pnt2>();

    /// 🪢️ Admits a nonempty trim in one clamped source quadratic span.
    pub fn new(source: &Curve3, range: (f64, f64)) -> Option<Self> {
        let Curve3::Nurbs { knots, controls, weights } = source else { return None; };
        let &[a, b, lo, hi, c, d] = knots.knots.as_slice() else { return None; };
        if knots.degree != 2 || controls.len() != 3 || weights.len() != 3 || a != lo || b != lo || c != hi || d != hi
            || !(lo <= range.0 && range.0 < range.1 && range.1 <= hi) || !(hi - lo).is_finite()
        { return None; }
        Some(Self {
            knots: [a, b, lo, hi, c, d],
            trim: [(range.0 - lo) / (hi - lo), (range.1 - lo) / (hi - lo)],
            controls: [Pnt2::default(); 3], weights: [0.; 3],
        })
    }

    /// 🎯️ Borrows and projects one positive-weight authored control, checking its plane distance.
    pub fn project(&mut self, source: &Curve3, index: usize, frame: Frame3, tolerance: f64) -> Option<()> {
        let Curve3::Nurbs { controls, weights, .. } = source else { return None; };
        let p = frame.to_local(*controls.get(index)?);
        let weight = *weights.get(index)?;
        if !p.x.is_finite() || !p.y.is_finite() || !(p.z.abs() <= tolerance) || !weight.is_finite() || weight <= 0. { return None; }
        self.controls[index] = Pnt2::new(p.x, p.y);
        self.weights[index] = weight;
        Some(())
    }

    /// 🛡️ Certifies that the cropped rational arc stays outside the chordal triangle and cannot loop.
    pub fn verifies(&self, frame: Frame3, endpoints: [Pnt3; 2], third: Pnt3, tolerance: f64) -> bool {
        let weight_scale = self.weights.into_iter().fold(0., f64::max);
        let homogeneous = std::array::from_fn::<_, 3, _>(|i| {
            let w = self.weights[i] / weight_scale;
            [self.controls[i].x * w, self.controls[i].y * w, w]
        });
        let blossom = |s, t| {
            let left = mix(homogeneous[0], homogeneous[1], s);
            let right = mix(homogeneous[1], homogeneous[2], s);
            let [x, y, w] = mix(left, right, t);
            Pnt2::new(x / w, y / w)
        };
        let [s, t] = self.trim;
        let [p, q, r] = [blossom(s, s), blossom(s, t), blossom(t, t)];
        if [p, q, r].into_iter().any(|p| !p.x.is_finite() || !p.y.is_finite()) { return false; }
        let [a, b] = endpoints.map(|p| local(frame, p));
        let near = |x: Pnt2, y: Pnt2| (x.x - y.x).hypot(x.y - y.y) <= tolerance;
        if !near(a, p) || !near(b, r) { return false; }
        let chord = r - p;
        let lead = (q - p).dot(chord);
        if !(0. < lead && lead < chord.dot(chord)) { return false; }
        let inside = orient2d(p, r, local(frame, third));
        let bulge = orient2d(p, r, q);
        inside != Orient::Zero && (bulge == Orient::Zero || bulge != inside)
    }

    /// 📦️ Creates the complete fixed-size pcurve under its separately quoted physical grant.
    pub fn materialize(&self) -> Result<Curve2, ValueError> {
        Ok(Curve2::Nurbs {
            knots: KnotVector { knots: exact_vec(self.knots)?, degree: 2 },
            controls: exact_vec(self.controls)?,
            weights: exact_vec(self.weights)?,
        })
    }

    /// ♻️ Counts owned allocations retained by the exact pcurve record.
    pub fn capacity(curve: &Curve2) -> usize {
        let Curve2::Nurbs { knots, controls, weights } = curve else { return 0; };
        (knots.knots.capacity() + weights.capacity()) * size_of::<f64>() + controls.capacity() * size_of::<Pnt2>()
    }
}

fn mix(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] { std::array::from_fn(|i| (1. - t) * a[i] + t * b[i]) }

fn local(frame: Frame3, p: Pnt3) -> Pnt2 {
    let p = frame.to_local(p);
    Pnt2::new(p.x, p.y)
}

fn exact_vec<T, const N: usize>(items: [T; N]) -> Result<Vec<T>, ValueError> {
    let mut out = Vec::new();
    out.try_reserve_exact(N).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "planar spline pcurve allocation failed"))?;
    out.extend(items);
    Ok(out)
}
