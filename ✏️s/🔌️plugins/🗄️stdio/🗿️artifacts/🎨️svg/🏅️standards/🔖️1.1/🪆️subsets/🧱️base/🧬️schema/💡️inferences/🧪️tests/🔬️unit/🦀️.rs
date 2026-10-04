use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SvgSnapshot::default();
    assert_eq!(SvgInference::infer(&snapshot).expect("valid materialized inference fixture"), SvgInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SvgInference::infer(&SvgSnapshot::default()).expect("valid materialized inference fixture"), SvgInference::default());
}
