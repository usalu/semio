//! 📥️ Deserialize `stdio.csv` from stdio.txt.

use crate::artifacts::csv::CsvSnapshot;
use crate::artifacts::txt::TxtSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub async fn register() {}

/// 📥 Parse csv text into a CsvSnapshot.
pub async fn deserialize(from: &TxtSnapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(crate::artifacts::csv::schema::snapshot::decode_csv_with(&from.to_body(), true))
}

/// 📥 Parse DSL/text bytes via txt then csv.
pub async fn deserialize_text(text: &str) -> Result<CsvSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
