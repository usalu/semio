use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::standards::isobmff::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Mp4Snapshot::default();
    assert_eq!(Mp4Inference::infer(&snapshot).expect("valid materialized inference fixture"), Mp4Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Mp4Inference::infer(&Mp4Snapshot::default()).expect("valid materialized inference fixture"), Mp4Inference::default());
}
