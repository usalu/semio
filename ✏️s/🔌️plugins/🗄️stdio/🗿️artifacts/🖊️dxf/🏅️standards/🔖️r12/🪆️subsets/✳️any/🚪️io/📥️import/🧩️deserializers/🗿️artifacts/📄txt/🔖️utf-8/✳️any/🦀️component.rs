//! 📥️ Deserialize `stdio.dxf` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::dxf::DxfSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse dxf text into a DxfSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<DxfSnapshot, store::TextError> {
    crate::artifacts::dxf::schema::snapshot::parse_dxf_document(&from.to_body())
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then dxf.
pub fn deserialize_text(text: &str) -> Result<DxfSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
