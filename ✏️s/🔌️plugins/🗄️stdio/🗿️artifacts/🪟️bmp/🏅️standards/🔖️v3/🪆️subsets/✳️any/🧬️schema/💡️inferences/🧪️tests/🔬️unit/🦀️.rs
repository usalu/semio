use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = BmpSnapshot::default();
    assert_eq!(BmpInference::infer(&snapshot), BmpInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(BmpInference::infer(&BmpSnapshot::default()), BmpInference::default());
}
