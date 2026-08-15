//! 📤️ Serialize `stdio.xml` to stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::xml::XmlSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode xml into a TxtSnapshot.
pub fn serialize(from: &XmlSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = String::from_utf8(from.export_utf8().map_err(store::PackError::Schema)?).map_err(|error| store::PackError::Schema(error.to_string()))?;
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &XmlSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
