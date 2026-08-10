//! 📤️ Serialize `stdio.csv` to stdio.txt.

use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use crate::artifacts::csv::CsvSnapshot;

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {}

/// 📤️ Encode csv into a TxtSnapshot.
pub fn serialize(from: &CsvSnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = crate::artifacts::csv::schema::snapshot::csv_table_to_text(&from.headers, &from.rows);
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &CsvSnapshot) -> Result<String, store::PackError> {
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}
//#endregion 🔖️Codec
