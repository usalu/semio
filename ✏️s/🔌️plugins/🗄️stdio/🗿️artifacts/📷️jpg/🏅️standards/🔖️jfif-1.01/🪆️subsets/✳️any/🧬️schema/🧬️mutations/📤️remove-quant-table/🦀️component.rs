//! 🧬️ Authoritative remove-quant-table mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveQuantTableMutation {
    pub id: u8,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for RemoveQuantTableMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "quant-table", kind: "remove-quant-table", record: "RemoveQuantTable" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { id } = self;
        protocol::MutationOutcome::new(contribute(base, *id))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let Self { id } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.quant_tables.iter().find(|t| t.id == *id) {
            Some(existing) => vec![JpgMutation::ReplaceQuantTable(crate::artifacts::jpg::schema::mutations::ReplaceQuantTableMutation { table: existing.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "remove quant table".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-quant-table".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, id: u8) -> JpgDiff {
    if !base.quant_tables.iter().any(|t| t.id == id) {
        return JpgDiff::default();
    }
    JpgDiff { quant_tables: Some(JpgQuantTablesDiff { removed: vec![id], modified: vec![], added: vec![] }), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed remove-quant-table payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
