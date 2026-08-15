//! 📤️ Serialize `stdio.dxf` to stdio.txt.

use crate::artifacts::dxf::DxfSnapshot;
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode dxf into a TxtSnapshot.
pub fn serialize(from: &DxfSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::dxf::schema::snapshot::print_dxf_document(from);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &DxfSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
