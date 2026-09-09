use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Mp3Snapshot::default();
    assert_eq!(Mp3Inference::infer(&snapshot), Mp3Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Mp3Inference::infer(&Mp3Snapshot::default()), Mp3Inference::default());
}
