//! 🕳️ `delete-texture` — removes an id-keyed texture.

use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteTexture {
    pub id: String,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for DeleteTexture
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<DeleteTexture> + From<crate::schema::mutations::create_texture::CreateTexture>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "texture", kind: "delete-texture", record: "DeletedTexture" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete texture \"{}\"", self.id), &format!("Textur \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioMeshSnapshot, Op> for DeleteTexture
where
    Op: protocol::ColdMutation<SemioMeshSnapshot, ReferenceDiff = crate::schema::diff::SemioMeshDiff> + From<DeleteTexture> + From<crate::schema::mutations::create_texture::CreateTexture>,
{
    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioMeshSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
