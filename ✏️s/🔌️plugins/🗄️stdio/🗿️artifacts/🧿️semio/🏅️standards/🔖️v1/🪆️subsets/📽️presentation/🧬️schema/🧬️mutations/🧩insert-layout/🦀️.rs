//! 🧩 `insert-layout` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioPresentationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SlideLayout;
#[allow(unused_imports)]
use crate::schema::mutations::remove_layout;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLayout {
    pub layout: SlideLayout,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioPresentationSnapshot, Op> for InsertLayout
where
    Op: protocol::Mutation<SemioPresentationSnapshot, Diff = crate::schema::diff::SemioPresentationDiff> + From<InsertLayout> + From<crate::schema::mutations::remove_layout::RemoveLayout>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "layout", kind: "insert-layout", record: "InsertLayout" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert layout", "Layout einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioPresentationSnapshot, Op> for InsertLayout
where
    Op: protocol::ColdMutation<SemioPresentationSnapshot, ReferenceDiff = crate::schema::diff::SemioPresentationDiff> + From<InsertLayout> + From<crate::schema::mutations::remove_layout::RemoveLayout>,
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
