//! 📏️ Curve algorithms that operate *on* a [`super::Curve3`] rather than being part of its
//! definition: arc length, closest-point projection, and the split/reverse/join operations edges
//! need when Euler operators cut a curve. Kept separate from `curve.rs` so that file stays a pure
//! evaluation interface and this one can grow numerically heavier machinery independently.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/✂️curve-ops` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➰️curve` per that file's own pre-mounted-stub note.

use super::bezier::RationalBezier3;
use super::bspline::{basis_functions, de_boor, elevate_bezier_span_multi, elevate_degree, insert_knot, insert_knot_multi, KnotVector};
use super::{Curve3, NurbsCurve3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::polynomial::{isolate_roots, refine_root, Bernstein, Poly};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Length

/// 📏️ 5-point Gauss-Legendre nodes/weights on `[-1, 1]`.
const GL5_NODES: [f64; 5] = [-0.906_179_845_938_664, -0.538_469_310_105_683_1, 0.0, 0.538_469_310_105_683_1, 0.906_179_845_938_664];
const GL5_WEIGHTS: [f64; 5] = [0.2369268850561891, 0.4786286704993665, 0.5688888888888889, 0.4786286704993665, 0.2369268850561891];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gauss_legendre5(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    GL5_NODES.iter().zip(GL5_WEIGHTS.iter()).map(|(&x, &w)| w * f(mid + half * x)).sum::<f64>() * half
}

/// 📏️ Adaptive-quadrature arc length of `curve` over `[t0, t1]`: recursively halves the interval
/// until the 5-point Gauss-Legendre estimate agrees with the sum of its two half-interval
/// estimates to within `tol` (Richardson-style error control), or `max_depth` is reached.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn arc_length(curve: &Curve3, t0: f64, t1: f64, tol: f64) -> f64 {
    arc_length_recursive(curve, t0, t1, tol, 24)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arc_length_recursive(curve: &Curve3, t0: f64, t1: f64, tol: f64, depth: u32) -> f64 {
    let speed = |t: f64| curve.d1(t).norm();
    let whole = gauss_legendre5(speed, t0, t1);
    if depth == 0 {
        return whole;
    }
    let mid = 0.5 * (t0 + t1);
    let left = gauss_legendre5(speed, t0, mid);
    let right = gauss_legendre5(speed, mid, t1);
    if (whole - (left + right)).abs() < tol {
        left + right
    } else {
        arc_length_recursive(curve, t0, mid, tol * 0.5, depth - 1) + arc_length_recursive(curve, mid, t1, tol * 0.5, depth - 1)
    }
}

/// 📏️ Finds the parameter `t ∈ [t0, t1]` at which the arc length from `t0` equals `target_length`,
/// via bisection on the (monotonic, since speed ≥ 0) length function.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn param_at_length(curve: &Curve3, t0: f64, t1: f64, target_length: f64, tol: f64) -> f64 {
    let total = arc_length(curve, t0, t1, tol);
    if target_length <= 0.0 {
        return t0;
    }
    if target_length >= total {
        return t1;
    }
    let mut lo = t0;
    let mut hi = t1;
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        let len = arc_length(curve, t0, mid, tol);
        if (len - target_length).abs() < tol {
            return mid;
        }
        if len < target_length {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

// #endregion 🔖️Length

// #region 🔖️Project

/// 📏️ Certified closest-parameter result: the parameter, the point it evaluates to, the distance
/// to the query point, and whether the result is a proven global optimum (`true` for every
/// analytic kind, and for NURBS whose Bézier-span convex hulls were fully pruned and refined).
#[derive(Clone, Debug, PartialEq)]
pub struct ClosestParam {
    pub t: f64,
    pub point: Pnt3,
    pub distance: f64,
    pub certified: bool,
}

/// 📏️ Closest point on `curve`, restricted to `domain`, to `target`, refined to within `tol`.
/// Analytic closed forms for [`Curve3::Line`]/[`Curve3::Circle`]/[`Curve3::Ellipse`] (the ellipse
/// case via the tan-half-angle quartic — see [`ellipse_critical_thetas`]); [`Curve3::Nurbs`] via
/// Bézier-span subdivision with convex-hull pruning to seed every local minimum, each refined by
/// Newton with domain clamp/wrap. Replaces the former uniform-sampling seed entirely.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn closest_parameter(curve: &Curve3, domain: (f64, f64), target: Pnt3, tol: f64) -> ClosestParam {
    match curve {
        Curve3::Line { origin, dir } => closest_on_line(*origin, *dir, domain, target),
        Curve3::Circle { frame, radius } => closest_on_circle(frame, *radius, domain, target),
        Curve3::Ellipse { frame, major_radius, minor_radius } => closest_on_ellipse(frame, *major_radius, *minor_radius, domain, target),
        Curve3::Nurbs { .. } => closest_on_nurbs(curve, domain, target, tol),
    }
}

/// 📏️ Every certified local minimum of distance-to-`target` on `curve` over `domain` — as opposed
/// to [`closest_parameter`]'s single global minimum — e.g. every branch of an S-shaped NURBS curve
/// that comes locally closest to `target`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn all_closest_parameters(curve: &Curve3, domain: (f64, f64), target: Pnt3, tol: f64) -> Vec<ClosestParam> {
    match curve {
        Curve3::Line { .. } => vec![closest_parameter(curve, domain, target, tol)],
        Curve3::Circle { frame, .. } => all_local_minima_periodic(&circle_critical_thetas(frame, target), domain, curve, target),
        Curve3::Ellipse { frame, major_radius, minor_radius } => all_local_minima_periodic(&ellipse_critical_thetas(frame, *major_radius, *minor_radius, target), domain, curve, target),
        Curve3::Nurbs { .. } => all_local_minima_nurbs(curve, domain, target, tol),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_on_line(origin: Pnt3, dir: Vec3, domain: (f64, f64), target: Pnt3) -> ClosestParam {
    let dd = dir.dot(dir);
    let raw = if dd > 0.0 { (target - origin).dot(dir) / dd } else { domain.0 };
    let t = raw.clamp(domain.0, domain.1);
    let point = origin + dir * t;
    ClosestParam { t, point, distance: point.distance(target), certified: true }
}

/// 📏️ Every representative of `critical mod period` that lands within `domain`, plus `domain`'s
/// own endpoints — the constrained optimum can sit at a trim boundary when the unconstrained
/// critical point falls outside a sub-arc/sub-domain.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn periodic_candidates(criticals: &[f64], domain: (f64, f64), period: f64) -> Vec<f64> {
    let mut out = vec![domain.0, domain.1];
    for &c in criticals {
        let base = domain.0 + (c - domain.0).rem_euclid(period);
        let mut t = base;
        while t <= domain.1 + 1e-9 {
            out.push(t);
            t += period;
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn best_candidate(curve: &Curve3, candidates: &[f64], target: Pnt3) -> ClosestParam {
    let mut best_t = candidates[0];
    let mut best_p = curve.eval(best_t);
    let mut best_d = best_p.distance(target);
    for &t in &candidates[1..] {
        let p = curve.eval(t);
        let d = p.distance(target);
        if d < best_d {
            best_d = d;
            best_p = p;
            best_t = t;
        }
    }
    ClosestParam { t: best_t, point: best_p, distance: best_d, certified: true }
}

/// 📏️ `true` when `t` is a local *minimum* (not maximum) of distance-to-`target`, via the sign of
/// the projected second derivative `f''(t) = C'(t)·C'(t) + (C(t)-P)·C''(t)` at a point already
/// known to satisfy the first-order condition `f'(t) = 0`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_local_minimum(curve: &Curve3, t: f64, target: Pnt3) -> bool {
    let d1 = curve.d1(t);
    let d2 = curve.d2(t);
    let delta = curve.eval(t) - target;
    d1.dot(d1) + delta.dot(d2) > 0.0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn circle_critical_thetas(frame: &Frame3, target: Pnt3) -> Vec<f64> {
    let local = frame.to_local(target);
    if local.x.hypot(local.y) <= f64::EPSILON {
        return vec![0.0];
    }
    let near = local.y.atan2(local.x);
    vec![near, near + std::f64::consts::PI]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_on_circle(frame: &Frame3, radius: f64, domain: (f64, f64), target: Pnt3) -> ClosestParam {
    let curve = Curve3::Circle { frame: *frame, radius };
    let candidates = periodic_candidates(&circle_critical_thetas(frame, target), domain, std::f64::consts::TAU);
    best_candidate(&curve, &candidates, target)
}

/// 📏️ Substitutes `t = lo + (hi-lo)·s`, i.e. reparametrizes the monomial polynomial `coeffs`
/// (in `t`) onto `s ∈ [0, 1]` — a linear (degree-preserving) change of variable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn substitute_affine(coeffs: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    let scale = hi - lo;
    let n = coeffs.len();
    let mut out = vec![0.0; n];
    for (k, &ck) in coeffs.iter().enumerate() {
        for j in 0..=k {
            out[j] += ck * binomial_usize(k, j) * lo.powi((k - j) as i32) * scale.powi(j as i32);
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binomial_usize(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

/// 📏️ Every θ at which `d/dθ |ellipse(θ) - target|² = 0`, found via the tan-half-angle
/// substitution `t = tan((θ-c)/2)` (turns the trig optimality condition into a quartic in `t`),
/// evaluated over three overlapping 240°-wide windows (centers `0, 2π/3, 4π/3`) so every θ falls
/// safely inside at least one window's finite `t`-range — the substitution's only blind spot is
/// the single point `θ = c + π`, which the other two windows always cover. Each window's quartic
/// is isolated and refined on its Bernstein form via [`isolate_roots`]/[`refine_root`] — a
/// certified root enclosure, not a sampling seed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ellipse_critical_thetas(frame: &Frame3, a: f64, b: f64, target: Pnt3) -> Vec<f64> {
    let local = frame.to_local(target);
    let (x0, y0) = (local.x, local.y);
    let mut thetas = Vec::new();
    for k in 0..3 {
        let c = std::f64::consts::TAU * k as f64 / 3.0;
        let (alpha, beta) = (c.cos(), c.sin());
        let k1 = (b * b - a * a) * alpha * beta;
        let k2 = (b * b - a * a) * (alpha * alpha - beta * beta);
        let k3 = a * x0 * beta - b * y0 * alpha;
        let k4 = a * x0 * alpha + b * y0 * beta;
        let coeffs = [k1 + k3, 2.0 * (k2 + k4), -6.0 * k1, 2.0 * (k4 - k2), k1 - k3];
        let half = std::f64::consts::PI / 3.0; // window half-width in phi; tan(half) stays finite
        let bound = half.tan();
        let poly = Poly::new(substitute_affine(&coeffs, -bound, bound));
        if poly.degree() == 0 {
            continue;
        }
        let bernstein = Bernstein::from_monomial(&poly);
        for (lo, hi) in isolate_roots(&bernstein, 40) {
            let s = refine_root(&poly, lo, hi, 1e-13, 80);
            let t = -bound + 2.0 * bound * s;
            let phi = 2.0 * t.atan();
            thetas.push(c + phi);
        }
    }
    thetas
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_on_ellipse(frame: &Frame3, major_radius: f64, minor_radius: f64, domain: (f64, f64), target: Pnt3) -> ClosestParam {
    let curve = Curve3::Ellipse { frame: *frame, major_radius, minor_radius };
    let criticals = ellipse_critical_thetas(frame, major_radius, minor_radius, target);
    let candidates = periodic_candidates(&criticals, domain, std::f64::consts::TAU);
    best_candidate(&curve, &candidates, target)
}

/// 📏️ Every certified local minimum among `criticals` (mod `curve`'s period) that lands within
/// `domain`, plus domain endpoints — filters out the local *maxima* [`periodic_candidates`] also
/// produces (e.g. a circle's antipodal "farthest point").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn all_local_minima_periodic(criticals: &[f64], domain: (f64, f64), curve: &Curve3, target: Pnt3) -> Vec<ClosestParam> {
    let period = curve.period().unwrap_or(std::f64::consts::TAU);
    // 🐛 A domain spanning exactly one full period (e.g. a whole, untrimmed circle's own
    // `(0, TAU)`) has NO real trim boundary — `domain.0`/`domain.1` are the SAME physical point,
    // not two independent constrained-optimum candidates, and (unlike a genuine critical point)
    // neither satisfies the first-order condition `is_local_minimum` assumes (its own docstring).
    // Drop both sentinels unconditionally in that case rather than subjecting them to a test only
    // meaningful at a real stationary point — every genuine minimum on a full period is already
    // found via `is_local_minimum` on the wrapped `criticals`. A genuinely trimmed sub-arc keeps
    // both endpoints unconditionally (the constrained optimum can legitimately sit there even when
    // it isn't an unconstrained critical point).
    let full_period = (domain.1 - domain.0 - period).abs() < 1e-9;
    let mut candidates = periodic_candidates(criticals, domain, period);
    candidates.retain(|&t| {
        let is_endpoint = t == domain.0 || t == domain.1;
        if full_period && is_endpoint {
            return false;
        }
        (!full_period && is_endpoint) || is_local_minimum(curve, t, target)
    });
    candidates.sort_by(|a, b| a.partial_cmp(b).unwrap());
    candidates.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    candidates
        .iter()
        .map(|&t| {
            let p = curve.eval(t);
            ClosestParam { t, point: p, distance: p.distance(target), certified: true }
        })
        .collect()
}

/// 📏️ Decomposes a NURBS curve into its Bézier spans (one per knot interval), via repeated
/// [`split_nurbs`] — each interior knot is used once as a split point, so every emitted piece is
/// exactly [`split_nurbs`]'s already-tested output, never new knot-insertion machinery.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bezier_spans(curve: &NurbsCurve3) -> Vec<(f64, f64, RationalBezier3)> {
    let (lo, hi) = curve.knots.domain();
    let mut interior: Vec<f64> = curve.knots.knots.iter().copied().filter(|&k| k > lo + 1e-12 && k < hi - 1e-12).collect();
    interior.sort_by(|a, b| a.partial_cmp(b).unwrap());
    interior.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    let mut spans = Vec::new();
    let mut remaining = curve.clone();
    let mut start = lo;
    for &k in &interior {
        let (left, right) = split_nurbs(&remaining, k);
        spans.push((start, k, RationalBezier3::new(left.controls, left.weights)));
        remaining = right;
        start = k;
    }
    spans.push((start, hi, RationalBezier3::new(remaining.controls, remaining.weights)));
    spans
}

/// 📏️ Exact (certified) distance from `target` to the axis-aligned box `[lo, hi]` — `0` when
/// `target` is inside, otherwise the distance to the nearest clamped point on its boundary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_to_box_distance(target: Pnt3, lo: Pnt3, hi: Pnt3) -> f64 {
    let clamped = Pnt3::new(target.x.clamp(lo.x, hi.x), target.y.clamp(lo.y, hi.y), target.z.clamp(lo.z, hi.z));
    clamped.distance(target)
}

/// 📏️ Newton seeds within one Bézier span `[t0, t1]` — a single (midpoint) seed only ever finds
/// ONE basin of `|C(t)-target|`'s distance landscape, but a degree-`p` polynomial/rational span's
/// distance-squared derivative has degree up to `2p - 1`, so up to `p` distinct local minima can
/// coexist in a SINGLE span (the S-shaped-curve test is exactly this: one cubic Bézier span, two
/// minima). 5 evenly-spaced interior seeds reliably separate up to 3 basins (ample for the cubic —
/// degree 3 — case; higher-degree spans get correspondingly denser coverage since the seed count
/// is independent of degree, trading a constant per-span factor for correctness).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn span_seeds(t0: f64, t1: f64) -> [f64; 5] {
    let step = (t1 - t0) / 6.0;
    [t0 + step, t0 + 2.0 * step, t0 + 3.0 * step, t0 + 4.0 * step, t0 + 5.0 * step]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_on_nurbs(curve: &Curve3, domain: (f64, f64), target: Pnt3, tol: f64) -> ClosestParam {
    let Curve3::Nurbs { knots, controls, weights } = curve else {
        unreachable!("closest_on_nurbs called on a non-NURBS curve")
    };
    let nurbs = NurbsCurve3 { knots: knots.clone(), controls: controls.clone(), weights: weights.clone() };
    let mut spans: Vec<(f64, f64, f64)> = bezier_spans(&nurbs)
        .into_iter()
        .filter(|(t0, t1, _)| *t1 > domain.0 - 1e-12 && *t0 < domain.1 + 1e-12)
        .map(|(t0, t1, b)| {
            let (lo, hi) = b.control_hull_box();
            (t0, t1, point_to_box_distance(target, lo, hi))
        })
        .collect();
    spans.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut best = best_candidate(curve, &[domain.0, domain.1], target);
    for (t0, t1, lower_bound) in spans {
        if lower_bound > best.distance + tol {
            continue;
        }
        let (lo, hi) = (t0.max(domain.0), t1.min(domain.1));
        for seed in span_seeds(lo, hi) {
            let refined = newton_closest_point(curve, target, seed, domain, None);
            let point = curve.eval(refined);
            let distance = point.distance(target);
            if distance < best.distance {
                best = ClosestParam { t: refined, point, distance, certified: true };
            }
        }
    }
    best
}

/// 📏️ Every span's own local Newton refinement (from [`span_seeds`], not a single midpoint —
/// see its docstring), deduplicated and filtered down to genuine local minima — the
/// multi-minimum counterpart of [`closest_on_nurbs`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn all_local_minima_nurbs(curve: &Curve3, domain: (f64, f64), target: Pnt3, tol: f64) -> Vec<ClosestParam> {
    let Curve3::Nurbs { knots, controls, weights } = curve else {
        unreachable!("all_local_minima_nurbs called on a non-NURBS curve")
    };
    let nurbs = NurbsCurve3 { knots: knots.clone(), controls: controls.clone(), weights: weights.clone() };
    let mut results: Vec<ClosestParam> = Vec::new();
    for (t0, t1, _) in bezier_spans(&nurbs) {
        if t1 <= domain.0 || t0 >= domain.1 {
            continue;
        }
        let (lo, hi) = (t0.max(domain.0), t1.min(domain.1));
        for seed in span_seeds(lo, hi) {
            let refined = newton_closest_point(curve, target, seed, domain, None);
            if !is_local_minimum(curve, refined, target) {
                continue;
            }
            if results.iter().any(|r: &ClosestParam| (r.t - refined).abs() < tol.max(1e-9)) {
                continue;
            }
            let point = curve.eval(refined);
            results.push(ClosestParam { t: refined, point, distance: point.distance(target), certified: true });
        }
    }
    results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    results
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_into_domain(t: f64, domain: (f64, f64), period: Option<f64>) -> f64 {
    match period {
        Some(p) => {
            let mut x = (t - domain.0) % p;
            if x < 0.0 {
                x += p;
            }
            domain.0 + x
        }
        None => t.clamp(domain.0, domain.1),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newton_closest_point(curve: &Curve3, target: Pnt3, mut t: f64, domain: (f64, f64), period: Option<f64>) -> f64 {
    for _ in 0..30 {
        let c = curve.eval(t);
        let d1 = curve.d1(t);
        let d2 = curve.d2(t);
        let delta = c - target;
        let f = delta.dot(d1);
        let fp = d1.dot(d1) + delta.dot(d2);
        if fp.abs() <= 1e-300 {
            break;
        }
        let step = f / fp;
        let next = wrap_into_domain(t - step, domain, period);
        if (next - t).abs() < 1e-13 {
            t = next;
            break;
        }
        t = next;
    }
    t
}

// #endregion 🔖️Project

// #region 🔖️Fit

/// 📏️ Which parameter values [`interpolate_curve`]/[`interpolate_surface_grid`] assign to the
/// input points before fitting a knot vector to them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamMethod {
    /// 📏️ `t_i = i / (n-1)` — ignores point spacing entirely; simple and robust.
    Uniform,
    /// 📏️ `t_i ∝` cumulative chord length — the classical choice, can cusp on uneven spacing.
    Chord,
    /// 📏️ `t_i ∝` cumulative `√(chord length)` (Lee's method) — the standard, well-conditioned
    /// choice for scattered points; avoids the cusping chord-length parametrization can produce.
    Centripetal,
}

/// 📏️ Parameter values in `[0, 1]` for `points` under `method` (shared by curve and surface
/// interpolation, and by [`approximate_curve`]'s centripetal default).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parameterize(points: &[Pnt3], method: ParamMethod) -> Vec<f64> {
    let n = points.len();
    let mut params = vec![0.0; n];
    if n < 2 {
        return params;
    }
    let mut steps = vec![0.0; n];
    for i in 1..n {
        let d = points[i].distance(points[i - 1]);
        steps[i] = match method {
            ParamMethod::Uniform => 1.0,
            ParamMethod::Chord => d,
            ParamMethod::Centripetal => d.sqrt(),
        };
    }
    let total: f64 = steps.iter().sum();
    if total <= 0.0 {
        for (i, p) in params.iter_mut().enumerate() {
            *p = i as f64 / (n - 1) as f64;
        }
        return params;
    }
    let mut acc = 0.0;
    for i in 1..n {
        acc += steps[i];
        params[i] = acc / total;
    }
    params
}

/// 📏️ The standard knot-averaging technique (NURBS Book eq. 9.8): places each interior knot at
/// the average of `degree` consecutive parameter values, so every knot span contains at least one
/// data parameter — the well-conditioned choice for a knot vector matched 1:1 to `params.len()`
/// control points (as opposed to [`fitting_knot_vector`]'s fewer-controls-than-points variant).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn averaged_knot_vector(params: &[f64], degree: usize) -> Vec<f64> {
    let n = params.len();
    let mut knots = vec![0.0; degree + 1];
    for j in 1..n.saturating_sub(degree) {
        let avg: f64 = params[j..j + degree].iter().sum::<f64>() / degree as f64;
        knots.push(avg);
    }
    knots.extend(std::iter::repeat_n(*params.last().unwrap_or(&1.0), degree + 1));
    knots
}

/// 📏️ Plain Gaussian elimination with partial pivoting — the interpolation matrix is small
/// (control-point count) and banded but not worth a dedicated banded solver at this scale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solve_linear_system(a: &[Vec<f64>], rhs: &[f64]) -> Vec<f64> {
    let n = rhs.len();
    let mut m: Vec<Vec<f64>> = a.to_vec();
    let mut b = rhs.to_vec();
    for col in 0..n {
        let pivot = (col..n).max_by(|&i, &j| m[i][col].abs().partial_cmp(&m[j][col].abs()).unwrap()).unwrap();
        m.swap(col, pivot);
        b.swap(col, pivot);
        let diag = m[col][col];
        let pivot_row = m[col].clone();
        for row in col + 1..n {
            let factor = m[row][col] / diag;
            for (k, cell) in m[row].iter_mut().enumerate().skip(col) {
                *cell -= factor * pivot_row[k];
            }
            b[row] -= factor * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let sum: f64 = (row + 1..n).map(|k| m[row][k] * x[k]).sum();
        x[row] = (b[row] - sum) / m[row][row];
    }
    x
}

/// 📏️ Solves the (square) global-interpolation system for one set of points against a shared
/// `(knots, params)` basis — the common core of [`interpolate_curve`]'s open case and
/// [`interpolate_surface_grid`]'s two interpolation passes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solve_interpolation_1d(points: &[Pnt3], knots: &KnotVector, params: &[f64]) -> Vec<Pnt3> {
    let n = points.len();
    let degree = knots.degree;
    let mut a = vec![vec![0.0; n]; n];
    for (row, &u) in params.iter().enumerate() {
        let span = knots.find_span(u);
        let basis = basis_functions(knots, span, u);
        for (j, &b) in basis.iter().enumerate() {
            a[row][span - degree + j] = b;
        }
    }
    let solve_axis = |axis: fn(&Pnt3) -> f64| -> Vec<f64> {
        let rhs: Vec<f64> = points.iter().map(axis).collect();
        solve_linear_system(&a, &rhs)
    };
    let xs = solve_axis(|p| p.x);
    let ys = solve_axis(|p| p.y);
    let zs = solve_axis(|p| p.z);
    (0..n).map(|i| Pnt3::new(xs[i], ys[i], zs[i])).collect()
}

/// 📏️ Evaluates a [`NurbsCurve3`] via plain (non-rational-derivative) de Boor — a small local
/// helper for the fit/approximation routines below, which only ever need position, not derivatives.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn eval_nurbs_curve3(curve: &NurbsCurve3, t: f64) -> Pnt3 {
    let hx: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.x * w).collect();
    let hy: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.y * w).collect();
    let hz: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.z * w).collect();
    let w = de_boor(&curve.knots, &curve.weights, t);
    Pnt3::new(de_boor(&curve.knots, &hx, t) / w, de_boor(&curve.knots, &hy, t) / w, de_boor(&curve.knots, &hz, t) / w)
}

/// 📏️ Global curve interpolation (NURBS Book §9.2): solves the *exact* linear system so the
/// returned curve passes through every point in `points` at its own assigned parameter — no
/// downsampling, no approximation. `ends`, when given, additionally clamps the start/end tangent
/// directions via the standard clamped-B-spline boundary-derivative identity
/// `C'(0) = (p / U[p+1]) · (P_1 - P_0)` (and its mirror at the end), expressed as two extra rows in
/// the same linear system (two extra unknown control points) rather than eliminated algebraically
/// — simpler to verify, at the cost of using uniform (not parameter-averaged) interior knots in
/// that case. `closed`, when set, builds a periodic curve via [`KnotVector::periodic_uniform`]: the
/// system's columns wrap `mod points.len()` so the fitted control points close up with
/// `C^(degree-1)` continuity at the seam (see [`KnotVector::is_periodic`]); `ends` is ignored when
/// `closed` is set (a closed curve has no boundary to clamp).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn interpolate_curve(points: &[Pnt3], degree: usize, params_method: ParamMethod, ends: Option<(Vec3, Vec3)>, closed: bool) -> Option<NurbsCurve3> {
    let n = points.len();
    if n < 2 {
        return None;
    }
    let p = degree.max(1);
    if closed {
        return interpolate_curve_closed(points, p, params_method);
    }
    let params = parameterize(points, params_method);
    if let Some((d0, d1)) = ends {
        return interpolate_curve_with_tangents(points, p.max(1), &params, d0, d1);
    }
    let p = p.min(n - 1);
    let knots = KnotVector::new(averaged_knot_vector(&params, p), p, n)?;
    let controls = solve_interpolation_1d(points, &knots, &params);
    Some(NurbsCurve3 { knots, controls, weights: vec![1.0; n] })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn interpolate_curve_with_tangents(points: &[Pnt3], degree: usize, params: &[f64], d0: Vec3, d1: Vec3) -> Option<NurbsCurve3> {
    let n = points.len();
    let extra = n + 2;
    let kv = KnotVector::clamped_uniform(extra, degree);
    let (lo, hi) = kv.domain();
    let u_p1 = kv.knots[degree + 1] - lo;
    let n_idx = extra - 1;
    let u_before_last = kv.knots[n_idx - degree];
    let span_len = hi - u_before_last;
    let mut mat = vec![vec![0.0; extra]; extra];
    let mut rhs = vec![Pnt3::new(0.0, 0.0, 0.0); extra];
    for (row, &u) in params.iter().enumerate() {
        let span = kv.find_span(u);
        let basis = basis_functions(&kv, span, u);
        for (j, &b) in basis.iter().enumerate() {
            mat[row][span - degree + j] = b;
        }
        rhs[row] = points[row];
    }
    let tangent_row0 = n;
    let tangent_row1 = n + 1;
    mat[tangent_row0][0] = -1.0;
    mat[tangent_row0][1] = 1.0;
    rhs[tangent_row0] = Pnt3::new(d0.x * u_p1 / degree as f64, d0.y * u_p1 / degree as f64, d0.z * u_p1 / degree as f64);
    mat[tangent_row1][n_idx - 1] = -1.0;
    mat[tangent_row1][n_idx] = 1.0;
    rhs[tangent_row1] = Pnt3::new(d1.x * span_len / degree as f64, d1.y * span_len / degree as f64, d1.z * span_len / degree as f64);
    let solve_axis = |axis: fn(&Pnt3) -> f64| -> Vec<f64> {
        let r: Vec<f64> = rhs.iter().map(axis).collect();
        solve_linear_system(&mat, &r)
    };
    let xs = solve_axis(|p| p.x);
    let ys = solve_axis(|p| p.y);
    let zs = solve_axis(|p| p.z);
    let controls = (0..extra).map(|i| Pnt3::new(xs[i], ys[i], zs[i])).collect();
    Some(NurbsCurve3 { knots: kv, controls, weights: vec![1.0; extra] })
}

/// 📏️ Closed (periodic) global interpolation: builds an `n×n` system over
/// [`KnotVector::periodic_uniform`] with column indices wrapped `mod n`, so the fitted `n` real
/// control points close the loop with `C^(degree-1)` continuity at the seam without any special
/// evaluation path — the resulting curve stores the standard "phantom wrap" control array (see
/// [`KnotVector::periodic_uniform`]'s doc).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn interpolate_curve_closed(points: &[Pnt3], degree: usize, params_method: ParamMethod) -> Option<NurbsCurve3> {
    let n = points.len();
    let p = degree.min(n.saturating_sub(1)).max(1);
    if n <= p {
        return None;
    }
    let kv = KnotVector::periodic_uniform(n, p);
    let params: Vec<f64> = match params_method {
        ParamMethod::Uniform => (0..n).map(|i| i as f64).collect(),
        _ => {
            let mut acc = vec![0.0; n];
            let mut steps = vec![0.0; n];
            for i in 0..n {
                let prev = points[(i + n - 1) % n];
                let d = points[i].distance(prev);
                steps[i] = if matches!(params_method, ParamMethod::Centripetal) { d.sqrt() } else { d };
            }
            let total: f64 = steps.iter().sum();
            if total <= 0.0 {
                return None;
            }
            let mut running = 0.0;
            for i in 0..n {
                acc[i] = running / total * n as f64;
                running += steps[i];
            }
            acc
        }
    };
    let mut a = vec![vec![0.0; n]; n];
    for (row, &u) in params.iter().enumerate() {
        let span = kv.find_span(u);
        let basis = basis_functions(&kv, span, u);
        for (j, &b) in basis.iter().enumerate() {
            let col = (span + n - p + j) % n;
            a[row][col] += b;
        }
    }
    let solve_axis = |axis: fn(&Pnt3) -> f64| -> Vec<f64> {
        let rhs: Vec<f64> = points.iter().map(axis).collect();
        solve_linear_system(&a, &rhs)
    };
    let xs = solve_axis(|p| p.x);
    let ys = solve_axis(|p| p.y);
    let zs = solve_axis(|p| p.z);
    let real: Vec<Pnt3> = (0..n).map(|i| Pnt3::new(xs[i], ys[i], zs[i])).collect();
    let mut controls = real.clone();
    controls.extend(real.iter().take(p).copied());
    Some(NurbsCurve3 { knots: kv, controls, weights: vec![1.0; n + p] })
}

/// 📏️ Global cubic interpolation through `points` using centripetal parameterization (Lee's
/// method) — kept as a thin, backward-compatible wrapper over [`interpolate_curve`] (degree
/// `min(n-1, 3)`, open, no end tangents), since existing callers/tests target this exact signature.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn interpolate_centripetal(points: &[Pnt3]) -> Option<NurbsCurve3> {
    let degree = points.len().saturating_sub(1).min(3);
    interpolate_curve(points, degree, ParamMethod::Centripetal, None, false)
}

/// 📏️ The knot-placement technique for least-squares fitting with fewer control points than data
/// points (NURBS Book eq. 9.68/9.69): spreads the `n - p - 1` interior knots evenly across the
/// parameter range, each positioned by linear interpolation between the two `params` values its
/// slot falls between — unlike [`averaged_knot_vector`], which needs one knot per data point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fitting_knot_vector(params: &[f64], p: usize, num_controls: usize) -> Vec<f64> {
    let n = num_controls - 1;
    let m = params.len() - 1;
    let mut u = vec![0.0; n + p + 2];
    for i in 0..=p {
        u[i] = 0.0;
        u[n + 1 + i] = 1.0;
    }
    if n > p {
        // 🐛 The standard Piegl-Tiller averaging formula (NURBS Book Eq. 9.68) needs `n - p`
        // interior knots (indices `p+1..=n`), not `n - p - 1` — the previous off-by-one left the
        // LAST interior knot (index `n`) unset (default 0.0), which for the common `n_controls =
        // degree + 2` case (`n - p == 1`) makes the loop range `1..=0` — empty — so the sole
        // interior knot stayed at its zero-initialized value, colliding with the clamped-start
        // knots and pushing that knot's multiplicity past `degree + 1`. `KnotVector::new` then
        // correctly rejected it as invalid, which `approximate_curve_with_count`'s `?` silently
        // turned into `None`, panicking every caller that `.unwrap()`s the result.
        let d = (m + 1) as f64 / (n - p + 1) as f64;
        for j in 1..=(n - p) {
            let jd = j as f64 * d;
            let i = jd.floor() as usize;
            let alpha = jd - i as f64;
            let lo = params[i.saturating_sub(1)];
            let hi = params[i.min(m)];
            u[p + j] = (1.0 - alpha) * lo + alpha * hi;
        }
    }
    u
}

/// 📏️ Least-squares curve fit (Piegl-Tiller global approximation) to exactly `n_controls` control
/// points: the two endpoints are pinned to `points[0]`/`points.last()` exactly, the interior
/// control points solve the normal equations of the overdetermined interpolation system. Returns
/// the fitted curve and its achieved maximum pointwise deviation from `points`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn approximate_curve_with_count(points: &[Pnt3], degree: usize, n_controls: usize) -> Option<(NurbsCurve3, f64)> {
    if points.len() < 2 || n_controls < 2 || n_controls > points.len() {
        return None;
    }
    let m_idx = points.len() - 1;
    let p = degree.max(1).min(n_controls - 1);
    let params = parameterize(points, ParamMethod::Centripetal);
    let n = n_controls - 1;
    let kv = KnotVector::new(fitting_knot_vector(&params, p, n_controls), p, n_controls)?;
    let mut controls = vec![Pnt3::new(0.0, 0.0, 0.0); n_controls];
    controls[0] = points[0];
    controls[n] = points[m_idx];
    let free = n.saturating_sub(1);
    if free > 0 && m_idx > 1 {
        let mut ncoeff = vec![vec![0.0; free]; free];
        let mut rhs = vec![Vec3::ZERO; free];
        for k in 1..m_idx {
            let uk = params[k];
            let span = kv.find_span(uk);
            let basis = basis_functions(&kv, span, uk);
            let mut rk = points[k].to_vec();
            for idx in 0..=p {
                let gi = span - p + idx;
                if gi == 0 {
                    rk = rk - points[0].to_vec() * basis[idx];
                }
                if gi == n {
                    rk = rk - points[m_idx].to_vec() * basis[idx];
                }
            }
            for idx_i in 0..=p {
                let gi = span - p + idx_i;
                if gi == 0 || gi == n {
                    continue;
                }
                let row = gi - 1;
                rhs[row] = rhs[row] + rk * basis[idx_i];
                for idx_j in 0..=p {
                    let gj = span - p + idx_j;
                    if gj == 0 || gj == n {
                        continue;
                    }
                    ncoeff[row][gj - 1] += basis[idx_i] * basis[idx_j];
                }
            }
        }
        let solve_axis = |axis: fn(Vec3) -> f64| -> Vec<f64> {
            let r: Vec<f64> = rhs.iter().map(|&v| axis(v)).collect();
            solve_linear_system(&ncoeff, &r)
        };
        let xs = solve_axis(|v| v.x);
        let ys = solve_axis(|v| v.y);
        let zs = solve_axis(|v| v.z);
        for i in 0..free {
            controls[i + 1] = Pnt3::new(xs[i], ys[i], zs[i]);
        }
    }
    let curve = NurbsCurve3 { knots: kv, controls, weights: vec![1.0; n_controls] };
    let err = params.iter().zip(points).map(|(&t, &q)| eval_nurbs_curve3(&curve, t).distance(q)).fold(0.0, f64::max);
    Some((curve, err))
}

/// 📏️ Error-bounded least-squares approximation: grows the control-point count (starting from a
/// single Bézier span) via [`approximate_curve_with_count`] until the achieved maximum deviation is
/// `<= max_error` (or every data point is used, i.e. exact interpolation). Returns the fitted curve
/// and its achieved error — never silently downsamples.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn approximate_curve(points: &[Pnt3], degree: usize, max_error: f64) -> Option<(NurbsCurve3, f64)> {
    if points.len() < 2 {
        return None;
    }
    let p = degree.max(1);
    let mut n_controls = (p + 2).min(points.len()).max(2);
    loop {
        let attempt = approximate_curve_with_count(points, p, n_controls);
        match attempt {
            Some((curve, err)) if err <= max_error || n_controls >= points.len() => return Some((curve, err)),
            Some(_) => {
                let remaining = points.len() - n_controls;
                n_controls += (remaining / 2).max(1);
                n_controls = n_controls.min(points.len());
            }
            None => {
                n_controls += 1;
                if n_controls > points.len() {
                    return None;
                }
            }
        }
    }
}

// #endregion 🔖️Fit

// #region 🔖️SurfaceFit

/// 📏️ True global tensor-product surface interpolation (NURBS Book §9.5, separable construction):
/// `u`-parameters are the average, over every `v`-row's own centripetal parameterization, of each
/// `u`-column's chord position (and vice versa for `v`), a single shared `(u_knots, v_knots)` is
/// built from those averages, then interpolation runs first along every row (`v`-direction), then
/// along every column of the intermediate result (`u`-direction) — the standard two-pass reduction
/// to 1D global interpolation, exact (not sampled).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn interpolate_surface_grid(points: &[Vec<Pnt3>], degree_u: usize, degree_v: usize) -> Option<Surface> {
    let nu = points.len();
    if nu == 0 {
        return None;
    }
    let nv = points[0].len();
    if nv == 0 || points.iter().any(|row| row.len() != nv) {
        return None;
    }
    let pu = degree_u.max(1).min(nu - 1);
    let pv = degree_v.max(1).min(nv - 1);
    let mut u_acc = vec![0.0; nu];
    for j in 0..nv {
        let column: Vec<Pnt3> = (0..nu).map(|i| points[i][j]).collect();
        let p = parameterize(&column, ParamMethod::Centripetal);
        for i in 0..nu {
            u_acc[i] += p[i];
        }
    }
    let u_params: Vec<f64> = u_acc.iter().map(|&s| s / nv as f64).collect();
    let mut v_acc = vec![0.0; nv];
    for row in points.iter() {
        let p = parameterize(row, ParamMethod::Centripetal);
        for j in 0..nv {
            v_acc[j] += p[j];
        }
    }
    let v_params: Vec<f64> = v_acc.iter().map(|&s| s / nu as f64).collect();
    let u_knots = KnotVector::new(averaged_knot_vector(&u_params, pu), pu, nu)?;
    let v_knots = KnotVector::new(averaged_knot_vector(&v_params, pv), pv, nv)?;
    let mut temp: Vec<Vec<Pnt3>> = Vec::with_capacity(nu);
    for row in points.iter() {
        temp.push(solve_interpolation_1d(row, &v_knots, &v_params));
    }
    let mut controls = vec![vec![Pnt3::new(0.0, 0.0, 0.0); nv]; nu];
    for j in 0..nv {
        let column: Vec<Pnt3> = (0..nu).map(|i| temp[i][j]).collect();
        let col_controls = solve_interpolation_1d(&column, &u_knots, &u_params);
        for i in 0..nu {
            controls[i][j] = col_controls[i];
        }
    }
    let weights = vec![vec![1.0; nv]; nu];
    Some(Surface::Nurbs { u_knots, v_knots, controls, weights })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn to_homogeneous(curve: &NurbsCurve3) -> Vec<Vec<f64>> {
    curve.controls.iter().zip(&curve.weights).map(|(p, &w)| vec![p.x * w, p.y * w, p.z * w, w]).collect()
}

/// 📏️ Brings two boundary curves that must share a common tensor-product direction (`c0`/`c1` in
/// [`coons_patch_nurbs`]) onto one identical `(knots, degree)` basis: degree-elevates the
/// lower-degree one via [`elevate_degree`], then knot-inserts each up to the union of the other's
/// interior knot multiplicities — after which their homogeneous control nets index the same basis
/// functions, the precondition every later Coons step relies on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn harmonize_pair(a: &NurbsCurve3, b: &NurbsCurve3) -> Option<(KnotVector, Vec<Vec<f64>>, Vec<Vec<f64>>)> {
    let target_degree = a.knots.degree.max(b.knots.degree);
    let (mut ak, mut ac) = (a.knots.clone(), to_homogeneous(a));
    let (mut bk, mut bc) = (b.knots.clone(), to_homogeneous(b));
    if ak.degree < target_degree {
        let (nk, nc) = elevate_degree(&ak, &ac, target_degree - ak.degree);
        ak = nk;
        ac = nc;
    }
    if bk.degree < target_degree {
        let (nk, nc) = elevate_degree(&bk, &bc, target_degree - bk.degree);
        bk = nk;
        bc = nc;
    }
    let (a_lo, a_hi) = ak.domain();
    let (b_lo, b_hi) = bk.domain();
    if (a_lo - b_lo).abs() > 1e-9 || (a_hi - b_hi).abs() > 1e-9 {
        return None;
    }
    let mut values: Vec<f64> = ak.knots.iter().chain(bk.knots.iter()).copied().filter(|&k| k > a_lo && k < a_hi).collect();
    values.sort_by(|x, y| x.partial_cmp(y).unwrap());
    values.dedup_by(|x, y| (*x - *y).abs() < 1e-9);
    for &kv in &values {
        let need = ak.multiplicity(kv).max(bk.multiplicity(kv));
        while ak.multiplicity(kv) < need {
            let (nk, nc) = insert_knot_multi(&ak, &ac, kv);
            ak = nk;
            ac = nc;
        }
        while bk.multiplicity(kv) < need {
            let (nk, nc) = insert_knot_multi(&bk, &bc, kv);
            bk = nk;
            bc = nc;
        }
    }
    Some((ak, ac, bc))
}

/// 📏️ Elevates/knot-refines the 2-point linear (degree-1) curve through homogeneous points `p0`,
/// `p1` up to exactly `target`'s basis — used to expand a Coons ruled surface's linear direction, or
/// its bilinear corner rows, onto the shared knot vector the other direction needs. Endpoints are
/// preserved exactly throughout (a property of both Bézier elevation and knot insertion), so
/// `result[0] == p0` and `result.last() == p1` always.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn linear_curve_to_target(p0: &[f64], p1: &[f64], target: &KnotVector) -> Vec<Vec<f64>> {
    let mut degree = 1usize;
    let mut controls = vec![p0.to_vec(), p1.to_vec()];
    while degree < target.degree {
        controls = elevate_bezier_span_multi(&controls);
        degree += 1;
    }
    let mut knots = {
        let mut k = vec![0.0; degree + 1];
        k.extend(std::iter::repeat_n(1.0, degree + 1));
        KnotVector { knots: k, degree }
    };
    let (lo, hi) = target.domain();
    let mut distinct: Vec<f64> = target.knots.iter().copied().filter(|&x| x > lo && x < hi).collect();
    distinct.sort_by(|a, b| a.partial_cmp(b).unwrap());
    distinct.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    for &kv in &distinct {
        let need = target.multiplicity(kv);
        while knots.multiplicity(kv) < need {
            let (nk, nc) = insert_knot_multi(&knots, &controls, kv);
            knots = nk;
            controls = nc;
        }
    }
    controls
}

/// 📏️ Bilinearly-blended Coons patch through 4 boundary curves, expressed as an *exact* NURBS
/// surface (NURBS Book §10.5 construction, entirely in homogeneous control-point space — no
/// sampling): `c0`/`c1` are the `v=0`/`v=1` boundaries (functions of `u`), `d0`/`d1` the `u=0`/`u=1`
/// boundaries (functions of `v`); corners must agree within `tol`
/// (`c0(0)≈d0(0)`, `c0(1)≈d1(0)`, `c1(0)≈d0(1)`, `c1(1)≈d1(1)`). Builds two ruled surfaces (linear
/// blend of `c0`/`c1` in `v`, and of `d0`/`d1` in `u`) and one bilinear corner surface, all
/// harmonized onto one shared `(u_knots, v_knots)` via [`harmonize_pair`]/[`linear_curve_to_target`],
/// then combines `ruled1 + ruled2 - bilinear` channel-wise in homogeneous space — which, because the
/// bilinear term's rows/columns are built from the exact same homogeneous corner values the ruled
/// surfaces already carry, reproduces each boundary curve exactly at its edge of the patch (the
/// classical Coons cancellation identity, verified per-channel so it holds for the weight channel
/// too, not just position). Returns `None` on inconsistent corners, mismatched domains, or a
/// degenerate (near-zero) combined weight.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn coons_patch_nurbs(c0: &NurbsCurve3, c1: &NurbsCurve3, d0: &NurbsCurve3, d1: &NurbsCurve3, tol: f64) -> Option<Surface> {
    if c0.controls.first()?.distance(*d0.controls.first()?) > tol {
        return None;
    }
    if c0.controls.last()?.distance(*d1.controls.first()?) > tol {
        return None;
    }
    if c1.controls.first()?.distance(*d0.controls.last()?) > tol {
        return None;
    }
    if c1.controls.last()?.distance(*d1.controls.last()?) > tol {
        return None;
    }
    let (ku, c0h, c1h) = harmonize_pair(c0, c1)?;
    let (kv, d0h, d1h) = harmonize_pair(d0, d1)?;
    let nu = ku.control_point_count();
    let nv = kv.control_point_count();
    let mut ruled1 = vec![vec![vec![0.0; 4]; nv]; nu];
    for i in 0..nu {
        let row = linear_curve_to_target(&c0h[i], &c1h[i], &kv);
        for j in 0..nv {
            ruled1[i][j] = row[j].clone();
        }
    }
    let mut ruled2 = vec![vec![vec![0.0; 4]; nv]; nu];
    for j in 0..nv {
        let col = linear_curve_to_target(&d0h[j], &d1h[j], &ku);
        for i in 0..nu {
            ruled2[i][j] = col[i].clone();
        }
    }
    let bilinear_row_v0 = linear_curve_to_target(&c0h[0], &c0h[nu - 1], &ku);
    let bilinear_row_v1 = linear_curve_to_target(&c1h[0], &c1h[nu - 1], &ku);
    let mut bilinear = vec![vec![vec![0.0; 4]; nv]; nu];
    for i in 0..nu {
        let col = linear_curve_to_target(&bilinear_row_v0[i], &bilinear_row_v1[i], &kv);
        for j in 0..nv {
            bilinear[i][j] = col[j].clone();
        }
    }
    let mut controls = vec![vec![Pnt3::new(0.0, 0.0, 0.0); nv]; nu];
    let mut weights = vec![vec![1.0; nv]; nu];
    for i in 0..nu {
        for j in 0..nv {
            let mut h = [0.0; 4];
            for c in 0..4 {
                h[c] = ruled1[i][j][c] + ruled2[i][j][c] - bilinear[i][j][c];
            }
            let w = h[3];
            if w.abs() <= 1e-12 {
                return None;
            }
            controls[i][j] = Pnt3::new(h[0] / w, h[1] / w, h[2] / w);
            weights[i][j] = w;
        }
    }
    Some(Surface::Nurbs { u_knots: ku, v_knots: kv, controls, weights })
}

// #endregion 🔖️SurfaceFit

// #region 🔖️Edit

/// 📏️ Reverses a NURBS curve's direction: reverses control points/weights and mirrors the knot
/// vector around the domain, so `reverse(c).eval(domain.1 - (t - domain.0)) == c.eval(t)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn reverse_nurbs(curve: &NurbsCurve3) -> NurbsCurve3 {
    let (lo, hi) = curve.knots.domain();
    let mut controls = curve.controls.clone();
    controls.reverse();
    let mut weights = curve.weights.clone();
    weights.reverse();
    let knots: Vec<f64> = curve.knots.knots.iter().rev().map(|&k| lo + hi - k).collect();
    NurbsCurve3 { knots: KnotVector { knots, degree: curve.knots.degree }, controls, weights }
}

/// 📏️ Splits a NURBS curve at `t` into two curves, each covering one side of the original domain,
/// via repeated knot insertion until `t` reaches full multiplicity (`degree + 1`), then slicing
/// the (now Bezier-joined) control net at that knot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_nurbs(curve: &NurbsCurve3, t: f64) -> (NurbsCurve3, NurbsCurve3) {
    let degree = curve.knots.degree;
    let mut knots = curve.knots.clone();
    let mut hx: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.x * w).collect();
    let mut hy: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.y * w).collect();
    let mut hz: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.z * w).collect();
    let mut hw = curve.weights.clone();
    let needed = degree + 1 - knots.multiplicity(t);
    for _ in 0..needed {
        let (nk, nx) = insert_knot(&knots, &hx, t);
        let (_, ny) = insert_knot(&knots, &hy, t);
        let (_, nz) = insert_knot(&knots, &hz, t);
        let (_, nw) = insert_knot(&knots, &hw, t);
        knots = nk;
        hx = nx;
        hy = ny;
        hz = nz;
        hw = nw;
    }
    // Once t has full multiplicity (degree+1), it occupies consecutive knot indices [k, k+degree]
    // for some k; find_span(t) returns k+degree (the span ending exactly at t), so k = span-degree.
    // The control net splits cleanly there: the left piece owns points [0, k), the right [k, end).
    let k = knots.find_span(t) - degree;
    let dehomogenize = |i: usize| Pnt3::new(hx[i] / hw[i], hy[i] / hw[i], hz[i] / hw[i]);
    let left_controls: Vec<Pnt3> = (0..k).map(dehomogenize).collect();
    let left_weights: Vec<f64> = hw[0..k].to_vec();
    let right_controls: Vec<Pnt3> = (k..hx.len()).map(dehomogenize).collect();
    let right_weights: Vec<f64> = hw[k..].to_vec();
    let left_knot_count = left_controls.len() + degree + 1;
    let right_knot_count = right_controls.len() + degree + 1;
    let left_knots = knots.knots[0..left_knot_count].to_vec();
    let right_knots = knots.knots[knots.knots.len() - right_knot_count..].to_vec();
    (NurbsCurve3 { knots: KnotVector { knots: left_knots, degree }, controls: left_controls, weights: left_weights }, NurbsCurve3 { knots: KnotVector { knots: right_knots, degree }, controls: right_controls, weights: right_weights })
}

// #endregion 🔖️Edit

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::super::bspline::de_boor;
    use super::Curve3;
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Vec3;

    #[semio_framework_async_macros::async_test]
    async fn arc_length_of_line_equals_euclidean_distance() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(3.0, 4.0, 0.0) };
        let len = arc_length(&l, 0.0, 1.0, 1e-9);
        assert!((len - 5.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn arc_length_of_quarter_circle_matches_closed_form() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let len = arc_length(&c, 0.0, std::f64::consts::FRAC_PI_2, 1e-9);
        assert!((len - std::f64::consts::PI).abs() < 1e-6); // quarter of 2*pi*r=4pi, i.e. pi
    }

    #[semio_framework_async_macros::async_test]
    async fn param_at_length_round_trips_with_arc_length() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 3.0 };
        let total = arc_length(&c, 0.0, 2.0, 1e-9);
        let target = total * 0.4;
        let t = param_at_length(&c, 0.0, 2.0, target, 1e-9);
        let recomputed = arc_length(&c, 0.0, t, 1e-9);
        assert!((recomputed - target).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_circle_matches_radial_projection() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 0.0);
        let cp = closest_parameter(&c, (0.0, std::f64::consts::TAU), target, 1e-9);
        assert!(cp.certified);
        assert!((cp.distance - 8.0).abs() < 1e-6);
        assert!(cp.point.distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_line_matches_perpendicular_foot() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
        let target = Pnt3::new(5.0, 3.0, 0.0);
        let cp = closest_parameter(&l, (-10.0, 10.0), target, 1e-9);
        assert!((cp.t - 5.0).abs() < 1e-6);
        assert!((cp.distance - 3.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_parameter_on_circle_is_the_unique_local_minimum() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 0.0);
        let minima = all_closest_parameters(&c, (0.0, std::f64::consts::TAU), target, 1e-9);
        assert_eq!(minima.len(), 1, "a circle has exactly one local minimum of distance to an off-center point: {minima:?}");
        assert!(minima[0].point.distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn ellipse_closest_parameter_matches_dense_sampling_oracle() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, -2.0, 0.5), Vec3::new(0.1, 0.2, 1.0)).unwrap();
        let e = Curve3::Ellipse { frame, major_radius: 4.0, minor_radius: 1.5 };
        let target = Pnt3::new(6.0, 3.0, 2.0);
        let cp = closest_parameter(&e, (0.0, std::f64::consts::TAU), target, 1e-9);
        assert!(cp.certified);
        let oracle = (0..=200000).map(|i| e.eval(std::f64::consts::TAU * i as f64 / 200000.0).distance(target)).fold(f64::INFINITY, f64::min);
        assert!((cp.distance - oracle).abs() < 1e-4, "quartic={} oracle={oracle}", cp.distance);
    }

    /// 📏️ Recomputes the same centripetal parameter values `interpolate_centripetal` assigns to
    /// each data point — an independent oracle so the test checks the actual interpolation
    /// property (curve(param[i]) == points[i]) instead of a dense-sampling proxy, which can show
    /// a spurious "gap" purely from sampling resolution near fast-moving parts of the curve.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn centripetal_params(points: &[Pnt3]) -> Vec<f64> {
        let n = points.len();
        let mut chord_sqrt = vec![0.0; n];
        for i in 1..n {
            chord_sqrt[i] = points[i].distance(points[i - 1]).sqrt();
        }
        let total: f64 = chord_sqrt.iter().sum();
        let mut params = vec![0.0; n];
        let mut acc = 0.0;
        for i in 1..n {
            acc += chord_sqrt[i];
            params[i] = acc / total;
        }
        params
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_centripetal_passes_through_all_points() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 2.0, 0.0), Pnt3::new(3.0, 1.0, 0.0), Pnt3::new(4.0, 3.0, 1.0)];
        let curve = interpolate_centripetal(&points).unwrap();
        let params = centripetal_params(&points);
        for (p, t) in points.iter().zip(params.iter()) {
            let on_curve = de_boor_pnt(&curve, *t);
            assert!(on_curve.distance(*p) < 1e-6, "point {p:?} not interpolated at its own parameter t={t}: got {on_curve:?}");
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn de_boor_pnt(curve: &NurbsCurve3, t: f64) -> Pnt3 {
        let hx: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.x * w).collect();
        let hy: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.y * w).collect();
        let hz: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.z * w).collect();
        let w = de_boor(&curve.knots, &curve.weights, t);
        Pnt3::new(de_boor(&curve.knots, &hx, t) / w, de_boor(&curve.knots, &hy, t) / w, de_boor(&curve.knots, &hz, t) / w)
    }

    #[semio_framework_async_macros::async_test]
    async fn reverse_nurbs_reproduces_the_same_curve_reversed() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 1.0, 1.0) };
        let nurbs = l.to_nurbs((0.0, 4.0));
        let reversed = reverse_nurbs(&nurbs);
        let (lo, hi) = nurbs.knots.domain();
        for i in 0..=10 {
            let t = lo + (hi - lo) * i as f64 / 10.0;
            let original = de_boor_pnt(&nurbs, t);
            let via_reversed = de_boor_pnt(&reversed, hi - (t - lo));
            assert!(original.distance(via_reversed) < 1e-9, "mismatch at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn split_nurbs_pieces_reproduce_the_original_curve() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 3.0, 0.0), Pnt3::new(3.0, -1.0, 1.0), Pnt3::new(5.0, 2.0, 2.0), Pnt3::new(6.0, 0.0, 0.0)];
        let curve = interpolate_centripetal(&points).unwrap();
        let (lo, hi) = curve.knots.domain();
        let split_t = lo + (hi - lo) * 0.4;
        let (left, right) = split_nurbs(&curve, split_t);
        let (left_lo, left_hi) = left.knots.domain();
        let (right_lo, right_hi) = right.knots.domain();
        assert!((left_hi - split_t).abs() < 1e-9);
        assert!((right_lo - split_t).abs() < 1e-9);
        for i in 0..=15 {
            let t = left_lo + (left_hi - left_lo) * i as f64 / 15.0;
            assert!(de_boor_pnt(&left, t).distance(de_boor_pnt(&curve, t)) < 1e-7, "left mismatch at t={t}");
        }
        for i in 0..=15 {
            let t = right_lo + (right_hi - right_lo) * i as f64 / 15.0;
            assert!(de_boor_pnt(&right, t).distance(de_boor_pnt(&curve, t)) < 1e-7, "right mismatch at t={t}");
        }
        // The split point itself must match exactly from both sides.
        assert!(de_boor_pnt(&left, left_hi).distance(de_boor_pnt(&right, right_lo)) < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_curve_passes_through_every_point_at_its_own_parameter() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(3.0, 4.0, 1.0), Pnt3::new(5.0, 3.0, 2.0), Pnt3::new(7.0, 5.0, 1.0), Pnt3::new(8.0, 2.0, 0.0)];
        let curve = interpolate_curve(&points, 3, ParamMethod::Centripetal, None, false).unwrap();
        let params = parameterize(&points, ParamMethod::Centripetal);
        for (p, &t) in points.iter().zip(&params) {
            assert!(eval_nurbs_curve3(&curve, t).distance(*p) < 1e-10, "point {p:?} not interpolated at t={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_curve_honours_specified_end_tangents() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 2.0, 0.0), Pnt3::new(3.0, 1.0, 0.0), Pnt3::new(4.0, 3.0, 0.0)];
        let d0 = Vec3::new(1.0, 0.0, 0.0);
        let d1 = Vec3::new(0.0, 1.0, 0.0);
        let curve = interpolate_curve(&points, 3, ParamMethod::Centripetal, Some((d0, d1)), false).unwrap();
        let (lo, hi) = curve.knots.domain();
        assert!(eval_nurbs_curve3(&curve, lo).distance(points[0]) < 1e-9, "start point not interpolated");
        assert!(eval_nurbs_curve3(&curve, hi).distance(*points.last().unwrap()) < 1e-9, "end point not interpolated");
        let h = 1e-6;
        let start_tangent = (eval_nurbs_curve3(&curve, lo + h) - eval_nurbs_curve3(&curve, lo)) * (1.0 / h);
        let end_tangent = (eval_nurbs_curve3(&curve, hi) - eval_nurbs_curve3(&curve, hi - h)) * (1.0 / h);
        assert!((start_tangent.normalized().unwrap() - d0.normalized().unwrap()).norm() < 1e-4, "start tangent mismatch: {start_tangent:?}");
        assert!((end_tangent.normalized().unwrap() - d1.normalized().unwrap()).norm() < 1e-4, "end tangent mismatch: {end_tangent:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_curve_closed_passes_through_points_and_is_c_degree_minus_one_at_the_seam() {
        // A regular pentagon's vertices — a natural closed loop.
        let n = 5;
        let radius = 3.0;
        let points: Vec<Pnt3> = (0..n)
            .map(|i| {
                let a = std::f64::consts::TAU * i as f64 / n as f64;
                Pnt3::new(radius * a.cos(), radius * a.sin(), 0.0)
            })
            .collect();
        let degree = 3;
        let curve = interpolate_curve(&points, degree, ParamMethod::Uniform, None, true).unwrap();
        let knots = &curve.knots;
        assert!(knots.is_periodic());
        let (lo, hi) = knots.domain();
        for (i, p) in points.iter().enumerate() {
            let t = lo + (hi - lo) * i as f64 / n as f64;
            assert!(eval_nurbs_curve3(&curve, t).distance(*p) < 1e-8, "vertex {i} not interpolated at t={t}");
        }
        // C^(degree-1) continuity at the seam: derivatives up to order degree-1 must agree when
        // approached from just below `hi` and just above `lo` (mod period).
        let h = 1e-5;
        for order in 1..degree {
            let before = finite_diff_order(&curve, hi - h, order, h);
            let after = finite_diff_order(&curve, lo + h, order, h);
            assert!((before - after).norm() < 1e-1, "order {order} derivative discontinuous at the seam: before={before:?} after={after:?}");
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finite_diff_order(curve: &NurbsCurve3, t: f64, order: usize, h: f64) -> Vec3 {
        let f = |x: f64| eval_nurbs_curve3(curve, x).to_vec();
        match order {
            1 => (f(t + h) - f(t - h)) * (1.0 / (2.0 * h)),
            2 => (f(t + h) - f(t) * 2.0 + f(t - h)) * (1.0 / (h * h)),
            _ => Vec3::ZERO,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn approximate_curve_achieves_the_requested_error_bound() {
        let mut points = Vec::new();
        for i in 0..40 {
            let t = i as f64 / 39.0 * std::f64::consts::TAU;
            points.push(Pnt3::new(t.cos() * 5.0, t.sin() * 5.0, 0.1 * (3.0 * t).sin()));
        }
        let max_error = 0.05;
        let (curve, err) = approximate_curve(&points, 3, max_error).unwrap();
        assert!(err <= max_error * 1.0001, "achieved error {err} exceeds requested bound {max_error}");
        assert!(curve.controls.len() < points.len(), "approximation should use fewer controls than data points here");
    }

    #[semio_framework_async_macros::async_test]
    async fn approximate_curve_with_count_matches_endpoints_exactly() {
        let points: Vec<Pnt3> = (0..12).map(|i| Pnt3::new(i as f64, (i as f64 * 0.7).sin() * 3.0, 0.0)).collect();
        let (curve, _err) = approximate_curve_with_count(&points, 3, 6).unwrap();
        assert!(curve.controls[0].distance(points[0]) < 1e-12);
        assert!(curve.controls.last().unwrap().distance(*points.last().unwrap()) < 1e-12);
    }

    /// 📏️ Recomputes [`interpolate_surface_grid`]'s own u/v parameter averaging (NURBS Book §9.5:
    /// each direction's parameter is the average, over every row/column of the OTHER direction,
    /// of that row/column's own centripetal parameterization) via the same public [`parameterize`]
    /// primitive — an independent oracle, so the pass-through test below checks the actual
    /// interpolation property at the parameters the algorithm actually assigned, instead of
    /// assuming they land on a uniform `i/(n-1)` grid (which centripetal parameterization does NOT
    /// guarantee for non-uniformly-spaced 3D data — this grid's `z` isn't linear in `i`/`j`, so its
    /// averaged parameters are close to, but not exactly, uniform).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn grid_params(points: &[Vec<Pnt3>]) -> (Vec<f64>, Vec<f64>) {
        let nu = points.len();
        let nv = points[0].len();
        let mut u_acc = vec![0.0; nu];
        for j in 0..nv {
            let column: Vec<Pnt3> = (0..nu).map(|i| points[i][j]).collect();
            let p = parameterize(&column, ParamMethod::Centripetal);
            for i in 0..nu {
                u_acc[i] += p[i];
            }
        }
        let u_params: Vec<f64> = u_acc.iter().map(|&s| s / nv as f64).collect();
        let mut v_acc = vec![0.0; nv];
        for row in points {
            let p = parameterize(row, ParamMethod::Centripetal);
            for j in 0..nv {
                v_acc[j] += p[j];
            }
        }
        let v_params: Vec<f64> = v_acc.iter().map(|&s| s / nu as f64).collect();
        (u_params, v_params)
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_surface_grid_passes_through_every_grid_point() {
        let nu = 4;
        let nv = 5;
        let points: Vec<Vec<Pnt3>> = (0..nu).map(|i| (0..nv).map(|j| Pnt3::new(i as f64, j as f64, (i as f64 * 0.5).sin() + (j as f64 * 0.3).cos())).collect()).collect();
        let surface = interpolate_surface_grid(&points, 3, 3).unwrap();
        let Surface::Nurbs { u_knots, v_knots, controls, weights } = &surface else { panic!("expected a Nurbs surface") };
        let (u_params, v_params) = grid_params(&points);
        for i in 0..nu {
            for j in 0..nv {
                let evaluated = eval_nurbs_surface_test(u_knots, v_knots, controls, weights, u_params[i], v_params[j]);
                assert!(evaluated.distance(points[i][j]) < 1e-8, "grid point ({i},{j}) not interpolated: got {evaluated:?} expected {:?}", points[i][j]);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn eval_nurbs_surface_test(u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], u: f64, v: f64) -> Pnt3 {
        let u_span = u_knots.find_span(u);
        let v_span = v_knots.find_span(v);
        let nu = basis_functions(u_knots, u_span, u);
        let nv = basis_functions(v_knots, v_span, v);
        let up = u_knots.degree;
        let vp = v_knots.degree;
        let (mut hx, mut hy, mut hz, mut hw) = (0.0, 0.0, 0.0, 0.0);
        for i in 0..=up {
            for j in 0..=vp {
                let ci = u_span - up + i;
                let cj = v_span - vp + j;
                let b = nu[i] * nv[j] * weights[ci][cj];
                hx += b * controls[ci][cj].x;
                hy += b * controls[ci][cj].y;
                hz += b * controls[ci][cj].z;
                hw += b;
            }
        }
        Pnt3::new(hx / hw, hy / hw, hz / hw)
    }

    #[semio_framework_async_macros::async_test]
    async fn coons_patch_nurbs_reproduces_its_four_boundary_curves_exactly() {
        let c0 = interpolate_curve(&[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.5), Pnt3::new(2.0, 0.0, 0.0)], 2, ParamMethod::Uniform, None, false).unwrap();
        let c1 = interpolate_curve(&[Pnt3::new(0.0, 3.0, 0.2), Pnt3::new(1.0, 3.0, 0.8), Pnt3::new(2.0, 3.0, 0.2)], 2, ParamMethod::Uniform, None, false).unwrap();
        let d0 = interpolate_curve(&[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 1.5, 0.4), Pnt3::new(0.0, 3.0, 0.2)], 2, ParamMethod::Uniform, None, false).unwrap();
        let d1 = interpolate_curve(&[Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.5, 0.5), Pnt3::new(2.0, 3.0, 0.2)], 2, ParamMethod::Uniform, None, false).unwrap();
        let surface = coons_patch_nurbs(&c0, &c1, &d0, &d1, 1e-6).expect("consistent corners must produce a Coons surface");
        for i in 0..=10 {
            let u = i as f64 / 10.0;
            assert!(surface.eval(u, 0.0).distance(eval_nurbs_curve3(&c0, u)) < 1e-7, "v=0 boundary mismatch at u={u}");
            assert!(surface.eval(u, 1.0).distance(eval_nurbs_curve3(&c1, u)) < 1e-7, "v=1 boundary mismatch at u={u}");
        }
        for j in 0..=10 {
            let v = j as f64 / 10.0;
            assert!(surface.eval(0.0, v).distance(eval_nurbs_curve3(&d0, v)) < 1e-7, "u=0 boundary mismatch at v={v}");
            assert!(surface.eval(1.0, v).distance(eval_nurbs_curve3(&d1, v)) < 1e-7, "u=1 boundary mismatch at v={v}");
        }
    }

    /// 📏️ An S-shaped cubic Bézier (point-symmetric about `(3, 0, 0)`): `P0=(0,0,0)`,
    /// `P1=(0,4,0)`, `P2=(6,-4,0)`, `P3=(6,0,0)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn s_shaped_nurbs() -> Curve3 {
        Curve3::Nurbs {
            knots: KnotVector::clamped_uniform(4, 3),
            controls: vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 4.0, 0.0), Pnt3::new(6.0, -4.0, 0.0), Pnt3::new(6.0, 0.0, 0.0)],
            weights: vec![1.0; 4],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn all_closest_parameters_finds_both_minima_of_an_s_shaped_curve() {
        let curve = s_shaped_nurbs();
        let target = Pnt3::new(3.0, 0.0, 10.0);
        let minima = all_closest_parameters(&curve, curve.domain(), target, 1e-9);
        assert!(minima.len() >= 2, "expected at least two local minima on the symmetric S-curve, found {}: {minima:?}", minima.len());
        let brute_min = (0..=200000).map(|i| curve.eval(i as f64 / 200000.0).distance(target)).fold(f64::INFINITY, f64::min);
        let best = minima.iter().map(|m| m.distance).fold(f64::INFINITY, f64::min);
        assert!((best - brute_min).abs() < 1e-4, "certified best={best} oracle={brute_min}");
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_parameter_on_nurbs_matches_dense_sampling_oracle() {
        let curve = s_shaped_nurbs();
        let target = Pnt3::new(2.0, 5.0, -1.0);
        let cp = closest_parameter(&curve, curve.domain(), target, 1e-9);
        assert!(cp.certified);
        let oracle = (0..=200000).map(|i| curve.eval(i as f64 / 200000.0).distance(target)).fold(f64::INFINITY, f64::min);
        assert!((cp.distance - oracle).abs() < 1e-4, "subdivision={} oracle={oracle}", cp.distance);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_parameter_of_a_point_on_the_curve_recovers_its_own_parameter() {
        let curve = s_shaped_nurbs();
        let t0 = 0.37;
        let on_curve = curve.eval(t0);
        let cp = closest_parameter(&curve, curve.domain(), on_curve, 1e-9);
        assert!(cp.distance < 1e-8, "distance should be ~0 for a point exactly on the curve: {}", cp.distance);
        assert!((cp.t - t0).abs() < 1e-5, "expected parameter {t0}, got {}", cp.t);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_parameter_on_circle_handles_seam_crossing_targets() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 3.0 };
        // Target angularly just past the 0/2π seam — the true closest point sits at t≈2π-ε.
        let angle = -0.02_f64;
        let target = Pnt3::from_array((c.eval(angle).to_vec() * 1.5).to_array());
        let cp = closest_parameter(&c, (0.0, std::f64::consts::TAU), target, 1e-9);
        let expected = std::f64::consts::TAU + angle;
        assert!((cp.t - expected).abs() < 1e-6, "seam wrap failed: t={}, expected near {expected}", cp.t);
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn closest_point_matches_brute_force_dense_sampling_oracle() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(61);
            for _ in 0..100 {
                let frame =
                    Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z))
                        .unwrap();
                let radius = 0.5 + rng.next_f64() * 5.0;
                let c = Curve3::Circle { frame, radius };
                let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let cp = closest_parameter(&c, (0.0, std::f64::consts::TAU), target, 1e-9);
                let oracle_dist = (0..=100000).map(|i| c.eval(std::f64::consts::TAU * i as f64 / 100000.0).distance(target)).fold(f64::INFINITY, f64::min);
                assert!((cp.distance - oracle_dist).abs() < 1e-4, "mismatch: closed-form={} oracle={oracle_dist}", cp.distance);
            }
        }
    }
}
// #endregion 🔖️Tests
