//! 🔀 `set-primitive-topology` — sets a primitive's draw-mode enum — a real address (`mesh_id`+`primitive_id`) plus one field is exactly the narrow case `set` survives for (SMO's `set-panel-visibility` precedent).

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTopology};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPrimitiveTopology {
    pub mesh_id: String,
    pub primitive_id: String,
    pub topology: SemioTopology,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for SetPrimitiveTopology {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "primitive", kind: "set-primitive-topology", record: "SetPrimitiveTopology" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Set primitive \"{}\" topology in mesh \"{}\"", self.primitive_id, self.mesh_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive_id.clone()]
    }
}
//#endregion 🔖️Payload
