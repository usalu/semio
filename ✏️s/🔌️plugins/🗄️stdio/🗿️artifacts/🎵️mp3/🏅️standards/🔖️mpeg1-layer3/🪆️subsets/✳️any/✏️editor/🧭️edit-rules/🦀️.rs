//! 🧭️ The details-pane vocabulary of the mp3 editor: which snapshot pointer raises which concrete kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule};

/// 📚 The two tags and the frame list are each replaced whole by the kind that sets them.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/id3v2", "set-id3v2", "id3v2"), EntityRule::new("/frames", "set-frames", "frames"), EntityRule::new("/id3v1", "set-id3v1", "id3v1")],
    inserts: &[],
    removes: &[],
};
