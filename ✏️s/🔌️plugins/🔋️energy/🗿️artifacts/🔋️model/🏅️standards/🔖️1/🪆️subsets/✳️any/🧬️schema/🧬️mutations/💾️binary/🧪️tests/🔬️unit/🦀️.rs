
use super::*;

#[semio_framework_async_macros::async_test]
async fn every_kind_round_trips_through_this_codec() {
    for operation in crate::mutations::wire_probes() {
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
