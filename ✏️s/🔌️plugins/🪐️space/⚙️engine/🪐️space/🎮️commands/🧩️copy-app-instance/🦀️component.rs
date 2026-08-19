//! 🧩️ 🧩️ S Studio app command — `copy-app-instance`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "copy-app-instance")]
pub struct CopyAppInstance {}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable
/// only through that macro-generated path (`SpaceApp::handle` always routes this command through
/// `apply` below instead), so it degrades to treating the selection as empty (copies nothing).
pub async fn handle(_payload: &CopyAppInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(vec![SpaceConfigMutation::SetClipboard { node_ids: Vec::new() }]))
}

pub async fn apply(_payload: &CopyAppInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(vec![SpaceConfigMutation::SetClipboard { node_ids: interaction.selection("graph").ids.clone() }]))
}
