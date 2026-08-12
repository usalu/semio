//! 💣️ `delete-model` — removes the entry matching `child_id` from `models`. Idempotent no-op if
//! absent; the inverse escrows the removed handle from BASE.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteModel {
    pub child_id: String,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for DeleteModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "model", kind: "delete-model", record: "DeletedModel" };

    fn diff(&self, base: &SemioKitSnapshot) -> <SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete model child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}
//#endregion 🔖️Payload
