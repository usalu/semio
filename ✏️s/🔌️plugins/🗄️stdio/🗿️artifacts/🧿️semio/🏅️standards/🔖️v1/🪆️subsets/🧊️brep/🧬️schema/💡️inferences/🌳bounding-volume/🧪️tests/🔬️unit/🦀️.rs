
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use std::collections::HashMap;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
    let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
    let vertices: Vec<_> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
    let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
    let mut edges = HashMap::new();
    for &(a, b) in &edge_pairs {
        let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
        let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
        edges.insert((a, b), edge);
        edges.insert((b, a), edge);
    }
    let face_defs = [[0, 1, 2], [0, 3, 1], [1, 3, 2], [2, 3, 0]];
    let mut faces = Vec::new();
    for tri in face_defs {
        let normal = (positions[tri[1]] - positions[tri[0]]).cross(positions[tri[2]] - positions[tri[0]]);
        let frame = Frame3::from_normal(positions[tri[0]], normal).unwrap();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let members: Vec<(EdgeId, bool)> = (0..3)
            .map(|i| {
                let a = tri[i];
                let b = tri[(i + 1) % 3];
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
async fn face_bvh_builds_over_tetrahedron_with_four_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let bvh = build_face_bvh(&body, solid).unwrap();
    let hits = bvh.query_ray([-0.5, 0.25, 0.25], [1.0, 0.0, 0.0]);
    assert!(!hits.is_empty(), "ray through tetrahedron should hit at least one face leaf");
    for face in &hits {
        assert!(body.solid_faces(solid).contains(face));
    }
    let near = bvh.query_nearest([0.25, 0.25, 0.25]).unwrap();
    assert!(body.solid_faces(solid).contains(&near));
}

#[semio_framework_async_macros::async_test]
async fn edge_bvh_builds_six_edges_on_tetrahedron() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let bvh = build_edge_bvh(&body, solid).unwrap();
    let probe = Aabb { min: [0.4, 0.0, 0.0], max: [0.6, 0.1, 0.1] };
    let hits = bvh.query_aabb(&probe);
    assert!(!hits.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn empty_solid_yields_empty_face_bvh_queries() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let shell = add_shell(&mut body, vec![], &mut rec);
    let solid = add_solid(&mut body, shell, vec![], &mut rec);
    let bvh = build_face_bvh(&body, solid).unwrap();
    assert!(bvh.query_ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).is_empty());
    assert!(bvh.query_aabb(&Aabb { min: [0.0, 0.0, 0.0], max: [1.0, 1.0, 1.0] }).is_empty());
    assert!(bvh.query_nearest([0.5, 0.5, 0.5]).is_none());
}

#[semio_framework_async_macros::async_test]
async fn missing_solid_returns_kernel_error() {
    let body = Body::new();
    let bogus = SolidId::from_raw(9, 9);
    assert!(matches!(build_face_bvh(&body, bogus), Err(KernelError::MissingEntity(_))));
    assert!(matches!(build_edge_bvh(&body, bogus), Err(KernelError::MissingEntity(_))));
}

#[semio_framework_async_macros::async_test]
async fn face_bvh_query_ray_ordered_is_sorted_near_to_far() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let bvh = build_face_bvh(&body, solid).unwrap();
    let hits = bvh.query_ray_ordered([-0.5, 0.25, 0.25], [1.0, 0.0, 0.0]);
    assert!(!hits.is_empty());
    for w in hits.windows(2) {
        assert!(w[0].1 <= w[1].1);
    }
}

#[semio_framework_async_macros::async_test]
async fn face_bvh_closest_face_finds_true_nearest_not_just_aabb_nearest() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let bvh = build_face_bvh(&body, solid).unwrap();
    let probe = Pnt3::new(0.2, 0.2, -0.5);
    let (face, closest, dist) = bvh.closest_face(&body, probe).unwrap();
    assert!(body.solid_faces(solid).contains(&face));
    assert!(dist > 0.0 && dist.is_finite());
    assert!((closest.z - 0.0).abs() < 1e-6, "closest point on the z=0 base face should have z≈0, got {closest:?}");
}

#[semio_framework_async_macros::async_test]
async fn solid_bvh_wraps_face_index_by_solid_id() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let sbvh = SolidBvh::build(&body, solid).unwrap();
    assert_eq!(sbvh.solid, solid);
    let hits = sbvh.faces().query_ray([-0.5, 0.25, 0.25], [1.0, 0.0, 0.0]);
    assert!(!hits.is_empty());
    let (face, _, dist) = sbvh.closest_face(&body, Pnt3::new(0.2, 0.2, -0.5)).unwrap();
    assert!(body.solid_faces(solid).contains(&face));
    assert!(dist.is_finite());
}

#[semio_framework_async_macros::async_test]
async fn face_bvh_refit_reflects_moved_geometry() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let mut bvh = build_face_bvh(&body, solid).unwrap();
    for (_, v) in body.vertices.iter_mut() {
        v.position.x += 10.0;
    }
    bvh.refit(&body);
    let hits = bvh.query_ray([9.5, 0.25, 0.25], [1.0, 0.0, 0.0]);
    assert!(!hits.is_empty(), "refit BVH should find faces at the moved location");
}
