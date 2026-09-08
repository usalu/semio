//! 👥️ 👥️ S Studio app command — `presence-heartbeat`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "presence-heartbeat")]
pub struct PresenceHeartbeat {
    pub client_id: String,
    pub name: String,
}

/// 🐢️ A heartbeat only records this client's own identity for the presence broadcast — it must
/// declare `None` `ui_scope` so it never triggers a full-shell `refresh-ui` for the sending client.
pub fn handle(payload: &PresenceHeartbeat, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let config_mutations = vec![SpaceConfigMutation::SetClient { client_id: Some(payload.client_id.clone()), client_name: Some(payload.name.clone()) }];
    Ok(Emit { config_mutations, ui_scope: semio_framework::kernel::UiDirtyScope::None, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
