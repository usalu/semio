//! 🖼️ `create-texture` — brings a new id-keyed texture into existence. A duplicate `id` already present in `base` is a no-op.

use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTexture};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTexture {
    pub texture: SemioTexture,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreateTexture {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "texture", kind: "create-texture", record: "CreatedTexture" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create texture \"{}\"", self.texture.id), &format!("Textur \"{}\" erstellen", self.texture.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.texture.id.clone()]
    }
}
//#endregion 🔖️Payload
