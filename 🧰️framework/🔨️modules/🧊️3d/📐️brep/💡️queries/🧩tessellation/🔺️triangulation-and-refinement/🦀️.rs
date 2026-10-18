//! 🔺️ Triangulation and refinement: the constrained Bowyer-Watson triangulation with its edge recovery and geometric predicates and the adaptive interior refinement. See [`super`].

use super::*;

// #region ▲Triangulate
/// 📐 A CCW-oriented triangle referencing indices into a shared UV/position point array.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Tri {
    pub(super) a: usize,
    pub(super) b: usize,
    pub(super) c: usize,
}

/// 📐 Builds the outer/hole boundary rings into an unconstrained Bowyer–Watson Delaunay
/// triangulation of the whole point set, recovers every boundary/hole ring edge as an exact
/// triangulation edge via edge flips, then trims to the region inside the outer ring and outside
/// every hole ring — replacing the old ear-clip+fan-fallback+hole-bridging pipeline.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_constrained_triangulation(uvs: &[(f64, f64)], outer: (usize, usize), holes: &[(usize, usize)]) -> Result<Vec<Tri>, KernelError> {
    if outer.1 - outer.0 < 3 {
        return Err(KernelError::Operation("degenerate outer loop".to_string()));
    }
    let mut tris = bowyer_watson(uvs);
    let mut constraints: Vec<(usize, usize)> = Vec::new();
    push_ring_constraints(&mut constraints, outer.0, outer.1);
    for &(s, e) in holes {
        push_ring_constraints(&mut constraints, s, e);
    }
    for &(i, j) in &constraints {
        recover_edge(uvs, &mut tris, i, j);
    }
    let outer_poly: Vec<(f64, f64)> = uvs[outer.0..outer.1].to_vec();
    let hole_polys: Vec<Vec<(f64, f64)>> = holes.iter().map(|&(s, e)| uvs[s..e].to_vec()).collect();
    tris.retain(|t| {
        let c = centroid3(uvs[t.a], uvs[t.b], uvs[t.c]);
        point_in_polygon(&outer_poly, c) && hole_polys.iter().all(|h| !point_in_polygon(h, c))
    });
    Ok(tris)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_ring_constraints(out: &mut Vec<(usize, usize)>, start: usize, end: usize) {
    let count = end - start;
    for i in 0..count {
        out.push((start + i, start + (i + 1) % count));
    }
}

/// 📐 Incremental Bowyer–Watson: a super-triangle enclosing every point, then one-by-one point
/// insertion (remove circumcircle-violating triangles, re-fan the cavity boundary around the new
/// point), finally discarding any triangle still touching a super-triangle vertex.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bowyer_watson(pts: &[(f64, f64)]) -> Vec<Tri> {
    let n = pts.len();
    if n < 3 {
        return Vec::new();
    }
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for &(x, y) in pts {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let dx = (max_x - min_x).max(1e-6);
    let dy = (max_y - min_y).max(1e-6);
    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;
    let r = dx.max(dy) * 20.0 + 10.0;
    let mut all: Vec<(f64, f64)> = pts.to_vec();
    let s0 = all.len();
    all.push((cx - 2.0 * r, cy - r));
    let s1 = all.len();
    all.push((cx + 2.0 * r, cy - r));
    let s2 = all.len();
    all.push((cx, cy + 2.0 * r));
    let mut tris = vec![ensure_ccw(&all, Tri { a: s0, b: s1, c: s2 })];
    for i in 0..n {
        insert_point_into(&all, &mut tris, i);
    }
    tris.retain(|t| t.a < n && t.b < n && t.c < n);
    tris
}

/// 📐 Single-point Bowyer–Watson insertion into an existing triangulation. Safe to reuse for
/// interior Steiner refinement on an already-trimmed/constrained mesh: a constrained ring edge is
/// (after trimming) owned by exactly one triangle, so it can never be a *shared* cavity edge that
/// insertion removes — it always survives as a cavity-boundary edge, reattached to the new point.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn insert_point_into(pts: &[(f64, f64)], tris: &mut Vec<Tri>, p_idx: usize) {
    let p = pts[p_idx];
    let mut bad = Vec::new();
    let mut good = Vec::new();
    for &t in tris.iter() {
        if in_circumcircle(pts, t, p) {
            bad.push(t);
        } else {
            good.push(t);
        }
    }
    if bad.is_empty() {
        return;
    }
    let (bad, rejected) = connected_cavity(pts, bad, p);
    good.extend(rejected);
    let mut directed: Vec<(usize, usize)> = Vec::new();
    for t in &bad {
        directed.push((t.a, t.b));
        directed.push((t.b, t.c));
        directed.push((t.c, t.a));
    }
    let present: std::collections::HashSet<(usize, usize)> = directed.iter().copied().collect();
    let boundary: Vec<(usize, usize)> = directed.iter().copied().filter(|&(u, v)| !present.contains(&(v, u))).collect();
    let mut new_tris = good;
    for (u, v) in boundary {
        new_tris.push(ensure_ccw(pts, Tri { a: u, b: v, c: p_idx }));
    }
    *tris = new_tris;
}

/// 📐 The part of `bad` that is the cavity of `p`: the triangles reachable from the one containing `p` through shared edges. On a
/// trimmed (constrained) mesh a circumcircle can reach a triangle on the far side of a notch of the boundary; such a triangle is no
/// part of `p`'s star-shaped cavity and re-fanning it overlaps the mesh, which doubled the triangle count on every insertion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn connected_cavity(pts: &[(f64, f64)], bad: Vec<Tri>, p: (f64, f64)) -> (Vec<Tri>, Vec<Tri>) {
    let contains = |t: &Tri| orient(pts[t.a], pts[t.b], p) >= -1e-12 && orient(pts[t.b], pts[t.c], p) >= -1e-12 && orient(pts[t.c], pts[t.a], p) >= -1e-12;
    let Some(start) = bad.iter().position(contains) else { return (bad, Vec::new()) };
    let mut edges: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (i, t) in bad.iter().enumerate() {
        for (u, v) in [(t.a, t.b), (t.b, t.c), (t.c, t.a)] {
            edges.entry((u.min(v), u.max(v))).or_default().push(i);
        }
    }
    let mut reached = vec![false; bad.len()];
    let mut stack = vec![start];
    reached[start] = true;
    while let Some(i) = stack.pop() {
        let t = bad[i];
        for (u, v) in [(t.a, t.b), (t.b, t.c), (t.c, t.a)] {
            for &j in &edges[&(u.min(v), u.max(v))] {
                if !reached[j] {
                    reached[j] = true;
                    stack.push(j);
                }
            }
        }
    }
    let (mut cavity, mut rejected) = (Vec::new(), Vec::new());
    for (t, reached) in bad.into_iter().zip(reached) {
        if reached { cavity.push(t) } else { rejected.push(t) }
    }
    (cavity, rejected)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ensure_ccw(pts: &[(f64, f64)], t: Tri) -> Tri {
    if orient(pts[t.a], pts[t.b], pts[t.c]) < 0.0 {
        Tri { a: t.a, b: t.c, c: t.b }
    } else {
        t
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn orient(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn orient_sign(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> i32 {
    let v = orient(a, b, c);
    if v > 1e-12 {
        1
    } else if v < -1e-12 {
        -1
    } else {
        0
    }
}

/// 📐 Standard incircle determinant test (assumes `t` is CCW, guaranteed by [`ensure_ccw`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn in_circumcircle(pts: &[(f64, f64)], t: Tri, p: (f64, f64)) -> bool {
    let a = pts[t.a];
    let b = pts[t.b];
    let c = pts[t.c];
    let adx = a.0 - p.0;
    let ady = a.1 - p.1;
    let bdx = b.0 - p.0;
    let bdy = b.1 - p.1;
    let cdx = c.0 - p.0;
    let cdy = c.1 - p.1;
    let al = adx * adx + ady * ady;
    let bl = bdx * bdx + bdy * bdy;
    let cl = cdx * cdx + cdy * cdy;
    let det = adx * (bdy * cl - bl * cdy) - ady * (bdx * cl - bl * cdx) + al * (bdx * cdy - bdy * cdx);
    let scale = al.max(bl).max(cl).max(1.0);
    det > 1e-9 * scale
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_in_triangulation(tris: &[Tri], i: usize, j: usize) -> bool {
    tris.iter().any(|t| {
        let e = [(t.a, t.b), (t.b, t.c), (t.c, t.a)];
        e.iter().any(|&(u, v)| (u == i && v == j) || (u == j && v == i))
    })
}

/// 📐 Sloan-style constraint recovery: repeatedly flip a triangulation edge that properly crosses
/// segment `(i, j)` (and whose quad is convex, so the flip is valid) until `(i, j)` itself is an
/// edge, or no more flips are found (best-effort — leaves whatever partial recovery it achieved
/// rather than failing the whole tessellation over one pathological edge).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn recover_edge(pts: &[(f64, f64)], tris: &mut [Tri], i: usize, j: usize) {
    if i == j {
        return;
    }
    let cap = tris.len() * 8 + 40;
    let mut guard = 0;
    while !edge_in_triangulation(tris, i, j) && guard < cap {
        guard += 1;
        if !try_flip_towards(pts, tris, i, j) {
            break;
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn try_flip_towards(pts: &[(f64, f64)], tris: &mut [Tri], i: usize, j: usize) -> bool {
    let pi = pts[i];
    let pj = pts[j];
    for a in 0..tris.len() {
        let ta = tris[a];
        let edges = [(ta.a, ta.b, ta.c), (ta.b, ta.c, ta.a), (ta.c, ta.a, ta.b)];
        for &(u, v, opp1) in &edges {
            if u == i || u == j || v == i || v == j {
                continue;
            }
            if !segments_properly_intersect(pts[u], pts[v], pi, pj) {
                continue;
            }
            if let Some((b, opp2)) = find_opposite(tris, a, u, v) {
                if is_convex_quad(&[pts[u], pts[opp2], pts[v], pts[opp1]]) {
                    let t1 = ensure_ccw(pts, Tri { a: u, b: opp2, c: opp1 });
                    let t2 = ensure_ccw(pts, Tri { a: opp2, b: v, c: opp1 });
                    tris[a] = t1;
                    tris[b] = t2;
                    return true;
                }
            }
        }
    }
    false
}

/// 📐 The triangle (besides `skip`) sharing directed edge `(v, u)` — i.e. the neighbor across
/// `skip`'s `(u, v)` edge — plus that neighbor's opposite (non-shared) vertex.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_opposite(tris: &[Tri], skip: usize, u: usize, v: usize) -> Option<(usize, usize)> {
    for (b, t) in tris.iter().enumerate() {
        if b == skip {
            continue;
        }
        if t.a == v && t.b == u {
            return Some((b, t.c));
        }
        if t.b == v && t.c == u {
            return Some((b, t.a));
        }
        if t.c == v && t.a == u {
            return Some((b, t.b));
        }
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_convex_quad(q: &[(f64, f64); 4]) -> bool {
    let mut sign = 0.0_f64;
    for i in 0..4 {
        let a = q[i];
        let b = q[(i + 1) % 4];
        let c = q[(i + 2) % 4];
        let cross = (b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0);
        if cross.abs() < 1e-14 {
            return false;
        }
        if sign == 0.0 {
            sign = cross.signum();
        } else if cross.signum() != sign {
            return false;
        }
    }
    true
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segments_properly_intersect(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), p4: (f64, f64)) -> bool {
    let d1 = orient_sign(p3, p4, p1);
    let d2 = orient_sign(p3, p4, p2);
    let d3 = orient_sign(p1, p2, p3);
    let d4 = orient_sign(p1, p2, p4);
    d1 != 0 && d2 != 0 && d3 != 0 && d4 != 0 && d1 != d2 && d3 != d4
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn centroid3(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> (f64, f64) {
    ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_in_polygon(ring: &[(f64, f64)], p: (f64, f64)) -> bool {
    if ring.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = ring.len() - 1;
    for i in 0..ring.len() {
        let (ui, vi) = ring[i];
        let (uj, vj) = ring[j];
        if ((vi > p.1) != (vj > p.1)) && (p.0 < (uj - ui) * (p.1 - vi) / (vj - vi) + ui) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn flatten_tris(tris: &[Tri]) -> Vec<u32> {
    let mut out = Vec::with_capacity(tris.len() * 3);
    for t in tris {
        out.push(t.a as u32);
        out.push(t.b as u32);
        out.push(t.c as u32);
    }
    out
}

/// 📐 Adaptive interior refinement: while any triangle's chordal or angular deviation exceeds
/// tolerance, insert a Steiner point at the surface point for that triangle's UV centroid (always
/// strictly interior, never touching a constrained boundary/hole edge) and re-insert it via
/// [`insert_point_into`], bounded by [`MAX_REFINE_ITERS`] passes and [`MAX_INTERIOR_POINTS`] total.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn refine_adaptive(surface: &Surface, positions: &mut Vec<Pnt3>, uvs: &mut Vec<(f64, f64)>, poles: &mut Vec<bool>, tris: &mut Vec<Tri>, deflection: f64, angular_tol: f64) {
    for _ in 0..MAX_REFINE_ITERS {
        let mut candidates: Vec<(f64, f64)> = Vec::new();
        for &t in tris.iter() {
            if let Some(uv) = triangle_needs_refine(surface, positions, uvs, t, deflection, angular_tol) {
                candidates.push(uv);
            }
        }
        if candidates.is_empty() || positions.len() >= MAX_INTERIOR_POINTS {
            break;
        }
        let mut inserted_any = false;
        for uv in candidates {
            if positions.len() >= MAX_INTERIOR_POINTS {
                break;
            }
            if too_close(uvs, uv, 1e-6) {
                continue;
            }
            let p3 = surface.eval(uv.0, uv.1);
            let idx = positions.len();
            positions.push(p3);
            uvs.push(uv);
            poles.push(surface.is_degenerate_uv(uv.0, uv.1));
            insert_point_into(uvs, tris, idx);
            inserted_any = true;
        }
        if !inserted_any {
            break;
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn too_close(uvs: &[(f64, f64)], p: (f64, f64), tol: f64) -> bool {
    uvs.iter().any(|&q| (q.0 - p.0).abs() < tol && (q.1 - p.1).abs() < tol)
}

/// 📐 A triangle needs refining if either its worst edge-midpoint chordal deviation from the true
/// surface, or the angle between two of its vertices' surface normals, exceeds tolerance — in
/// which case the candidate Steiner point is the surface point at its UV centroid.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn triangle_needs_refine(surface: &Surface, positions: &[Pnt3], uvs: &[(f64, f64)], t: Tri, deflection: f64, angular_tol: f64) -> Option<(f64, f64)> {
    let pts3 = [positions[t.a], positions[t.b], positions[t.c]];
    let uv3 = [uvs[t.a], uvs[t.b], uvs[t.c]];
    let mut worst_chordal = 0.0_f64;
    for e in 0..3 {
        let (p0, uv0) = (pts3[e], uv3[e]);
        let (p1, uv1) = (pts3[(e + 1) % 3], uv3[(e + 1) % 3]);
        let mid_uv = (0.5 * (uv0.0 + uv1.0), 0.5 * (uv0.1 + uv1.1));
        let dev = surface.eval(mid_uv.0, mid_uv.1).distance(p0.lerp(p1, 0.5));
        worst_chordal = worst_chordal.max(dev);
    }
    let normals: Vec<Vec3> = uv3.iter().filter_map(|&uv| surface.normal(uv.0, uv.1)).collect();
    let mut worst_angular = 0.0_f64;
    for i in 0..normals.len() {
        for j in (i + 1)..normals.len() {
            worst_angular = worst_angular.max(normals[i].angle_to(normals[j]));
        }
    }
    if worst_chordal > deflection || worst_angular > angular_tol {
        Some(centroid3(uv3[0], uv3[1], uv3[2]))
    } else {
        None
    }
}
// #endregion ▲Triangulate
