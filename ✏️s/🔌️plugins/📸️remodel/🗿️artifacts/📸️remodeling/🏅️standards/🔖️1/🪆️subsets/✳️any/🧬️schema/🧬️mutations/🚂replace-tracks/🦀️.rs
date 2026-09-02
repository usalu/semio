//! 🔁 Remodeling mutation — `ReplaceTracks`: whole-value swap of `ReconstructionResults.tracks`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodeling::{MotionTrackSummary, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁 `replace-tracks` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-tracks")]
pub struct ReplaceTracks {
    pub tracks: Vec<MotionTrackSummary>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_tracks(tracks: Vec<MotionTrackSummary>) -> RemodelingMutation {
    RemodelingMutation::ReplaceTracks(ReplaceTracks { tracks })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceTracks {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "tracks", kind: "replace-tracks", record: "ReplacedTracks" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace tracks".to_string()
    }
}
//#endregion 🔖️Mutation
