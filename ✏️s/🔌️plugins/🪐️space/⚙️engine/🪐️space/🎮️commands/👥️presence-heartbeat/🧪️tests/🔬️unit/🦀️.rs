
use super::*;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use crate::engine::space::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PresenceHeartbeat(PresenceHeartbeat { client_id: "c1".into(), name: "Ada".into() }));
}

#[semio_framework_async_macros::async_test]
async fn presence_heartbeat_declares_none_ui_scope() {
    use crate::demo_space_projection;
    use crate::engine::space::SpaceCommand;
    use crate::engine::space::unit_tests::context::studio_emit;
    use semio_framework::kernel::UiDirtyScope;
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let emit = studio_emit(&projection, &config, &SpaceCommand::PresenceHeartbeat(PresenceHeartbeat { client_id: "client-test-c".into(), name: "Cass".into() })).await.expect("handle");
    assert!(matches!(emit.ui_scope, UiDirtyScope::None), "presenceHeartbeat must declare None, got {:?}", emit.ui_scope);
}
