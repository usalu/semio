use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioSnapshot::default();
    assert_eq!(SemioInference::infer(&snapshot), SemioInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioInference::infer(&SemioSnapshot::default()), SemioInference::default());
}
