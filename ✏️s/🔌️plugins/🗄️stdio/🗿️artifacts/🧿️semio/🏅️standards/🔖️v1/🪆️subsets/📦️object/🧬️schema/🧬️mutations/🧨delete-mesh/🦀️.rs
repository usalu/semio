//! 🧨️ `delete-mesh` — clears the object's `mesh` CHILD slot. Idempotent; inverse escrows from BASE.

use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct DeleteMesh {}

impl<Op> protocol::MutationKind<SemioObjectSnapshot, Op> for DeleteMesh
where
    Op: protocol::Mutation<SemioObjectSnapshot, Diff = crate::schema::diff::SemioObjectDiff> + From<DeleteMesh> + From<crate::schema::mutations::create_mesh::CreateMesh>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "mesh", kind: "delete-mesh", record: "DeletedMesh" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete mesh child", "Netzkind löschen")
    }
    fn target(&self) -> Vec<String> {
        vec!["mesh".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioObjectSnapshot, Op> for DeleteMesh
where
    Op: protocol::ColdMutation<SemioObjectSnapshot, ReferenceDiff = crate::schema::diff::SemioObjectDiff> + From<DeleteMesh> + From<crate::schema::mutations::create_mesh::CreateMesh>,
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
