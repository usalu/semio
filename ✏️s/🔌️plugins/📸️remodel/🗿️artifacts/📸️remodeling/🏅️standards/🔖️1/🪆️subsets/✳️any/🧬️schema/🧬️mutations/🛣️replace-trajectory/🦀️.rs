//! 🔁 Remodeling mutation — `ReplaceTrajectory`: whole-value swap of `ReconstructionResults.trajectory`, a large
//! structured sub-payload swapped wholesale by the reconstruction engine or a clear/reset command.

use crate::artifacts::remodeling::{CameraTrajectory, RemodelingSnapshot};
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁 `replace-trajectory` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-trajectory")]
pub struct ReplaceTrajectory {
    #[value(default)]
    #[serde(default)]
    #[dsl(block)]
    pub trajectory: Option<CameraTrajectory>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_trajectory(trajectory: Option<CameraTrajectory>) -> RemodelingMutation {
    RemodelingMutation::ReplaceTrajectory(ReplaceTrajectory { trajectory })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceTrajectory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "trajectory", kind: "replace-trajectory", record: "ReplacedTrajectory" };

    async fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Replace trajectory".to_string()
    }
}
//#endregion 🔖️Mutation
