//! 🚪️ Artifact representation codecs.

use crate::editor::dag::config::DagConfigMutation;

impl protocol::OpBinary for DagConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

semio_os_kernel::original_ordinal_operation_receiving!(DagConfigMutation);
