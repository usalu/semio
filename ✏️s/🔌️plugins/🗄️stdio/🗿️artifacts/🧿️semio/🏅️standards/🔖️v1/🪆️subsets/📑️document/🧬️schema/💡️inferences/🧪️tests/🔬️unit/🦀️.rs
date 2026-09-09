use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioDocumentSnapshot::default();
    assert_eq!(SemioDocumentInference::infer(&snapshot), SemioDocumentInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioDocumentInference::infer(&SemioDocumentSnapshot::default()), SemioDocumentInference::default());
}
