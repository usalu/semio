use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = TiffSnapshot::default();
    assert_eq!(TiffInference::infer(&snapshot).expect("valid materialized inference fixture"), TiffInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(TiffInference::infer(&TiffSnapshot::default()).expect("valid materialized inference fixture"), TiffInference::default());
}
