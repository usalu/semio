//! 👁️ 👁️ Remodel play app commands command — `set-active-utility`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation, RemodelWorldCamera};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🧰️ The host-injected `setActiveUtility` action (framework-owned id — see
/// `semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID`), routed into remodel's own config.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
