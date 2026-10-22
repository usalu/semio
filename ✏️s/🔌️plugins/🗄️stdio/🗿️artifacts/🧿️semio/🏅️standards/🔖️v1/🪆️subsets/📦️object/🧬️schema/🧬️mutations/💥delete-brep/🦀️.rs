//! 💥️ `delete-brep` — clears the object's `brep` CHILD slot. Idempotent (a no-op if already
//! empty); the inverse captures the escrowed handle from BASE so undo restores it exactly.

use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct DeleteBrep {}

impl<Op> protocol::MutationKind<SemioObjectSnapshot, Op> for DeleteBrep
where
    Op: protocol::Mutation<SemioObjectSnapshot, Diff = crate::schema::diff::SemioObjectDiff> + From<DeleteBrep> + From<crate::schema::mutations::create_brep::CreateBrep>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "brep", kind: "delete-brep", record: "DeletedBrep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete brep child", "B-Rep-Kind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["brep".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioObjectSnapshot, Op> for DeleteBrep
where
    Op: protocol::ColdMutation<SemioObjectSnapshot, ReferenceDiff = crate::schema::diff::SemioObjectDiff> + From<DeleteBrep> + From<crate::schema::mutations::create_brep::CreateBrep>,
{
    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioObjectSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
