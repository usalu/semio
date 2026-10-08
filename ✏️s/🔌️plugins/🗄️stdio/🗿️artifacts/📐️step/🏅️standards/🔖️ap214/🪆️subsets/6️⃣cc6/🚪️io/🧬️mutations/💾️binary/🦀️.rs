//! 📡️ Native tagged value records for the owned CC6 mutation vocabulary.

use crate::standards::v_ap214::subsets::cc6::schema::mutations::StepCc6Mutation;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

impl protocol::OpBinary for StepCc6Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        protocol::tagged_value_binary::encode_op(COMPONENT_PROTOCOL_SEMIO, protocol::tagged_value_binary::VariantTag::Key, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        protocol::tagged_value_binary::decode_op(COMPONENT_PROTOCOL_SEMIO, protocol::tagged_value_binary::VariantTag::Key, bytes)
    }
}
