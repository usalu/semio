//! 🧭️ The details-pane vocabulary of the PPTX editor: which JSON-pointer edit raises which concrete kind.
//!
//! The table [`EDIT_RULES`] is empty: a deck is edited through the nodes of its XML parts. A node below `/xmlParts/<n>/document/root` is addressed
//! by `children/<i>` pairs; replacing a child raises `replace-xml-node` on that child, any other edit inside an element (its name, attributes,
//! text, the insertion, removal or order of its children) raises `replace-xml-node` on the element, whose address carries its revision. The
//! parts of the package, its relationships and its content types have no kind and are refused.

use super::*;
use semio_framework_value::{FromValue, ToValue};
use semio_s_artifact_stdio_zip::opc::OpcRelationship;
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditError, SnapshotEditEvent};

/// 📚 No document field has a kind of its own: nodes are resolved by [`special`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

fn refusal(path: &str, message: impl Into<String>) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.schema-invalid", path, message)
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

fn node_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &PptxSnapshot, part_row: usize, tail: &[&str]) -> Result<Option<Vec<PptxMutation>>, SnapshotEditError> {
    let part = snapshot.xml_parts.get(part_row).ok_or_else(|| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("there is no part {part_row}")))?;
    let root = part.document.root.as_ref().ok_or_else(|| refusal(pointer, "the part has no root element"))?;
    let (mut path, rest) = locate(tail, pointer)?;
    let replaced_child = match (rest, event) {
        (["children", position], SnapshotEditEvent::SetValue { value, .. }) => {
            let at: usize = position.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("'{position}' is no child position")))?;
            path.push(at);
            Some(XmlNode::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))?)
        }
        _ => None,
    };
    let mut node = node_at(root, &path, pointer)?;
    if replaced_child.is_none() {
        while !matches!(node, XmlNode::Element { .. }) {
            path.pop();
            node = node_at(root, &path, pointer)?;
        }
    }
    let replacement = match replaced_child {
        Some(child) => child,
        None => {
            let prefix = format!("/xmlParts/{part_row}/document/root{}", path.iter().map(|index| format!("/children/{index}")).collect::<String>());
            let current = node.to_value();
            let next = edited_subtree(&current, &prefix, event)?;
            if next == current {
                return Ok(Some(Vec::new()));
            }
            XmlNode::from_value(next).map_err(|error| refusal(pointer, error.to_string()))?
        }
    };
    if *node == replacement {
        return Ok(Some(Vec::new()));
    }
    let address = xml_address::pptx_xml_address(snapshot, &part.path, path).map_err(|message| refusal(pointer, message))?;
    Ok(Some(vec![PptxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node: replacement })]))
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
fn relationship_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &PptxSnapshot, tail: &[&str]) -> Result<Option<Vec<PptxMutation>>, SnapshotEditError> {
    let [raw_owner, position, below @ ..] = tail else { return Ok(None) };
    let owner = raw_owner.replace("~1", "/").replace("~0", "~");
    let list = opc_layer::with_package(snapshot, |opc| opc.relationships.relationships(&owner).cloned()).map_err(|message| refusal(pointer, message))?.unwrap_or_default();
    match (below, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = opc_position(position, list.len(), pointer, true)?;
            let relationship = OpcRelationship::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))?;
            Ok(Some(vec![PptxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &relationship, Some(at)))]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = opc_position(position, list.len(), pointer, false)?;
            Ok(Some(vec![PptxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner, id: list[at].id.clone() })]))
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
                return Ok(Some(vec![PptxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &next, None))]));
            }
            if list.iter().any(|existing| existing.id == next.id) {
                return Err(refusal(pointer, format!("relationship {:?} already exists", next.id)));
            }
            Ok(Some(vec![
                PptxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: owner.clone(), id: current.id.clone() }),
                PptxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &next, Some(at))),
            ]))
        }
    }
}

/// 📇️ The kinds of an edit below `/opc/contentTypes/defaults|overrides/<i>`: inserting or removing a row writes or removes that entry, and changing a row writes the
/// changed entry in place (a changed name removes the old entry and writes the new one at the same position).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_type_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &PptxSnapshot, tail: &[&str]) -> Result<Option<Vec<PptxMutation>>, SnapshotEditError> {
    let [kind @ ("defaults" | "overrides"), position, below @ ..] = tail else { return Ok(None) };
    let is_override = *kind == "overrides";
    let list = opc_layer::with_package(snapshot, |opc| if is_override { opc.content_types.overrides.clone() } else { opc.content_types.defaults.clone() }).map_err(|message| refusal(pointer, message))?;
    match (below, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = opc_position(position, list.len(), pointer, true)?;
            let (name, content_type) = <(String, String)>::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))?;
            Ok(Some(vec![PptxMutation::SetContentType(set_content_type::SetContentType { is_override, name, content_type, index: Some(at) })]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = opc_position(position, list.len(), pointer, false)?;
            Ok(Some(vec![PptxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override, name: list[at].0.clone() })]))
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
                return Ok(Some(vec![PptxMutation::SetContentType(set_content_type::SetContentType { is_override, name: next.0, content_type: next.1, index: None })]));
            }
            if list.iter().any(|(existing, _)| *existing == next.0) {
                return Err(refusal(pointer, format!("content type entry {:?} already exists", next.0)));
            }
            Ok(Some(vec![
                PptxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override, name: current.0.clone() }),
                PptxMutation::SetContentType(set_content_type::SetContentType { is_override, name: next.0, content_type: next.1, index: Some(at) }),
            ]))
        }
    }
}

/// 🎯 The kinds of an edit of a node below a part's root; `None` hands the edit on to the (empty) table.
pub fn special(event: &SnapshotEditEvent, snapshot: &PptxSnapshot) -> Result<Option<Vec<PptxMutation>>, SnapshotEditError> {
    let Some(pointer) = event_path(event) else { return Ok(None) };
    match pointer.split('/').skip(1).collect::<Vec<_>>().as_slice() {
        ["xmlParts", row, "document", "root", tail @ ..] => {
            let part_row = row.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("'{row}' is no part position")))?;
            node_edit(event, pointer, snapshot, part_row, tail)
        }
        ["opc", "relationships", tail @ ..] => relationship_edit(event, pointer, snapshot, tail),
        ["opc", "contentTypes", tail @ ..] => content_type_edit(event, pointer, snapshot, tail),
        _ => Ok(None),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
