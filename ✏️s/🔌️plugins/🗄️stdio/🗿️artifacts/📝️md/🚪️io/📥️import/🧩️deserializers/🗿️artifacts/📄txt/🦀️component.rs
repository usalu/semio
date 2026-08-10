//! 📥️ Deserialize `stdio.md` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse md text into a MdSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body: from.text.clone() })
}

/// 📥 Parse DSL/text bytes via txt then md.
pub fn deserialize_text(text: &str) -> Result<MdSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
