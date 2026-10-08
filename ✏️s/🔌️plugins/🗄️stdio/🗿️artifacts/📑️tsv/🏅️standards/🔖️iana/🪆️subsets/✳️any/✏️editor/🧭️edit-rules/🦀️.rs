//! 🧭️ The details-pane vocabulary of the tsv editor: which snapshot pointer raises which concrete kind.
//! A change of a row's cell count (a cell inserted, removed or the whole row replaced) is a computed gesture answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

/// 📚 Rows and cells are addressed by position.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/trailingNewline", "set-trailing-newline", "trailingNewline"),
        EntityRule::new("/lineEnding", "set-line-ending", "lineEnding"),
        EntityRule::new("/records/*/*", "set-cell", "value").selecting(&[Selector::Index("rowIndex"), Selector::Index("fieldIndex")]),
    ],
    inserts: &[InsertRule::new("/records", "insert-row", "row").at("index")],
    removes: &[RemoveRule::by_index("/records", "remove-row", "index")],
};
