//! 💾️ Direct compact CSV snapshot-patch binary codec.

use super::*;
use protocol::OpBinary;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "patch-snapshot");

pub fn encode(value: &CsvMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let CsvMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(patch.encode_op())
}

pub fn decode(bytes: &[u8]) -> Result<CsvMutation, protocol::ProtocolError> {
    editing::SnapshotPatch::decode_op(bytes).map(|patch| CsvMutation::PatchSnapshot(PatchSnapshot { patch }))
}
