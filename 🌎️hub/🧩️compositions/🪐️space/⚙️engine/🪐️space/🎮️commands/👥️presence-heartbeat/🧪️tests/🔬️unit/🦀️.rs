
use semio_framework_plugin::{ArtifactView, ConfigView};
use semio_hub_space_commands_media::presence_heartbeat::{PresenceHeartbeat, handle, handle_with_identity};
use semio_hub_space_foundation::config::SpaceConfig;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use semio_hub_space_app::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::PresenceHeartbeat(PresenceHeartbeat {}));
}

#[semio_framework_async_macros::async_test]
async fn presence_heartbeat_declares_none_ui_scope() {
    use crate::demo_space_projection;
    use semio_hub_space_app::SpaceCommand;
    use semio_framework::kernel::UiDirtyScope;
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let history = semio_hub_space_app::unit_tests::context::empty_history();
    let doc = ArtifactView::new(&projection, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    assert!(handle(&PresenceHeartbeat {}, &doc, &cfg).is_err());
    let identity = semio_framework_plugin::ViewSessionIdentity { user_id: "client-test-c".into(), display_name: "Cass".into() };
    let emit = handle_with_identity(&PresenceHeartbeat {}, &identity, &doc, &cfg).expect("handle");
    assert!(emit.config_mutations.is_empty());
    assert!(matches!(emit.ui_scope, UiDirtyScope::None), "presenceHeartbeat must declare None, got {:?}", emit.ui_scope);
}
