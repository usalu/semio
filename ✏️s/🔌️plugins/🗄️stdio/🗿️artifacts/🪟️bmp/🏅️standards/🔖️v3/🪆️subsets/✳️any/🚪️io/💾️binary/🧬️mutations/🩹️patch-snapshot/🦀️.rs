//! 💾️ Direct patch-snapshot binary codec: the patch's canonical JSON bytes.
use crate::standards::v_v3::subsets::any::schema::mutations::*;
use crate::standards::v_v3::subsets::any::schema::snapshot::*;
use crate::standards::v_v3::subsets::any::io::binary::diff::*;

use crate::schema::mutations::{BmpMutation, PatchSnapshot};
use semio_s_artifact_stdio_contract::editing;
use crate::standards::v_v3::subsets::any::io::binary::mutations::Entry;

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "patch-snapshot");
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };
pub fn encode(value: &BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let BmpMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(protocol::OpBinary::encode_op(patch))
}
pub fn decode(bytes: &[u8]) -> Result<BmpMutation, protocol::ProtocolError> {
    <editing::SnapshotPatch as protocol::OpBinary>::decode_op(bytes).map(|patch| BmpMutation::PatchSnapshot(PatchSnapshot { patch }))
}
