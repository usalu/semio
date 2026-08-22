//! 🔁️ Retry-stage reconstruction command.

use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::commands::run_reconstruction;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️RetryStage
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "retry-stage")]
pub struct RetryStage {
    pub stage: String,
}

/// 🔁️ Supersedes the live generation and resumes through the same bounded scheduler path.
pub async fn handle(payload: &RetryStage, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    run_reconstruction::begin_stage_reconstruction(doc, &payload.stage)
}
//#endregion 🔖️RetryStage
