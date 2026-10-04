use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = JpgSnapshot::default();
    assert_eq!(JpgInference::infer(&snapshot).expect("valid materialized inference fixture"), JpgInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(JpgInference::infer(&JpgSnapshot::default()).expect("valid materialized inference fixture"), JpgInference::default());
}
