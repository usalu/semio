//! 🧬️ Authoritative remove-huffman-table mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveHuffmanTableMutation {
    pub key: JpgHuffmanTableKey,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for RemoveHuffmanTableMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "huffman-table", kind: "remove-huffman-table", record: "RemoveHuffmanTable" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { key } = self;
        protocol::MutationOutcome::new(contribute(base, *key))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let Self { key } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.huffman_tables.iter().find(|t| t.class == key.class && t.id == key.id) {
            Some(existing) => vec![JpgMutation::ReplaceHuffmanTable(crate::artifacts::jpg::schema::mutations::ReplaceHuffmanTableMutation { table: existing.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "remove huffman table".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-huffman-table".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, key: JpgHuffmanTableKey) -> JpgDiff {
    if !base.huffman_tables.iter().any(|t| huffman_key(t) == key) {
        return JpgDiff::default();
    }
    JpgDiff { huffman_tables: Some(JpgHuffmanTablesDiff { removed: vec![key], modified: vec![], added: vec![] }), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️.json")).expect("committed remove-huffman-table payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
