
use super::*;
use crate::engine::space::S_PLAY_PARAMETERS_TAB_ID;
use crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW;
use protocol::Mutation;

async fn round_trip(config: &SpaceConfig, operation: &SpaceConfigMutation) -> SpaceConfig {
    let (forward, _messages) = store::apply_mutation(config, operation).expect("valid mutation");
    let backwards = operation.inverse(config);
    let mut restored = forward.clone();
    for back in &backwards {
        let (next, _messages) = store::apply_mutation(&restored, back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn space_config_default_matches_the_expected_sticky_defaults() {
    let config = SpaceConfig::default();
    assert_eq!(config.active_panel_tab, S_PLAY_CATALOGUE_TAB_ID);
    assert!(config.camera.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn space_config_dsl_text_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&SpaceConfig::default());
}

#[semio_framework_async_macros::async_test]
async fn set_camera_round_trips_and_keys_by_window_id() {
    let config = SpaceConfig::default();
    let camera = SpaceWindowCamera { x: 12.0, y: -4.0, zoom: 2.0 };
    let operation = SpaceConfigMutation::SetCamera { window_id: S_PLAY_WINDOW_WORKFLOW.into(), camera };
    let next = round_trip(&config, &operation).await;
    assert_eq!(next.camera.get(S_PLAY_WINDOW_WORKFLOW), Some(&camera));
}

#[semio_framework_async_macros::async_test]
async fn set_active_panel_tab_round_trips() {
    let config = SpaceConfig::default();
    let operation = SpaceConfigMutation::SetActivePanelTab { tab_id: S_PLAY_PARAMETERS_TAB_ID.into() };
    let next = round_trip(&config, &operation).await;
    assert_eq!(next.active_panel_tab, S_PLAY_PARAMETERS_TAB_ID);
}

#[semio_framework_async_macros::async_test]
async fn space_config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::Snapshot { config: SpaceConfig::default() });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetActiveNode { node_id: Some("a".into()) });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetFocusedNode { node_id: None });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetClipboard { node_ids: vec!["a".into()] });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetCollapsed { node_ids: vec!["a".into()] });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetPreviewOff { node_ids: vec!["a".into()] });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetCamera { window_id: "s-workflow".into(), camera: SpaceWindowCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetWorkflowEngagementInput { value: "draw draw".into() });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetCompiledDagEngagementInput { value: "".into() });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetPendingImport { node_id: Some("a".into()), format: Some("dwg".into()) });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetPendingImport { node_id: None, format: None });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetSpaceId { space_id: Some("demo".into()) });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetClient { client_id: Some("c1".into()), client_name: Some("Ada".into()) });
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetActivePanelTab { tab_id: "s-play-catalogue".into() });
}

#[semio_framework_async_macros::async_test]
async fn space_config_dsl_pack_equivalence() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&SpaceConfig::default());
}
