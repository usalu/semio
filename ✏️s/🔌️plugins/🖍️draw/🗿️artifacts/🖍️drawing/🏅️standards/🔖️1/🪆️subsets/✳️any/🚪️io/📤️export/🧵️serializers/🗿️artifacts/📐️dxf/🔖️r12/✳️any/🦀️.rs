//! 🚪️ Drawing to DXF R12: bytes come only from the owned job-driven export (`DocumentVectorJob`, semio drawing projection, the one stdio DXF writer).

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct DrawingIntoDxf;

impl Serializer<DrawingSnapshot> for DrawingIntoDxf {
    const INTO: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    const OWNED_FACTORY: Option<semio_os_kernel::io::io_mechanism::OwnedSerializerFactory> = Some(crate::owned::DXF_FACTORY);
    async fn serialize(_: &DrawingSnapshot, _: &ArchiveChildren, _: &mut semio_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Err(crate::owned_only_refusal("s.stdio.dxf"))
    }
}
