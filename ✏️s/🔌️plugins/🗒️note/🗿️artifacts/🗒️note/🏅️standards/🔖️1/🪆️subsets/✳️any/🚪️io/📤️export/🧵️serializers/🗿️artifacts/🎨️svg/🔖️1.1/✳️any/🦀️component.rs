//! note -> svg
use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, SvgSnapshot};
use semio_s_plugin_stdio::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;
pub fn register() {}
pub fn serialize(snapshot: &NoteSnapshot) -> Result<SvgSnapshot, String> {
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot)?;
    Ok(SvgSnapshot { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: parse_svg_xml(&svg)? })
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    Ok(crate::artifacts::note::engine::note_document_to_svg(snapshot)?.0.into_bytes())
}
