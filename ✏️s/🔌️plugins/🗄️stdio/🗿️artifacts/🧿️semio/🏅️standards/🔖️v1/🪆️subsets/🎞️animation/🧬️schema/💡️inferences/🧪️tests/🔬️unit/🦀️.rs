use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioAnimationSnapshot::default();
    assert_eq!(SemioAnimationInference::infer(&snapshot), SemioAnimationInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioAnimationInference::infer(&SemioAnimationSnapshot::default()), SemioAnimationInference::default());
}
