//! 🧭️ The details-pane vocabulary of the IFC4 editor: which JSON-pointer edit raises which concrete kind.
//!
//! The three header records, the name and arguments of an entity and the entity list each have a kind, so [`EDIT_RULES`] is complete: an entity is
//! addressed by its id, an argument by its position, and an edit below an argument replaces that argument. Complex constituents, the entity ids
//! and the document schema have no kind and are refused.

use super::*;
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};

const ENTITY: &[Selector] = &[Selector::Field { payload: "id", field: "id" }];
const ARGUMENT: &[Selector] = &[Selector::Field { payload: "id", field: "id" }, Selector::Index("index")];

/// 📚 Every pointer an IFC4 edit resolves to one kind.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/header/fileDescription", "set-file-description", "values"),
        EntityRule::new("/header/fileName", "set-file-name", "values"),
        EntityRule::new("/header/fileSchema", "set-file-schema", "values"),
        EntityRule::new("/entities/*/name", "set-entity-name", "name").selecting(ENTITY),
        EntityRule::new("/entities/*/args/*", "set-entity-arg", "value").selecting(ARGUMENT),
    ],
    inserts: &[InsertRule::new("/entities", "insert-entity", "entity").at("index"), InsertRule::new("/entities/*/args", "insert-entity-arg", "value").at("index").selecting(ENTITY)],
    removes: &[RemoveRule::by_key("/entities", "remove-entity", "id", "id"), RemoveRule::by_index("/entities/*/args", "remove-entity-arg", "index").selecting(ENTITY)],
};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
