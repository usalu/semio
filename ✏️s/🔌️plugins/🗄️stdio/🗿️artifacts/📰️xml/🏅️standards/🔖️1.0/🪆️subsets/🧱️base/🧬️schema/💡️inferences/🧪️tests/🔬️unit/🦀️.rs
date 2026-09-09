use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = XmlSnapshot::default();
    assert_eq!(XmlInference::infer(&snapshot), XmlInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(XmlInference::infer(&XmlSnapshot::default()), XmlInference::default());
}
