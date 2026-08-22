//! 🪓 Remodel mutation — `DeleteStream`: removes an id-keyed media stream.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🪓 `delete-stream` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-stream")]
pub struct DeleteStream {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_stream(id: String) -> RemodelMutation {
    RemodelMutation::DeleteStream(DeleteStream { id })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for DeleteStream {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "stream", kind: "delete-stream", record: "DeletedStream" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete stream \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
