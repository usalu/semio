//! 🚪️ Artifact representation codecs: the tagged binary command frame of the Home transient lane.

/// 📡️ The authored binary protocol for this facet.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::editor::home::transient::HomeTransientMutation;

dsl::original_tagged_operation_receiving!(HomeTransientMutation, COMPONENT_PROTOCOL_SEMIO);

impl protocol::OpBinary for HomeTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(COMPONENT_PROTOCOL_SEMIO, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
