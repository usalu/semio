//! ⏱️ Remodeling mutation — `ChangeStreamSync`: sets one media stream's `sync_offset_ms`.

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ⏱️ `change-stream-sync` payload — FINAL-state `sync_offset_ms`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-stream-sync")]
pub struct ChangeStreamSync {
    pub id: String,
    pub new_sync_offset_ms: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_stream_sync<Op: From<ChangeStreamSync>>(id: String, new_sync_offset_ms: f64) -> Op {
    Op::from(ChangeStreamSync { id, new_sync_offset_ms })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for ChangeStreamSync
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<ChangeStreamSync>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "stream", kind: "change-stream-sync", record: "ChangedStreamSync" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change stream \"{}\" sync offset to {}ms", self.id, self.new_sync_offset_ms), &format!("Synchronisationsversatz von Datenstrom \"{}\" auf {}ms ändern", self.id, self.new_sync_offset_ms))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for ChangeStreamSync
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<ChangeStreamSync>,
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
