//! 🚪️ note <- dxf — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Only `DxfEntity::Line` is
//! mapped back to ink blocks — never a general DXF importer, so this hop is `IoFidelity::Lossy`.

use crate::artifacts::note::schema::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::{parse_dxf_document, DxfEntity};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct DxfIntoNote;

impl Deserializer<NoteSnapshot> for DxfIntoNote {
    const FROM: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "DxfIntoNote: expected a text dxf payload".to_string(), diagnostics: Vec::new() });
        };
        let dxf = parse_dxf_document(text).map_err(|error| IoError { message: format!("DxfIntoNote: {error}"), diagnostics: Vec::new() })?;
        let mut snap = empty_note_snapshot();
        snap.id = create_note_id("dxf-import");
        snap.title = Some("Imported DXF".into());
        let mut i = 0usize;
        for entity in &dxf.entities {
            if let DxfEntity::Line { start, end, .. } = entity {
                snap.blocks.push(NoteBlockNode::Ink {
                    id: format!("dxf-line-{i}"),
                    name: "Line".into(),
                    x: start[0].min(end[0]),
                    y: start[1].min(end[1]),
                    width: (start[0] - end[0]).abs().max(1.0),
                    height: (start[1] - end[1]).abs().max(1.0),
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    points: vec![[start[0], start[1]], [end[0], end[1]]],
                    stroke_width: 1.0,
                    color: [0.0, 0.0, 0.0, 1.0],
                });
                i += 1;
            }
        }
        Ok(IoOutcome::clean(snap))
    }
}
