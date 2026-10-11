//! ✂️ Remodeling mutation — `DeleteRigExtrinsic`: removes a camera-id-keyed rig pose.

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;
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
pub fn delete_rig_extrinsic(camera_id: String) -> RemodelingMutation {
    RemodelingMutation::DeleteRigExtrinsic(DeleteRigExtrinsic { camera_id })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for DeleteRigExtrinsic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "rig-extrinsic", kind: "delete-rig-extrinsic", record: "DeletedRigExtrinsic" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete rig extrinsic \"{}\"", self.camera_id), &format!("Extrinsische Rig-Kalibrierung \"{}\" löschen", self.camera_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.camera_id.clone()]
    }
}
//#endregion 🔖️Mutation
