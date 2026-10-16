use crate::LasSnapshot;
use crate::standards::v1_0::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = LasSnapshot::default();
    assert_eq!(LasInference::infer(&snapshot).expect("valid materialized inference fixture"), LasInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(LasInference::infer(&LasSnapshot::default()).expect("valid materialized inference fixture"), LasInference::default());
}
