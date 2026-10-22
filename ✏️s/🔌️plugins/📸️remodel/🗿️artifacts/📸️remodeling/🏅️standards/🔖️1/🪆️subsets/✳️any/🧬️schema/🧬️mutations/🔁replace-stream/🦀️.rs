//! 🔁 Remodeling mutation — `ReplaceStreamSource`: whole-value swap of one media stream's `VideoSource`
//! provenance, written wholesale by `ImportVideoDone`.

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::VideoSource;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-stream-source` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-stream-source")]
pub struct ReplaceStreamSource {
    pub id: String,
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub source: Option<VideoSource>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_stream_source<Op: From<ReplaceStreamSource>>(id: String, source: Option<VideoSource>) -> Op {
    Op::from(ReplaceStreamSource { id, source })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for ReplaceStreamSource
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<ReplaceStreamSource>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "stream", kind: "replace-stream-source", record: "ReplacedStreamSource" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace stream \"{}\" source", self.id), &format!("Quelle von Datenstrom \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for ReplaceStreamSource
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<ReplaceStreamSource>,
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
