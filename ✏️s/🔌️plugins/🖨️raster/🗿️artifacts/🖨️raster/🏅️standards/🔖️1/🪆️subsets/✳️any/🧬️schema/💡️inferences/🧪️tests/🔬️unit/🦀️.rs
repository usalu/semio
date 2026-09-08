
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = RasterSnapshot::default();
    assert_eq!(RasterInference::infer(&snapshot), RasterInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(RasterInference::infer(&RasterSnapshot::default()), RasterInference::default());
}
