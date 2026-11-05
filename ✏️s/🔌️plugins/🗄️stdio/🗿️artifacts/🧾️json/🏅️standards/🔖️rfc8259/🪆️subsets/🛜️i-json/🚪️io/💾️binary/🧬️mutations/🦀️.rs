//! 💾️ Schema-authored wire tags for the distinct eight-verb I-JSON mutation vocabulary.
use crate::standards::v_rfc8259::subsets::i_json::schema::mutations::JsonIJsonMutation;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");

impl protocol::OpBinary for JsonIJsonMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}
