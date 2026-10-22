//! 🖼️ `create-texture` — brings a new id-keyed texture into existence. A duplicate `id` already present in `base` is a no-op.

use crate::schema::snapshot::{SemioMeshSnapshot, SemioTexture};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTexture {
    pub texture: SemioTexture,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for CreateTexture
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<CreateTexture> + From<crate::schema::mutations::delete_texture::DeleteTexture>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "texture", kind: "create-texture", record: "CreatedTexture" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create texture \"{}\"", self.texture.id), &format!("Textur \"{}\" erstellen", self.texture.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.texture.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioMeshSnapshot, Op> for CreateTexture
where
    Op: protocol::ColdMutation<SemioMeshSnapshot, ReferenceDiff = crate::schema::diff::SemioMeshDiff> + From<CreateTexture> + From<crate::schema::mutations::delete_texture::DeleteTexture>,
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
