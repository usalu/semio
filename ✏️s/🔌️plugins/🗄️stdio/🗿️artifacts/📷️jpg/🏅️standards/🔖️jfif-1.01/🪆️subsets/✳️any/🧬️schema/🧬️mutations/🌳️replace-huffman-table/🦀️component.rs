//! 🧬️ Authoritative replace-huffman-table mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceHuffmanTableMutation {
    pub table: JpgHuffmanTable,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ReplaceHuffmanTableMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "huffman-table", kind: "replace-huffman-table", record: "ReplaceHuffmanTable" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { table } = self;
        protocol::MutationOutcome::new(contribute(base, table.clone()))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let Self { table } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        {
            let key = JpgHuffmanTableKey { class: table.class, id: table.id };
            match base.huffman_tables.iter().find(|t| t.class == key.class && t.id == key.id) {
                Some(existing) => vec![JpgMutation::ReplaceHuffmanTable(crate::artifacts::jpg::schema::mutations::ReplaceHuffmanTableMutation { table: existing.clone() })],
                None => vec![JpgMutation::RemoveHuffmanTable(crate::artifacts::jpg::schema::mutations::RemoveHuffmanTableMutation { key })],
            }
        }
    }
    fn label(&self) -> String {
        "replace huffman table".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-huffman-table".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, table: JpgHuffmanTable) -> JpgDiff {
    let key = huffman_key(&table);
    let d = match base.huffman_tables.iter().find(|t| huffman_key(t) == key) {
        Some(existing) => {
            let fd = JpgHuffmanTableDiff::between(existing, &table);
            if fd.is_empty() {
                JpgHuffmanTablesDiff::default()
            } else {
                JpgHuffmanTablesDiff { removed: vec![], modified: vec![JpgHuffmanTableModified { key, diff: fd }], added: vec![] }
            }
        }
        None => JpgHuffmanTablesDiff { removed: vec![], modified: vec![], added: vec![JpgHuffmanTableAdded { index: base.huffman_tables.len(), item: table }] },
    };
    JpgDiff { huffman_tables: (!d.is_empty()).then_some(d), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    let vector: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️component.json")).expect("authored mutation vector");
    serde_json::from_value(vector["mutation"].clone()).expect("direct mutation payload")
}
#[cfg(test)]
#[path = "🧪️tests/🦀️component.rs"]
mod tests;
