//! 🧬️ Authoritative remove-palette-entry mutation.
use crate::artifacts::bmp::schema::diff::{self, *};
use crate::artifacts::bmp::schema::mutations::BmpMutation;
use crate::artifacts::bmp::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemovePaletteEntryMutation {
    pub index: usize,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for RemovePaletteEntryMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "palette-entry", kind: "remove-palette-entry", record: "RemovePaletteEntry" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(BmpDiff { palette: Some(BmpPaletteDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }), ..Default::default() })
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        let Self { index } = self;
        let outcome = <Self as protocol::MutationKind<BmpSnapshot, BmpMutation>>::diff(self, base);
        if <BmpDiff as protocol::DiffAlgebra<BmpSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.palette.get(*index) {
            Some(entry) => vec![BmpMutation::InsertPaletteEntry(crate::artifacts::bmp::schema::mutations::InsertPaletteEntryMutation { index: *index, entry: entry.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> String {
        "remove palette entry".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-palette-entry".into()]
    }
}

//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> BmpMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed remove-palette-entry payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
