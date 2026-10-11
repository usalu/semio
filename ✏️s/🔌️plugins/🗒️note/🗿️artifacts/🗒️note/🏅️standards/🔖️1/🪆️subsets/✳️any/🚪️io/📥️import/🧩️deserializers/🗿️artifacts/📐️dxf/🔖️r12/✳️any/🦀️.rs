//! 🚪️ note <- dxf — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Only `DxfEntity::Line` is
//! mapped back to ink blocks — never a general DXF importer, so this hop is `IoFidelity::Lossy`.

use crate::schema::{create_note_id, NoteIdOwner};
use crate::standards::v1::subsets::any::io::text::snapshot::{empty_note_snapshot};
use crate::{NoteBlockNode, NoteSnapshot};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_dxf::schema::snapshot::{DxfEntity, DxfLine};
use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::io::text::snapshot::{parse_dxf_document};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct DxfIntoNote;

impl Deserializer<NoteSnapshot> for DxfIntoNote {
    const FROM: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<NoteSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DxfIntoNote: expected a text dxf payload".to_string())));
        };
        let dxf = parse_dxf_document(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DxfIntoNote: {error}"))))?;
        let mut ids = NoteIdOwner::new(format!("dxf-import:{}", text.len()), 0);
        let mut snap = empty_note_snapshot();
        snap.id = create_note_id(&mut ids, "dxf-import");
        snap.title = Some("Imported DXF".into());
        let mut i = 0usize;
        for entity in &dxf.entities {
            if let DxfEntity::Line(DxfLine { start, end, .. }) = entity {
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
