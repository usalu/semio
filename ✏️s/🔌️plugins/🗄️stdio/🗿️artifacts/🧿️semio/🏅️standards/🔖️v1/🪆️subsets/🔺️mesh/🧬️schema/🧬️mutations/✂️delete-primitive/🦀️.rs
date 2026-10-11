//! ✂️ `delete-primitive` — removes primitive `primitive_id` from mesh `mesh_id`.

use crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeletePrimitive {
    pub mesh_id: String,
    pub primitive_id: String,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for DeletePrimitive {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "primitive", kind: "delete-primitive", record: "DeletedPrimitive" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<SemioMeshMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete primitive \"{}\" from mesh \"{}\"", self.primitive_id, self.mesh_id), &format!("Primitiv \"{}\" aus Netz \"{}\" löschen", self.primitive_id, self.mesh_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive_id.clone()]
    }
}
//#endregion 🔖️Payload
