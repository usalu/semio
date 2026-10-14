//! ➕ Remodeling mutation — `AddStreamFrame`: appends one `FrameRef` to an EXISTING media stream and
//! sets its `kind` (image import handlers append to a stream that already exists; creating a brand
//! new stream instead goes through `create-stream`).

use crate::standards::v1::subsets::any::schema::mutations::remove_stream_frame::RemoveStreamFrame;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FrameRef;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕ `add-stream-frame` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-stream-frame")]
pub struct AddStreamFrame {
    pub id: String,
    #[dsl(block)]
    pub frame: FrameRef,
    pub kind: MediaKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_stream_frame<Op: From<AddStreamFrame>>(id: String, frame: FrameRef, kind: MediaKind) -> Op {
    Op::from(AddStreamFrame { id, frame, kind })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for AddStreamFrame
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<RemoveStreamFrame>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "stream", kind: "add-stream-frame", record: "AddedStreamFrame" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add frame {} to stream \"{}\"", self.frame.index, self.id), &format!("Einzelbild {} zu Datenstrom \"{}\" hinzufügen", self.frame.index, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
