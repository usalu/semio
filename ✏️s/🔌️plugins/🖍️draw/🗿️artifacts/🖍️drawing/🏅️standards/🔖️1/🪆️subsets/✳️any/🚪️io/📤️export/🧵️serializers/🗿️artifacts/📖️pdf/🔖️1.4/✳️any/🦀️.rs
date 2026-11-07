//! 🚪️ Drawing to PDF 1.4: bytes come only from the owned job-driven export (`DocumentVectorJob` then `PdfWriteJob`); the page is the artboard and every node keeps its own matrix.

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct DrawingIntoPdf;

impl Serializer<DrawingSnapshot> for DrawingIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    const OWNED_FACTORY: Option<semio_os_kernel::io::io_mechanism::OwnedSerializerFactory> = Some(crate::owned::PDF_FACTORY);
    async fn serialize(_: &DrawingSnapshot, _: &ArchiveChildren, _: &mut semio_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Err(crate::owned_only_refusal("s.stdio.pdf"))
    }
}

#[path="🧵️write/🦀️.rs"]
pub mod write;
