//! 🚪️ Native artifact representation codecs.
use super::super::super::*;

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for SpaceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

semio_os_kernel::original_ordinal_operation_receiving!(SpaceMutation);
semio_os_kernel::artifact_persisted_binary_operation!(SpaceMutation);
