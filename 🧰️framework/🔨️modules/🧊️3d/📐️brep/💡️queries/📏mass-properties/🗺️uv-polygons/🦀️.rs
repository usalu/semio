//! 🗺️ UV polygons: the face loop UV polygons with their coedge sampling, the chord deviation segment count, the signed area and the point in polygon test. See [`super`].

use super::*;

// #region 🔖️Loops
/// 📏 UV boundary polygon for a (possibly curved-edge) loop — samples EACH coedge at a
/// curvature-adaptive point count (not just its start vertex: W1-E's flagged gap, a straight-edge
/// polygon is exact with one sample per coedge, but any curved-edge loop with few coedges — every
/// lateral/cap/spherical/toroidal face `🧱️primitives/🦀️.rs` builds — degenerated to a 1-4 point
/// "polygon" and made `point_in_uv_polygon`/the ear-clip quadrature under/over-integrate).
/// 🐛 FIX: an earlier pass bumped this from 1 sample/coedge to a FIXED `EDGE_SAMPLES = 8`, which
/// fixed the degenerate-polygon shape but is still tolerance-blind — a `chord_tol = 1e-3` caller
/// (`solid_volume`) got an 8-gon inscribed circle whose area already undershoots by ~10% (n=8's
/// sagitta is a full 7.6% of a r=1.5 circle), regardless of how tight the caller's tolerance is.
/// The interior quadrature (`integrate_triangle_adaptive`) genuinely refines to `chord_tol`, but
/// only ever refines WITHIN the ear-clipped boundary's fixed straight-chord approximation, so a
/// coarse boundary chord caps accuracy no matter how deep the interior recursion goes. Now derives
/// `n` per-coedge from `chord_tol` via the same closed-form chordal-deviation formula tessellation
/// uses for edge sampling (`segments_for_chord_deviation`, kept as a separate small copy here per
/// doctrine — different call shape, `+1` for point count vs segment count, no angular-tol term
/// since this feeds a quadrature boundary, not a rendered mesh's normal quality).
/// Prefers the coedge's own p-curve (`Coedge::pcurve`/`prange`, mirroring classification.rs's
/// `coedge_uv_sample` pattern — kept close per doctrine rather than cross-imported) over
/// reprojecting the 3D curve, and reads it in the coedge's OWN order regardless of `forward`,
/// reversing only when `forward == false` (W1-E's binding convention, fixed alongside this pass —
/// see [`coedge_uv_sample`]'s own doc); the 3D-curve fallback already reverses via `t`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn loop_uv_polygon(body: &Body, loop_id: semio_framework_3d_brep_representation::arena::LoopId, surface: &Surface, chord_tol: f64) -> Result<Vec<Pnt2>, KernelError> {
    loop_uv_polygon_with(body, loop_id, surface, chord_tol, false)
}

/// 📏 [`loop_uv_polygon`] with a choice of boundary sampling: `coarse_pcurves` samples a straight UV p-curve
/// once per `π/4` of `u` instead of by the 3D chord deviation of its curve, which is exact (a straight UV
/// segment needs no more) and keeps the mass quadrature's triangulation free of collinear slivers.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn loop_uv_polygon_with(body: &Body, loop_id: semio_framework_3d_brep_representation::arena::LoopId, surface: &Surface, chord_tol: f64, coarse_pcurves: bool) -> Result<Vec<Pnt2>, KernelError> {
    let mut poly: Vec<Pnt2> = Vec::new();
    let mut prev_u: Option<f64> = None;
    let mut prev_was_pole = false;
    let coedges = body.loop_coedges(loop_id);
    for (ci, coedge_id) in coedges.iter().enumerate() {
        let co = body.coedges.get(*coedge_id).ok_or_else(|| KernelError::MissingEntity("coedge".into()))?;
        let n = coedge_sample_count(body, co, chord_tol, coarse_pcurves).max(2);
        for i in 0..n {
            let s = i as f64 / (n - 1) as f64;
            let mut uv = coedge_uv_sample(body, co, surface, s)?;
            let is_pole = surface.is_degenerate_uv(uv.x, uv.y);
            if i == n - 1 && ci + 1 != coedges.len() && !is_pole {
                continue;
            }
            if surface.is_u_periodic() {
                if !prev_was_pole {
                    if let Some(pu) = prev_u {
                        uv.x = unwrap_u(pu, uv.x);
                    }
                }
                prev_u = Some(uv.x);
            }
            prev_was_pole = is_pole;
            if poly.last().is_some_and(|last| (last.x - uv.x).abs() <= UV_WELD && (last.y - uv.y).abs() <= UV_WELD) {
                continue;
            }
            poly.push(uv);
        }
    }
    while poly.len() > 1 {
        let (Some(first), Some(last)) = (poly.first().copied(), poly.last().copied()) else { break };
        if (first.x - last.x).abs() > UV_WELD || (first.y - last.y).abs() > UV_WELD {
            break;
        }
        poly.pop();
    }
    Ok(poly)
}

/// 📏 UV coincidence threshold for [`loop_uv_polygon`]'s duplicate-vertex weld — far below any
/// meaningful parametric feature (a full periodic domain is `2π` wide) and far above the rounding
/// two different p-curves can produce for the same shared vertex.
const UV_WELD: f64 = 1e-12;

/// 📏 Curvature-adaptive point count for one coedge's boundary contribution: with `coarse_pcurves` a
/// straight UV p-curve needs a point per `π/4` of `u` (so periodic unwrapping never jumps a seam) and no
/// more; exact 2 for a straight 3D `Line`, chordal-deviation-derived for `Circle`/`Ellipse` (their own radius, or the
/// major radius as a conservative upper bound for an ellipse's tighter minor-axis curvature), a
/// generously fine fixed count for `Nurbs` (no cheap closed-form curvature here; this file's
/// quadrature is not performance-critical enough to warrant the recursive bisection tessellation's
/// `sample_curve_adaptive` uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_sample_count(body: &Body, co: &semio_framework_3d_brep_representation::topology::Coedge, chord_tol: f64, coarse_pcurves: bool) -> usize {
    if let (true, Some(semio_framework_3d_brep_curve::Curve2::Line { dir, .. })) = (coarse_pcurves, co.pcurve.and_then(|id| body.curves2.get(id))) {
        let turn = dir.x.abs() * (co.prange.1 - co.prange.0).abs();
        return (turn / std::f64::consts::FRAC_PI_4).ceil().max(1.0) as usize + 1;
    }
    let Some(edge) = body.edges.get(co.edge) else { return 8 };
    let Some(curve) = body.curves3.get(edge.curve) else { return 8 };
    let (t0, t1) = edge.range;
    let arc_range = (t1 - t0).abs();
    let radius = match curve {
        Curve3::Line { .. } => return 2,
        Curve3::Circle { radius, .. } => *radius,
        Curve3::Ellipse { major_radius, .. } => *major_radius,
        Curve3::Nurbs { .. } => return 32,
    };
    segments_for_chord_deviation(radius, arc_range, chord_tol) + 1
}

/// 📏 Exact segment count for a circular arc of `radius` spanning `arc_range` radians so the chord
/// deviates from the arc by at most `deflection`: `n = ceil(arc_range / (2·acos(1 − deflection/radius)))`
/// — same closed form as tessellation's `segments_for_chord_deviation`, minus its angular-tol term
/// (see [`loop_uv_polygon`]'s doc for why that term doesn't apply here). The angular step is capped
/// at `π/4` regardless of `deflection`: a boundary polygon is only a polygon with at least three
/// chords around a full turn, and a caller whose tolerance is coarse relative to the radius (the
/// sliver probe on an 8 mm bore) otherwise collapsed a whole circle to two points and measured a
/// real face as zero area.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segments_for_chord_deviation(radius: f64, arc_range: f64, deflection: f64) -> usize {
    if radius <= 0.0 || arc_range <= 0.0 {
        return 1;
    }
    let d = deflection.max(1e-12).min(radius * 1.999);
    let ratio = (1.0 - d / radius).clamp(-1.0, 1.0);
    let theta = (2.0 * ratio.acos()).min(std::f64::consts::FRAC_PI_4);
    if theta <= 1e-9 {
        return ((arc_range / 1e-9).ceil() as usize).clamp(1, 200_000);
    }
    ((arc_range / theta).ceil() as usize).max(1)
}

/// 📏 One coedge, sampled at `s ∈ [0,1]` in UV — pcurve when present (edge's own order, see
/// [`loop_uv_polygon`]'s doc), else the 3D edge curve reprojected via `surface_uv`, reversed for a
/// backward coedge so the walk stays physically continuous.
/// 🐛 FIX (ticket `26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME` FX-5, same class as W2-B's fix to
/// `classification.rs`'s own `coedge_uv_sample`): the p-curve branch used to ignore `co.forward`
/// entirely (`p0 + (p1 - p0) * s` regardless of direction) — `prange` is stored in the underlying
/// edge's OWN curve order, never reparametrized per coedge, so a `forward == false` coedge must
/// walk it from `p1` down to `p0`, exactly like the no-pcurve fallback below already does for the
/// 3D curve's `range`. Left unfixed this silently traced every reversed pcurve-bearing coedge (half
/// of any non-planar loop) BACKWARDS relative to the rest of the ring, producing a self-crossing
/// UV boundary polygon whose shoelace/ear-clip quadrature integrates the wrong region — this is
/// what made the cylinder's general-path volume come out at ~24% of the closed-form value.
/// 📐 Signed area of one loop's own `(u, v)` polygon — POSITIVE exactly when the loop is
/// counter-clockwise in its surface's natural parametrisation. That sense is what decides which
/// side of the loop the face's trimmed region is on, so an OUTER loop must be positive and a hole
/// loop negative, whatever the face's `flipped` says about its normal. Exposed for
/// `validate_body`'s own winding check, which is the only invariant that catches a face built with
/// a backwards circuit — such a face can still be topologically coherent and can still integrate to
/// the right magnitude, so nothing else notices it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn loop_uv_signed_area(body: &Body, loop_id: semio_framework_3d_brep_representation::arena::LoopId, surface: &Surface, chord_tol: f64) -> Result<f64, KernelError> {
    let polygon = loop_uv_polygon(body, loop_id, surface, chord_tol)?;
    if polygon.len() < 3 {
        return Ok(0.0);
    }
    let mut twice = 0.0;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        twice += a.x * b.y - b.x * a.y;
    }
    Ok(0.5 * twice)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_uv_sample(body: &Body, co: &semio_framework_3d_brep_representation::topology::Coedge, surface: &Surface, s: f64) -> Result<Pnt2, KernelError> {
    if let Some(pcurve_id) = co.pcurve {
        let pcurve = body.curves2.get(pcurve_id).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?;
        let (p0, p1) = co.prange;
        let p = if co.forward { p0 + (p1 - p0) * s } else { p1 - (p1 - p0) * s };
        return Ok(pcurve.eval(p));
    }
    let edge = body.edges.get(co.edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    let (t0, t1) = edge.range;
    let t = if co.forward { t0 + (t1 - t0) * s } else { t1 - (t1 - t0) * s };
    Ok(surface_uv(surface, curve.eval(t)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_u(prev: f64, u: f64) -> f64 {
    let mut w = u;
    let pi = std::f64::consts::PI;
    while w - prev > pi {
        w -= std::f64::consts::TAU;
    }
    while w - prev < -pi {
        w += std::f64::consts::TAU;
    }
    w
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn surface_uv(surface: &Surface, p: Pnt3) -> Pnt2 {
    match surface {
        Surface::Plane { frame } => {
            let l = frame.to_local(p);
            Pnt2::new(l.x, l.y)
        }
        Surface::Cylinder { frame, radius: _ } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            Pnt2::new(u, l.z)
        }
        Surface::Cone { frame, half_angle } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let v = l.z / half_angle.tan().max(1e-15);
            Pnt2::new(u, v)
        }
        Surface::Sphere { frame, radius } => {
            let l = (p - frame.origin).normalized().unwrap_or(Vec3::Z);
            let v = l.z.clamp(-1.0, 1.0).asin();
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let _ = radius;
            Pnt2::new(u, v)
        }
        Surface::Torus { frame, major_radius, minor_radius } => {
            let l = frame.to_local(p);
            let u = l.y.atan2(l.x).rem_euclid(std::f64::consts::TAU);
            let radial = (l.x * l.x + l.y * l.y).sqrt();
            let v = ((radial - *major_radius) / minor_radius.max(1e-15)).clamp(-1.0, 1.0).acos();
            Pnt2::new(u, v)
        }
        Surface::Nurbs { .. } => {
            let domain = surface.domain();
            let closest = surface_ops::closest_uv(surface, domain, p, 1e-9);
            Pnt2::new(closest.u, closest.v)
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn point_in_uv_polygon(u: f64, v: f64, poly: &[Pnt2]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let yi = poly[i].y;
        let yj = poly[j].y;
        if (yi > v) != (yj > v) {
            let xi = poly[i].x;
            let xj = poly[j].x;
            let x_int = (xj - xi) * (v - yi) / (yj - yi) + xi;
            if u < x_int {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}
// #endregion 🔖️Loops
