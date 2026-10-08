//! 🧭️ The details-pane vocabulary of the ply editor: which snapshot pointer raises which concrete kind.
//! A row value edit (its property name comes from the element's declaration) and a comment replacement (a remove then an insert) are computed gestures answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

/// 📚 Rows are addressed by their element's name and their position; comments by position.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/format", "set-format", "format")],
    inserts: &[
        InsertRule::new("/comments", "insert-comment", "comment").at("index"),
        InsertRule::new("/elements", "add-element", "element").at("index"),
        InsertRule::new("/elements/*/rows", "insert-row", "row").at("index").selecting(&[Selector::Field { payload: "elementName", field: "name" }]),
    ],
    removes: &[
        RemoveRule::by_index("/comments", "remove-comment", "index"),
        RemoveRule::by_key("/elements", "remove-element", "name", "name"),
        RemoveRule::by_index("/elements/*/rows", "remove-row", "index").selecting(&[Selector::Field { payload: "elementName", field: "name" }]),
    ],
};
