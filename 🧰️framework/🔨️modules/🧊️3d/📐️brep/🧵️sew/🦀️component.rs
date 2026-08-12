//! 🧵 Free-face sewing: tolerance edge matching and coedge pairing.
//!
//! Merges geometrically coincident vertices and edges across independent faces, then assembles
//! a closed shell and solid. See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

use std::collections::HashMap;

use crate::brep::arena::{ArenaId, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use crate::brep::curve::Curve3;
use crate::brep::error::KernelError;
use crate::brep::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::brep::history::OpRecorder;
use crate::brep::tolerance::Tol;
use crate::brep::topo::Body;
use crate::brep::vec::{Pnt3, Vec3};

// #region 🔖️Api

/// 🧵 Sew loose faces into one solid by merging coincident boundary edges within `tolerance`.
pub fn sew_faces(body: &mut Body, faces: &[FaceId], tolerance: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    if faces.len() < 2 {
        return Err(KernelError::InvalidInput("sewing requires at least 2 faces".into()));
    }
    let tol = if tolerance > 0.0 && tolerance.is_finite() {
        Tol::new(tolerance)
    } else {
        Tol::DEFAULT
    };
    let linear = tol.value();
    let snapshots = snapshot_faces(body, faces)?;
    let resolution = 1.0 / linear;
    let mut vertex_map: HashMap<(i64, i64, i64), VertexId> = HashMap::new();
    let mut edge_map: HashMap<(VertexId, VertexId), EdgeId> = HashMap::new();
    let mut new_faces = Vec::with_capacity(snapshots.len());
    for snap in &snapshots {
        let mut members = Vec::with_capacity(snap.edge_endpoints.len());
        for &(start_pt, end_pt) in &snap.edge_endpoints {
            let v_start = get_or_create_vertex(body, start_pt, resolution, tol, &mut vertex_map, rec);
            let v_end = get_or_create_vertex(body, end_pt, resolution, tol, &mut vertex_map, rec);
            let (v_lo, v_hi) = if v_start <= v_end { (v_start, v_end) } else { (v_end, v_start) };
            let forward = v_start == v_lo;
            let edge = *edge_map.entry((v_lo, v_hi)).or_insert_with(|| {
                let p0 = body.vertices.get(v_lo).expect("vertex").position;
                let p1 = body.vertices.get(v_hi).expect("vertex").position;
                let curve = body.curves3.insert(Curve3::Line { origin: p0, dir: p1 - p0 });
                make_edge(body, curve, (0.0, 1.0), v_lo, v_hi, tol, rec)
            });
            members.push((edge, forward));
        }
        let placeholder = FaceId::from_raw(0, 0);
        let outer = make_loop(body, placeholder, &members);
        let face = add_face(body, snap.surface, Some(outer), vec![], snap.flipped, snap.tol, rec);
        body.loops.get_mut(outer).expect("loop").face = face;
        new_faces.push(face);
    }
    let shell = add_shell(body, new_faces, rec);
    Ok(add_solid(body, shell, vec![], rec))
}

// #endregion 🔖️Api

// #region 🔖️Snapshot

struct FaceSnapshot {
    surface: SurfaceId,
    flipped: bool,
    tol: Tol,
    edge_endpoints: Vec<(Pnt3, Pnt3)>,
}

fn snapshot_faces(body: &Body, faces: &[FaceId]) -> Result<Vec<FaceSnapshot>, KernelError> {
    let mut out = Vec::with_capacity(faces.len());
    for &fid in faces {
        let face = body
            .faces
            .get(fid)
            .ok_or_else(|| KernelError::MissingEntity(format!("face {fid}")))?;
        let outer = face.outer.ok_or_else(|| KernelError::Operation(format!("face {fid} has no outer loop")))?;
        let mut edge_endpoints = Vec::new();
        for coedge_id in body.loop_coedges(outer) {
            let coedge = body
                .coedges
                .get(coedge_id)
                .ok_or_else(|| KernelError::MissingEntity(format!("coedge {coedge_id}")))?;
            let edge = body
                .edges
                .get(coedge.edge)
                .ok_or_else(|| KernelError::MissingEntity(format!("edge {}", coedge.edge)))?;
            let p0 = body.vertices.get(edge.v0).expect("v0").position;
            let p1 = body.vertices.get(edge.v1).expect("v1").position;
            let (start_pt, end_pt) = if coedge.forward { (p0, p1) } else { (p1, p0) };
            edge_endpoints.push((start_pt, end_pt));
        }
        out.push(FaceSnapshot {
            surface: face.surface,
            flipped: face.flipped,
            tol: face.tol,
            edge_endpoints,
        });
    }
    Ok(out)
}

fn get_or_create_vertex(
    body: &mut Body,
    p: Pnt3,
    resolution: f64,
    tol: Tol,
    map: &mut HashMap<(i64, i64, i64), VertexId>,
    rec: &mut OpRecorder,
) -> VertexId {
    let key = (
        (p.x * resolution).round() as i64,
        (p.y * resolution).round() as i64,
        (p.z * resolution).round() as i64,
    );
    *map.entry(key).or_insert_with(|| make_vertex(body, p, tol, rec))
}

// #endregion 🔖️Snapshot

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::mat::Frame3;

    fn make_loose_quad(body: &mut Body, p0: Pnt3, p1: Pnt3, p2: Pnt3, p3: Pnt3, normal: Vec3) -> FaceId {
        let mut rec = OpRecorder::new();
        let tol = Tol::DEFAULT;
        let frame = Frame3::from_normal(p0, normal).expect("plane frame");
        let surface = body.surfaces.insert(crate::brep::surface::Surface::Plane { frame });
        let v0 = make_vertex(body, p0, tol, &mut rec);
        let v1 = make_vertex(body, p1, tol, &mut rec);
        let v2 = make_vertex(body, p2, tol, &mut rec);
        let v3 = make_vertex(body, p3, tol, &mut rec);
        let mut line = |a: Pnt3, b: Pnt3, va: VertexId, vb: VertexId| {
            let curve = body.curves3.insert(Curve3::Line { origin: a, dir: b - a });
            make_edge(body, curve, (0.0, 1.0), va, vb, tol, &mut rec)
        };
        let e0 = line(p0, p1, v0, v1);
        let e1 = line(p1, p2, v1, v2);
        let e2 = line(p2, p3, v2, v3);
        let e3 = line(p3, p0, v3, v0);
        let placeholder = FaceId::from_raw(0, 0);
        let outer = make_loop(body, placeholder, &[(e0, true), (e1, true), (e2, true), (e3, true)]);
        let face = add_face(body, surface, Some(outer), vec![], false, tol, &mut rec);
        body.loops.get_mut(outer).unwrap().face = face;
        face
    }

    fn unique_edges_on_solid(body: &Body, solid: SolidId) -> usize {
        let mut edges = std::collections::HashSet::new();
        for fid in body.solid_faces(solid) {
            for cid in body.face_coedges(fid) {
                let e = body.coedges.get(cid).unwrap().edge;
                edges.insert((e.raw_index(), e.raw_generation()));
            }
        }
        edges.len()
    }

    #[test]
    fn sew_two_adjacent_quads_shares_one_edge() {
        let mut body = Body::new();
        let f0 = make_loose_quad(
            &mut body,
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(1.0, 1.0, 0.0),
            Pnt3::new(0.0, 1.0, 0.0),
            Vec3::Z,
        );
        let f1 = make_loose_quad(
            &mut body,
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(2.0, 0.0, 0.0),
            Pnt3::new(2.0, 1.0, 0.0),
            Pnt3::new(1.0, 1.0, 0.0),
            Vec3::Z,
        );
        let mut rec = OpRecorder::new();
        let solid = sew_faces(&mut body, &[f0, f1], 1e-6, &mut rec).unwrap();
        assert_eq!(body.solid_faces(solid).len(), 2);
        assert_eq!(unique_edges_on_solid(&body, solid), 7);
    }

    #[test]
    fn sew_six_box_faces_into_solid() {
        let mut body = Body::new();
        let bottom = make_loose_quad(
            &mut body,
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(1.0, 1.0, 0.0),
            Pnt3::new(0.0, 1.0, 0.0),
            -Vec3::Z,
        );
        let top = make_loose_quad(
            &mut body,
            Pnt3::new(0.0, 0.0, 1.0),
            Pnt3::new(1.0, 0.0, 1.0),
            Pnt3::new(1.0, 1.0, 1.0),
            Pnt3::new(0.0, 1.0, 1.0),
            Vec3::Z,
        );
        let front = make_loose_quad(
            &mut body,
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 1.0),
            Pnt3::new(0.0, 0.0, 1.0),
            -Vec3::Y,
        );
        let back = make_loose_quad(
            &mut body,
            Pnt3::new(0.0, 1.0, 0.0),
            Pnt3::new(1.0, 1.0, 0.0),
            Pnt3::new(1.0, 1.0, 1.0),
            Pnt3::new(0.0, 1.0, 1.0),
            Vec3::Y,
        );
        let left = make_loose_quad(
            &mut body,
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(0.0, 1.0, 0.0),
            Pnt3::new(0.0, 1.0, 1.0),
            Pnt3::new(0.0, 0.0, 1.0),
            -Vec3::X,
        );
        let right = make_loose_quad(
            &mut body,
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(1.0, 1.0, 0.0),
            Pnt3::new(1.0, 1.0, 1.0),
            Pnt3::new(1.0, 0.0, 1.0),
            Vec3::X,
        );
        let mut rec = OpRecorder::new();
        let solid = sew_faces(&mut body, &[bottom, top, front, back, left, right], 1e-6, &mut rec).unwrap();
        assert_eq!(body.solid_faces(solid).len(), 6);
        assert_eq!(unique_edges_on_solid(&body, solid), 12);
    }

    #[test]
    fn sew_single_face_rejects() {
        let mut body = Body::new();
        let f = make_loose_quad(
            &mut body,
            Pnt3::new(0.0, 0.0, 0.0),
            Pnt3::new(1.0, 0.0, 0.0),
            Pnt3::new(1.0, 1.0, 0.0),
            Pnt3::new(0.0, 1.0, 0.0),
            Vec3::Z,
        );
        let mut rec = OpRecorder::new();
        assert!(sew_faces(&mut body, &[f], 1e-6, &mut rec).is_err());
    }
}

// #endregion 🔖️Tests
