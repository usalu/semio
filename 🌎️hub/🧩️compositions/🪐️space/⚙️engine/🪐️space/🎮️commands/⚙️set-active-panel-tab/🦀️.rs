//! 🧭️ 🧭️ S Studio app command — `set-active-panel-tab`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation, ActivePanelTabSetting};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "active-panel-tab")]
pub struct SetActivePanelTab {
    pub tab_id: String,
}

pub fn handle(payload: &SetActivePanelTab, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(vec![SpaceConfigMutation::SetActivePanelTab(ActivePanelTabSetting { tab_id: payload.tab_id.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
