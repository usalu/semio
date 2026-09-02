//! 🛑️ User-addressable reconstruction cancellation.

use crate::artifacts::remodel::{op::RemodelMutation, RemodelSnapshot};
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "cancel-reconstruction")]
pub struct CancelReconstruction {}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub async fn handle(_payload: &CancelReconstruction, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(crate::editor::remodel::commands::run_reconstruction::cancel_current_reconstruction(doc.snapshot))
}
//#endregion 🔖️Handler
