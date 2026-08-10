//! 📥️ Deserialize `stdio.csv` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {}

/// 📥 Parse csv text into a CsvSnapshot.
pub fn deserialize(from: &TxtSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let (headers, rows) = crate::artifacts::csv::schema::snapshot::csv_table_from_text(from.text.as_str());
    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows })
}

/// 📥 Parse DSL/text bytes via txt then csv.
pub fn deserialize_text(text: &str) -> Result<CsvSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
//#endregion 🔖️Codec
