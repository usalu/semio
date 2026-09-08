
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioAudioSnapshot::default();
    assert_eq!(SemioAudioInference::infer(&snapshot), SemioAudioInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioAudioInference::infer(&SemioAudioSnapshot::default()), SemioAudioInference::default());
}
