use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioMeshSnapshot::default();
    assert_eq!(SemioMeshInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioMeshInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioMeshInference::infer(&SemioMeshSnapshot::default()).expect("valid materialized inference fixture"), SemioMeshInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_covers_every_primitive_by_composite_key() {
    use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
    use crate::schema::snapshot::{SemioMesh, SemioPrimitive};
    let snapshot = SemioMeshSnapshot { meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), positions: vec![SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }], ..Default::default() }] }], ..Default::default() };
    let inference = SemioMeshInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert!(inference.aabb.contains_key(&aabb_key("m1", "p1")));
}
