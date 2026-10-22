//! 🔩 Remodeling mutation — `UpdateRigExtrinsic`: full-record replace of an EXISTING rig pose (one
//! rigid pose = `{rotation_wxyz, translation_m}`, inseparable — same `update` reasoning as
//! `update-camera-calibration`).

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RigExtrinsic;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔩 `update-rig-extrinsic` payload — full FINAL-state `RigExtrinsic` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-rig-extrinsic")]
pub struct UpdateRigExtrinsic {
    #[dsl(block)]
    pub extrinsic: RigExtrinsic,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_rig_extrinsic<Op: From<UpdateRigExtrinsic>>(extrinsic: RigExtrinsic) -> Op {
    Op::from(UpdateRigExtrinsic { extrinsic })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for UpdateRigExtrinsic
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<UpdateRigExtrinsic>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "rig-extrinsic", kind: "update-rig-extrinsic", record: "UpdatedRigExtrinsic" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update rig extrinsic \"{}\"", self.extrinsic.camera_id), &format!("Extrinsische Rig-Kalibrierung \"{}\" aktualisieren", self.extrinsic.camera_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.extrinsic.camera_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for UpdateRigExtrinsic
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<UpdateRigExtrinsic>,
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
