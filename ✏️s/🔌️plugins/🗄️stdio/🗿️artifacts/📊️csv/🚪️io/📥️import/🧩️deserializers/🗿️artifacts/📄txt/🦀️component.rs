//! deser csv via txt
use crate::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
pub fn register() {}
pub fn deserialize(from: &TxtSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let value = serde_csv::from_str(from.text.trim()).map_err(|e| store::TextError::new(format!("csv parse: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), value })
}
pub fn deserialize_text(text: &str) -> Result<CsvSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
