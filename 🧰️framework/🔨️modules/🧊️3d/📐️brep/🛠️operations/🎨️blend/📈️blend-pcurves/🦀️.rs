//! 📈️ Blend p-curves: the closed-form affine, planar, blend and interpolated parameter curves with their deviation check and the surface and curve kind readers. See [`super`].

use super::*;

// #region 🔖️Pcurve
/// 🎨️ The exact p-curve of a curve whose `(u, v)` image is a straight, AFFINELY parametrized line
/// — which nearly every boundary this file mints is: an iso-line of its own analytic patch (a
/// cylinder's ruling and its cross circle, a torus's tube arc and azimuth circle, a sphere's
/// meridian and equator, a cone's ruling) or any straight line on a plane. Both ends plus interior
/// probes (which is what a full-circle boundary's periodic unwrapping needs) are inverted through
/// the certified analytic [`closest_uv`], the affine map through them is formed, and the candidate
/// is VERIFIED against the real curve — so a boundary whose parameter is not affine in the
/// surface's (a circle on a plane, whose image is a conic) reports `None` instead of quietly
/// returning a wrong answer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn affine_uv_pcurve(surface: &Surface, curve: &Curve3, range: (f64, f64)) -> Option<Curve2> {
    let span = range.1 - range.0;
    if span.abs() <= 1e-15 {
        return None;
    }
    let probes = 8usize;
    let mut uvs: Vec<(f64, f64, bool)> = Vec::with_capacity(probes + 1);
    for i in 0..=probes {
        let t = range.0 + span * i as f64 / probes as f64;
        let raw = closest_uv(surface, surface.domain(), curve.eval(t), BLEND_EPS);
        uvs.push((raw.u, raw.v, surface.is_degenerate_uv(raw.u, raw.v)));
    }
    // 🧭️ A pole's `u` is meaningless (its inversion returns an arbitrary branch), and a boundary
    // ENDING at one — every meridian of a spherical corner does — would otherwise be fitted through
    // that arbitrary value and come out slanted. Carry `u` in from the nearest sample that has one,
    // in both directions, before anything is unwrapped or fitted.
    let first_good = uvs.iter().position(|entry| !entry.2)?;
    for i in (0..first_good).rev() {
        uvs[i].0 = uvs[i + 1].0;
    }
    for i in first_good + 1..uvs.len() {
        if uvs[i].2 {
            uvs[i].0 = uvs[i - 1].0;
        }
    }
    for i in 1..uvs.len() {
        let (pu, pv) = (uvs[i - 1].0, uvs[i - 1].1);
        if surface.is_u_periodic() {
            uvs[i].0 = unwrap_near(uvs[i].0, pu);
        }
        if surface.is_v_periodic() {
            uvs[i].1 = unwrap_near(uvs[i].1, pv);
        }
    }
    let (u0, v0, _) = uvs[0];
    let (u1, v1, _) = uvs[probes];
    let du = (u1 - u0) / span;
    let dv = (v1 - v0) / span;
    let candidate = Curve2::Line { origin: Pnt2::new(u0 - du * range.0, v0 - dv * range.0), dir: Vec2::new(du, dv) };
    if pcurve_deviation(surface, &candidate, curve, range) <= BLEND_EPS {
        Some(candidate)
    } else {
        None
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pcurve_deviation(surface: &Surface, pcurve: &Curve2, curve: &Curve3, range: (f64, f64)) -> f64 {
    let span = range.1 - range.0;
    (0..=32)
        .map(|i| {
            let t = range.0 + span * f64::from(i) / 32.0;
            let uv = pcurve.eval(t);
            surface.eval(uv.x, uv.y).distance(curve.eval(t))
        })
        .fold(0.0f64, f64::max)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_near(x: f64, near: f64) -> f64 {
    let mut y = x;
    while y - near > PI {
        y -= TAU;
    }
    while near - y > PI {
        y += TAU;
    }
    y
}

/// 🎨️ A curve that LIES IN a plane projects into that plane's `(u, v)` exactly, parameter for
/// parameter: a line maps to a line for any orientation, and a rational B-spline maps to the same
/// spline over its own control points' plane coordinates (every point of a rational curve is a
/// convex-weighted combination of its controls, so coplanar controls give a coplanar curve and the
/// projection commutes with the evaluation). Conics with an independent frame are left to
/// [`exact_pcurve`]'s own signed-radius ellipse shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn planar_pcurve(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    let Surface::Plane { frame } = surface else { return None };
    match curve {
        Curve3::Line { origin, dir } => {
            let local = frame.to_local(*origin);
            let direction = frame.to_local_vector(*dir);
            Some(Curve2::Line { origin: Pnt2::new(local.x, local.y), dir: Vec2::new(direction.x, direction.y) })
        }
        Curve3::Nurbs { knots, controls, weights } => {
            let mut planar = Vec::with_capacity(controls.len());
            for control in controls {
                let local = frame.to_local(*control);
                if local.z.abs() > 1e-9 {
                    return None;
                }
                planar.push(Pnt2::new(local.x, local.y));
            }
            Some(Curve2::Nurbs { knots: knots.clone(), controls: planar, weights: weights.clone() })
        }
        _ => None,
    }
}

/// 🎨️ Every p-curve this file stores. [`affine_uv_pcurve`] and [`planar_pcurve`] answer in closed
/// form; [`exact_pcurve`] adds the conic-on-a-plane shortcut; only then — and only for a curve no
/// closed form covers, which in practice is a variable fillet's elliptical end trim on its own cone
/// — is `Surface::project_curve`'s numeric fit used, and its result is CERTIFIED against the real
/// curve before it is accepted. A pair no route can answer is refused, never approximated.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn blend_pcurve(surface: &Surface, curve: &Curve3, range: (f64, f64)) -> Result<Curve2, KernelError> {
    if let Some(pcurve) = affine_uv_pcurve(surface, curve, range) {
        return Ok(pcurve);
    }
    if let Some(pcurve) = planar_pcurve(surface, curve) {
        return Ok(pcurve);
    }
    if let Some(pcurve) = exact_pcurve(surface, curve, range) {
        if pcurve_deviation(surface, &pcurve, curve, range) <= BLEND_EPS {
            return Ok(pcurve);
        }
    }
    // 🧭️ The ONE interpolated p-curve in this engine, deliberately reachable only here: a variable
    // fillet's cap plane cuts its cone in a conic whose `(u, v)` image is transcendental in the
    // conic's own parameter, so no closed form exists. It is affordable exactly because the support
    // is analytic — its inversion is a closed form, unlike the NURBS patches whose sampled fits
    // this rewrite exists to eliminate.
    if !matches!((surface, curve), (Surface::Cone { .. }, Curve3::Nurbs { .. })) {
        return Err(KernelError::Operation(format!("blend: no closed-form p-curve exists for {} / {}", surface_kind(surface), curve_kind(curve))));
    }
    if let Some(interpolated) = interpolated_uv_pcurve(surface, curve, range) {
        return Ok(interpolated);
    }
    Err(KernelError::Operation(format!("blend: no p-curve within tolerance exists for {} / {}", surface_kind(surface), curve_kind(curve))))
}

/// 🎨️ A p-curve interpolated through exactly inverted `(u, v)` samples, parametrized by the 3D
/// curve's OWN parameter — which is what the same-parameter law requires and what
/// `Surface::project_curve`'s fit cannot give (it re-parametrizes centripetally, so its result is
/// only ever valid for a caller that re-parametrizes with it). Densified until the deviation
/// certifies, so the answer is exact to `1e-10` in practice and refused outright above `1e-8`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn interpolated_uv_pcurve(surface: &Surface, curve: &Curve3, range: (f64, f64)) -> Option<Curve2> {
    let span = range.1 - range.0;
    let mut best: Option<Curve2> = None;
    for count in [16usize, 32, 64, 128, 256] {
        let mut samples: Vec<Pnt3> = Vec::with_capacity(count + 1);
        let mut previous: Option<(f64, f64)> = None;
        for i in 0..=count {
            let t = range.0 + span * i as f64 / count as f64;
            let raw = closest_uv(surface, surface.domain(), curve.eval(t), BLEND_EPS);
            let (mut u, mut v) = (raw.u, raw.v);
            if let Some((pu, pv)) = previous {
                if surface.is_u_periodic() {
                    u = unwrap_near(u, pu);
                }
                if surface.is_v_periodic() {
                    v = unwrap_near(v, pv);
                }
            }
            previous = Some((u, v));
            samples.push(Pnt3::new(u, v, 0.0));
        }
        let fitted = interpolate_curve(&samples, 3, ParamMethod::Uniform, None, false)?;
        let knots = KnotVector::new(fitted.knots.knots.iter().map(|k| range.0 + span * k).collect(), fitted.knots.degree, fitted.controls.len())?;
        let candidate = Curve2::Nurbs { knots, controls: fitted.controls.iter().map(|p| Pnt2::new(p.x, p.y)).collect(), weights: fitted.weights };
        let deviation = pcurve_deviation(surface, &candidate, curve, range);
        if deviation <= BLEND_EPS {
            return Some(candidate);
        }
        if deviation <= FIT_EPS {
            best = Some(candidate);
        }
    }
    best
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surface_kind(surface: &Surface) -> &'static str {
    match surface {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Nurbs { .. } => "nurbs surface",
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn curve_kind(curve: &Curve3) -> &'static str {
    match curve {
        Curve3::Line { .. } => "line",
        Curve3::Circle { .. } => "circle",
        Curve3::Ellipse { .. } => "ellipse",
        Curve3::Nurbs { .. } => "nurbs curve",
    }
}
// #endregion 🔖️Pcurve
