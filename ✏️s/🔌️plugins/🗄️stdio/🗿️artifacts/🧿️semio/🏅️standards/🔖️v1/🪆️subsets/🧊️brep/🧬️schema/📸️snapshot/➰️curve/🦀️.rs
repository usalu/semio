//! 🌀️ Analytic and free-form 3D curves ([`Curve3`]) and their 2D parameter-space counterparts
//! ([`Curve2`], the pcurve representation). Every variant supports position/derivative evaluation
//! and an *exact* [`Curve3::to_nurbs`]/[`Curve2::to_nurbs`] conversion — the single representation
//! every downstream algorithm (intersection, tessellation, STEP export) can fall back to when it
//! doesn't have an analytic fast path for a particular curve kind.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/➰️curve` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, together with
//! its `🪢️bspline`/`🎢️bezier`/`✂️curve-ops` siblings as local children (per this file's own
//! pre-mounted-stub note — no 1:1 file mapping), mirroring the `⚙️engine` → `🟫️step`/`📦️mesh-io`
//! local-mount pattern from wave PEEL3.

// #region 🔖️Submodules

#[path = "🎢️bezier/🦀️.rs"]
pub mod bezier;
#[path = "🪢️bspline/🦀️.rs"]
pub mod bspline;
#[path = "✂️curve-ops/🦀️.rs"]
pub mod curve_ops;

// #endregion 🔖️Submodules

use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt2, Pnt3, Vec2, Vec3};
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
                curve_derivatives_rational(knots, &controls_h, t, order).into_iter().map(|v| Vec3::new(v[0], v[1], v[2])).collect()
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
    arc_to_nurbs_with_span(frame, radius_x, radius_y, domain, std::f64::consts::TAU / 3.0)
}

/// 🌀️ The exact conic-arc-to-NURBS construction a rational quadratic Bezier span reproduces the
/// circle/ellipse's OWN shape exactly, but — since `cos`/`sin` are transcendental — its parameter
/// is a Möbius (tan-half-angle) reparametrization of angle, not angle itself: a span's error away
/// from the angle-linear point is `O(half_span³)` (peaks at `≈ 0.0321·radius·half_span³`, derived
/// via a series expansion of the closed-form rational Bezier around `half_span = 0`, verified
/// numerically), zero only at each span's two endpoints and exact midpoint. [`arc_to_nurbs`] caps
/// spans at a fixed 120° (adequate when only the SHAPE, not the parameter-to-angle correspondence,
/// matters — the common case); [`refined_max_span`] picks a caller-chosen tolerance instead, for
/// the rarer case (`Curve3::transformed`'s non-similarity fallback) that needs the NURBS to
/// reproduce `(radius·cos t, radius·sin t)` to a numeric tolerance at arbitrary `t`, not just as a
/// set of points.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arc_to_nurbs_with_span(frame: &Frame3, radius_x: f64, radius_y: f64, domain: (f64, f64), max_span: f64) -> NurbsCurve3 {
    let span = domain.1 - domain.0;
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
    /// ➰️ The same curve translated by `delta` in parameter space — the exact operation, per
    /// kind, not a refit. A p-curve on a periodic surface is only defined up to whole periods in
    /// the periodic directions, and the branch a p-curve was born in (whatever the surface
    /// inversion that produced it happened to return) need not be the branch the face's own
    /// boundary ring is written in; imprinting one into the other without re-aligning it produces a
    /// UV polygon whose pieces sit periods apart, which has no interior at all.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn translated(&self, delta: Vec2) -> Curve2 {
        match self {
            Curve2::Line { origin, dir } => Curve2::Line { origin: *origin + delta, dir: *dir },
            Curve2::Circle { center, radius } => Curve2::Circle { center: *center + delta, radius: *radius },
            Curve2::Ellipse { center, x_axis, major_radius, minor_radius } => Curve2::Ellipse { center: *center + delta, x_axis: *x_axis, major_radius: *major_radius, minor_radius: *minor_radius },
            Curve2::Nurbs { knots, controls, weights } => Curve2::Nurbs { knots: knots.clone(), controls: controls.iter().map(|p| *p + delta).collect(), weights: weights.clone() },
        }
    }
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔁️Transform

use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

impl Curve3 {
    /// 🌀️ Exact affine transform. `Line` stays exact under ANY invertible affine map — a line's
    /// image under any invertible linear map is again a line, with no orthonormality or domain
    /// constraint (unlike every other variant). `Circle`/`Ellipse` stay analytic under a similarity
    /// (uniform scale/rotation/translation, optionally with reflection — [`crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3::is_similarity`]),
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
    /// natural domain, then transform every control point (weights unchanged). `Circle`/`Ellipse`
    /// use [`refined_max_span`] (not the coarser 120°-span default) so `transformed.eval(t)` stays
    /// within `1e-9` of `map.apply_point(self.eval(t))` at every `t`, not just at span boundaries
    /// — required because [`Self::transformed`]'s own contract is a pointwise pushforward
    /// (`transformed.eval(t) == map(self.eval(t))`), not merely a shape-preserving conversion.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transformed_via_nurbs(&self, map: &Affine3) -> Curve3 {
        let nurbs = match self {
            Curve3::Circle { frame, radius } => arc_to_nurbs_with_span(frame, *radius, *radius, self.domain(), refined_max_span(*radius)),
            Curve3::Ellipse { frame, major_radius, minor_radius } => arc_to_nurbs_with_span(frame, *major_radius, *minor_radius, self.domain(), refined_max_span(major_radius.max(*minor_radius))),
            _ => self.to_nurbs(self.domain()),
        };
        Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls.into_iter().map(|p| map.apply_point(p)).collect(), weights: nurbs.weights }
    }
}

/// 🌀️ The largest per-span half-angle that keeps [`arc_to_nurbs_with_span`]'s parametrization
/// within `1e-9` of the true `radius·(cos t, sin t)` point at every `t` (not just span
/// boundaries), from the leading-order error bound `peak ≈ 0.0321·radius·half_span³` documented
/// on [`arc_to_nurbs_with_span`], solved for `half_span` and halved again for margin against the
/// series' next (`O(half_span⁵)`) term. Never coarser than the standard 120° span.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refined_max_span(radius: f64) -> f64 {
    let tol = 1e-9;
    let r = radius.abs().max(1e-9);
    let half_span = (tol / (0.0321 * r)).cbrt() * 0.5;
    (2.0 * half_span).min(std::f64::consts::TAU / 3.0)
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️transform/🦀️.rs"]
mod transform_tests;
// #endregion 🔖️Tests

// #endregion 🔁️Transform

// #region 🎯️Pcurve

use crate::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;

/// 🎯️ Fits a p-curve for `curve` on `surface` restricted to `domain` — the [`Surface::project_curve`]
/// worker, split out so the seam-splitting entry point ([`Surface::project_curve_pieces`]) can call
/// it once per seam-free sub-domain.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fit_pcurve(surface: &Surface, curve: &Curve3, domain: (f64, f64), tol: f64) -> Curve2 {
    use curve_ops::{interpolate_curve, parameterize, ParamMethod};
    let surf_domain = surface.domain();
    let mut n_samples = 8usize;
    loop {
        let ts: Vec<f64> = (0..=n_samples).map(|i| domain.0 + (domain.1 - domain.0) * i as f64 / n_samples as f64).collect();
        let mut uv_pts: Vec<Pnt3> = Vec::with_capacity(ts.len());
        let mut prev: Option<(f64, f64)> = None;
        for &t in &ts {
            let target = curve.eval(t);
            let closest = closest_uv(surface, surf_domain, target, tol.min(1e-9));
            let (mut u, mut v) = (closest.u, closest.v);
            if let Some((pu, pv)) = prev {
                if surface.is_u_periodic() {
                    u = unwrap_near(u, pu, std::f64::consts::TAU);
                }
                if surface.is_v_periodic() {
                    v = unwrap_near(v, pv, std::f64::consts::TAU);
                }
            }
            prev = Some((u, v));
            uv_pts.push(Pnt3::new(u, v, 0.0));
        }
        let params = parameterize(&uv_pts, ParamMethod::Centripetal);
        if let Some(fitted3) = interpolate_curve(&uv_pts, 3, ParamMethod::Centripetal, None, false) {
            let pcurve = nurbs3_to_curve2(&fitted3);
            // 🐛 A SINGLE midpoint per inter-sample interval under-resolves the actual worst-case
            // deviation: it can converge (report `max_dev <= tol`) while a DIFFERENT point inside
            // the same interval — not the exact midpoint — is still off by 10-40× `tol` (confirmed:
            // this loop used to report a converged `6e-5` at `n_samples=128` while a point at
            // `s=0.1` was independently measured `0.0024` from the true curve). 4 evenly-spaced
            // interior probes per interval catch that without assuming any particular worst-point
            // location.
            let mut max_dev = 0.0f64;
            for (i, w) in ts.windows(2).enumerate() {
                for k in 1..=4 {
                    let frac = k as f64 / 5.0;
                    let t_probe = w[0] + (w[1] - w[0]) * frac;
                    let real = curve.eval(t_probe);
                    let s_probe = params[i] + (params[i + 1] - params[i]) * frac;
                    let uv = pcurve.eval(s_probe);
                    let approx = surface.eval(uv.x, uv.y);
                    max_dev = max_dev.max(real.distance(approx));
                }
            }
            if max_dev <= tol || n_samples >= 1024 {
                return pcurve;
            }
        }
        n_samples *= 2;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_near(x: f64, near: f64, period: f64) -> f64 {
    let mut y = x;
    while y - near > period * 0.5 {
        y -= period;
    }
    while near - y > period * 0.5 {
        y += period;
    }
    y
}

/// 🎯️ Drops the (always-zero) z-coordinate a `(u, v, 0)`-embedded [`NurbsCurve3`] fit carries,
/// producing the equivalent [`Curve2`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nurbs3_to_curve2(curve: &NurbsCurve3) -> Curve2 {
    let controls = curve.controls.iter().map(|p| Pnt2::new(p.x, p.y)).collect();
    Curve2::Nurbs { knots: curve.knots.clone(), controls, weights: curve.weights.clone() }
}

/// 🎯️ Detects whether `fit_pcurve`'s naive single-shot fit crossed a periodic seam in a way that
/// needs an explicit domain split (the unwrapped raw samples jumped by close to a full period
/// between two adjacent low-density samples, rather than smoothly tracking the surface) and, if
/// so, bisects `domain` at the crossing and fits each seam-free side independently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn seam_crossings(surface: &Surface, curve: &Curve3, domain: (f64, f64)) -> Vec<f64> {
    let probes = 16usize;
    let ts: Vec<f64> = (0..=probes).map(|i| domain.0 + (domain.1 - domain.0) * i as f64 / probes as f64).collect();
    let mut raw: Vec<(f64, f64)> = Vec::with_capacity(ts.len());
    for &t in &ts {
        let closest = closest_uv(surface, surface.domain(), curve.eval(t), 1e-6);
        raw.push((closest.u, closest.v));
    }
    let mut crossings = Vec::new();
    let period = std::f64::consts::TAU;
    for i in 0..probes {
        let (u0, v0) = raw[i];
        let (u1, v1) = raw[i + 1];
        let jumps = (surface.is_u_periodic() && (u1 - u0).abs() > period * 0.4) || (surface.is_v_periodic() && (v1 - v0).abs() > period * 0.4);
        if jumps {
            let mut lo = ts[i];
            let mut hi = ts[i + 1];
            for _ in 0..30 {
                let mid = 0.5 * (lo + hi);
                let a = closest_uv(surface, surface.domain(), curve.eval(lo), 1e-6);
                let m = closest_uv(surface, surface.domain(), curve.eval(mid), 1e-6);
                let far_from_a = (surface.is_u_periodic() && (m.u - a.u).abs() > period * 0.4) || (surface.is_v_periodic() && (m.v - a.v).abs() > period * 0.4);
                if far_from_a {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            crossings.push(0.5 * (lo + hi));
        }
    }
    crossings
}

impl Surface {
    /// 🎯️ Evaluates a p-curve at its own parameter `t`, mapping through `surface.eval` — the
    /// trivial half of the p-curve contract; [`Surface::project_curve`] is the fitting half.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval_pcurve(&self, pcurve: &Curve2, t: f64) -> Pnt3 {
        let uv = pcurve.eval(t);
        self.eval(uv.x, uv.y)
    }

    /// 🎯️ Fits a `Curve2` p-curve approximating `curve` (restricted to `domain`) on `self`, to
    /// within `tol` 3D deviation. Analytic shortcuts for a [`Curve3::Line`]/[`Curve3::Circle`] on
    /// a [`Surface::Plane`] whose in-plane axes already match the curve's own frame exactly (the
    /// common case for edges constructed directly in a face's frame); otherwise samples by
    /// (adaptively densified) subdivision, projects each sample through the certified
    /// [`crate::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_uv`], unwraps periodic directions for continuity, and interpolates
    /// via [`curve_ops::interpolate_curve`], refining the sample density until the actual 3D
    /// deviation (checked at inter-sample midpoints, not just at the fitted points themselves) is
    /// within `tol`. Delegates to [`Surface::project_curve_pieces`] when the curve crosses a
    /// periodic seam, returning only its first piece — callers that need every piece (e.g. a
    /// seam-crossing trim edge that must become several coedges) should call
    /// [`Surface::project_curve_pieces`] directly.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn project_curve(&self, curve: &Curve3, domain: (f64, f64), tol: f64) -> Curve2 {
        if let Some(shortcut) = analytic_pcurve_shortcut(self, curve) {
            return shortcut;
        }
        self.project_curve_pieces(curve, domain, tol).into_iter().next().unwrap_or(Curve2::Line { origin: Pnt2::new(0.0, 0.0), dir: Vec2::new(1.0, 0.0) })
    }

    /// 🎯️ [`Surface::project_curve`], but split at every periodic-seam crossing so each returned
    /// piece stays within one seam-free stretch of `self`'s parameter domain.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn project_curve_pieces(&self, curve: &Curve3, domain: (f64, f64), tol: f64) -> Vec<Curve2> {
        if !self.is_u_periodic() && !self.is_v_periodic() {
            return vec![fit_pcurve(self, curve, domain, tol)];
        }
        let mut breaks = seam_crossings(self, curve, domain);
        breaks.retain(|&b| b > domain.0 + 1e-9 && b < domain.1 - 1e-9);
        breaks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut bounds = vec![domain.0];
        bounds.extend(breaks);
        bounds.push(domain.1);
        bounds.windows(2).map(|w| fit_pcurve(self, curve, (w[0], w[1]), tol)).collect()
    }
}

/// 🎯️ Exact p-curve for the narrow (but common) case a curve's own frame is already aligned with
/// the plane's frame — a line always projects to a 2D line exactly; a circle/ellipse whose frame
/// shares the plane's `x`/`y`/`z` axes (not merely coplanar — [`crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve2::Circle`] has no
/// independent rotation of its own) projects to the matching 2D conic exactly, parameter for
/// parameter. Any other in-plane rotation, or any non-planar surface, falls through to the
/// general numeric fit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn analytic_pcurve_shortcut(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    let Surface::Plane { frame } = surface else { return None };
    match curve {
        Curve3::Line { origin, dir } => {
            let lo = frame.to_local(*origin);
            let dv = frame.to_local_vector(*dir);
            Some(Curve2::Line { origin: Pnt2::new(lo.x, lo.y), dir: Vec2::new(dv.x, dv.y) })
        }
        Curve3::Circle { frame: cf, radius } if frames_aligned(frame, cf) => {
            let lo = frame.to_local(cf.origin);
            Some(Curve2::Circle { center: Pnt2::new(lo.x, lo.y), radius: *radius })
        }
        Curve3::Ellipse { frame: cf, major_radius, minor_radius } if frames_aligned(frame, cf) => {
            let lo = frame.to_local(cf.origin);
            Some(Curve2::Ellipse { center: Pnt2::new(lo.x, lo.y), x_axis: Vec2::new(1.0, 0.0), major_radius: *major_radius, minor_radius: *minor_radius })
        }
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frames_aligned(a: &Frame3, b: &Frame3) -> bool {
    (a.x - b.x).norm() < 1e-9 && (a.y - b.y).norm() < 1e-9 && (a.z - b.z).norm() < 1e-9 && (a.to_local(b.origin).z).abs() < 1e-9
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️pcurve/🦀️.rs"]
mod pcurve_tests;
// #endregion 🔖️Tests

// #endregion 🎯️Pcurve
