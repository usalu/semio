//! 🧭️ The details-pane vocabulary of the csv editor: which snapshot pointer raises which concrete kind.
//! A change of a row's field count (a field inserted, removed or the whole row replaced) is a computed gesture answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule, InsertRule, RemoveRule, Selector};

const FIELD: &[Selector] = &[Selector::Index("recordIndex"), Selector::Index("fieldIndex")];

/// 📚 Records and fields are addressed by position; a field's text and its quoting travel together in `set-field`.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/hasHeader", "set-has-header", "hasHeader"),
        EntityRule::new("/records/*/fields/*/value", "set-field", "value").selecting(FIELD).carrying(&[Carried { payload: "quoted", pointer: "/records/*/fields/*/quoted" }]),
        EntityRule::new("/records/*/fields/*/quoted", "set-field", "quoted").selecting(FIELD).carrying(&[Carried { payload: "value", pointer: "/records/*/fields/*/value" }]),
    ],
    inserts: &[InsertRule::new("/records", "insert-record", "record").at("index")],
    removes: &[RemoveRule::by_index("/records", "remove-record", "index")],
};
