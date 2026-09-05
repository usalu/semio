//! 👁️ 👁️ Remodeling play app commands command — `set-active-utility`.

use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧰️ The host-injected `setActiveUtility` action (framework-owned id — see
/// `semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID`), routed into remodeling's own config.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

pub async fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelingConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
