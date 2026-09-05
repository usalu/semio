//! 🧬️ Authoritative replace-pixel-data mutation.
use crate::artifacts::bmp::schema::diff::{self, *};
use crate::artifacts::bmp::schema::mutations::BmpMutation;
use crate::artifacts::bmp::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePixelDataMutation {
    pub pixels: Vec<u8>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for ReplacePixelDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "pixel-data", kind: "replace-pixel-data", record: "ReplacePixelData" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let Self { pixels } = self;
        protocol::MutationOutcome::new(BmpDiff { pixels: Some(pixels.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        let Self { pixels } = self;
        let outcome = <Self as protocol::MutationKind<BmpSnapshot, BmpMutation>>::diff(self, base);
        if <BmpDiff as protocol::DiffAlgebra<BmpSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        {
            vec![BmpMutation::ReplacePixelData(crate::artifacts::bmp::schema::mutations::ReplacePixelDataMutation { pixels: base.pixels.clone() })]
        }
    }
    fn label(&self) -> String {
        "replace pixel data".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-pixel-data".into()]
    }
}

//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> BmpMutation {
    serde_json::from_str(include_str!("🧪️tests/🟣️direct-behavior/🦠️mutation/🔣️.json")).expect("committed replace-pixel-data payload")
}
#[cfg(test)]
#[path = "🧪️tests/🟣️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
