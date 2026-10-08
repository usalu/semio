//! 🧭️ The value editor's edit rules: which snapshot pointer raises which ONE concrete value mutation kind. An edit no kind expresses is refused, never approximated.

use crate::editor::semio_base::edit_plumbing::{build, edited, ent, event_path, field, item, keyed, named, position, record, rem, resolve, segments, unsupported, Reshape};
use crate::standards::v1::subsets::value::schema::mutations::SemioValueMutation;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use semio_framework_plugin::Fault;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{EditRules, SnapshotEditEvent};

/// 📚 Every pointer outside the root tree a value editor edit resolves, one rule per kind that sets or removes through it.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[ent("/nodes/*/value", "set-node", &[named("id", "id")], "value")],
    inserts: &[],
    removes: &[rem("/nodes", "remove-node", &[], keyed("id", "id"))],
};

struct Reached<'a> {
    trail: Vec<DslValue>,
    node: &'a DslValue,
    at: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn kind_of(node: &DslValue) -> Option<&str> {
    match field(node, "kind") {
        Some(DslValue::String(kind)) => Some(kind),
        _ => None,
    }
}

/// 🌱 Walks the pointer from the root through map entries and list items; stops at the node that owns the rest of the pointer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reach<'a>(root: &'a DslValue, segs: &[String]) -> Reached<'a> {
    let (mut trail, mut node, mut at) = (Vec::new(), root, 1);
    loop {
        let step = match kind_of(node) {
            Some("map") if segs.get(at).map(String::as_str) == Some("entries") && segs.get(at + 2).map(String::as_str) == Some("value") => {
                let entry = segs.get(at + 1).and_then(|segment| segment.parse::<usize>().ok()).and_then(|index| field(node, "entries").and_then(|entries| item(entries, index)));
                entry.and_then(|entry| Some((record([("kind", DslValue::String("key".to_string())), ("key", field(entry, "key")?.clone())]), field(entry, "value")?, 3)))
            }
            Some("list") if segs.get(at).map(String::as_str) == Some("items") && segs.len() > at + 2 => {
                let index = segs.get(at + 1).and_then(|segment| segment.parse::<usize>().ok());
                index.and_then(|index| Some((record([("kind", DslValue::String("index".to_string())), ("index", DslValue::uint(index as u64))]), item(field(node, "items")?, index)?, 2)))
            }
            _ => None,
        };
        let Some((segment, child, consumed)) = step else { return Reached { trail, node, at } };
        trail.push(segment);
        node = child;
        at += consumed;
    }
}

/// 🌱 Edits of the root tree resolve to the value kind they address: a list item or map entry is inserted or removed, a node is set; every other edit inside a node sets that node. Everything else goes through [`EDIT_RULES`].
pub(crate) fn special(event: &SnapshotEditEvent, snapshot: &SemioValueSnapshot) -> Result<Option<Vec<SemioValueMutation>>, Fault> {
    let path = event_path(event);
    let segs = segments(path)?;
    let tree = snapshot.to_value();
    match segs.first().map(String::as_str) {
        Some("nodes") => return nodes(event, &segs, snapshot, &tree),
        Some("root") => {}
        _ => return resolve::<SemioValueSnapshot, SemioValueMutation>(&EDIT_RULES, &[] as &[(&str, Reshape)], snapshot, event).map(Some),
    }
    let root = field(&tree, "root").ok_or_else(|| unsupported(path, "the document has no root"))?;
    let reached = reach(root, &segs);
    let location = DslValue::Array(reached.trail.clone());
    let make = |kind: &str, members: Vec<(&str, DslValue)>| build::<SemioValueSnapshot, SemioValueMutation>(&tree, &[], kind, members.into_iter().map(|(name, value)| (name.to_string(), value)).collect(), path).map(Some);
    let rest: Vec<&str> = segs[reached.at..].iter().map(String::as_str).collect();
    match (event, kind_of(reached.node), rest.as_slice()) {
        (SnapshotEditEvent::InsertValue { value, .. }, Some("list"), ["items", at]) => {
            let length = field(reached.node, "items").map_or(0, |items| match items {
                DslValue::Array(rows) => rows.len(),
                _ => 0,
            });
            make("insert-list-item", vec![("path", location), ("index", DslValue::uint(position(at, Some(length), path)? as u64)), ("value", value.clone())])
        }
        (SnapshotEditEvent::RemoveValue { .. }, Some("list"), ["items", at]) => make("remove-list-item", vec![("path", location), ("index", DslValue::uint(position(at, None, path)? as u64))]),
        (SnapshotEditEvent::InsertValue { value, .. }, Some("map"), ["entries", at]) => {
            let length = match field(reached.node, "entries") {
                Some(DslValue::Array(rows)) => rows.len(),
                _ => 0,
            };
            let (Some(key), Some(entry)) = (field(value, "key"), field(value, "value")) else { return Err(unsupported(path, "a map entry is a record with a key and a value")) };
            make("set-map-entry", vec![("path", location), ("key", key.clone()), ("value", entry.clone()), ("at", DslValue::uint(position(at, Some(length), path)? as u64))])
        }
        (SnapshotEditEvent::RemoveValue { .. }, Some("map"), ["entries", at]) => {
            let entry = field(reached.node, "entries").and_then(|entries| item(entries, position(at, None, path).ok()?)).ok_or_else(|| unsupported(path, "the map has no such entry"))?;
            make("remove-map-entry", vec![("path", location), ("key", field(entry, "key").cloned().unwrap_or(DslValue::Null))])
        }
        _ => {
            let next = edited(reached.node, event, reached.at)?;
            if &next == reached.node {
                return Ok(Some(Vec::new()));
            }
            if rest.is_empty() && !matches!(event, SnapshotEditEvent::SetValue { .. }) {
                return Err(unsupported(path, "no kind removes or moves a whole value"));
            }
            make("set-value", vec![("path", location), ("value", next)])
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nodes(event: &SnapshotEditEvent, segs: &[String], snapshot: &SemioValueSnapshot, tree: &DslValue) -> Result<Option<Vec<SemioValueMutation>>, Fault> {
    let path = event_path(event);
    match (event, segs) {
        (SnapshotEditEvent::InsertValue { value, .. }, [_, at]) => {
            let length = match field(tree, "nodes") {
                Some(DslValue::Array(rows)) => rows.len(),
                _ => 0,
            };
            let (Some(id), Some(node)) = (field(value, "id"), field(value, "value")) else { return Err(unsupported(path, "a node is a record with an id and a value")) };
            let members = vec![("id".to_string(), id.clone()), ("value".to_string(), node.clone()), ("at".to_string(), DslValue::uint(position(at, Some(length), path)? as u64))];
            build::<SemioValueSnapshot, SemioValueMutation>(tree, &[], "set-node", members, path).map(Some)
        }
        _ => resolve::<SemioValueSnapshot, SemioValueMutation>(&EDIT_RULES, &[] as &[(&str, Reshape)], snapshot, event).map(Some),
    }
}
