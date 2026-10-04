use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioValueSnapshot::default();
    assert_eq!(SemioValueInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioValueInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioValueInference::infer(&SemioValueSnapshot::default()).expect("valid materialized inference fixture"), SemioValueInference::default());
}
