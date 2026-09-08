
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = LasSnapshot::default();
    assert_eq!(LasInference::infer(&snapshot), LasInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(LasInference::infer(&LasSnapshot::default()), LasInference::default());
}
