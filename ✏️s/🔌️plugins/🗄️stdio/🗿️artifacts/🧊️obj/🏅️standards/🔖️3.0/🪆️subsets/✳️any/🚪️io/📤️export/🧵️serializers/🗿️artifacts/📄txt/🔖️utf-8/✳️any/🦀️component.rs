//! 📤️ Serialize `stdio.obj` to stdio.txt.

use crate::artifacts::obj::ObjSnapshot;
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode obj into a TxtSnapshot.
pub fn serialize(from: &ObjSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::obj::engine::encode_obj(from);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &ObjSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
