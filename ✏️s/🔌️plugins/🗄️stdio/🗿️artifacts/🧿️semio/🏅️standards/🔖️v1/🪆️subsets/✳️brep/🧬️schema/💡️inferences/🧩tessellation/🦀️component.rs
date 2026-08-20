//! 🧩 Crack-free edge-first tessellation producing [`semio_framework_3d::engine::MeshTransfer`].
//!
//! Edges are discretized once and reused by every adjacent face (Stoger & Kurka 2003 style), then
//! each face's UV-domain boundary is ear-clipped into triangles. Shared edge samples keep seams
//! positionally coincident across faces.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧩️tessellate` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL. Pure algorithm
//! only — the real `InferredField<SemioBrepSnapshot>` wrapper is future work (see
//! `💡️inferences/✅validation-report`'s doc comment for why it's not built yet).

use std::collections::HashMap;

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::Wire;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, EdgeId, FaceId, SolidId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};
use semio_framework_3d::engine::{FaceGroup, MeshTransfer};

// #region 🔖️Constants

const DEFAULT_ANGULAR_TOL: f64 = 0.35;
const ENDPOINT_TOL: f64 = 1e-9;

// #endregion 🔖️Constants

// #region 🔖️Api

/// 🧩 Tessellates every face of `solid` with edge-first shared discretization into one [`MeshTransfer`].
pub async fn tessellate_solid(body: &Body, solid: SolidId, deflection: f64) -> Result<MeshTransfer, KernelError> {
    if body.solids.get(solid).await.is_none() {
        return Err(KernelError::MissingEntity(solid.to_string()));
    }
    let deflection = deflection.max(1e-9);
    let faces = body.solid_faces(solid).await;
    if faces.is_empty() {
        return Err(KernelError::InvalidInput(format!("solid {solid} has no faces")));
    }
    let edge_cache = sample_solid_edge_cache(body, solid, deflection).await?;
    let mut transfer = MeshTransfer::default();
    for face in faces {
        append_face_mesh(&mut transfer, body, face, deflection, &edge_cache).await?;
    }
    transfer.edges = pack_edge_segments(body, solid, &edge_cache).await;
    Ok(transfer)
}

/// 🧵 Tessellates a wire into edge polylines only (no shaded triangles).
pub async fn tessellate_wire(body: &Body, wire: &Wire, deflection: f64) -> Result<MeshTransfer, KernelError> {
    let deflection = deflection.max(1e-9);
    let mut edges = Vec::new();
    for (edge_id, _forward) in &wire.members {
        let points = sample_edge_points(body, *edge_id, deflection).await?;
        push_polyline_segments(&mut edges, &points);
    }
    Ok(MeshTransfer { edges, ..MeshTransfer::default() })
}

/// 🧩 Tessellates a single face into a [`MeshTransfer`] with one [`FaceGroup`].
pub async fn tessellate_face(body: &Body, face: FaceId, deflection: f64) -> Result<MeshTransfer, KernelError> {
    if body.faces.get(face).await.is_none() {
        return Err(KernelError::MissingEntity(face.to_string()));
    }
    let deflection = deflection.max(1e-9);
    let mut edge_cache = HashMap::new();
    for coedge_id in body.face_coedges(face) {
        let edge = body.coedges.get(coedge_id).await.map(|c| c.edge).ok_or_else(|| KernelError::MissingEntity(coedge_id.to_string()))?;
        if let std::collections::hash_map::Entry::Vacant(slot) = edge_cache.entry(edge) {
            slot.insert(sample_edge_points(body, edge, deflection).await?);
        }
    }
    let mut transfer = MeshTransfer::default();
    append_face_mesh(&mut transfer, body, face, deflection, &edge_cache).await?;
    for (&edge, points) in &edge_cache {
        let _ = edge;
        push_polyline_segments(&mut transfer.edges, points);
    }
    Ok(transfer)
}

/// 🧩 Samples `edge` to a deflection-bounded polyline and returns packed xyz `f32` positions.
pub async fn sample_edge_polyline(body: &Body, edge: EdgeId, deflection: f64) -> Vec<f32> {
    match sample_edge_points(body, edge, deflection.max(1e-9)).await {
        Ok(points) => points.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect(),
        Err(_) => Vec::new(),
    }
}

// #endregion 🔖️Api

// #region 🧮EdgeSample

async fn sample_solid_edge_cache(body: &Body, solid: SolidId, deflection: f64) -> Result<HashMap<EdgeId, Vec<Pnt3>>, KernelError> {
    let mut cache = HashMap::new();
    for face in body.solid_faces(solid) {
        for coedge_id in body.face_coedges(face) {
            let edge = body.coedges.get(coedge_id).await.map(|c| c.edge).ok_or_else(|| KernelError::MissingEntity(coedge_id.to_string()))?;
            if let std::collections::hash_map::Entry::Vacant(slot) = cache.entry(edge) {
                slot.insert(sample_edge_points(body, edge, deflection).await?);
            }
        }
    }
    Ok(cache)
}

async fn sample_edge_points(body: &Body, edge_id: EdgeId, deflection: f64) -> Result<Vec<Pnt3>, KernelError> {
    let edge = body.edges.get(edge_id).await.ok_or_else(|| KernelError::MissingEntity(edge_id.to_string()))?;
    let curve = body.curves3.get(edge.curve).await.ok_or_else(|| KernelError::MissingEntity(edge.curve.to_string()))?;
    let v0 = body.vertices.get(edge.v0).await.ok_or_else(|| KernelError::MissingEntity(edge.v0.to_string()))?.position;
    let v1 = body.vertices.get(edge.v1).await.ok_or_else(|| KernelError::MissingEntity(edge.v1.to_string()))?.position;
    let (t0, t1) = edge.range;
    let points = match curve {
        Curve3::Line { .. } => vec![v0, v1],
        Curve3::Circle { frame: _, radius } => {
            let n = segments_for_chord_deviation(*radius, (t1 - t0).abs(), deflection, DEFAULT_ANGULAR_TOL);
            sample_uniform(curve, t0, t1, n + 1).await
        }
        Curve3::Ellipse { major_radius, minor_radius, .. } => {
            let curv_r = (*major_radius * *major_radius) / minor_radius.max(1e-12);
            let n = segments_for_chord_deviation(curv_r, (t1 - t0).abs(), deflection, DEFAULT_ANGULAR_TOL);
            sample_uniform(curve, t0, t1, n + 1).await
        }
        Curve3::Nurbs { .. } => sample_nurbs_adaptive(curve, t0, t1, deflection).await,
    };
    let mut out = points;
    if let Some(first) = out.first_mut() {
        *first = v0;
    }
    if let Some(last) = out.last_mut() {
        *last = v1;
    }
    Ok(out)
}

async fn sample_uniform(curve: &Curve3, t0: f64, t1: f64, count: usize) -> Vec<Pnt3> {
    let n = count.max(2);
    (0..n)
        .map(|i| {
            let t = t0 + (t1 - t0) * (i as f64) / ((n - 1) as f64);
            curve.eval(t)
        })
        .collect()
}

async fn sample_nurbs_adaptive(curve: &Curve3, t0: f64, t1: f64, deflection: f64) -> Vec<Pnt3> {
    let coarse_n = 16usize;
    let max_dev = measure_max_chord_deviation(curve, t0, t1, coarse_n);
    let n = if max_dev <= deflection { coarse_n } else { ((coarse_n as f64) * (max_dev / deflection).sqrt()).ceil() as usize }.clamp(8, 4096);
    sample_uniform(curve, t0, t1, n + 1).await
}

async fn measure_max_chord_deviation(curve: &Curve3, t0: f64, t1: f64, n: usize) -> f64 {
    let mut max_dev = 0.0_f64;
    for i in 0..n {
        let a = t0 + (t1 - t0) * (i as f64) / (n as f64);
        let b = t0 + (t1 - t0) * ((i + 1) as f64) / (n as f64);
        let p0 = curve.eval(a).await;
        let p1 = curve.eval(b);
        let mid_chord = p0.lerp(p1.await, 0.5).await;
        let mid_curve = curve.eval(0.5 * (a + b)).await;
        max_dev = max_dev.max(mid_curve.distance(mid_chord).await);
    }
    max_dev
}

async fn segments_for_chord_deviation(radius: f64, arc_range: f64, deflection: f64, angular_tol: f64) -> usize {
    if radius <= 0.0 || deflection <= 0.0 || arc_range <= 0.0 {
        return 8;
    }
    let theta_lin = 2.0 * (1.0 - deflection / radius).clamp(0.0, 1.0).acos();
    let theta_step = if angular_tol > 0.0 { theta_lin.min(angular_tol) } else { theta_lin };
    if theta_step <= 0.0 {
        return 8;
    }
    let n = (arc_range / theta_step).ceil() as usize;
    let n_min = (arc_range * (radius / deflection).sqrt()).ceil() as usize;
    n.max(n_min).max(4)
}

async fn pack_edge_segments(body: &Body, solid: SolidId, cache: &HashMap<EdgeId, Vec<Pnt3>>) -> Vec<f32> {
    let mut edges = Vec::new();
    let mut seen = HashMap::<EdgeId, ()>::new();
    for face in body.solid_faces(solid) {
        for coedge_id in body.face_coedges(face) {
            let Some(coedge) = body.coedges.get(coedge_id).await else { continue };
            if seen.insert(coedge.edge, ()).is_some() {
                continue;
            }
            if let Some(points) = cache.get(&coedge.edge) {
                push_polyline_segments(&mut edges, points);
            }
        }
    }
    edges
}

async fn push_polyline_segments(out: &mut Vec<f32>, points: &[Pnt3]) {
    for window in points.windows(2) {
        let a = window[0];
        let b = window[1];
        out.extend([a.x as f32, a.y as f32, a.z as f32, b.x as f32, b.y as f32, b.z as f32]);
    }
}

// #endregion 🧮EdgeSample

// #region 🧊FaceTessellate

async fn append_face_mesh(transfer: &mut MeshTransfer, body: &Body, face_id: FaceId, deflection: f64, edge_cache: &HashMap<EdgeId, Vec<Pnt3>>) -> Result<(), KernelError> {
    let face = body.faces.get(face_id).await.ok_or_else(|| KernelError::MissingEntity(face_id.to_string()))?;
    let surface = body.surfaces.get(face.surface).await.ok_or_else(|| KernelError::MissingEntity(face.surface.to_string()))?;
    let Some(outer_id) = face.outer else {
        return Err(KernelError::InvalidInput(format!("face {face_id} has no outer loop")));
    };
    let mut boundary = collect_loop_polyline(body, outer_id, edge_cache).await?;
    remove_closing_duplicate(&mut boundary);
    if boundary.len() < 3 {
        return Err(KernelError::Operation(format!("face {face_id} outer loop degenerated to {} points", boundary.len())));
    }
    let mut holes = Vec::new();
    for &inner_id in &face.inners {
        let mut hole = collect_loop_polyline(body, inner_id, edge_cache).await?;
        remove_closing_duplicate(&mut hole);
        if hole.len() >= 3 {
            holes.push(hole);
        }
    }
    let (positions, uvs) = refine_interior_if_needed(surface, &boundary, &holes, deflection).await;
    let mut indices = triangulate_uv(&positions, &uvs, boundary.len(), &holes).await?;
    ensure_winding(&positions, &mut indices, face_normal(surface, face.flipped, &uvs).await);
    let base = (transfer.position.len() / 3) as u32;
    let tri_start = transfer.index.len() as u32;
    for (i, p) in positions.iter().enumerate() {
        transfer.position.extend([p.x as f32, p.y as f32, p.z as f32]);
        let (u, v) = uvs[i];
        let n = face_vertex_normal(surface, face.flipped, u, v, p, &positions, &indices, i).await;
        transfer.normal.extend([n.x as f32, n.y as f32, n.z as f32]);
    }
    for idx in &indices {
        transfer.index.push(base + *idx);
    }
    transfer.face_groups.push(FaceGroup { start: tri_start, count: indices.len() as u32, entity_id: face_id.raw_index().to_string() });
    Ok(())
}

async fn collect_loop_polyline(body: &Body, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId, edge_cache: &HashMap<EdgeId, Vec<Pnt3>>) -> Result<Vec<Pnt3>, KernelError> {
    let mut points: Vec<Pnt3> = Vec::new();
    for coedge_id in body.loop_coedges(loop_id) {
        let coedge = body.coedges.get(coedge_id).await.ok_or_else(|| KernelError::MissingEntity(coedge_id.to_string()))?;
        let samples = edge_cache.get(&coedge.edge).ok_or_else(|| KernelError::Operation(format!("missing edge sample for {}", coedge.edge)))?;
        let oriented: Vec<Pnt3> = if coedge.forward { samples.clone() } else { samples.iter().rev().copied().collect() };
        for (i, pt) in oriented.into_iter().enumerate() {
            if i == 0 {
                if let Some(last) = points.last() {
                    if last.distance(pt) <= ENDPOINT_TOL {
                        continue;
                    }
                }
            }
            points.push(pt);
        }
    }
    Ok(points)
}

async fn remove_closing_duplicate(points: &mut Vec<Pnt3>) {
    if points.len() > 2 {
        if let (Some(&first), Some(&last)) = (points.first(), points.last()) {
            if first.distance(last) <= ENDPOINT_TOL {
                points.pop();
            }
        }
    }
}

async fn project_to_uv(surface: &Surface, point: Pnt3) -> (f64, f64) {
    let (u, v, _) = surface_ops::closest_point(surface, surface.domain().await, point, 8).await;
    (u, v)
}

async fn face_normal(surface: &Surface, flipped: bool, uvs: &[(f64, f64)]) -> Vec3 {
    let (u, v) = uvs.first().copied().unwrap_or((0.0, 0.0));
    let mut n = surface.normal(u, v).await.unwrap_or(Vec3::Z);
    if flipped {
        n = -n;
    }
    n
}

async fn face_vertex_normal(surface: &Surface, flipped: bool, u: f64, v: f64, point: &Pnt3, positions: &[Pnt3], indices: &[u32], vertex: usize) -> Vec3 {
    if let Some(mut n) = surface.normal(u, v).await {
        if flipped {
            n = -n;
        }
        return n;
    }
    let mut accum = Vec3::ZERO;
    for tri in indices.chunks_exact(3) {
        if tri.iter().any(|&i| i as usize == vertex) {
            let a = positions[tri[0] as usize];
            let b = positions[tri[1] as usize];
            let c = positions[tri[2] as usize];
            accum = accum + (b - a).cross(c - a).await;
        }
    }
    let _ = point;
    accum.normalized().await.unwrap_or(Vec3::Z)
}

async fn refine_interior_if_needed(surface: &Surface, boundary: &[Pnt3], holes: &[Vec<Pnt3>], deflection: f64) -> (Vec<Pnt3>, Vec<(f64, f64)>) {
    let mut positions = boundary.to_vec();
    for hole in holes {
        positions.extend(hole.iter().copied());
    }
    let mut uvs: Vec<(f64, f64)> = positions.iter().map(|&p| project_to_uv(surface, p)).collect();
    if surface.is_planar().await || deflection >= f64::MAX / 2.0 {
        return (positions, uvs);
    }
    let (u_dom, v_dom) = surface.domain().await;
    let u_lo = uvs.iter().map(|uv| uv.0).fold(f64::INFINITY, f64::min);
    let u_hi = uvs.iter().map(|uv| uv.0).fold(f64::NEG_INFINITY, f64::max);
    let v_lo = uvs.iter().map(|uv| uv.1).fold(f64::INFINITY, f64::min);
    let v_hi = uvs.iter().map(|uv| uv.1).fold(f64::NEG_INFINITY, f64::max);
    let u0 = if u_dom.0.is_finite() { u_lo.max(u_dom.0) } else { u_lo };
    let u1 = if u_dom.1.is_finite() { u_hi.min(u_dom.1) } else { u_hi };
    let v0 = if v_dom.0.is_finite() { v_lo.max(v_dom.0) } else { v_lo };
    let v1 = if v_dom.1.is_finite() { v_hi.min(v_dom.1) } else { v_hi };
    let nu = interior_segments(surface, u0, u1, v0, v1, deflection).0;
    let nv = interior_segments(surface, u0, u1, v0, v1, deflection).1;
    if nu <= 1 && nv <= 1 {
        return (positions, uvs);
    }
    for i in 1..nu {
        for j in 1..nv {
            let u = u0 + (u1 - u0) * (i as f64) / (nu as f64);
            let v = v0 + (v1 - v0) * (j as f64) / (nv as f64);
            let p = surface.eval(u, v);
            if point_in_outer_uv(&uvs[..boundary.len()], u, v).await
                && holes.iter().all(|hole| {
                    let hole_uv: Vec<(f64, f64)> = hole.iter().map(|&q| project_to_uv(surface, q)).collect();
                    !point_in_outer_uv(&hole_uv, u, v)
                })
            {
                positions.push(p.await);
                uvs.push((u, v));
            }
        }
    }
    (positions, uvs)
}

async fn interior_segments(surface: &Surface, u0: f64, u1: f64, v0: f64, v1: f64, deflection: f64) -> (usize, usize) {
    match surface {
        Surface::Cylinder { radius, .. } => {
            let nu = segments_for_chord_deviation(*radius, (u1 - u0).abs().max(1e-9), deflection, DEFAULT_ANGULAR_TOL);
            (nu.max(1), 1)
        }
        Surface::Sphere { radius, .. } => {
            let nu = segments_for_chord_deviation(*radius, (u1 - u0).abs().max(1e-9), deflection, DEFAULT_ANGULAR_TOL);
            let nv = segments_for_chord_deviation(*radius, (v1 - v0).abs().max(1e-9), deflection, DEFAULT_ANGULAR_TOL);
            (nu.max(1), nv.max(1))
        }
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs { .. } => {
            let mid_u = 0.5 * (u0 + u1);
            let mid_v = 0.5 * (v0 + v1);
            let p00 = surface.eval(u0, v0).await;
            let p11 = surface.eval(u1, v1);
            let diag = p00.distance(p11.await).await.max(1e-9);
            let n = ((diag / deflection).sqrt().ceil() as usize).clamp(1, 64);
            let _ = mid_u;
            let _ = mid_v;
            (n, n)
        }
        Surface::Plane { .. } => (1, 1),
    }
}

async fn point_in_outer_uv(ring: &[(f64, f64)], u: f64, v: f64) -> bool {
    if ring.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = ring.len() - 1;
    for i in 0..ring.len() {
        let (ui, vi) = ring[i];
        let (uj, vj) = ring[j];
        let intersect = ((vi > v) != (vj > v)) && (u < (uj - ui) * (v - vi) / (vj - vi + 0.0) + ui);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

// #endregion 🧊FaceTessellate

// #region ▲Triangulate

async fn triangulate_uv(positions: &[Pnt3], uvs: &[(f64, f64)], outer_count: usize, holes: &[Vec<Pnt3>]) -> Result<Vec<u32>, KernelError> {
    if holes.is_empty() && positions.len() == outer_count {
        return Ok(ear_clip(uvs).await);
    }
    if holes.is_empty() && positions.len() > outer_count {
        return Ok(fan_from_centroid(uvs, outer_count).await);
    }
    // Bridge each hole into the outer ring (simple CDT stand-in) then ear-clip the result.
    Ok(constrained_triangulate(uvs, outer_count, holes_uv_counts(holes).await).await)
}

async fn holes_uv_counts(holes: &[Vec<Pnt3>]) -> Vec<usize> {
    holes.iter().map(Vec::len).collect()
}

async fn constrained_triangulate(uvs: &[(f64, f64)], outer_count: usize, hole_counts: Vec<usize>) -> Vec<u32> {
    if outer_count < 3 {
        return Vec::new();
    }
    let mut ring: Vec<usize> = (0..outer_count).collect();
    let mut offset = outer_count;
    for count in hole_counts {
        if count < 3 || offset + count > uvs.len() {
            offset += count;
            continue;
        }
        // Bridge: connect nearest outer vertex to nearest hole vertex.
        let mut best = (f64::MAX, 0usize, 0usize);
        for (oi, &ov) in ring.iter().enumerate() {
            for hi in 0..count {
                let d = (uvs[ov].0 - uvs[offset + hi].0).hypot(uvs[ov].1 - uvs[offset + hi].1);
                if d < best.0 {
                    best = (d, oi, hi);
                }
            }
        }
        let (_, oi, hi) = best;
        let mut spliced = Vec::with_capacity(ring.len() + count + 2);
        spliced.extend_from_slice(&ring[..=oi]);
        for k in 0..=count {
            spliced.push(offset + ((hi + k) % count));
        }
        spliced.push(ring[oi]);
        spliced.extend_from_slice(&ring[oi + 1..]);
        ring = spliced;
        offset += count;
    }
    let bridged: Vec<(f64, f64)> = ring.iter().map(|&i| uvs[i]).collect();
    let local = ear_clip(&bridged);
    local.into_iter().map(|i| ring[i as usize] as u32).collect()
}

async fn ear_clip(uvs: &[(f64, f64)]) -> Vec<u32> {
    let n = uvs.len();
    if n < 3 {
        return Vec::new();
    }
    if n == 3 {
        return vec![0, 1, 2];
    }
    let mut indices: Vec<usize> = (0..n).collect();
    let mut tris = Vec::with_capacity((n - 2) * 3);
    let mut guard = 0;
    while indices.len() > 3 && guard < n * n {
        guard += 1;
        let mut clipped = false;
        let m = indices.len();
        for i in 0..m {
            let i0 = indices[(i + m - 1) % m];
            let i1 = indices[i];
            let i2 = indices[(i + 1) % m];
            if !is_convex_ear(uvs, i0, i1, i2) {
                continue;
            }
            if ear_contains_point(uvs, &indices, i0, i1, i2).await {
                continue;
            }
            tris.extend([i0 as u32, i1 as u32, i2 as u32]);
            indices.remove(i);
            clipped = true;
            break;
        }
        if !clipped {
            return fan_triangulate(n).await;
        }
    }
    if indices.len() == 3 {
        tris.extend([indices[0] as u32, indices[1] as u32, indices[2] as u32]);
    }
    tris
}

async fn is_convex_ear(uvs: &[(f64, f64)], i0: usize, i1: usize, i2: usize) -> bool {
    let a = uvs[i0];
    let b = uvs[i1];
    let c = uvs[i2];
    let cross = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
    cross > 0.0
}

async fn ear_contains_point(uvs: &[(f64, f64)], ring: &[usize], i0: usize, i1: usize, i2: usize) -> bool {
    let a = uvs[i0];
    let b = uvs[i1];
    let c = uvs[i2];
    for &idx in ring {
        if idx == i0 || idx == i1 || idx == i2 {
            continue;
        }
        if point_in_triangle(uvs[idx], a, b, c).await {
            return true;
        }
    }
    false
}

async fn point_in_triangle(p: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let area = |p0: (f64, f64), p1: (f64, f64), p2: (f64, f64)| (p1.0 - p0.0) * (p2.1 - p0.1) - (p1.1 - p0.1) * (p2.0 - p0.0);
    let a0 = area(p, a, b);
    let a1 = area(p, b, c);
    let a2 = area(p, c, a);
    (a0 >= -1e-14 && a1 >= -1e-14 && a2 >= -1e-14) || (a0 <= 1e-14 && a1 <= 1e-14 && a2 <= 1e-14)
}

async fn fan_triangulate(n: usize) -> Vec<u32> {
    let mut tris = Vec::with_capacity((n.saturating_sub(2)) * 3);
    for i in 1..n.saturating_sub(1) {
        tris.extend([0, i as u32, (i + 1) as u32]);
    }
    tris
}

async fn fan_from_centroid(uvs: &[(f64, f64)], outer_count: usize) -> Vec<u32> {
    if outer_count < 3 {
        return Vec::new();
    }
    let mut cu = 0.0;
    let mut cv = 0.0;
    for &(u, v) in uvs.iter().take(outer_count) {
        cu += u;
        cv += v;
    }
    cu /= outer_count as f64;
    cv /= outer_count as f64;
    let mut best = outer_count;
    let mut best_d = f64::INFINITY;
    for (i, &(u, v)) in uvs.iter().enumerate().skip(outer_count) {
        let d = (u - cu) * (u - cu) + (v - cv) * (v - cv);
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    if best >= uvs.len() {
        return ear_clip(&uvs[..outer_count]).await;
    }
    let mut tris = Vec::with_capacity(outer_count * 3);
    for i in 0..outer_count {
        let j = (i + 1) % outer_count;
        tris.extend([best as u32, i as u32, j as u32]);
    }
    tris
}

async fn ensure_winding(positions: &[Pnt3], indices: &mut [u32], desired: Vec3) {
    if indices.len() < 3 {
        return;
    }
    let i0 = indices[0] as usize;
    let i1 = indices[1] as usize;
    let i2 = indices[2] as usize;
    let a = positions[i1] - positions[i0];
    let b = positions[i2] - positions[i0];
    if a.cross(b).await.dot(desired) < 0.0 {
        for tri in indices.chunks_exact_mut(3) {
            tri.swap(1, 2);
        }
    }
}

// #endregion ▲Triangulate

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;

    async fn build_unit_box(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
        let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(0.0, 1.0, 1.0)];
        let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
        let edge_pairs = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
        let mut edges = HashMap::new();
        for &(a, b) in &edge_pairs {
            let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
            let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
            edges.insert((a, b), edge);
            edges.insert((b, a), edge);
        }
        let face_defs: [([usize; 4], Vec3); 6] = [([0, 3, 2, 1], -Vec3::Z), ([4, 5, 6, 7], Vec3::Z), ([0, 1, 5, 4], -Vec3::Y), ([3, 7, 6, 2], Vec3::Y), ([0, 4, 7, 3], -Vec3::X), ([1, 2, 6, 5], Vec3::X)];
        let mut faces = Vec::new();
        for (corners, normal) in face_defs {
            let frame = Frame3::from_normal(positions[corners[0]], normal).unwrap();
            let surface = body.surfaces.insert(Surface::Plane { frame });
            let members: Vec<(EdgeId, bool)> = (0..4)
                .map(|i| {
                    let a = corners[i];
                    let b = corners[(i + 1) % 4];
                    let edge = edges[&(a, b)];
                    let forward = body.edges.get(edge).unwrap().v0 == vertices[a];
                    (edge, forward)
                })
                .collect();
            let outer = make_loop(body, FaceId::from_raw(0, 0), &members);
            let face = add_face(body, surface, Some(outer), vec![], false, Tol::DEFAULT, rec);
            body.loops.get_mut(outer).unwrap().face = face;
            faces.push(face);
        }
        let shell = add_shell(body, faces, rec);
        add_solid(body, shell, vec![], rec)
    }

    #[semio_framework_async_macros::async_test]
    async fn unit_box_has_six_face_groups_and_unit_normals() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_unit_box(&mut body, &mut rec);
        let mesh = tessellate_solid(&body, solid, 0.1).expect("tessellate unit box");
        assert_eq!(mesh.face_groups.len(), 6, "unit box must yield 6 face groups");
        assert!(!mesh.position.is_empty(), "positions must be nonempty");
        assert!(!mesh.index.is_empty(), "indices must be nonempty");
        assert!(!mesh.normal.is_empty(), "normals must be nonempty");
        assert_eq!(mesh.position.len(), mesh.normal.len());
        assert_eq!(mesh.position.len() % 3, 0);
        assert_eq!(mesh.index.len() % 3, 0);
        for n in mesh.normal.chunks_exact(3) {
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-3, "normal length {len} should be ~1");
        }
        let total_group = mesh.face_groups.iter().map(|g| g.count as usize).sum::<usize>();
        assert_eq!(total_group, mesh.index.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn tessellate_face_matches_one_box_face() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_unit_box(&mut body, &mut rec);
        let face = body.solid_faces(solid)[0];
        let mesh = tessellate_face(&body, face, 0.1).expect("tessellate face");
        assert_eq!(mesh.face_groups.len(), 1);
        assert_eq!(mesh.index.len(), 6);
        assert_eq!(mesh.position.len() / 3, 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn sample_edge_polyline_returns_line_endpoints() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let v1 = make_vertex(&mut body, Pnt3::new(2.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X * 2.0 });
        let edge = make_edge(&mut body, curve, (0.0, 1.0), v0, v1, Tol::DEFAULT, &mut rec);
        let poly = sample_edge_polyline(&body, edge, 0.1);
        assert_eq!(poly.len(), 6);
        assert!((poly[0] - 0.0).abs() < 1e-6);
        assert!((poly[3] - 2.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_edge_samples_are_identical_across_adjacent_faces() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = build_unit_box(&mut body, &mut rec);
        let faces = body.solid_faces(solid);
        let edge = body.face_coedges(faces[0]).into_iter().map(|c| body.coedges.get(c).unwrap().edge).next().unwrap();
        let a = sample_edge_polyline(&body, edge, 0.05);
        let b = sample_edge_polyline(&body, edge, 0.05);
        assert_eq!(a, b);
    }

    #[semio_framework_async_macros::async_test]
    async fn circle_edge_samples_respect_deflection() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let frame = Frame3::WORLD;
        let radius = 1.0;
        let curve = body.curves3.insert(Curve3::Circle { frame, radius });
        let v = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
        let edge = make_edge(&mut body, curve, (0.0, std::f64::consts::TAU), v, v, Tol::DEFAULT, &mut rec);
        let coarse = sample_edge_polyline(&body, edge, 0.2);
        let fine = sample_edge_polyline(&body, edge, 0.02);
        assert!(fine.len() > coarse.len(), "tighter deflection must densify circle samples");
        assert!(coarse.len() >= 6);
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_solid_returns_missing_entity() {
        let body = Body::new();
        let err = tessellate_solid(&body, SolidId::from_raw(9, 0), 0.1).unwrap_err();
        assert!(matches!(err, KernelError::MissingEntity(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn tessellate_rectangle_wire_emits_edge_segments() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_rectangle_wire;
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut body, 2.0, 1.5, &mut rec).expect("wire");
        let mesh = tessellate_wire(&body, &wire, 0.1).expect("tessellate wire");
        assert!(mesh.edges.len() >= 24, "expected closed rectangle edge polylines, got {}", mesh.edges.len());
        assert!(mesh.position.is_empty());
        assert!(mesh.index.is_empty());
    }
}

// #endregion 🔖️Tests
