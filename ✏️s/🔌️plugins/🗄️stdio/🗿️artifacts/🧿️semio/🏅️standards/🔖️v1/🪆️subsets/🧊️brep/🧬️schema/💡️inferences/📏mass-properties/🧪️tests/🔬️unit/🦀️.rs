
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::{Body, Coedge, Edge, Face, Loop, Shell, Solid, Vertex};
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use std::f64::consts::PI;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn null_coedge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn insert_vertex(body: &mut Body, position: Pnt3) -> VertexId {
    let label = body.new_label();
    body.vertices.insert(Vertex { position, tol: Tol::DEFAULT, label })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn insert_edge(body: &mut Body, curve: crate::standards::v1::subsets::brep::schema::snapshot::arena::Curve3Id, range: (f64, f64), v0: VertexId, v1: VertexId) -> EdgeId {
    let label = body.new_label();
    body.edges.insert(Edge { curve, range, v0, v1, tol: Tol::DEFAULT, label })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn make_quad_loop(body: &mut Body, face: FaceId, corners: [Pnt3; 4]) -> crate::standards::v1::subsets::brep::schema::snapshot::arena::LoopId {
    let verts: Vec<_> = corners.iter().map(|&p| insert_vertex(body, p)).collect();
    let curves: Vec<_> = (0..4)
        .map(|i| {
            let a = corners[i];
            let b = corners[(i + 1) % 4];
            body.curves3.insert(Curve3::Line { origin: a, dir: b - a })
        })
        .collect();
    let edges: Vec<_> = (0..4).map(|i| insert_edge(body, curves[i], (0.0, 1.0), verts[i], verts[(i + 1) % 4])).collect();
    let loop_id = body.loops.insert(Loop { first: null_coedge(), face });
    let coedges: Vec<_> = edges.iter().map(|&e| body.coedges.insert(Coedge { edge: e, forward: true, pcurve: None, prange: (0.0, 1.0), loop_id, next: null_coedge(), prev: null_coedge() })).collect();
    for i in 0..4 {
        let c = body.coedges.get_mut(coedges[i]).unwrap();
        c.next = coedges[(i + 1) % 4];
        c.prev = coedges[(i + 3) % 4];
    }
    body.loops.get_mut(loop_id).unwrap().first = coedges[0];
    loop_id
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn add_planar_face(body: &mut Body, frame: Frame3, corners: [Pnt3; 4], flipped: bool) -> FaceId {
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let label = body.new_label();
    let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped, tol: Tol::DEFAULT, label });
    let loop_id = make_quad_loop(body, face, corners);
    body.faces.get_mut(face).unwrap().outer = Some(loop_id);
    face
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn make_box_solid(body: &mut Body, origin: Pnt3, w: f64, d: f64, h: f64) -> SolidId {
    let o = origin;
    let z0 = Frame3::from_normal(o, -Vec3::Z).unwrap();
    let z1 = Frame3::from_normal(o + Vec3::new(0.0, 0.0, h), Vec3::Z).unwrap();
    let y0 = Frame3::from_normal(o, -Vec3::Y).unwrap();
    let y1 = Frame3::from_normal(o + Vec3::new(0.0, d, 0.0), Vec3::Y).unwrap();
    let x0 = Frame3::from_normal(o, -Vec3::X).unwrap();
    let x1 = Frame3::from_normal(o + Vec3::new(w, 0.0, 0.0), Vec3::X).unwrap();
    let f_bottom = add_planar_face(body, z0, [o, o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, d, 0.0), o + Vec3::new(0.0, d, 0.0)], false);
    let f_top = add_planar_face(body, z1, [o + Vec3::new(0.0, 0.0, h), o + Vec3::new(w, 0.0, h), o + Vec3::new(w, d, h), o + Vec3::new(0.0, d, h)], false);
    let f_front = add_planar_face(body, y0, [o, o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, 0.0, h), o + Vec3::new(0.0, 0.0, h)], false);
    let f_back = add_planar_face(body, y1, [o + Vec3::new(0.0, d, 0.0), o + Vec3::new(0.0, d, h), o + Vec3::new(w, d, h), o + Vec3::new(w, d, 0.0)], false);
    let f_left = add_planar_face(body, x0, [o, o + Vec3::new(0.0, 0.0, h), o + Vec3::new(0.0, d, h), o + Vec3::new(0.0, d, 0.0)], false);
    let f_right = add_planar_face(body, x1, [o + Vec3::new(w, 0.0, 0.0), o + Vec3::new(w, d, 0.0), o + Vec3::new(w, d, h), o + Vec3::new(w, 0.0, h)], false);
    let label = body.new_label();
    let shell = body.shells.insert(Shell { faces: vec![f_bottom, f_top, f_front, f_back, f_left, f_right], label });
    let solid_label = body.new_label();
    body.solids.insert(Solid { outer: shell, inners: vec![], label: solid_label })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn make_uv_sphere(body: &mut Body, radius: f64, n_long: usize, n_lat: usize) -> SolidId {
    let frame = Frame3::WORLD;
    let surface = body.surfaces.insert(Surface::Sphere { frame, radius });
    let mut faces = Vec::new();
    for i in 0..n_lat {
        let v0 = -PI / 2.0 + PI * (i as f64) / n_lat as f64;
        let v1 = -PI / 2.0 + PI * ((i + 1) as f64) / n_lat as f64;
        for j in 0..n_long {
            let u0 = TAU * (j as f64) / n_long as f64;
            let u1 = TAU * ((j + 1) as f64) / n_long as f64;
            let corners = [sphere_corner(&frame, radius, u0, v0), sphere_corner(&frame, radius, u1, v0), sphere_corner(&frame, radius, u1, v1), sphere_corner(&frame, radius, u0, v1)];
            let label = body.new_label();
            let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: false, tol: Tol::DEFAULT, label });
            let loop_id = make_quad_loop(body, face, corners);
            body.faces.get_mut(face).unwrap().outer = Some(loop_id);
            faces.push(face);
        }
    }
    let label = body.new_label();
    let shell = body.shells.insert(Shell { faces, label });
    let solid_label = body.new_label();
    body.solids.insert(Solid { outer: shell, inners: vec![], label: solid_label })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sphere_corner(frame: &Frame3, radius: f64, u: f64, v: f64) -> Pnt3 {
    Surface::Sphere { frame: *frame, radius }.eval(u, v)
}

const TAU: f64 = 2.0 * PI;

#[semio_framework_async_macros::async_test]
async fn unit_box_volume_and_area() {
    let mut body = Body::new();
    let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    let vol = solid_volume(&body, solid, 0.1).unwrap();
    let area = solid_surface_area(&body, solid, 0.1).unwrap();
    assert!((vol - 1.0).abs() < 1e-9, "volume {vol}");
    assert!((area - 6.0).abs() < 1e-9, "area {area}");
}

#[semio_framework_async_macros::async_test]
async fn box_mass_properties_and_bbox() {
    let mut body = Body::new();
    let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 2.0, 3.0, 4.0);
    let vol = solid_volume(&body, solid, 0.1).unwrap();
    assert!((vol - 24.0).abs() < 1e-8);
    let com = solid_center_of_mass(&body, solid, 0.1).unwrap();
    assert!((com.x - 1.0).abs() < 1e-8);
    assert!((com.y - 1.5).abs() < 1e-8);
    assert!((com.z - 2.0).abs() < 1e-8);
    let bb = solid_bounding_box(&body, solid).unwrap();
    assert!((bb.min.x - 0.0).abs() < 1e-9);
    assert!((bb.max.x - 2.0).abs() < 1e-9);
    assert!((bb.max.z - 4.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn edge_length_on_unit_box() {
    let mut body = Body::new();
    let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    let face = body.solid_faces(solid)[0];
    let coedge = body.loop_coedges(body.faces.get(face).unwrap().outer.unwrap())[0];
    let edge = body.coedges.get(coedge).unwrap().edge;
    let len = edge_length(&body, edge).unwrap();
    assert!((len - 1.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn sphere_volume_coarse_tessellation() {
    let mut body = Body::new();
    let r = 2.0;
    let solid = make_uv_sphere(&mut body, r, 12, 8);
    let vol = solid_volume(&body, solid, 0.15).unwrap();
    let expected = 4.0 / 3.0 * PI * r * r * r;
    assert!((vol - expected).abs() < 0.02 * expected, "vol {vol} expected {expected}");
}

#[semio_framework_async_macros::async_test]
async fn distance_and_closest_point_between_boxes() {
    let mut body = Body::new();
    let a = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    let b = make_box_solid(&mut body, Pnt3::new(3.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    let d = distance_solid_solid(&body, a, b).unwrap();
    assert!((d - 2.0).abs() < 0.25, "distance {d}");
    let (cp, dist) = closest_point_on_solid(&body, b, Pnt3::new(0.5, 0.5, 0.5)).unwrap();
    assert!(dist > 1.5 && dist < 3.5, "dist {dist}");
    assert!(cp.x > 2.5);
}

#[semio_framework_async_macros::async_test]
async fn box_classifies_via_the_one_authoritative_classifier() {
    use crate::standards::v1::subsets::brep::schema::engine::PointClassification;
    use crate::standards::v1::subsets::brep::schema::inferences::classification::point_in_solid;
    let mut body = Body::new();
    let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    assert_eq!(point_in_solid(&body, solid, Pnt3::new(0.5, 0.5, 0.5), 1e-9).unwrap(), PointClassification::Inside);
    assert_eq!(point_in_solid(&body, solid, Pnt3::new(2.0, 2.0, 2.0), 1e-9).unwrap(), PointClassification::Outside);
}

#[semio_framework_async_macros::async_test]
async fn face_area_unit_square() {
    let mut body = Body::new();
    let frame = Frame3::WORLD;
    let face = add_planar_face(&mut body, frame, [Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(1.0, 0.0, 0.0), Pnt3::new(1.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], false);
    let area = face_area(&body, face, 0.1).unwrap();
    assert!((area - 1.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn solid_mass_properties_box_uses_the_analytic_fast_path_and_matches_closed_form() {
    let mut body = Body::new();
    let solid = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 2.0, 3.0, 4.0);
    let mp = solid_mass_properties(&body, solid, 1e-4).unwrap();
    assert!((mp.volume - 24.0).abs() < 1e-9, "volume {}", mp.volume);
    assert!((mp.area - 52.0).abs() < 1e-9, "area {}", mp.area);
    assert!((mp.centroid.x - 1.0).abs() < 1e-9 && (mp.centroid.y - 1.5).abs() < 1e-9 && (mp.centroid.z - 2.0).abs() < 1e-9);
    let expected_ixx = mp.volume * (3.0 * 3.0 + 4.0 * 4.0) / 12.0;
    assert!((mp.inertia[0][0] - expected_ixx).abs() < 1e-6, "ixx {} expected {expected_ixx}", mp.inertia[0][0]);
    assert_eq!(mp.error_estimate, 0.0, "the box analytic fast path is exact");
}

#[semio_framework_async_macros::async_test]
async fn solid_mass_properties_sphere_matches_closed_form() {
    let mut body = Body::new();
    let r = 2.0;
    let solid = make_uv_sphere(&mut body, r, 12, 8);
    let mp = solid_mass_properties(&body, solid, 1e-4).unwrap();
    let expected_vol = 4.0 / 3.0 * PI * r * r * r;
    let expected_area = 4.0 * PI * r * r;
    assert!((mp.volume - expected_vol).abs() < 0.02 * expected_vol, "vol {} expected {expected_vol}", mp.volume);
    assert!((mp.area - expected_area).abs() < 0.02 * expected_area, "area {} expected {expected_area}", mp.area);
    assert!(mp.centroid.to_vec().norm() < 1e-6, "sphere centroid should be at the origin, got {:?}", mp.centroid);
}

#[semio_framework_async_macros::async_test]
async fn solid_mass_properties_cylinder_general_path_matches_closed_form_within_error_estimate() {
    use crate::standards::v1::subsets::brep::schema::diff::primitives::make_cylinder;
    use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let radius = 1.0;
    let height = 3.0;
    let solid = make_cylinder(&mut body, radius, height, &mut rec).unwrap();
    let mp = solid_mass_properties(&body, solid, 1e-4).unwrap();
    let expected_vol = PI * radius * radius * height;
    assert!((mp.volume - expected_vol).abs() < 0.05 * expected_vol, "vol {} expected {expected_vol}", mp.volume);
    assert!(mp.error_estimate.is_finite() && mp.error_estimate >= 0.0);
}

#[semio_framework_async_macros::async_test]
async fn distance_solid_solid_overlap_returns_zero_via_real_classifier() {
    let mut body = Body::new();
    let a = make_box_solid(&mut body, Pnt3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0);
    let b = make_box_solid(&mut body, Pnt3::new(1.0, 1.0, 1.0), 2.0, 2.0, 2.0);
    let d = distance_solid_solid(&body, a, b).unwrap();
    assert_eq!(d, 0.0, "overlapping boxes must report zero distance");
}
