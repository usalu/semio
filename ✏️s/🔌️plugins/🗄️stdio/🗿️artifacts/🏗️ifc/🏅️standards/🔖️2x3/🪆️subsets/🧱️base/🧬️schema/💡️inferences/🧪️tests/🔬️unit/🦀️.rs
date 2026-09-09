use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Ifc2x3Snapshot::default();
    assert_eq!(Ifc2x3Inference::infer(&snapshot), Ifc2x3Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Ifc2x3Inference::infer(&Ifc2x3Snapshot::default()), Ifc2x3Inference::default());
}
