//! 📥️ Imports the first PDF 1.4 page as one text block.

use crate::schema::{create_note_id, empty_note_snapshot, NoteIdOwner};
use crate::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::decode_pdf;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PdfIntoNote;

impl Deserializer<NoteSnapshot> for PdfIntoNote {
    const FROM: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PdfIntoNote: expected a binary pdf payload".to_string(), diagnostics: Vec::new() });
        };
        let pdf = decode_pdf(bytes).map_err(|error| IoError { message: format!("PdfIntoNote: decode failed: {error}"), diagnostics: Vec::new() })?;
        let mut ids = NoteIdOwner::new(format!("pdf-import:{}", bytes.len()), 0);
        let mut snap = empty_note_snapshot();
        snap.id = create_note_id(&mut ids, "pdf-import");
        snap.title = Some("Imported PDF".into());
        let page = pdf.pages.first().cloned().unwrap_or_default();
        let PageDoc { width, height, text } = page;
        let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text, bold: None, italic: None, underline: None, link: None }] }];
        snap.blocks.push(NoteBlockNode::Text {
            content: crate::note_text_child_record("pdf-text-1", &paragraphs),
            id: "pdf-text-1".into(),
            name: "PDF".into(),
            x: 0.0,
            y: 0.0,
            width: width.max(1.0),
            height: height.max(1.0),
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
