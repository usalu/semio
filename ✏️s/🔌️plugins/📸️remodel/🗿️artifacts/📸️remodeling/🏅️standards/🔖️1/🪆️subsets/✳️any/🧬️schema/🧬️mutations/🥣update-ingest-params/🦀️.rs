//! ⚙️ Remodeling mutation — `UpdateIngestParams`: full-record replace of `ReconstructionParams.ingest` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-ingest-params` payload — full FINAL-state `IngestParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-ingest-params")]
pub struct UpdateIngestParams {
    #[dsl(block)]
    pub params: IngestParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_ingest_params<Op: From<UpdateIngestParams>>(params: IngestParams) -> Op {
    Op::from(UpdateIngestParams { params })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for UpdateIngestParams
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<UpdateIngestParams>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "ingest-params", kind: "update-ingest-params", record: "UpdatedIngestParams" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update ingest params", "Importparameter aktualisieren")
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for UpdateIngestParams
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<UpdateIngestParams>,
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
