//! 🧬️ Authoritative change-gamma mutation.
use crate::schema::diff::*;
use crate::schema::mutations::PngMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeGammaMutation {
    pub gama: Option<u32>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangeGammaMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "gamma", kind: "change-gamma", record: "ChangeGamma" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { gama } = self;
        protocol::MutationOutcome::new(contribute(base, *gama))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ChangeGamma(ChangeGammaMutation { gama: base.gama })]
    }
    fn label(&self) -> String {
        "change gamma".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-gamma".into()]
    }
}
pub fn contribute(base: &PngSnapshot, gama: Option<u32>) -> PngDiff {
    PngDiff { gama: (base.gama != gama).then_some(gama), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Gama), PngChunkMarker::Gama, base.gama.is_some(), gama.is_some()), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    dsl::json::from_json_str(include_str!("🧪️tests/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-gamma payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
