//! 📤️ Serialize `stdio.md` to stdio.txt.

use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::md::MdSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode md into a TxtSnapshot.
pub fn serialize(from: &MdSnapshot) -> Result<TxtSnapshot, store::PackError> {
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text: from.body.clone() })
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &MdSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
