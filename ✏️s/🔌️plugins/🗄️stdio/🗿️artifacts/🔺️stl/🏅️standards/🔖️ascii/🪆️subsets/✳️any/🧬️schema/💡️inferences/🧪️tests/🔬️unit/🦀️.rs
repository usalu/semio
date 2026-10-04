use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = StlSnapshot::default();
    assert_eq!(StlInference::infer(&snapshot).expect("valid materialized inference fixture"), StlInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(StlInference::infer(&StlSnapshot::default()).expect("valid materialized inference fixture"), StlInference::default());
}
