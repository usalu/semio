
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioMeshSnapshot::default();
    assert_eq!(SemioMeshInference::infer(&snapshot), SemioMeshInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioMeshInference::infer(&SemioMeshSnapshot::default()), SemioMeshInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_covers_every_primitive_by_composite_key() {
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
    use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
    let snapshot = SemioMeshSnapshot { meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), positions: vec![SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }], ..Default::default() }] }], ..Default::default() };
    let inference = SemioMeshInference::infer(&snapshot);
    assert!(inference.aabb.contains_key(&aabb_key("m1", "p1")));
}
