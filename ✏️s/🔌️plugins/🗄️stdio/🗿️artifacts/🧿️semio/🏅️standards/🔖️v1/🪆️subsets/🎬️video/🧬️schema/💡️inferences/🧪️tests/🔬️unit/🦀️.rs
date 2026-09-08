
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioVideoSnapshot::default();
    assert_eq!(SemioVideoInference::infer(&snapshot), SemioVideoInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioVideoInference::infer(&SemioVideoSnapshot::default()), SemioVideoInference::default());
}
