//! 🔍️ 🔍️ S Studio app command — `close-focused-instance`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation, FocusedNodeSetting};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "close-focused-instance")]
pub struct CloseFocusedInstance {}

pub fn handle(_payload: &CloseFocusedInstance, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    Ok(Emit::config(vec![SpaceConfigMutation::SetFocusedNode(FocusedNodeSetting { node_id: None })]))
}
