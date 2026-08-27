//! 🧬️ Authoritative change-physical-dims mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangePhysicalDimsMutation {
    pub phys: Option<PngPhysicalDims>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangePhysicalDimsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "physical-dims", kind: "change-physical-dims", record: "ChangePhysicalDims" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { phys } = self;
        protocol::MutationOutcome::new(contribute(base, *phys))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let Self { phys } = self;
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ChangePhysicalDims(crate::artifacts::png::schema::mutations::ChangePhysicalDimsMutation { phys: base.phys })]
    }
    fn label(&self) -> String {
        "change physical dims".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-physical-dims".into()]
    }
}
pub fn contribute(base: &PngSnapshot, phys: Option<PngPhysicalDims>) -> PngDiff {
    PngDiff { phys: (base.phys != phys).then_some(phys), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Phys), PngChunkMarker::Phys, base.phys.is_some(), phys.is_some()), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed change-physical-dims payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
