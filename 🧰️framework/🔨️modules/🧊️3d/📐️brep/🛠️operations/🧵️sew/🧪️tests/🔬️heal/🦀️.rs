use semio_framework_3d_brep_construction::sew::*;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::{Body, EntityRef};
use semio_framework_3d_brep_construction::primitives::make_box;
use semio_framework_3d_brep_curve::Curve3;

#[semio_framework_async_macros::async_test]
async fn heal_solid_noop_on_valid_box() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
    let report = heal_solid(&mut body, solid, 1e-4, &mut rec).unwrap();
    assert_eq!(report.total_repairs(), 0);
}

#[semio_framework_async_macros::async_test]
async fn healing_a_selected_solid_preserves_unrelated_vertices_and_refuses_atomically() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️scope.json")).unwrap();
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let [width, depth, height]: [f64; 3] = serde_json::from_value(fixture["selectedSize"].clone()).unwrap();
    let target = make_box(&mut body, width, depth, height, &mut rec).unwrap();
    let [width, depth, height]: [f64; 3] = serde_json::from_value(fixture["otherSize"].clone()).unwrap();
    let other = make_box(&mut body, width, depth, height, &mut rec).unwrap();

    let reach = body.reachable_from(&[EntityRef::Solid(other)]);
    let other_before = reach.vertices.iter().map(|&id| (id, body.vertices.get(id).unwrap().position)).collect::<Vec<_>>();
    let healed = heal_solid(&mut body, target, fixture["healingTolerance"].as_f64().unwrap(), &mut rec);
    assert!(other_before.iter().all(|&(id, point)| body.vertices.get(id).unwrap().position == point));
    assert_eq!(healed.unwrap().total_repairs(), 0);

    let before = body.vertices.iter().map(|(id, vertex)| (id, vertex.position)).collect::<Vec<_>>();
    assert!(heal_solid(&mut body, target, fixture["collapseTolerance"].as_f64().unwrap(), &mut rec).is_err());
    assert!(before.iter().all(|&(id, point)| body.vertices.get(id).unwrap().position == point));
}

#[semio_framework_async_macros::async_test]
async fn defeature_stock_stage_constructs_closed_boundary() {
    use semio_framework_3d_brep_construction::{primitives::SolidPrimitive, staged::drive_solid};
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let stock = SolidPrimitive::axis_box(2.0, 2.0, 2.0);
    let solid = drive_solid(&mut DefeatureJob::restoring_stock(stock).unwrap(), &mut body, &mut rec).unwrap();
    let reach = body.reachable_from(&[EntityRef::Solid(solid)]);
    assert_eq!(reach.faces.len(), 6);
    for edge in reach.edges {
        let uses = body.edge_coedges(edge);
        assert_eq!(uses.len(), 2);
        assert_ne!(body.coedges.get(uses[0]).unwrap().forward, body.coedges.get(uses[1]).unwrap().forward);
    }
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
