use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::make_box;
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn box_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut edges: Vec<EdgeId> = solid_edge_set(body, solid).into_iter().collect();
    edges.sort_by_key(|e| format!("{e:?}"));
    edges
}

#[semio_framework_async_macros::async_test]
async fn fillet_one_box_edge_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (w, d, h, r) = (2.0, 2.0, 2.0, 0.3);
    let solid = make_box(&mut body, w, d, h, &mut rec).unwrap();
    let v0 = solid_volume(&body, solid, 1e-6).unwrap();
    let edge = box_edges(&body, solid)[0];
    let len = edge_length(&body, edge).unwrap();
    let out = fillet_edges(&mut body, solid, &[edge], r, &mut rec).unwrap();
    let v1 = solid_volume(&body, out, 1e-4).unwrap();
    let closed_form = v0 - len * (r * r - std::f64::consts::PI * r * r / 4.0);
    assert!((v1 - closed_form).abs() < 1e-2 * closed_form.max(1.0), "v1={v1} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn fillet_all_box_edges_is_valid_and_decreases_volume() {
    // Known gap (see 📓️w2d-blends-offsets-draft.md): vertex-blend spherical corner patches
    // are not implemented, so a full 12-edge fillet leaves the 8 corners without a proper
    // rounded-corner patch — this asserts the weaker, honest property (a valid solid, strictly
    // less volume than the box, still positive) rather than the exact rounded-box closed form.
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 3.0, 3.0, 3.0, &mut rec).unwrap();
    let v0 = solid_volume(&body, solid, 1e-6).unwrap();
    let edges = box_edges(&body, solid);
    assert_eq!(edges.len(), 12);
    let out = fillet_edges(&mut body, solid, &edges, 0.3, &mut rec).unwrap();
    assert!(!body.solid_faces(out).is_empty());
    let v1 = solid_volume(&body, out, 1e-4).unwrap();
    assert!(v1 > 0.0 && v1 < v0, "v0={v0} v1={v1}");
}

#[semio_framework_async_macros::async_test]
async fn fillet_plane_cylinder_junction_decreases_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = crate::standards::v1::subsets::brep::schema::diff::primitives::make_cylinder(&mut body, 1.0, 2.0, &mut rec).unwrap();
    let v0 = solid_volume(&body, solid, 1e-4).unwrap();
    let solid_faces: HashSet<FaceId> = body.solid_faces(solid).into_iter().collect();
    let edge = solid_edge_set(&body, solid).into_iter().find(|&e| edge_two_faces(&body, &solid_faces, e).is_ok()).expect("a real dihedral edge (lateral/cap) exists");
    let out = fillet_edges(&mut body, solid, &[edge], 0.2, &mut rec).unwrap();
    let v1 = solid_volume(&body, out, 1e-4).unwrap();
    assert!(v1 > 0.0 && v1 < v0, "v0={v0} v1={v1}");
}

#[semio_framework_async_macros::async_test]
async fn chamfer_asymmetric_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (w, d, h, d1, d2) = (2.0, 2.0, 2.0, 0.2, 0.35);
    let solid = make_box(&mut body, w, d, h, &mut rec).unwrap();
    let v0 = solid_volume(&body, solid, 1e-6).unwrap();
    let edge = box_edges(&body, solid)[0];
    let len = edge_length(&body, edge).unwrap();
    let out = chamfer_edges(&mut body, solid, &[edge], d1, d2, &mut rec).unwrap();
    let v1 = solid_volume(&body, out, 1e-4).unwrap();
    let closed_form = v0 - 0.5 * d1 * d2 * len;
    assert!((v1 - closed_form).abs() < 1e-2 * closed_form.max(1.0), "v1={v1} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn variable_fillet_is_monotone_in_radius() {
    let mut body_a = Body::new();
    let mut rec_a = OpRecorder::new();
    let solid_a = make_box(&mut body_a, 2.0, 2.0, 2.0, &mut rec_a).unwrap();
    let edge_a = box_edges(&body_a, solid_a)[0];
    let out_a = fillet_variable(&mut body_a, solid_a, edge_a, 0.1, 0.2, &mut rec_a).unwrap();
    let v_a = solid_volume(&body_a, out_a, 1e-4).unwrap();

    let mut body_b = Body::new();
    let mut rec_b = OpRecorder::new();
    let solid_b = make_box(&mut body_b, 2.0, 2.0, 2.0, &mut rec_b).unwrap();
    let edge_b = box_edges(&body_b, solid_b)[0];
    let out_b = fillet_variable(&mut body_b, solid_b, edge_b, 0.1, 0.4, &mut rec_b).unwrap();
    let v_b = solid_volume(&body_b, out_b, 1e-4).unwrap();

    assert!(v_b < v_a, "growing the far-end radius should remove strictly more material: v_a={v_a} v_b={v_b}");
}

#[semio_framework_async_macros::async_test]
async fn fillet_determinism() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let edge = box_edges(&body, solid)[0];
    let a = fillet_edges(&mut body, solid, &[edge], 0.2, &mut rec).unwrap();
    let solid2 = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let edge2 = box_edges(&body, solid2).into_iter().find(|&e| (edge_length(&body, e).unwrap() - edge_length(&body, edge).unwrap()).abs() < 1e-9).unwrap();
    let b = fillet_edges(&mut body, solid2, &[edge2], 0.2, &mut rec).unwrap();
    assert_eq!(body.solid_faces(a).len(), body.solid_faces(b).len());
    let va = solid_volume(&body, a, 1e-4).unwrap();
    let vb = solid_volume(&body, b, 1e-4).unwrap();
    assert!((va - vb).abs() < 1e-6, "va={va} vb={vb}");
}

#[semio_framework_async_macros::async_test]
async fn reject_zero_radius_and_empty_edges() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let edge = box_edges(&body, solid)[0];
    assert!(fillet_edges(&mut body, solid, &[edge], 0.0, &mut rec).is_err());
    assert!(chamfer_edges(&mut body, solid, &[edge], 0.0, 0.1, &mut rec).is_err());
    assert!(fillet_variable(&mut body, solid, edge, 0.0, 0.1, &mut rec).is_err());
    assert!(fillet_edges(&mut body, solid, &[], 0.1, &mut rec).is_err());
    assert!(chamfer_edges(&mut body, solid, &[], 0.1, 0.1, &mut rec).is_err());
}
