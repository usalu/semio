//! ⚙️ Remodeling mutation — `UpdateIngestParams`: full-record replace of `ReconstructionParams.ingest` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{IngestParams, RemodelingSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-ingest-params` payload — full FINAL-state `IngestParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-ingest-params")]
pub struct UpdateIngestParams {
    #[dsl(block)]
    pub params: IngestParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_ingest_params(params: IngestParams) -> RemodelingMutation {
    RemodelingMutation::UpdateIngestParams(UpdateIngestParams { params })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for UpdateIngestParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "ingest-params", kind: "update-ingest-params", record: "UpdatedIngestParams" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update ingest params".to_string()
    }
}
//#endregion 🔖️Mutation
