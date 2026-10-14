//! 🔁 Remodeling mutation — `ReplaceDense`: whole-value swap of `ReconstructionResults.dense`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::DenseCloud;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-dense` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-dense")]
pub struct ReplaceDense {
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub dense: Option<DenseCloud>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_dense<Op: From<ReplaceDense>>(dense: Option<DenseCloud>) -> Op {
    Op::from(ReplaceDense { dense })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for ReplaceDense
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<ReplaceDense>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "dense", kind: "replace-dense", record: "ReplacedDense" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace dense", "Dichte Punktwolke ersetzen")
    }
}
//#endregion 🔖️Mutation
