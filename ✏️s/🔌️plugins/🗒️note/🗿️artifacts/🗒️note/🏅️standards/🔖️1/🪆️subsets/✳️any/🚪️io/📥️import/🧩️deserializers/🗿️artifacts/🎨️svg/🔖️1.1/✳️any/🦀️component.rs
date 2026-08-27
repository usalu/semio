//! 🚪️ note <- svg — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Does not really parse the SVG
//! scene graph back into blocks — it dumps the (truncated) raw XML into one text block, an honest
//! `IoFidelity::Lossy` stub, unchanged behaviour from the pre-migration free function.

use crate::artifacts::note::schema::{create_note_id, empty_note_snapshot, NoteIdOwner};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct SvgIntoNote;

impl Deserializer<NoteSnapshot> for SvgIntoNote {
    const FROM: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Text(xml) = payload else {
            return Err(IoError { message: "SvgIntoNote: expected a text svg payload".to_string(), diagnostics: Vec::new() });
        };
        let mut ids = NoteIdOwner::new(format!("svg-import:{}", xml.len()), 0);
        let mut snap = empty_note_snapshot();
        snap.id = create_note_id(&mut ids, "svg-import");
        snap.title = Some("Imported SVG".into());
        let paragraphs = vec![NoteTextParagraph { runs: vec![NoteTextRun { text: xml.chars().take(512).collect(), bold: None, italic: None, underline: None, link: None }] }];
        snap.blocks.push(NoteBlockNode::Text {
            content: crate::artifacts::note::note_text_child_record("svg-text-1", &paragraphs),
            id: "svg-text-1".into(),
            name: "SVG".into(),
            x: 0.0,
            y: 0.0,
            width: 400.0,
            height: 200.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            font_size: 14.0,
            font_weight: "normal".into(),
            align: "left".into(),
        });
        Ok(IoOutcome::clean(snap))
    }
}
