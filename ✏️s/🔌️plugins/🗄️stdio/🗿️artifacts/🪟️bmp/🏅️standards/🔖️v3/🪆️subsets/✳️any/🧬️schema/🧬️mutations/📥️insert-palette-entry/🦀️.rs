//! 🧬️ Authoritative insert-palette-entry mutation.
use crate::schema::diff::*;
use crate::schema::mutations::BmpMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertPaletteEntryMutation {
    pub index: usize,
    pub entry: BmpPaletteEntry,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for InsertPaletteEntryMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "palette-entry", kind: "insert-palette-entry", record: "InsertPaletteEntry" };
    fn diff(&self, _base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let Self { index, entry } = self;
        protocol::MutationOutcome::new(BmpDiff { palette: Some(BmpPaletteDiff { removed: Vec::new(), modified: Vec::new(), added: vec![BmpPaletteAdded { index: *index, entry: entry.clone() }] }), ..Default::default() })
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        let Self { index, .. } = self;
        let outcome = <Self as protocol::MutationKind<BmpSnapshot, BmpMutation>>::diff(self, base);
        if <BmpDiff as protocol::DiffAlgebra<BmpSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        {
            vec![BmpMutation::RemovePaletteEntry(crate::schema::mutations::RemovePaletteEntryMutation { index: *index })]
        }
    }
    fn label(&self) -> String {
        "insert palette entry".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-palette-entry".into()]
    }
}

//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> BmpMutation {
    dsl::json::from_json_str(include_str!("🧪️tests/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed insert-palette-entry payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
