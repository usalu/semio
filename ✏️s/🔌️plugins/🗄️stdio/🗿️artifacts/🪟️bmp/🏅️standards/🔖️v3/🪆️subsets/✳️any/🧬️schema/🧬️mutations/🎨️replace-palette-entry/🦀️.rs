//! 🧬️ Authoritative replace-palette-entry mutation.
use crate::schema::diff::*;
use crate::schema::mutations::BmpMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePaletteEntryMutation {
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
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for ReplacePaletteEntryMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "palette-entry", kind: "replace-palette-entry", record: "ReplacePaletteEntry" };
    fn diff(&self, _base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let Self { index, entry } = self;
        protocol::MutationOutcome::new(BmpDiff { palette: Some(BmpPaletteDiff { removed: Vec::new(), modified: vec![BmpPaletteModified { index: *index, entry: entry.clone() }], added: Vec::new() }), ..Default::default() })
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        let Self { index, .. } = self;
        let outcome = <Self as protocol::MutationKind<BmpSnapshot, BmpMutation>>::diff(self, base);
        if <BmpDiff as protocol::DiffAlgebra<BmpSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.palette.get(*index) {
            Some(entry) => vec![BmpMutation::ReplacePaletteEntry(ReplacePaletteEntryMutation { index: *index, entry: entry.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "replace palette entry".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-palette-entry".into()]
    }
}

//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> BmpMutation {
    dsl::json::from_json_str(include_str!("🧪️tests/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed replace-palette-entry payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
