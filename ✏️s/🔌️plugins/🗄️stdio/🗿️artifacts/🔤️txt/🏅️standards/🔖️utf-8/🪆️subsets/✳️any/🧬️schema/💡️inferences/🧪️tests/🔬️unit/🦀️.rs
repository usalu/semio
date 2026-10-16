use crate::TxtSnapshot;
use crate::standards::v_utf_8::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = TxtSnapshot::default();
    assert_eq!(TxtInference::infer(&snapshot).expect("valid materialized inference fixture"), TxtInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(TxtInference::infer(&TxtSnapshot::default()).expect("valid materialized inference fixture"), TxtInference::default());
}
