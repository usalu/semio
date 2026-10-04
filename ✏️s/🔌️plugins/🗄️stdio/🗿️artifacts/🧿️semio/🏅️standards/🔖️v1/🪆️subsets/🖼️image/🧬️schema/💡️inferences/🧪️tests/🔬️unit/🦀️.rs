use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioImageSnapshot::default();
    assert_eq!(SemioImageInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioImageInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioImageInference::infer(&SemioImageSnapshot::default()).expect("valid materialized inference fixture"), SemioImageInference::default());
}
