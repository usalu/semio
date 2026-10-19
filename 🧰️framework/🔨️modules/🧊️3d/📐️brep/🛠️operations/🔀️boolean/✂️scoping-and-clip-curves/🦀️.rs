//! ✂️ Scoping and clip curves: the new solid issue scoping and the clipping of an intersection curve to both face trims with the periodic UV helpers and boundary curve lookups. See [`super`].

use super::*;

// #region 🔖️Scoping
/// 🔀 Filters `issues` down to the ones attributable to solids the boolean just CREATED — a
/// pre-existing operand may already have carried invalid topology, and the boolean must not be
/// blamed for it. Resolves each issue's `entity` string against a forward map of every string
/// [`validate_body`]'s checks can emit for something owned by a `pre_existing_solids` member
/// (built by [`pre_existing_entity_strings`]) rather than inverting `raw_index()` back into an id,
/// which would silently desync the moment either side's format changes independently. An entity
/// that resolves to neither a pre-existing nor a new solid (orphaned, or unparseable) is kept —
/// conservative, since a boolean that leaves an unattributable issue is a real failure.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn issues_scoped_to_new_solids(body: &Body, pre_existing_solids: &HashSet<SolidId>, issues: Vec<ValidationIssue>) -> Vec<ValidationIssue> {
    let stale = pre_existing_entity_strings(body, pre_existing_solids);
    issues.into_iter().filter(|issue| !stale.contains(&issue.entity)).collect()
}

/// 🔀 Every `entity` string [`validate_body`]'s checks can emit for something owned — directly, or
/// via shell/face/loop/coedge containment down to the edge and vertex level — by one of `solids`.
/// See [`issues_scoped_to_new_solids`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pre_existing_entity_strings(body: &Body, solids: &HashSet<SolidId>) -> HashSet<String> {
    let mut owned = HashSet::new();
    for &solid_id in solids {
        owned.insert(format!("solid-{}", solid_id.raw_index()));
        let Some(solid) = body.solids.get(solid_id) else { continue };
        for &void_shell in &solid.inners {
            owned.insert(format!("solid-{}-void-shell-{}", solid_id.raw_index(), void_shell.raw_index()));
        }
        let faces = body.solid_faces(solid_id);
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                owned.insert(format!("face-{}-face-{}", faces[i].raw_index(), faces[j].raw_index()));
            }
        }
        for &face_id in &faces {
            owned.insert(format!("face-{}", face_id.raw_index()));
            for loop_id in body.face_loops(face_id) {
                owned.insert(format!("loop-{}", loop_id.raw_index()));
                for coedge_id in body.loop_coedges(loop_id) {
                    owned.insert(format!("coedge-{}", coedge_id.raw_index()));
                    let Some(coedge) = body.coedges.get(coedge_id) else { continue };
                    owned.insert(format!("edge-{}", coedge.edge.raw_index()));
                    let Some(edge) = body.edges.get(coedge.edge) else { continue };
                    owned.insert(format!("vertex-{}", edge.v0.raw_index()));
                    owned.insert(format!("vertex-{}", edge.v1.raw_index()));
                }
            }
        }
        for shell_id in body.solid_shells(solid_id) {
            for face_id in body.shell_faces(shell_id) {
                for coedge_id in body.face_coedges(face_id) {
                    let Some(coedge) = body.coedges.get(coedge_id) else { continue };
                    owned.insert(format!("shell-{}-edge-{}", shell_id.raw_index(), coedge.edge.raw_index()));
                }
            }
        }
    }
    owned
}

// #region 🔖️Clip
/// 🔀 Clips `ic`'s shared parameter domain to the sub-ranges where BOTH `pcurve_a` (on `face_a`)
/// and `pcurve_b` (on `face_b`) land inside their own face's trim, sampling at a fixed resolution
/// (matching [`semio_framework_3d_brep_intersect::surface_surface::general_marching`]'s own documented-simplification sampling style).
/// A periodic curve whose valid runs cover (within 0.1% of the period) the WHOLE domain is
/// returned as ONE fully-closed range (`closed = true`) — this covers both "never touches either
/// boundary" and "touches a seam at exactly one physical point," which reads as a narrow gap
/// splitting one closed loop into two adjacent open runs (a latitude circle's seam touch can land
/// at any phase, not just the circle's own `t=0`/`t=period` ends). Otherwise every maximal valid
/// sub-run becomes an open range with its boundary refined by bisection (`closed = false`). Does
/// not handle a genuinely PARTIAL run that wraps across the periodic domain's own `t=lo`/`t=hi`
/// seam (documented gap — not hit by the analytic primitive pairs this wave targets).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn clip_intcurve_to_faces(body: &Body, ic: &IntCurve, face_a: FaceId, face_b: FaceId, tol: f64) -> Vec<(f64, f64, bool, Vec<f64>)> {
    let (lo, hi) = intcurve_finite_bracket(body, ic, face_a, face_b);
    if hi.partial_cmp(&lo) != Some(std::cmp::Ordering::Greater) {
        return Vec::new();
    }
    let periodic = ic.curve3.is_periodic() && ic.curve3.period().is_some_and(|p| (hi - lo - p).abs() < 1e-6 * p.max(1.0));
    const N: usize = 64;
    let valid = |t: f64| -> bool {
        let ua = wrap_uv_for_surface(body, face_a, ic.pcurve_a.eval(t));
        let ub = wrap_uv_for_surface(body, face_b, ic.pcurve_b.eval(t));
        point_in_face_uv_closure_periodic(body, face_a, ua, tol) && point_in_face_uv_closure_periodic(body, face_b, ub, tol)
    };
    let mut inside = [false; N + 1];
    for (i, slot) in inside.iter_mut().enumerate() {
        let t = lo + (hi - lo) * (i as f64 / N as f64);
        *slot = valid(t);
    }
    let cell = (hi - lo) / N as f64;
    let reach = (cell, curve3_extent(&ic.curve3, (lo, lo + cell)).max(1e-12) * 2.0);
    let mut runs: Vec<(f64, f64)> = Vec::new();
    let mut i = 0usize;
    while i <= N {
        if inside[i] {
            let start = i;
            while i <= N && inside[i] {
                i += 1;
            }
            let end = i - 1;
            let t_at = |k: usize| lo + (hi - lo) * (k as f64 / N as f64);
            let t0 = if start > 0 { snap_clip_endpoint(body, ic, (face_a, face_b), refine_boundary(&valid, t_at(start - 1), t_at(start)), reach, tol) } else { t_at(start) };
            let t1 = if end < N { snap_clip_endpoint(body, ic, (face_a, face_b), refine_boundary(&valid, t_at(end + 1), t_at(end)), reach, tol) } else { t_at(end) };
            if t1 > t0 {
                runs.push((t0, t1));
            }
        } else {
            i += 1;
        }
    }
    if periodic && !runs.is_empty() {
        let covered: f64 = runs.iter().map(|&(t0, t1)| t1 - t0).sum();
        if (hi - lo - covered) < (hi - lo) * 1e-3 {
            return vec![(lo, hi, true, boundary_touch_parameters(body, ic, (face_a, face_b), (lo, hi), true, tol))];
        }
    }
    runs.into_iter().map(|(t0, t1)| (t0, t1, false, Vec::new())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refine_boundary(valid: &impl Fn(f64) -> bool, mut outside: f64, mut inside: f64) -> f64 {
    for _ in 0..40 {
        let mid = 0.5 * (outside + inside);
        if valid(mid) {
            inside = mid;
        } else {
            outside = mid;
        }
    }
    inside
}

/// 🔀 [`IntCurve::domain`] is infinite for an unbounded [`semio_framework_3d_brep_curve::Curve3::Line`] (e.g. plane/plane,
/// coincident-cylinder-axis lines); windows it to the parameter run where the line passes through
/// the two faces' AABB INTERSECTION (a slab clip per axis, each slab widened by a small margin), so
/// [`clip_intcurve_to_faces`]'s fixed 64-cell sampling spends its cells where an imprint can exist
/// at all — a line's own `domain` (finite or not) is intersected with that window too. 🐛 The window
/// used to be a radius of 4× the LARGER face's diagonal around the pair's centroid: for a box slab cutting through a beam (process3d's crosscut, 0.02 m of a 3 m
/// stock) the valid run was a fraction of one sampling cell, every pair clipped to nothing, no
/// imprint was queued, and the stitch answered a non-manifold body (ticket
/// 26/09/15/DEV-PROCESS-REACT-E2E). A line that never enters the intersection AABB answers an
/// empty (`hi < lo`) bracket; a non-line unbounded curve keeps the old radius window.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn intcurve_finite_bracket(body: &Body, ic: &IntCurve, face_a: FaceId, face_b: FaceId) -> (f64, f64) {
    let (lo, hi) = (ic.domain.min, ic.domain.max);
    let is_line = matches!(ic.curve3, semio_framework_3d_brep_curve::Curve3::Line { .. });
    if lo.is_finite() && hi.is_finite() && !is_line {
        return (lo, hi);
    }
    let Ok((ba, bb)) = face_aabb(body, face_a).and_then(|ba| face_aabb(body, face_b).map(|bb| (ba, bb))) else {
        return (if lo.is_finite() { lo } else { -100.0 }, if hi.is_finite() { hi } else { 100.0 });
    };
    if let semio_framework_3d_brep_curve::Curve3::Line { origin, dir } = &ic.curve3 {
        let extent = (0..3).map(|axis| (ba.max[axis] - ba.min[axis]).max(bb.max[axis] - bb.min[axis])).fold(0.0f64, f64::max).max(1e-9);
        let margin = extent * 1e-3;
        let (mut t_lo, mut t_hi) = (f64::NEG_INFINITY, f64::INFINITY);
        for axis in 0..3 {
            let (min, max) = (ba.min[axis].max(bb.min[axis]) - margin, ba.max[axis].min(bb.max[axis]) + margin);
            let (o, d) = ([origin.x, origin.y, origin.z][axis], [dir.x, dir.y, dir.z][axis]);
            if d.abs() <= 1e-12 {
                if o < min || o > max {
                    return (1.0, 0.0);
                }
                continue;
            }
            let (a, b) = ((min - o) / d, (max - o) / d);
            t_lo = t_lo.max(a.min(b));
            t_hi = t_hi.min(a.max(b));
        }
        if t_lo.is_finite() && t_hi.is_finite() {
            return (if lo.is_finite() { lo.max(t_lo) } else { t_lo }, if hi.is_finite() { hi.min(t_hi) } else { t_hi });
        }
    }
    let dx = (ba.max[0] - ba.min[0]).max(bb.max[0] - bb.min[0]);
    let dy = (ba.max[1] - ba.min[1]).max(bb.max[1] - bb.min[1]);
    let dz = (ba.max[2] - ba.min[2]).max(bb.max[2] - bb.min[2]);
    let radius = (dx * dx + dy * dy + dz * dz).sqrt().max(1.0) * 4.0;
    (if lo.is_finite() { lo } else { -radius }, if hi.is_finite() { hi } else { radius })
}

/// 🔀 Diameter (max pairwise distance) of 16 samples of `curve` across `domain` — a cheap,
/// curve-kind-agnostic stand-in for "how big is this intersection curve", used to distinguish a
/// genuine contact circle/ellipse from a near-zero-radius numerical artifact of a tolerant
/// overlap test (see the near-tangent-spheres skip at this function's call site).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn curve3_extent(curve: &semio_framework_3d_brep_curve::Curve3, domain: (f64, f64)) -> f64 {
    const K: usize = 16;
    let (lo, hi) = domain;
    if hi.partial_cmp(&lo) != Some(std::cmp::Ordering::Greater) {
        return 0.0;
    }
    let pts: Vec<Pnt3> = (0..=K).map(|i| curve.eval(lo + (hi - lo) * (i as f64 / K as f64))).collect();
    let mut max_d = 0.0f64;
    for i in 0..pts.len() {
        for j in (i + 1)..pts.len() {
            max_d = max_d.max(pts[i].distance(pts[j]));
        }
    }
    max_d
}

/// 🔀 `true` when `point` lies within `tol` of `loop_id`'s own boundary edges — distinguishes a
/// closed imprint curve that genuinely bounds a small interior sub-region from one that grazes a
/// doubly-used seam edge (see [`ImprintKind::SeamCrossing`]). Uses [`closest_parameter`]'s
/// certified/analytic projection (exact for `Line`/`Circle`/`Ellipse`, Newton-refined for
/// `Nurbs`), not a fixed sample grid: 24 discrete samples per edge (the previous approach) are
/// almost always MORE than `tol` away from the true closest point on the curve even when `point`
/// lies exactly ON it — e.g. a great-circle seam edge sampled every ~0.13 rad has no reason to
/// land within `1e-6` of an arbitrary touch parameter — silently misclassifying every genuine
/// seam touch as `Interior`. 🐛 Found live: a sphere/sphere lens's touch point at the seam's own
/// v≈0.6435 rad sampled false under the old 24-point check, misrouting the split to `Interior`,
/// whose resulting polar-cap face then has a UV boundary that is a straight line across the full
/// periodic `u` domain (zero enclosed area in flat UV) — `interior_point_of_face`'s grid scan
/// then correctly, but unhelpfully, finds no inside point at all.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_touches_loop_boundary(body: &Body, loop_id: LoopId, point: Pnt3, tol: f64) -> bool {
    let linear = tol.max(1e-9);
    for cid in body.loop_coedges(loop_id) {
        let Some(co) = body.coedges.get(cid) else { continue };
        let Some(edge) = body.edges.get(co.edge) else { continue };
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        let cp = closest_parameter(curve, edge.range, point, linear);
        if cp.distance <= linear {
            return true;
        }
    }
    false
}

/// 🔀 Wraps a p-curve sample into the face's own surface's natural periodic domain before a trim
/// test — a periodic curve's own parametrization can legitimately produce `u`/`v` outside
/// `[0, 2π)` (e.g. a phase-shifted azimuthal line), which is still the same physical point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_uv_for_surface(body: &Body, face: FaceId, uv: Pnt2) -> Pnt2 {
    let Some(face_data) = body.faces.get(face) else { return uv };
    let Some(surface) = body.surfaces.get(face_data.surface) else { return uv };
    let mut out = uv;
    if surface.is_u_periodic() {
        out.x = out.x.rem_euclid(std::f64::consts::TAU);
    }
    if surface.is_v_periodic() {
        out.y = out.y.rem_euclid(std::f64::consts::TAU);
    }
    out
}

/// 🔀 `point_in_face_uv`, but tolerant of a periodic loop's own UV representation having drifted
/// by any whole number of periods from `wrap_uv_for_surface`'s canonical `[0, 2π)` window.
/// `classification::loop_uv_polygon_sampled` unwraps each face's OWN boundary polygon
/// continuously starting from its own first sample (chaining `unwrap_angle` across coedges), so
/// two DIFFERENT faces' polygons can legitimately end up centered on different multiples of `2π`
/// even for the "same" physical seam — comparing a single canonically-wrapped query point against
/// that polygon is not reliable. Instead of assuming any particular offset, tries the query at
/// every small integer shift and accepts the first that lands inside — cheap (a handful of extra
/// `point_in_face_uv` calls) and correct regardless of which offset either polygon settled on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn point_in_face_uv_periodic(body: &Body, face: FaceId, uv: Pnt2, tol: f64) -> bool {
    uv_probe_over_period_shifts(body, face, uv, |candidate| point_in_face_uv(body, face, candidate, tol).unwrap_or(false))
}

/// 🔀 [`point_in_face_uv_periodic`] against the trim's CLOSURE — see
/// [`semio_framework_3d_brep_queries::classification::point_in_face_uv_closure`]
/// for why a clip must use the closed test (poles, seams and coincident arcs all live exactly on
/// the boundary and are invisible to the open one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_face_uv_closure_periodic(body: &Body, face: FaceId, uv: Pnt2, tol: f64) -> bool {
    uv_probe_over_period_shifts(body, face, uv, |candidate| point_in_face_uv_closure(body, face, candidate, tol).unwrap_or(false))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn uv_probe_over_period_shifts(body: &Body, face: FaceId, uv: Pnt2, probe: impl Fn(Pnt2) -> bool) -> bool {
    let Some(face_data) = body.faces.get(face) else { return false };
    let Some(surface) = body.surfaces.get(face_data.surface) else { return false };
    let tau = std::f64::consts::TAU;
    let u_shifts: &[f64] = if surface.is_u_periodic() { &[0.0, tau, -tau, 2.0 * tau, -2.0 * tau, 3.0 * tau, -3.0 * tau] } else { &[0.0] };
    let v_shifts: &[f64] = if surface.is_v_periodic() { &[0.0, tau, -tau, 2.0 * tau, -2.0 * tau] } else { &[0.0] };
    for &du in u_shifts {
        for &dv in v_shifts {
            if probe(Pnt2::new(uv.x + du, uv.y + dv)) {
                return true;
            }
        }
    }
    false
}

/// 🔀 Every boundary curve (3D curve + its edge's own range) of `face`'s outer ring — the exact
/// geometry a clip endpoint has to land ON, and the geometry a closed imprint curve grazes when it
/// crosses a seam.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn outer_boundary_curves(body: &Body, face: FaceId) -> Vec<(semio_framework_3d_brep_curve::Curve3, (f64, f64))> {
    let mut out = Vec::new();
    let Some(outer) = body.faces.get(face).and_then(|f| f.outer) else { return out };
    for coedge_id in body.loop_coedges(outer) {
        let Some(coedge) = body.coedges.get(coedge_id) else { continue };
        let Some(edge) = body.edges.get(coedge.edge) else { continue };
        let Some(curve) = body.curves3.get(edge.curve) else { continue };
        out.push((curve.clone(), edge.range));
    }
    out
}

/// 🔀 Distance from `point` to the nearest of `curves`, and that curve's index — [`closest_parameter`]
/// is analytic for `Line`/`Circle`/`Ellipse` (including a POINT edge, whose zero-direction line
/// projects to its own origin) and Newton-refined for `Nurbs`, so this is the certified distance,
/// not a sampled estimate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn nearest_boundary_curve(curves: &[(semio_framework_3d_brep_curve::Curve3, (f64, f64))], point: Pnt3, tol: f64) -> Option<(usize, f64)> {
    nearest_transversal_boundary_curve(curves, point, None, tol)
}

/// 🔀 [`nearest_boundary_curve`] restricted to curves the intersection curve actually CROSSES at
/// `point`: a boundary the curve runs ALONG (tangents parallel — a segment lying on an operand's own
/// edge, the flush-box case) is at distance 0 everywhere and would pin the endpoint where it already
/// is, while the crossing that really ends the run is the perpendicular boundary a few `tol` away.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn nearest_transversal_boundary_curve(curves: &[(semio_framework_3d_brep_curve::Curve3, (f64, f64))], point: Pnt3, along: Option<Vec3>, tol: f64) -> Option<(usize, f64)> {
    let mut best: Option<(usize, f64)> = None;
    for (index, (curve, range)) in curves.iter().enumerate() {
        let closest = closest_parameter(curve, *range, point, tol);
        if let (Some(direction), Some(tangent)) = (along, curve.tangent(closest.t)) {
            if direction.cross(tangent).norm() <= 1e-6 * direction.norm().max(1e-12) * tangent.norm().max(1e-12) {
                continue;
            }
        }
        if best.is_none_or(|(_, bd)| closest.distance < bd) {
            best = Some((index, closest.distance));
        }
    }
    best
}
// #endregion 🔖️Clip
// #endregion 🔖️ExactImprintEngine
