//! 📤️ Serialize `stdio.ply` to stdio.txt.

use crate::artifacts::ply::PlySnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📤️ Encode ply into a TxtSnapshot (the real ascii ply wire text, via the engine's canonical
/// encode — no more `write_ply_text(vertices, faces)` mesh-only shortcut, see Ticket
/// 26/08/10/ARTIFACT-SYSTEM-OVERHAUL).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &PlySnapshot) -> Result<TxtSnapshot, store::PackError> {
    let bytes = crate::artifacts::ply::engine::encode_ply(from).map_err(store::PackError::Schema)?;
    let text = String::from_utf8(bytes).map_err(|e| store::PackError::Schema(format!("ply: encoded body not utf8: {e}")))?;
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_text(from: &PlySnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
