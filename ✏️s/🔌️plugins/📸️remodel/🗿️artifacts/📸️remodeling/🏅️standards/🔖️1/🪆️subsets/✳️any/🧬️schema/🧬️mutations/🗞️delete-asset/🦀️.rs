//! 🗞️ Remodeling mutation — `DeleteAsset`: removes one key-addressed `ImageAsset`. No app call site
//! removes an asset today; this exists as `create-asset`'s inverse-only counterpart (a mutation kind
//! is real even without its own command call site) and for collection completeness.

use crate::standards::v1::subsets::any::schema::mutations::append_content::AppendContent;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗞️ `delete-asset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-asset")]
pub struct DeleteAsset {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_asset<Op: From<DeleteAsset>>(key: String) -> Op {
    Op::from(DeleteAsset { key })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for DeleteAsset
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<AppendContent> + From<CommitReconstruction>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "asset", kind: "delete-asset", record: "DeletedAsset" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete asset \"{}\"", self.key), &format!("Asset \"{}\" löschen", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for DeleteAsset
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<AppendContent> + From<CommitReconstruction>,
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
