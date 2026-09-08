
use super::*;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use crate::engine::space::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementSubmit(WorkflowEngagementSubmit { value: Some("draw draw".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementSubmit(crate::engine::space::commands::compiled_dag_engagement_submit::CompiledDagEngagementSubmit {}));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementInput(crate::engine::space::commands::workflow_engagement_input::WorkflowEngagementInput { value: "draw draw".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementInput(crate::engine::space::commands::compiled_dag_engagement_input::CompiledDagEngagementInput { value: "".into() }));
}
