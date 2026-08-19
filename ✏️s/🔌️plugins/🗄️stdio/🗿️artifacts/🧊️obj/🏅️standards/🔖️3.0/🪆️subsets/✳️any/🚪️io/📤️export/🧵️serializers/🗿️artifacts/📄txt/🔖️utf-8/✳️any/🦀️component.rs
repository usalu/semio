//! 📤️ Serialize `stdio.obj` to stdio.txt.

use crate::artifacts::obj::ObjSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub async fn register() {}

/// 📤️ Encode obj into a TxtSnapshot.
pub async fn serialize(from: &ObjSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::obj::engine::encode_obj(from);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub async fn serialize_text(from: &ObjSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
