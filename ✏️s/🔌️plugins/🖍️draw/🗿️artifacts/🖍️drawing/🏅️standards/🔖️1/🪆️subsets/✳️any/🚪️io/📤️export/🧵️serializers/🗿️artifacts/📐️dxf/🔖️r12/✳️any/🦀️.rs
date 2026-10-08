//! 🚪️ drawing -> dxf — foreign `Serializer<DrawingSnapshot>`: the document's flattened scene projected
//! onto `s.stdio.semio/v1/drawing` (`drawing_document_to_semio_drawing`, the same projection the svg
//! export uses) and written as DXF R12 entities (a circle stays a CIRCLE, curves become sampled polylines) by that subset's own export leaf — the one dxf writer every
//! owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: gradients, blend modes and fill rules have no `SemioDrawingSnapshot`
//! equivalent, and there is no dxf import (a dxf file carries no drawing layer model).
use crate::standards::v1::subsets::any::io::drawing_document_to_semio_drawing;
use crate::DrawingSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct DrawingIntoDxf;

impl Serializer<DrawingSnapshot> for DrawingIntoDxf {
    const INTO: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &DrawingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let error = |message: String| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DrawingIntoDxf: {message}")));
        let bytes = encode_drawing(&drawing_document_to_semio_drawing(from), SemioDrawingFormat::Dxf).map_err(error)?;
        Ok(IoOutcome::clean(IoPayload::Text(String::from_utf8(bytes).map_err(|e| error(e.to_string()))?)))
    }
}
