
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = PptxSnapshot::default();
    assert_eq!(PptxInference::infer(&snapshot), PptxInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(PptxInference::infer(&PptxSnapshot::default()), PptxInference::default());
}
