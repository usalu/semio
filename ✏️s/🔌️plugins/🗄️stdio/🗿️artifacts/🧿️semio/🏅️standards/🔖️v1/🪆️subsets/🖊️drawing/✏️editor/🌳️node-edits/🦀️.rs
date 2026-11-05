//! 🌳️ Scene-graph node edits of the drawing editor: a pointer below `/layers/<n>/root` names one node by its child indices, and the gesture resolves to the node mutation kind that expresses it. Anything else falls through to the rules table.

use crate::schema::diff::NodePath;
use crate::schema::mutations::{create_node::CreateNode, delete_node::DeleteNode, move_node::MoveNode, replace_path::ReplacePath, SemioDrawingMutation};
use crate::schema::snapshot::{DrawNode, PathSegment, SemioDrawingSnapshot};
use semio_plugin::Fault;
use semio_framework_value::FromValue;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
use semio_s_artifact_stdio_semio_base::editor::edit_plumbing::{event_path, unsupported};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;

struct Addressed<'a> {
    at: NodePath,
    node: &'a DrawNode,
    rest: Vec<String>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn index(segment: &str, path: &str) -> Result<usize, Fault> {
    let canonical = !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit()) && (segment == "0" || !segment.starts_with('0'));
    canonical.then(|| segment.parse::<usize>().ok()).flatten().ok_or_else(|| unsupported(path, format!("'{segment}' is not a canonical list index")))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn segments(path: &str) -> Result<Vec<String>, Fault> {
    let rest = path.strip_prefix('/').ok_or_else(|| unsupported(path, "a pointer starts with '/'"))?;
    rest.split('/')
        .map(|segment| {
            let mut out = String::new();
            let mut characters = segment.chars();
            while let Some(character) = characters.next() {
                match (character, character == '~') {
                    (_, false) => out.push(character),
                    _ => match characters.next() {
                        Some('0') => out.push('~'),
                        Some('1') => out.push('/'),
                        _ => return Err(unsupported(path, "'~' must be followed by '0' or '1'")),
                    },
                }
            }
            (!out.is_empty()).then_some(out).ok_or_else(|| unsupported(path, "a pointer has no empty segment"))
        })
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn address<'a>(snapshot: &'a SemioDrawingSnapshot, path: &str) -> Result<Option<Addressed<'a>>, Fault> {
    let parts = segments(path)?;
    if parts.len() < 3 || parts[0] != "layers" || parts[2] != "root" {
        return Ok(None);
    }
    let layer = index(&parts[1], path)?;
    let mut node = &snapshot.layers.get(layer).ok_or_else(|| unsupported(path, "the layer does not exist"))?.root;
    let mut indices = Vec::new();
    let mut cursor = 3;
    while cursor + 1 < parts.len() && parts[cursor] == "children" {
        let DrawNode::Group { children, .. } = node else { return Err(unsupported(path, "only a group has children")) };
        let child = index(&parts[cursor + 1], path)?;
        node = children.get(child).ok_or_else(|| unsupported(path, "the child does not exist"))?;
        indices.push(child);
        cursor += 2;
    }
    Ok(Some(Addressed { at: NodePath { layer, path: indices }, node, rest: parts[cursor..].to_vec() }))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value<T: FromValue>(value: &semio_framework_value::DslValue, path: &str) -> Result<T, Fault> {
    T::from_value(value.clone()).map_err(|error| unsupported(path, error.to_string()))
}

/// 🌳️ Resolves insert, remove, move and path replacement of scene-graph nodes; `None` leaves the event to the rules table.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn special(event: &SnapshotEditEvent, snapshot: &SemioDrawingSnapshot) -> Result<Option<Vec<SemioDrawingMutation>>, Fault> {
    let path = event_path(event);
    match event {
        SnapshotEditEvent::InsertValue { value: inserted, .. } => {
            let mut parts = segments(path)?;
            let Some(slot) = parts.pop() else { return Ok(None) };
            if slot == "children" {
                return Err(unsupported(path, "children needs an index or '-'"));
            }
            let parent_path = format!("/{}", parts.join("/"));
            if parts.last().map(String::as_str) != Some("children") {
                return Ok(None);
            }
            parts.pop();
            let owner = format!("/{}", parts.join("/"));
            let Some(parent) = address(snapshot, &owner)?.filter(|parent| parent.rest.is_empty()) else { return Ok(None) };
            let DrawNode::Group { children, .. } = parent.node else { return Err(unsupported(&parent_path, "only a group has children")) };
            let at = if slot == "-" { children.len() } else { index(&slot, path)? };
            if at > children.len() {
                return Err(unsupported(path, "the position is past the last child"));
            }
            Ok(Some(vec![SemioDrawingMutation::CreateNode(CreateNode { parent: parent.at, index: at, node: value(inserted, path)? })]))
        }
        SnapshotEditEvent::RemoveValue { .. } => {
            let Some(target) = address(snapshot, path)? else { return Ok(None) };
            if target.rest == ["children"] {
                return Err(unsupported(path, "children needs an index"));
            }
            if !target.rest.is_empty() {
                return Ok(None);
            }
            if target.at.path.is_empty() {
                return Err(unsupported(path, "a layer's root group is replaced by deleting the layer"));
            }
            Ok(Some(vec![SemioDrawingMutation::DeleteNode(DeleteNode { at: target.at })]))
        }
        SnapshotEditEvent::SetValue { value: set, .. } => {
            let Some(target) = address(snapshot, path)? else { return Ok(None) };
            match (target.rest.as_slice(), target.node) {
                ([field], DrawNode::Text { .. } | DrawNode::Image { .. }) if field == "at" => Ok(Some(vec![SemioDrawingMutation::MoveNode(MoveNode { at: target.at, new_origin: value::<SemioPoint2>(set, path)? })])),
                ([field], DrawNode::Path { .. }) if field == "segments" => Ok(Some(vec![SemioDrawingMutation::ReplacePath(ReplacePath { at: target.at, new_segments: value::<Vec<PathSegment>>(set, path)? })])),
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}
