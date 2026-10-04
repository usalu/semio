//! 💾️ Direct patch-snapshot binary codec: the patch's canonical JSON bytes.

use super::*;
use crate::schema::mutations::binary::Entry;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "patch-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let BmpMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(protocol::OpBinary::encode_op(patch))
}
pub fn decode(bytes: &[u8]) -> Result<BmpMutation, protocol::ProtocolError> {
    <editing::SnapshotPatch as protocol::OpBinary>::decode_op(bytes).map(|patch| BmpMutation::PatchSnapshot(PatchSnapshot { patch }))
}
