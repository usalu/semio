//! 🧭️ General marching: the finite domain and cell helpers, the Gauss-Newton seeds, the tangent predictor and corrector march and the general marching tracer. See [`super`].

use super::*;

// #region 🔖️General
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn finite_domain(surface: &Surface) -> ((f64, f64), (f64, f64)) {
    let ((u0, u1), (v0, v1)) = surface.domain();
    let u_hi = if u1.is_finite() { u1 } else { u0 + std::f64::consts::TAU };
    let u_lo = if u0.is_finite() { u0 } else { u_hi - std::f64::consts::TAU };
    let v_hi = if v1.is_finite() { v1 } else { 10.0 };
    let v_lo = if v0.is_finite() { v0 } else { -10.0 };
    ((u_lo, u_hi), (v_lo, v_hi))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn cell_aabb(surface: &Surface, u0: f64, u1: f64, v0: f64, v1: f64) -> (Pnt3, Pnt3) {
    let mut lo = surface.eval(u0, v0);
    let mut hi = lo;
    for &(u, v) in &[(u0, v0), (u0, v1), (u1, v0), (u1, v1), ((u0 + u1) * 0.5, (v0 + v1) * 0.5)] {
        let p = surface.eval(u, v);
        lo = Pnt3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = Pnt3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    (lo, hi)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn aabb_overlap(a: (Pnt3, Pnt3), b: (Pnt3, Pnt3), tol: f64) -> bool {
    a.0.x - tol <= b.1.x && b.0.x - tol <= a.1.x && a.0.y - tol <= b.1.y && b.0.y - tol <= a.1.y && a.0.z - tol <= b.1.z && b.0.z - tol <= a.1.z
}

/// 🏄 Damped Gauss-Newton on the 4-unknown, 3-residual system `S_a(u_a,v_a) − S_b(u_b,v_b) = 0`
/// (underdetermined by one DOF along the curve — the damping just picks the nearby minimum-norm
/// root, which is exactly what a marching seed needs).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn gauss_newton_seed(a: &Surface, mut ua: f64, mut va: f64, b: &Surface, mut ub: f64, mut vb: f64, tol: f64) -> Option<SurfaceIntersectionSample> {
    for _ in 0..30 {
        let da = a.derivatives(ua, va);
        let db = b.derivatives(ub, vb);
        let r = da.point - db.point;
        if r.norm() <= tol {
            return Some((da.point, ua, va, ub, vb));
        }
        let cols = [da.du, da.dv, -db.du, -db.dv];
        let mut rows = vec![vec![0.0; 4]; 4];
        let mut rhs = vec![0.0; 4];
        for i in 0..4 {
            for j in 0..4 {
                rows[i][j] = cols[i].dot(cols[j]);
            }
            rows[i][i] += 1e-6;
            rhs[i] = -cols[i].dot(r);
        }
        let delta = super::super::shared::gauss_elim(&rows, &rhs);
        ua += delta[0];
        va += delta[1];
        ub += delta[2];
        vb += delta[3];
    }
    let pa = a.eval(ua, va);
    let pb = b.eval(ub, vb);
    if pa.distance(pb) <= tol * 20.0 {
        Some((pa, ua, va, ub, vb))
    } else {
        None
    }
}

/// 🏄 Coarse quadtree-lite seed finder: bins each surface's finite domain into an `n×n` cell grid,
/// keeps cell pairs whose sampled AABBs overlap, and Gauss-Newton-converges each surviving pair's
/// cell centers to a start point.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn find_seeds(a: &Surface, dom_a: ((f64, f64), (f64, f64)), b: &Surface, dom_b: ((f64, f64), (f64, f64)), tol: f64) -> Vec<SurfaceIntersectionSample> {
    const N: usize = 10;
    let cells = |surface: &Surface, dom: ((f64, f64), (f64, f64))| -> Vec<SurfacePatchBounds> {
        let mut out = Vec::new();
        for i in 0..N {
            for j in 0..N {
                let u0 = dom.0 .0 + (dom.0 .1 - dom.0 .0) * (i as f64 / N as f64);
                let u1 = dom.0 .0 + (dom.0 .1 - dom.0 .0) * ((i + 1) as f64 / N as f64);
                let v0 = dom.1 .0 + (dom.1 .1 - dom.1 .0) * (j as f64 / N as f64);
                let v1 = dom.1 .0 + (dom.1 .1 - dom.1 .0) * ((j + 1) as f64 / N as f64);
                out.push((u0, u1, v0, v1, cell_aabb(surface, u0, u1, v0, v1)));
            }
        }
        out
    };
    let cells_a = cells(a, dom_a);
    let cells_b = cells(b, dom_b);
    let mut seeds = Vec::new();
    for &(au0, au1, av0, av1, abox) in &cells_a {
        for &(bu0, bu1, bv0, bv1, bbox) in &cells_b {
            if !aabb_overlap(abox, bbox, tol) {
                continue;
            }
            if let Some(seed) = gauss_newton_seed(a, (au0 + au1) * 0.5, (av0 + av1) * 0.5, b, (bu0 + bu1) * 0.5, (bv0 + bv1) * 0.5, tol) {
                seeds.push(seed);
            }
        }
    }
    seeds
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn march_step(surface: &Surface, dom: ((f64, f64), (f64, f64)), tol: f64) -> f64 {
    let p00 = surface.eval(dom.0 .0, dom.1 .0);
    let p11 = surface.eval(dom.0 .1, dom.1 .1);
    let diag = p00.distance(p11).max(tol * 100.0);
    (diag * 0.02).clamp(tol * 20.0, diag.max(tol * 20.0))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn tangent_step_predict(du: Vec3, dv: Vec3, delta: Vec3) -> (f64, f64) {
    let e = du.dot(du);
    let f = du.dot(dv);
    let g = dv.dot(dv);
    let r0 = du.dot(delta);
    let r1 = dv.dot(delta);
    let det = e * g - f * f;
    if det.abs() <= 1e-300 {
        return (0.0, 0.0);
    }
    ((g * r0 - f * r1) / det, (e * r1 - f * r0) / det)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn out_of_domain(x: f64, lo: f64, hi: f64, periodic: bool, tol: f64) -> bool {
    if periodic || !(lo.is_finite() && hi.is_finite()) {
        false
    } else {
        x < lo - tol || x > hi + tol
    }
}

/// 🏄 Predictor-corrector march from `start` in one direction along the intersection curve:
/// predicts the next 3D point along `na × nb` (the curve's tangent), projects that step into each
/// surface's own tangent-plane basis for an initial `(u, v)` guess, then Newton-corrects the joint
/// system back onto `S_a = S_b`. Stops at a domain border, a tangential-contact point, or loop
/// closure back near `start`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
#[allow(clippy::too_many_arguments)]
fn march_direction(a: &Surface, dom_a: ((f64, f64), (f64, f64)), b: &Surface, dom_b: ((f64, f64), (f64, f64)), start: SurfaceIntersectionSample, tol: f64, sign: f64, max_steps: usize) -> Vec<SurfaceIntersectionSample> {
    let mut out = Vec::new();
    let (_, mut ua, mut va, mut ub, mut vb) = start;
    let step = march_step(a, dom_a, tol);
    for _ in 0..max_steps {
        let da = a.derivatives(ua, va);
        let db = b.derivatives(ub, vb);
        let (Some(na), Some(nb)) = (da.du.cross(da.dv).normalized(), db.du.cross(db.dv).normalized()) else { break };
        let raw = na.cross(nb);
        let tn = raw.norm();
        if tn <= 1e-9 {
            break;
        }
        let tangent = raw * (sign / tn);
        let (dua, dva) = tangent_step_predict(da.du, da.dv, tangent * step);
        let (dub, dvb) = tangent_step_predict(db.du, db.dv, tangent * step);
        let mut nua = ua + dua;
        let mut nva = va + dva;
        let mut nub = ub + dub;
        let mut nvb = vb + dvb;
        let mut ok = false;
        for _ in 0..8 {
            let da2 = a.derivatives(nua, nva);
            let db2 = b.derivatives(nub, nvb);
            let r = da2.point - db2.point;
            if r.norm() <= tol {
                ok = true;
                break;
            }
            let cols = [da2.du, da2.dv, -db2.du, -db2.dv];
            let mut rows = vec![vec![0.0; 4]; 4];
            let mut rhs = vec![0.0; 4];
            for i in 0..4 {
                for j in 0..4 {
                    rows[i][j] = cols[i].dot(cols[j]);
                }
                rows[i][i] += 1e-8;
                rhs[i] = -cols[i].dot(r);
            }
            let delta = super::super::shared::gauss_elim(&rows, &rhs);
            nua += delta[0];
            nva += delta[1];
            nub += delta[2];
            nvb += delta[3];
        }
        if !ok {
            break;
        }
        if out_of_domain(nua, dom_a.0 .0, dom_a.0 .1, a.is_u_periodic(), tol)
            || out_of_domain(nva, dom_a.1 .0, dom_a.1 .1, a.is_v_periodic(), tol)
            || out_of_domain(nub, dom_b.0 .0, dom_b.0 .1, b.is_u_periodic(), tol)
            || out_of_domain(nvb, dom_b.1 .0, dom_b.1 .1, b.is_v_periodic(), tol)
        {
            break;
        }
        ua = nua;
        va = nva;
        ub = nub;
        vb = nvb;
        let pt = a.eval(ua, va);
        out.push((pt, ua, va, ub, vb));
        if out.len() > 3 && pt.distance(start.0) <= tol * 20.0 {
            break;
        }
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn trace_from_seed(a: &Surface, dom_a: ((f64, f64), (f64, f64)), b: &Surface, dom_b: ((f64, f64), (f64, f64)), seed: SurfaceIntersectionSample, tol: f64) -> Option<Vec<SurfaceIntersectionSample>> {
    const MAX_STEPS: usize = 400;
    let fwd = march_direction(a, dom_a, b, dom_b, seed, tol, 1.0, MAX_STEPS);
    if fwd.len() >= 3 && fwd.last().unwrap().0.distance(seed.0) <= tol * 20.0 {
        let mut trace = vec![seed];
        trace.extend(fwd);
        return Some(trace);
    }
    let bwd = march_direction(a, dom_a, b, dom_b, seed, tol, -1.0, MAX_STEPS);
    let mut trace: Vec<_> = bwd.into_iter().rev().collect();
    trace.push(seed);
    trace.extend(fwd);
    if trace.len() < 2 {
        return None;
    }
    Some(trace)
}

/// 🏄 The fallback for every pair without a dedicated exact case: seed, march, fit. Each traced
/// branch becomes one `Fitted` [`IntCurve`] with an honestly measured `max_error` (the actual
/// deviation of the fitted curve/p-curves from the original traced samples).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn general_marching(a: &Surface, b: &Surface, tol: f64) -> Vec<IntCurve> {
    let dom_a = finite_domain(a);
    let dom_b = finite_domain(b);
    let seeds = find_seeds(a, dom_a, b, dom_b, tol);
    let mut traces: Vec<Vec<SurfaceIntersectionSample>> = Vec::new();
    let mut used: Vec<Pnt3> = Vec::new();
    for seed in seeds {
        if used.iter().any(|p: &Pnt3| p.distance(seed.0) <= tol * 20.0) {
            continue;
        }
        if let Some(trace) = trace_from_seed(a, dom_a, b, dom_b, seed, tol) {
            for s in &trace {
                used.push(s.0);
            }
            traces.push(trace);
        }
    }
    let mut out = Vec::new();
    for trace in traces {
        if trace.len() < 2 {
            continue;
        }
        let pts3: Vec<Pnt3> = trace.iter().map(|s| s.0).collect();
        let params = super::super::shared::centripetal_params(&pts3);
        let Some(nurbs) = super::super::shared::interpolate_params_3d(&pts3, &params) else { continue };
        let curve3 = Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights };
        let mut pts2a: Vec<Pnt2> = trace.iter().map(|s| Pnt2::new(s.1, s.2)).collect();
        let mut pts2b: Vec<Pnt2> = trace.iter().map(|s| Pnt2::new(s.3, s.4)).collect();
        unwrap_pts2(&mut pts2a, a);
        unwrap_pts2(&mut pts2b, b);
        let Some(pcurve_a) = super::super::shared::interpolate_params_2d(&pts2a, &params) else { continue };
        let Some(pcurve_b) = super::super::shared::interpolate_params_2d(&pts2b, &params) else { continue };
        let mut max_err = 0.0f64;
        for (i, &t) in params.iter().enumerate() {
            max_err = max_err.max(curve3.eval(t).distance(pts3[i]));
            let uv_a = pcurve_a.eval(t);
            max_err = max_err.max(a.eval(uv_a.x, uv_a.y).distance(pts3[i]));
            let uv_b = pcurve_b.eval(t);
            max_err = max_err.max(b.eval(uv_b.x, uv_b.y).distance(pts3[i]));
        }
        let (d0, d1) = curve3.domain();
        out.push(IntCurve { curve3, pcurve_a, pcurve_b, domain: ParamDomain { min: d0, max: d1 }, kind: IntCurveKind::Fitted { max_error: max_err } });
    }
    out
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn unwrap_pts2(pts: &mut [Pnt2], surface: &Surface) {
    if surface.is_u_periodic() {
        let mut us: Vec<f64> = pts.iter().map(|p| p.x).collect();
        super::super::shared::unwrap_periodic(&mut us, std::f64::consts::TAU);
        for (p, u) in pts.iter_mut().zip(us) {
            p.x = u;
        }
    }
    if surface.is_v_periodic() {
        let mut vs: Vec<f64> = pts.iter().map(|p| p.y).collect();
        super::super::shared::unwrap_periodic(&mut vs, std::f64::consts::TAU);
        for (p, v) in pts.iter_mut().zip(vs) {
            p.y = v;
        }
    }
}
// #endregion 🔖️General
