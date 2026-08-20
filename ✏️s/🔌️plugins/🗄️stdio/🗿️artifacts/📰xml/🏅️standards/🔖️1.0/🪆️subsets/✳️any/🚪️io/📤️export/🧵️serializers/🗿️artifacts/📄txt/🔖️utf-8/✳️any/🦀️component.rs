//! 📤️ Serialize `stdio.xml` to stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::xml::XmlSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub async fn register() {}

/// 📤️ Encode xml into a TxtSnapshot.
pub async fn serialize(from: &XmlSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = String::from_utf8(from.export_utf8().await.map_err(store::PackError::Schema)?).map_err(|error| store::PackError::Schema(error.to_string()))?;
    Ok(TxtSnapshot::from_body(&text).await)
}

/// 📤️ Encode as txt DSL.
pub async fn serialize_text(from: &XmlSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from).await?).await)
}
//#endregion 🔖️Codec
