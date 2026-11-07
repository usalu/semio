//! 🚪️ Drawing to DWG AC1018: bytes come only from the owned job-driven export (`DocumentVectorJob`, semio drawing projection, the one stdio DWG writer).

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };

pub struct DrawingIntoDwg;

impl Serializer<DrawingSnapshot> for DrawingIntoDwg {
    const INTO: Dialect = DWG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    const OWNED_FACTORY: Option<semio_os_kernel::io::io_mechanism::OwnedSerializerFactory> = Some(crate::owned::DWG_FACTORY);
    async fn serialize(_: &DrawingSnapshot, _: &ArchiveChildren, _: &mut semio_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Err(crate::owned_only_refusal("s.stdio.dwg"))
    }
}
