//! 💾️ Direct compact JPEG snapshot-patch binary codec.

use crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::Entry;
use protocol::OpBinary;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "patch-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JpgMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(patch.encode_op())
}
pub fn decode(bytes: &[u8]) -> Result<JpgMutation, protocol::ProtocolError> {
    editing::SnapshotPatch::decode_op(bytes).map(|patch| JpgMutation::PatchSnapshot(PatchSnapshot { patch }))
}
