
use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = BcfSnapshot::default();
    assert_eq!(BcfInference::infer(&snapshot), BcfInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(BcfInference::infer(&BcfSnapshot::default()), BcfInference::default());
}
