
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DrawingSnapshot::default();
    assert_eq!(DrawingInference::infer(&snapshot), DrawingInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DrawingInference::infer(&DrawingSnapshot::default()), DrawingInference::default());
}
