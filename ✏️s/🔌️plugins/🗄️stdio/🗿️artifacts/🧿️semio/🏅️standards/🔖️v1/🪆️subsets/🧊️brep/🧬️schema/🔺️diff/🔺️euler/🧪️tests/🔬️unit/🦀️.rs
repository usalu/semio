use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_planar_face_from_wire, make_rectangle_wire};
use crate::standards::v1::subsets::brep::schema::inferences::validation_report::validate_body;

/// ✂️ Builds the topology of a unit tetrahedron (4 vertices, 6 edges, 4 triangular faces, 1
/// shell, 1 solid) purely through the checked editors above — the flagship "assemble a real
/// closed solid from scratch" gate for this phase.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn build_tetrahedron(body: &mut Body, rec: &mut OpRecorder) -> SolidId {
    let positions = [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0), Pnt3::new(0.0, 0.0, 1.0)];
    let vertices: Vec<VertexId> = positions.iter().map(|&p| make_vertex(body, p, Tol::DEFAULT, rec)).collect();
    let edge_pairs = [(0, 1), (1, 2), (2, 0), (0, 3), (1, 3), (2, 3)];
    let mut edges = std::collections::HashMap::new();
    for &(a, b) in &edge_pairs {
        let curve = body.curves3.insert(Curve3::Line { origin: positions[a], dir: positions[b] - positions[a] });
        let edge = make_edge(body, curve, (0.0, 1.0), vertices[a], vertices[b], Tol::DEFAULT, rec);
        edges.insert((a, b), edge);
        edges.insert((b, a), edge);
    }
    // Four triangular faces of a tetrahedron with vertex indices 0,1,2,3.
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
async fn tetrahedron_satisfies_euler_poincare_formula() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let vertex_count = body.vertices.len() as i64;
    let edge_count = body.edges.len() as i64;
    let face_count = body.solid_faces(solid).len() as i64;
    assert_eq!(vertex_count, 4);
    assert_eq!(edge_count, 6);
    assert_eq!(face_count, 4);
    assert_eq!(vertex_count - edge_count + face_count, 2, "V - E + F must equal 2 for a genus-0 closed solid");
}

#[semio_framework_async_macros::async_test]
async fn tetrahedron_build_records_every_entity_as_generated() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    build_tetrahedron(&mut body, &mut rec);
    let delta = rec.into_delta();
    assert_eq!(delta.generated.len(), 4 + 6 + 4 + 1 + 1, "vertices + edges + faces + shell + solid");
    assert!(delta.deleted.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn each_face_loop_is_a_closed_ring_of_three_coedges() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    for face in body.solid_faces(solid) {
        let outer = body.faces.get(face).unwrap().outer.unwrap();
        assert_eq!(body.loop_coedges(outer).len(), 3);
    }
}

#[semio_framework_async_macros::async_test]
async fn split_edge_on_a_free_edge_creates_two_edges_and_a_vertex() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let v0 = make_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let v1 = make_vertex(&mut body, Pnt3::new(4.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
    let edge = make_edge(&mut body, curve, (0.0, 4.0), v0, v1, Tol::DEFAULT, &mut rec);
    let (e1, e2, new_vertex) = split_edge(&mut body, edge, 1.5, Pnt3::new(1.5, 0.0, 0.0), &mut rec);
    assert!(!body.edges.contains(edge));
    assert_eq!(body.edges.get(e1).unwrap().v0, v0);
    assert_eq!(body.edges.get(e1).unwrap().v1, new_vertex);
    assert_eq!(body.edges.get(e2).unwrap().v0, new_vertex);
    assert_eq!(body.edges.get(e2).unwrap().v1, v1);
    assert_eq!(body.edges.get(e1).unwrap().range, (0.0, 1.5));
    assert_eq!(body.edges.get(e2).unwrap().range, (1.5, 4.0));
}

#[semio_framework_async_macros::async_test]
async fn split_edge_within_a_loop_ring_preserves_ring_validity() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = build_tetrahedron(&mut body, &mut rec);
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let coedges_before = body.loop_coedges(outer);
    let target_coedge = coedges_before[0];
    let target_edge = body.coedges.get(target_coedge).unwrap().edge;
    let (t0, t1) = body.edges.get(target_edge).unwrap().range;
    let mid = 0.5 * (t0 + t1);
    let curve = body.edges.get(target_edge).unwrap().curve;
    let position = body.curves3.get(curve).unwrap().eval(mid);
    split_edge(&mut body, target_edge, mid, position, &mut rec);
    let coedges_after = body.loop_coedges(outer);
    assert_eq!(coedges_after.len(), coedges_before.len() + 1, "the ring gains exactly one coedge");
    // The ring must still be a single closed cycle covering every live coedge in the loop.
    let mut seen = std::collections::HashSet::new();
    for c in &coedges_after {
        assert!(seen.insert(*c), "ring must not repeat a coedge");
    }
    for c in &coedges_after {
        let co = body.coedges.get(*c).unwrap();
        assert!(coedges_after.contains(&co.next));
        assert!(coedges_after.contains(&co.prev));
    }
}

#[semio_framework_async_macros::async_test]
async fn split_edge_on_a_self_referential_single_coedge_loop_produces_a_valid_two_coedge_ring() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let curve = body.curves3.insert(Curve3::Circle { frame, radius: 1.0 });
    let v = make_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0), Tol::DEFAULT, &mut rec);
    let edge = make_edge(&mut body, curve, (0.0, std::f64::consts::TAU), v, v, Tol::DEFAULT, &mut rec);
    let loop_id = make_loop(&mut body, FaceId::from_raw(0, 0), &[(edge, true)]);
    let coedges_before = body.loop_coedges(loop_id);
    assert_eq!(coedges_before.len(), 1);
    let single = coedges_before[0];
    assert_eq!(body.coedges.get(single).unwrap().next, single);
    assert_eq!(body.coedges.get(single).unwrap().prev, single);
    split_edge(&mut body, edge, std::f64::consts::PI, Pnt3::new(-1.0, 0.0, 0.0), &mut rec);
    let coedges_after = body.loop_coedges(loop_id);
    assert_eq!(coedges_after.len(), 2);
    let a = body.coedges.get(coedges_after[0]).unwrap();
    let b = body.coedges.get(coedges_after[1]).unwrap();
    assert_eq!(a.next, coedges_after[1]);
    assert_eq!(b.next, coedges_after[0]);
    assert_eq!(a.prev, coedges_after[1]);
    assert_eq!(b.prev, coedges_after[0]);
}

#[semio_framework_async_macros::async_test]
async fn split_rectangle_face_into_two() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let wire = make_rectangle_wire(&mut body, 2.0, 2.0, &mut rec).unwrap();
    let face = make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).unwrap();
    let (f0, f1) = split_planar_face_by_line(&mut body, face, Pnt3::new(1.0, -1.0, 0.0), Pnt3::new(1.0, 3.0, 0.0)).unwrap();
    assert_ne!(f0, f1);
    assert!(body.faces.contains(f0));
    assert!(body.faces.contains(f1));
    let loop0 = body.faces.get(f0).unwrap().outer.unwrap();
    let loop1 = body.faces.get(f1).unwrap().outer.unwrap();
    assert_eq!(body.loop_coedges(loop0).len(), 4);
    assert_eq!(body.loop_coedges(loop1).len(), 4);
    assert_eq!(body.faces.len(), 2);
    assert_eq!(body.edges.len(), 7);
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn split_rejects_non_cutting_line() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let wire = make_rectangle_wire(&mut body, 2.0, 2.0, &mut rec).unwrap();
    let face = make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, &mut rec).unwrap();
    let err = split_planar_face_by_line(&mut body, face, Pnt3::new(3.0, 0.0, 0.0), Pnt3::new(3.0, 2.0, 0.0)).unwrap_err();
    assert!(matches!(err, KernelError::Operation(_)));
}

#[semio_framework_async_macros::async_test]
async fn split_rejects_missing_face() {
    let mut body = Body::new();
    let err = split_planar_face_by_line(&mut body, FaceId::from_raw(0, 0), Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0)).unwrap_err();
    assert!(matches!(err, KernelError::MissingEntity(_)));
}

#[semio_framework_async_macros::async_test]
async fn resolve_edge_containing_param_picks_survivor_after_split() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face =
        crate::standards::v1::subsets::brep::schema::diff::primitives::make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(4.0, 0.0, 0.0), Pnt3::new(4.0, 2.0, 0.0), Pnt3::new(0.0, 2.0, 0.0)], &mut rec).expect("rect face");
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let edge = body.coedges.get(body.loop_coedges(outer)[0]).unwrap().edge;
    let e = body.edges.get(edge).unwrap().clone();
    let mid_t = (e.range.0 + e.range.1) * 0.5;
    let mid_p = body.curves3.get(e.curve).unwrap().eval(mid_t);
    let (e1, e2, _) = split_edge(&mut body, edge, mid_t, mid_p, &mut rec);
    let low_t = e.range.0 + (e.range.1 - e.range.0) * 0.25;
    let high_t = e.range.0 + (e.range.1 - e.range.0) * 0.75;
    let low_edge = resolve_edge_containing_param(&body, e1, e2, low_t).expect("low");
    let high_edge = resolve_edge_containing_param(&body, e1, e2, high_t).expect("high");
    assert_ne!(low_edge, high_edge);
}
