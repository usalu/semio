//! 🧭️ Surface algorithms that operate *on* a [`super::Surface`]: closest-point
//! projection (with closed-form fast paths for the surfaces that admit one) and Coons-patch
//! transfinite interpolation from four boundary curves. Kept separate from `surface.rs` for the
//! same reason as [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops`] versus [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve`].
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🪡️surface-ops` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `🏄️surface` per that file's own pre-mounted-stub note.

use super::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::{insert_knot, KnotVector};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Project

/// 🧭️ Certified closest-UV result: the parameters, the point they evaluate to, the distance to
/// the query point, and whether the result is a proven global optimum (`true` for every analytic
/// kind and for NURBS whose Bézier-patch convex hulls were fully pruned and refined).
#[derive(Clone, Debug, PartialEq)]
pub struct ClosestUv {
    pub u: f64,
    pub v: f64,
    pub point: Pnt3,
    pub distance: f64,
    pub certified: bool,
}

/// 🧭️ Closest point on `surface`, restricted to `domain`, to `target`, refined to within `tol`.
/// Exact closed forms for every analytic [`Surface`] kind (poles, apex and both torus periods
/// handled explicitly, each via [`Surface::eval`] on the *original* surface — no throwaway
/// reconstruction); [`Surface::Nurbs`] via Bézier-patch subdivision with convex-hull pruning
/// seeding a damped 2D Newton on the exact rational derivatives (via [`Surface::derivatives`], so
/// whichever derivative implementation is current is what Newton uses), falling back to a 1D
/// solve along the non-degenerate direction when the Jacobian is singular (poles/apex/seams).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn closest_uv(surface: &Surface, domain: ((f64, f64), (f64, f64)), target: Pnt3, tol: f64) -> ClosestUv {
    match surface {
        Surface::Plane { frame } => {
            let local = frame.to_local(target);
            let u = local.x.clamp((domain.0).0, (domain.0).1);
            let v = local.y.clamp((domain.1).0, (domain.1).1);
            eval_candidate(surface, u, v, target)
        }
        Surface::Sphere { frame, .. } => {
            let local = frame.to_local(target).to_vec();
            let n = local.normalized().unwrap_or(Vec3::Z);
            let u = wrap_periodic(n.y.atan2(n.x), domain.0);
            let v = n.z.clamp(-1.0, 1.0).asin().clamp((domain.1).0, (domain.1).1);
            eval_candidate(surface, u, v, target)
        }
        Surface::Cylinder { frame, .. } => {
            let local = frame.to_local(target);
            let radial = local.x.hypot(local.y);
            let raw_u = if radial <= f64::EPSILON { 0.0 } else { local.y.atan2(local.x) };
            let u = wrap_periodic(raw_u, domain.0);
            let v = local.z.clamp((domain.1).0, (domain.1).1);
            eval_candidate(surface, u, v, target)
        }
        Surface::Cone { frame, half_angle } => closest_on_cone(surface, frame, *half_angle, domain, target),
        Surface::Torus { frame, major_radius, .. } => closest_on_torus(surface, frame, *major_radius, domain, target),
        Surface::Nurbs { .. } => closest_on_nurbs_surface(surface, domain, target, tol),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_or_clamp(x: f64, lo: f64, hi: f64, periodic: bool) -> f64 {
    if periodic {
        let period = hi - lo;
        let mut w = (x - lo) % period;
        if w < 0.0 {
            w += period;
        }
        lo + w
    } else {
        x.clamp(lo, hi)
    }
}

/// 🧭️ Wraps an unconstrained periodic angle into `domain` (mirrors
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops`]'s identical fix for closed curves) — shared by every surface-of-revolution
/// direction (cylinder/cone/sphere `u`, torus `u` and `v`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_periodic(theta: f64, param_domain: (f64, f64)) -> f64 {
    let hi = if param_domain.1.is_finite() { param_domain.1 } else { param_domain.0 + std::f64::consts::TAU };
    wrap_or_clamp(theta, param_domain.0, hi, true)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn eval_candidate(surface: &Surface, u: f64, v: f64, target: Pnt3) -> ClosestUv {
    let p = surface.eval(u, v);
    ClosestUv { u, v, point: p, distance: p.distance(target), certified: true }
}

/// 🧭️ Cone: minimizing over `u` for fixed `v` is the same circle-projection as cylinder/sphere,
/// leaving `f(v) = (v·tanα - ρ0)² + (v - z0)²` — a plain upward parabola in `v`, minimized at the
/// closed form below (clamped to `domain`'s `v`-range, e.g. the apex when the unconstrained
/// minimum is negative).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_on_cone(surface: &Surface, frame: &Frame3, half_angle: f64, domain: ((f64, f64), (f64, f64)), target: Pnt3) -> ClosestUv {
    let local = frame.to_local(target);
    let rho0 = local.x.hypot(local.y);
    let tan_a = half_angle.tan();
    let raw_v = (tan_a * rho0 + local.z) / (tan_a * tan_a + 1.0);
    let v = raw_v.clamp((domain.1).0, (domain.1).1);
    let raw_u = if rho0 <= f64::EPSILON { 0.0 } else { local.y.atan2(local.x) };
    let u = wrap_periodic(raw_u, domain.0);
    eval_candidate(surface, u, v, target)
}

/// 🧭️ Torus: the closest point always lies in `target`'s own meridional half-plane (`u` = its
/// azimuth, exact for any surface of revolution), and within that plane the tube cross-section is
/// a plain circle of radius `minor_radius` centered at `(major_radius, 0)` — `v` is its exact
/// circle-closest-point angle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_on_torus(surface: &Surface, frame: &Frame3, major_radius: f64, domain: ((f64, f64), (f64, f64)), target: Pnt3) -> ClosestUv {
    let local = frame.to_local(target);
    let rho0 = local.x.hypot(local.y);
    let raw_u = if rho0 <= f64::EPSILON { 0.0 } else { local.y.atan2(local.x) };
    let u = wrap_periodic(raw_u, domain.0);
    let raw_v = local.z.atan2(rho0 - major_radius);
    let v = wrap_periodic(raw_v, domain.1);
    eval_candidate(surface, u, v, target)
}

// #endregion 🔖️Project

// #region 🧩️NurbsPatch

/// 🧩️ Inserts `t` into the u-direction of a rational surface control net (Boehm's algorithm
/// applied independently to every fixed-`v` "row", the standard tensor-product generalization of
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::insert_knot`]) — geometrically a no-op, used to raise a knot to full
/// multiplicity for Bézier-patch extraction.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn insert_u_knot_grid(u_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], t: f64) -> (KnotVector, Vec<Vec<Pnt3>>, Vec<Vec<f64>>) {
    let nu = controls.len();
    let nv = controls[0].len();
    let mut new_knots = u_knots.clone();
    let mut columns_p: Vec<Vec<Pnt3>> = Vec::with_capacity(nv);
    let mut columns_w: Vec<Vec<f64>> = Vec::with_capacity(nv);
    for j in 0..nv {
        let hx: Vec<f64> = (0..nu).map(|i| controls[i][j].x * weights[i][j]).collect();
        let hy: Vec<f64> = (0..nu).map(|i| controls[i][j].y * weights[i][j]).collect();
        let hz: Vec<f64> = (0..nu).map(|i| controls[i][j].z * weights[i][j]).collect();
        let hw: Vec<f64> = (0..nu).map(|i| weights[i][j]).collect();
        let (nk, nhx) = insert_knot(u_knots, &hx, t);
        let (_, nhy) = insert_knot(u_knots, &hy, t);
        let (_, nhz) = insert_knot(u_knots, &hz, t);
        let (_, nhw) = insert_knot(u_knots, &hw, t);
        new_knots = nk;
        let col_p: Vec<Pnt3> = (0..nhw.len()).map(|i| Pnt3::new(nhx[i] / nhw[i], nhy[i] / nhw[i], nhz[i] / nhw[i])).collect();
        columns_p.push(col_p);
        columns_w.push(nhw);
    }
    let new_nu = columns_p[0].len();
    let mut controls_out = vec![Vec::with_capacity(nv); new_nu];
    let mut weights_out = vec![Vec::with_capacity(nv); new_nu];
    for j in 0..nv {
        for i in 0..new_nu {
            controls_out[i].push(columns_p[j][i]);
            weights_out[i].push(columns_w[j][i]);
        }
    }
    (new_knots, controls_out, weights_out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transpose_grid_p(g: &[Vec<Pnt3>]) -> Vec<Vec<Pnt3>> {
    let nu = g.len();
    let nv = g[0].len();
    (0..nv).map(|j| (0..nu).map(|i| g[i][j]).collect()).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transpose_grid_w(g: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let nu = g.len();
    let nv = g[0].len();
    (0..nv).map(|j| (0..nu).map(|i| g[i][j]).collect()).collect()
}

/// 🧩️ [`insert_u_knot_grid`]'s v-direction twin, via transpose.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn insert_v_knot_grid(v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], t: f64) -> (KnotVector, Vec<Vec<Pnt3>>, Vec<Vec<f64>>) {
    let ct = transpose_grid_p(controls);
    let wt = transpose_grid_w(weights);
    let (nk, ct2, wt2) = insert_u_knot_grid(v_knots, &ct, &wt, t);
    (nk, transpose_grid_p(&ct2), transpose_grid_w(&wt2))
}

type Grid = (KnotVector, Vec<Vec<Pnt3>>, Vec<Vec<f64>>);

/// 🧩️ Splits a surface control grid at u-parameter `t` into two grids covering each side —
/// the exact 2D-grid analog of [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::split_nurbs`] (repeated knot insertion
/// to full multiplicity, then slicing the u-rows at the resulting breakpoint).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_grid_u(u_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], t: f64) -> (Grid, Grid) {
    let degree = u_knots.degree;
    let mut uk = u_knots.clone();
    let mut ctrl = controls.to_vec();
    let mut wts = weights.to_vec();
    let needed = degree + 1 - uk.multiplicity(t);
    for _ in 0..needed {
        let (nk, nc, nw) = insert_u_knot_grid(&uk, &ctrl, &wts, t);
        uk = nk;
        ctrl = nc;
        wts = nw;
    }
    let k = uk.find_span(t) - degree;
    let left_ctrl = ctrl[0..k].to_vec();
    let left_wts = wts[0..k].to_vec();
    let right_ctrl = ctrl[k..].to_vec();
    let right_wts = wts[k..].to_vec();
    let left_knot_count = left_ctrl.len() + degree + 1;
    let right_knot_count = right_ctrl.len() + degree + 1;
    let left_knots = uk.knots[0..left_knot_count].to_vec();
    let right_knots = uk.knots[uk.knots.len() - right_knot_count..].to_vec();
    ((KnotVector { knots: left_knots, degree }, left_ctrl, left_wts), (KnotVector { knots: right_knots, degree }, right_ctrl, right_wts))
}

/// 🧩️ [`split_grid_u`]'s v-direction twin, via transpose.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_grid_v(v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], t: f64) -> (Grid, Grid) {
    let ct = transpose_grid_p(controls);
    let wt = transpose_grid_w(weights);
    let ((lk, lc, lw), (rk, rc, rw)) = split_grid_u(v_knots, &ct, &wt, t);
    ((lk, transpose_grid_p(&lc), transpose_grid_w(&lw)), (rk, transpose_grid_p(&rc), transpose_grid_w(&rw)))
}

/// 🧩️ An axis-aligned box guaranteed to contain the patch (convex hull of a positive-weight
/// rational tensor-product Bézier patch's control net — the 2D analog of
/// [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bezier::RationalBezier3::control_hull_box`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hull_box(grid: &[Vec<Pnt3>]) -> (Pnt3, Pnt3) {
    let mut lo = grid[0][0];
    let mut hi = grid[0][0];
    for row in grid {
        for &p in row {
            lo = Pnt3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Pnt3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
    }
    (lo, hi)
}

/// 🧩️ Exact (certified) distance from `target` to the axis-aligned box `[lo, hi]` — `0` when
/// `target` is inside, otherwise the distance to the nearest clamped point on its boundary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_to_box_distance(target: Pnt3, lo: Pnt3, hi: Pnt3) -> f64 {
    let clamped = Pnt3::new(target.x.clamp(lo.x, hi.x), target.y.clamp(lo.y, hi.y), target.z.clamp(lo.z, hi.z));
    clamped.distance(target)
}

/// 🧩️ Decomposes a tensor-product NURBS surface into its Bézier patches (one per `(u, v)` knot
/// cell), each tagged with its exact `(u0, u1, v0, v1)` sub-domain and control-net hull box.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bezier_patches(u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>]) -> Vec<(f64, f64, f64, f64, (Pnt3, Pnt3))> {
    let (ulo, uhi) = u_knots.domain();
    let mut u_interior: Vec<f64> = u_knots.knots.iter().copied().filter(|&k| k > ulo + 1e-12 && k < uhi - 1e-12).collect();
    u_interior.sort_by(|a, b| a.partial_cmp(b).unwrap());
    u_interior.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    let mut u_strips: Vec<(f64, f64, KnotVector, Vec<Vec<Pnt3>>, Vec<Vec<f64>>)> = Vec::new();
    let mut remaining_c = controls.to_vec();
    let mut remaining_w = weights.to_vec();
    let mut remaining_k = u_knots.clone();
    let mut start = ulo;
    for &t in &u_interior {
        let ((_, lc, lw), (rk, rc, rw)) = split_grid_u(&remaining_k, &remaining_c, &remaining_w, t);
        u_strips.push((start, t, v_knots.clone(), lc, lw));
        remaining_k = rk;
        remaining_c = rc;
        remaining_w = rw;
        start = t;
    }
    u_strips.push((start, uhi, v_knots.clone(), remaining_c, remaining_w));

    let (vlo, vhi) = v_knots.domain();
    let mut v_interior: Vec<f64> = v_knots.knots.iter().copied().filter(|&k| k > vlo + 1e-12 && k < vhi - 1e-12).collect();
    v_interior.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v_interior.dedup_by(|a, b| (*a - *b).abs() < 1e-12);

    let mut patches = Vec::new();
    for (u0, u1, vk0, ctrl0, wts0) in u_strips {
        let mut rk = vk0;
        let mut rc = ctrl0;
        let mut rw = wts0;
        let mut vstart = vlo;
        for &t in &v_interior {
            let ((_, lc, _), (next_k, next_c, next_w)) = split_grid_v(&rk, &rc, &rw, t);
            patches.push((u0, u1, vstart, t, hull_box(&lc)));
            rk = next_k;
            rc = next_c;
            rw = next_w;
            vstart = t;
        }
        patches.push((u0, u1, vstart, vhi, hull_box(&rc)));
    }
    patches
}

// #endregion 🧩️NurbsPatch

// #region 🧭️NurbsClosestUv

/// 🧭️ 2D Newton on `f(u,v) = (S(u,v)-P)·Su = 0`, `(S(u,v)-P)·Sv = 0`, using [`Surface::derivatives`]
/// (exact rational derivatives when available, finite differences otherwise — this code is
/// derivative-implementation-agnostic by construction), with periodic wrap/clamp per direction. A
/// near-singular Jacobian (a pole or apex, where `Su`/`Sv` nearly vanish) falls back to a 1D
/// Newton step along whichever direction still has a nonzero second derivative.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newton_uv(surface: &Surface, target: Pnt3, mut u: f64, mut v: f64, domain: ((f64, f64), (f64, f64))) -> (f64, f64) {
    let (u_dom, v_dom) = domain;
    let u_periodic = surface.is_u_periodic();
    let v_periodic = surface.is_v_periodic();
    let u_hi = if u_dom.1.is_finite() { u_dom.1 } else { u_dom.0 + std::f64::consts::TAU };
    let v_hi = if v_dom.1.is_finite() { v_dom.1 } else { v_dom.0 + std::f64::consts::TAU };
    for _ in 0..30 {
        let d = surface.derivatives(u, v);
        let delta = d.point - target;
        let fu = delta.dot(d.du);
        let fv = delta.dot(d.dv);
        let fuu = d.du.dot(d.du) + delta.dot(d.duu);
        let fuv = d.du.dot(d.dv) + delta.dot(d.duv);
        let fvv = d.dv.dot(d.dv) + delta.dot(d.dvv);
        let det = fuu * fvv - fuv * fuv;
        let (step_u, step_v) = if det.abs() > 1e-9 {
            ((fu * fvv - fv * fuv) / det, (fv * fuu - fu * fuv) / det)
        } else if fuu.abs() > 1e-12 {
            (fu / fuu, 0.0)
        } else if fvv.abs() > 1e-12 {
            (0.0, fv / fvv)
        } else {
            break;
        };
        let next_u = wrap_or_clamp(u - step_u, u_dom.0, u_hi, u_periodic);
        let next_v = wrap_or_clamp(v - step_v, v_dom.0, v_hi, v_periodic);
        if (next_u - u).abs() < 1e-13 && (next_v - v).abs() < 1e-13 {
            u = next_u;
            v = next_v;
            break;
        }
        u = next_u;
        v = next_v;
    }
    (u, v)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn closest_on_nurbs_surface(surface: &Surface, domain: ((f64, f64), (f64, f64)), target: Pnt3, tol: f64) -> ClosestUv {
    let Surface::Nurbs { u_knots, v_knots, controls, weights } = surface else {
        unreachable!("closest_on_nurbs_surface called on a non-NURBS surface")
    };
    let mut patches: Vec<(f64, f64, f64, f64, f64)> = bezier_patches(u_knots, v_knots, controls, weights)
        .into_iter()
        .filter(|(u0, u1, v0, v1, _)| *u1 > (domain.0).0 - 1e-12 && *u0 < (domain.0).1 + 1e-12 && *v1 > (domain.1).0 - 1e-12 && *v0 < (domain.1).1 + 1e-12)
        .map(|(u0, u1, v0, v1, (lo, hi))| (u0, u1, v0, v1, point_to_box_distance(target, lo, hi)))
        .collect();
    patches.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap());
    let (u_dom, v_dom) = domain;
    let mut best = eval_candidate(surface, u_dom.0, v_dom.0, target);
    let corners = [(u_dom.0, v_dom.0), (u_dom.1, v_dom.0), (u_dom.0, v_dom.1), (u_dom.1, v_dom.1)];
    for &(cu, cv) in &corners {
        let cand = eval_candidate(surface, cu, cv, target);
        if cand.distance < best.distance {
            best = cand;
        }
    }
    for (u0, u1, v0, v1, lower_bound) in patches {
        if lower_bound > best.distance + tol {
            continue;
        }
        let seed_u = 0.5 * (u0.max(u_dom.0) + u1.min(u_dom.1));
        let seed_v = 0.5 * (v0.max(v_dom.0) + v1.min(v_dom.1));
        let (ru, rv) = newton_uv(surface, target, seed_u, seed_v, domain);
        let candidate = eval_candidate(surface, ru, rv, target);
        if candidate.distance < best.distance {
            best = candidate;
        }
    }
    best
}

// #endregion 🧭️NurbsClosestUv

// #region 🔖️Coons

/// 🧭️ Bilinear Coons-patch transfinite interpolation from four boundary curves parametrized on
/// `[0, 1]`: `c0`/`c1` are the `v=0`/`v=1` boundaries (functions of `u`), `d0`/`d1` are the `u=0`/
/// `u=1` boundaries (functions of `v`). Requires the four curves to agree at shared corners
/// (`c0(0)==d0(0)`, `c0(1)==d1(0)`, `c1(0)==d0(1)`, `c1(1)==d1(1)`) — the caller is responsible for
/// that consistency; this function does not check it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn coons_patch_eval(c0: &dyn Fn(f64) -> Pnt3, c1: &dyn Fn(f64) -> Pnt3, d0: &dyn Fn(f64) -> Pnt3, d1: &dyn Fn(f64) -> Pnt3, u: f64, v: f64) -> Pnt3 {
    let p00 = c0(0.0);
    let p10 = c0(1.0);
    let p01 = c1(0.0);
    let p11 = c1(1.0);
    let ruled_uv = c0(u).to_vec() * (1.0 - v) + c1(u).to_vec() * v;
    let ruled_vu = d0(v).to_vec() * (1.0 - u) + d1(v).to_vec() * u;
    let bilinear_corners = (p00.to_vec() * (1.0 - u) * (1.0 - v) + p10.to_vec() * u * (1.0 - v) + p01.to_vec() * (1.0 - u) * v + p11.to_vec() * u * v).to_array();
    Pnt3::from_array((ruled_uv + ruled_vu).to_array()) - Vec3::from_array(bilinear_corners)
}

// #endregion 🔖️Coons

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_plane_matches_orthogonal_projection() {
        let frame = Frame3::WORLD;
        let s = Surface::Plane { frame };
        let target = Pnt3::new(2.0, 3.0, 5.0);
        let cp = closest_uv(&s, s.domain(), target, 1e-9);
        assert!(cp.certified);
        assert!((cp.u - 2.0).abs() < 1e-9);
        assert!((cp.v - 3.0).abs() < 1e-9);
        assert!((cp.distance - 5.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_sphere_matches_radial_projection() {
        let frame = Frame3::from_normal(Pnt3::new(1.0, 1.0, 1.0), Vec3::Z).unwrap();
        let s = Surface::Sphere { frame, radius: 3.0 };
        let target = Pnt3::new(1.0, 1.0, 21.0); // 20 units above the sphere along its axis
        let cp = closest_uv(&s, s.domain(), target, 1e-9);
        assert!((cp.distance - 17.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_sphere_pole_is_certified_and_exact() {
        let frame = Frame3::WORLD;
        let s = Surface::Sphere { frame, radius: 2.0 };
        let target = Pnt3::new(0.0, 0.0, 10.0); // directly above the north pole
        let cp = closest_uv(&s, s.domain(), target, 1e-9);
        assert!(cp.certified);
        assert!((cp.distance - 8.0).abs() < 1e-6);
        assert!(cp.point.distance(Pnt3::new(0.0, 0.0, 2.0)) < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_cylinder_matches_expected_geometry() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cylinder { frame, radius: 2.0 };
        let target = Pnt3::new(10.0, 0.0, 5.0);
        let cp = closest_uv(&s, ((0.0, std::f64::consts::TAU), (0.0, 10.0)), target, 1e-9);
        assert!((cp.distance - 8.0).abs() < 1e-5, "distance mismatch: {}", cp.distance);
        assert!(cp.point.distance(Pnt3::new(2.0, 0.0, 5.0)) < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_cone_apex_when_unconstrained_minimum_is_negative() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cone { frame, half_angle: std::f64::consts::FRAC_PI_6 };
        // Straight down from the apex: the unconstrained v* is negative, so the true closest
        // point on the (v >= 0) cone is the apex itself.
        let target = Pnt3::new(0.0, 0.0, -5.0);
        let cp = closest_uv(&s, s.domain(), target, 1e-9);
        assert!(cp.certified);
        assert!((cp.distance - 5.0).abs() < 1e-6);
        assert!(cp.point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_torus_matches_meridional_projection() {
        let frame = Frame3::WORLD;
        let s = Surface::Torus { frame, major_radius: 5.0, minor_radius: 1.0 };
        let target = Pnt3::new(5.0, 0.0, 5.0); // straight up from the tube center at u=0
        let cp = closest_uv(&s, s.domain(), target, 1e-9);
        assert!(cp.certified);
        assert!((cp.distance - 4.0).abs() < 1e-6, "distance mismatch: {}", cp.distance);
        assert!(cp.point.distance(Pnt3::new(5.0, 0.0, 1.0)) < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn closest_point_on_cylinder_seam_wraps_correctly() {
        let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        let s = Surface::Cylinder { frame, radius: 1.0 };
        let angle = -0.01_f64;
        let target = Pnt3::new(2.0 * angle.cos(), 2.0 * angle.sin(), 0.0);
        let cp = closest_uv(&s, ((0.0, std::f64::consts::TAU), (-5.0, 5.0)), target, 1e-9);
        let expected = std::f64::consts::TAU + angle;
        assert!((cp.u - expected).abs() < 1e-6, "seam wrap failed: u={}, expected near {expected}", cp.u);
    }

    #[semio_framework_async_macros::async_test]
    async fn coons_patch_reproduces_boundary_curves_exactly() {
        let c0 = |u: f64| Pnt3::new(u, 0.0, 0.0);
        let c1 = |u: f64| Pnt3::new(u, 1.0, u * u);
        let d0 = |v: f64| Pnt3::new(0.0, v, 0.0);
        let d1 = |v: f64| Pnt3::new(1.0, v, v);
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, t, 0.0).distance(c0(t)) < 1e-9, "v=0 boundary mismatch at u={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, t, 1.0).distance(c1(t)) < 1e-9, "v=1 boundary mismatch at u={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, 0.0, t).distance(d0(t)) < 1e-9, "u=0 boundary mismatch at v={t}");
            assert!(coons_patch_eval(&c0, &c1, &d0, &d1, 1.0, t).distance(d1(t)) < 1e-9, "u=1 boundary mismatch at v={t}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn coons_patch_of_planar_boundaries_is_the_bilinear_plane() {
        let c0 = |u: f64| Pnt3::new(u, 0.0, 0.0);
        let c1 = |u: f64| Pnt3::new(u, 1.0, 0.0);
        let d0 = |v: f64| Pnt3::new(0.0, v, 0.0);
        let d1 = |v: f64| Pnt3::new(1.0, v, 0.0);
        let p = coons_patch_eval(&c0, &c1, &d0, &d1, 0.3, 0.7);
        assert!(p.distance(Pnt3::new(0.3, 0.7, 0.0)) < 1e-9);
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn closest_point_on_cylinder_matches_brute_force_grid_oracle() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(71);
            for _ in 0..50 {
                let frame =
                    Frame3::from_normal(Pnt3::new(rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0, rng.next_f64() * 4.0 - 2.0), Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5).normalized().unwrap_or(Vec3::Z))
                        .unwrap();
                let radius = 0.5 + rng.next_f64() * 5.0;
                let s = Surface::Cylinder { frame, radius };
                let target = Pnt3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let domain = ((0.0, std::f64::consts::TAU), (-15.0, 15.0));
                let cp = closest_uv(&s, domain, target, 1e-9);
                let mut oracle = f64::INFINITY;
                for i in 0..2000 {
                    let u = std::f64::consts::TAU * i as f64 / 2000.0;
                    for j in 0..200 {
                        let v = -15.0 + 30.0 * j as f64 / 200.0;
                        oracle = oracle.min(s.eval(u, v).distance(target));
                    }
                }
                assert!(cp.distance <= oracle + 1e-3, "closed-form found {} worse than oracle {oracle}", cp.distance);
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn closest_point_on_nurbs_patch_matches_dense_sampling_oracle() {
            let frame = Frame3::WORLD;
            let cyl = Surface::Cylinder { frame, radius: 2.5 };
            // A NURBS surface built by densely sampling a cylinder patch is an independent,
            // easily oracled shape for the patch-subdivision + Newton path.
            let u_knots = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector::clamped_uniform(5, 3);
            let v_knots = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector::clamped_uniform(4, 3);
            let controls: Vec<Vec<Pnt3>> = (0..5)
                .map(|i| {
                    let u = std::f64::consts::PI * i as f64 / 4.0;
                    (0..4).map(|j| cyl.eval(u, j as f64 * 2.0)).collect()
                })
                .collect();
            let weights = vec![vec![1.0; 4]; 5];
            let nurbs = Surface::Nurbs { u_knots, v_knots, controls, weights };
            let mut rng = semio_framework_geometry::random::Rng::from_seed(83);
            for _ in 0..20 {
                let target = Pnt3::new(rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 6.0);
                let cp = closest_uv(&nurbs, nurbs.domain(), target, 1e-9);
                let (u_dom, v_dom) = nurbs.domain();
                let mut oracle = f64::INFINITY;
                for i in 0..400 {
                    let u = u_dom.0 + (u_dom.1 - u_dom.0) * i as f64 / 400.0;
                    for j in 0..400 {
                        let v = v_dom.0 + (v_dom.1 - v_dom.0) * j as f64 / 400.0;
                        oracle = oracle.min(nurbs.eval(u, v).distance(target));
                    }
                }
                assert!(cp.distance <= oracle + 1e-2, "certified={} worse than oracle={oracle}", cp.distance);
            }
        }
    }
}
// #endregion 🔖️Tests
