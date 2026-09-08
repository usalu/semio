
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DwgSnapshot::default();
    assert_eq!(DwgInference::infer(&snapshot), DwgInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DwgInference::infer(&DwgSnapshot::default()), DwgInference::default());
}
