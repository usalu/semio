use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioSnapshot::default();
    assert_eq!(SemioInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioInference::infer(&SemioSnapshot::default()).expect("valid materialized inference fixture"), SemioInference::default());
}
