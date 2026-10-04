use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = WavSnapshot::default();
    assert_eq!(WavInference::infer(&snapshot).expect("valid materialized inference fixture"), WavInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(WavInference::infer(&WavSnapshot::default()).expect("valid materialized inference fixture"), WavInference::default());
}
