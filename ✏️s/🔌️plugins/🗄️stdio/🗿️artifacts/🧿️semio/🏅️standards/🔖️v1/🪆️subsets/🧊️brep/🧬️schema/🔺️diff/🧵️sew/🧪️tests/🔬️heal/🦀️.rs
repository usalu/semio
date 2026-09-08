
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::make_box;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;

#[semio_framework_async_macros::async_test]
async fn heal_solid_noop_on_valid_box() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
    let report = heal_solid(&mut body, solid, 1e-4, &mut rec).unwrap();
    assert_eq!(report.total_repairs(), 0);
}

#[semio_framework_async_macros::async_test]
async fn defeature_removes_one_box_face() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
    let face = body.solid_faces(solid)[0];
    let out = defeature(&mut body, solid, std::slice::from_ref(&face), &mut rec).unwrap();
    assert_eq!(out, solid);
    assert_eq!(body.shell_faces(body.solids.get(solid).unwrap().outer).len(), 5);
}

#[semio_framework_async_macros::async_test]
async fn defeature_rejects_empty_selection() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    assert!(defeature(&mut body, solid, &[], &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn defeature_rejects_removing_too_many_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let faces = body.solid_faces(solid);
    assert!(defeature(&mut body, solid, &faces[0..3], &mut rec).is_err());
}

#[semio_framework_async_macros::async_test]
async fn convert_to_nurbs_upgrades_box_planes_and_edges() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let count = convert_to_nurbs(&mut body, solid, &mut rec).unwrap();
    assert!(count >= 6);
    for fid in body.solid_faces(solid) {
        let sid = body.faces.get(fid).unwrap().surface;
        assert!(matches!(body.surfaces.get(sid), Some(Surface::Nurbs { .. })));
    }
    for fid in body.solid_faces(solid) {
        for coedge_id in body.face_coedges(fid) {
            let edge_id = body.coedges.get(coedge_id).unwrap().edge;
            let curve_id = body.edges.get(edge_id).unwrap().curve;
            assert!(matches!(body.curves3.get(curve_id), Some(Curve3::Nurbs { .. })));
        }
    }
}
