//! 🔁️ Loop areas and volume moments: the straight loop area, the face and loop volume contributions and moments and the Newell area. See [`super`].

use super::*;

// #region 🔖️Loops
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn loop_area(body: &Body, face: FaceId, loop_id: semio_framework_3d_brep_representation::arena::LoopId, chord_tol: f64) -> Result<f64, KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).is_some_and(|f| f.flipped);
    match surface {
        Surface::Plane { frame } if loop_has_only_straight_edges(body, loop_id) => {
            let pts = loop_positions(body, loop_id)?;
            Ok(newell_area(&pts, outward_plane_normal(frame, flipped)))
        }
        _ => {
            let boundary = loop_uv_polygon(body, loop_id, surface, chord_tol)?;
            let (m, _err) = loop_moments(surface, flipped, &boundary, chord_tol.max(1e-5));
            Ok(m[IDX_AREA])
        }
    }
}

/// 📏 True only when every coedge of `loop_id` walks a straight [`Curve3::Line`] — the fast
/// one-vertex-per-coedge `loop_positions` path (below) is exact ONLY for such loops. A planar
/// face's boundary can still carry a curved (`Circle`/`Ellipse`/`Nurbs`) edge — a cylinder's cap,
/// a plate with a round hole — and `loop_positions` silently degenerates that loop to a 1-4 point
/// "polygon" (e.g. a cylinder cap's single full-circle coedge collapses to ONE point, zeroing the
/// cap's whole area/volume contribution): the same under-sampling class of bug as
/// [`loop_uv_polygon`]'s, just for the OTHER (planar fast-path) branch of `loop_area`/
/// `loop_volume_moments`. Route those loops through the curvature-adaptive general path instead
/// (`face_moments_general`'s own doc already notes the general quadrature is exact on a flat facet
/// too, so this loses nothing but the fast path's cheapness).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_has_only_straight_edges(body: &Body, loop_id: semio_framework_3d_brep_representation::arena::LoopId) -> bool {
    body.loop_coedges(loop_id).into_iter().all(|coedge| body.coedges.get(coedge).and_then(|co| body.edges.get(co.edge)).and_then(|edge| body.curves3.get(edge.curve)).is_some_and(|curve| matches!(curve, Curve3::Line { .. })))
}

/// ⚖️ ONE face's divergence-theorem contribution to the signed volume of the shell that owns it —
/// the atomic unit [`shell_signed_volume`] sums, and therefore the smallest piece of that sum a
/// budgeted validator can do per step. Exposed so a resumable pass
/// (`semio_framework_3d_brep_validation::BodyValidationJob`)
/// can accumulate the same total one face at a time instead of paying the whole shell in one
/// non-preemptible call (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn face_volume_contribution(body: &Body, face: FaceId, chord_tol: f64) -> Result<f64, KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut vol = 0.0;
    if let Some(outer) = face_ent.outer {
        vol += loop_volume_contribution(body, face, outer, chord_tol)?;
    }
    for &inner in &face_ent.inners {
        vol -= loop_volume_contribution(body, face, inner, chord_tol)?;
    }
    Ok(vol)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn face_volume_moments(body: &Body, face: FaceId, chord_tol: f64) -> Result<(f64, f64, f64, f64), KernelError> {
    let Some(face_ent) = body.faces.get(face) else {
        return Err(KernelError::MissingEntity("face".into()));
    };
    let mut sv = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    if let Some(outer) = face_ent.outer {
        let (a, b, c, d) = loop_volume_moments(body, face, outer, chord_tol)?;
        sv += a;
        mx += b;
        my += c;
        mz += d;
    }
    for &inner in &face_ent.inners {
        let (a, b, c, d) = loop_volume_moments(body, face, inner, chord_tol)?;
        sv -= a;
        mx -= b;
        my -= c;
        mz -= d;
    }
    Ok((sv, mx, my, mz))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_volume_contribution(body: &Body, face: FaceId, loop_id: semio_framework_3d_brep_representation::arena::LoopId, chord_tol: f64) -> Result<f64, KernelError> {
    let (sv, _, _, _) = loop_volume_moments(body, face, loop_id, chord_tol)?;
    Ok(sv / 6.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_volume_moments(body: &Body, face: FaceId, loop_id: semio_framework_3d_brep_representation::arena::LoopId, chord_tol: f64) -> Result<(f64, f64, f64, f64), KernelError> {
    let surface = face_surface(body, face)?;
    let flipped = body.faces.get(face).is_some_and(|f| f.flipped);
    match surface {
        Surface::Plane { .. } if loop_has_only_straight_edges(body, loop_id) => {
            let mut pts = loop_positions(body, loop_id)?;
            if let Some(natural) = surface.normal(0.0, 0.0) {
                if polygon_area_vector(&pts).dot(natural) < 0.0 {
                    pts.reverse();
                }
            }
            let (sv, mx, my, mz) = signed_tetra_sum(&pts);
            if flipped {
                Ok((-sv, -mx, -my, -mz))
            } else {
                Ok((sv, mx, my, mz))
            }
        }
        _ => {
            let boundary = loop_uv_polygon(body, loop_id, surface, chord_tol)?;
            let (m, _err) = loop_moments(surface, flipped, &boundary, chord_tol.max(1e-5));
            Ok((6.0 * m[IDX_VOL], 24.0 * m[IDX_MX], 24.0 * m[IDX_MY], 24.0 * m[IDX_MZ]))
        }
    }
}

/// 📐 Twice the vector area of the closed polygon `pts` — its direction is the winding's normal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn polygon_area_vector(pts: &[Pnt3]) -> Vec3 {
    let Some(&first) = pts.first() else { return Vec3::new(0.0, 0.0, 0.0) };
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, &p) in pts.iter().enumerate() {
        let q = pts[(i + 1) % pts.len()];
        area = area + (p - first).cross(q - first);
    }
    area
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn signed_tetra_sum(pts: &[Pnt3]) -> (f64, f64, f64, f64) {
    if pts.len() < 3 {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let p0 = pts[0];
    let mut sv = 0.0;
    let mut mx = 0.0;
    let mut my = 0.0;
    let mut mz = 0.0;
    for i in 1..pts.len() - 1 {
        let a = p0.to_vec();
        let b = pts[i] - p0;
        let c = pts[i + 1] - p0;
        let tet = a.dot(b.cross(c));
        sv += tet;
        mx += tet * (p0.x + pts[i].x + pts[i + 1].x);
        my += tet * (p0.y + pts[i].y + pts[i + 1].y);
        mz += tet * (p0.z + pts[i].z + pts[i + 1].z);
    }
    (sv, mx, my, mz)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_positions(body: &Body, loop_id: semio_framework_3d_brep_representation::arena::LoopId) -> Result<Vec<Pnt3>, KernelError> {
    let mut pts = Vec::new();
    for coedge in body.loop_coedges(loop_id) {
        let (v0, _) = body.coedge_endpoints(coedge).ok_or_else(|| KernelError::InvalidInput("open coedge".into()))?;
        let v = body.vertices.get(v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?;
        pts.push(v.position);
    }
    Ok(pts)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn face_surface(body: &Body, face: FaceId) -> Result<&Surface, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn outward_plane_normal(frame: &semio_framework_3d_brep_vector::matrix::Frame3, flipped: bool) -> Vec3 {
    let mut n = frame.z;
    if flipped {
        n = -n;
    }
    n
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newell_area(pts: &[Pnt3], normal: Vec3) -> f64 {
    if pts.len() < 3 {
        return 0.0;
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        cx += pts[i].y * pts[j].z - pts[i].z * pts[j].y;
        cy += pts[i].z * pts[j].x - pts[i].x * pts[j].z;
        cz += pts[i].x * pts[j].y - pts[i].y * pts[j].x;
    }
    let area_vec = Vec3::new(cx, cy, cz);
    0.5 * area_vec.dot(normal).abs()
}
// #endregion 🔖️Loops
