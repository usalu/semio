//! 🧭️ The details-pane vocabulary of the txt editor: which snapshot pointer raises which concrete kind.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

/// 📚 Lines are addressed by position; the terminator and the line ending are their own kinds.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/lines/*", "set-line", "text").selecting(&[Selector::Index("index")]),
        EntityRule::new("/trailingNewline", "set-trailing-newline", "value"),
        EntityRule::new("/lineEnding", "set-line-ending", "value"),
    ],
    inserts: &[InsertRule::new("/lines", "insert-line", "text").at("index")],
    removes: &[RemoveRule::by_index("/lines", "remove-line", "index")],
};
