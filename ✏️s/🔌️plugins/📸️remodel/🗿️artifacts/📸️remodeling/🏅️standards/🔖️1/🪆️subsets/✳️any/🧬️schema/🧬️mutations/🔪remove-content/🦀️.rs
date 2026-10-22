//! 🔪️ Remodeling mutation — `RemoveContent`: removes every leaf of one durable content entry from leaf
//! index `from` on; `from` 0 removes the entry. It is `append-content`'s exact inverse.

use crate::standards::v1::subsets::any::schema::mutations::append_content::AppendContent;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔪️ `remove-content` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-content")]
pub struct RemoveContent {
    pub content_id: String,
    pub from: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_content<Op: From<RemoveContent>>(content_id: String, from: u64) -> Op {
    Op::from(RemoveContent { content_id, from })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for RemoveContent
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<AppendContent>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "content", kind: "remove-content", record: "RemovedContent" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove content \"{}\" leaves from {}", self.content_id, self.from), &format!("Blätter von Inhalt \"{}\" aus {} entfernen", self.content_id, self.from))
    }
    fn target(&self) -> Vec<String> {
        vec![self.content_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for RemoveContent
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<AppendContent>,
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
