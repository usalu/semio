//! 📤️ Serialize `stdio.xml` to stdio.txt.

use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode xml into a TxtSnapshot.
pub fn serialize(from: &XmlSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::xml::schema::snapshot::xml_document_to_text(&from.doc);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &XmlSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
