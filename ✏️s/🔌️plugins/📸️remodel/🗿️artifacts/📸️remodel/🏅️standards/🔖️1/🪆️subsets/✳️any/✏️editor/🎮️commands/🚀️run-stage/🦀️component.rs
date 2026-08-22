//! ▶️ Run-stage reconstruction command.

use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::commands::run_reconstruction;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️RunStage
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "run-stage")]
pub struct RunStage {
    pub stage: String,
}

/// ▶️ Starts the requested stage as a fresh generation on the resumable pipeline.
pub async fn handle(payload: &RunStage, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    run_reconstruction::begin_stage_reconstruction(doc, &payload.stage)
}
//#endregion 🔖️RunStage
