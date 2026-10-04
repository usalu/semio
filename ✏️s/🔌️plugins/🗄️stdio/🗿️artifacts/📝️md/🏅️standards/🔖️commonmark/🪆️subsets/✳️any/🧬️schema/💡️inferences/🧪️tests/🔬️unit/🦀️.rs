use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = MdSnapshot::default();
    assert_eq!(MdInference::infer(&snapshot).expect("valid materialized inference fixture"), MdInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(MdInference::infer(&MdSnapshot::default()).expect("valid materialized inference fixture"), MdInference::default());
}
