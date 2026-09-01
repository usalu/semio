//! 🧬️ Direct insert-array-element mutation owner.
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
pub struct InsertArrayElementPayload {
    pub path: JsonPath,
    pub index: usize,
    pub value: JsonValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum InsertArrayElementMutation { Apply(InsertArrayElementPayload), Restore(JsonDiff) }

impl protocol::MutationKind<JsonSnapshot, super::JsonMutation> for InsertArrayElementMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "array-element", kind: "insert-array-element", record: "InsertedArrayElement" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<JsonDiff> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(match resolve(&base.value, &payload.path) { Some(JsonValue::Array { items }) => diff_at_path(&payload.path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonArrayAdded { index: payload.index.min(items.len()), item: payload.value.clone() }] } })), _ => JsonDiff::default() }),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &JsonSnapshot) -> Vec<super::JsonMutation> {
        let outcome = <Self as protocol::MutationKind<JsonSnapshot, super::JsonMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <JsonDiff as protocol::DiffAlgebra<JsonSnapshot>>::inverse(outcome.diff(), base);
        vec![super::JsonMutation::InsertArrayElement(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "Insert Array Element".to_string() }
    fn target(&self) -> Vec<String> { vec!["insert-array-element".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<InsertArrayElementMutation as protocol::MutationKind<JsonSnapshot, super::super::JsonMutation>>::SEMANTICS.kind, "insert-array-element"); }
}
