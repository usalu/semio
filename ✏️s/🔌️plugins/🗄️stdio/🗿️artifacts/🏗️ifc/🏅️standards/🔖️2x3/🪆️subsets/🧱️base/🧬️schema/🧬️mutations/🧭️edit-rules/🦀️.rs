//! 🧭️ The details-pane vocabulary of the IFC 2x3 editor: which JSON-pointer edit raises which concrete kind.
//!
//! The header is set whole, an instance is written by `upsert-instance` (in place by its id) and removed by its id, so [`EDIT_RULES`] is complete:
//! an edit below the header or an instance replaces that header or instance. The document schema and the EDM preamble have no kind and are refused.
//! The three model-view subsets edit the same document and share this table.

use super::*;
use semio_framework_value::{FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, EntityRule, InsertRule, RemoveRule, SnapshotEditError, SnapshotEditEvent};

/// 📚 Every pointer an IFC 2x3 edit resolves to one kind.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/document/header", "set-header", "header"), EntityRule::new("/document/instances/*", "upsert-instance", "instance")],
    inserts: &[InsertRule::new("/document/instances", "upsert-instance", "instance").at("index")],
    removes: &[RemoveRule::by_key("/document/instances", "remove-instance", "id", "id")],
};

/// 🆔️ The rows that give the instance at position `row` another id in place: the renamed instance is inserted at that position and the old one removed, so the
/// instance keeps its position and the gesture undoes row by row. A new id another instance already holds is refused.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn renamed_instance(event: &SnapshotEditEvent, pointer: &str, snapshot: &Ifc2x3Snapshot, row: usize) -> Result<Vec<Ifc2x3Mutation>, SnapshotEditError> {
    let current = snapshot.document.instances.get(row).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("there is no instance {row}")))?;
    let renamed = Part21Instance::from_value(edited_subtree(&current.to_value(), &format!("/document/instances/{row}"), event)?).map_err(|error| SnapshotEditError::new("snapshot-edit.schema-invalid", pointer, error.to_string()))?;
    if renamed.id == current.id {
        return Ok(Vec::new());
    }
    if snapshot.document.instances.iter().any(|existing| existing.id == renamed.id) {
        return Err(SnapshotEditError::new("snapshot-edit.schema-invalid", pointer, format!("instance #{} already exists", renamed.id)));
    }
    Ok(vec![
        Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: renamed, index: Some(row) }),
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: current.id }),
    ])
}

/// 🎯 The rows of an edit that changes an instance's id; `None` hands every other edit on to the table.
pub fn special(event: &SnapshotEditEvent, snapshot: &Ifc2x3Snapshot) -> Result<Option<Vec<Ifc2x3Mutation>>, SnapshotEditError> {
    let SnapshotEditEvent::SetValue { path, .. } = event else { return Ok(None) };
    let segments: Vec<&str> = path.split('/').skip(1).collect();
    let ["document", "instances", row, rest @ ..] = segments.as_slice() else { return Ok(None) };
    let row: usize = row.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("'{row}' is no instance position")))?;
    match rest {
        ["id"] | [] => {
            let rows = renamed_instance(event, path, snapshot, row)?;
            Ok((!rows.is_empty() || rest == ["id"]).then_some(rows))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
