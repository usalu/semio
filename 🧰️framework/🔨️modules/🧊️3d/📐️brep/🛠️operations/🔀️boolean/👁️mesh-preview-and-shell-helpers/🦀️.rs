//! 👁️ Mesh preview and shell helpers: the opt-in mesh preview boolean and the outer face, vertex position and probe point helpers. See [`super`].

use super::*;

// #region 🔖️MeshPreview
/// 🔀 The pre-rewrite tessellate→centroid-classify→triangle-soup boolean, kept as an explicit
/// opt-in for callers that pass `OpQuality::MeshDerivedBRep` — `boolean_solid` itself never calls
/// this.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boolean_solid_mesh_preview(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    require_tol(tol)?;
    require_solid(body, a)?;
    require_solid(body, b)?;
    let deflection = tol.max(1e-3);
    let mesh_a = tessellate_solid(body, a, deflection)?;
    let mesh_b = tessellate_solid(body, b, deflection)?;
    let mut points = Vec::new();
    let mut triangles: Vec<[Pnt3; 3]> = Vec::new();
    append_kept_triangles(body, &mesh_a, b, op, true, tol, (&mut points, &mut triangles))?;
    append_kept_triangles(body, &mesh_b, a, op, false, tol, (&mut points, &mut triangles))?;
    if triangles.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult("mesh boolean produced no triangles".into())));
    }
    match solid_from_triangle_soup(body, &triangles, rec) {
        Ok(id) => Ok(id),
        Err(_) => make_convex_hull(body, &points, rec).map_err(|e| match e {
            KernelError::InvalidInput(msg) => KernelError::Boolean(BooleanError::InvalidResult(msg)),
            other => other,
        }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn append_kept_triangles(body: &Body, mesh: &MeshTransfer, other: SolidId, op: BooleanOp, from_a: bool, tol: f64, (out_points, out_tris): (&mut Vec<Pnt3>, &mut Vec<[Pnt3; 3]>)) -> Result<(), KernelError> {
    let npos = mesh.position.len() / 3;
    if !mesh.index.len().is_multiple_of(3) {
        return Err(KernelError::InvalidInput("mesh index length must be a multiple of 3".into()));
    }
    for tri in mesh.index.as_chunks::<3>().0 {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= npos || i1 >= npos || i2 >= npos {
            return Err(KernelError::InvalidInput("mesh index out of range".into()));
        }
        let p0 = mesh_position(mesh, i0);
        let p1 = mesh_position(mesh, i1);
        let p2 = mesh_position(mesh, i2);
        let centroid = Pnt3::new((p0.x + p1.x + p2.x) / 3.0, (p0.y + p1.y + p2.y) / 3.0, (p0.z + p1.z + p2.z) / 3.0);
        let class = local_point_in_solid(body, other, centroid, tol)?;
        if keep_face(op, from_a, class) {
            out_points.push(p0);
            out_points.push(p1);
            out_points.push(p2);
            out_tris.push([p0, p1, p2]);
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn mesh_position(mesh: &MeshTransfer, i: usize) -> Pnt3 {
    let o = i * 3;
    Pnt3::new(mesh.position[o] as f64, mesh.position[o + 1] as f64, mesh.position[o + 2] as f64)
}
// #endregion 🔖️MeshPreview

// #region 🔖️ShellHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn solid_from_outer_faces(body: &mut Body, outer_faces: Vec<FaceId>, inner_face_sets: Vec<Vec<FaceId>>, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if outer_faces.is_empty() {
        return Err(KernelError::InvalidInput("outer shell requires at least one face".into()));
    }
    let outer = add_shell(body, outer_faces, rec);
    let mut inners = Vec::with_capacity(inner_face_sets.len());
    for faces in inner_face_sets {
        if faces.is_empty() {
            return Err(KernelError::InvalidInput("inner shell requires at least one face".into()));
        }
        inners.push(add_shell(body, faces, rec));
    }
    Ok(add_solid(body, outer, inners, rec))
}

/// 🔀 A deep copy of `solid`'s OUTER faces, detached from any shell/solid wrapper and ready to be
/// re-assembled into somebody else's shell — the operand-owning counterpart of [`outer_faces`],
/// which hands back the operand's own face ids and therefore may only ever be READ from.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn detached_copy_of_outer_faces(body: &mut Body, solid: SolidId, rec: &mut OpRecorder) -> Result<Vec<FaceId>, KernelError> {
    let copy = copy_solid(body, solid, rec)?;
    let faces = outer_faces(body, copy)?;
    let keep: HashSet<FaceId> = faces.iter().copied().collect();
    remove_solid_and_orphans(body, copy, &keep, rec);
    Ok(faces)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn outer_faces(body: &Body, solid: SolidId) -> Result<Vec<FaceId>, KernelError> {
    let data = body.solids.get(solid).ok_or_else(|| KernelError::MissingEntity(format!("solid {solid}")))?;
    let shell = body.shells.get(data.outer).ok_or_else(|| KernelError::MissingEntity(format!("shell {}", data.outer)))?;
    Ok(shell.faces.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn solid_vertex_positions(body: &Body, solid: SolidId) -> Vec<Pnt3> {
    let mut seen: HashSet<VertexId> = HashSet::new();
    let mut points = Vec::new();
    for face in body.solid_faces(solid) {
        let Some(face_ent) = body.faces.get(face) else {
            continue;
        };
        let mut loops = Vec::new();
        if let Some(outer) = face_ent.outer {
            loops.push(outer);
        }
        loops.extend(face_ent.inners.iter().copied());
        for loop_id in loops {
            let Some(loop_ent) = body.loops.get(loop_id) else {
                continue;
            };
            let start = loop_ent.first;
            let mut cur = start;
            loop {
                if let Some((v0, _)) = body.coedge_endpoints(cur) {
                    if seen.insert(v0) {
                        if let Some(v) = body.vertices.get(v0) {
                            points.push(v.position);
                        }
                    }
                }
                let Some(coedge) = body.coedges.get(cur) else {
                    break;
                };
                cur = coedge.next;
                if cur == start {
                    break;
                }
            }
        }
    }
    points
}

/// 🔀 Surface samples proving a containment claim: every vertex, seven points along every edge and a grid of interior points on every
/// face. Vertices alone prove nothing for a curved solid — a sphere has two, both on its axis.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn solid_probe_points(body: &Body, solid: SolidId, tol: f64) -> Vec<Pnt3> {
    let mut points = solid_vertex_positions(body, solid);
    let mut seen_edges: HashSet<EdgeId> = HashSet::new();
    for face in body.solid_faces(solid) {
        for coedge in body.face_coedges(face) {
            let Some(edge_id) = body.coedges.get(coedge).map(|c| c.edge) else { continue };
            if !seen_edges.insert(edge_id) {
                continue;
            }
            let Some(edge) = body.edges.get(edge_id) else { continue };
            let Some(curve) = body.curves3.get(edge.curve) else { continue };
            for k in 1..8 {
                points.push(curve.eval(edge.range.0 + (edge.range.1 - edge.range.0) * k as f64 / 8.0));
            }
        }
        let Some(face_data) = body.faces.get(face) else { continue };
        let (Some(surface), Some(outer)) = (body.surfaces.get(face_data.surface), face_data.outer) else { continue };
        let ring = sample_loop_uv(body, outer, surface, 12);
        if ring.is_empty() {
            continue;
        }
        let (mut u0, mut u1, mut v0, mut v1) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
        for p in &ring {
            u0 = u0.min(p.x);
            u1 = u1.max(p.x);
            v0 = v0.min(p.y);
            v1 = v1.max(p.y);
        }
        const GRID: usize = 5;
        for i in 0..GRID {
            for j in 0..GRID {
                let uv = Pnt2::new(u0 + (u1 - u0) * (i as f64 + 0.5) / GRID as f64, v0 + (v1 - v0) * (j as f64 + 0.5) / GRID as f64);
                if point_in_face_uv_periodic(body, face, uv, tol) {
                    points.push(surface.eval(uv.x, uv.y));
                }
            }
        }
    }
    points
}
// #endregion 🔖️ShellHelpers
