//! 🔗 `set-primitive-material` — sets (or, given `None`, clears) a primitive's `material_id` reference — real address, one field.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPrimitiveMaterial {
    pub mesh_id: String,
    pub primitive_id: String,
    pub material_id: Option<String>,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for SetPrimitiveMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "primitive", kind: "set-primitive-material", record: "SetPrimitiveMaterial" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set primitive \"{}\" material in mesh \"{}\"", self.primitive_id, self.mesh_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive_id.clone()]
    }
}
//#endregion 🔖️Payload
