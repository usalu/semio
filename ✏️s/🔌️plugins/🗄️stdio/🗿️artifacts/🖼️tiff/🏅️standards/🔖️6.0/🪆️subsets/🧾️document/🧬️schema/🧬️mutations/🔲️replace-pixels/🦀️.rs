//! 🧬️ Authoritative replace-pixels mutation.
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePixelsMutation {
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
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for ReplacePixelsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "pixels", kind: "replace-pixels", record: "ReplacePixels" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { pixels } = self;
        protocol::MutationOutcome::new(contribute(base, pixels.clone()))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        let outcome = <Self as protocol::MutationKind<TiffSnapshot, TiffMutation>>::diff(self, base);
        if <TiffDiff as protocol::DiffAlgebra<TiffSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![TiffMutation::ReplacePixels(ReplacePixelsMutation { pixels: base.pixels.clone() })]
    }
    fn label(&self) -> String {
        "replace pixels".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-pixels".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, pixels: Vec<u8>) -> TiffDiff {
    TiffDiff { pixels: (base.pixels != pixels).then_some(pixels), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🔲️replace-pixels/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed replace-pixels payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
