//! 📥️ Deserialize `stdio.md` from stdio.txt.

use crate::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub async fn register() {}

/// 📥 Parse md text into a MdSnapshot.
pub async fn deserialize(from: &TxtSnapshot) -> Result<MdSnapshot, store::TextError> {
    let blocks = crate::artifacts::md::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(&from.to_body());
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks })
}

/// 📥 Parse DSL/text bytes via txt then md.
pub async fn deserialize_text(text: &str) -> Result<MdSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
