//! 🔎 Remodeling mutation — `AddGcpObservation`: appends one `GcpObservation` to an existing GCP.

use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::{GcpObservation, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔎 `add-gcp-observation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-gcp-observation")]
pub struct AddGcpObservation {
    pub id: String,
    #[dsl(block)]
    pub observation: GcpObservation,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_gcp_observation(id: String, observation: GcpObservation) -> RemodelingMutation {
    RemodelingMutation::AddGcpObservation(AddGcpObservation { id, observation })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for AddGcpObservation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "gcp", kind: "add-gcp-observation", record: "AddedGcpObservation" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add observation to GCP \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
