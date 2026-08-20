//! 🏷️ `create-properties` — sets the kit's `properties` CHILD slot to a new owned `value` tree
//! handle (overwrite-aware — undo restores whichever handle was there before).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProperties {
    pub child_id: String,
    pub target: store::os_io::ArtifactRef,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for CreateProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "properties", kind: "create-properties", record: "CreatedProperties" };

    async fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Create properties child {}", self.child_id)
    }
    async fn target(&self) -> Vec<String> {
        vec!["properties".to_string()]
    }
}
//#endregion 🔖️Payload
