//! 📤️ Serialize `stdio.stl` to stdio.txt.

use crate::artifacts::stl::StlSnapshot;
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode stl into a TxtSnapshot.
pub fn serialize(from: &StlSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::stl::engine::encode_stl_ascii(from);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &StlSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
