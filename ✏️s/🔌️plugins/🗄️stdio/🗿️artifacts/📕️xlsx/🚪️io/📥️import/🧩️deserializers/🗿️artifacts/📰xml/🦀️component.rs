//! 📥️ Deserialize `stdio.xml` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse xml text into a XlsxSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<XlsxSnapshot, store::TextError> {
    let doc = crate::artifacts::xlsx::schema::snapshot::xml_document_from_text(from.text.trim()).map_err(|e| {
        store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))
    })?;
    Ok(XlsxSnapshot { schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(), doc })
}

/// 📥 Parse DSL/text bytes via txt then xml.
pub fn deserialize_text(text: &str) -> Result<XlsxSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
