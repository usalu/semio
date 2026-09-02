//! 🚪️ drawing -> dxf — foreign `Serializer<DrawingSnapshot>` (design.md §3). Honest not-yet-implemented
//! stub: the pre-migration free function this replaces printed the artifact's OWN `.drawing` DSL text
//! and mislabeled it as DXF bytes — a real correctness bug. Fixed here by refusing honestly instead
//! of perpetuating the mislabeled payload; real DXF export is out of scope for this cutover.

use crate::artifacts::drawing::DrawingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct DrawingIntoDxf;

impl Serializer<DrawingSnapshot> for DrawingIntoDxf {
    const INTO: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &DrawingSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "DrawingIntoDxf: DXF export is not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
