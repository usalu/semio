//! 🎬️ `insert-timeline` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use crate::schema::snapshot::AnimTimeline;
#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioAnimationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_timeline;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertTimeline {
    pub index: usize,
    pub timeline: AnimTimeline,
}

impl<Op> protocol::MutationKind<SemioAnimationSnapshot, Op> for InsertTimeline
where
    Op: protocol::Mutation<SemioAnimationSnapshot, Diff = crate::schema::diff::SemioAnimationDiff> + From<InsertTimeline> + From<crate::schema::mutations::remove_timeline::RemoveTimeline>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "timeline", kind: "insert-timeline", record: "InsertTimeline" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert timeline", "Zeitleiste einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioAnimationSnapshot, Op> for InsertTimeline
where
    Op: protocol::ColdMutation<SemioAnimationSnapshot, ReferenceDiff = crate::schema::diff::SemioAnimationDiff> + From<InsertTimeline> + From<crate::schema::mutations::remove_timeline::RemoveTimeline>,
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
