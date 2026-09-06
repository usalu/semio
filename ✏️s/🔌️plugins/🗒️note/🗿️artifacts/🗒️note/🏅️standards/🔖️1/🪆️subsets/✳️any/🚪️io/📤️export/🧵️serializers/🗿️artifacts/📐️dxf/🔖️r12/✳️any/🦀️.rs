//! 🚪️ note -> dxf — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Only ink stroke segments
//! become `DxfEntity::Line` entries — every other block kind is dropped entirely, so this hop is
//! `IoFidelity::Lossy`. DXF R12's ASCII text form is DXF's own native encoding, so the payload is
//! `Text`, never a raw-bytes wrapper.

use crate::artifacts::note::schema::flatten_blocks;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::{print_dxf_document, DxfEntity};
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct NoteIntoDxf;

impl Serializer<NoteSnapshot> for NoteIntoDxf {
    const INTO: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &NoteSnapshot) -> IoResult<IoPayload> {
        let mut entities = Vec::new();
        for block in flatten_blocks(&from.blocks) {
            if let NoteBlockNode::Ink { points, .. } = block {
                for pair in points.windows(2) {
                    entities.push(DxfEntity::Line { start: [pair[0][0], pair[0][1], 0.0], end: [pair[1][0], pair[1][1], 0.0], layer: "0".into(), unknown_group_codes: Vec::new() });
                }
            }
        }
        let snapshot = DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), entities, ..DxfSnapshot::default() };
        Ok(IoOutcome::clean(IoPayload::Text(print_dxf_document(&snapshot))))
    }
}
