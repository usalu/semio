//! 📥️ Deserialize `stdio.ply` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse ply text into a PlySnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<PlySnapshot, store::TextError> {
    let (vertices, faces) = crate::artifacts::ply::schema::snapshot::parse_ply_text(from.text.as_str())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    Ok(PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), vertices, faces })
}

/// 📥 Parse DSL/text bytes via txt then ply.
pub fn deserialize_text(text: &str) -> Result<PlySnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
