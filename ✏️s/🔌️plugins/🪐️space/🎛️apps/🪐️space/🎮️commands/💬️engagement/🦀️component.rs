//! 💬️ S Studio app — window engagement input/submit commands (the drag-drop-catalogue hint bar +
//! compiled-DAG status strip).

use crate::apps::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowDocument, WorkflowMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};

//#region 🔖️WorkflowEngagementSubmit
pub mod workflow_engagement_submit {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "workflow-engagement-submit")]
    pub struct WorkflowEngagementSubmit {
        pub value: Option<String>,
    }

    pub fn handle(payload: &WorkflowEngagementSubmit, _doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        let raw = payload.value.clone().unwrap_or_else(|| cfg.projection.workflow_engagement_input.clone());
        let mut parts = raw.split_whitespace();
        match (parts.next(), parts.next()) {
            (Some(plugin_id), Some(app_id)) => match crate::apps::space::engine::add_workflow_node_operation(plugin_id, app_id, None, 80.0, 80.0) {
                Some((operation, node_id)) => Ok(Emit { document_mutations: vec![operation], config_mutations: vec![SpaceConfigMutation::SetActiveNode { node_id: Some(node_id) }], ..Default::default() }),
                None => Ok(Emit::default()),
            },
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️WorkflowEngagementSubmit

//#region 🔖️CompiledDagEngagementSubmit
pub mod compiled_dag_engagement_submit {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "compiled-dag-engagement-submit")]
    pub struct CompiledDagEngagementSubmit {}

    pub fn handle(_payload: &CompiledDagEngagementSubmit, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CompiledDagEngagementSubmit

//#region 🔖️WorkflowEngagementInput
pub mod workflow_engagement_input {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "workflow-engagement-input")]
    pub struct WorkflowEngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &WorkflowEngagementInput, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SpaceConfigMutation::SetWorkflowEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️WorkflowEngagementInput

//#region 🔖️CompiledDagEngagementInput
pub mod compiled_dag_engagement_input {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "compiled-dag-engagement-input")]
    pub struct CompiledDagEngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &CompiledDagEngagementInput, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SpaceConfigMutation::SetCompiledDagEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️CompiledDagEngagementInput

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        use crate::apps::space::SpaceCommand;
        store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementSubmit(workflow_engagement_submit::WorkflowEngagementSubmit { value: Some("draw draw".into()) }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementSubmit(compiled_dag_engagement_submit::CompiledDagEngagementSubmit {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementInput(workflow_engagement_input::WorkflowEngagementInput { value: "draw draw".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementInput(compiled_dag_engagement_input::CompiledDagEngagementInput { value: "".into() }));
    }
}
//#endregion 🧪️Tests
