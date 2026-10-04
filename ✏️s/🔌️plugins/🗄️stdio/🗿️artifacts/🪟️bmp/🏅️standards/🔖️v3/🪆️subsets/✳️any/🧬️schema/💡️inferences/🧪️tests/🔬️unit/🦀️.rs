use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = BmpSnapshot::default();
    assert_eq!(BmpInference::infer(&snapshot).expect("valid materialized inference fixture"), BmpInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(BmpInference::infer(&BmpSnapshot::default()).expect("valid materialized inference fixture"), BmpInference::default());
}
