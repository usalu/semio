use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioBrepSnapshot::default();
    assert_eq!(SemioBrepInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioBrepInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioBrepInference::infer(&SemioBrepSnapshot::default()).expect("valid materialized inference fixture"), SemioBrepInference::default());
}
