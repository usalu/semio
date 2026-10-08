//! 🧭️ The details-pane vocabulary of the zip editor: which snapshot pointer raises which concrete kind.
//! Adding a member is a computed gesture (the kind names the member it goes before) answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule, RemoveRule, Selector};

const ENTRY: &[Selector] = &[Selector::Field { payload: "name", field: "name" }];

/// 📚 Members are addressed by their unique name.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/entries/*/data", "set-entry-data", "data").selecting(ENTRY),
        EntityRule::new("/entries/*/name", "rename-entry", "newName").selecting(ENTRY),
        EntityRule::new("/comment", "set-archive-comment", "comment").carrying(&[Carried { payload: "commentUtf8", pointer: "/commentUtf8" }]),
        EntityRule::new("/commentUtf8", "set-archive-comment", "commentUtf8").carrying(&[Carried { payload: "comment", pointer: "/comment" }]),
    ],
    inserts: &[],
    removes: &[RemoveRule::by_key("/entries", "remove-entry", "name", "name")],
};
