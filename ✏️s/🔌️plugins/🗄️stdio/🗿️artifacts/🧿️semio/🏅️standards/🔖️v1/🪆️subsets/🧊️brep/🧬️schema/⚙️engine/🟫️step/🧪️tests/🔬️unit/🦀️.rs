use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::make_box;

#[semio_framework_async_macros::async_test]
async fn box_round_trip_topology_counts() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
    let step = write_step(&body, &[solid]).unwrap();
    assert!(step.contains("MANIFOLD_SOLID_BREP"));
    assert!(step.contains("ADVANCED_FACE"));
    let read = read_step(&step).unwrap();
    assert_eq!(read.vertices.len(), 8);
    assert_eq!(read.edges.len(), 12);
    assert_eq!(read.faces.len(), 6);
    assert_eq!(read.solids.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn write_step_rejects_empty_solids() {
    let body = Body::new();
    assert!(write_step(&body, &[]).is_err());
}

#[semio_framework_async_macros::async_test]
async fn read_step_rejects_missing_data_section() {
    assert!(read_step("ISO-10303-21;").is_err());
}
