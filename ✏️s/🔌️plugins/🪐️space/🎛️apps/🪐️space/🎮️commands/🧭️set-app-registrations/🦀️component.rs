//! 🧭️ 🧭️ S Studio app command — `set-app-registrations`.

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-app-registrations")]
pub struct SetAppRegistrations {
    pub json: String,
}

/// 🪐️ Pure host-hint side effect; no document/config mutation, so the default full-refresh `Emit`
/// is enough to pick up the newly-registered apps on the next catalogue render. The actual
/// `register_app_io` OS-registry bridge is `engine::apply_app_registrations` — this handler stays
/// dispatch-only per the per-app recipe (command files parse + delegate, they don't call OS-host
/// registration APIs directly).
pub fn handle(payload: &SetAppRegistrations, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    crate::apps::space::engine::apply_app_registrations(&payload.json);
    Ok(Emit::default())
}
