//! 📥️ Deserialize `stdio.xml` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::xml::XmlSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse xml text into a XmlSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<XmlSnapshot, store::TextError> {
    XmlSnapshot::import_utf8(from.to_body().as_bytes()).map_err(|e| {
        store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))
    })
}

/// 📥 Parse DSL/text bytes via txt then xml.
pub fn deserialize_text(text: &str) -> Result<XmlSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
