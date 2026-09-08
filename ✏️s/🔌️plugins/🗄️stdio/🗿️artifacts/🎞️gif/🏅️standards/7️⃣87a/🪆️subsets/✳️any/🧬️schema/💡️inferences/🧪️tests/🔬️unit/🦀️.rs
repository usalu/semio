
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = GifSnapshot::default();
    assert_eq!(GifInference::infer(&snapshot), GifInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(GifInference::infer(&GifSnapshot::default()), GifInference::default());
}
