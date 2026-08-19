//! xml bridge stub for stdio.bcf
use crate::artifacts::bcf::BcfSnapshot;
use crate::artifacts::xml::{schema::snapshot::XmlDocument, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
pub async fn register() {}
pub async fn serialize(_from: &BcfSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() })
}
