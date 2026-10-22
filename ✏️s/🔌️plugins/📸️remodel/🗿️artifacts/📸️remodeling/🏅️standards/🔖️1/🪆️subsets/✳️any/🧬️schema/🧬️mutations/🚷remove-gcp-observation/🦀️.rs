//! 🚷 Remodeling mutation — `RemoveGcpObservation`: removes one `GcpObservation` from a GCP by its
//! BASE-state index. Internal counterpart to `add-gcp-observation` (its own inverse): observations
//! are only ever appended, so this only round-trips exactly when `observation_index` addresses the
//! LAST observation — the only way this mutation is ever emitted.

use crate::standards::v1::subsets::any::schema::mutations::add_gcp_observation::AddGcpObservation;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚷 `remove-gcp-observation` payload — `observation_index` is a BASE-state position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-gcp-observation")]
pub struct RemoveGcpObservation {
    pub id: String,
    pub observation_index: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_gcp_observation<Op: From<RemoveGcpObservation>>(id: String, observation_index: u32) -> Op {
    Op::from(RemoveGcpObservation { id, observation_index })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for RemoveGcpObservation
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<AddGcpObservation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "gcp", kind: "remove-gcp-observation", record: "RemovedGcpObservation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove observation {} from GCP \"{}\"", self.observation_index, self.id), &format!("Beobachtung {} aus GCP \"{}\" entfernen", self.observation_index, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for RemoveGcpObservation
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<AddGcpObservation>,
{
    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
