//! ⚙️ Remodel mutation — `UpdateIngestParams`: full-record replace of `ReconstructionParams.ingest` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{IngestParams, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-ingest-params` payload — full FINAL-state `IngestParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-ingest-params")]
pub struct UpdateIngestParams {
    #[dsl(block)]
    pub params: IngestParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_ingest_params(params: IngestParams) -> RemodelMutation {
    RemodelMutation::UpdateIngestParams(UpdateIngestParams { params })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for UpdateIngestParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "ingest-params", kind: "update-ingest-params", record: "UpdatedIngestParams" };

    fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update ingest params".to_string()
    }
}
//#endregion 🔖️Mutation
