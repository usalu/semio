//! 🧬️ Authoritative change-srgb-intent mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeSrgbIntentMutation {
    pub srgb: Option<PngSrgbIntent>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangeSrgbIntentMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "srgb-intent", kind: "change-srgb-intent", record: "ChangeSrgbIntent" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { srgb } = self;
        protocol::MutationOutcome::new(contribute(base, *srgb))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let Self { srgb } = self;
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ChangeSrgbIntent(crate::artifacts::png::schema::mutations::ChangeSrgbIntentMutation { srgb: base.srgb })]
    }
    fn label(&self) -> String {
        "change srgb intent".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-srgb-intent".into()]
    }
}
pub fn contribute(base: &PngSnapshot, srgb: Option<PngSrgbIntent>) -> PngDiff {
    PngDiff { srgb: (base.srgb != srgb).then_some(srgb), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Srgb), PngChunkMarker::Srgb, base.srgb.is_some(), srgb.is_some()), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-srgb-intent payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
