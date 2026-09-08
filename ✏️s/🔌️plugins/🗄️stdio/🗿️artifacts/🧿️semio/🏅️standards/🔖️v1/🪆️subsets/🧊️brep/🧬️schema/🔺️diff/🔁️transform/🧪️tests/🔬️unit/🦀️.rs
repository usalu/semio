
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere};
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

#[semio_framework_async_macros::async_test]
async fn transform_solid_preserves_face_and_edge_counts_for_a_box() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let box_id = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
    let faces_before = body.solid_faces(box_id).len();
    let edges_before: std::collections::HashSet<EdgeId> = body.solid_faces(box_id).into_iter().flat_map(|f| body.face_coedges(f)).filter_map(|c| body.coedges.get(c).map(|co| co.edge)).collect();
    let map = Affine3::rotation_about(Pnt3::new(1.0, 0.0, 0.0), Vec3::new(0.2, 1.0, 0.3), 0.7).compose(&Affine3::translation(Vec3::new(3.0, -1.0, 2.0)));
    let transformed = transform_solid(&mut body, box_id, &map, &mut rec).unwrap();
    let faces_after = body.solid_faces(transformed).len();
    let edges_after: std::collections::HashSet<EdgeId> = body.solid_faces(transformed).into_iter().flat_map(|f| body.face_coedges(f)).filter_map(|c| body.coedges.get(c).map(|co| co.edge)).collect();
    assert_eq!(faces_before, faces_after);
    assert_eq!(edges_before.len(), edges_after.len());
}

#[semio_framework_async_macros::async_test]
async fn transform_solid_keeps_analytic_surface_kinds_for_a_cylinder_under_similarity() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let cyl = make_cylinder(&mut body, 1.5, 4.0, &mut rec).unwrap();
    let map = Affine3::rotation_about(Pnt3::new(0.0, 0.0, 0.0), Vec3::Y, 0.5).compose(&Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0)));
    let transformed = transform_solid(&mut body, cyl, &map, &mut rec).unwrap();
    for f in body.solid_faces(transformed) {
        let surface_id = body.faces.get(f).unwrap().surface;
        let surface = body.surfaces.get(surface_id).unwrap();
        assert!(matches!(surface, Surface::Cylinder { .. } | Surface::Plane { .. }), "expected an analytic surface kind, got {surface:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn transform_solid_scales_volume_by_the_determinant_magnitude() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let sphere = make_sphere(&mut body, 2.0, &mut rec).unwrap();
    let volume_before = solid_volume(&body, sphere, 1e-4).unwrap();
    let map = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 3.0));
    let transformed = transform_solid(&mut body, sphere, &map, &mut rec).unwrap();
    let volume_after = solid_volume(&body, transformed, 1e-4).unwrap();
    let expected = volume_before * map.determinant().abs();
    assert!((volume_after - expected).abs() / expected < 1e-3, "volume {volume_after} vs expected {expected}");
}

#[semio_framework_async_macros::async_test]
async fn transform_solid_rotate_then_inverse_rotate_round_trips_vertex_positions() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let box_id = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let original_positions: Vec<Pnt3> = body
        .solid_faces(box_id)
        .into_iter()
        .flat_map(|f| body.face_coedges(f))
        .filter_map(|c| body.coedges.get(c).map(|co| co.edge))
        .filter_map(|e| body.edges.get(e).map(|edge| edge.v0))
        .filter_map(|v| body.vertices.get(v).map(|vertex| vertex.position))
        .collect();
    let map = Affine3::rotation_about(Pnt3::new(0.5, 0.5, 0.5), Vec3::new(1.0, 2.0, 3.0), 1.234);
    let inverse = map.inverse().unwrap();
    let rotated = transform_solid(&mut body, box_id, &map, &mut rec).unwrap();
    let back = transform_solid(&mut body, rotated, &inverse, &mut rec).unwrap();
    let back_positions: Vec<Pnt3> = body
        .solid_faces(back)
        .into_iter()
        .flat_map(|f| body.face_coedges(f))
        .filter_map(|c| body.coedges.get(c).map(|co| co.edge))
        .filter_map(|e| body.edges.get(e).map(|edge| edge.v0))
        .filter_map(|v| body.vertices.get(v).map(|vertex| vertex.position))
        .collect();
    assert_eq!(original_positions.len(), back_positions.len());
    for p in &back_positions {
        let closest = original_positions.iter().map(|o| o.distance(*p)).fold(f64::INFINITY, f64::min);
        assert!(closest < 1e-9, "round-tripped vertex {p:?} did not land within 1e-9 of an original vertex");
    }
}

#[semio_framework_async_macros::async_test]
async fn copy_solid_is_identical_geometry_at_a_fresh_identity() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let box_id = make_box(&mut body, 2.0, 1.0, 1.0, &mut rec).unwrap();
    let copy = copy_solid(&mut body, box_id, &mut rec).unwrap();
    assert_ne!(box_id, copy);
    let volume_original = solid_volume(&body, box_id, 1e-4).unwrap();
    let volume_copy = solid_volume(&body, copy, 1e-4).unwrap();
    assert!((volume_original - volume_copy).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn transform_solid_generated_delta_covers_every_new_face() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let box_id = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let mut transform_rec = OpRecorder::new();
    let transformed = transform_solid(&mut body, box_id, &Affine3::translation(Vec3::new(1.0, 0.0, 0.0)), &mut transform_rec).unwrap();
    let delta = transform_rec.into_delta();
    let new_labels: std::collections::HashSet<PersistentLabel> = body.solid_faces(transformed).into_iter().filter_map(|f| body.faces.get(f).map(|face| face.label)).collect();
    for label in new_labels {
        assert!(delta.generated.contains(&label), "face label {label:?} missing from generated delta");
    }
}

#[semio_framework_async_macros::async_test]
async fn transform_solid_in_place_records_modified_not_generated() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let box_id = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let mut transform_rec = OpRecorder::new();
    transform_solid_in_place(&mut body, box_id, &Affine3::translation(Vec3::new(2.0, 0.0, 0.0)), &mut transform_rec).unwrap();
    let delta = transform_rec.into_delta();
    assert!(delta.generated.is_empty(), "in-place transform must never generate new entities");
    assert!(!delta.modified.is_empty());
}
