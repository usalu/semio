use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioAudioSnapshot::default();
    assert_eq!(SemioAudioInference::infer(&snapshot).expect("valid materialized inference fixture"), SemioAudioInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioAudioInference::infer(&SemioAudioSnapshot::default()).expect("valid materialized inference fixture"), SemioAudioInference::default());
}
