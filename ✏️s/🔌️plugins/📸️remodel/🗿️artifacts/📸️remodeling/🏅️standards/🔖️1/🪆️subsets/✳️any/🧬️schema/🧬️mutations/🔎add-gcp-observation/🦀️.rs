//! 🔎 Remodeling mutation — `AddGcpObservation`: appends one `GcpObservation` to an existing GCP.

use crate::standards::v1::subsets::any::schema::mutations::remove_gcp_observation::RemoveGcpObservation;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GcpObservation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔎 `add-gcp-observation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn add_gcp_observation<Op: From<AddGcpObservation>>(id: String, observation: GcpObservation) -> Op {
    Op::from(AddGcpObservation { id, observation })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for AddGcpObservation
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<RemoveGcpObservation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "gcp", kind: "add-gcp-observation", record: "AddedGcpObservation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add observation to GCP \"{}\"", self.id), &format!("Beobachtung zu GCP \"{}\" hinzufügen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for AddGcpObservation
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<RemoveGcpObservation>,
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
