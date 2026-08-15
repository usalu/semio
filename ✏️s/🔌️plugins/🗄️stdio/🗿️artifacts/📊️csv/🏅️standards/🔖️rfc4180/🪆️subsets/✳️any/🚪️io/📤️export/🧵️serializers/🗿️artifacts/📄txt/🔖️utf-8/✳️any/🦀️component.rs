//! 📤️ Serialize `stdio.csv` to stdio.txt.

use crate::artifacts::csv::CsvSnapshot;
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode csv into a TxtSnapshot.
pub fn serialize(from: &CsvSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::csv::schema::snapshot::encode_csv(from);
    Ok(TxtSnapshot::from_body(&text))
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &CsvSnapshot) -> Result<String, store::PackError> {
    Ok(store::ArtifactDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
