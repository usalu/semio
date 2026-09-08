//! 📥️ Deserialize `stdio.md` from stdio.txt.

use crate::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 📥 Parse md text into a MdSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<MdSnapshot, store::TextError> {
    let blocks = crate::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(&from.to_body());
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks })
}

/// 📥 Parse DSL/text bytes via txt then md.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<MdSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
