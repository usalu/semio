//! 🚷 Remodel mutation — `RemoveGcpObservation`: removes one `GcpObservation` from a GCP by its
//! BASE-state index. Internal counterpart to `add-gcp-observation` (its own inverse): observations
//! are only ever appended, so this only round-trips exactly when `observation_index` addresses the
//! LAST observation — the only way this mutation is ever emitted.

use crate::artifacts::remodel::RemodelSnapshot;
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList};
use crate::artifacts::remodel::mutations::RemodelMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🚷 `remove-gcp-observation` payload — `observation_index` is a BASE-state position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-gcp-observation")]
pub struct RemoveGcpObservation {
    pub id: String,
    pub observation_index: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_gcp_observation(id: String, observation_index: u32) -> RemodelMutation {
    RemodelMutation::RemoveGcpObservation(RemoveGcpObservation { id, observation_index })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for RemoveGcpObservation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "gcp", kind: "remove-gcp-observation", record: "RemovedGcpObservation" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove observation {} from GCP \"{}\"", self.observation_index, self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
