//! 🔁️ Retry-stage reconstruction command.

use crate::editor::remodeling::commands::run_reconstruction;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RetryStage
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "retry-stage")]
pub struct RetryStage {
    pub stage: String,
}

/// 🔁️ Supersedes the live generation and resumes through the same bounded scheduler path.
pub fn handle(payload: &RetryStage, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    run_reconstruction::begin_stage_reconstruction(doc, &payload.stage)
}
//#endregion 🔖️RetryStage
