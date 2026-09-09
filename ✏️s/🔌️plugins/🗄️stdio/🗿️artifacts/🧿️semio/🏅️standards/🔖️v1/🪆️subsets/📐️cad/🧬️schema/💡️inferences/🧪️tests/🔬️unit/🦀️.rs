use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioCadSnapshot::default();
    assert_eq!(SemioCadInference::infer(&snapshot), SemioCadInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioCadInference::infer(&SemioCadSnapshot::default()), SemioCadInference::default());
}
