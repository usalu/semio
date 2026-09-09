//! 🧬️ Authoritative remove-quant-table mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveQuantTableMutation {
    pub id: u8,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
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
            Some(existing) => vec![JpgMutation::ReplaceQuantTable(crate::schema::mutations::ReplaceQuantTableMutation { table: existing.clone() })],
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
    dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🧹️remove-quant-table/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed remove-quant-table payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
