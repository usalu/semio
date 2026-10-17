//! 🎨 `create-material` — brings a new id-keyed material into existence. A duplicate `id` already present in `base` is a no-op.

use crate::schema::snapshot::{SemioMaterial, SemioMeshSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateMaterial {
    pub material: SemioMaterial,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for CreateMaterial
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<CreateMaterial> + From<crate::schema::mutations::delete_material::DeleteMaterial>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create material \"{}\"", self.material.id), &format!("Material \"{}\" erstellen", self.material.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.material.id.clone()]
    }
}
//#endregion 🔖️Payload
