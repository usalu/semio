//! 🪒 `remove-slide` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioPresentationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_slide;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSlide {
    pub index: usize,
}

impl<Op> protocol::MutationKind<SemioPresentationSnapshot, Op> for RemoveSlide
where
    Op: protocol::Mutation<SemioPresentationSnapshot, Diff = crate::schema::diff::SemioPresentationDiff> + From<RemoveSlide> + From<crate::schema::mutations::insert_slide::InsertSlide>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "slide", kind: "remove-slide", record: "RemoveSlide" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove slide", "Folie entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioPresentationSnapshot, Op> for RemoveSlide
where
    Op: protocol::ColdMutation<SemioPresentationSnapshot, ReferenceDiff = crate::schema::diff::SemioPresentationDiff> + From<RemoveSlide> + From<crate::schema::mutations::insert_slide::InsertSlide>,
{
    fn diff(&self, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioPresentationSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
