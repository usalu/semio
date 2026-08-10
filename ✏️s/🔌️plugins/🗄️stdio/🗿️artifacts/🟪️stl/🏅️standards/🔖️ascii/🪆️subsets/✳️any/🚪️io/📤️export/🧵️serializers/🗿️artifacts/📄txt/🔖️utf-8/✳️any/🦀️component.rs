//! 📤️ Serialize `stdio.stl` to stdio.txt.

use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::stl::StlSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode stl into a TxtSnapshot.
pub fn serialize(from: &StlSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::stl::schema::snapshot::write_stl_text(&from.vertices, &from.faces);
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &StlSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
