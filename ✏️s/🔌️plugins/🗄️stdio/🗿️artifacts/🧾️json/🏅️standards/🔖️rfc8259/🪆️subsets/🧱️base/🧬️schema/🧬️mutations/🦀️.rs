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

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = JsonSnapshot, diff = JsonDiff, schema = "s.stdio.json")]
pub enum JsonMutation {
    SetMember(SetMemberMutation),
    RemoveMember(RemoveMemberMutation),
    InsertArrayElement(InsertArrayElementMutation),
    RemoveArrayElement(RemoveArrayElementMutation),
    SetScalar(SetScalarMutation),
}




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
