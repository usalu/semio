//! xml bridge stub for stdio.xlsx
use crate::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;
pub async fn register() {}
pub async fn deserialize(_from: &XmlSnapshot) -> Result<XlsxSnapshot, store::TextError> {
    Ok(XlsxSnapshot { schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
