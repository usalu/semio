//! 🛑️ User-addressable reconstruction cancellation.

use crate::artifacts::remodeling::{op::RemodelingMutation, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "cancel-reconstruction")]
pub struct CancelReconstruction {}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub async fn handle(_payload: &CancelReconstruction, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(crate::editor::remodeling::commands::run_reconstruction::cancel_current_reconstruction(doc.snapshot))
}
//#endregion 🔖️Handler
