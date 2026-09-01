//! 🧬️ Authoritative replace-palette-entry mutation.
use crate::artifacts::bmp::schema::diff::{self, *};
use crate::artifacts::bmp::schema::mutations::BmpMutation;
use crate::artifacts::bmp::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePaletteEntryMutation {
    pub index: usize,
    pub entry: BmpPaletteEntry,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for ReplacePaletteEntryMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "palette-entry", kind: "replace-palette-entry", record: "ReplacePaletteEntry" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let Self { index, entry } = self;
        protocol::MutationOutcome::new(BmpDiff { palette: Some(BmpPaletteDiff { removed: Vec::new(), modified: vec![BmpPaletteModified { index: *index, entry: entry.clone() }], added: Vec::new() }), ..Default::default() })
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        let Self { index, entry } = self;
        let outcome = <Self as protocol::MutationKind<BmpSnapshot, BmpMutation>>::diff(self, base);
        if <BmpDiff as protocol::DiffAlgebra<BmpSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.palette.get(*index) {
            Some(entry) => vec![BmpMutation::ReplacePaletteEntry(crate::artifacts::bmp::schema::mutations::ReplacePaletteEntryMutation { index: *index, entry: entry.clone() })],
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
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed replace-palette-entry payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
