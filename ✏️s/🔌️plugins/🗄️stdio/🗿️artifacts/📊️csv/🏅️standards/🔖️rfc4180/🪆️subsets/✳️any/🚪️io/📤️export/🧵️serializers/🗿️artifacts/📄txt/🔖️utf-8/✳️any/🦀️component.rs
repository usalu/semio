//! 📤️ Serialize `stdio.csv` to stdio.txt.

use crate::artifacts::csv::CsvSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub async fn register() {}

/// 📤️ Encode csv into a TxtSnapshot.
pub async fn serialize(from: &CsvSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::csv::schema::snapshot::encode_csv(from);
    Ok(TxtSnapshot::from_body(&text).await)
}

/// 📤️ Encode as txt DSL.
pub async fn serialize_text(from: &CsvSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from).await?).await)
}
//#endregion 🔖️Codec
