//! xml bridge stub for stdio.bcf
use crate::artifacts::bcf::{BcfSnapshot, STDIO_BCF_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;
pub fn register() {}
pub fn deserialize(_from: &XmlSnapshot) -> Result<BcfSnapshot, store::TextError> {
    Ok(BcfSnapshot { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
