//! 📥️ Deserialize `stdio.csv` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::csv::CsvSnapshot;

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse csv text into a CsvSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(crate::artifacts::csv::schema::snapshot::decode_csv_with(&from.to_body(), true))
}

/// 📥 Parse DSL/text bytes via txt then csv.
pub fn deserialize_text(text: &str) -> Result<CsvSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
