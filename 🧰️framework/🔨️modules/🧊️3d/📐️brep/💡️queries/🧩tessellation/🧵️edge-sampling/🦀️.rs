//! 🧵️ Edge sampling: the single pass edge discretization with its uniform and adaptive curve sampling, polyline helpers and curve and surface kinds. See [`super`].

use super::*;

// #region 🧮EdgeSample
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn sample_edge_points(body: &Body, edge_id: EdgeId, deflection: f64) -> Result<Vec<(f64, Pnt3)>, KernelError> {
    let edge = body.edges.get(edge_id).ok_or_else(|| KernelError::MissingEntity(edge_id.to_string()))?;
    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity(edge.curve.to_string()))?;
    let v0 = body.vertices.get(edge.v0).ok_or_else(|| KernelError::MissingEntity(edge.v0.to_string()))?.position;
    let v1 = body.vertices.get(edge.v1).ok_or_else(|| KernelError::MissingEntity(edge.v1.to_string()))?.position;
    let (t0, t1) = edge.range;
    let mut points = match curve {
        Curve3::Line { .. } => vec![(t0, v0), (t1, v1)],
        Curve3::Circle { radius, .. } => {
            let n = segments_for_chord_deviation(*radius, (t1 - t0).abs(), deflection, DEFAULT_ANGULAR_TOL);
            sample_uniform(curve, t0, t1, n + 1)
        }
        Curve3::Ellipse { major_radius, minor_radius, .. } => {
            let curv_r = (*major_radius * *major_radius) / minor_radius.max(1e-12);
            let n = segments_for_chord_deviation(curv_r, (t1 - t0).abs(), deflection, DEFAULT_ANGULAR_TOL);
            sample_uniform(curve, t0, t1, n + 1)
        }
        Curve3::Nurbs { .. } => sample_curve_adaptive(curve, t0, t1, deflection, DEFAULT_ANGULAR_TOL),
    };
    if let Some(first) = points.first_mut() {
        first.1 = v0;
    }
    if let Some(last) = points.last_mut() {
        last.1 = v1;
    }
    Ok(points)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_uniform(curve: &Curve3, t0: f64, t1: f64, count: usize) -> Vec<(f64, Pnt3)> {
    let n = count.max(2);
    (0..n)
        .map(|i| {
            let t = t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64);
            (t, curve.eval(t))
        })
        .collect()
}

/// 📐 Recursive chordal + angular bisection (no fixed sample count): halves any span whose
/// midpoint deviates from the chord by more than `deflection`, or whose tangent turns by more
/// than `angular_tol`, up to [`MAX_CURVE_SUBDIV_DEPTH`] — the "exact" adaptive criterion replacing
/// the old coarse-grid-then-rescale heuristic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sample_curve_adaptive(curve: &Curve3, t0: f64, t1: f64, deflection: f64, angular_tol: f64) -> Vec<(f64, Pnt3)> {
    let mut out = vec![(t0, curve.eval(t0)), (t1, curve.eval(t1))];
    subdivide_curve_segment(curve, t0, t1, deflection, angular_tol, 0, &mut out);
    out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    out.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-13);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn subdivide_curve_segment(curve: &Curve3, ta: f64, tb: f64, deflection: f64, angular_tol: f64, depth: u32, out: &mut Vec<(f64, Pnt3)>) {
    if depth >= MAX_CURVE_SUBDIV_DEPTH {
        return;
    }
    let pa = curve.eval(ta);
    let pb = curve.eval(tb);
    let tm = 0.5 * (ta + tb);
    let pm = curve.eval(tm);
    let dev = pm.distance(pa.lerp(pb, 0.5));
    let ang = match (curve.tangent(ta), curve.tangent(tb)) {
        (Some(a), Some(b)) => a.angle_to(b),
        _ => 0.0,
    };
    if dev > deflection || ang > angular_tol {
        subdivide_curve_segment(curve, ta, tm, deflection, angular_tol, depth + 1, out);
        out.push((tm, pm));
        subdivide_curve_segment(curve, tm, tb, deflection, angular_tol, depth + 1, out);
    }
}

/// 📐 Exact segment count for a circular arc of `radius` spanning `arc_range` radians so that the
/// chord deviates from the arc by at most `deflection` *and* each step turns by at most
/// `angular_tol`: `n = ceil(arc_range / min(2·acos(1 − deflection/radius), angular_tol))`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segments_for_chord_deviation(radius: f64, arc_range: f64, deflection: f64, angular_tol: f64) -> usize {
    if radius <= 0.0 || arc_range <= 0.0 {
        return 1;
    }
    let d = deflection.min(radius * 1.999);
    let ratio = (1.0 - d / radius).clamp(-1.0, 1.0);
    let theta_chord = 2.0 * ratio.acos();
    let theta_step = if angular_tol > 0.0 { theta_chord.min(angular_tol) } else { theta_chord };
    if theta_step <= 1e-9 {
        return ((arc_range / 1e-9).ceil() as usize).clamp(1, 200_000);
    }
    ((arc_range / theta_step).ceil() as usize).max(1)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn polyline_length(points: &[(f64, Pnt3)]) -> f64 {
    points.windows(2).map(|w| w[0].1.distance(w[1].1)).sum()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn push_polyline_segments(out: &mut Vec<f32>, points: &[Pnt3]) {
    for window in points.windows(2) {
        let a = window[0];
        let b = window[1];
        out.extend([a.x as f32, a.y as f32, a.z as f32, b.x as f32, b.y as f32, b.z as f32]);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn curve_kind_of(curve: &Curve3) -> CurveKind {
    match curve {
        Curve3::Line { .. } => CurveKind::Line,
        Curve3::Circle { .. } => CurveKind::Circle,
        Curve3::Ellipse { .. } => CurveKind::Ellipse,
        Curve3::Nurbs { .. } => CurveKind::Nurbs,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn surface_kind_of(surface: &Surface) -> SurfaceKind {
    match surface {
        Surface::Plane { .. } => SurfaceKind::Plane,
        Surface::Cylinder { .. } => SurfaceKind::Cylinder,
        Surface::Cone { .. } => SurfaceKind::Cone,
        Surface::Sphere { .. } => SurfaceKind::Sphere,
        Surface::Torus { .. } => SurfaceKind::Torus,
        Surface::Nurbs { .. } => SurfaceKind::Nurbs,
    }
}
// #endregion 🧮EdgeSample
