//! 🧬️ Authoritative insert-palette-entry mutation.
use crate::artifacts::bmp::schema::diff::{self, *};
use crate::artifacts::bmp::schema::mutations::BmpMutation;
use crate::artifacts::bmp::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertPaletteEntryMutation {
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
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for InsertPaletteEntryMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "palette-entry", kind: "insert-palette-entry", record: "InsertPaletteEntry" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let Self { index, entry } = self;
        protocol::MutationOutcome::new(BmpDiff { palette: Some(BmpPaletteDiff { removed: Vec::new(), modified: Vec::new(), added: vec![BmpPaletteAdded { index: *index, entry: entry.clone() }] }), ..Default::default() })
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        let Self { index, entry } = self;
        let outcome = <Self as protocol::MutationKind<BmpSnapshot, BmpMutation>>::diff(self, base);
        if <BmpDiff as protocol::DiffAlgebra<BmpSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        {
            vec![BmpMutation::RemovePaletteEntry(crate::artifacts::bmp::schema::mutations::RemovePaletteEntryMutation { index: *index })]
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
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed insert-palette-entry payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
