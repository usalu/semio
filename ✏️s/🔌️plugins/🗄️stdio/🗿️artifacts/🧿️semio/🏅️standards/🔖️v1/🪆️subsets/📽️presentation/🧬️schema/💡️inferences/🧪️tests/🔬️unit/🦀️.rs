use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioPresentationSnapshot::default();
    assert_eq!(SemioPresentationInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioPresentationInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioPresentationInference::infer(&SemioPresentationSnapshot::default()).expect("valid materialized inference fixture"), SemioPresentationInference::default());
}
