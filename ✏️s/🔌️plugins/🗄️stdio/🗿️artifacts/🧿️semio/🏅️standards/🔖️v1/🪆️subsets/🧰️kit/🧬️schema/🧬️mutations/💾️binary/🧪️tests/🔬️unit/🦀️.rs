use super::*;
use crate::standards::v1::subsets::kit::schema::mutations::text::demo_mutation_cases;
use protocol::OpBinary;

#[semio_framework_async_macros::async_test]
async fn op_binary_roundtrip_law() {
    for m in demo_mutation_cases() {
        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioKitMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}
