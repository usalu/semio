//! 📥️ Deserialize `stdio.xml` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse xml text into a XmlSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<XmlSnapshot, store::TextError> {
    let doc = crate::artifacts::xml::schema::snapshot::xml_document_from_text(from.text.trim()).map_err(|e| {
        store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))
    })?;
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
}

/// 📥 Parse DSL/text bytes via txt then xml.
pub fn deserialize_text(text: &str) -> Result<XmlSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
