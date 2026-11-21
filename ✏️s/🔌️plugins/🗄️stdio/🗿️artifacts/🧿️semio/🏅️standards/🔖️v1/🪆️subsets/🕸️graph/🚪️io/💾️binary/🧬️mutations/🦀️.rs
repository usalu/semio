//! ⚡️ Semio graph operation encoding and original receiving share the authored record tags.

use crate::schema::mutations::SemioGraphMutation;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

impl protocol::OpBinary for SemioGraphMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Key, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(COMPONENT_PROTOCOL_SEMIO, dsl::tagged_value_binary::VariantTag::Key, bytes)
    }
}

dsl::original_pack_value_operation_receiving!(SemioGraphMutation, tagged_key = COMPONENT_PROTOCOL_SEMIO);
dsl::artifact_persisted_binary_operation!(SemioGraphMutation);
