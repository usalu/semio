//! 👥️ 👥️ S Studio app command — `presence-heartbeat`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "presence-heartbeat")]
pub struct PresenceHeartbeat {
}

/// 🐢️ The macro-only route has no host view and therefore cannot claim a session identity.
pub fn handle(_payload: &PresenceHeartbeat, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Err(Fault::from("s.space.session-identity-required"))
}

/// 🪪️ Accepts a heartbeat only with the current host-owned session identity and never persists a copy.
pub fn handle_with_identity(
    _payload: &PresenceHeartbeat,
    identity: &semio_framework_plugin::ViewSessionIdentity,
    _doc: &ArtifactView<'_, WorkflowSnapshot>,
    _cfg: &ConfigView<'_, SpaceConfig>,
) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    if !crate::view_session_identity_valid(identity) {
        return Err(Fault::from("s.space.session-identity-required"));
    }
    Ok(Emit { ui_scope: semio_framework::kernel::UiDirtyScope::None, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
