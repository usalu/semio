//! 🧊️ Face meshing: the face mesh assembly with its loop UV collection, seam and pole handling, the tessellation report measurement, vertex normals, winding fix and vertex welding. See [`super`].

use super::*;

// #region 🧊FaceTessellate
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn append_face_mesh(transfer: &mut MeshTransfer, report: &mut TessellationReport, body: &Body, face_id: FaceId, deflection: f64, edge_cache: &EdgeSampleCache) -> Result<(), KernelError> {
    let face = body.faces.get(face_id).ok_or_else(|| KernelError::MissingEntity(face_id.to_string()))?;
    let surface = body.surfaces.get(face.surface).ok_or_else(|| KernelError::MissingEntity(face.surface.to_string()))?;
    let Some(outer_id) = face.outer else {
        return Err(KernelError::InvalidInput(format!("face {face_id} has no outer loop")));
    };
    let (mut boundary_pos, mut boundary_uv, mut boundary_pole) = collect_loop_uv(body, outer_id, surface, edge_cache)?;
    remove_closing_duplicate_uv(&mut boundary_pos, &mut boundary_uv, &mut boundary_pole);
    split_pole_branches(&mut boundary_pos, &mut boundary_uv, &mut boundary_pole);
    if boundary_pos.len() < 3 {
        return Err(KernelError::Operation(format!("face {face_id} outer loop degenerated to {} points", boundary_pos.len())));
    }
    let outer_count = boundary_pos.len();
    let mut positions = boundary_pos;
    let mut uvs = boundary_uv;
    let mut poles = boundary_pole;
    let mut hole_ranges: Vec<(usize, usize)> = Vec::new();
    for &inner_id in &face.inners {
        let (mut hole_pos, mut hole_uv, mut hole_pole) = collect_loop_uv(body, inner_id, surface, edge_cache)?;
        remove_closing_duplicate_uv(&mut hole_pos, &mut hole_uv, &mut hole_pole);
        split_pole_branches(&mut hole_pos, &mut hole_uv, &mut hole_pole);
        if hole_pos.len() < 3 {
            continue;
        }
        let start = positions.len();
        positions.extend(hole_pos);
        uvs.extend(hole_uv);
        poles.extend(hole_pole);
        hole_ranges.push((start, positions.len()));
    }
    let mut tris = build_constrained_triangulation(&uvs, (0, outer_count), &hole_ranges)?;
    if !surface.is_planar() {
        refine_adaptive(surface, &mut positions, &mut uvs, &mut poles, &mut tris, deflection, DEFAULT_ANGULAR_TOL);
    }
    weld_and_compact(&mut positions, &mut uvs, &poles, &mut tris, POLE_WELD_TOL);
    let mut indices = flatten_tris(&tris);
    let flipped = face.flipped;
    let desired_at = |uv: (f64, f64)| -> Vec3 {
        let mut n = surface.normal(uv.0, uv.1).unwrap_or(Vec3::Z);
        if flipped {
            n = -n;
        }
        n
    };
    fix_winding_per_triangle(&positions, &uvs, &mut indices, &desired_at);
    let (max_chordal, max_angular) = measure_report(surface, &positions, &uvs, &indices);
    report.max_chordal = report.max_chordal.max(max_chordal);
    report.max_angular = report.max_angular.max(max_angular);
    let base = (transfer.position.len() / 3) as u32;
    let tri_start = transfer.index.len() as u32;
    for (i, p) in positions.iter().enumerate() {
        transfer.position.extend([p.x as f32, p.y as f32, p.z as f32]);
        let n = vertex_normal(surface, flipped, uvs[i], &positions, &indices, i);
        transfer.normal.extend([n.x as f32, n.y as f32, n.z as f32]);
    }
    for idx in &indices {
        transfer.index.push(base + *idx);
    }
    let mut area = 0.0;
    for tri in indices.as_chunks::<3>().0 {
        let a = positions[tri[0] as usize];
        let b = positions[tri[1] as usize];
        let c = positions[tri[2] as usize];
        area += (b - a).cross(c - a).norm() * 0.5;
    }
    let label = face.label.0.to_string();
    let normal_first = desired_at(uvs.first().copied().unwrap_or((0.0, 0.0)));
    transfer.face_groups.push(FaceGroup { start: tri_start, count: indices.len() as u32, entity_id: label.clone() });
    transfer.face_infos.push(FaceInfo { entity_id: label, surface_kind: surface_kind_of(surface), area, normal: normal_first.to_array() });
    Ok(())
}

/// 🧭 Walks one loop's coedges, reusing the shared edge cache for 3D positions and resolving each
/// point's `(u, v)` from the coedge's stored p-curve when present (`Coedge::pcurve`), else via
/// surface closest-point projection — then unwraps across periodic seams (choosing the UV branch
/// nearest the previous sample) and pins the arbitrary `u` at poles/apexes to the previous branch
/// so a boundary loop that touches a singularity stays a single well-formed ring vertex there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn collect_loop_uv(body: &Body, loop_id: LoopId, surface: &Surface, edge_cache: &EdgeSampleCache) -> Result<TessellationLoopUv, KernelError> {
    let mut positions: Vec<Pnt3> = Vec::new();
    let mut uvs: Vec<(f64, f64)> = Vec::new();
    let mut poles: Vec<bool> = Vec::new();
    let mut prev: Option<(f64, f64)> = None;
    let mut prev_is_pole = false;
    for coedge_id in body.loop_coedges(loop_id) {
        let coedge = body.coedges.get(coedge_id).ok_or_else(|| KernelError::MissingEntity(coedge_id.to_string()))?;
        let edge = body.edges.get(coedge.edge).ok_or_else(|| KernelError::MissingEntity(coedge.edge.to_string()))?;
        let samples = edge_cache.get(&coedge.edge).ok_or_else(|| KernelError::Operation(format!("missing edge sample for {}", coedge.edge)))?;
        let ordered: Vec<(f64, Pnt3)> = if coedge.forward { samples.clone() } else { samples.iter().rev().copied().collect() };
        for (i, &(t, p)) in ordered.iter().enumerate() {
            if i == 0 {
                if let Some(&last) = positions.last() {
                    if last.distance(p) <= ENDPOINT_TOL {
                        continue;
                    }
                }
            }
            let (raw_uv, is_pole) = coedge_point_uv(body, surface, coedge, edge, t, p);
            // A pole's own `u` is meaningless and gets pinned to whatever branch we arrived from
            // (below) — but that pinned value must NOT then anchor the departing seam's own branch
            // once we leave the pole: a lune-shaped face (sphere/cone) legitimately re-enters the
            // seam at the OPPOSITE `u` branch after a pole (e.g. `0` in, `2π` out), and treating the
            // pinned pole value as still-live continuity would wrongly snap that outgoing branch
            // back to the incoming one, collapsing the whole loop onto a single meridian.
            let unwrap_ref = if prev_is_pole { None } else { prev };
            let uv = unwrap_uv(surface, raw_uv, unwrap_ref, is_pole);
            prev = Some(uv);
            prev_is_pole = is_pole;
            positions.push(p);
            uvs.push(uv);
            poles.push(is_pole);
        }
    }
    Ok((positions, uvs, poles))
}

/// 🧭 `(u, v)` for one edge sample: the coedge's stored p-curve reparametrized from the edge's own
/// `t` via `Coedge::prange`/`Edge::range` when present, else `surface_ops::closest_uv`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_point_uv(body: &Body, surface: &Surface, coedge: &Coedge, edge: &Edge, t: f64, p: Pnt3) -> ((f64, f64), bool) {
    let uv = match coedge.pcurve.and_then(|id| body.curves2.get(id)) {
        Some(pcurve) => {
            let (er0, er1) = edge.range;
            let (pr0, pr1) = coedge.prange;
            let frac = if (er1 - er0).abs() > 1e-14 { (t - er0) / (er1 - er0) } else { 0.0 };
            let pt = pr0 + frac * (pr1 - pr0);
            let uv2 = pcurve.eval(pt);
            (uv2.x, uv2.y)
        }
        None => {
            let closest = surface_ops::closest_uv(surface, surface.domain(), p, 1e-9);
            (closest.u, closest.v)
        }
    };
    let is_pole = surface.is_degenerate_uv(uv.0, uv.1);
    (uv, is_pole)
}

/// 🧭 Picks the branch `uv ± k·period` nearest `prev` for each periodic coordinate (so a boundary
/// loop crossing a seam stays continuous in UV instead of jumping); at a pole/apex `u` is
/// meaningless, so it's pinned to `prev.0` instead of being (arbitrarily) unwrapped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_uv(surface: &Surface, mut uv: (f64, f64), prev: Option<(f64, f64)>, is_pole: bool) -> (f64, f64) {
    if let Some(prev) = prev {
        if is_pole {
            uv.0 = prev.0;
            return uv;
        }
        if surface.is_u_periodic() {
            uv.0 = nearest_branch(uv.0, prev.0, std::f64::consts::TAU);
        }
        if surface.is_v_periodic() {
            uv.1 = nearest_branch(uv.1, prev.1, std::f64::consts::TAU);
        }
    }
    uv
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nearest_branch(x: f64, reference: f64, period: f64) -> f64 {
    let k = ((reference - x) / period).round();
    x + k * period
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remove_closing_duplicate_uv(positions: &mut Vec<Pnt3>, uvs: &mut Vec<(f64, f64)>, poles: &mut Vec<bool>) {
    if positions.len() > 2 {
        if let (Some(&first), Some(&last)) = (positions.first(), positions.last()) {
            if first.distance(last) <= ENDPOINT_TOL {
                positions.pop();
                uvs.pop();
                poles.pop();
            }
        }
    }
}

/// 🧭 Splits every pole vertex whose two ring neighbours sit on different `u` branches into one ring vertex per branch
/// (the same 3D point twice, both flagged pole — [`weld_and_compact`] fans them back into one vertex). A face whose
/// boundary passes a pole WITHOUT an explicit degenerate edge — a cone's apex, whose single seam is walked up and back
/// down, or two meridians meeting at an apex — arrives on one branch and departs on another; left as one UV vertex, the
/// ring closed through a diagonal across the whole `u` range instead of along the degenerate side `v = v_pole`, and the
/// triangulation dropped the corner it cut off (every cone lost ~21 % of its volume).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_pole_branches(positions: &mut Vec<Pnt3>, uvs: &mut Vec<(f64, f64)>, poles: &mut Vec<bool>) {
    let n = uvs.len();
    if n < 3 {
        return;
    }
    let mut split_positions = Vec::with_capacity(n + 2);
    let mut split_uvs = Vec::with_capacity(n + 2);
    let mut split_poles = Vec::with_capacity(n + 2);
    for i in 0..n {
        let (arriving, departing) = (uvs[(i + n - 1) % n], uvs[(i + 1) % n]);
        let branches = poles[i] && !poles[(i + n - 1) % n] && !poles[(i + 1) % n] && (arriving.0 - departing.0).abs() > POLE_BRANCH_TOL;
        if branches {
            split_positions.extend([positions[i], positions[i]]);
            split_uvs.extend([(arriving.0, uvs[i].1), (departing.0, uvs[i].1)]);
            split_poles.extend([true, true]);
        } else {
            split_positions.push(positions[i]);
            split_uvs.push(uvs[i]);
            split_poles.push(poles[i]);
        }
    }
    *positions = split_positions;
    *uvs = split_uvs;
    *poles = split_poles;
}

/// 📐 Sums the deviation-at-edge-midpoint (chordal) and normal-angle-between-vertices (angular)
/// across every triangle's edges — the same metric [`triangle_needs_refine`] converges against,
/// so on a converged mesh this reports (at worst) the tolerance the caller asked for.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn measure_report(surface: &Surface, positions: &[Pnt3], uvs: &[(f64, f64)], indices: &[u32]) -> (f64, f64) {
    let mut max_chordal = 0.0_f64;
    let mut max_angular = 0.0_f64;
    for tri in indices.as_chunks::<3>().0 {
        let idx = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
        let pts3 = [positions[idx[0]], positions[idx[1]], positions[idx[2]]];
        let uv3 = [uvs[idx[0]], uvs[idx[1]], uvs[idx[2]]];
        for e in 0..3 {
            let (p0, uv0) = (pts3[e], uv3[e]);
            let (p1, uv1) = (pts3[(e + 1) % 3], uv3[(e + 1) % 3]);
            let mid_uv = (0.5 * (uv0.0 + uv1.0), 0.5 * (uv0.1 + uv1.1));
            let dev = surface.eval(mid_uv.0, mid_uv.1).distance(p0.lerp(p1, 0.5));
            max_chordal = max_chordal.max(dev);
        }
        let normals: Vec<Vec3> = uv3.iter().filter_map(|&uv| surface.normal(uv.0, uv.1)).collect();
        for i in 0..normals.len() {
            for j in (i + 1)..normals.len() {
                max_angular = max_angular.max(normals[i].angle_to(normals[j]));
            }
        }
    }
    (max_chordal, max_angular)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vertex_normal(surface: &Surface, flipped: bool, uv: (f64, f64), positions: &[Pnt3], indices: &[u32], vertex: usize) -> Vec3 {
    if let Some(mut n) = surface.normal(uv.0, uv.1) {
        if flipped {
            n = -n;
        }
        return n;
    }
    let mut accum = Vec3::ZERO;
    for tri in indices.as_chunks::<3>().0 {
        if tri.iter().any(|&i| i as usize == vertex) {
            let a = positions[tri[0] as usize];
            let b = positions[tri[1] as usize];
            let c = positions[tri[2] as usize];
            accum = accum + (b - a).cross(c - a);
        }
    }
    accum.normalized().unwrap_or(Vec3::Z)
}

/// 📐 Per-triangle winding fix (no global flip heuristic): each triangle's own 3D cross product is
/// compared against the surface's own normal at that triangle's UV centroid, so triangles on
/// different parts of a curved face are each judged against their own local normal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fix_winding_per_triangle(positions: &[Pnt3], uvs: &[(f64, f64)], indices: &mut [u32], desired_at: &dyn Fn((f64, f64)) -> Vec3) {
    for tri in indices.as_chunks_mut::<3>().0 {
        let (ia, ib, ic) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
        let a = positions[ia];
        let b = positions[ib];
        let c = positions[ic];
        let uv_c = centroid3(uvs[ia], uvs[ib], uvs[ic]);
        let desired = desired_at(uv_c);
        let actual = (b - a).cross(c - a);
        if actual.dot(desired) < 0.0 {
            tri.swap(1, 2);
        }
    }
}

/// 🫧 Collapses coincident 3D vertices that are BOTH flagged [`is_pole`](coedge_point_uv) (poles,
/// cone apexes — any UV-distinct samples that evaluate to the same 3D point because the surface's
/// own parametrization is degenerate there) into one, dropping the now-degenerate triangles this
/// creates. Since every triangle that touched the ring around the singularity now shares the one
/// welded vertex, this *is* the "fan around the pole" the CDT itself has no notion of. Crucially
/// this must NOT weld two non-pole seam-duplicate vertices (e.g. a cylinder's `u=0`/`u=2π` corner,
/// which lands on the exact same 3D point by periodicity but needs its own distinct UV on each
/// side of the seam) — welding those collapses two legitimately-separate boundary rectangle
/// corners into one, letting later triangles reference the wrong UV and blow up chordal deviation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn weld_and_compact(positions: &mut Vec<Pnt3>, uvs: &mut Vec<(f64, f64)>, poles: &[bool], tris: &mut Vec<Tri>, weld_tol: f64) {
    let n = positions.len();
    let cell = weld_tol.max(1e-12);
    let key = |p: Pnt3| -> (i64, i64, i64) { ((p.x / cell).round() as i64, (p.y / cell).round() as i64, (p.z / cell).round() as i64) };
    let mut buckets: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    let mut remap: Vec<usize> = (0..n).collect();
    for i in 0..n {
        if !poles[i] {
            let (kx, ky, kz) = key(positions[i]);
            buckets.entry((kx, ky, kz)).or_default().push(i);
            continue;
        }
        let (kx, ky, kz) = key(positions[i]);
        let mut found: Option<usize> = None;
        'outer: for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if let Some(list) = buckets.get(&(kx + dx, ky + dy, kz + dz)) {
                        for &j in list {
                            if poles[j] && positions[remap[j]].distance(positions[i]) <= weld_tol {
                                found = Some(remap[j]);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        match found {
            Some(rep) => remap[i] = rep,
            None => buckets.entry((kx, ky, kz)).or_default().push(i),
        }
    }
    for t in tris.iter_mut() {
        t.a = remap[t.a];
        t.b = remap[t.b];
        t.c = remap[t.c];
    }
    tris.retain(|t| t.a != t.b && t.b != t.c && t.a != t.c);
    let mut used = vec![false; n];
    for t in tris.iter() {
        used[t.a] = true;
        used[t.b] = true;
        used[t.c] = true;
    }
    let mut new_index = vec![usize::MAX; n];
    let mut new_positions = Vec::new();
    let mut new_uvs = Vec::new();
    for i in 0..n {
        if used[i] {
            new_index[i] = new_positions.len();
            new_positions.push(positions[i]);
            new_uvs.push(uvs[i]);
        }
    }
    for t in tris.iter_mut() {
        t.a = new_index[t.a];
        t.b = new_index[t.b];
        t.c = new_index[t.c];
    }
    *positions = new_positions;
    *uvs = new_uvs;
}
// #endregion 🧊FaceTessellate
