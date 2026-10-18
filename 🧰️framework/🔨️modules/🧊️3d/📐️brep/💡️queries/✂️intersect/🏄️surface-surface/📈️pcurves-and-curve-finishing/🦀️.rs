//! 📈️ P-curves and curve finishing: the closed-form and sampled p-curves on each support, their error measurement and the finishing of an intersection curve with its finite domain. See [`super`].

use super::*;

// #region 🔖️PCurve
/// 🏄 Builds `curve3`'s p-curve on `surface`: closed-form wherever the geometry admits one,
/// certified-fit sampling otherwise. Returns `(pcurve, is_exact)`.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_pcurve(surface: &Surface, curve3: &Curve3, tol: f64) -> (Curve2, bool) {
    match (surface, curve3) {
        (Surface::Plane { frame }, Curve3::Line { origin, dir }) => {
            let o = frame.to_local(*origin);
            let d = frame.to_local_vector(*dir);
            (Curve2::Line { origin: Pnt2::new(o.x, o.y), dir: Vec2::new(d.x, d.y) }, true)
        }
        (Surface::Plane { frame }, Curve3::Circle { frame: cf, radius }) => {
            let center = frame.to_local(cf.origin);
            let x_local = frame.to_local_vector(cf.x);
            (Curve2::Ellipse { center: Pnt2::new(center.x, center.y), x_axis: Vec2::new(x_local.x, x_local.y), major_radius: *radius, minor_radius: planar_pcurve_sense(frame, cf) * *radius }, true)
        }
        (Surface::Plane { frame }, Curve3::Ellipse { frame: cf, major_radius, minor_radius }) => {
            let center = frame.to_local(cf.origin);
            let x_local = frame.to_local_vector(cf.x);
            (Curve2::Ellipse { center: Pnt2::new(center.x, center.y), x_axis: Vec2::new(x_local.x, x_local.y), major_radius: *major_radius, minor_radius: planar_pcurve_sense(frame, cf) * *minor_radius }, true)
        }
        (Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. }, Curve3::Line { origin, dir }) => {
            if let (Some((_, axis_dir)), Some(unit)) = (super::super::shared::axis_of(surface), dir.normalized()) {
                if unit.cross(axis_dir).norm() <= tol.max(1e-12) {
                    return (linear_pcurve_on_axisymmetric(surface, *origin, unit), true);
                }
            }
            sample_and_fit_pcurve(surface, curve3, tol)
        }
        (Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. }, Curve3::Circle { frame: cf, .. } | Curve3::Ellipse { frame: cf, .. }) => {
            if let Some((_, axis_dir)) = super::super::shared::axis_of(surface) {
                if let Some(n) = cf.z.normalized() {
                    if n.cross(axis_dir).norm() <= tol.max(1e-12) {
                        let (u0, v0) = super::super::shared::exact_uv(surface, curve3.eval(0.0));
                        let sign = if n.dot(axis_dir) >= 0.0 { 1.0 } else { -1.0 };
                        return (Curve2::Line { origin: Pnt2::new(u0, v0), dir: Vec2::new(sign, 0.0) }, true);
                    }
                    if matches!(surface, Surface::Torus { .. }) && n.dot(axis_dir).abs() <= tol.max(1e-9) {
                        return (meridian_pcurve_on_torus(surface, curve3), true);
                    }
                }
            }
            sample_and_fit_pcurve(surface, curve3, tol)
        }
        _ => sample_and_fit_pcurve(surface, curve3, tol),
    }
}

/// 🏄 `+1` or `−1`: whether a conic's own `(x, y)` frame maps onto the plane's local frame with
/// the same handedness [`Curve2::Ellipse::eval`] assumes.
///
/// That evaluator derives its second axis as `x_axis.perp()` — a FIXED right-handed quarter turn in
/// the plane's own local frame. When the conic's normal runs ANTI-parallel to the plane's (a
/// cylinder's intersection circle against a box's bottom face, whose outward normal points the
/// other way), the true second axis is the other one, and the p-curve traverses the conic backwards:
/// every parameter maps to the ANTIPODE, `2r` away, which `validate_body`'s same-parameter check
/// duly reported on every box-through-cylinder boolean. Negating `minor_radius` flips exactly that
/// term — exact for either handedness, no fit — the same signed-radius convention
/// `diff::offset::exact_pcurve_for_circle_on_plane` already documents.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn planar_pcurve_sense(plane: &Frame3, conic: &Frame3) -> f64 {
    let x = plane.to_local_vector(conic.x);
    let y = plane.to_local_vector(conic.y);
    if x.x * y.y - x.y * y.x >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

/// 🏄 The p-curve of a line lying exactly on an axisymmetric surface with `dir ∥` that surface's
/// axis (a ruling): `(u, v)` is affine in the line's own parameter, determined exactly by two
/// samples (no fitting — the relation genuinely is affine for a ruling).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn linear_pcurve_on_axisymmetric(surface: &Surface, origin: Pnt3, dir: Vec3) -> Curve2 {
    let (u0, v0) = super::super::shared::exact_uv(surface, origin);
    let (u1, v1) = super::super::shared::exact_uv(surface, origin + dir);
    let mut du = u1 - u0;
    if du > std::f64::consts::PI {
        du -= std::f64::consts::TAU;
    }
    if du < -std::f64::consts::PI {
        du += std::f64::consts::TAU;
    }
    Curve2::Line { origin: Pnt2::new(u0, v0), dir: Vec2::new(du, v1 - v0) }
}

/// 🏄 The p-curve of a [`Curve3::Circle`]/[`Curve3::Ellipse`] lying exactly in a
/// [`Surface::Torus`]'s own axis-containing (meridian) plane: any curve on the torus confined to
/// such a plane is one of the two meridian tube circles (a torus's cross-section by any
/// axis-containing plane is always exactly that pair — same argument [`plane_torus`]'s own
/// axis-containing branch already relies on), so `(u, v)` is affine in the curve's own parameter
/// (`u` constant, `v` linear) exactly like [`linear_pcurve_on_axisymmetric`]'s ruling case — just
/// derived from two samples spaced `1e-4` rad apart rather than a full unit step, since `v`'s
/// slope is ±1 over the full `[0, 2π)` domain and a unit-step sample could itself wrap.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn meridian_pcurve_on_torus(surface: &Surface, curve3: &Curve3) -> Curve2 {
    let eps = 1e-4;
    let (u0, v0) = super::super::shared::exact_uv(surface, curve3.eval(0.0));
    let (u1, v1) = super::super::shared::exact_uv(surface, curve3.eval(eps));
    let unwrap = |mut d: f64| {
        if d > std::f64::consts::PI {
            d -= std::f64::consts::TAU;
        }
        if d < -std::f64::consts::PI {
            d += std::f64::consts::TAU;
        }
        d
    };
    let du = unwrap(u1 - u0) / eps;
    let dv = unwrap(v1 - v0) / eps;
    Curve2::Line { origin: Pnt2::new(u0, v0), dir: Vec2::new(du, dv) }
}

/// 🏄 Samples `curve3` across its natural domain, inverts each sample exactly via
/// [`super::shared::exact_uv`], unwraps periodic directions, and globally interpolates — a
/// certified-error NURBS p-curve for the pairs `build_pcurve` has no closed form for.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_and_fit_pcurve(surface: &Surface, curve3: &Curve3, _tol: f64) -> (Curve2, bool) {
    let domain = curve3.domain();
    let n = 33usize;
    let mut params = Vec::with_capacity(n);
    let mut pts2 = Vec::with_capacity(n);
    for i in 0..n {
        let t = domain.0 + (domain.1 - domain.0) * (i as f64 / (n - 1) as f64);
        params.push(t);
        let (u, v) = super::super::shared::exact_uv(surface, curve3.eval(t));
        pts2.push(Pnt2::new(u, v));
    }
    if surface.is_u_periodic() {
        let mut us: Vec<f64> = pts2.iter().map(|p| p.x).collect();
        super::super::shared::unwrap_periodic(&mut us, std::f64::consts::TAU);
        for (p, u) in pts2.iter_mut().zip(us) {
            p.x = u;
        }
    }
    if surface.is_v_periodic() {
        let mut vs: Vec<f64> = pts2.iter().map(|p| p.y).collect();
        super::super::shared::unwrap_periodic(&mut vs, std::f64::consts::TAU);
        for (p, v) in pts2.iter_mut().zip(vs) {
            p.y = v;
        }
    }
    let curve2 = super::super::shared::interpolate_params_2d(&pts2, &params).unwrap_or(Curve2::Line { origin: pts2[0], dir: Vec2::ZERO });
    (curve2, false)
}

/// 🏄 Densely resamples `curve3` and compares each `pcurve.eval(t)` (mapped through `surface`)
/// against `curve3.eval(t)` — the certified error a `Fitted` [`IntCurveKind`] reports.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn measure_pcurve_error(curve3: &Curve3, surface: &Surface, pcurve: &Curve2) -> f64 {
    let (t0, t1) = curve3.domain();
    if !(t0.is_finite() && t1.is_finite()) {
        return 0.0;
    }
    let mut max_e = 0.0f64;
    for i in 0..=16 {
        let t = t0 + (t1 - t0) * (i as f64 / 16.0);
        let uv = pcurve.eval(t);
        max_e = max_e.max(surface.eval(uv.x, uv.y).distance(curve3.eval(t)));
    }
    max_e
}

/// 🏄 Builds both p-curves for `curve3` against `(surf_a, surf_b)` and assembles the finished
/// [`IntCurve`], with `kind` reflecting whether either side needed a fit.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(super) fn finish_intcurve(curve3: Curve3, surf_a: &Surface, surf_b: &Surface, tol: f64) -> IntCurve {
    let (pcurve_a, exact_a) = build_pcurve(surf_a, &curve3, tol);
    let (pcurve_b, exact_b) = build_pcurve(surf_b, &curve3, tol);
    let kind = if exact_a && exact_b {
        IntCurveKind::Exact
    } else {
        let ea = if exact_a { 0.0 } else { measure_pcurve_error(&curve3, surf_a, &pcurve_a) };
        let eb = if exact_b { 0.0 } else { measure_pcurve_error(&curve3, surf_b, &pcurve_b) };
        IntCurveKind::Fitted { max_error: ea.max(eb) }
    };
    let (d0, d1) = finite_curve3_domain(&curve3, surf_a, surf_b, tol);
    IntCurve { curve3, pcurve_a, pcurve_b, domain: ParamDomain { min: d0, max: d1 }, kind }
}

/// 🏄 `curve3`'s own natural domain, unless it's infinite — [`Curve3::Line`]'s is always `(-∞,
/// ∞)` (both supports here are themselves infinite/unbounded surfaces: two planes, a plane and an
/// infinite cylinder, two parallel infinite cylinders), which previously flowed straight into
/// [`IntCurve::domain`] unmodified: every consumer that samples across that domain (this file's
/// own `assert_on_both`, any downstream trim) computed `t0 + (t1 - t0) * frac` on `±∞`, i.e. `NaN`
/// at every sample. Bounded the same way `curve_surface::curve_sample_domain` already bounds an
/// infinite line against a single surface (`shared::line_domain_against_surface`) — here against
/// BOTH supports, unioned, since the line lies on both.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn finite_curve3_domain(curve3: &Curve3, surf_a: &Surface, surf_b: &Surface, tol: f64) -> (f64, f64) {
    let (d0, d1) = curve3.domain();
    if d0.is_finite() && d1.is_finite() {
        return (d0, d1);
    }
    let Curve3::Line { origin, dir } = curve3 else { return (d0, d1) };
    let bound_a = super::super::shared::line_domain_against_surface(origin, dir, surf_a, tol);
    let bound_b = super::super::shared::line_domain_against_surface(origin, dir, surf_b, tol);
    match (bound_a, bound_b) {
        (Ok(a), Ok(b)) => (a.0.min(b.0), a.1.max(b.1)),
        (Ok(a), Err(_)) => a,
        (Err(_), Ok(b)) => b,
        (Err(_), Err(_)) => (-10.0, 10.0),
    }
}
// #endregion 🔖️PCurve
