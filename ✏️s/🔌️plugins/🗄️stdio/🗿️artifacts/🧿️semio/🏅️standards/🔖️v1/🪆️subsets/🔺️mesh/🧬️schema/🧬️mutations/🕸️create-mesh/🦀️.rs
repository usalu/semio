//! 🕸️ `create-mesh` — brings a new id-keyed mesh into existence. A duplicate `id` already present in `base` is a no-op (never a duplicate id).

use crate::schema::snapshot::{SemioMesh, SemioMeshSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateMesh {
    pub mesh: SemioMesh,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for CreateMesh
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<CreateMesh> + From<crate::schema::mutations::delete_mesh::DeleteMesh>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mesh", kind: "create-mesh", record: "CreatedMesh" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create mesh \"{}\"", self.mesh.id), &format!("Netz \"{}\" erstellen", self.mesh.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.mesh.id.clone()]
    }
}
//#endregion 🔖️Payload
