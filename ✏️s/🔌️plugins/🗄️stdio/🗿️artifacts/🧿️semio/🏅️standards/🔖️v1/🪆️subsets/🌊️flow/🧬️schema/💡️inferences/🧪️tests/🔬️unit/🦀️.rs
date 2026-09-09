use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioFlowSnapshot::default();
    assert_eq!(SemioFlowInference::infer(&snapshot), SemioFlowInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioFlowInference::infer(&SemioFlowSnapshot::default()), SemioFlowInference::default());
}
