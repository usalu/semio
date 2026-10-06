//! 🧬️ Authoritative remove-other-segment mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveOtherSegmentMutation {
    pub index: usize,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for RemoveOtherSegmentMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "other-segment", kind: "remove-other-segment", record: "RemoveOtherSegment" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(contribute(base, *index))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Result<Vec<JpgMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let Self { index } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        match base.other_segments.get(*index) {
            Some(segment) => vec![JpgMutation::InsertOtherSegment(crate::schema::mutations::InsertOtherSegmentMutation { index: *index, segment: segment.clone() })],
            None => Vec::new(),
        }
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove other segment", "Sonstiges Segment entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-other-segment".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, index: usize) -> JpgDiff {
    if index >= base.other_segments.len() {
        return JpgDiff::default();
    }
    JpgDiff { other_segments: Some(JpgOtherSegmentsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
//#endregion Semantics


#[cfg(test)]
#[path = "🧪️tests/🎯️direct/🦀️.rs"]
mod tests_direct_behavior;
