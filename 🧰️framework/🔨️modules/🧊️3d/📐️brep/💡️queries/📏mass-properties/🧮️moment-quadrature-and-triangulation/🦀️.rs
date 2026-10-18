//! 🧮️ Moment quadrature and triangulation: the moment component layout, the adaptive triangle quadrature and the planar polygon ear clipping. See [`super`].

use super::*;

// #region 🔖️Quadrature
/// 📏 One UV triangle's surface-integral contributions, all in NATURAL physical units (no legacy
/// ×6/×24 tetra-sum scaling): `[area, vol, mx, my, mz, ∫x²dV, ∫y²dV, ∫z²dV, ∫xy dV, ∫xz dV, ∫yz dV]`.
/// Every entry after `area` is a divergence-theorem surface integral of the form `∮ f(p)·n dA`,
/// and since `n dA = (du × dv) du dv` exactly (no need to normalize the cross product), each is
/// just `f(p) · cross` at the sample point — the SAME per-sample machinery serves volume, first
/// moments (centroid) and second moments (inertia) at once.
pub(crate) const MOMENT_COMPONENTS: usize = 14;

pub(super) const IDX_AREA: usize = 0;

pub(super) const IDX_VOL: usize = 1;

pub(super) const IDX_MX: usize = 2;

pub(super) const IDX_MY: usize = 3;

pub(super) const IDX_MZ: usize = 4;

pub(super) const IDX_JXX2: usize = 5;

pub(super) const IDX_JYY2: usize = 6;

pub(super) const IDX_JZZ2: usize = 7;

pub(super) const IDX_JXY: usize = 8;

pub(super) const IDX_JXZ: usize = 9;

pub(super) const IDX_JYZ: usize = 10;

pub(super) const IDX_AMX: usize = 11;

pub(super) const IDX_AMY: usize = 12;

pub(super) const IDX_AMZ: usize = 13;

/// 📏 6-point symmetric (degree-4) triangle quadrature rule in barycentric coordinates — exact
/// for the cubic integrands second moments need on a flat facet, and a stable adaptive-refinement
/// base case for curved ones.
const TRI_BARY: [[f64; 3]; 6] = [
    [0.108_103_018_168_070, 0.445_948_490_915_965, 0.445_948_490_915_965],
    [0.445_948_490_915_965, 0.108_103_018_168_070, 0.445_948_490_915_965],
    [0.445_948_490_915_965, 0.445_948_490_915_965, 0.108_103_018_168_070],
    [0.816_847_572_980_459, 0.091_576_213_509_771, 0.091_576_213_509_771],
    [0.091_576_213_509_771, 0.816_847_572_980_459, 0.091_576_213_509_771],
    [0.091_576_213_509_771, 0.091_576_213_509_771, 0.816_847_572_980_459],
];

const TRI_WEIGHT: [f64; 6] = [0.223_381_589_678_011, 0.223_381_589_678_011, 0.223_381_589_678_011, 0.109_951_743_655_322, 0.109_951_743_655_322, 0.109_951_743_655_322];

/// 📏 A signed moment vector followed by the matching absolute-magnitude vector (`Σ w·|term|` per
/// component) — the magnitudes are the yardstick each component's refinement is judged against, so a
/// moment whose true value is zero (a symmetric solid's first moment) still converges relative to the
/// size of its own terms instead of passing on the volume alone.
const ACCUMULATOR: usize = 2 * MOMENT_COMPONENTS;

/// 📏 Single (non-adaptive) 6-point quadrature pass over one UV triangle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quad_triangle_once(surface: &Surface, flipped: bool, tri: [Pnt2; 3]) -> [f64; ACCUMULATOR] {
    let signed_area2 = (tri[1].x - tri[0].x) * (tri[2].y - tri[0].y) - (tri[1].y - tri[0].y) * (tri[2].x - tri[0].x);
    let tri_area = 0.5 * signed_area2.abs();
    let mut acc = [0.0; ACCUMULATOR];
    if tri_area < 1e-15 {
        return acc;
    }
    for k in 0..6 {
        let [l1, l2, l3] = TRI_BARY[k];
        let u = l1 * tri[0].x + l2 * tri[1].x + l3 * tri[2].x;
        let v = l1 * tri[0].y + l2 * tri[1].y + l3 * tri[2].y;
        let d = surface.derivatives(u, v);
        let p = d.point;
        let mut cross = d.du.cross(d.dv);
        if flipped {
            cross = -cross;
        }
        let w = TRI_WEIGHT[k] * tri_area;
        let area = cross.norm();
        let mut terms = [0.0; MOMENT_COMPONENTS];
        terms[IDX_AREA] = area;
        terms[IDX_AMX] = area * p.x;
        terms[IDX_AMY] = area * p.y;
        terms[IDX_AMZ] = area * p.z;
        terms[IDX_VOL] = (p.x * cross.x + p.y * cross.y + p.z * cross.z) / 3.0;
        terms[IDX_MX] = 0.5 * p.x * p.x * cross.x;
        terms[IDX_MY] = 0.5 * p.y * p.y * cross.y;
        terms[IDX_MZ] = 0.5 * p.z * p.z * cross.z;
        terms[IDX_JXX2] = (p.x * p.x * p.x / 3.0) * cross.x;
        terms[IDX_JYY2] = (p.y * p.y * p.y / 3.0) * cross.y;
        terms[IDX_JZZ2] = (p.z * p.z * p.z / 3.0) * cross.z;
        terms[IDX_JXY] = 0.5 * p.x * p.x * p.y * cross.x;
        terms[IDX_JXZ] = 0.5 * p.x * p.x * p.z * cross.x;
        terms[IDX_JYZ] = 0.5 * p.y * p.y * p.z * cross.y;
        for i in 0..MOMENT_COMPONENTS {
            acc[i] += w * terms[i];
            acc[MOMENT_COMPONENTS + i] += w * terms[i].abs();
        }
    }
    acc
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_triangle_4(tri: [Pnt2; 3]) -> [[Pnt2; 3]; 4] {
    let m01 = tri[0].lerp(tri[1], 0.5);
    let m12 = tri[1].lerp(tri[2], 0.5);
    let m20 = tri[2].lerp(tri[0], 0.5);
    [[tri[0], m01, m20], [m01, tri[1], m12], [m20, m12, tri[2]], [m01, m12, m20]]
}

/// 📏 Adaptively refines one UV triangle by quartering until EVERY moment component's change between
/// one refinement level and the next falls within that component's `budget`, or `depth` is exhausted;
/// a child inherits a quarter of its parent's budget since it covers a quarter of the area. Returns the
/// accumulated moments plus a Richardson-style absolute error estimate for the volume component (the
/// coarse/fine gap at whichever level accepted the result).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn integrate_triangle_adaptive(surface: &Surface, flipped: bool, tri: [Pnt2; 3], budget: &[f64; MOMENT_COMPONENTS], depth: u32) -> ([f64; ACCUMULATOR], f64) {
    let coarse = quad_triangle_once(surface, flipped, tri);
    let subs = split_triangle_4(tri);
    let mut fine = [0.0; ACCUMULATOR];
    for s in &subs {
        let c = quad_triangle_once(surface, flipped, *s);
        for i in 0..ACCUMULATOR {
            fine[i] += c[i];
        }
    }
    let local_err = (fine[IDX_VOL] - coarse[IDX_VOL]).abs();
    if depth == 0 {
        return (fine, local_err);
    }
    if (0..MOMENT_COMPONENTS).all(|i| (fine[i] - coarse[i]).abs() <= budget[i]) {
        (fine, local_err)
    } else {
        let quarter = budget.map(|b| b / 4.0);
        let mut total = [0.0; ACCUMULATOR];
        let mut err_sum = 0.0;
        for s in subs {
            let (r, e) = integrate_triangle_adaptive(surface, flipped, s, &quarter, depth - 1);
            for i in 0..ACCUMULATOR {
                total[i] += r[i];
            }
            err_sum += e;
        }
        (total, err_sum)
    }
}

const ADAPTIVE_MAX_DEPTH: u32 = 6;

/// 📏 Parametric area of one UV triangle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn uv_triangle_area(tri: &[Pnt2; 3]) -> f64 {
    0.5 * ((tri[1].x - tri[0].x) * (tri[2].y - tri[0].y) - (tri[1].y - tri[0].y) * (tri[2].x - tri[0].x)).abs()
}

/// 📏 One loop's surface-integral moments over its ear-clipped UV triangulation — the caller adds
/// this for the outer loop and subtracts it for each inner (hole) loop to get the face's trimmed
/// total, exactly mirroring the existing planar `signed_tetra_sum` +outer/-inner pattern.
///
/// The loop's total refinement budget is `tol` of each component's summed absolute magnitude (one
/// coarse pass), shared out between its triangles in proportion to their parametric area, so a sliver
/// is not held to the relative accuracy of the whole face.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn loop_moments(surface: &Surface, flipped: bool, poly: &[Pnt2], tol: f64) -> ([f64; MOMENT_COMPONENTS], f64) {
    let triangles = ear_clip(poly);
    let total_area: f64 = triangles.iter().map(uv_triangle_area).sum();
    let mut scale = [0.0; MOMENT_COMPONENTS];
    for tri in &triangles {
        let coarse = quad_triangle_once(surface, flipped, *tri);
        for i in 0..MOMENT_COMPONENTS {
            scale[i] += coarse[MOMENT_COMPONENTS + i];
        }
    }
    let mut total = [0.0; MOMENT_COMPONENTS];
    let mut err = 0.0;
    for tri in triangles {
        let share = if total_area > 0.0 { uv_triangle_area(&tri) / total_area } else { 0.0 };
        let budget = scale.map(|magnitude| tol * magnitude * share);
        let (m, e) = integrate_triangle_adaptive(surface, flipped, tri, &budget, ADAPTIVE_MAX_DEPTH);
        for i in 0..MOMENT_COMPONENTS {
            total[i] += m[i];
        }
        err += e;
    }
    (total, err)
}
// #endregion 🔖️Quadrature

// #region 🔖️Triangulation
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn polygon_signed_area_2d(poly: &[Pnt2]) -> f64 {
    let mut a = 0.0;
    let n = poly.len();
    for i in 0..n {
        let p = poly[i];
        let q = poly[(i + 1) % n];
        a += p.x * q.y - q.x * p.y;
    }
    0.5 * a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_or_on_triangle(p: Pnt2, a: Pnt2, b: Pnt2, c: Pnt2) -> bool {
    let d1 = orient2d(a, b, p);
    let d2 = orient2d(b, c, p);
    let d3 = orient2d(c, a, p);
    let has_neg = matches!(d1, Orient::Negative) || matches!(d2, Orient::Negative) || matches!(d3, Orient::Negative);
    let has_pos = matches!(d1, Orient::Positive) || matches!(d2, Orient::Positive) || matches!(d3, Orient::Positive);
    !(has_neg && has_pos)
}

/// 📐 Ear-clipping triangulation of a simple (non-self-intersecting) polygon, robust to either
/// winding direction (normalized to CCW internally). Holes are handled by the caller triangulating
/// each loop separately and subtracting — no hole-bridging needed. A degenerate input (collinear
/// runs that starve the ear search) stops early rather than looping forever, covering whatever
/// prefix was already clipped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ear_clip(poly: &[Pnt2]) -> Vec<[Pnt2; 3]> {
    if poly.len() < 3 {
        return Vec::new();
    }
    let mut pts = poly.to_vec();
    if polygon_signed_area_2d(&pts) < 0.0 {
        pts.reverse();
    }
    let mut idx: Vec<usize> = (0..pts.len()).collect();
    let mut tris = Vec::new();
    let guard_limit = pts.len() * pts.len() + 8;
    let mut guard = 0usize;
    while idx.len() > 3 && guard < guard_limit {
        guard += 1;
        let n = idx.len();
        let mut ear_at: Option<usize> = None;
        for i in 0..n {
            let prev = idx[(i + n - 1) % n];
            let cur = idx[i];
            let next = idx[(i + 1) % n];
            let (a, b, c) = (pts[prev], pts[cur], pts[next]);
            if orient2d(a, b, c) != Orient::Positive {
                continue;
            }
            let blocked = idx.iter().any(|&k| k != prev && k != cur && k != next && point_in_or_on_triangle(pts[k], a, b, c));
            if !blocked {
                ear_at = Some(i);
                break;
            }
        }
        // 🩹 No ear at all means the remaining ring is numerically degenerate somewhere (a run of
        // exactly-collinear boundary samples, which every dense curve discretisation produces), NOT
        // that the polygon is untriangulable. Bailing out here silently returns a PARTIAL
        // triangulation, and every moment integrated over it — area, volume, centroid — is then
        // short by whatever the abandoned tail covered. Clip the flattest remaining corner instead:
        // its own triangle contributes ~nothing to the integral, and removing it always makes
        // progress, so the loop terminates with the whole polygon consumed.
        let ear_at = ear_at.or_else(|| {
            let n = idx.len();
            (0..n)
                .map(|i| {
                    let (a, b, c) = (pts[idx[(i + n - 1) % n]], pts[idx[i]], pts[idx[(i + 1) % n]]);
                    (i, ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)).abs())
                })
                .min_by(|left, right| left.1.partial_cmp(&right.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
        });
        let Some(i) = ear_at else { break };
        let n = idx.len();
        let prev = idx[(i + n - 1) % n];
        let cur = idx[i];
        let next = idx[(i + 1) % n];
        tris.push([pts[prev], pts[cur], pts[next]]);
        idx.remove(i);
    }
    if idx.len() == 3 {
        tris.push([pts[idx[0]], pts[idx[1]], pts[idx[2]]]);
    }
    tris
}
// #endregion 🔖️Triangulation
