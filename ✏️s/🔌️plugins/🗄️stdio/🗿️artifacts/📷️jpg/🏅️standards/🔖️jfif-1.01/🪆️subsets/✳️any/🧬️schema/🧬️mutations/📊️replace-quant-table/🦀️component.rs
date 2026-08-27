//! 🧬️ Authoritative replace-quant-table mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceQuantTableMutation {
    pub table: JpgQuantTable,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ReplaceQuantTableMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "quant-table", kind: "replace-quant-table", record: "ReplaceQuantTable" };
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
        match base.quant_tables.iter().find(|t| t.id == table.id) {
            Some(existing) => vec![JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: existing.clone() })],
            None => vec![JpgMutation::RemoveQuantTable(crate::artifacts::jpg::schema::mutations::RemoveQuantTableMutation { id: table.id })],
        }
    }
    fn label(&self) -> String {
        "replace quant table".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-quant-table".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, table: JpgQuantTable) -> JpgDiff {
    let d = match base.quant_tables.iter().position(|t| t.id == table.id) {
        Some(_) => {
            let existing = base.quant_tables.iter().find(|t| t.id == table.id).unwrap();
            let fd = JpgQuantTableDiff::between(existing, &table);
            if fd.is_empty() {
                JpgQuantTablesDiff::default()
            } else {
                JpgQuantTablesDiff { removed: vec![], modified: vec![JpgQuantTableModified { id: table.id, diff: fd }], added: vec![] }
            }
        }
        None => JpgQuantTablesDiff { removed: vec![], modified: vec![], added: vec![JpgQuantTableAdded { index: base.quant_tables.len(), item: table }] },
    };
    JpgDiff { quant_tables: (!d.is_empty()).then_some(d), ..Default::default() }
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
