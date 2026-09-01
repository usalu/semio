//! 🕸️ `create-mesh` — sets the object's `mesh` CHILD slot to a new owned handle (overwrite-aware,
//! same convention as `create-brep`).

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, delete_mesh};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateMesh {
    pub child_id: String,
    pub target: store::os_io::ArtifactRef,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for CreateMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mesh", kind: "create-mesh", record: "CreatedMesh" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create mesh child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec!["mesh".to_string()]
    }
}
//#endregion 🔖️Payload
