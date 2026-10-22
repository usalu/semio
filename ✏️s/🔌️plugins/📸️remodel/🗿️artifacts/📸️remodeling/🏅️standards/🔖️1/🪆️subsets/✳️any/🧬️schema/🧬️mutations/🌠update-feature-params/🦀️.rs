//! ⚙️ Remodeling mutation — `UpdateFeatureParams`: full-record replace of `ReconstructionParams.feature` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FeatureParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-feature-params` payload — full FINAL-state `FeatureParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-feature-params")]
pub struct UpdateFeatureParams {
    #[dsl(block)]
    pub params: FeatureParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_feature_params<Op: From<UpdateFeatureParams>>(params: FeatureParams) -> Op {
    Op::from(UpdateFeatureParams { params })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for UpdateFeatureParams
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<UpdateFeatureParams>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "feature-params", kind: "update-feature-params", record: "UpdatedFeatureParams" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update feature params", "Merkmalsparameter aktualisieren")
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for UpdateFeatureParams
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<UpdateFeatureParams>,
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
