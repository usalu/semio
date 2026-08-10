//! 📥️ Deserialize `stdio.xml` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::pptx::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse xml text into a PptxSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<PptxSnapshot, store::TextError> {
    let doc = crate::artifacts::pptx::schema::snapshot::xml_document_from_text(from.text.trim()).map_err(|e| {
        store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))
    })?;
    Ok(PptxSnapshot { schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(), doc })
}

/// 📥 Parse DSL/text bytes via txt then xml.
pub fn deserialize_text(text: &str) -> Result<PptxSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
