use crate::PlySnapshot;
use crate::standards::v1_0::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = PlySnapshot::default();
    assert_eq!(PlyInference::infer(&snapshot).expect("valid materialized inference fixture"), PlyInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(PlyInference::infer(&PlySnapshot::default()).expect("valid materialized inference fixture"), PlyInference::default());
}
