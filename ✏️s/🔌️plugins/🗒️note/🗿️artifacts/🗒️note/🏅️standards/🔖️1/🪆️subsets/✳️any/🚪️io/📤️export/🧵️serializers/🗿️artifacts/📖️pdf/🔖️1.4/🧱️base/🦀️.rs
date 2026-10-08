//! 📤️ Exports note text onto one PDF 1.4 page.

use crate::standards::v1::subsets::any::io::note_document_bounds;
use crate::schema::flatten_blocks;
use crate::{NoteBlockNode, NoteSnapshot};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::io::encode_pdf;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::empty_pdf_snapshot;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PageDoc;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct NoteIntoPdf;

impl Serializer<NoteSnapshot> for NoteIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &NoteSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let (w, h) = note_document_bounds(from);
        let mut text = String::new();
        if let Some(title) = &from.title {
            text.push_str(title);
            text.push(' ');
        }
        for block in flatten_blocks(&from.blocks) {
            if let NoteBlockNode::Text { content, .. } = block {
                for paragraph in crate::note_block_text(content) {
                    for run in &paragraph.runs {
                        text.push_str(&run.text);
                        text.push(' ');
                    }
                }
            }
        }
        let mut snapshot = empty_pdf_snapshot();
        let mut page = PageDoc::new(w.max(1) as f64, h.max(1) as f64);
        page.text = text.trim().to_string();
        snapshot.pages = vec![page];
        let bytes = encode_pdf(&snapshot).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("NoteIntoPdf: encode failed: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
