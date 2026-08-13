//! note <- pdf
//!
//! 🩹️ `stdio_gap`/foreign-lag fix — see the paired export leaf's doc comment (same wave, the
//! single-page `PageDoc` -> multi-page `pages: Vec<PdfPage>`/`media_box` shape plus
//! `decode_pdf`'s error type change). Only the first page is mapped — the same single-page scope
//! the old reader covered. `decode_pdf` comes directly from 1.7's own `subsets::any::io` (the
//! `pdf::engine` shim it used to reach through is dissolved, ticket 26/08/12/ENGINELESS-
//! ARTIFACTS-AND-APP-STATE-MACHINES).
use crate::artifacts::note::schema::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PdfPage;
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;
pub fn register() {}
pub fn deserialize(from: &PdfSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("pdf-import");
    snap.title = Some("Imported PDF".into());
    let page = from.pages.first().cloned().unwrap_or_default();
    let PdfPage { media_box: [x0, y0, x1, y1], text, .. } = page;
    snap.blocks.push(NoteBlockNode::Text {
        id: "pdf-text-1".into(), name: "PDF".into(), x: 0.0, y: 0.0,
        width: (x1 - x0).max(1.0), height: (y1 - y0).max(1.0),
        rotation: 0.0, visible: true, locked: false,
        paragraphs: vec![NoteTextParagraph { runs: vec![NoteTextRun { text, bold: None, italic: None, underline: None, link: None }] }],
        font_size: 12.0, font_weight: "normal".into(), align: "left".into(),
    });
    Ok(snap)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    deserialize(&semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf(bytes).map_err(|e| e.to_string())?)
}
