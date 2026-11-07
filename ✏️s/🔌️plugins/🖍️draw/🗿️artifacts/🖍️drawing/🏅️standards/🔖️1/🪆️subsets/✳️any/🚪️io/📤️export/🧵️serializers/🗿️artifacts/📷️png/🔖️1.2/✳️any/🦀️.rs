//! 🚪️ Drawing to PNG: bytes come only from the owned job-driven raster export (`DocumentRasterJob` then `PngEncodeJob`).

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct DrawingIntoPng;

impl Serializer<DrawingSnapshot> for DrawingIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    const OWNED_FACTORY: Option<semio_os_kernel::io::io_mechanism::OwnedSerializerFactory> = Some(crate::owned::PNG_FACTORY);
    async fn serialize(_: &DrawingSnapshot, _: &ArchiveChildren, _: &mut semio_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Err(crate::owned_only_refusal("s.stdio.png"))
    }
}
