//! 📥️ Deserialize `stdio.svg` from stdio.xml.

use crate::artifacts::xml::XmlSnapshot;
use crate::artifacts::svg::SvgSnapshot;

pub fn register() {}

pub fn deserialize(from: &XmlSnapshot) -> Result<SvgSnapshot, store::TextError> {
    let text = crate::artifacts::xml::schema::snapshot::xml_document_to_text(&from.doc);
    SvgSnapshot::import_utf8(text.as_bytes()).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
