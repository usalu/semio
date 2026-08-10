//! note <- pdf
use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;
pub fn register() {}
pub fn deserialize(from: &PdfSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("pdf-import");
    snap.title = Some("Imported PDF".into());
    snap.blocks.push(NoteBlockNode::Text {
        id: "pdf-text-1".into(), name: "PDF".into(), x: 0.0, y: 0.0,
        width: from.page.width.max(1.0), height: from.page.height.max(1.0),
        rotation: 0.0, visible: true, locked: false,
        paragraphs: vec![NoteTextParagraph { runs: vec![NoteTextRun { text: from.page.text.clone(), bold: None, italic: None, underline: None, link: None }] }],
        font_size: 12.0, font_weight: "normal".into(), align: "left".into(),
    });
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    deserialize(&semio_s_plugin_stdio::artifacts::pdf::engine::decode_pdf(bytes)?)
}
