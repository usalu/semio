//! 📦️ Remodeling mutation — `AppendContent`: appends raw 4 KiB leaves to one durable content entry
//! (`durable_artifacts[content_id]`). A reconstruction run publishes every sparse cloud, mesh and raster
//! it produced through these leaves, and `commit-reconstruction` then names the complete content by id.

use crate::standards::v1::subsets::any::schema::mutations::remove_content::RemoveContent;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ByteBuffer;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingContentKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📦️ `append-content` payload: `chunks` are literal raw leaves placed at leaf index `first`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "append-content")]
pub struct AppendContent {
    pub content_id: String,
    pub kind: RemodelingContentKind,
    pub mime: Option<String>,
    pub width: u32,
    pub height: u32,
    pub first: u64,
    pub chunks: Vec<ByteBuffer>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn append_content<Op: From<AppendContent>>(payload: AppendContent) -> Op {
    Op::from(payload)
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for AppendContent
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<RemoveContent>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "append", entity: "content", kind: "append-content", record: "AppendedContent" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Append {} content leaves to \"{}\"", self.chunks.len(), self.content_id), &format!("{} Inhaltsblätter an \"{}\" anhängen", self.chunks.len(), self.content_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.content_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for AppendContent
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<RemoveContent>,
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
