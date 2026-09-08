
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = JpgSnapshot::default();
    assert_eq!(JpgInference::infer(&snapshot), JpgInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(JpgInference::infer(&JpgSnapshot::default()), JpgInference::default());
}
