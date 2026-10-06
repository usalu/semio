//! 💾️ Direct compact PNG snapshot-patch binary codec.

use crate::standards::v1_2::subsets::any::io::binary::mutations::*;
use crate::standards::v1_2::subsets::any::io::binary::mutations::Entry;
use protocol::OpBinary;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "patch-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(patch.encode_op())
}

pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    editing::SnapshotPatch::decode_op(bytes).map(|patch| PngMutation::PatchSnapshot(PatchSnapshot { patch }))
}
