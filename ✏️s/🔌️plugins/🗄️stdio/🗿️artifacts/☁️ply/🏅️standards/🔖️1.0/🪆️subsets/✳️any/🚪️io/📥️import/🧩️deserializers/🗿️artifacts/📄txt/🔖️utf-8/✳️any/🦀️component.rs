//! 📥️ Deserialize `stdio.ply` from stdio.txt.

use crate::artifacts::ply::PlySnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub async fn register() {}

/// 📥 Parse ply text into a PlySnapshot (real ascii/binary-format-declaring ply text, via the
/// engine's canonical decode — no more `parse_ply_text` mesh-only `(vertices, faces)` tuple).
pub async fn deserialize(from: &TxtSnapshot) -> Result<PlySnapshot, store::TextError> {
    crate::artifacts::ply::engine::decode_ply(from.to_body().await.as_bytes()).await.map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then ply.
pub async fn deserialize_text(text: &str) -> Result<PlySnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await
}
//#endregion 🔖️Codec
