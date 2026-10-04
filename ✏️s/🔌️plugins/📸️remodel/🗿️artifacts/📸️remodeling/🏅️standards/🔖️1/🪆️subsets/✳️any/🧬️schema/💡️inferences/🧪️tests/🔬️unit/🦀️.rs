use super::*;
use crate::standards::v1::subsets::any::io::seed_remodeling_mesh;
use protocol::Inference;
use semio_framework::MeshData;

fn triangle_snapshot() -> RemodelingSnapshot {
    // 🧱️ Durable content, the only mesh source `bounds` resolves in production (see its doc comment).
    let mut snapshot = RemodelingSnapshot::default();
    seed_remodeling_mesh(&mut snapshot, &MeshData { positions: vec![0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 0.0], indices: vec![0, 1, 2], ..MeshData::default() }).expect("a single triangle is inside the bounded envelope");
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = triangle_snapshot();
    assert_eq!(RemodelingInference::infer(&snapshot).expect("valid materialized inference fixture"), RemodelingInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(RemodelingInference::infer(&RemodelingSnapshot::default()).expect("valid materialized inference fixture"), RemodelingInference::default());
}

#[semio_framework_async_macros::async_test]
async fn bounds_covers_the_mesh_vertices_and_counts_it_exactly() {
    let inferred = RemodelingInference::infer(&triangle_snapshot()).expect("valid materialized inference fixture");
    assert_eq!(inferred.bounds.vertex_count, 3);
    assert_eq!(inferred.bounds.face_count, 1);
    assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0, 0.0]);
    assert_eq!(inferred.bounds.bounding_box.max, [2.0, 3.0, 0.0]);
}
