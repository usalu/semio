//! 🔁 Remodeling mutation — `ReplaceTracks`: whole-value swap of `ReconstructionResults.tracks`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MotionTrackSummary;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-tracks` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-tracks")]
pub struct ReplaceTracks {
    pub tracks: Vec<MotionTrackSummary>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_tracks<Op: From<ReplaceTracks>>(tracks: Vec<MotionTrackSummary>) -> Op {
    Op::from(ReplaceTracks { tracks })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for ReplaceTracks
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<ReplaceTracks>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "tracks", kind: "replace-tracks", record: "ReplacedTracks" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace tracks", "Merkmalsspuren ersetzen")
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for ReplaceTracks
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<ReplaceTracks>,
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
