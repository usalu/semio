//! 🚫️ `remove-block` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::add_block;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveBlock {
    pub name: String,
}

impl<Op> protocol::MutationKind<SemioCadSnapshot, Op> for RemoveBlock
where
    Op: protocol::Mutation<SemioCadSnapshot, Diff = crate::schema::diff::SemioCadDiff> + From<RemoveBlock> + From<crate::schema::mutations::add_block::AddBlock>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "block", kind: "remove-block", record: "RemoveBlock" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove block", "Block entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioCadSnapshot, Op> for RemoveBlock
where
    Op: protocol::ColdMutation<SemioCadSnapshot, ReferenceDiff = crate::schema::diff::SemioCadDiff> + From<RemoveBlock> + From<crate::schema::mutations::add_block::AddBlock>,
{
    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioCadSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
