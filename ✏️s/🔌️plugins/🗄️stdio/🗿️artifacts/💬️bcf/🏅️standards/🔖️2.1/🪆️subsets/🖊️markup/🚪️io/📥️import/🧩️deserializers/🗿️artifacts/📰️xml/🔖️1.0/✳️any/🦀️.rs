//! xml bridge stub for stdio.bcf
use crate::artifacts::bcf::{BcfSnapshot, STDIO_BCF_DOCUMENT_SCHEMA};
use crate::artifacts::xml::XmlSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(_from: &XmlSnapshot) -> Result<BcfSnapshot, store::TextError> {
    Ok(BcfSnapshot { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), ..Default::default() })
}
