//! 📌️ Clip endpoint snapping: the clip endpoint snap, the boundary touch parameters, the clip p-curves and the golden section minimum. See [`super`].

use super::*;

// #region 🔖️Scoping
// #region 🔖️Clip
/// 🔀 Pulls a bisection-refined clip endpoint onto the trim boundary it approximates, EXACTLY.
///
/// [`refine_boundary`] can only ever locate the crossing to wherever the (tolerant) trim predicate
/// flips, i.e. to within `tol` of the truth — and the two faces of a pair flip in different
/// directions, so the SAME physical corner comes out of two different face pairs up to `2·tol`
/// apart, which is outside the imprint weld radius and leaves two arcs that physically meet as an
/// unconnected pair. Alternating projection fixes that at the source: project the endpoint onto the
/// boundary curve it is closest to, re-solve the intersection curve's own parameter for that
/// projected point, repeat. For the transversal crossings a clip produces this is a contraction
/// and converges to the true curve/curve intersection in a handful of steps, so both face pairs
/// land on the identical point and the weld — and therefore the chain — succeeds.
///
/// Returns `t_guess` unchanged when no boundary curve is near enough to be the crossing (the run
/// ended at the sampled domain's own edge, not at a trim), so a genuinely unbounded run is left
/// alone.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn snap_clip_endpoint(body: &Body, ic: &IntCurve, (face_a, face_b): (FaceId, FaceId), t_guess: f64, (param_reach, space_reach): (f64, f64), tol: f64) -> f64 {
    let linear = tol.max(1e-12);
    let mut curves = outer_boundary_curves(body, face_a);
    curves.extend(outer_boundary_curves(body, face_b));
    if curves.is_empty() {
        return t_guess;
    }
    let mut point = ic.curve3.eval(t_guess);
    let Some((index, distance)) = nearest_transversal_boundary_curve(&curves, point, ic.curve3.tangent(t_guess), linear) else { return t_guess };
    if distance > space_reach {
        return t_guess;
    }
    let (curve, range) = &curves[index];
    let mut t = t_guess;
    for _ in 0..8 {
        let projected = closest_parameter(curve, *range, point, linear).point;
        let solved = closest_parameter(&ic.curve3, (t_guess - param_reach, t_guess + param_reach), projected, linear);
        if (solved.t - t).abs() <= 1e-15 {
            return solved.t;
        }
        t = solved.t;
        point = solved.point;
    }
    t
}

/// 🔀 Parameters at which a CLOSED imprint curve grazes either support face's own boundary ring —
/// the seam touches that decide [`ImprintKind::SeamCrossing`] and anchor its imprint vertex.
///
/// Derived from the geometry directly rather than from gaps in the trim sampling: once the clip
/// uses the trim's CLOSURE (as it must, so a curve reaching a pole or running along a seam is not
/// truncated), a seam touch leaves NO gap to read it off. Scans the distance to the nearest
/// boundary curve, keeps every local minimum that actually reaches within tolerance, and refines
/// each by golden-section search so the anchor lands on the seam edge itself — which is exactly
/// what `splice_boundary_vertex`'s point-on-edge test requires. A curve that is within tolerance of
/// a boundary over a large fraction of its length is not grazing it but COINCIDENT with it
/// ([`coincident_boundary_edge`]'s case), and yields no touches here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn boundary_touch_parameters(body: &Body, ic: &IntCurve, (face_a, face_b): (FaceId, FaceId), (lo, hi): (f64, f64), closed: bool, tol: f64) -> Vec<f64> {
    const N: usize = 512;
    let linear = tol.max(1e-9);
    let mut curves = outer_boundary_curves(body, face_a);
    curves.extend(outer_boundary_curves(body, face_b));
    if curves.is_empty() || hi <= lo {
        return Vec::new();
    }
    let step = (hi - lo) / N as f64;
    let at = |i: usize| lo + step * i as f64;
    let distance = |t: f64| nearest_boundary_curve(&curves, ic.curve3.eval(t), linear).map_or(f64::INFINITY, |(_, d)| d);
    let count = if closed { N } else { N + 1 };
    let samples: Vec<f64> = (0..count).map(|i| distance(at(i))).collect();
    if samples.iter().filter(|&&d| d <= linear).count() * 4 > count {
        return Vec::new();
    }
    let mut touches = Vec::new();
    let range = if closed { 0..N } else { 1..N };
    for i in range {
        let prev = samples[(i + count - 1) % count];
        let next = samples[(i + 1) % count];
        if !(samples[i] <= prev && samples[i] < next) {
            continue;
        }
        let refined = golden_section_minimum(&distance, at(i) - step, at(i) + step);
        if distance(refined) <= linear {
            touches.push(refined);
        }
    }
    touches
}

/// 🔀 The p-curve to imprint over the CLIPPED sub-range `(t0, t1)` and the p-curve parameter range that sub-range spans, repaired if
/// the one `intersect_surface_surface` supplied does not actually hold there.
///
/// An SSI p-curve is certified against the intersection curve's FULL natural domain, and where no closed form exists it is a global
/// interpolation of 33 inversion samples. A boolean never uses the full domain — it imprints a clipped sub-range — and a global fit
/// can be arbitrarily wrong on one: a great circle through a sphere's POLES has a `u` that jumps by π there, which no single
/// interpolation can represent. The supplied curve is therefore measured on the range actually used (to the edge's own tolerance,
/// which `validate_body`'s same-parameter check enforces) and rebuilt there when it misses: first AFFINELY (exact for every
/// latitude circle, meridian branch and ruling — the sub-arcs this stage produces on analytic supports), then by an adaptive
/// uniform-parameter interpolation of exact surface inversions refined until the measured deviation is within tolerance.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn pcurve_for_clip(surface: &Surface, curve3: &semio_framework_3d_brep_curve::Curve3, pcurve: &Curve2, (t0, t1): (f64, f64), tol: f64) -> (Curve2, (f64, f64)) {
    const K: usize = 33;
    let edge_tol = tol.min(Tol::DEFAULT.value()) * 0.5;
    let at = |i: usize| t0 + (t1 - t0) * i as f64 / (K - 1) as f64;
    let deviation = |candidate: &Curve2| (0..K).map(|i| { let uv = candidate.eval(at(i)); surface.eval(uv.x, uv.y).distance(curve3.eval(at(i))) }).fold(0.0f64, f64::max);
    if deviation(pcurve) <= edge_tol {
        return (pcurve.clone(), (t0, t1));
    }
    let domain = surface.domain();
    let mut samples: Vec<Pnt2> = Vec::with_capacity(K);
    let mut degenerate: Vec<bool> = Vec::with_capacity(K);
    for i in 0..K {
        let found = closest_uv(surface, domain, curve3.eval(at(i)), tol.max(1e-12));
        samples.push(Pnt2::new(found.u, found.v));
        degenerate.push(surface.is_degenerate_uv(found.u, found.v));
    }
    if let Some(anchor) = (0..K).find(|&i| !degenerate[i]) {
        for i in (0..anchor).rev() {
            samples[i].x = samples[i + 1].x;
        }
        for i in (anchor + 1)..K {
            if degenerate[i] {
                samples[i].x = samples[i - 1].x;
            }
        }
        let mut previous = samples[anchor];
        for i in 0..K {
            if surface.is_u_periodic() {
                samples[i].x = unwrap_to(previous.x, samples[i].x);
            }
            if surface.is_v_periodic() {
                samples[i].y = unwrap_to(previous.y, samples[i].y);
            }
            previous = samples[i];
        }
        let fitted = affine_pcurve_through(&samples, (t0, t1));
        if deviation(&fitted) <= edge_tol {
            return (fitted, (t0, t1));
        }
    }
    arrangement::fitted_pcurve(surface, curve3, (t0, t1), edge_tol)
}

/// 🔀 Least-squares affine `Curve2::Line` through UV `samples` taken at evenly spaced parameters
/// across `(t0, t1)`, in the SAME absolute parametrization the samples were taken in (so the
/// result is a drop-in replacement for the p-curve it repairs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn affine_pcurve_through(samples: &[Pnt2], (t0, t1): (f64, f64)) -> Curve2 {
    let n = samples.len();
    let at = |i: usize| t0 + (t1 - t0) * i as f64 / (n - 1) as f64;
    let mean_t: f64 = (0..n).map(at).sum::<f64>() / n as f64;
    let mean_u: f64 = samples.iter().map(|p| p.x).sum::<f64>() / n as f64;
    let mean_v: f64 = samples.iter().map(|p| p.y).sum::<f64>() / n as f64;
    let mut stt = 0.0;
    let mut stu = 0.0;
    let mut stv = 0.0;
    for (i, sample) in samples.iter().enumerate() {
        let dt = at(i) - mean_t;
        stt += dt * dt;
        stu += dt * (sample.x - mean_u);
        stv += dt * (sample.y - mean_v);
    }
    let (du, dv) = if stt > 1e-30 { (stu / stt, stv / stt) } else { (0.0, 0.0) };
    Curve2::Line { origin: Pnt2::new(mean_u - du * mean_t, mean_v - dv * mean_t), dir: Vec2::new(du, dv) }
}

/// 🔀 Parameter minimizing `f` on `[lo, hi]` by golden-section search — used to pin a seam touch
/// (a smooth, strictly unimodal distance minimum within one sampling cell) to full precision.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn golden_section_minimum(f: &impl Fn(f64) -> f64, mut lo: f64, mut hi: f64) -> f64 {
    const INV_PHI: f64 = 0.618_033_988_749_894_9;
    let (mut c, mut d) = (hi - (hi - lo) * INV_PHI, lo + (hi - lo) * INV_PHI);
    let (mut fc, mut fd) = (f(c), f(d));
    for _ in 0..80 {
        if fc < fd {
            hi = d;
            d = c;
            fd = fc;
            c = hi - (hi - lo) * INV_PHI;
            fc = f(c);
        } else {
            lo = c;
            c = d;
            fc = fd;
            d = lo + (hi - lo) * INV_PHI;
            fd = f(d);
        }
    }
    0.5 * (lo + hi)
}
// #endregion 🔖️Clip
// #endregion 🔖️ExactImprintEngine
