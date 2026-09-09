//! 🧬️ Authoritative change-transparency mutation.
use crate::schema::diff::*;
use crate::schema::mutations::PngMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeTransparencyMutation {
    pub trns: Option<PngTransparency>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangeTransparencyMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "transparency", kind: "change-transparency", record: "ChangeTransparency" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { trns } = self;
        protocol::MutationOutcome::new(contribute(base, trns))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ChangeTransparency(ChangeTransparencyMutation { trns: base.trns.clone() })]
    }
    fn label(&self) -> String {
        "change transparency".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-transparency".into()]
    }
}
pub fn contribute(base: &PngSnapshot, trns: &Option<PngTransparency>) -> PngDiff {
    PngDiff { trns: (base.trns != *trns).then(|| trns.clone()), chunk_order: chunk_order_presence_diff(&base.chunk_order, |m| matches!(m, PngChunkMarker::Trns), PngChunkMarker::Trns, base.trns.is_some(), trns.is_some()), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/👁️change-transparency/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-transparency payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
