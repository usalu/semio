//! 📤️ Serialize `stdio.ply` to stdio.txt.

use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::ply::PlySnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode ply into a TxtSnapshot.
pub fn serialize(from: &PlySnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::ply::schema::snapshot::write_ply_text(&from.vertices, &from.faces);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &PlySnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
