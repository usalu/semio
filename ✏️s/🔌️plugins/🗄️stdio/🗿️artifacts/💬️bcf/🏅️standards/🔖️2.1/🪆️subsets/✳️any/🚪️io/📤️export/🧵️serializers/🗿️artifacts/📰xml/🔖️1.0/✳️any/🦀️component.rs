//! xml bridge stub for stdio.bcf
use crate::artifacts::bcf::BcfSnapshot;
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA, schema::snapshot::XmlDocument};
pub fn register() {}
pub fn serialize(_from: &BcfSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
