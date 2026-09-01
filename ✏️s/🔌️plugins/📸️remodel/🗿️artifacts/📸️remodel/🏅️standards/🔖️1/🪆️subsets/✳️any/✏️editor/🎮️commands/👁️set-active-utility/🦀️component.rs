//! 👁️ 👁️ Remodel play app commands command — `set-active-utility`.

use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧰️ The host-injected `setActiveUtility` action (framework-owned id — see
/// `semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID`), routed into remodel's own config.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

pub async fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
