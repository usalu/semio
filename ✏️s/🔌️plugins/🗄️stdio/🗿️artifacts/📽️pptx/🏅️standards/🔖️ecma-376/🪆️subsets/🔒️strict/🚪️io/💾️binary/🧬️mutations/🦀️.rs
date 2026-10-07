//! 💾️ Tagged native value encoding for conformance mutations.
use crate::standards::v_ecma_376::subsets::strict::schema::mutations::PptxStrictMutation;
const PROTOCOL: &str = include_str!("📡️.protocol.semio");
impl protocol::OpBinary for PptxStrictMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::tagged_value_binary::encode_op(PROTOCOL, dsl::tagged_value_binary::VariantTag::Key, self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::tagged_value_binary::decode_op(PROTOCOL, dsl::tagged_value_binary::VariantTag::Key, bytes) }
}
