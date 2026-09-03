//! 🌀️ Analytic and free-form 3D curves ([`Curve3`]) and their 2D parameter-space counterparts
//! ([`Curve2`], the pcurve representation). Every variant supports position/derivative evaluation
//! and an *exact* [`Curve3::to_nurbs`]/[`Curve2::to_nurbs`] conversion — the single representation
//! every downstream algorithm (intersection, tessellation, STEP export) can fall back to when it
//! doesn't have an analytic fast path for a particular curve kind.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/➰️curve` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, together with
//! its `🪢️bspline`/`🎢️bezier`/`✂️curve-ops` siblings as local children (per this file's own
//! pre-mounted-stub note — no 1:1 file mapping), mirroring the `⚙️engine` → `📄️step`/`📦️mesh-io`
//! local-mount pattern from wave PEEL3.

// #region 🔖️Submodules

#[path = "🎢️bezier/🦀️.rs"]
pub mod bezier;
#[path = "🪢️bspline/🦀️.rs"]
pub mod bspline;
#[path = "✂️curve-ops/🦀️.rs"]
pub mod curve_ops;

// #endregion 🔖️Submodules

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};
use bspline::{curve_derivatives_rational, de_boor, KnotVector};

// #region 🔖️Curve3

/// 🌀️ A 3D curve. Each variant's *natural* domain is documented on the variant; a curve's actual
/// used range (e.g. an edge's `(t0, t1)`) is stored by the topology layer, not here — this keeps
/// geometry shareable between edges that trim the same underlying curve differently.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Curve3 {
    /// 🌀️ `origin + t·dir`. Natural domain `(-∞, ∞)`. `dir` need not be unit.
    Line { origin: Pnt3, dir: Vec3 },
    /// 🌀️ A full circle in `frame`'s xy-plane. Natural domain `[0, 2π)`, periodic.
    Circle { frame: Frame3, radius: f64 },
    /// 🌀️ A full ellipse in `frame`'s xy-plane (`frame.x` = major axis, `frame.y` = minor axis).
    /// Natural domain `[0, 2π)`, periodic.
    Ellipse { frame: Frame3, major_radius: f64, minor_radius: f64 },
    /// 🌀️ A rational B-spline curve. Natural domain = the knot vector's domain.
    Nurbs { knots: KnotVector, controls: Vec<Pnt3>, weights: Vec<f64> },
}

/// 🌀️ An explicit rational-NURBS representation, returned by [`Curve3::to_nurbs`] /
/// [`Curve2::to_nurbs`] — every curve kind's common denominator.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct NurbsCurve3 {
    pub knots: KnotVector,
    pub controls: Vec<Pnt3>,
    pub weights: Vec<f64>,
}

impl Curve3 {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn domain(&self) -> (f64, f64) {
        match self {
            Curve3::Line { .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => (0.0, std::f64::consts::TAU),
            Curve3::Nurbs { knots, .. } => knots.domain(),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_periodic(&self) -> bool {
        match self {
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => true,
            Curve3::Nurbs { knots, .. } => knots.is_periodic(),
            Curve3::Line { .. } => false,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn period(&self) -> Option<f64> {
        match self {
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => Some(std::f64::consts::TAU),
            Curve3::Nurbs { knots, .. } if knots.is_periodic() => {
                let (lo, hi) = knots.domain();
                Some(hi - lo)
            }
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval(&self, t: f64) -> Pnt3 {
        match self {
            Curve3::Line { origin, dir } => *origin + *dir * t,
            Curve3::Circle { frame, radius } => frame.to_world(Pnt3::new(radius * t.cos(), radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world(Pnt3::new(major_radius * t.cos(), minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { knots, controls, weights } => eval_nurbs_curve(knots, controls, weights, t),
        }
    }
    /// 🌀️ First derivative `dC/dt`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn d1(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { dir, .. } => *dir,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.sin(), radius * t.cos(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.sin(), minor_radius * t.cos(), 0.0)),
            Curve3::Nurbs { .. } => self.derivatives(t, 1)[1],
        }
    }
    /// 🌀️ Second derivative `d²C/dt²`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn d2(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { .. } => Vec3::ZERO,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.cos(), -radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.cos(), -minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { .. } => self.derivatives(t, 2)[2],
        }
    }
    /// 🌀️ All derivatives `d^0C/dt^0 .. d^orderC/dt^order` (index 0 = position) in one pass — exact
    /// through the rational de Boor `A_k(u)` recurrence for [`Curve3::Nurbs`] (any `order`, not
    /// just 1/2), closed-form for the analytic kinds. `d1`/`d2` are thin wrappers over this.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn derivatives(&self, t: f64, order: usize) -> Vec<Vec3> {
        match self {
            Curve3::Nurbs { knots, controls, weights } => {
                let controls_h: Vec<Vec<f64>> = controls.iter().zip(weights).map(|(p, &w)| vec![p.x * w, p.y * w, p.z * w, w]).collect();
                curve_derivatives_rational(knots, &controls_h, t, order)
                    .into_iter()
                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                    .collect()
            }
            _ => (0..=order)
                .map(|k| match k {
                    0 => self.eval(t).to_vec(),
                    1 => self.d1_analytic(t),
                    2 => self.d2_analytic(t),
                    _ => Vec3::ZERO,
                })
                .collect(),
        }
    }
    /// 🌀️ First derivative for the non-`Nurbs` (analytic) variants only — factored out of
    /// [`Self::d1`]/[`Self::derivatives`] so the latter can call it without recursing back into
    /// itself for the `Nurbs` case.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn d1_analytic(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { dir, .. } => *dir,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.sin(), radius * t.cos(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.sin(), minor_radius * t.cos(), 0.0)),
            Curve3::Nurbs { .. } => Vec3::ZERO,
        }
    }
    /// 🌀️ Second derivative for the non-`Nurbs` (analytic) variants only, see [`Self::d1_analytic`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn d2_analytic(&self, t: f64) -> Vec3 {
        match self {
            Curve3::Line { .. } => Vec3::ZERO,
            Curve3::Circle { frame, radius } => frame.to_world_vector(Vec3::new(-radius * t.cos(), -radius * t.sin(), 0.0)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => frame.to_world_vector(Vec3::new(-major_radius * t.cos(), -minor_radius * t.sin(), 0.0)),
            Curve3::Nurbs { .. } => Vec3::ZERO,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tangent(&self, t: f64) -> Option<Vec3> {
        self.d1(t).normalized()
    }
    /// 🌀️ Signed curvature magnitude `|C' × C''| / |C'|³` (the standard space-curve formula).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curvature(&self, t: f64) -> f64 {
        let d1 = self.d1(t);
        let d2 = self.d2(t);
        let speed = d1.norm();
        if speed <= f64::EPSILON {
            return 0.0;
        }
        d1.cross(d2).norm() / speed.powi(3)
    }
    /// 🌀️ An exact rational-NURBS representation over `domain` (required for [`Curve3::Line`],
    /// whose natural domain is unbounded). Arcs longer than 120° are split into equal spans of
    /// the standard rational-quadratic circular-arc construction for numerical conditioning.
    /// The returned curve traces exactly the same points over `domain` and agrees with `self` at
    /// `domain.0`/`domain.1` and every span breakpoint in between — but, as for any rational
    /// quadratic circle/ellipse representation, its *own* parametrization is not angle-linear
    /// except at those breakpoints (a well-known property of the construction, not an
    /// approximation: every point it produces still lies exactly on the circle/ellipse).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_nurbs(&self, domain: (f64, f64)) -> NurbsCurve3 {
        match self {
            Curve3::Line { origin, dir } => {
                let p0 = *origin + *dir * domain.0;
                let p1 = *origin + *dir * domain.1;
                NurbsCurve3 { knots: KnotVector::new(vec![domain.0, domain.0, domain.1, domain.1], 1, 2).unwrap(), controls: vec![p0, p1], weights: vec![1.0, 1.0] }
            }
            Curve3::Circle { frame, radius } => arc_to_nurbs(frame, *radius, *radius, domain),
            Curve3::Ellipse { frame, major_radius, minor_radius } => arc_to_nurbs(frame, *major_radius, *minor_radius, domain),
            Curve3::Nurbs { knots, controls, weights } => NurbsCurve3 { knots: knots.clone(), controls: controls.clone(), weights: weights.clone() },
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn eval_nurbs_curve(knots: &KnotVector, controls: &[Pnt3], weights: &[f64], t: f64) -> Pnt3 {
    let hx: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.x * w).collect();
    let hy: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.y * w).collect();
    let hz: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.z * w).collect();
    let w = de_boor(knots, weights, t);
    Pnt3::new(de_boor(knots, &hx, t) / w, de_boor(knots, &hy, t) / w, de_boor(knots, &hz, t) / w)
}

/// 🌀️ Converts a circular/elliptical arc over `domain` into an exact rational-quadratic NURBS,
/// splitting into `⌈span / 120°⌉` equal-angle spans (the standard well-conditioned construction:
/// each span's middle control point sits at `radius / cos(half-span)` with weight `cos(half-span)`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arc_to_nurbs(frame: &Frame3, radius_x: f64, radius_y: f64, domain: (f64, f64)) -> NurbsCurve3 {
    let span = domain.1 - domain.0;
    let max_span = std::f64::consts::TAU / 3.0; // 120 degrees
    let n_spans = (span.abs() / max_span).ceil().max(1.0) as usize;
    let step = span / n_spans as f64;
    let mut controls = Vec::with_capacity(2 * n_spans + 1);
    let mut weights = Vec::with_capacity(2 * n_spans + 1);
    let local_point = |angle: f64, r_scale: f64| Pnt3::new(radius_x * angle.cos() * r_scale, radius_y * angle.sin() * r_scale, 0.0);
    for span_i in 0..n_spans {
        let a0 = domain.0 + step * span_i as f64;
        let a1 = a0 + step;
        let half = (a1 - a0) * 0.5;
        let mid = (a0 + a1) * 0.5;
        let w1 = half.cos();
        let p0 = local_point(a0, 1.0);
        let p2 = local_point(a1, 1.0);
        let p1 = local_point(mid, 1.0 / w1);
        if span_i == 0 {
            controls.push(frame.to_world(p0));
            weights.push(1.0);
        }
        controls.push(frame.to_world(p1));
        weights.push(w1);
        controls.push(frame.to_world(p2));
        weights.push(1.0);
    }
    let mut knots = vec![domain.0, domain.0, domain.0];
    for span_i in 1..n_spans {
        let knot = domain.0 + step * span_i as f64;
        knots.push(knot);
        knots.push(knot);
    }
    knots.push(domain.1);
    knots.push(domain.1);
    knots.push(domain.1);
    NurbsCurve3 { knots: KnotVector::new(knots, 2, controls.len()).unwrap(), controls, weights }
}

// #endregion 🔖️Curve3

// #region 🔖️Curve2

/// 🌀️ A 2D curve, used as the pcurve type: a curve living in a face's `(u, v)` parameter domain.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Curve2 {
    Line { origin: Pnt2, dir: Vec2 },
    Circle { center: Pnt2, radius: f64 },
    Ellipse { center: Pnt2, x_axis: Vec2, major_radius: f64, minor_radius: f64 },
    Nurbs { knots: KnotVector, controls: Vec<Pnt2>, weights: Vec<f64> },
}

impl Curve2 {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn domain(&self) -> (f64, f64) {
        match self {
            Curve2::Line { .. } => (f64::NEG_INFINITY, f64::INFINITY),
            Curve2::Circle { .. } | Curve2::Ellipse { .. } => (0.0, std::f64::consts::TAU),
            Curve2::Nurbs { knots, .. } => knots.domain(),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval(&self, t: f64) -> Pnt2 {
        match self {
            Curve2::Line { origin, dir } => *origin + *dir * t,
            Curve2::Circle { center, radius } => *center + Vec2::new(radius * t.cos(), radius * t.sin()),
            Curve2::Ellipse { center, x_axis, major_radius, minor_radius } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                *center + x * (major_radius * t.cos()) + y * (minor_radius * t.sin())
            }
            Curve2::Nurbs { knots, controls, weights } => eval_nurbs_curve2(knots, controls, weights, t),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn d1(&self, t: f64) -> Vec2 {
        match self {
            Curve2::Line { dir, .. } => *dir,
            Curve2::Circle { radius, .. } => Vec2::new(-radius * t.sin(), radius * t.cos()),
            Curve2::Ellipse { x_axis, major_radius, minor_radius, .. } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                x * (-major_radius * t.sin()) + y * (minor_radius * t.cos())
            }
            Curve2::Nurbs { knots, controls, weights } => {
                let controls_h: Vec<Vec<f64>> = controls.iter().zip(weights).map(|(p, &w)| vec![p.x * w, p.y * w, w]).collect();
                let derivs = curve_derivatives_rational(knots, &controls_h, t, 1);
                Vec2::new(derivs[1][0], derivs[1][1])
            }
        }
    }
    /// 🌀️ Second derivative `d²C/dt²`, exact via the rational de Boor recurrence for
    /// [`Curve2::Nurbs`], closed-form for the analytic kinds.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn d2(&self, t: f64) -> Vec2 {
        match self {
            Curve2::Line { .. } => Vec2::ZERO,
            Curve2::Circle { radius, .. } => Vec2::new(-radius * t.cos(), -radius * t.sin()),
            Curve2::Ellipse { x_axis, major_radius, minor_radius, .. } => {
                let x = x_axis.normalized().unwrap_or(Vec2::new(1.0, 0.0));
                let y = x.perp();
                x * (-major_radius * t.cos()) + y * (-minor_radius * t.sin())
            }
            Curve2::Nurbs { knots, controls, weights } => {
                let controls_h: Vec<Vec<f64>> = controls.iter().zip(weights).map(|(p, &w)| vec![p.x * w, p.y * w, w]).collect();
                let derivs = curve_derivatives_rational(knots, &controls_h, t, 2);
                Vec2::new(derivs[2][0], derivs[2][1])
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn eval_nurbs_curve2(knots: &KnotVector, controls: &[Pnt2], weights: &[f64], t: f64) -> Pnt2 {
    let hx: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.x * w).collect();
    let hy: Vec<f64> = controls.iter().zip(weights).map(|(p, w)| p.y * w).collect();
    let w = de_boor(knots, weights, t);
    Pnt2::new(de_boor(knots, &hx, t) / w, de_boor(knots, &hy, t) / w)
}

// #endregion 🔖️Curve2

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fd_d1(curve: &Curve3, t: f64) -> Vec3 {
        let h = 1e-6;
        (curve.eval(t + h) - curve.eval(t - h)) * (1.0 / (2.0 * h))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fd_d2(curve: &Curve3, t: f64) -> Vec3 {
        let h = 1e-4;
        let a = curve.eval(t + h).to_vec();
        let b = curve.eval(t).to_vec();
        let c = curve.eval(t - h).to_vec();
        (a - b * 2.0 + c) * (1.0 / (h * h))
    }

    #[semio_framework_async_macros::async_test]
    async fn line_eval_and_derivatives() {
        let l = Curve3::Line { origin: Pnt3::new(1.0, 2.0, 3.0), dir: Vec3::new(2.0, 0.0, 0.0) };
        assert_eq!(l.eval(0.5), Pnt3::new(2.0, 2.0, 3.0));
        assert_eq!(l.d1(0.5), Vec3::new(2.0, 0.0, 0.0));
        assert_eq!(l.d2(0.5), Vec3::ZERO);
        assert_eq!(l.curvature(0.5), 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_eval_stays_on_circle_and_derivatives_match_finite_differences() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 3.0 };
        for i in 0..10 {
            let t = i as f64 * 0.5;
            let p = c.eval(t);
            assert!((p.to_vec().norm() - 3.0).abs() < 1e-9);
            assert!((c.d1(t) - fd_d1(&c, t)).norm() < 1e-5);
            assert!((c.d2(t) - fd_d2(&c, t)).norm() < 1e-2);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_curvature_equals_reciprocal_radius() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 1.0, 1.0), Vec3::X).unwrap();
        let c = Curve3::Circle { frame, radius: 2.5 };
        for t in [0.0, 1.0, 3.0, 5.5] {
            assert!((c.curvature(t) - 1.0 / 2.5).abs() < 1e-6, "curvature mismatch at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn ellipse_derivatives_match_finite_differences() {
        let frame = Frame3::WORLD;
        let e = Curve3::Ellipse { frame, major_radius: 4.0, minor_radius: 2.0 };
        for i in 0..8 {
            let t = i as f64 * 0.7;
            assert!((e.d1(t) - fd_d1(&e, t)).norm() < 1e-5, "d1 mismatch at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn line_to_nurbs_matches_line_eval() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 2.0, 3.0) };
        let nurbs = l.to_nurbs((0.0, 2.0));
        for i in 0..=10 {
            let t = i as f64 / 10.0 * 2.0;
            let via_nurbs = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
            assert!(via_nurbs.distance(l.eval(t)) < 1e-9, "mismatch at t={t}");
        }
    }

    /// 🌀️ The invariant a rational arc-to-NURBS conversion actually guarantees: every produced
    /// point lies exactly on the circle (radius match at the frame's own scale), and the curve
    /// agrees with the original at `domain.0`/`domain.1` — NOT pointwise parameter equality
    /// in between, since the standard construction is not angle-linear except at breakpoints
    /// (confirmed by hand + a standalone check: see phase-2 scope note).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_nurbs_traces_circle(nurbs: &NurbsCurve3, frame: &Frame3, radius: f64, domain: (f64, f64), samples: usize) {
        for i in 0..=samples {
            let t = domain.0 + (domain.1 - domain.0) * (i as f64 / samples as f64);
            let p = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
            let local = frame.to_local(p);
            assert!((local.to_vec().norm() - radius).abs() < 1e-8, "point at t={t} is not on the circle: radius {}", local.to_vec().norm());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_to_nurbs_traces_the_circle_exactly_for_small_arc() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 5.0 };
        let domain = (0.2, 0.2 + std::f64::consts::FRAC_PI_3); // 60 degrees, single span
        let nurbs = c.to_nurbs(domain);
        assert_nurbs_traces_circle(&nurbs, &frame, 5.0, domain, 20);
        assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-9);
        assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_to_nurbs_traces_the_circle_exactly_for_full_circle_multi_span() {
        let frame = Frame3::from_normal(Pnt3::new(2.0, -1.0, 0.5), Vec3::new(0.3, 0.2, 1.0)).unwrap();
        let c = Curve3::Circle { frame, radius: 1.7 };
        let domain = c.domain();
        let nurbs = c.to_nurbs(domain);
        assert!(nurbs.controls.len() > 3, "a full circle must be split into more than one span");
        assert_nurbs_traces_circle(&nurbs, &frame, 1.7, domain, 60);
        assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-8);
        assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-8);
    }

    #[semio_framework_async_macros::async_test]
    async fn ellipse_to_nurbs_traces_the_ellipse_exactly() {
        let frame = Frame3::WORLD;
        let major = 3.0;
        let minor = 1.0;
        let e = Curve3::Ellipse { frame, major_radius: major, minor_radius: minor };
        let domain = (0.0, std::f64::consts::PI * 1.5);
        let nurbs = e.to_nurbs(domain);
        for i in 0..=30 {
            let t = domain.0 + (domain.1 - domain.0) * (i as f64 / 30.0);
            let p = eval_nurbs_curve(&nurbs.knots, &nurbs.controls, &nurbs.weights, t);
            let local = frame.to_local(p);
            let residual = (local.x / major).powi(2) + (local.y / minor).powi(2) - 1.0;
            assert!(residual.abs() < 1e-8, "point at t={t} is not on the ellipse: residual={residual}");
        }
        assert!(nurbs.controls[0].distance(e.eval(domain.0)) < 1e-9);
        assert!(nurbs.controls.last().unwrap().distance(e.eval(domain.1)) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn curve2_line_and_circle_eval() {
        let l = Curve2::Line { origin: Pnt2::new(0.0, 0.0), dir: Vec2::new(1.0, 1.0) };
        assert_eq!(l.eval(2.0), Pnt2::new(2.0, 2.0));
        let c = Curve2::Circle { center: Pnt2::new(1.0, 1.0), radius: 2.0 };
        let p = c.eval(0.0);
        assert!(((p - Pnt2::new(1.0, 1.0)).norm() - 2.0).abs() < 1e-9);
    }

    /// 🌀️ A quarter-circle as an exact rational-quadratic NURBS (radius 1, centered at origin,
    /// `t=0` at `(1,0)`, `t=1` at `(0,1)`) — the same construction as [`Curve3::to_nurbs`] would
    /// produce for a 90° arc, built by hand here so the test has an independent oracle.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn quarter_circle_nurbs() -> Curve3 {
        let w = std::f64::consts::FRAC_PI_4.cos();
        Curve3::Nurbs {
            knots: KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).unwrap(),
            controls: vec![Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)],
            weights: vec![1.0, w, 1.0],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn nurbs_circle_d1_d2_are_exact_not_finite_difference() {
        let c = quarter_circle_nurbs();
        for i in 1..20 {
            let t = i as f64 / 20.0;
            let p = c.eval(t).to_vec();
            let d1 = c.d1(t);
            let d2 = c.d2(t);
            // On a unit circle: |C|=1, C·C'=0 (tangent ⟂ radius), and the exact curvature formula
            // |C'×C''|/|C'|³ must equal 1 (reciprocal of unit radius) to within 1e-9 — a much
            // tighter bound than the old finite-difference implementation could ever satisfy.
            assert!((p.norm() - 1.0).abs() < 1e-9, "off unit circle at t={t}");
            assert!(p.dot(d1).abs() < 1e-9, "tangent not perpendicular to radius at t={t}");
            let speed = d1.norm();
            let curvature = d1.cross(d2).norm() / speed.powi(3);
            assert!((curvature - 1.0).abs() < 1e-9, "curvature mismatch at t={t}: {curvature}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn nurbs_d1_matches_analytic_circle_d1_within_tight_tolerance() {
        let analytic = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
        let nurbs = quarter_circle_nurbs();
        for i in 1..20 {
            let nurbs_t = i as f64 / 20.0;
            // The rational-quadratic parametrization is not angle-linear (see `to_nurbs`'s own
            // doc), so derive the true angle from the NURBS point itself rather than assuming a
            // linear map, then compare unit tangent *directions* (which depend only on position on
            // the circle, not on parametrization speed).
            let p = nurbs.eval(nurbs_t);
            let angle = p.y.atan2(p.x);
            let a_dir = analytic.d1(angle).normalized().unwrap();
            let n_dir = nurbs.d1(nurbs_t).normalized().unwrap();
            assert!((a_dir - n_dir).norm() < 1e-6, "tangent direction mismatch at nurbs_t={nurbs_t} (angle={angle})");
        }
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn circle_to_nurbs_traces_the_circle_exactly_for_random_arcs() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(53);
            for _ in 0..100 {
                let frame =
                    Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z))
                        .unwrap();
                let radius = 0.1 + rng.next_f64() * 10.0;
                let c = Curve3::Circle { frame, radius };
                let a0 = rng.next_f64() * std::f64::consts::TAU;
                let span = rng.next_f64() * std::f64::consts::TAU * 1.5;
                let domain = (a0, a0 + span);
                let nurbs = c.to_nurbs(domain);
                assert_nurbs_traces_circle(&nurbs, &frame, radius, domain, 25);
                assert!(nurbs.controls[0].distance(c.eval(domain.0)) < 1e-7, "start point mismatch radius={radius} domain={domain:?}");
                assert!(nurbs.controls.last().unwrap().distance(c.eval(domain.1)) < 1e-7, "end point mismatch radius={radius} domain={domain:?}");
            }
        }
    }
}
// #endregion 🔖️Tests

// #region 🔁️Transform

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

impl Curve3 {
    /// 🌀️ Exact affine transform. `Line` stays exact under ANY invertible affine map — a line's
    /// image under any invertible linear map is again a line, with no orthonormality or domain
    /// constraint (unlike every other variant). `Circle`/`Ellipse` stay analytic under a similarity
    /// (uniform scale/rotation/translation, optionally with reflection — [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3::is_similarity`]),
    /// with `frame` mapped via [`Frame3::transformed`] and `radius`/`major_radius`/`minor_radius`
    /// scaled uniformly; a non-similarity map converts to the equivalent exact NURBS
    /// ([`Self::to_nurbs`] over the curve's own bounded natural domain) and transforms its control
    /// points (rational weights are affine-invariant, so they carry over unchanged either way).
    /// `Nurbs` always just transforms its control points.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn transformed(&self, map: &Affine3) -> Curve3 {
        match self {
            Curve3::Line { origin, dir } => Curve3::Line { origin: map.apply_point(*origin), dir: map.apply_vector(*dir) },
            Curve3::Nurbs { knots, controls, weights } => Curve3::Nurbs { knots: knots.clone(), controls: controls.iter().map(|p| map.apply_point(*p)).collect(), weights: weights.clone() },
            Curve3::Circle { frame, radius } => match map.is_similarity() {
                Some((_, scale, _)) => Curve3::Circle { frame: frame.transformed(map, scale), radius: radius * scale },
                None => self.transformed_via_nurbs(map),
            },
            Curve3::Ellipse { frame, major_radius, minor_radius } => match map.is_similarity() {
                Some((_, scale, _)) => Curve3::Ellipse { frame: frame.transformed(map, scale), major_radius: major_radius * scale, minor_radius: minor_radius * scale },
                None => self.transformed_via_nurbs(map),
            },
        }
    }
    /// 🌀️ The shared non-similarity fallback: convert to NURBS over the curve's own bounded
    /// natural domain, then transform every control point (weights unchanged).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transformed_via_nurbs(&self, map: &Affine3) -> Curve3 {
        let nurbs = self.to_nurbs(self.domain());
        Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls.into_iter().map(|p| map.apply_point(p)).collect(), weights: nurbs.weights }
    }
}

// #region 🔖️Tests
#[cfg(test)]
mod transform_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn line_transformed_matches_mapped_eval() {
        let l = Curve3::Line { origin: Pnt3::new(1.0, 2.0, 3.0), dir: Vec3::new(2.0, -1.0, 0.5) };
        let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 3.0, 5.0)).compose(&Affine3::translation(Vec3::new(1.0, 0.0, 0.0)));
        assert!(map.is_similarity().is_none(), "test fixture must actually be non-similarity");
        let transformed = l.transformed(&map);
        for i in 0..=5 {
            let t = i as f64 - 2.0;
            assert!(transformed.eval(t).distance(map.apply_point(l.eval(t))) < 1e-9, "line must stay exact under a non-similarity map at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_transformed_stays_circle_under_similarity_and_matches_mapped_eval() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, -1.0, 2.0), Vec3::new(0.3, 0.2, 1.0)).unwrap();
        let c = Curve3::Circle { frame, radius: 2.5 };
        let map = Affine3::rotation_about(Pnt3::new(0.5, 0.0, 0.0), Vec3::new(0.1, 1.0, 0.2), 0.7).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 3.0)));
        let (_, scale, _) = map.is_similarity().expect("rotation + uniform scale must be a similarity");
        let transformed = c.transformed(&map);
        assert!(matches!(transformed, Curve3::Circle { .. }), "a similarity must keep a circle a circle");
        if let Curve3::Circle { radius, .. } = transformed {
            assert!((radius - 2.5 * scale).abs() < 1e-9);
        }
        for i in 0..8 {
            let t = i as f64 * 0.8;
            assert!(transformed.eval(t).distance(map.apply_point(c.eval(t))) < 1e-7, "mismatch at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_transformed_under_reflection_still_matches_mapped_eval() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 4.0 };
        let map = Affine3::mirror(Pnt3::new(0.0, 0.0, 0.0), Vec3::X);
        let (_, _, is_reflection) = map.is_similarity().expect("a mirror must be a similarity");
        assert!(is_reflection);
        let transformed = c.transformed(&map);
        for i in 0..8 {
            let t = i as f64 * 0.8;
            assert!(transformed.eval(t).distance(map.apply_point(c.eval(t))) < 1e-9, "mismatch at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_transformed_via_nurbs_under_non_similarity_matches_mapped_eval() {
        let frame = Frame3::WORLD;
        let c = Curve3::Circle { frame, radius: 1.0 };
        let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0));
        assert!(map.is_similarity().is_none());
        let transformed = c.transformed(&map);
        assert!(matches!(transformed, Curve3::Nurbs { .. }), "non-similarity must force NURBS");
        for i in 0..=20 {
            let t = i as f64 / 20.0 * std::f64::consts::TAU;
            assert!(transformed.eval(t).distance(map.apply_point(c.eval(t))) < 1e-7, "mismatch at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn nurbs_transformed_matches_mapped_eval_and_keeps_weights() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 2.0, 3.0) };
        let base = l.to_nurbs((0.0, 1.0));
        let nurbs = Curve3::Nurbs { knots: base.knots, controls: base.controls, weights: vec![1.0, 2.0] };
        let map = Affine3::translation(Vec3::new(5.0, -1.0, 2.0));
        let transformed = nurbs.transformed(&map);
        if let (Curve3::Nurbs { weights: original_weights, .. }, Curve3::Nurbs { weights: transformed_weights, .. }) = (&nurbs, &transformed) {
            assert_eq!(original_weights, transformed_weights);
        } else {
            panic!("expected Nurbs variants");
        }
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!(transformed.eval(t).distance(map.apply_point(nurbs.eval(t))) < 1e-9, "mismatch at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn transformed_inverse_round_trips_a_circle() {
        let frame = Frame3::from_normal(Pnt3::new(2.0, 1.0, -1.0), Vec3::new(0.2, -0.4, 1.0)).unwrap();
        let c = Curve3::Circle { frame, radius: 3.3 };
        let map = Affine3::rotation_about(Pnt3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.1).compose(&Affine3::translation(Vec3::new(2.0, -3.0, 1.0)));
        let inverse = map.inverse().unwrap();
        let round_trip = c.transformed(&map).transformed(&inverse);
        for i in 0..6 {
            let t = i as f64 * 1.0;
            assert!(round_trip.eval(t).distance(c.eval(t)) < 1e-7, "round trip drifted at t={t}");
        }
    }
}
// #endregion 🔖️Tests

// #endregion 🔁️Transform
