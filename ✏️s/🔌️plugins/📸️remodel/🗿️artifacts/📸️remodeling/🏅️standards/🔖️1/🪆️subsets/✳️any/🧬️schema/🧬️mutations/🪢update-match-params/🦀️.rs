//! ⚙️ Remodeling mutation — `UpdateMatchParams`: full-record replace of `ReconstructionParams.matching` (always
//! set wholesale from the palette form's flat field list — genuinely inseparable).

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MatchParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⚙️ `update-match-params` payload — full FINAL-state `MatchParams`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-match-params")]
pub struct UpdateMatchParams {
    #[dsl(block)]
    pub params: MatchParams,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_match_params<Op: From<UpdateMatchParams>>(params: MatchParams) -> Op {
    Op::from(UpdateMatchParams { params })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for UpdateMatchParams
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<UpdateMatchParams>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "matching-params", kind: "update-match-params", record: "UpdatedMatchParams" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update matching params", "Zuordnungsparameter aktualisieren")
    }
}
//#endregion 🔖️Mutation
