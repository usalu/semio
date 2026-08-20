//! 📤️ Serialize `stdio.dxf` to stdio.txt.

use crate::artifacts::dxf::DxfSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub async fn register() {}

/// 📤️ Encode dxf into a TxtSnapshot.
pub async fn serialize(from: &DxfSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::dxf::schema::snapshot::print_dxf_document(from);
    Ok(TxtSnapshot::from_body(&text).await)
}

/// 📤️ Encode as txt DSL.
pub async fn serialize_text(from: &DxfSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from).await?).await)
}
//#endregion 🔖️Codec
