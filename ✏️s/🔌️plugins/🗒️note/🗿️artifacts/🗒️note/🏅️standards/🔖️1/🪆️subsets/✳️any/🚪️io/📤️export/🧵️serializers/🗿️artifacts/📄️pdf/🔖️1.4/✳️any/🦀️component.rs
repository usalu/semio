//! 🚪️ note -> pdf — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Flattens every text block's
//! content onto one PDF page (`stdio_gap`/foreign-lag fix carried over from the pre-migration free
//! function — stdio's 1.7 multi-page `PdfSnapshot.pages: Vec<PdfPage>`), losing all layout/visual
//! structure — an honest `IoFidelity::Lossy` hop.

use crate::artifacts::note::io::note_document_bounds;
use crate::artifacts::note::schema::flatten_blocks;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PdfPage;
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::io::encode_pdf;
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::empty_pdf_snapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct NoteIntoPdf;

impl Serializer<NoteSnapshot> for NoteIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &NoteSnapshot) -> IoResult<IoPayload> {
        let (w, h) = note_document_bounds(from);
        let mut text = String::new();
        if let Some(title) = &from.title {
            text.push_str(title);
            text.push(' ');
        }
        for block in flatten_blocks(&from.blocks) {
            if let NoteBlockNode::Text { content, .. } = block {
                for paragraph in crate::artifacts::note::note_block_text(content) {
                    for run in &paragraph.runs {
                        text.push_str(&run.text);
                        text.push(' ');
                    }
                }
            }
        }
        let mut snapshot = empty_pdf_snapshot();
        let mut page = PdfPage::new(w.max(1) as f64, h.max(1) as f64);
        page.text = text.trim().to_string();
        snapshot.pages = vec![page];
        let bytes = encode_pdf(&snapshot).map_err(|error| IoError { message: format!("NoteIntoPdf: encode failed: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
