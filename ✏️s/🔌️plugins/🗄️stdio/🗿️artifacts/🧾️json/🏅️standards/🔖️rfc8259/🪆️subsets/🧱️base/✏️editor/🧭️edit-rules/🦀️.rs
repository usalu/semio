//! 🧭️ The details-pane vocabulary of the json editors: a snapshot pointer walks the value tree (`/value`, then `members/<i>/value` or
//! `items/<i>` per level) to the node it addresses and raises the one kind of that node: a member or element insert and remove at its
//! container, a key rename as a remove then a set at the same position, and every other edit as `set-scalar` of the node's whole new value.
//! The table itself is empty because the kinds are addressed by a recursive path, not by a fixed pointer shape.

use crate::schema::mutations::{InsertArrayElementPayload, JsonMutation, JsonPath, JsonPathSegment, RemoveArrayElementPayload, RemoveMemberPayload, SetMemberPayload, SetScalarPayload};
use crate::schema::snapshot::{JsonMember, JsonSnapshot, JsonValue};
use semio_framework_plugin::Fault;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditEvent};

/// 📚 No fixed pointer shape: see [`resolve`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

fn fail(message: impl Into<String>) -> Fault {
    Fault::from(message.into())
}

fn segments_of(pointer: &str) -> Result<Vec<String>, Fault> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    pointer.strip_prefix('/').map(|rest| rest.split('/').map(|raw| raw.replace("~1", "/").replace("~0", "~")).collect()).ok_or_else(|| fail(format!("'{pointer}' is not a pointer")))
}

fn index_of(segment: &str, length: usize, insertion: bool) -> Result<usize, Fault> {
    let index = if insertion && segment == "-" { length } else { segment.parse::<usize>().map_err(|error| fail(error.to_string()))? };
    (index < length || (insertion && index == length)).then_some(index).ok_or_else(|| fail(format!("index {index} is outside 0..{length}")))
}

struct Located<'a> {
    path: Vec<JsonPathSegment>,
    node: &'a JsonValue,
    pointer: String,
    consumed: usize,
}

fn locate<'a>(root: &'a JsonValue, segments: &[String], container_edit: bool) -> Result<Located<'a>, Fault> {
    let mut located = Located { path: Vec::new(), node: root, pointer: "/value".to_string(), consumed: 0 };
    loop {
        let rest = &segments[located.consumed..];
        match (located.node, rest) {
            (JsonValue::Object { members }, [members_key, index, value_key, ..]) if members_key == "members" && value_key == "value" => {
                let member = &members[index_of(index, members.len(), false)?];
                located.path.push(JsonPathSegment::Key(member.key.clone()));
                located.pointer = format!("{}/members/{index}/value", located.pointer);
                located.node = &member.value;
                located.consumed += 3;
            }
            (JsonValue::Array { items }, [items_key, index, ..]) if items_key == "items" && !(container_edit && rest.len() == 2) => {
                located.path.push(JsonPathSegment::Index(index_of(index, items.len(), false)?));
                located.pointer = format!("{}/items/{index}", located.pointer);
                located.node = &items[index_of(index, items.len(), false)?];
                located.consumed += 2;
            }
            _ => return Ok(located),
        }
    }
}

/// 🎯 The concrete kinds an edit of the json document denotes, or `None` when the pointer is not inside `/value`.
pub fn resolve(snapshot: &JsonSnapshot, event: &SnapshotEditEvent) -> Result<Option<Vec<JsonMutation>>, Fault> {
    let (path, container_edit) = match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::RenameKey { path, .. } | SnapshotEditEvent::MoveValue { path, .. } => (path, false),
        SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } => (path, true),
        SnapshotEditEvent::ReplaceSource { .. } => return Ok(None),
    };
    let segments = segments_of(path)?;
    if segments.first().map(String::as_str) != Some("value") {
        return Ok(None);
    }
    let located = locate(&snapshot.value, &segments[1..], container_edit)?;
    let rest = &segments[1 + located.consumed..];
    let at: JsonPath = located.path.clone();
    let leaves = match (event, located.node, rest) {
        (SnapshotEditEvent::InsertValue { value, .. }, JsonValue::Object { members }, [key, index]) if key == "members" => {
            let member = JsonMember::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
            let index = index_of(index, members.len(), true)?;
            vec![JsonMutation::SetMember(SetMemberPayload { path: at, key: member.key, value: member.value, index: Some(index) })]
        }
        (SnapshotEditEvent::InsertValue { value, .. }, JsonValue::Array { items }, [key, index]) if key == "items" => {
            let element = JsonValue::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
            vec![JsonMutation::InsertArrayElement(InsertArrayElementPayload { path: at, index: index_of(index, items.len(), true)?, value: element })]
        }
        (SnapshotEditEvent::RemoveValue { .. }, JsonValue::Object { members }, [key, index]) if key == "members" => {
            vec![JsonMutation::RemoveMember(RemoveMemberPayload { path: at, key: members[index_of(index, members.len(), false)?].key.clone() })]
        }
        (SnapshotEditEvent::RemoveValue { .. }, JsonValue::Array { items }, [key, index]) if key == "items" => {
            vec![JsonMutation::RemoveArrayElement(RemoveArrayElementPayload { path: at, index: index_of(index, items.len(), false)? })]
        }
        (SnapshotEditEvent::SetValue { value: DslValue::String(renamed), .. }, JsonValue::Object { members }, [key, index, member_key]) if key == "members" && member_key == "key" => {
            let index = index_of(index, members.len(), false)?;
            let member = &members[index];
            vec![
                JsonMutation::RemoveMember(RemoveMemberPayload { path: at.clone(), key: member.key.clone() }),
                JsonMutation::SetMember(SetMemberPayload { path: at, key: renamed.clone(), value: member.value.clone(), index: Some(index) }),
            ]
        }
        _ => {
            let edited = edited_subtree(&located.node.to_value(), &located.pointer, event).map_err(|error| fail(error.to_string()))?;
            let replacement = JsonValue::from_value(edited).map_err(|error| fail(error.to_string()))?;
            if &replacement == located.node {
                Vec::new()
            } else {
                vec![JsonMutation::SetScalar(SetScalarPayload { path: at, value: replacement })]
            }
        }
    };
    Ok(Some(leaves))
}
