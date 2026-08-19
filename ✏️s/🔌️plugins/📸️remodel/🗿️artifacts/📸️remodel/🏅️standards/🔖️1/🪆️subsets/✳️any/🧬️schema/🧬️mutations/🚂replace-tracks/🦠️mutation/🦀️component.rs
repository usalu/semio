//! 🔁 Remodel mutation — `ReplaceTracks`: whole-value swap of `ReconstructionResults.tracks`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, MotionTrackSummary};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-tracks` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-tracks")]
pub struct ReplaceTracks {

    pub tracks: Vec<MotionTrackSummary>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_tracks(tracks: Vec<MotionTrackSummary>) -> RemodelMutation {
    RemodelMutation::ReplaceTracks(ReplaceTracks { tracks })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceTracks {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "tracks", kind: "replace-tracks", record: "ReplacedTracks" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace tracks".to_string()
    }
}
//#endregion 🔖️Mutation
