//! 📥️ Deserialize `stdio.ply` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::ply::PlySnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse ply text into a PlySnapshot (real ascii/binary-format-declaring ply text, via the
/// engine's canonical decode — no more `parse_ply_text` mesh-only `(vertices, faces)` tuple).
pub fn deserialize(from: &TxtSnapshot) -> Result<PlySnapshot, store::TextError> {
    crate::artifacts::ply::engine::decode_ply(from.to_body().as_bytes())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then ply.
pub fn deserialize_text(text: &str) -> Result<PlySnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
