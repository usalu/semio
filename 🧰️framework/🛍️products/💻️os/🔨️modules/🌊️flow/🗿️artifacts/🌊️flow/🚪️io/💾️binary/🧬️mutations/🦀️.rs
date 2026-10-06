//! 🚪️ Native artifact representation codecs.
use super::super::super::*;

impl crate::os_spr::OpBinary for FlowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::decode_op(bytes)
    }
}
