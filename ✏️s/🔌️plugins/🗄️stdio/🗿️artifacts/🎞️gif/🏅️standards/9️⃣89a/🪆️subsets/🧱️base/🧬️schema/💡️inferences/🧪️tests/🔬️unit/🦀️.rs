use crate::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use crate::standards::v89a::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = GifSnapshot::default();
    assert_eq!(GifInference::infer(&snapshot).expect("valid materialized inference fixture"), GifInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(GifInference::infer(&GifSnapshot::default()).expect("valid materialized inference fixture"), GifInference::default());
}
