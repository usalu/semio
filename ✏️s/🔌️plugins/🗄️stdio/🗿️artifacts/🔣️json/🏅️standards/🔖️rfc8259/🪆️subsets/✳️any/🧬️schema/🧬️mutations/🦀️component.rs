//! 🧬️ Transparent JsonMutation aggregate.
use crate::artifacts::json::schema::diff::JsonDiff;
use crate::artifacts::json::JsonSnapshot;
use serde::{Deserialize, Serialize};

pub use super::set_member::{SetMemberMutation, SetMemberPayload};
pub use super::remove_member::{RemoveMemberMutation, RemoveMemberPayload};
pub use super::insert_array_element::{InsertArrayElementMutation, InsertArrayElementPayload};
pub use super::remove_array_element::{RemoveArrayElementMutation, RemoveArrayElementPayload};
pub use super::set_scalar::{SetScalarMutation, SetScalarPayload};
pub use crate::artifacts::json::schema::mutation_support::{JsonPath, JsonPathSegment};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]
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
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) { *snapshot = next; }
    outcome
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<JsonMutation> {
    use crate::artifacts::json::schema::snapshot::JsonValue;
    vec![
        JsonMutation::SetMember(SetMemberMutation::Apply(SetMemberPayload { path: Vec::new(), key: "member".into(), value: JsonValue::Null })),
        JsonMutation::RemoveMember(RemoveMemberMutation::Apply(RemoveMemberPayload { path: Vec::new(), key: "member".into() })),
        JsonMutation::InsertArrayElement(InsertArrayElementMutation::Apply(InsertArrayElementPayload { path: Vec::new(), index: 0, value: JsonValue::Null })),
        JsonMutation::RemoveArrayElement(RemoveArrayElementMutation::Apply(RemoveArrayElementPayload { path: Vec::new(), index: 0 })),
        JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: Vec::new(), value: JsonValue::Null })),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    #[test]
    fn aggregate_roster_is_exact() { assert_eq!(JsonMutation::kinds().len(), 5); }
}
