//! 🔓️ `remove-keyframe` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioAnimationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_keyframe;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveKeyframe {
    pub timeline_index: usize,
    pub channel_index: usize,
    pub index: usize,
}

impl<Op> protocol::MutationKind<SemioAnimationSnapshot, Op> for RemoveKeyframe
where
    Op: protocol::Mutation<SemioAnimationSnapshot, Diff = crate::schema::diff::SemioAnimationDiff> + From<RemoveKeyframe> + From<crate::schema::mutations::insert_keyframe::InsertKeyframe>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "keyframe", kind: "remove-keyframe", record: "RemoveKeyframe" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove keyframe", "Schlüsselbild entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioAnimationSnapshot, Op> for RemoveKeyframe
where
    Op: protocol::ColdMutation<SemioAnimationSnapshot, ReferenceDiff = crate::schema::diff::SemioAnimationDiff> + From<RemoveKeyframe> + From<crate::schema::mutations::insert_keyframe::InsertKeyframe>,
{
    fn diff(&self, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioAnimationSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioAnimationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
