//! 📡️ Draw artifact — wire codec (encode_op/decode_op), renamed from the old `protocol` half
//! (constitutional: spr — state patch representation).

use crate::artifacts::draw::op::DrawOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `DrawOperation` to its binary command form.
pub fn encode_op(operation: &DrawOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DrawOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DrawOperation, protocol::ProtocolError> {
    DrawOperation::decode_op(bytes)
}
