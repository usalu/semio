//! 🏗️ `create-object` — appends a new owned CHILD handle to the kit's `objects` collection
//! (FINAL-state addressing per `📓️taxonomy.md`'s insert/create convention).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateObject {
    pub child_id: String,
    pub target: store::os_io::ArtifactRef,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for CreateObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "object", kind: "create-object", record: "CreatedObject" };

    fn diff(&self, base: &SemioKitSnapshot) -> <SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create object child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}
//#endregion 🔖️Payload
