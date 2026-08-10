//! Deserialize dag via stdio.svg.
use crate::artifacts::dag::DagSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::xml::schema::snapshot::xml_document_to_text;

pub fn register() {}

pub fn deserialize(from: &SvgSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_SVG_DOCUMENT_SCHEMA;
    let text = xml_document_to_text(&from.doc);
    <DagSnapshot as store::DocumentDsl>::parse_dsl(&text)
}
