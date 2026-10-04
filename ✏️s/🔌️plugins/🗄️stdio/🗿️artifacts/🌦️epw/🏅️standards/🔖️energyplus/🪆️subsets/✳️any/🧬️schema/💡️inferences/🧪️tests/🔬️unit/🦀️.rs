use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = EpwSnapshot::default();
    assert_eq!(EpwInference::infer(&snapshot).expect("valid materialized inference fixture"), EpwInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(EpwInference::infer(&EpwSnapshot::default()).expect("valid materialized inference fixture"), EpwInference::default());
}
