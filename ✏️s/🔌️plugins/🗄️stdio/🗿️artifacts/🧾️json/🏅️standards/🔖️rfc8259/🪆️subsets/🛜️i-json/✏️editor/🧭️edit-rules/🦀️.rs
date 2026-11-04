//! 🧭️ The details-pane vocabulary of the i-json editor: the same recursive value-tree addressing as the base json editor, whose locator it
//! reuses, raising the I-JSON verbs instead of the base ones: a member insert is `upsert-member`, a member remove `remove-member`, a key
//! rename the single `rename-member` (position kept, duplicate refused), an element insert and remove `insert-array-element` and
//! `remove-array-element`, a string write `set-string`, a number write `set-safe-number`, a root write `set-top-level`, and any other
//! whole-value write the member upsert or the element remove-then-insert that puts the value in place.

use crate::editor::json_any::edit_rules::{fail, index_of, locate, segments_of};
use crate::standards::v_rfc8259::subsets::base::schema::mutations::{JsonPath, JsonPathSegment};
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonMember, JsonSnapshot, JsonValue};
use crate::standards::v_rfc8259::subsets::i_json::schema::mutations::{insert_array_element, remove_array_element, remove_member, rename_member, set_safe_number, set_string, set_top_level, upsert_member, JsonIJsonMutation, JsonIJsonRoot};
use semio_framework_plugin::Fault;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, EditRules, SnapshotEditEvent};

/// 📚 No fixed pointer shape: see [`resolve`].
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

/// 🔁 The leaves that write `replacement` over the node at `at`, which currently holds `current`.
fn replace_node(at: JsonPath, current: &JsonValue, replacement: JsonValue) -> Result<Vec<JsonIJsonMutation>, Fault> {
    match (&replacement, current, at.split_last()) {
        (_, _, None) => {
            let root = JsonIJsonRoot::from_value(&replacement).ok_or_else(|| fail("an I-JSON document's top-level value is an object or an array"))?;
            Ok(vec![JsonIJsonMutation::SetTopLevel(set_top_level::SetTopLevel { root })])
        }
        (JsonValue::String { value }, JsonValue::String { .. }, _) => Ok(vec![JsonIJsonMutation::SetString(set_string::SetString { path: at.clone(), value: value.clone() })]),
        (JsonValue::Number { lexeme }, JsonValue::Number { .. }, _) => Ok(vec![JsonIJsonMutation::SetSafeNumber(set_safe_number::SetSafeNumber { path: at.clone(), lexeme: lexeme.clone() })]),
        (_, _, Some((JsonPathSegment::Key(key), parent))) => Ok(vec![JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: parent.to_vec(), key: key.clone(), value: replacement, index: None })]),
        (_, _, Some((JsonPathSegment::Index(index), parent))) => Ok(vec![
            JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: parent.to_vec(), index: *index }),
            JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: parent.to_vec(), index: *index, value: replacement }),
        ]),
    }
}

/// 🎯 The concrete kinds an edit of the i-json document denotes, or `None` when the pointer is not inside `/value`.
pub fn resolve(snapshot: &JsonSnapshot, event: &SnapshotEditEvent) -> Result<Option<Vec<JsonIJsonMutation>>, Fault> {
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
            vec![JsonIJsonMutation::UpsertMember(upsert_member::UpsertMember { path: at, key: member.key, value: member.value, index: Some(index) })]
        }
        (SnapshotEditEvent::InsertValue { value, .. }, JsonValue::Array { items }, [key, index]) if key == "items" => {
            let element = JsonValue::from_value(value.clone()).map_err(|error| fail(error.to_string()))?;
            vec![JsonIJsonMutation::InsertArrayElement(insert_array_element::InsertArrayElement { path: at, index: index_of(index, items.len(), true)?, value: element })]
        }
        (SnapshotEditEvent::RemoveValue { .. }, JsonValue::Object { members }, [key, index]) if key == "members" => {
            vec![JsonIJsonMutation::RemoveMember(remove_member::RemoveMember { path: at, key: members[index_of(index, members.len(), false)?].key.clone() })]
        }
        (SnapshotEditEvent::RemoveValue { .. }, JsonValue::Array { items }, [key, index]) if key == "items" => {
            vec![JsonIJsonMutation::RemoveArrayElement(remove_array_element::RemoveArrayElement { path: at, index: index_of(index, items.len(), false)? })]
        }
        (SnapshotEditEvent::SetValue { value: DslValue::String(renamed), .. }, JsonValue::Object { members }, [key, index, member_key]) if key == "members" && member_key == "key" => {
            let member = &members[index_of(index, members.len(), false)?];
            if &member.key == renamed {
                Vec::new()
            } else {
                vec![JsonIJsonMutation::RenameMember(rename_member::RenameMember { path: at, from: member.key.clone(), to: renamed.clone() })]
            }
        }
        _ => {
            let edited = edited_subtree(&located.node.to_value(), &located.pointer, event).map_err(|error| fail(error.to_string()))?;
            let replacement = JsonValue::from_value(edited).map_err(|error| fail(error.to_string()))?;
            if &replacement == located.node {
                Vec::new()
            } else {
                replace_node(at, located.node, replacement)?
            }
        }
    };
    Ok(Some(leaves))
}
