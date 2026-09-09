//! ▶️ Run-stage reconstruction command.

use crate::editor::remodeling::commands::run_reconstruction;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RunStage
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "run-stage")]
pub struct RunStage {
    pub stage: String,
}

/// ▶️ Starts the requested stage as a fresh generation on the resumable pipeline.
pub fn handle(payload: &RunStage, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(run_reconstruction::begin_stage_reconstruction(doc, &payload.stage))
}
//#endregion 🔖️RunStage
