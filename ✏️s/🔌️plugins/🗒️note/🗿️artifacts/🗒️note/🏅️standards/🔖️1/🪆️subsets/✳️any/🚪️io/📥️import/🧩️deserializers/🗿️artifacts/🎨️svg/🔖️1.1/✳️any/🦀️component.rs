//! note <- svg
use crate::artifacts::note::schema::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, write_svg_xml, SvgSnapshot};
use semio_s_plugin_stdio::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;
pub fn register() {}
pub fn deserialize(from: &SvgSnapshot) -> Result<NoteSnapshot, String> {
    let xml = write_svg_xml(&from.doc);
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("svg-import");
    snap.title = Some("Imported SVG".into());
    let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: xml.chars().take(512).collect(), bold: None, italic: None, underline: None, link: None }] }];
    snap.blocks.push(NoteBlockNode::Text {
        content: crate::artifacts::note::note_text_child_handle_and_cache("svg-text-1", &paragraphs),
        id: "svg-text-1".into(), name: "SVG".into(), x: 0.0, y: 0.0, width: 400.0, height: 200.0,
        rotation: 0.0, visible: true, locked: false,
        font_size: 14.0, font_weight: "normal".into(), align: "left".into(),
    });
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    deserialize(&SvgSnapshot { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: parse_svg_xml(text)? })
}
