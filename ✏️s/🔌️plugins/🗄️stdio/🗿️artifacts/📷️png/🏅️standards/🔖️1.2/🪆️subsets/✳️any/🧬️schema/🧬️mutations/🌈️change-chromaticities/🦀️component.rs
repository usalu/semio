//! 🧬️ Authoritative change-chromaticities mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeChromaticitiesMutation {
    pub chrm: Option<PngChromaticities>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
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
        let Self { chrm } = self;
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ChangeChromaticities(crate::artifacts::png::schema::mutations::ChangeChromaticitiesMutation { chrm: base.chrm })]
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
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed change-chromaticities payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
