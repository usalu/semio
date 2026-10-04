use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioDocumentSnapshot::default();
    assert_eq!(SemioDocumentInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioDocumentInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioDocumentInference::infer(&SemioDocumentSnapshot::default()).expect("valid materialized inference fixture"), SemioDocumentInference::default());
}
