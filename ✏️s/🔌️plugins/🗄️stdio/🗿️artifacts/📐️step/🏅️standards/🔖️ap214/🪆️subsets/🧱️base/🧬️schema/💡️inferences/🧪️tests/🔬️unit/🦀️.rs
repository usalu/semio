use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = StepSnapshot::default();
    assert_eq!(StepInference::infer(&snapshot), StepInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(StepInference::infer(&StepSnapshot::default()), StepInference::default());
}
