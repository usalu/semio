//! 🧬️ Authoritative change-chromaticities mutation.
use crate::schema::diff::*;
use crate::schema::mutations::PngMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeChromaticitiesMutation {
    pub chrm: Option<PngChromaticities>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangeChromaticitiesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "chromaticities", kind: "change-chromaticities", record: "ChangeChromaticities" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { chrm } = self;
        protocol::MutationOutcome::new(contribute(base, *chrm))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ChangeChromaticities(ChangeChromaticitiesMutation { chrm: base.chrm })]
    }
    fn label(&self) -> String {
        "change chromaticities".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-chromaticities".into()]
    }
}
pub fn contribute(base: &PngSnapshot, chrm: Option<PngChromaticities>) -> PngDiff {
    PngDiff { chrm: (base.chrm != chrm).then_some(chrm), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Chrm), PngChunkMarker::Chrm, base.chrm.is_some(), chrm.is_some()), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🌈️change-chromaticities/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-chromaticities payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
