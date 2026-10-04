use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioGraphSnapshot::default();
    assert_eq!(SemioGraphInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioGraphInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioGraphInference::infer(&SemioGraphSnapshot::default()).expect("valid materialized inference fixture"), SemioGraphInference::default());
}
