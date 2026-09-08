
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioPresentationSnapshot::default();
    assert_eq!(SemioPresentationInference::infer(&snapshot), SemioPresentationInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioPresentationInference::infer(&SemioPresentationSnapshot::default()), SemioPresentationInference::default());
}
