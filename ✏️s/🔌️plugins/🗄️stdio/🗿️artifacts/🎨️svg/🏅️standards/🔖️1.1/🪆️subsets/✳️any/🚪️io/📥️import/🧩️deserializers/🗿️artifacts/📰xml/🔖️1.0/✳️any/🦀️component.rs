//! 📥️ Deserialize `stdio.svg` from stdio.xml.

use crate::artifacts::xml::XmlSnapshot;
use crate::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &XmlSnapshot) -> Result<SvgSnapshot, store::TextError> {
    let text = crate::artifacts::xml::schema::snapshot::xml_document_to_text(&from.doc);
    let doc = crate::artifacts::svg::schema::snapshot::parse_svg_xml(&text)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    Ok(SvgSnapshot { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
}
