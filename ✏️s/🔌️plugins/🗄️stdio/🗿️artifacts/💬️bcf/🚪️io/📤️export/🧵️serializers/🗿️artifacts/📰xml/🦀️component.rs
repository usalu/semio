//! 📤️ Serialize `stdio.xml` to stdio.txt.

use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::bcf::BcfSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode xml into a TxtSnapshot.
pub fn serialize(from: &BcfSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::bcf::schema::snapshot::xml_document_to_text(&from.doc);
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &BcfSnapshot) -> Result<String, store::PackError> {
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
