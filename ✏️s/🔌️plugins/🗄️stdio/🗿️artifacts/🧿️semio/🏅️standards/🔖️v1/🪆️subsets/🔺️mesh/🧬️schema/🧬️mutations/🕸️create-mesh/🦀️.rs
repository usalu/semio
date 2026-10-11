//! 🕸️ `create-mesh` — brings a new id-keyed mesh into existence. A duplicate `id` already present in `base` is a no-op (never a duplicate id).

use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateMesh {
    pub mesh: SemioMesh,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for CreateMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mesh", kind: "create-mesh", record: "CreatedMesh" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
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
