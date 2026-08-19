//! 🪓️ `delete-object` — removes the entry matching `child_id` from `objects` (BASE-state
//! addressing). Idempotent no-op if absent; the inverse escrows the removed handle from BASE.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteObject {
    pub child_id: String,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for DeleteObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "object", kind: "delete-object", record: "DeletedObject" };

    async fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete object child {}", self.child_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}
//#endregion 🔖️Payload
