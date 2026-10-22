//! 🔗️ `set-relation` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use crate::{schema::snapshot::RelationKind, schema::snapshot::SemioModelSnapshot};
use protocol::Mutation;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRelation {
    pub id: String,
    #[value(default)]
    pub kind: Option<RelationKind>,
    #[value(default)]
    pub from: Option<String>,
    #[value(default)]
    pub to: Option<String>,
}

impl<Op> protocol::MutationKind<SemioModelSnapshot, Op> for SetRelation
where
    Op: protocol::Mutation<SemioModelSnapshot, Diff = crate::schema::diff::SemioModelDiff> + From<SetRelation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "relation", kind: "set-relation", record: "SetRelation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set relation", "Relation setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioModelSnapshot, Op> for SetRelation
where
    Op: protocol::ColdMutation<SemioModelSnapshot, ReferenceDiff = crate::schema::diff::SemioModelDiff> + From<SetRelation>,
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
