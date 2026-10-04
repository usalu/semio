//! 🩹️ Direct text identity for `patch-snapshot`: `patch-snapshot patch=<hex of the patch JSON>`.

pub const OPCODE: &str = "patch-snapshot";
pub const TEXT_OPCODE: &str = OPCODE;

use super::PatchSnapshot;

/// 🖨️ Prints this direct payload as its shared `patch-snapshot` op line.
pub fn print(payload: &PatchSnapshot) -> Result<String, String> {
    Ok(semio_s_artifact_stdio_contract::editing::snapshot_patch_text(&payload.patch))
}

/// 📥️ Parses this direct payload from its shared `patch-snapshot` op line.
pub fn parse(text: &str) -> Result<PatchSnapshot, String> {
    semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(text).map(|patch| PatchSnapshot { patch })
}
