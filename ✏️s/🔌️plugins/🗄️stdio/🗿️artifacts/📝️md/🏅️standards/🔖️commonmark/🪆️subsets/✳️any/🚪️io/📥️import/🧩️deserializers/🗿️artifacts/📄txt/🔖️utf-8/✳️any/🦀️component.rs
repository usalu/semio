//! 📥️ Deserialize `stdio.md` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse md text into a MdSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<MdSnapshot, store::TextError> {
    let blocks = crate::artifacts::md::engine::parse_markdown_blocks(&from.to_body());
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks })
}

/// 📥 Parse DSL/text bytes via txt then md.
pub fn deserialize_text(text: &str) -> Result<MdSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
