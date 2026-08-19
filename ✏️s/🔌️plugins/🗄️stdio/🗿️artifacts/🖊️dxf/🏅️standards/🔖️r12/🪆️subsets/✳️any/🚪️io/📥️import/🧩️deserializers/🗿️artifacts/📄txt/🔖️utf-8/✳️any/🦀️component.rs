//! 📥️ Deserialize `stdio.dxf` from stdio.txt.

use crate::artifacts::dxf::DxfSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub async fn register() {}

/// 📥 Parse dxf text into a DxfSnapshot.
pub async fn deserialize(from: &TxtSnapshot) -> Result<DxfSnapshot, store::TextError> {
    crate::artifacts::dxf::schema::snapshot::parse_dxf_document(&from.to_body()).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}

/// 📥 Parse DSL/text bytes via txt then dxf.
pub async fn deserialize_text(text: &str) -> Result<DxfSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
