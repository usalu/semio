
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = TiffSnapshot::default();
    assert_eq!(TiffInference::infer(&snapshot), TiffInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(TiffInference::infer(&TiffSnapshot::default()), TiffInference::default());
}
