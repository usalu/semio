use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DxfSnapshot::default();
    assert_eq!(DxfInference::infer(&snapshot).expect("valid materialized inference fixture"), DxfInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DxfInference::infer(&DxfSnapshot::default()).expect("valid materialized inference fixture"), DxfInference::default());
}
