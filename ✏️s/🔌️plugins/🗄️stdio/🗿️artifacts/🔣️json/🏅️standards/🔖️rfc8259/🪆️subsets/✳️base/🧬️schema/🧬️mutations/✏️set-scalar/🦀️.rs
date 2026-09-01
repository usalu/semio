//! 🧬️ Direct set-scalar mutation owner.
use crate::artifacts::json::schema::diff::{JsonArrayAdded, JsonArrayDiff, JsonDiff, JsonObjectAdded, JsonObjectDiff, JsonObjectModified, JsonValueDiff};
use crate::artifacts::json::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::artifacts::json::schema::snapshot::JsonValue;
use crate::artifacts::json::JsonSnapshot;

#[path = "📝️text/🦀️component.rs"]
pub mod text;
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetScalarPayload {
    pub path: JsonPath,
    pub value: JsonValue,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum SetScalarMutation { Apply(SetScalarPayload), Restore(JsonDiff) }

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for SetScalarMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "scalar", kind: "set-scalar", record: "SetScalar" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(match resolve(&base.value, &payload.path) { Some(old) if old != &payload.value => diff_at_path(&payload.path, Some(JsonValueDiff::Replace { value: payload.value.clone() })), _ => JsonDiff::default() }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &JsonSnapshot) -> Vec<super::JsonMutation> {
        let outcome = <Self as protocol::MutationKind<JsonSnapshot, super::JsonMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::inverse(outcome.diff(), base);
        vec![super::JsonMutation::SetScalar(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Set Scalar".to_string() }
    fn target(&self) -> Vec<String> { vec!["set-scalar".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<SetScalarMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "set-scalar"); }
}
