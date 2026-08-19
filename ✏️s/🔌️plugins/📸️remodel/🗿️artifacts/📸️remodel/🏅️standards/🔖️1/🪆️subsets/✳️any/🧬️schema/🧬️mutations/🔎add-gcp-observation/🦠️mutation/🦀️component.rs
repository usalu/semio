//! 🔎 Remodel mutation — `AddGcpObservation`: appends one `GcpObservation` to an existing GCP.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{GcpObservation, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔎 `add-gcp-observation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-gcp-observation")]
pub struct AddGcpObservation {
    pub id: String,
    #[dsl(block)]
    pub observation: GcpObservation,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_gcp_observation(id: String, observation: GcpObservation) -> RemodelMutation {
    RemodelMutation::AddGcpObservation(AddGcpObservation { id, observation })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for AddGcpObservation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "gcp", kind: "add-gcp-observation", record: "AddedGcpObservation" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add observation to GCP \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
