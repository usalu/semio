//! 💬️ 💬️ S Studio app command — `workflow-engagement-submit`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "workflow-engagement-submit")]
pub struct WorkflowEngagementSubmit {
    pub value: Option<String>,
}

pub fn handle(payload: &WorkflowEngagementSubmit, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let raw = payload.value.clone().unwrap_or_else(|| cfg.snapshot.workflow_engagement_input.clone());
    let mut parts = raw.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(plugin_id), Some(app_id)) => match crate::engine::space::engine::resolve_future(crate::engine::space::engine::add_workflow_node_operation(plugin_id, app_id, None, 80.0, 80.0)) {
            Some((operation, node_id)) => Ok(Emit { artifact_mutations: vec![operation], config_mutations: vec![SpaceConfigMutation::SetActiveNode { node_id: Some(node_id) }], ..Default::default() }),
            None => Ok(Emit::default()),
        },
        _ => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn space_command_op_text_round_trips_every_variant() {
        use crate::engine::space::SpaceCommand;
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementSubmit(WorkflowEngagementSubmit { value: Some("draw draw".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementSubmit(crate::engine::space::commands::compiled_dag_engagement_submit::CompiledDagEngagementSubmit {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementInput(crate::engine::space::commands::workflow_engagement_input::WorkflowEngagementInput { value: "draw draw".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementInput(crate::engine::space::commands::compiled_dag_engagement_input::CompiledDagEngagementInput { value: "".into() }));
    }
}
//#endregion 🧪️Tests
