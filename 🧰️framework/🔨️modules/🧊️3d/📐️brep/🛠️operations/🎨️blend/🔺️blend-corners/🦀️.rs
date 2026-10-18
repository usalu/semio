//! 🔺️ Blend corners: the corner record, the vertex star, the corner builder and surface and the conic end trim. See [`super`].

use super::*;

// #region 🔖️Corner
/// 🎨️ Everything the surgery needs at one vertex a blend reaches: where each blended face's own
/// boundary now stops, the run parameter each incident patch's tangency curve is trimmed to on each
/// of its faces, the unblended face that receives the blend's end curve as a new boundary, and the
/// corner patch that closes a fully blended vertex.
pub(super) struct Corner {
    pub(super) points: BTreeMap<FaceId, Pnt3>,
    pub(super) verts: BTreeMap<FaceId, VertexId>,
    pub(super) runs: BTreeMap<(EdgeId, FaceId), f64>,
    pub(super) cap: Option<FaceId>,
    pub(super) corner_surface: Option<Surface>,
    pub(super) corner_edges: Vec<EdgeId>,
}

/// 🎨️ Every solid edge and face incident to `vertex`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vertex_star(body: &Body, vertex: VertexId, solid_faces: &BTreeSet<FaceId>) -> (BTreeSet<EdgeId>, BTreeSet<FaceId>) {
    let mut edges = BTreeSet::new();
    let mut faces = BTreeSet::new();
    for &face in solid_faces {
        for cid in body.face_coedges(face) {
            let Some(c) = body.coedges.get(cid) else { continue };
            let Some(e) = body.edges.get(c.edge) else { continue };
            if e.v0 == vertex || e.v1 == vertex {
                edges.insert(c.edge);
                faces.insert(face);
            }
        }
    }
    (edges, faces)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tangency_param(patch: &Patch, i: usize, point: Pnt3) -> Result<f64, KernelError> {
    curve_param_at(&patch.tangency(i), point, (f64::NEG_INFINITY, f64::INFINITY)).ok_or_else(|| KernelError::Operation("blend: the corner point does not lie on the patch's own tangency curve".into()))
}

/// 🎨️ Classifies one vertex a blend reaches and derives its corner geometry.
///
/// * a CLOSED band's seam vertex splits into one vertex per adjacent face, joined by the band's own
///   seam curve (used twice by the band, exactly as a cylinder's lateral seam is);
/// * a vertex with exactly ONE blended edge keeps its two unblended edges, shortened to where the
///   tangency curves cross the third (cap) face's plane, and that cap face gains the blend's end
///   curve as a new boundary edge;
/// * a vertex with ALL THREE edges blended loses its three edges entirely: each face's two tangency
///   curves meet at one point and the three end curves bound a corner patch — a sphere octant for a
///   fillet (the ball resting in the corner touches all three faces, so those three points share
///   its centre and every end circle is a GREAT circle on it), a planar triangle for a chamfer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_corner(body: &Body, vertex: VertexId, incident: &[EdgeId], patches: &BTreeMap<EdgeId, Patch>, solid_faces: &BTreeSet<FaceId>, spec: BlendSpec) -> Result<Corner, KernelError> {
    let mut corner = Corner { points: BTreeMap::new(), verts: BTreeMap::new(), runs: BTreeMap::new(), cap: None, corner_surface: None, corner_edges: Vec::new() };
    if incident.len() == 1 && patches[&incident[0]].closed {
        let patch = &patches[&incident[0]];
        for i in 0..2 {
            corner.points.insert(patch.faces[i], patch.tangency(i).eval(0.0));
            corner.runs.insert((patch.edge, patch.faces[i]), 0.0);
        }
        return Ok(corner);
    }
    let (star_edges, star_faces) = vertex_star(body, vertex, solid_faces);
    if star_edges.len() != 3 || star_faces.len() != 3 {
        return Err(KernelError::Operation(format!("blend: vertex {vertex} is not trihedral ({} edges, {} faces) — no exact corner exists", star_edges.len(), star_faces.len())));
    }
    match incident.len() {
        1 => {
            let patch = &patches[&incident[0]];
            let cap = *star_faces.iter().find(|f| **f != patch.faces[0] && **f != patch.faces[1]).ok_or_else(|| KernelError::Operation("blend: no cap face at a partially blended vertex".into()))?;
            let (cap_point, cap_normal) = face_plane(body, cap)?;
            for i in 0..2 {
                let Curve3::Line { origin, dir } = patch.tangency(i) else {
                    return Err(KernelError::Operation("blend: a partially blended vertex needs straight tangency curves".into()));
                };
                let point = line_plane_point(origin, dir, cap_point, cap_normal).ok_or_else(|| KernelError::Operation("blend: the tangency curve runs parallel to its own cap face".into()))?;
                corner.runs.insert((patch.edge, patch.faces[i]), tangency_param(patch, i, point)?);
                corner.points.insert(patch.faces[i], point);
            }
            corner.cap = Some(cap);
            corner.corner_edges = vec![patch.edge];
        }
        3 => {
            let mut normals: BTreeMap<FaceId, Vec3> = BTreeMap::new();
            let vertex_position = body.vertices.get(vertex).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
            for &face in &star_faces {
                let owners: Vec<(EdgeId, usize)> = incident.iter().filter_map(|e| patches[e].faces.iter().position(|f| *f == face).map(|i| (*e, i))).collect();
                if owners.len() != 2 {
                    return Err(KernelError::Operation("blend: a fully blended vertex needs exactly two blended edges per face".into()));
                }
                let mut lines = Vec::with_capacity(2);
                for (edge, i) in &owners {
                    let Curve3::Line { origin, dir } = patches[edge].tangency(*i) else {
                        return Err(KernelError::Operation("blend: a fully blended vertex needs straight tangency curves".into()));
                    };
                    lines.push((origin, dir));
                }
                let point = line_line_point(lines[0].0, lines[0].1, lines[1].0, lines[1].1).ok_or_else(|| KernelError::Operation("blend: the two tangency curves on a face do not meet at the corner".into()))?;
                for (edge, i) in &owners {
                    corner.runs.insert((*edge, face), tangency_param(&patches[edge], *i, point)?);
                }
                normals.insert(face, outward_normal_at(body, face, vertex_position)?);
                corner.points.insert(face, point);
            }
            for &edge in incident {
                let patch = &patches[&edge];
                let a = corner.runs[&(edge, patch.faces[0])];
                let b = corner.runs[&(edge, patch.faces[1])];
                if (a - b).abs() > 1e-7 {
                    return Err(KernelError::Operation("blend: a corner patch needs its neighbours trimmed at one common station".into()));
                }
            }
            corner.corner_edges = incident.to_vec();
            corner.corner_surface = Some(corner_surface(&corner.points, &normals, spec)?);
        }
        _ => return Err(KernelError::Operation(format!("blend: vertex {vertex} has {} of its 3 edges blended — an exact corner needs either one or all", incident.len()))),
    }
    Ok(corner)
}

/// 🎨️ The corner patch closing a fully blended trihedral vertex: a sphere of the fillet's own
/// radius centred where the rolling ball rests against all three faces (so every end circle, having
/// that same centre and radius, is a GREAT circle on it), or the plane through the three chamfer
/// corner points. The sphere's frame is pinned with `z` on a face normal perpendicular to the other
/// two, which puts that face's corner at the pole and makes all three boundaries iso-lines: one
/// equator and two meridians.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn corner_surface(points: &BTreeMap<FaceId, Pnt3>, normals: &BTreeMap<FaceId, Vec3>, spec: BlendSpec) -> Result<Surface, KernelError> {
    let faces: Vec<FaceId> = points.keys().copied().collect();
    match spec.fillet_radius() {
        Some(r) => {
            for rotation in 0..3 {
                let (a, b, c) = (faces[rotation], faces[(rotation + 1) % 3], faces[(rotation + 2) % 3]);
                let (na, nb, nc) = (normals[&a], normals[&b], normals[&c]);
                if na.dot(nb).abs() > DIR_EPS || na.dot(nc).abs() > DIR_EPS {
                    continue;
                }
                let center = points[&a] - na * r;
                for face in &faces {
                    if (center + normals[face] * r).distance(points[face]) > 1e-7 {
                        return Err(KernelError::Operation("blend: the three corner points do not share one rolling-ball centre".into()));
                    }
                }
                return Ok(Surface::Sphere { frame: Frame3 { origin: center, x: nb, y: na.cross(nb), z: na }, radius: r });
            }
            Err(KernelError::Operation("blend: a spherical corner needs one face normal perpendicular to the other two".into()))
        }
        None => {
            let (p0, p1, p2) = (points[&faces[0]], points[&faces[1]], points[&faces[2]]);
            let normal = unit((p1 - p0).cross(p2 - p0), "chamfer corner normal")?;
            let outward = normals.values().fold(Vec3::ZERO, |acc, n| acc + *n);
            let normal = if normal.dot(outward) >= 0.0 { normal } else { -normal };
            Ok(Surface::Plane { frame: Frame3::from_normal(p0, normal).ok_or_else(|| KernelError::Operation("blend: degenerate chamfer corner plane".into()))? })
        }
    }
}

/// 🎨️ The exact conic a cap plane cuts out of a cone patch, as a rational quadratic Bézier from
/// `faces[0]`'s tangency ruling to `faces[1]`'s.
///
/// The cone's ruling direction at azimuth `u` is `D(u)`, an affine function of `(cos u, sin u)`, so
/// it is exactly a rational quadratic over the standard arc control net `C_i` with weights `w_i`.
/// The plane meets that ruling at `A + D(u)·c/(D(u)·n)` — a PERSPECTIVE image of `D`, which is
/// exactly the rational quadratic with control points `A + D_i·c/(D_i·n)` and weights `w_i(D_i·n)`
/// (substituting them back collapses the ratio to the same formula, so the identity is exact, not a
/// fit). Every control point lies in the cap plane, which is what makes the p-curve on the cap
/// exact too — only the p-curve on the CONE has no closed form, and that one is certified numerically.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn conic_end_trim(patch: &Patch, plane_point: Pnt3, plane_normal: Vec3) -> Result<Curve3, KernelError> {
    let Surface::Cone { frame, half_angle } = &patch.surface else {
        return Err(KernelError::Operation("blend: only a cone patch can be trimmed by an oblique cap plane".into()));
    };
    let (u0, u1) = (patch.cross[0], patch.cross[1]);
    let half = 0.5 * (u1 - u0);
    if !(half.abs() > 0.0 && half.abs() < FRAC_PI_2) {
        return Err(KernelError::Operation("blend: the blend's angular span needs a single conic arc".into()));
    }
    let mid = 0.5 * (u0 + u1);
    let cos_half = half.cos();
    let net = [(u0.cos(), u0.sin(), 1.0), (mid.cos() / cos_half, mid.sin() / cos_half, cos_half), (u1.cos(), u1.sin(), 1.0)];
    let tan_a = half_angle.tan();
    let mut controls = Vec::with_capacity(3);
    let mut weights = Vec::with_capacity(3);
    let mut offset = (plane_point - frame.origin).dot(plane_normal);
    let mut normal = plane_normal;
    let mut directions = Vec::with_capacity(3);
    for (cx, cy, _) in net {
        directions.push(frame.to_world_vector(Vec3::new(tan_a * cx, tan_a * cy, 1.0)));
    }
    if directions.iter().all(|d| d.dot(normal) < 0.0) {
        normal = -normal;
        offset = -offset;
    }
    for (i, (_, _, w)) in net.into_iter().enumerate() {
        let denominator = directions[i].dot(normal);
        if denominator <= DIR_EPS {
            return Err(KernelError::Operation("blend: a ruling of this blend never reaches its own cap plane".into()));
        }
        controls.push(frame.origin + directions[i] * (offset / denominator));
        weights.push(w * denominator);
    }
    let knots = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2, 3).ok_or_else(|| KernelError::Operation("blend: conic knot vector".into()))?;
    Ok(Curve3::Nurbs { knots, controls, weights })
}
// #endregion 🔖️Corner
