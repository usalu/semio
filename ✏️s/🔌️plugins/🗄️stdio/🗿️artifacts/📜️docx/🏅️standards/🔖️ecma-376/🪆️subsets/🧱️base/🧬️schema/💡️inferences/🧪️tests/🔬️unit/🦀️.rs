
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DocxSnapshot::default();
    assert_eq!(DocxInference::infer(&snapshot), DocxInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DocxInference::infer(&DocxSnapshot::default()), DocxInference::default());
}
