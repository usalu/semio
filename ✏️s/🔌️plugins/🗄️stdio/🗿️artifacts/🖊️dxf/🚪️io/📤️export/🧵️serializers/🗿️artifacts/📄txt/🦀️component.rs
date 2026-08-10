//! 📤️ Serialize `stdio.dxf` to stdio.txt.

use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::dxf::DxfSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode dxf into a TxtSnapshot.
pub fn serialize(from: &DxfSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::dxf::schema::snapshot::write_dxf_text(&from.lines);
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &DxfSnapshot) -> Result<String, store::PackError> {
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
