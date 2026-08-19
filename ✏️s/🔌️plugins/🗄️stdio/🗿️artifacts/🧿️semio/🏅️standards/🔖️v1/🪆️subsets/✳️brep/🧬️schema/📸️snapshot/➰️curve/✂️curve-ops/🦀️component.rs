//! 📏️ Curve algorithms that operate *on* a [`super::Curve3`] rather than being part of its
//! definition: arc length, closest-point projection, and the split/reverse/join operations edges
//! need when Euler operators cut a curve. Kept separate from `curve.rs` so that file stays a pure
//! evaluation interface and this one can grow numerically heavier machinery independently.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/✂️curve-ops` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➰️curve` per that file's own pre-mounted-stub note.

use super::bspline::{basis_functions, insert_knot, KnotVector};
use super::{Curve3, NurbsCurve3};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::Pnt3;

// #region 🔖️Length

/// 📏️ 5-point Gauss-Legendre nodes/weights on `[-1, 1]`.
const GL5_NODES: [f64; 5] = [-0.906_179_845_938_664, -0.538_469_310_105_683_1, 0.0, 0.538_469_310_105_683_1, 0.906_179_845_938_664];
const GL5_WEIGHTS: [f64; 5] = [0.2369268850561891, 0.4786286704993665, 0.5688888888888889, 0.4786286704993665, 0.2369268850561891];

async fn gauss_legendre5(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let mid = 0.5 * (a + b);
    let half = 0.5 * (b - a);
    GL5_NODES.iter().zip(GL5_WEIGHTS.iter()).map(|(&x, &w)| w * f(mid + half * x)).sum::<f64>() * half
}

/// 📏️ Adaptive-quadrature arc length of `curve` over `[t0, t1]`: recursively halves the interval
/// until the 5-point Gauss-Legendre estimate agrees with the sum of its two half-interval
/// estimates to within `tol` (Richardson-style error control), or `max_depth` is reached.
pub async fn arc_length(curve: &Curve3, t0: f64, t1: f64, tol: f64) -> f64 {
    arc_length_recursive(curve, t0, t1, tol, 24)
}

async fn arc_length_recursive(curve: &Curve3, t0: f64, t1: f64, tol: f64, depth: u32) -> f64 {
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
pub async fn param_at_length(curve: &Curve3, t0: f64, t1: f64, target_length: f64, tol: f64) -> f64 {
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

/// 📏️ Closest point on `curve` (restricted to `domain`) to `target`: coarse uniform sampling
/// (`samples` intervals) seeds a safeguarded Newton refinement of `f(t) = (C(t)-P)·C'(t) = 0`
/// (the standard first-order optimality condition for point-curve distance) from the best sample
/// and its neighbors, keeping the global best result found. Returns `(t, distance)`.
pub async fn closest_point(curve: &Curve3, domain: (f64, f64), target: Pnt3, samples: usize) -> (f64, f64) {
    let mut best_t = domain.0;
    let mut best_d2 = curve.eval(domain.0).distance_sq(target);
    for i in 0..=samples {
        let t = domain.0 + (domain.1 - domain.0) * (i as f64 / samples as f64);
        let d2 = curve.eval(t).distance_sq(target);
        if d2 < best_d2 {
            best_d2 = d2;
            best_t = t;
        }
    }
    // For a periodic curve, the true minimum can sit just across the domain boundary from the
    // best coarse sample (e.g. near angle 0 when the closest point is actually at 2π-ε) — a hard
    // clamp would trap Newton exactly at that boundary. Wrap into the period instead of clamping.
    let refined = newton_closest_point(curve, target, best_t, domain, curve.period());
    let refined_d2 = curve.eval(refined).distance_sq(target);
    if refined_d2 < best_d2 {
        (refined, refined_d2.sqrt())
    } else {
        (best_t, best_d2.sqrt())
    }
}

async fn wrap_into_domain(t: f64, domain: (f64, f64), period: Option<f64>) -> f64 {
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

async fn newton_closest_point(curve: &Curve3, target: Pnt3, mut t: f64, domain: (f64, f64), period: Option<f64>) -> f64 {
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

/// 📏️ All local extrema of distance-to-`target` on `curve` over `domain` (both minima and
/// maxima), found by sign changes of `f(t) = (C(t)-P)·C'(t)` across a uniform sample, each refined
/// by the same Newton step as [`closest_point`]. Used where a caller needs every critical point,
/// not just the global closest (e.g. offset self-intersection analysis in later phases).
pub async fn all_extrema(curve: &Curve3, domain: (f64, f64), target: Pnt3, samples: usize) -> Vec<f64> {
    let f = |t: f64| (curve.eval(t) - target).dot(curve.d1(t));
    let mut roots = Vec::new();
    let mut prev_t = domain.0;
    let mut prev_f = f(prev_t);
    for i in 1..=samples {
        let t = domain.0 + (domain.1 - domain.0) * (i as f64 / samples as f64);
        let ft = f(t);
        if prev_f == 0.0 {
            roots.push(prev_t);
        } else if prev_f.signum() != ft.signum() {
            roots.push(newton_closest_point(curve, target, 0.5 * (prev_t + t), (prev_t, t), None));
        }
        prev_t = t;
        prev_f = ft;
    }
    if prev_f == 0.0 {
        roots.push(prev_t);
    }
    roots
}

// #endregion 🔖️Project

// #region 🔖️Fit

/// 📏️ Global cubic interpolation through `points` using centripetal parameterization (Lee's
/// method) — the standard, well-conditioned choice for interpolating scattered points without
/// the cusping chord-length parametrization can produce.
pub async fn interpolate_centripetal(points: &[Pnt3]) -> Option<NurbsCurve3> {
    let n = points.len();
    if n < 2 {
        return None;
    }
    let degree = (n - 1).min(3);
    let mut chord_sqrt = vec![0.0; n];
    for i in 1..n {
        chord_sqrt[i] = points[i].distance(points[i - 1]).sqrt();
    }
    let total: f64 = chord_sqrt.iter().sum();
    if total <= 0.0 {
        return None;
    }
    let mut params = vec![0.0; n];
    let mut acc = 0.0;
    for i in 1..n {
        acc += chord_sqrt[i];
        params[i] = acc / total;
    }
    let mut knots = vec![0.0; degree + 1];
    for j in 1..n - degree {
        let avg: f64 = params[j..j + degree].iter().sum::<f64>() / degree as f64;
        knots.push(avg);
    }
    knots.extend(std::iter::repeat_n(1.0, degree + 1));
    let kv = KnotVector::new(knots, degree, n)?;
    let mut a = vec![vec![0.0; n]; n];
    for (row, &u) in params.iter().enumerate() {
        let span = kv.find_span(u);
        let basis = basis_functions(&kv, span, u);
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
    let controls = (0..n).map(|i| Pnt3::new(xs[i], ys[i], zs[i])).collect();
    Some(NurbsCurve3 { knots: kv, controls, weights: vec![1.0; n] })
}

/// 📏️ Plain Gaussian elimination with partial pivoting — the interpolation matrix is small
/// (control-point count) and banded but not worth a dedicated banded solver at this scale.
async fn solve_linear_system(a: &[Vec<f64>], rhs: &[f64]) -> Vec<f64> {
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

// #endregion 🔖️Fit

// #region 🔖️Edit

/// 📏️ Reverses a NURBS curve's direction: reverses control points/weights and mirrors the knot
/// vector around the domain, so `reverse(c).eval(domain.1 - (t - domain.0)) == c.eval(t)`.
pub async fn reverse_nurbs(curve: &NurbsCurve3) -> NurbsCurve3 {
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
pub async fn split_nurbs(curve: &NurbsCurve3, t: f64) -> (NurbsCurve3, NurbsCurve3) {
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

    #[test]
    async fn arc_length_of_line_equals_euclidean_distance() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(3.0, 4.0, 0.0) };
        let len = arc_length(&l, 0.0, 1.0, 1e-9);
        assert!((len - 5.0).abs() < 1e-6);
    }

    #[test]
    async fn arc_length_of_quarter_circle_matches_closed_form() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let len = arc_length(&c, 0.0, std::f64::consts::FRAC_PI_2, 1e-9);
        assert!((len - std::f64::consts::PI).abs() < 1e-6); // quarter of 2*pi*r=4pi, i.e. pi
    }

    #[test]
    async fn param_at_length_round_trips_with_arc_length() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 3.0 };
        let total = arc_length(&c, 0.0, 2.0, 1e-9);
        let target = total * 0.4;
        let t = param_at_length(&c, 0.0, 2.0, target, 1e-9);
        let recomputed = arc_length(&c, 0.0, t, 1e-9);
        assert!((recomputed - target).abs() < 1e-6);
    }

    #[test]
    async fn closest_point_on_circle_matches_radial_projection() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 0.0);
        let (t, dist) = closest_point(&c, (0.0, std::f64::consts::TAU), target, 64);
        assert!((dist - 8.0).abs() < 1e-6);
        let p = c.eval(t);
        assert!(p.distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6);
    }

    #[test]
    async fn closest_point_on_line_matches_perpendicular_foot() {
        let l = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
        let target = Pnt3::new(5.0, 3.0, 0.0);
        let (t, dist) = closest_point(&l, (-10.0, 10.0), target, 40);
        assert!((t - 5.0).abs() < 1e-6);
        assert!((dist - 3.0).abs() < 1e-6);
    }

    #[test]
    async fn all_extrema_finds_both_near_and_far_points_on_circle() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let c = Curve3::Circle { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 0.0);
        let extrema = all_extrema(&c, (0.0, std::f64::consts::TAU), target, 64);
        assert_eq!(extrema.len(), 2);
        let near = c.eval(extrema[0]).distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6 || c.eval(extrema[1]).distance(Pnt3::new(2.0, 0.0, 0.0)) < 1e-6;
        let far = c.eval(extrema[0]).distance(Pnt3::new(-2.0, 0.0, 0.0)) < 1e-6 || c.eval(extrema[1]).distance(Pnt3::new(-2.0, 0.0, 0.0)) < 1e-6;
        assert!(near && far);
    }

    /// 📏️ Recomputes the same centripetal parameter values `interpolate_centripetal` assigns to
    /// each data point — an independent oracle so the test checks the actual interpolation
    /// property (curve(param[i]) == points[i]) instead of a dense-sampling proxy, which can show
    /// a spurious "gap" purely from sampling resolution near fast-moving parts of the curve.
    async fn centripetal_params(points: &[Pnt3]) -> Vec<f64> {
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

    #[test]
    async fn interpolate_centripetal_passes_through_all_points() {
        let points = vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 2.0, 0.0), Pnt3::new(3.0, 1.0, 0.0), Pnt3::new(4.0, 3.0, 1.0)];
        let curve = interpolate_centripetal(&points).unwrap();
        let params = centripetal_params(&points);
        for (p, t) in points.iter().zip(params.iter()) {
            let on_curve = de_boor_pnt(&curve, *t);
            assert!(on_curve.distance(*p) < 1e-6, "point {p:?} not interpolated at its own parameter t={t}: got {on_curve:?}");
        }
    }

    async fn de_boor_pnt(curve: &NurbsCurve3, t: f64) -> Pnt3 {
        let hx: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.x * w).collect();
        let hy: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.y * w).collect();
        let hz: Vec<f64> = curve.controls.iter().zip(&curve.weights).map(|(p, w)| p.z * w).collect();
        let w = de_boor(&curve.knots, &curve.weights, t);
        Pnt3::new(de_boor(&curve.knots, &hx, t) / w, de_boor(&curve.knots, &hy, t) / w, de_boor(&curve.knots, &hz, t) / w)
    }

    #[test]
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

    #[test]
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

    mod quick {
        use super::*;

        #[test]
        async fn closest_point_matches_brute_force_dense_sampling_oracle() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(61);
            for _ in 0..100 {
                let frame =
                    Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z))
                        .unwrap();
                let radius = 0.5 + rng.next_f64() * 5.0;
                let c = Curve3::Circle { frame, radius };
                let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let (_, dist) = closest_point(&c, (0.0, std::f64::consts::TAU), target, 32);
                let oracle_dist = (0..=100000).map(|i| c.eval(std::f64::consts::TAU * i as f64 / 100000.0).distance(target)).fold(f64::INFINITY, f64::min);
                assert!((dist - oracle_dist).abs() < 1e-4, "mismatch: newton={dist} oracle={oracle_dist}");
            }
        }
    }
}
// #endregion 🔖️Tests
