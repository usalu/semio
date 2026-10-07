//! 💾️ Direct compact JSON snapshot-patch binary codec.

use crate::standards::v_rfc8259::subsets::base::schema::mutations::{JsonMutation, patch_snapshot::PatchSnapshot};
use semio_s_artifact_stdio_contract::editing;
use protocol::OpBinary;

pub const BINARY_TAG: u8 = 6;

pub fn encode(value: &JsonMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let JsonMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(patch.encode_op())
}

pub fn decode(bytes: &[u8]) -> Result<JsonMutation, protocol::ProtocolError> {
    editing::SnapshotPatch::decode_op(bytes).map(|patch| JsonMutation::PatchSnapshot(PatchSnapshot { patch }))
}
