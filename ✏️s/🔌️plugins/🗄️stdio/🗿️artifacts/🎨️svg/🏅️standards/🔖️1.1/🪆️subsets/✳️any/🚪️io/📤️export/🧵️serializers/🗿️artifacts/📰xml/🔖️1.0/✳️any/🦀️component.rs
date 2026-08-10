//! 📤️ Serialize `stdio.svg` to stdio.xml.

use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use crate::artifacts::svg::SvgSnapshot;

pub fn register() {}

pub fn serialize(from: &SvgSnapshot) -> Result<XmlSnapshot, store::PackError> {
    let text = crate::artifacts::svg::schema::snapshot::write_svg_xml(&from.doc);
    let doc = crate::artifacts::xml::schema::snapshot::xml_document_from_text(&text)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
}
