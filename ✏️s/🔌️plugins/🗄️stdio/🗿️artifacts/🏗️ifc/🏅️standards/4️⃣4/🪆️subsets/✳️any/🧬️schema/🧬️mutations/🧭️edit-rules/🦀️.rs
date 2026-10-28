//! 🧭️ The details-pane vocabulary of the IFC4 editor: which JSON-pointer edit raises which concrete kind.
//!
//! The three header records, the name and arguments of an entity and the entity list each have a kind, so [`EDIT_RULES`] is complete: an entity is
//! addressed by its id, an argument by its position, and an edit below an argument replaces that argument. Complex constituents use a whole-list replacement; entity ids
//! and the document schema have no kind and are refused.

use super::*;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule, Selector};
use semio_s_artifact_stdio_contract::editing::{SnapshotEditError, SnapshotEditEvent, edited_subtree};

const ENTITY: &[Selector] = &[Selector::Field { payload: "id", field: "id" }];
const ARGUMENT: &[Selector] = &[Selector::Field { payload: "id", field: "id" }, Selector::Index("index")];

/// 📚 Every pointer an IFC4 edit resolves to one kind.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/header/fileDescription", "set-file-description", "values"),
        EntityRule::new("/header/fileName", "set-file-name", "values"),
        EntityRule::new("/header/fileSchema", "set-file-schema", "values"),
        EntityRule::new("/entities/*/name", "set-entity-name", "name").selecting(ENTITY),
        EntityRule::new("/entities/*/complex", "set-entity-complex", "complex").selecting(ENTITY),
        EntityRule::new("/entities/*/args/*", "set-entity-arg", "value").selecting(ARGUMENT),
    ],
    inserts: &[InsertRule::new("/entities", "insert-entity", "entity").at("index"), InsertRule::new("/entities/*/args", "insert-entity-arg", "value").at("index").selecting(ENTITY)],
    removes: &[RemoveRule::by_key("/entities", "remove-entity", "id", "id"), RemoveRule::by_index("/entities/*/args", "remove-entity-arg", "index").selecting(ENTITY)],
};

/// 🧩️ Plans complex-field gestures against its default empty list, including removing the whole optional field.
pub fn complex_edit(event: &SnapshotEditEvent, snapshot: &IfcSnapshot) -> Result<Option<Vec<IfcMutation>>, SnapshotEditError> {
    let path = match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::MoveValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } => path,
        SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
    };
    let segments: Vec<&str> = path.split('/').collect();
    let ["", "entities", position, "complex", ..] = segments.as_slice() else { return Ok(None) };
    let Ok(index) = position.parse::<usize>() else { return Ok(None) };
    if index.to_string() != *position {
        return Ok(None);
    }
    let entity = snapshot.entities.get(index).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, "complex entity does not exist"))?;
    let prefix = format!("/entities/{index}/complex");
    let value = if path == &prefix {
        match event {
            SnapshotEditEvent::SetValue { value, .. } => value.clone(),
            SnapshotEditEvent::InsertValue { value, .. } if entity.complex.is_empty() => value.clone(),
            SnapshotEditEvent::InsertValue { .. } => return Err(SnapshotEditError::new("snapshot-edit.key-exists", path, "complex field already exists")),
            SnapshotEditEvent::RemoveValue { .. } => DslValue::Array(Vec::new()),
            _ => return Err(SnapshotEditError::new("snapshot-edit.unsupported-path", path, "the gesture does not replace the complex field")),
        }
    } else {
        edited_subtree(&entity.complex.to_value(), &prefix, event)?
    };
    let complex = Vec::from_value(value).map_err(|error| SnapshotEditError::new("snapshot-edit.schema-invalid", path, error.to_string()))?;
    Ok(Some(if semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::ifc_complex_identical(&entity.complex, &complex) {
        Vec::new()
    } else {
        vec![IfcMutation::SetEntityComplex(set_entity_complex::SetEntityComplex { id: entity.id, complex })]
    }))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
