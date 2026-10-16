use crate::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use crate::standards::v2_1::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = BcfSnapshot::default();
    assert_eq!(BcfInference::infer(&snapshot).expect("valid materialized inference fixture"), BcfInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(BcfInference::infer(&BcfSnapshot::default()).expect("valid materialized inference fixture"), BcfInference::default());
}
