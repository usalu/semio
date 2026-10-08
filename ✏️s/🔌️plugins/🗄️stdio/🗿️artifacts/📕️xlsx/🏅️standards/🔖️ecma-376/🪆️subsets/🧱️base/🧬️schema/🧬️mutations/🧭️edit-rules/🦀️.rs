//! 🧭️ The details-pane vocabulary of the XLSX editor: which JSON-pointer edit raises which concrete kind.
//!
//! The table [`EDIT_RULES`] is empty: a workbook is edited through the nodes of its XML parts, and [`special`] reads what the addressed part is.
//! - In a worksheet, an edit at or below a `<c>` element raises `set-cell` carrying the edited element verbatim; inserting or removing a `<c>` child of a
//!   `<row>` raises `insert-cell` / `remove-cell` at the cell's reference.
//! - In the shared strings, an edit at or below an `<si>` raises `set-shared-string`; inserting or removing an `<si>` child raises
//!   `insert-shared-string` / `remove-shared-string` at its table position.
//! - In the workbook, changing the `name` of a `<sheet>` raises `rename-sheet`, inserting a named `<sheet>` raises `insert-sheet` (an empty sheet at that position)
//!   and removing a `<sheet>` raises `remove-sheet`.
//!
//! Every other node (rows, columns, styles, merged ranges, the other parts, the OPC layer) has no kind and is refused.

use super::*;
use crate::schema::vocabulary::{column_index, column_letters_of};
use canonical_edit::{node_cell_value, part_role, shared_string_position, shared_string_text, sheet_element_name, PartRole};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_zip::opc::OpcRelationship;
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditError, SnapshotEditEvent};

/// 📚 No document field has a kind of its own: parts and nodes are resolved by [`special`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

fn refusal(pointer: &str, message: impl Into<String>) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.schema-invalid", pointer, message)
}

fn unsupported(pointer: &str, message: impl Into<String>) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.unsupported-path", pointer, message)
}

fn event_path(event: &SnapshotEditEvent) -> Option<&str> {
    match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::MoveValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } => Some(path),
        SnapshotEditEvent::ReplaceSource { .. } => None,
    }
}

fn node_at<'a>(root: &'a XmlNode, path: &[usize], pointer: &str) -> Result<&'a XmlNode, SnapshotEditError> {
    path.iter().try_fold(root, |node, index| match node {
        XmlNode::Element { children, .. } => children.get(*index).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("child {index} is outside the {} children", children.len()))),
        _ => Err(refusal(pointer, "the pointer descends through a node that is not an element")),
    })
}

fn locate<'a>(tail: &'a [&'a str], pointer: &str) -> Result<(Vec<usize>, &'a [&'a str]), SnapshotEditError> {
    let mut node = Vec::new();
    let mut rest = tail;
    while rest.len() >= 3 && rest[0] == "children" {
        node.push(rest[1].parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("'{}' is no child position", rest[1])))?);
        rest = &rest[2..];
    }
    Ok((node, rest))
}

fn child_position(segment: &str, length: usize, pointer: &str, insert: bool) -> Result<usize, SnapshotEditError> {
    let at = if insert && segment == "-" { length } else { segment.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("'{segment}' is no child position")))? };
    if at > length || (!insert && at == length) {
        return Err(SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("position {at} is outside the {length} children")));
    }
    Ok(at)
}

fn decode_node(value: &DslValue, pointer: &str) -> Result<XmlNode, SnapshotEditError> {
    XmlNode::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))
}

fn edited_node(node: &XmlNode, prefix: &str, event: &SnapshotEditEvent, pointer: &str) -> Result<XmlNode, SnapshotEditError> {
    decode_node(&edited_subtree(&node.to_value(), prefix, event)?, pointer)
}

fn reference_of(node: &XmlNode) -> Option<(u32, u32)> {
    let XmlNode::Element { attrs, .. } = node else { return None };
    let reference = attrs.iter().find(|attr| attr.name == "r")?.value.as_str();
    let column = column_index(column_letters_of(reference))?;
    let row = reference.trim_start_matches(|character: char| character.is_ascii_alphabetic()).parse().ok()?;
    Some((row, column))
}

struct Located<'a> {
    event: &'a SnapshotEditEvent,
    pointer: &'a str,
    snapshot: &'a XlsxSnapshot,
    part_row: usize,
    part_path: &'a str,
    root: &'a XmlNode,
    path: Vec<usize>,
    rest: &'a [&'a str],
}

impl Located<'_> {
    fn prefix(&self, path: &[usize]) -> String {
        format!("/xmlParts/{}/document/root{}", self.part_row, path.iter().map(|index| format!("/children/{index}")).collect::<String>())
    }

    fn list_row(&self) -> Option<&str> {
        match self.rest {
            ["children", position] => Some(position),
            _ => None,
        }
    }
}

fn worksheet_edit(at: &Located<'_>, sheet: &str) -> Result<Option<Vec<XlsxMutation>>, SnapshotEditError> {
    let pointer = at.pointer;
    let map = |message: String| refusal(pointer, message);
    if at.path.len() >= 3 {
        let cell_path = &at.path[..3];
        let cell = node_at(at.root, cell_path, pointer)?;
        let address = cell_address::xlsx_cell_address_at_path(at.snapshot, at.part_path, cell_path.to_vec()).map_err(map)?;
        if address.local_name != "c" {
            return Ok(None);
        }
        let patched = edited_node(cell, &at.prefix(cell_path), at.event, pointer)?;
        if patched == *cell {
            return Ok(Some(Vec::new()));
        }
        let value = node_cell_value(at.snapshot, at.part_path, &cell_path[..2], &patched).map_err(|message| refusal(pointer, message))?;
        return Ok(Some(vec![XlsxMutation::SetCell(set_cell::SetCell { address, value, node: Some(patched) })]));
    }
    let (Some(position), 2) = (at.list_row(), at.path.len()) else { return Ok(None) };
    let row = node_at(at.root, &at.path, pointer)?;
    let XmlNode::Element { children, .. } = row else { return Err(refusal(pointer, "the addressed row is not an element")) };
    let cell_path = |index: usize| [at.path.as_slice(), &[index]].concat();
    match at.event {
        SnapshotEditEvent::InsertValue { value, .. } => {
            child_position(position, children.len(), pointer, true)?;
            let node = decode_node(value, pointer)?;
            let (row_number, column) = reference_of(&node).ok_or_else(|| refusal(pointer, "an inserted cell carries a reference such as B7 in its r attribute"))?;
            let address = cell_address::xlsx_cell_vacancy_address(at.snapshot, sheet, row_number, column).map_err(|message| refusal(pointer, message))?;
            let value = node_cell_value(at.snapshot, at.part_path, &at.path, &node).map_err(|message| refusal(pointer, message))?;
            Ok(Some(vec![XlsxMutation::InsertCell(insert_cell::InsertCell { address, value, node: Some(node) })]))
        }
        SnapshotEditEvent::RemoveValue { .. } => {
            let index = child_position(position, children.len(), pointer, false)?;
            let address = cell_address::xlsx_cell_address_at_path(at.snapshot, at.part_path, cell_path(index)).map_err(map)?;
            if address.local_name != "c" {
                return Ok(None);
            }
            Ok(Some(vec![XlsxMutation::RemoveCell(remove_cell::RemoveCell { address })]))
        }
        SnapshotEditEvent::SetValue { value, .. } => {
            let index = child_position(position, children.len(), pointer, false)?;
            let node = decode_node(value, pointer)?;
            if children[index] == node {
                return Ok(Some(Vec::new()));
            }
            let address = cell_address::xlsx_cell_address_at_path(at.snapshot, at.part_path, cell_path(index)).map_err(map)?;
            if address.local_name != "c" {
                return Ok(None);
            }
            let value = node_cell_value(at.snapshot, at.part_path, &at.path, &node).map_err(|message| refusal(pointer, message))?;
            Ok(Some(vec![XlsxMutation::SetCell(set_cell::SetCell { address, value, node: Some(node) })]))
        }
        SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::ReplaceSource { .. } => Ok(None),
    }
}

fn shared_strings_edit(at: &Located<'_>) -> Result<Option<Vec<XlsxMutation>>, SnapshotEditError> {
    let pointer = at.pointer;
    let map = |message: String| refusal(pointer, message);
    let XmlNode::Element { children: entries, .. } = at.root else { return Err(refusal(pointer, "the shared strings part has no element root")) };
    if let (Some(position), true) = (at.list_row(), at.path.is_empty()) {
        return match at.event {
            SnapshotEditEvent::InsertValue { value, .. } => {
                let physical = child_position(position, entries.len(), pointer, true)?;
                let node = decode_node(value, pointer)?;
                let (index, _) = shared_string_position(at.snapshot, physical).map_err(map)?;
                let text = shared_string_text(at.snapshot, &node).map_err(|message| refusal(pointer, message))?;
                Ok(Some(vec![XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: text, index: Some(index), node: Some(node) })]))
            }
            SnapshotEditEvent::RemoveValue { .. } => {
                let physical = child_position(position, entries.len(), pointer, false)?;
                let (index, is_entry) = shared_string_position(at.snapshot, physical).map_err(map)?;
                if !is_entry {
                    return Err(unsupported(pointer, "only a shared string entry is removed"));
                }
                Ok(Some(vec![XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index })]))
            }
            SnapshotEditEvent::SetValue { value, .. } => {
                let physical = child_position(position, entries.len(), pointer, false)?;
                set_entry(at, physical, decode_node(value, pointer)?)
            }
            SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::ReplaceSource { .. } => Ok(None),
        };
    }
    let Some(&physical) = at.path.first() else { return Ok(None) };
    let entry = node_at(at.root, &at.path[..1], pointer)?;
    let patched = edited_node(entry, &at.prefix(&at.path[..1]), at.event, pointer)?;
    set_entry(at, physical, patched)
}

fn set_entry(at: &Located<'_>, physical: usize, patched: XmlNode) -> Result<Option<Vec<XlsxMutation>>, SnapshotEditError> {
    let pointer = at.pointer;
    let XmlNode::Element { children, .. } = at.root else { return Err(refusal(pointer, "the shared strings part has no element root")) };
    if children[physical] == patched {
        return Ok(Some(Vec::new()));
    }
    let (index, is_entry) = shared_string_position(at.snapshot, physical).map_err(|message| refusal(pointer, message))?;
    if !is_entry {
        return Err(unsupported(pointer, "only a shared string entry is edited"));
    }
    let text = shared_string_text(at.snapshot, &patched).map_err(|message| refusal(pointer, message))?;
    Ok(Some(vec![XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value: text, node: Some(patched) })]))
}

fn workbook_edit(at: &Located<'_>) -> Result<Option<Vec<XlsxMutation>>, SnapshotEditError> {
    let pointer = at.pointer;
    let sheet_row = |segment: &str| -> Result<usize, SnapshotEditError> { segment.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("'{segment}' is no child position"))) };
    if let (Some(position), 1) = (at.list_row(), at.path.len()) {
        let sheets = node_at(at.root, &at.path, pointer)?;
        let XmlNode::Element { children, .. } = sheets else { return Err(refusal(pointer, "the addressed node is not an element")) };
        return match at.event {
            SnapshotEditEvent::RemoveValue { .. } => {
                let name = children.get(sheet_row(position)?).and_then(sheet_element_name).ok_or_else(|| unsupported(pointer, "only a sheet is removed"))?;
                Ok(Some(vec![XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: name.to_string() })]))
            }
            SnapshotEditEvent::InsertValue { value, .. } => {
                let physical = child_position(position, children.len(), pointer, true)?;
                let node = decode_node(value, pointer)?;
                let name = sheet_element_name(&node).ok_or_else(|| unsupported(pointer, "only a sheet with a name is inserted"))?;
                let index = children[..physical].iter().filter(|child| sheet_element_name(child).is_some()).count();
                let insert = insert_sheet::InsertSheet::minted(at.snapshot, XlsxSheet { name: name.to_string(), cells: Vec::new() }, Some(index)).map_err(|message| refusal(pointer, message))?;
                Ok(Some(vec![XlsxMutation::InsertSheet(insert)]))
            }
            _ => Ok(None),
        };
    }
    if at.path.len() != 2 {
        return Ok(None);
    }
    let sheet = node_at(at.root, &at.path, pointer)?;
    let Some(old) = sheet_element_name(sheet) else { return Ok(None) };
    let patched = edited_node(sheet, &at.prefix(&at.path), at.event, pointer)?;
    if patched == *sheet {
        return Ok(Some(Vec::new()));
    }
    let (XmlNode::Element { attrs: before, children: before_children, name: before_name }, XmlNode::Element { attrs: after, children: after_children, name: after_name }) = (sheet, &patched) else { return Ok(None) };
    let unchanged_apart_from_name = before_name == after_name && before_children == after_children && before.len() == after.len() && before.iter().zip(after).all(|(left, right)| left.name == right.name && (left.name == "name" || left.value == right.value));
    match (sheet_element_name(&patched), unchanged_apart_from_name) {
        (Some(new_name), true) => Ok(Some(vec![XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: old.to_string(), new_name: new_name.to_string() })])),
        _ => Err(unsupported(pointer, "a sheet entry is edited through its name only")),
    }
}

/// 🔢️ The list position `segment` names in a list of `length` rows (`-` appends when `insert`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn opc_position(segment: &str, length: usize, pointer: &str, insert: bool) -> Result<usize, SnapshotEditError> {
    let at = if insert && segment == "-" { length } else { segment.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("'{segment}' is no list position")))? };
    if at > length || (!insert && at == length) {
        return Err(SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("position {at} is outside the {length} rows")));
    }
    Ok(at)
}

/// 🔗️ The kinds of an edit below `/opc/relationships/<owner>/<i>`: inserting or removing a row writes or removes that relationship, and changing a field of a row writes
/// the changed relationship in place (a changed id removes the old relationship and writes the new one at the same position).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn relationship_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &XlsxSnapshot, tail: &[&str]) -> Result<Option<Vec<XlsxMutation>>, SnapshotEditError> {
    let [raw_owner, position, below @ ..] = tail else { return Ok(None) };
    let owner = raw_owner.replace("~1", "/").replace("~0", "~");
    let list = opc_layer::with_package(snapshot, |opc| opc.relationships.relationships(&owner).cloned()).map_err(|message| refusal(pointer, message))?.unwrap_or_default();
    match (below, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = opc_position(position, list.len(), pointer, true)?;
            let relationship = OpcRelationship::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))?;
            Ok(Some(vec![XlsxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &relationship, Some(at)))]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = opc_position(position, list.len(), pointer, false)?;
            Ok(Some(vec![XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner, id: list[at].id.clone() })]))
        }
        (_, SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::ReplaceSource { .. }) => Ok(None),
        _ => {
            let at = opc_position(position, list.len(), pointer, false)?;
            let current = &list[at];
            let prefix = format!("/opc/relationships/{raw_owner}/{at}");
            let next = OpcRelationship::from_value(edited_subtree(&current.to_value(), &prefix, event)?).map_err(|error| refusal(pointer, error.to_string()))?;
            if next == *current {
                return Ok(Some(Vec::new()));
            }
            if next.id == current.id {
                return Ok(Some(vec![XlsxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &next, None))]));
            }
            if list.iter().any(|existing| existing.id == next.id) {
                return Err(refusal(pointer, format!("relationship {:?} already exists", next.id)));
            }
            Ok(Some(vec![
                XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: owner.clone(), id: current.id.clone() }),
                XlsxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &next, Some(at))),
            ]))
        }
    }
}

/// 📇️ The kinds of an edit below `/opc/contentTypes/defaults|overrides/<i>`: inserting or removing a row writes or removes that entry, and changing a row writes the
/// changed entry in place (a changed name removes the old entry and writes the new one at the same position).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_type_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &XlsxSnapshot, tail: &[&str]) -> Result<Option<Vec<XlsxMutation>>, SnapshotEditError> {
    let [kind @ ("defaults" | "overrides"), position, below @ ..] = tail else { return Ok(None) };
    let is_override = *kind == "overrides";
    let list = opc_layer::with_package(snapshot, |opc| if is_override { opc.content_types.overrides.clone() } else { opc.content_types.defaults.clone() }).map_err(|message| refusal(pointer, message))?;
    match (below, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = opc_position(position, list.len(), pointer, true)?;
            let (name, content_type) = <(String, String)>::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))?;
            Ok(Some(vec![XlsxMutation::SetContentType(set_content_type::SetContentType { is_override, name, content_type, index: Some(at) })]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = opc_position(position, list.len(), pointer, false)?;
            Ok(Some(vec![XlsxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override, name: list[at].0.clone() })]))
        }
        (_, SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::ReplaceSource { .. }) => Ok(None),
        _ => {
            let at = opc_position(position, list.len(), pointer, false)?;
            let current = &list[at];
            let prefix = format!("/opc/contentTypes/{kind}/{at}");
            let next = <(String, String)>::from_value(edited_subtree(&current.to_value(), &prefix, event)?).map_err(|error| refusal(pointer, error.to_string()))?;
            if next == *current {
                return Ok(Some(Vec::new()));
            }
            if next.0 == current.0 {
                return Ok(Some(vec![XlsxMutation::SetContentType(set_content_type::SetContentType { is_override, name: next.0, content_type: next.1, index: None })]));
            }
            if list.iter().any(|(existing, _)| *existing == next.0) {
                return Err(refusal(pointer, format!("content type entry {:?} already exists", next.0)));
            }
            Ok(Some(vec![
                XlsxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override, name: current.0.clone() }),
                XlsxMutation::SetContentType(set_content_type::SetContentType { is_override, name: next.0, content_type: next.1, index: Some(at) }),
            ]))
        }
    }
}

/// 🎯 The kinds of an edit of a node below a part's root; `None` hands the edit on to the (empty) table.
pub fn special(event: &SnapshotEditEvent, snapshot: &XlsxSnapshot) -> Result<Option<Vec<XlsxMutation>>, SnapshotEditError> {
    let Some(pointer) = event_path(event) else { return Ok(None) };
    let segments: Vec<&str> = pointer.split('/').skip(1).collect();
    match segments.as_slice() {
        ["opc", "relationships", tail @ ..] => return relationship_edit(event, pointer, snapshot, tail),
        ["opc", "contentTypes", tail @ ..] => return content_type_edit(event, pointer, snapshot, tail),
        _ => {}
    }
    let ["xmlParts", row, "document", "root", tail @ ..] = segments.as_slice() else { return Ok(None) };
    let part_row: usize = row.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("'{row}' is no part position")))?;
    let part = snapshot.xml_parts.get(part_row).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("there is no part {part_row}")))?;
    let root = part.document.root.as_ref().ok_or_else(|| refusal(pointer, "the part has no root element"))?;
    let (path, rest) = locate(tail, pointer)?;
    let at = Located { event, pointer, snapshot, part_row, part_path: &part.path, root, path, rest };
    match part_role(snapshot, &part.path).map_err(|message| refusal(pointer, message))? {
        PartRole::Worksheet(sheet) => worksheet_edit(&at, &sheet),
        PartRole::SharedStrings => shared_strings_edit(&at),
        PartRole::Workbook => workbook_edit(&at),
        PartRole::Other => Ok(None),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
