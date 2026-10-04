use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioTextSnapshot::default();
    assert_eq!(SemioTextInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioTextInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioTextInference::infer(&SemioTextSnapshot::default()).expect("valid materialized inference fixture"), SemioTextInference::default());
}
