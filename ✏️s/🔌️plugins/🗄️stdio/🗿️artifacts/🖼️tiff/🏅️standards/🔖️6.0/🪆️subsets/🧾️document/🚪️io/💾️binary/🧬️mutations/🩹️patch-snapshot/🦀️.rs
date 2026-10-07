//! 💾️ Direct compact TIFF snapshot-patch binary codec.
use crate::standards::v6_0::subsets::document::schema::mutations::*;
use crate::standards::v6_0::subsets::document::schema::mutations::patch_snapshot::PatchSnapshot;
use crate::standards::v6_0::subsets::document::schema::snapshot::*;
use crate::standards::v6_0::subsets::document::io::binary::diff::*;

use crate::standards::v6_0::subsets::document::io::binary::mutations::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::Entry;
use protocol::OpBinary;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "patch-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let TiffMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(patch.encode_op())
}
pub fn decode(bytes: &[u8]) -> Result<TiffMutation, protocol::ProtocolError> {
    semio_s_artifact_stdio_contract::editing::SnapshotPatch::decode_op(bytes).map(|patch| TiffMutation::PatchSnapshot(PatchSnapshot { patch }))
}
