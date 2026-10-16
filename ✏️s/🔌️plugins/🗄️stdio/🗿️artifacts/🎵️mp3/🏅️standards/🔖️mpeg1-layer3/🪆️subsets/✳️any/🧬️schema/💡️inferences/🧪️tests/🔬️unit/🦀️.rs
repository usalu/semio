use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use crate::standards::mpeg1_layer3::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Mp3Snapshot::default();
    assert_eq!(Mp3Inference::infer(&snapshot).expect("valid materialized inference fixture"), Mp3Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Mp3Inference::infer(&Mp3Snapshot::default()).expect("valid materialized inference fixture"), Mp3Inference::default());
}
