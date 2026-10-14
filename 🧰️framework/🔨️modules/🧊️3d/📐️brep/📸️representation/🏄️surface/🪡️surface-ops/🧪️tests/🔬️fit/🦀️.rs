use super::*;
use semio_framework_3d_brep_curve::bspline::basis_functions;
use semio_framework_3d_brep_curve::curve_ops::{eval_nurbs_curve3, interpolate_curve};

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
