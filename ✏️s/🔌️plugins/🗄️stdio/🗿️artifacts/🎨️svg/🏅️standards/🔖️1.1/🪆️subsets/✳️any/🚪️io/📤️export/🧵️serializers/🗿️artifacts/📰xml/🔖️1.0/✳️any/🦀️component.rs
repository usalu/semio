//! 📤️ Serialize `stdio.svg` to stdio.xml.

use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &SvgSnapshot) -> Result<XmlSnapshot, store::PackError> {
    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: from.doc.clone() })
}
