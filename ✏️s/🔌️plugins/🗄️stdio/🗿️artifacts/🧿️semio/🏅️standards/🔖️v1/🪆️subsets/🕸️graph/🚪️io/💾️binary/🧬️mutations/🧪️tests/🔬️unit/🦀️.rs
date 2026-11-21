use crate::schema::mutations::SemioGraphMutation;
use crate::io::binary::mutations::*;
use crate::io::text::mutations::demo_mutation_cases;
use protocol::OpBinary;
use store::os_dsl::original_receiving::{ReceivingStock, RetainedCloneGrant, receive_to_completion};

#[semio_framework_async_macros::async_test]
async fn op_binary_roundtrip_law() {
    for m in demo_mutation_cases() {
        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioGraphMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        let budget = ReceivingStock {
            stock: RetainedCloneGrant {
                maximum_items: 1 << 20,
                maximum_copy_bytes: 1 << 28,
                maximum_capacity_bytes: 1 << 28,
                maximum_release_bytes: 1 << 28,
                maximum_depth: 128,
            },
            quantum: RetainedCloneGrant {
                maximum_items: 64,
                maximum_copy_bytes: 1 << 20,
                maximum_capacity_bytes: 1 << 20,
                maximum_release_bytes: 2 << 20,
                maximum_depth: 128,
            },
            fuel: 1 << 20,
        };
        let received = receive_to_completion::<SemioGraphMutation>(&encoded, (0, encoded.len()), budget)
            .unwrap_or_else(|error| panic!("original receiving failed for {m:?}: {error:?}"));
        assert_eq!(received, m, "original receiving mismatch for {m:?}");
    }
}
