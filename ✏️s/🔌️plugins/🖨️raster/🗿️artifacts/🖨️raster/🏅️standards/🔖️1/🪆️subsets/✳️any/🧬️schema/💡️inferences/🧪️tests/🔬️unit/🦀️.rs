use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = RasterSnapshot::default();
    assert_eq!(RasterInference::infer(&snapshot).expect("valid materialized inference fixture"), RasterInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(RasterInference::infer(&RasterSnapshot::default()).expect("valid materialized inference fixture"), RasterInference::default());
}
