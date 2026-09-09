use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = TxtSnapshot::default();
    assert_eq!(TxtInference::infer(&snapshot), TxtInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(TxtInference::infer(&TxtSnapshot::default()), TxtInference::default());
}
