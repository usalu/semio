//! 🧬️ Authoritative replace-pixels mutation.
use crate::artifacts::jpg::schema::diff::*;
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;

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
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ReplacePixelsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "pixels", kind: "replace-pixels", record: "ReplacePixels" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { pixels } = self;
        protocol::MutationOutcome::new(contribute(base, pixels.clone()))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels: base.pixels.clone() })]
    }
    fn label(&self) -> String {
        "replace pixels".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-pixels".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, pixels: Vec<u8>) -> JpgDiff {
    JpgDiff { pixels: (base.pixels != pixels).then_some(pixels), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    dsl::json::from_json_str(include_str!("🧪️tests/🎯️direct-behavior-679189/🦠️mutation/🔣️.json")).expect("committed replace-pixels payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior-679189/🦀️.rs"]
mod tests_direct_behavior;
