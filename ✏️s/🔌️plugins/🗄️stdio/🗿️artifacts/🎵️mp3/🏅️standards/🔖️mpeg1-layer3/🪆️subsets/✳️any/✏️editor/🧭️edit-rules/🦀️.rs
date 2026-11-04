//! 🧭️ The details-pane vocabulary of the mp3 editor: which snapshot pointer raises which concrete kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule};

/// 📚 The two tags and the frame list are each replaced whole by the kind that sets them; one ID3v2 frame is added (or replaced when its id exists) and removed by its id.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/id3v2", "set-id3v2", "id3v2"), EntityRule::new("/frames", "set-frames", "frames"), EntityRule::new("/id3v1", "set-id3v1", "id3v1")],
    inserts: &[InsertRule::new("/id3v2/frames", "set-id3-frame", "frame")],
    removes: &[RemoveRule::by_key("/id3v2/frames", "remove-id3-frame", "id", "id")],
};
