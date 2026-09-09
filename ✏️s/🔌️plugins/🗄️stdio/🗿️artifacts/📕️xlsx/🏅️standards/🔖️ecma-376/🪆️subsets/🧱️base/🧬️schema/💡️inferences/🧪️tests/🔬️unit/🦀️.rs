use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = XlsxSnapshot::default();
    assert_eq!(XlsxInference::infer(&snapshot), XlsxInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(XlsxInference::infer(&XlsxSnapshot::default()), XlsxInference::default());
}
