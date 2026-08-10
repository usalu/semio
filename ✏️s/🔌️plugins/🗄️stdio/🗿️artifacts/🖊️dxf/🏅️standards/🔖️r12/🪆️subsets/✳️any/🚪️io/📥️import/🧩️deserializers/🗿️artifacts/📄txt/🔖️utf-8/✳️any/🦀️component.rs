//! 📥️ Deserialize `stdio.dxf` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse dxf text into a DxfSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<DxfSnapshot, store::TextError> {
    let lines = crate::artifacts::dxf::schema::snapshot::parse_dxf_text(from.text.as_str())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    Ok(DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
}

/// 📥 Parse DSL/text bytes via txt then dxf.
pub fn deserialize_text(text: &str) -> Result<DxfSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
