//! 🧬️ Direct set-member mutation owner.
use crate::artifacts::json::schema::diff::{JsonArrayAdded, JsonArrayDiff, JsonDiff, JsonObjectAdded, JsonObjectDiff, JsonObjectModified, JsonValueDiff};
use crate::artifacts::json::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::artifacts::json::schema::snapshot::JsonValue;
use crate::artifacts::json::JsonSnapshot;

#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetMemberPayload {
    pub path: JsonPath,
    pub key: String,
    pub value: JsonValue,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetMemberMutation { Apply(SetMemberPayload), Restore(JsonDiff) }

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for SetMemberMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "member", kind: "set-member", record: "SetMember" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(match resolve(&base.value, &payload.path) {
                Some(JsonValue::Object { members }) => match members.iter().find(|member| member.key == payload.key) {
                    Some(existing) => { let leaf = crate::artifacts::json::schema::diff::value_diff_between(&existing.value, &payload.value); diff_at_path(&payload.path, leaf.map(|diff| JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: payload.key.clone(), diff }] } })) }
                    None => diff_at_path(&payload.path, Some(JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonObjectAdded { index: members.len(), key: payload.key.clone(), item: payload.value.clone() }] } })),
                },
                _ => JsonDiff::default(),
            }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &JsonSnapshot) -> Vec<super::JsonMutation> {
        let outcome = <Self as protocol::MutationKind<JsonSnapshot, super::JsonMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::inverse(outcome.diff(), base);
        vec![super::JsonMutation::SetMember(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Member".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-member".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<SetMemberMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "set-member"); }
}
