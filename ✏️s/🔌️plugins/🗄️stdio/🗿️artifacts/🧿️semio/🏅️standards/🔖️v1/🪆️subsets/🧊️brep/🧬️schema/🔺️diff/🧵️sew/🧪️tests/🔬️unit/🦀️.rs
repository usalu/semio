
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn make_loose_quad(body: &mut Body, p0: Pnt3, p1: Pnt3, p2: Pnt3, p3: Pnt3, normal: Vec3) -> FaceId {
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let frame = Frame3::from_normal(p0, normal).expect("plane frame");
    let surface = body.surfaces.insert(Surface::Plane { frame });
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

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn unique_edges_on_solid(body: &Body, solid: SolidId) -> usize {
    let mut edges = HashSet::new();
    for fid in body.solid_faces(solid) {
        for cid in body.face_coedges(fid) {
            let e = body.coedges.get(cid).unwrap().edge;
            edges.insert((e.raw_index(), e.raw_generation()));
        }
    }
    edges.len()
}

#[semio_framework_async_macros::async_test]
async fn sew_two_adjacent_quads_shares_one_edge() {
    let mut body = Body::new();
    let f0 = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Vec3::Z);
    let f1 = make_loose_quad(&mut body, Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Vec3::Z);
    let mut rec = OpRecorder::new();
    let solid = sew_faces(&mut body, &[f0, f1], 1e-6, &mut rec).unwrap();
    assert_eq!(body.solid_faces(solid).len(), 2);
    assert_eq!(unique_edges_on_solid(&body, solid), 7);
}

#[semio_framework_async_macros::async_test]
async fn sew_six_box_faces_into_solid() {
    let mut body = Body::new();
    let bottom = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), -Vec3::Z);
    let top = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(0.0, 1.0, 1.0), Vec3::Z);
    let front = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 1.0), Pnt3::new(0.0, 0.0, 1.0), -Vec3::Y);
    let back = make_loose_quad(&mut body, Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(0.0, 1.0, 1.0), Vec3::Y);
    let left = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 1.0), Pnt3::new(0.0, 0.0, 1.0), -Vec3::X);
    let right = make_loose_quad(&mut body, Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(1.0, 1.0, 1.0), Pnt3::new(1.0, 0.0, 1.0), Vec3::X);
    let mut rec = OpRecorder::new();
    let solid = sew_faces(&mut body, &[bottom, top, front, back, left, right], 1e-6, &mut rec).unwrap();
    assert_eq!(body.solid_faces(solid).len(), 6);
    assert_eq!(unique_edges_on_solid(&body, solid), 12);
}

#[semio_framework_async_macros::async_test]
async fn sew_single_face_rejects() {
    let mut body = Body::new();
    let f = make_loose_quad(&mut body, Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Vec3::Z);
    let mut rec = OpRecorder::new();
    assert!(sew_faces(&mut body, &[f], 1e-6, &mut rec).is_err());
}
