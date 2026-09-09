use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = IfcSnapshot::default();
    assert_eq!(IfcInference::infer(&snapshot), IfcInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(IfcInference::infer(&IfcSnapshot::default()), IfcInference::default());
}
