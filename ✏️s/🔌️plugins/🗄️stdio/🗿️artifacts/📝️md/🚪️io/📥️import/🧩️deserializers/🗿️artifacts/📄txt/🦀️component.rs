//! deser md via txt
use crate::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
pub fn register() {}
pub fn deserialize(from: &TxtSnapshot) -> Result<MdSnapshot, store::TextError> {
    let value = serde_md::from_str(from.text.trim()).map_err(|e| store::TextError::new(format!("md parse: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), value })
}
pub fn deserialize_text(text: &str) -> Result<MdSnapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}
