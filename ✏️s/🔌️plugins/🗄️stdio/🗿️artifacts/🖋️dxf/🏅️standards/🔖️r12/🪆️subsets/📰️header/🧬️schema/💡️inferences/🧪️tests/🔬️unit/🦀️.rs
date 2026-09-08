
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DxfSnapshot::default();
    assert_eq!(DxfInference::infer(&snapshot), DxfInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DxfInference::infer(&DxfSnapshot::default()), DxfInference::default());
}
