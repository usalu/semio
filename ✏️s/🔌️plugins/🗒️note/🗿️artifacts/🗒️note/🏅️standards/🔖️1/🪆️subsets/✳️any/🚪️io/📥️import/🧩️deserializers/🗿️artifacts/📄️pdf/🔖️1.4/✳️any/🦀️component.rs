//! 🚪️ note <- pdf — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Only the first page's text is
//! mapped into one text block (`stdio_gap`/foreign-lag fix carried over from the pre-migration free
//! function — stdio's 1.7 multi-page model) — an honest `IoFidelity::Lossy` hop.

use crate::artifacts::note::schema::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PdfPage;
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PdfIntoNote;

impl Deserializer<NoteSnapshot> for PdfIntoNote {
    const FROM: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PdfIntoNote: expected a binary pdf payload".to_string(), diagnostics: Vec::new() });
        };
        let pdf = decode_pdf(bytes).map_err(|error| IoError { message: format!("PdfIntoNote: decode failed: {error}"), diagnostics: Vec::new() })?;
        let mut snap = empty_note_snapshot();
        snap.id = create_note_id("pdf-import");
        snap.title = Some("Imported PDF".into());
        let page = pdf.pages.first().cloned().unwrap_or_default();
        let PdfPage { media_box: [x0, y0, x1, y1], text, .. } = page;
        let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text, bold: None, italic: None, underline: None, link: None }] }];
        snap.blocks.push(NoteBlockNode::Text {
            content: crate::artifacts::note::note_text_child_record("pdf-text-1", &paragraphs),
            id: "pdf-text-1".into(),
            name: "PDF".into(),
            x: 0.0,
            y: 0.0,
            width: (x1 - x0).max(1.0),
            height: (y1 - y0).max(1.0),
            rotation: 0.0,
            visible: true,
            locked: false,
            font_size: 12.0,
            font_weight: "normal".into(),
            align: "left".into(),
        });
        Ok(IoOutcome::clean(snap))
    }
}
