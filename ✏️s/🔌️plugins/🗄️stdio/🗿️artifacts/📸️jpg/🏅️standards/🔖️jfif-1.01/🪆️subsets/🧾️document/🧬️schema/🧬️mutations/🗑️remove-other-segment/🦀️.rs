//! 🧬️ Authoritative remove-other-segment mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
        Ok(base.image.other_segments.get(self.index).map(|segment| JpgMutation::InsertOtherSegment(crate::schema::mutations::InsertOtherSegmentMutation { index: self.index, segment: segment.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove other segment", "Sonstiges Segment entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-other-segment".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, index: usize) -> JpgDiff {
    if index >= base.image.other_segments.len() {
        return JpgDiff::default();
    }
    JpgDiff { other_segments: Some(JpgOtherSegmentsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
//#endregion Semantics
