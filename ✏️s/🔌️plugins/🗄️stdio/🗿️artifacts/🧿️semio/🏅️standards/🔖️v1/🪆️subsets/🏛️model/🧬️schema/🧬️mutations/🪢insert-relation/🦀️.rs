//! 🪢️ `insert-relation` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use crate::{schema::snapshot::ModelRelation, schema::snapshot::SemioModelSnapshot};
use protocol::Mutation;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRelation {
    pub relation: ModelRelation,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioModelSnapshot, Op> for InsertRelation
where
    Op: protocol::Mutation<SemioModelSnapshot, Diff = crate::schema::diff::SemioModelDiff> + From<InsertRelation> + From<crate::schema::mutations::remove_relation::RemoveRelation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "relation", kind: "insert-relation", record: "InsertRelation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert relation", "Relation einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioModelSnapshot, Op> for InsertRelation
where
    Op: protocol::ColdMutation<SemioModelSnapshot, ReferenceDiff = crate::schema::diff::SemioModelDiff> + From<InsertRelation> + From<crate::schema::mutations::remove_relation::RemoveRelation>,
{
    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioModelSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
