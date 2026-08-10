//! note -> pdf
use crate::artifacts::note::engine::{flatten_blocks, note_document_bounds};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::engine::{encode_pdf, empty_pdf_snapshot};
use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc;
pub fn register() {}
pub fn serialize(snapshot: &NoteSnapshot) -> Result<semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot, String> {
    let (w, h) = note_document_bounds(snapshot);
    let mut text = String::new();
    if let Some(title) = &snapshot.title { text.push_str(title); text.push(' '); }
    for block in flatten_blocks(&snapshot.blocks) {
        if let NoteBlockNode::Text { paragraphs, .. } = block {
            for p in paragraphs { for r in &p.runs { text.push_str(&r.text); text.push(' '); } }
        }
    }
    let mut snap = empty_pdf_snapshot();
    snap.page = PageDoc { width: w.max(1) as f64, height: h.max(1) as f64, text: text.trim().to_string() };
    Ok(snap)
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> { encode_pdf(&serialize(snapshot)?) }
