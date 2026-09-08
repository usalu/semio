
use super::super::bspline::de_boor;
use super::Curve3;
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::Vec3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;

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
    Curve3::Nurbs { knots: KnotVector::clamped_uniform(4, 3), controls: vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 4.0, 0.0), Pnt3::new(6.0, -4.0, 0.0), Pnt3::new(6.0, 0.0, 0.0)], weights: vec![1.0; 4] }
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
                Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z)).unwrap();
            let radius = 0.5 + rng.next_f64() * 5.0;
            let c = Curve3::Circle { frame, radius };
            let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
            let cp = closest_parameter(&c, (0.0, std::f64::consts::TAU), target, 1e-9);
            let oracle_dist = (0..=100000).map(|i| c.eval(std::f64::consts::TAU * i as f64 / 100000.0).distance(target)).fold(f64::INFINITY, f64::min);
            assert!((cp.distance - oracle_dist).abs() < 1e-4, "mismatch: closed-form={} oracle={oracle_dist}", cp.distance);
        }
    }
}
