//! 🧬️ Authoritative insert-other-segment mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertOtherSegmentMutation {
    pub index: usize,
    pub segment: JpgSegment,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for InsertOtherSegmentMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "other-segment", kind: "insert-other-segment", record: "InsertOtherSegment" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { index, segment } = self;
        protocol::MutationOutcome::new(contribute(base, *index, segment.clone()))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let Self { index, segment } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        {
            vec![JpgMutation::RemoveOtherSegment(crate::artifacts::jpg::schema::mutations::RemoveOtherSegmentMutation { index: (*index).min(base.other_segments.len()) })]
        }
    }
    fn label(&self) -> String {
        "insert other segment".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-other-segment".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, index: usize, segment: JpgSegment) -> JpgDiff {
    let at = index.min(base.other_segments.len());
    JpgDiff { other_segments: Some(JpgOtherSegmentsDiff { removed: vec![], modified: vec![], added: vec![JpgSegmentAdded { index: at, item: segment }] }), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed insert-other-segment payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
