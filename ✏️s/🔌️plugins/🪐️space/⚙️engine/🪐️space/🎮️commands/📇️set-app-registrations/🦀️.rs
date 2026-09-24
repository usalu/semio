//! 🧭️ 🧭️ S Studio app command — `set-app-registrations`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-app-registrations")]
pub struct SetAppRegistrations {
    pub json: String,
}

/// 🪐️ Pure host-hint side effect; no document/config mutation, so the default full-refresh `Emit`
/// is enough to pick up the newly-registered apps on the next catalogue render. The actual
/// `register_app_io` OS-registry bridge is `engine::apply_app_registrations`; a malformed roster is
/// refused as `s.space.app-registrations-malformed`.
pub fn handle(payload: &SetAppRegistrations, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    semio_framework_plugin::resolve_ready(crate::engine::space::engine::apply_app_registrations(&payload.json))?;
    Ok(Emit::default())
}
