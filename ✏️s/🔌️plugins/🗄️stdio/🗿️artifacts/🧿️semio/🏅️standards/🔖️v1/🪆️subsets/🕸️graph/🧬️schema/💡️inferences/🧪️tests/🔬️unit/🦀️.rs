use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioGraphSnapshot::default();
    assert_eq!(SemioGraphInference::infer(&snapshot), SemioGraphInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioGraphInference::infer(&SemioGraphSnapshot::default()), SemioGraphInference::default());
}
