//! 🚪️ drawing -> dwg — foreign `Serializer<DrawingSnapshot>`: the document's flattened scene projected
//! onto `s.stdio.semio/v1/drawing` (`drawing_document_to_semio_drawing`, the same projection the svg
//! export uses) and written as DWG entities by that subset's own export leaf — the one dwg writer every
//! owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: gradients, blend modes and fill rules have no `SemioDrawingSnapshot`
//! equivalent, and there is no dwg import (a dwg file carries no drawing layer model).
use crate::drawing_document_to_semio_drawing;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use {semio_s_artifact_stdio_semio_drawing_export::encode_drawing, semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat};

pub const DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };

pub struct DrawingIntoDwg;

impl Serializer<DrawingSnapshot> for DrawingIntoDwg {
    const INTO: Dialect = DWG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &DrawingSnapshot, _: &ArchiveChildren, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let error = |message: String| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DrawingIntoDwg: {message}")));
        let bytes = encode_drawing(&drawing_document_to_semio_drawing(from).map_err(error)?, SemioDrawingFormat::Dwg).map_err(error)?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
