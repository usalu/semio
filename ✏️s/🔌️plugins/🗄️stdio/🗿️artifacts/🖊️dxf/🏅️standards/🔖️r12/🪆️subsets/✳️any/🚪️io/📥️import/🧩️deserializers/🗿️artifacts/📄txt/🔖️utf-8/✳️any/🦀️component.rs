//! 📥️ Deserialize `stdio.dxf` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse dxf text into a DxfSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<DxfSnapshot, store::TextError> {
    let tags = crate::artifacts::dxf::schema::snapshot::tokenize_dxf(from.text.as_str())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    Ok(DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), tags })
}

/// 📥 Parse DSL/text bytes via txt then dxf.
pub fn deserialize_text(text: &str) -> Result<DxfSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
