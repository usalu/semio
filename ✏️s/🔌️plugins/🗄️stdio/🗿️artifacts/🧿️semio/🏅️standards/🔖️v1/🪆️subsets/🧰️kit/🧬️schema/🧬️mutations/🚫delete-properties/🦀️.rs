//! 🚫️ `delete-properties` — clears the kit's `properties` CHILD slot. Idempotent; inverse
//! escrows from BASE.

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteProperties {}

impl<Op> protocol::MutationKind<SemioKitSnapshot, Op> for DeleteProperties
where
    Op: protocol::Mutation<SemioKitSnapshot, Diff = crate::schema::diff::SemioKitDiff> + From<DeleteProperties> + From<crate::schema::mutations::create_properties::CreateProperties>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "properties", kind: "delete-properties", record: "DeletedProperties" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete properties child", "Eigenschaftenkind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["properties".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioKitSnapshot, Op> for DeleteProperties
where
    Op: protocol::ColdMutation<SemioKitSnapshot, ReferenceDiff = crate::schema::diff::SemioKitDiff> + From<DeleteProperties> + From<crate::schema::mutations::create_properties::CreateProperties>,
{
    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioKitSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
