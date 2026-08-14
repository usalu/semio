//! 📥️ Deserialize `stdio.svg` from stdio.xml.

use crate::artifacts::xml::XmlSnapshot;
use crate::artifacts::xml::schema::snapshot::XmlNode;
use crate::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &XmlSnapshot) -> Result<SvgSnapshot, store::TextError> {
    match &from.doc.root {
        Some(XmlNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => {
            Ok(SvgSnapshot { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: from.doc.clone() })
        }
        _ => Err(store::TextError::new("root element must be svg", dsl::TextSpan::at(1, 1))),
    }
}
