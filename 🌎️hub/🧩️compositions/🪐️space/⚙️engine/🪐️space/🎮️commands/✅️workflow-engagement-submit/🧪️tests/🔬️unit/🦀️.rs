
use semio_hub_space_commands_media::workflow_engagement_submit::WorkflowEngagementSubmit;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use semio_hub_space_app::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementSubmit(WorkflowEngagementSubmit { value: Some("draw draw".into()) }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementSubmit(semio_hub_space_commands_media::compiled_dag_engagement_submit::CompiledDagEngagementSubmit {}));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementInput(semio_hub_space_commands_media::workflow_engagement_input::WorkflowEngagementInput { value: "draw draw".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementInput(semio_hub_space_commands_media::compiled_dag_engagement_input::CompiledDagEngagementInput { value: "".into() }));
}
