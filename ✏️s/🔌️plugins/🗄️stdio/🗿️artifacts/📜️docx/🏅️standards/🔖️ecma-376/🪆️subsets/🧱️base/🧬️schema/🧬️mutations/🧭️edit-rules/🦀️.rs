//! 🧭️ The details-pane vocabulary of the DOCX editor: which JSON-pointer edit raises which concrete kind.
//!
//! The table [`EDIT_RULES`] is empty: every pointer of a package is addressed by its part or its XML node, which [`special`] resolves.
//! - A node below `/xmlParts/<n>/document/root` is addressed by `children/<i>` pairs. Inserting or removing a child raises `insert-xml-node` /
//!   `remove-xml-node` under the parent's address; any other edit inside an element (its name, attributes, text, the order of its children) or a
//!   replaced child raises `replace-xml-node` on the addressed element. The part's document is never replaced as a whole.
//! - A row of `/xmlParts` or `/opc/parts` inserts or removes a part (`set-part` at that position, `remove-part`); a part's content type or binary
//!   payload is set through `set-part`.

use super::*;
use semio_framework_value::{FromValue, ToValue};
use semio_s_artifact_stdio_zip::opc::OpcRelationship;
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditError, SnapshotEditEvent};
use semio_s_artifact_stdio_zip::opc::OpcPart;

/// 📚 No document field has a kind of its own: parts and nodes are resolved by [`special`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

fn refusal(path: &str, message: impl Into<String>) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.schema-invalid", path, message)
}

fn faulted(path: &str, error: ValueError) -> SnapshotEditError {
    refusal(path, error.to_string())
}

fn segments(path: &str) -> Vec<&str> {
    path.split('/').skip(1).collect()
}

fn row(segment: &str, length: usize, path: &str, insert: bool) -> Result<usize, SnapshotEditError> {
    let at = if insert && segment == "-" { length } else { segment.parse().map_err(|_| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("'{segment}' is no list position")))? };
    if at > length || (!insert && at == length) {
        return Err(SnapshotEditError::new("snapshot-edit.index-out-of-bounds", path, format!("position {at} is outside the {length} rows")));
    }
    Ok(at)
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

fn xml_node_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &DocxSnapshot, part_row: usize, part: &DocxXmlPart, tail: &[&str]) -> Result<Option<Vec<DocxMutation>>, SnapshotEditError> {
    let document = part.materialize_document_exact().map_err(|error| faulted(pointer, error))?;
    let root = document.root.as_ref().ok_or_else(|| refusal(pointer, "the part has no root element"))?;
    let (path, rest) = locate(tail, pointer)?;
    let node = node_at(root, &path, pointer)?;
    let address = |node_path: Vec<usize>| docx_xml_address(snapshot, &part.path, node_path).map_err(|error| faulted(pointer, error));
    if let ["children", position] = rest {
        let XmlNode::Element { children, .. } = node else { return Err(refusal(pointer, "the addressed node is not an element")) };
        match event {
            SnapshotEditEvent::InsertValue { value, .. } => {
                let at = row(position, children.len(), pointer, true)?;
                let child = XmlNode::from_value(value.clone()).map_err(|error| faulted(pointer, error))?;
                return Ok(Some(vec![DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: address(path)?, index: at, node: child })]));
            }
            SnapshotEditEvent::RemoveValue { .. } => {
                let at = row(position, children.len(), pointer, false)?;
                let child = address([path.as_slice(), &[at]].concat())?;
                return Ok(Some(vec![DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent: address(path)?, index: at, expected_name: child.expected_name, revision: child.revision })]));
            }
            SnapshotEditEvent::SetValue { value, .. } => {
                let at = row(position, children.len(), pointer, false)?;
                let child = XmlNode::from_value(value.clone()).map_err(|error| faulted(pointer, error))?;
                if children[at] == child {
                    return Ok(Some(Vec::new()));
                }
                return Ok(Some(vec![DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: address([path.as_slice(), &[at]].concat())?, node: child })]));
            }
            SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::RenameKey { .. } | SnapshotEditEvent::ReplaceSource { .. } => {}
        }
    }
    let prefix = format!("/xmlParts/{part_row}/document/root{}", path.iter().map(|index| format!("/children/{index}")).collect::<String>());
    let current = node.to_value();
    let next = edited_subtree(&current, &prefix, event)?;
    if next == current {
        return Ok(Some(Vec::new()));
    }
    let replacement = XmlNode::from_value(next).map_err(|error| faulted(pointer, error))?;
    Ok(Some(vec![DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: address(path)?, node: replacement })]))
}

fn xml_part_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &DocxSnapshot, parts: &[&str]) -> Result<Option<Vec<DocxMutation>>, SnapshotEditError> {
    let Some((position, rest)) = parts.split_first() else { return Ok(None) };
    let length = snapshot.xml_parts.iter().count();
    match (rest, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = row(position, length, pointer, true)?;
            let part = DocxXmlPart::from_value(value.clone()).map_err(|error| faulted(pointer, error))?;
            let document = part.materialize_document_exact().map_err(|error| faulted(pointer, error))?;
            Ok(Some(vec![DocxMutation::SetPart(set_part::SetPart { path: part.path, content_type: part.content_type, payload: set_part::DocxPartContent::Xml { document }, index: Some(at), override_index: None })]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = row(position, length, pointer, false)?;
            let part = snapshot.xml_parts.iter().nth(at).ok_or_else(|| refusal(pointer, "the part is missing"))?;
            Ok(Some(vec![DocxMutation::RemovePart(remove_part::RemovePart { path: part.path.clone() })]))
        }
        (["contentType"], SnapshotEditEvent::SetValue { value, .. }) => {
            let at = row(position, length, pointer, false)?;
            let part = snapshot.xml_parts.iter().nth(at).ok_or_else(|| refusal(pointer, "the part is missing"))?;
            let content_type = String::from_value(value.clone()).map_err(|error| faulted(pointer, error))?;
            if content_type == part.content_type {
                return Ok(Some(Vec::new()));
            }
            let document = part.materialize_document_exact().map_err(|error| faulted(pointer, error))?;
            Ok(Some(vec![DocxMutation::SetPart(set_part::SetPart { path: part.path.clone(), content_type, payload: set_part::DocxPartContent::Xml { document }, index: None, override_index: None })]))
        }
        (["document", "root", tail @ ..], _) => {
            let at = row(position, length, pointer, false)?;
            let part = snapshot.xml_parts.iter().nth(at).ok_or_else(|| refusal(pointer, "the part is missing"))?;
            xml_node_edit(event, pointer, snapshot, at, part, tail)
        }
        _ => Ok(None),
    }
}

fn binary_part_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &DocxSnapshot, parts: &[&str]) -> Result<Option<Vec<DocxMutation>>, SnapshotEditError> {
    let Some((position, rest)) = parts.split_first() else { return Ok(None) };
    let package = snapshot.opc.materialize_package_exact().map_err(|error| faulted(pointer, error))?;
    match (rest, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = row(position, package.parts.len(), pointer, true)?;
            let part = OpcPart::from_value(value.clone()).map_err(|error| faulted(pointer, error))?;
            Ok(Some(vec![DocxMutation::SetPart(set_part::SetPart { path: part.path, content_type: part.content_type, payload: set_part::DocxPartContent::Binary { bytes: part.bytes }, index: Some(at), override_index: None })]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = row(position, package.parts.len(), pointer, false)?;
            Ok(Some(vec![DocxMutation::RemovePart(remove_part::RemovePart { path: package.parts[at].path.clone() })]))
        }
        (["contentType" | "bytes", ..], _) => {
            let at = row(position, package.parts.len(), pointer, false)?;
            let current = &package.parts[at];
            let next = OpcPart::from_value(edited_subtree(&current.to_value(), &format!("/opc/parts/{at}"), event)?).map_err(|error| faulted(pointer, error))?;
            if next == *current {
                return Ok(Some(Vec::new()));
            }
            Ok(Some(vec![DocxMutation::SetPart(set_part::SetPart { path: next.path, content_type: next.content_type, payload: set_part::DocxPartContent::Binary { bytes: next.bytes }, index: None, override_index: None })]))
        }
        _ => Ok(None),
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
fn relationship_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &DocxSnapshot, tail: &[&str]) -> Result<Option<Vec<DocxMutation>>, SnapshotEditError> {
    let [raw_owner, position, below @ ..] = tail else { return Ok(None) };
    let owner = raw_owner.replace("~1", "/").replace("~0", "~");
    let list = opc_layer::with_package(snapshot, |opc| opc.relationships.relationships(&owner).cloned()).map_err(|message| refusal(pointer, message))?.unwrap_or_default();
    match (below, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = opc_position(position, list.len(), pointer, true)?;
            let relationship = OpcRelationship::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))?;
            Ok(Some(vec![DocxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &relationship, Some(at)))]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = opc_position(position, list.len(), pointer, false)?;
            Ok(Some(vec![DocxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner, id: list[at].id.clone() })]))
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
                return Ok(Some(vec![DocxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &next, None))]));
            }
            if list.iter().any(|existing| existing.id == next.id) {
                return Err(refusal(pointer, format!("relationship {:?} already exists", next.id)));
            }
            Ok(Some(vec![
                DocxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: owner.clone(), id: current.id.clone() }),
                DocxMutation::SetRelationship(set_relationship::SetRelationship::of(&owner, &next, Some(at))),
            ]))
        }
    }
}

/// 📇️ The kinds of an edit below `/opc/contentTypes/defaults|overrides/<i>`: inserting or removing a row writes or removes that entry, and changing a row writes the
/// changed entry in place (a changed name removes the old entry and writes the new one at the same position).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_type_edit(event: &SnapshotEditEvent, pointer: &str, snapshot: &DocxSnapshot, tail: &[&str]) -> Result<Option<Vec<DocxMutation>>, SnapshotEditError> {
    let [kind @ ("defaults" | "overrides"), position, below @ ..] = tail else { return Ok(None) };
    let is_override = *kind == "overrides";
    let list = opc_layer::with_package(snapshot, |opc| if is_override { opc.content_types.overrides.clone() } else { opc.content_types.defaults.clone() }).map_err(|message| refusal(pointer, message))?;
    match (below, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = opc_position(position, list.len(), pointer, true)?;
            let (name, content_type) = <(String, String)>::from_value(value.clone()).map_err(|error| refusal(pointer, error.to_string()))?;
            Ok(Some(vec![DocxMutation::SetContentType(set_content_type::SetContentType { is_override, name, content_type, index: Some(at) })]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => {
            let at = opc_position(position, list.len(), pointer, false)?;
            Ok(Some(vec![DocxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override, name: list[at].0.clone() })]))
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
                return Ok(Some(vec![DocxMutation::SetContentType(set_content_type::SetContentType { is_override, name: next.0, content_type: next.1, index: None })]));
            }
            if list.iter().any(|(existing, _)| *existing == next.0) {
                return Err(refusal(pointer, format!("content type entry {:?} already exists", next.0)));
            }
            Ok(Some(vec![
                DocxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override, name: current.0.clone() }),
                DocxMutation::SetContentType(set_content_type::SetContentType { is_override, name: next.0, content_type: next.1, index: Some(at) }),
            ]))
        }
    }
}

/// 🎯 The kinds of an edit of a part or of a node below a part's root; `None` hands the edit on to the (empty) table.
pub fn special(event: &SnapshotEditEvent, snapshot: &DocxSnapshot) -> Result<Option<Vec<DocxMutation>>, SnapshotEditError> {
    let Some(pointer) = event_path(event) else { return Ok(None) };
    match segments(pointer).as_slice() {
        ["xmlParts", parts @ ..] => xml_part_edit(event, pointer, snapshot, parts),
        ["opc", "parts", parts @ ..] => binary_part_edit(event, pointer, snapshot, parts),
        ["opc", "relationships", tail @ ..] => relationship_edit(event, pointer, snapshot, tail),
        ["opc", "contentTypes", tail @ ..] => content_type_edit(event, pointer, snapshot, tail),
        _ => Ok(None),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
