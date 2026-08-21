//! 🏛️ `create-model` — appends a new owned CHILD handle to the kit's `models` collection.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModel {
    pub child_id: String,
    pub target: store::os_io::ArtifactRef,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for CreateModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "model", kind: "create-model", record: "CreatedModel" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create model child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}
//#endregion 🔖️Payload
