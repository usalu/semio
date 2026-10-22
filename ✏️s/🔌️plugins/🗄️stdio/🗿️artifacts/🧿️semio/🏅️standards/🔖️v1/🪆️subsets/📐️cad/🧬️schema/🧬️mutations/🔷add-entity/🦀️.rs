//! 🔷️ `add-entity` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use crate::schema::snapshot::CadEntityRecord;
#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_entity;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddEntity {
    pub entity: CadEntityRecord,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioCadSnapshot, Op> for AddEntity
where
    Op: protocol::Mutation<SemioCadSnapshot, Diff = crate::schema::diff::SemioCadDiff> + From<AddEntity> + From<crate::schema::mutations::remove_entity::RemoveEntity>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "entity", kind: "add-entity", record: "AddEntity" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add entity", "Entität hinzufügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioCadSnapshot, Op> for AddEntity
where
    Op: protocol::ColdMutation<SemioCadSnapshot, ReferenceDiff = crate::schema::diff::SemioCadDiff> + From<AddEntity> + From<crate::schema::mutations::remove_entity::RemoveEntity>,
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
