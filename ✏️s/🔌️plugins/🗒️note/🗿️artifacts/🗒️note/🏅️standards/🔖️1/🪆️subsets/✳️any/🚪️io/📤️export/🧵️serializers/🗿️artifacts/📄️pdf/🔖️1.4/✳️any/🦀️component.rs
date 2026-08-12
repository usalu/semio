//! note -> pdf
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--report.md`): stdio's top-level `pdf::schema`/`pdf::engine`/`pdf::PdfSnapshot` shims were
//! repointed from the old 1.4 `PageDoc{width,height,text}` stub to 1.7's real multi-page object
//! graph (`PdfSnapshot.pages: Vec<PdfPage>`, `PdfPage{media_box:[f64;4],crop_box,rotate,text}`) —
//! a concurrent stdio wave's S-6 canonicalization. `encode_pdf` also now returns
//! `Result<_, PdfEngineError>` instead of `Result<_, String>`; `.to_string()`d at this leaf's own
//! `String`-error boundary (`PdfEngineError` implements `Display`).
use crate::artifacts::note::engine::{flatten_blocks, note_document_bounds};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::engine::{encode_pdf, empty_pdf_snapshot};
use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PdfPage;
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
    let mut page = PdfPage::new(w.max(1) as f64, h.max(1) as f64);
    page.text = text.trim().to_string();
    snap.pages = vec![page];
    Ok(snap)
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> { encode_pdf(&serialize(snapshot)?).map_err(|e| e.to_string()) }
