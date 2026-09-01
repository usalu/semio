//! 🧬️ Direct remove-array-element mutation owner.
use crate::artifacts::json::schema::diff::{JsonArrayAdded, JsonArrayDiff, JsonDiff, JsonObjectAdded, JsonObjectDiff, JsonObjectModified, JsonValueDiff};
use crate::artifacts::json::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::artifacts::json::schema::snapshot::JsonValue;
use crate::artifacts::json::JsonSnapshot;
use serde::{Deserialize, Serialize};

#[path = "📝️text/🦀️component.rs"]
pub mod text;
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct RemoveArrayElementPayload {
    pub path: JsonPath,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum RemoveArrayElementMutation { Apply(RemoveArrayElementPayload), Restore(JsonDiff) }

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for RemoveArrayElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "array-element", kind: "remove-array-element", record: "RemovedArrayElement" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(match resolve(&base.value, &payload.path) { Some(JsonValue::Array { items }) if payload.index < items.len() => diff_at_path(&payload.path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: vec![payload.index], modified: Vec::new(), added: Vec::new() } })), _ => JsonDiff::default() }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &JsonSnapshot) -> Vec<super::JsonMutation> {
        let outcome = <Self as protocol::MutationKind<JsonSnapshot, super::JsonMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::inverse(outcome.diff(), base);
        vec![super::JsonMutation::RemoveArrayElement(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Remove Array Element".to_string() }
    fn target(&self) -> Vec<String> { vec!["remove-array-element".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<RemoveArrayElementMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "remove-array-element"); }
}
