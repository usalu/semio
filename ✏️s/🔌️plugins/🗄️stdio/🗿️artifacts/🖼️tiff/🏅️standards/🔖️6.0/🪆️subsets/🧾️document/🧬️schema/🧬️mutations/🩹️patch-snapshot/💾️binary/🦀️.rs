//! 💾️ Direct compact TIFF snapshot-patch binary codec.

use super::*;
use crate::schema::mutations::binary::Entry;
use protocol::OpBinary;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "patch-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let TiffMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(patch.encode_op())
}
pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    editing::SnapshotPatch::decode_op(bytes).map(|patch| TiffMutation::PatchSnapshot(PatchSnapshot { patch }))
}
