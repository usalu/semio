//! 📥️ Deserialize `stdio.stl` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse stl text into a StlSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<StlSnapshot, store::TextError> {
    let (vertices, faces) = crate::artifacts::stl::schema::snapshot::parse_stl_text(from.text.as_str())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces })
}

/// 📥 Parse DSL/text bytes via txt then stl.
pub fn deserialize_text(text: &str) -> Result<StlSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
