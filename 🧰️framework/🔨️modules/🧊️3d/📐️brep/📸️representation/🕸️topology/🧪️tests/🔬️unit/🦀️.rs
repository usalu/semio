use super::*;
use crate::arena::ArenaId;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::Vec3;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn null_coedge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn null_loop() -> LoopId {
    ArenaId::from_raw(0, 0)
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn null_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

// Small test-only builders that pre-fetch `body.new_label()` into a local before the
// `insert(...)` call — calling `body.new_label()` inline as an argument to `body.x.insert(..)`
// is a double mutable borrow of `body` the borrow checker rejects even though the fields are
// disjoint (the two calls are nested, not sequential).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn insert_vertex(body: &mut Body, position: Pnt3) -> VertexId {
    let label = body.new_label();
    body.vertices.insert(Vertex { position, tol: Tol::DEFAULT, label })
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn insert_edge(body: &mut Body, curve: Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId) -> EdgeId {
    let label = body.new_label();
    body.edges.insert(Edge { curve, range, v0, v1, tol: Tol::DEFAULT, label })
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn insert_face(body: &mut Body, surface: SurfaceId) -> FaceId {
    let label = body.new_label();
    body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: false, tol: Tol::DEFAULT, label })
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn insert_shell(body: &mut Body, faces: Vec<FaceId>) -> ShellId {
    let label = body.new_label();
    body.shells.insert(Shell { faces, label })
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn insert_solid(body: &mut Body, outer: ShellId, inners: Vec<ShellId>) -> SolidId {
    let label = body.new_label();
    body.solids.insert(Solid { outer, inners, label })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn make_triangle_loop(body: &mut Body, face: FaceId, positions: [Pnt3; 3]) -> LoopId {
    let vertices: Vec<VertexId> = positions.iter().map(|&p| insert_vertex(body, p)).collect();
    let curves: Vec<Curve3Id> = (0..3)
        .map(|i| {
            let a = positions[i];
            let b = positions[(i + 1) % 3];
            body.curves3.insert(Curve3::Line { origin: a, dir: b - a })
        })
        .collect();
    let edges: Vec<EdgeId> = (0..3).map(|i| insert_edge(body, curves[i], (0.0, 1.0), vertices[i], vertices[(i + 1) % 3])).collect();
    let loop_id = body.loops.insert(Loop { first: null_coedge(), face });
    let coedge_ids: Vec<CoedgeId> = edges.iter().map(|&e| body.coedges.insert(Coedge { edge: e, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() })).collect();
    for i in 0..3 {
        let coedge = body.coedges.get_mut(coedge_ids[i]).unwrap();
        coedge.next = coedge_ids[(i + 1) % 3];
        coedge.prev = coedge_ids[(i + 2) % 3];
    }
    body.loops.get_mut(loop_id).unwrap().first = coedge_ids[0];
    loop_id
}

#[semio_framework_async_macros::async_test]
async fn loop_coedges_walks_the_full_ring_once() {
    let mut body = Body::new();
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let face = insert_face(&mut body, surface);
    let loop_id = make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)]);
    let coedges = body.loop_coedges(loop_id);
    assert_eq!(coedges.len(), 3);
    assert_eq!(coedges[0], body.loops.get(loop_id).unwrap().first);
}

#[semio_framework_async_macros::async_test]
async fn face_loops_includes_outer_and_all_inner_loops() {
    let mut body = Body::new();
    let frame = Frame3::WORLD;
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let face = insert_face(&mut body, surface);
    let outer = make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(10.0, 0.0, 0.0), Pnt3::new(0.0, 10.0, 0.0)]);
    let inner = make_triangle_loop(&mut body, face, [Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(1.0, 2.0, 0.0)]);
    body.faces.get_mut(face).unwrap().outer = Some(outer);
    body.faces.get_mut(face).unwrap().inners = vec![inner];
    let loops = body.face_loops(face);
    assert_eq!(loops.len(), 2);
    assert!(loops.contains(&outer));
    assert!(loops.contains(&inner));
    assert_eq!(body.face_coedges(face).len(), 6);
}

#[semio_framework_async_macros::async_test]
async fn shell_and_solid_traversal_returns_all_members() {
    let mut body = Body::new();
    let frame = Frame3::WORLD;
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let f1 = insert_face(&mut body, surface);
    let f2 = insert_face(&mut body, surface);
    let shell = insert_shell(&mut body, vec![f1, f2]);
    let inner_shell = insert_shell(&mut body, vec![]);
    let solid = insert_solid(&mut body, shell, vec![inner_shell]);
    assert_eq!(body.shell_faces(shell), vec![f1, f2]);
    assert_eq!(body.solid_shells(solid), vec![shell, inner_shell]);
    assert_eq!(body.solid_faces(solid), vec![f1, f2]);
}

#[semio_framework_async_macros::async_test]
async fn coedge_endpoints_respects_orientation() {
    let mut body = Body::new();
    let v0 = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
    let v1 = insert_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0));
    let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
    let edge = insert_edge(&mut body, curve, (0.0, 1.0), v0, v1);
    let loop_id = body.loops.insert(Loop { first: null_coedge(), face: null_face() });
    let fwd = body.coedges.insert(Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() });
    let rev = body.coedges.insert(Coedge { edge, forward: false, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() });
    assert_eq!(body.coedge_endpoints(fwd), Some((v0, v1)));
    assert_eq!(body.coedge_endpoints(rev), Some((v1, v0)));
}

#[semio_framework_async_macros::async_test]
async fn vertex_edges_and_edge_coedges_find_all_incident_entries() {
    let mut body = Body::new();
    let v0 = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
    let v1 = insert_vertex(&mut body, Pnt3::new(1.0, 0.0, 0.0));
    let curve = body.curves3.insert(Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X });
    let edge = insert_edge(&mut body, curve, (0.0, 1.0), v0, v1);
    body.coedges.insert(Coedge { edge, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: null_loop(), next: null_coedge(), prev: null_coedge() });
    body.coedges.insert(Coedge { edge, forward: false, pcurve: None, prange: (0.0, 1.0), loop_id: null_loop(), next: null_coedge(), prev: null_coedge() });
    assert_eq!(body.vertex_edges(v0), vec![edge]);
    assert_eq!(body.vertex_edges(v1), vec![edge]);
    assert_eq!(body.edge_coedges(edge).len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn json_round_trips_a_whole_body() {
    let mut body = Body::new();
    let frame = Frame3::WORLD;
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let face = insert_face(&mut body, surface);
    make_triangle_loop(&mut body, face, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)]);
    let json = semio_framework_pack_json::to_json_string(&body);
    let back: Body = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(back.vertices.len(), body.vertices.len());
    assert_eq!(back.edges.len(), body.edges.len());
    assert_eq!(back.faces.len(), body.faces.len());
}

#[semio_framework_async_macros::async_test]
async fn deep_copy_produces_an_independent_body() {
    let mut body = Body::new();
    let v = insert_vertex(&mut body, Pnt3::new(0.0, 0.0, 0.0));
    let mut copy = body.deep_copy();
    copy.vertices.get_mut(v).unwrap().position = Pnt3::new(9.0, 9.0, 9.0);
    assert_ne!(body.vertices.get(v).unwrap().position, copy.vertices.get(v).unwrap().position);
}
