use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Mp4Snapshot::default();
    assert_eq!(Mp4Inference::infer(&snapshot), Mp4Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Mp4Inference::infer(&Mp4Snapshot::default()), Mp4Inference::default());
}
