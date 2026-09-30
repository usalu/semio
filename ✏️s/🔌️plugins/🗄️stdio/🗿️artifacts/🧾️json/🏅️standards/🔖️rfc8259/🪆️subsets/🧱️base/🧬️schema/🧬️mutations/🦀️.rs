//! 🧬️ Transparent JsonMutation aggregate.
use crate::schema::diff::JsonDiff;
use crate::JsonSnapshot;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

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
    PatchSnapshot(patch_snapshot::PatchSnapshot),
}

pub fn apply_json_mutation(snapshot: &mut JsonSnapshot, mutation: &JsonMutation) -> protocol::MutationOutcome<JsonDiff> {
    let outcome = <JsonMutation as protocol::Mutation<JsonSnapshot>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome
}

/// 📥️ Decodes one leaf wire payload (a `🥒️.feature` row's `params`: the leaf's `payload_value()`, no aggregate tag) into
/// the operation of semantic kind `kind` through the derive-generated `Mutation::from_payload_value`, so a caller that
/// cannot name the trait reads the committed wire instead of re-declaring it field by field.
pub fn decode_json_mutation_payload_json(kind: &str, payload: &str) -> Result<JsonMutation, String> {
    let value = pack::parse_json(payload).map_err(|error| error.to_string())?;
    <JsonMutation as protocol::Mutation<JsonSnapshot>>::from_payload_value(kind, pack::json_to_dsl_value(&value)).map_err(|error| error.to_string())
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<JsonMutation> {
    use crate::schema::snapshot::JsonValue;
    vec![
        patch_snapshot::test_case(),
        JsonMutation::SetMember(SetMemberMutation::Apply(SetMemberPayload { path: Vec::new(), key: "member".into(), value: JsonValue::Null })),
        JsonMutation::RemoveMember(RemoveMemberMutation::Apply(RemoveMemberPayload { path: Vec::new(), key: "member".into() })),
        JsonMutation::InsertArrayElement(InsertArrayElementMutation::Apply(InsertArrayElementPayload { path: Vec::new(), index: 0, value: JsonValue::Null })),
        JsonMutation::RemoveArrayElement(RemoveArrayElementMutation::Apply(RemoveArrayElementPayload { path: Vec::new(), index: 0 })),
        JsonMutation::SetScalar(SetScalarMutation::Apply(SetScalarPayload { path: Vec::new(), value: JsonValue::Null })),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
