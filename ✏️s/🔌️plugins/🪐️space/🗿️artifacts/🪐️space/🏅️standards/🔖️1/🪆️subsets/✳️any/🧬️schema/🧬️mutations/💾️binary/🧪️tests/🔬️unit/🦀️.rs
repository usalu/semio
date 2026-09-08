
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::touch_artifact;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = touch_artifact("artifact-1".into(), 7, "user:1".into());
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
