//! 🔁 Remodel mutation — `ReplaceTrajectory`: whole-value swap of `ReconstructionResults.trajectory`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::{CameraTrajectory, RemodelSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁 `replace-trajectory` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-trajectory")]
pub struct ReplaceTrajectory {
    #[serde(default)]
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_trajectory(trajectory: Option<CameraTrajectory>) -> RemodelMutation {
    RemodelMutation::ReplaceTrajectory(ReplaceTrajectory { trajectory })
}

impl protocol::MutationKind<RemodelSnapshot, RemodelMutation> for ReplaceTrajectory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "trajectory", kind: "replace-trajectory", record: "ReplacedTrajectory" };

    async fn diff(&self, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace trajectory".to_string()
    }
}
//#endregion 🔖️Mutation
