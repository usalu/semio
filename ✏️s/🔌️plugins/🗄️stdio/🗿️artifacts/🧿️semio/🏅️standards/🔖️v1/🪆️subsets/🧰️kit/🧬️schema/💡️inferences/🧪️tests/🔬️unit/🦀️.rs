use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioKitSnapshot::default();
    assert_eq!(SemioKitInference::infer(&snapshot), SemioKitInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioKitInference::infer(&SemioKitSnapshot::default()), SemioKitInference::default());
}
