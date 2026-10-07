//! 🩹️ Direct binary identity for `patch-snapshot`: the patch's canonical JSON bytes.

pub const TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"), "patch-snapshot");
pub const BINARY_TAG: u8 = TAG;

use crate::standards::v1_7::subsets::base::schema::mutations::patch_snapshot::PatchSnapshot;

/// 📤️ Encodes this direct payload as the patch's canonical JSON bytes.
pub fn encode(payload: &PatchSnapshot) -> Result<Vec<u8>, String> {
    protocol::OpBinary::encode_op(&payload.patch).map_err(|error| error.to_string())
}

/// 📥️ Decodes this direct payload from the patch's canonical JSON bytes.
pub fn decode(bytes: &[u8]) -> Result<PatchSnapshot, String> {
    semio_s_artifact_stdio_contract::editing::snapshot_patch_from_bytes(bytes).map(|patch| PatchSnapshot { patch })
}
