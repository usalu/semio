//! ✂️ Remodeling mutation — `DeleteRigExtrinsic`: removes a camera-id-keyed rig pose.

use crate::standards::v1::subsets::any::schema::mutations::create_rig_extrinsic::CreateRigExtrinsic;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ `delete-rig-extrinsic` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-rig-extrinsic")]
pub struct DeleteRigExtrinsic {
    pub camera_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_rig_extrinsic<Op: From<DeleteRigExtrinsic>>(camera_id: String) -> Op {
    Op::from(DeleteRigExtrinsic { camera_id })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for DeleteRigExtrinsic
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<CreateRigExtrinsic>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "rig-extrinsic", kind: "delete-rig-extrinsic", record: "DeletedRigExtrinsic" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete rig extrinsic \"{}\"", self.camera_id), &format!("Extrinsische Rig-Kalibrierung \"{}\" löschen", self.camera_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.camera_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for DeleteRigExtrinsic
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<CreateRigExtrinsic>,
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
