use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = TsvSnapshot::default();
    assert_eq!(TsvInference::infer(&snapshot), TsvInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(TsvInference::infer(&TsvSnapshot::default()), TsvInference::default());
}
