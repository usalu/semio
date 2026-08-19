//! 📤️ Serialize `stdio.svg` to stdio.xml.

use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn serialize(from: &SvgSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: from.doc.clone() })
}
