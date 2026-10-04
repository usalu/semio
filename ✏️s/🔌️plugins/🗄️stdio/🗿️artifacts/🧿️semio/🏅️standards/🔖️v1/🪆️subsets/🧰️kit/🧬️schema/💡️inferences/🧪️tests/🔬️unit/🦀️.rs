use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioKitSnapshot::default();
    assert_eq!(SemioKitInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioKitInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioKitInference::infer(&SemioKitSnapshot::default()).expect("valid materialized inference fixture"), SemioKitInference::default());
}
