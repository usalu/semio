//! 🧬️ Transparent JsonMutation aggregate.
use crate::schema::diff::JsonDiff;
use crate::schema::snapshot::JsonValue;
use crate::JsonSnapshot;


pub use super::insert_array_element::{InsertArrayElementMutation, InsertArrayElementPayload};
pub use super::remove_array_element::{RemoveArrayElementMutation, RemoveArrayElementPayload};
pub use super::remove_member::{RemoveMemberMutation, RemoveMemberPayload};
pub use super::set_member::{SetMemberMutation, SetMemberPayload};
pub use super::set_scalar::{SetScalarMutation, SetScalarPayload};
pub use crate::schema::mutation_support::{JsonPath, JsonPathSegment};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = JsonSnapshot, diff = JsonDiff, schema = "s.stdio.json")]
pub enum JsonMutation {
    SetMember(SetMemberMutation),
    RemoveMember(RemoveMemberMutation),
    InsertArrayElement(InsertArrayElementMutation),
    RemoveArrayElement(RemoveArrayElementMutation),
    SetScalar(SetScalarMutation),
}

pub fn apply_json_mutation(snapshot: &mut JsonSnapshot, mutation: &JsonMutation) -> protocol::MutationOutcome<JsonDiff> {
    let outcome = <JsonMutation as protocol::Mutation<JsonSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to `next`: the value tree walked in place — an object keeps the members it shares (removed ones
/// by key, new ones inserted at their final position), an array pairs items by index with its diverging tails removed or inserted,
/// and any scalar or kind change is a `set-scalar` at that path. A pure member reorder has no leaf and is refused by the exact net.
pub fn net_mutations(base: &JsonSnapshot, next: &JsonSnapshot) -> Vec<JsonMutation> {
    let mut leaves = Vec::new();
    net_value(&mut Vec::new(), &base.value, &next.value, &mut leaves);
    leaves
}

fn net_value(path: &mut Vec<JsonPathSegment>, before: &JsonValue, after: &JsonValue, leaves: &mut Vec<JsonMutation>) {
    if before == after {
        return;
    }
    match (before, after) {
        (JsonValue::Object { members: old }, JsonValue::Object { members: new }) => {
            leaves.extend(old.iter().filter(|member| !new.iter().any(|kept| kept.key == member.key)).map(|member| JsonMutation::RemoveMember(RemoveMemberPayload { path: path.clone(), key: member.key.clone() })));
            for (index, member) in new.iter().enumerate() {
                match old.iter().find(|existing| existing.key == member.key) {
                    Some(existing) => {
                        path.push(JsonPathSegment::Key(member.key.clone()));
                        net_value(path, &existing.value, &member.value, leaves);
                        path.pop();
                    }
                    None => leaves.push(JsonMutation::SetMember(SetMemberPayload { path: path.clone(), key: member.key.clone(), value: member.value.clone(), index: Some(index) })),
                }
            }
        }
        (JsonValue::Array { items: old }, JsonValue::Array { items: new }) => {
            let paired = old.len().min(new.len());
            for (index, (before, after)) in old.iter().zip(new).enumerate() {
                path.push(JsonPathSegment::Index(index));
                net_value(path, before, after, leaves);
                path.pop();
            }
            leaves.extend((paired..old.len()).rev().map(|index| JsonMutation::RemoveArrayElement(RemoveArrayElementPayload { path: path.clone(), index })));
            leaves.extend(new.iter().enumerate().skip(paired).map(|(index, value)| JsonMutation::InsertArrayElement(InsertArrayElementPayload { path: path.clone(), index, value: value.clone() })));
        }
        _ => leaves.push(JsonMutation::SetScalar(SetScalarPayload { path: path.clone(), value: after.clone() })),
    }
}
//#endregion 🔖️Net

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<JsonMutation> {
    use crate::schema::snapshot::JsonValue;
    vec![
        JsonMutation::SetMember(SetMemberPayload { path: Vec::new(), key: "member".into(), value: JsonValue::Null, index: None }),
        JsonMutation::RemoveMember(RemoveMemberPayload { path: Vec::new(), key: "member".into() }),
        JsonMutation::InsertArrayElement(InsertArrayElementPayload { path: Vec::new(), index: 0, value: JsonValue::Null }),
        JsonMutation::RemoveArrayElement(RemoveArrayElementPayload { path: Vec::new(), index: 0 }),
        JsonMutation::SetScalar(SetScalarPayload { path: Vec::new(), value: JsonValue::Null }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
