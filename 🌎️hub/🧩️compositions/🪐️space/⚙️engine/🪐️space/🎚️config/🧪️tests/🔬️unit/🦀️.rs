
use super::*;
use crate::engine::space::S_PLAY_PARAMETERS_TAB_ID;
use crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW;
use protocol::Mutation;

async fn round_trip(config: &SpaceConfig, operation: &SpaceConfigMutation) -> SpaceConfig {
    let (forward, _messages) = store::apply_mutation(config, operation).expect("valid mutation");
    let backwards = operation.inverse(config).expect("valid inverse config mutation");
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
async fn remove_camera_deletes_the_window_row_and_inverts_to_set_camera() {
    let camera = SpaceWindowCamera { x: 1.0, y: 2.0, zoom: 3.0 };
    let mut config = SpaceConfig::default();
    config.camera.insert(S_PLAY_WINDOW_WORKFLOW.into(), camera);
    let next = round_trip(&config, &SpaceConfigMutation::RemoveCamera { window_id: S_PLAY_WINDOW_WORKFLOW.into() }).await;
    assert!(next.camera.is_empty());
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
    store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetActivePanelTab { tab_id: "s-play-catalogue".into() });
}

#[semio_framework_async_macros::async_test]
async fn space_config_dsl_pack_equivalence() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&SpaceConfig::default());
}

fn busy_config() -> SpaceConfig {
    SpaceConfig {
        camera: BTreeMap::from([("left".to_string(), SpaceWindowCamera { x: 1.0, y: 2.0, zoom: 3.0 }), ("right".to_string(), SpaceWindowCamera { x: 4.0, y: 5.0, zoom: 6.0 })]),
        collapsed_node_ids: vec!["c".into()],
        preview_off_node_ids: vec!["p".into()],
        active_node_id: Some("a".into()),
        focused_node_id: Some("f".into()),
        clipboard_node_ids: vec!["k".into()],
        workflow_engagement_input: "draw".into(),
        compiled_dag_engagement_input: "dag".into(),
        pending_import_node_id: Some("i".into()),
        pending_import_format: Some("dwg".into()),
        active_panel_tab: S_PLAY_PARAMETERS_TAB_ID.into(),
        space_id: Some("demo".into()),
    }
}

#[semio_framework_async_macros::async_test]
async fn every_operation_obeys_the_inverse_sum_law_including_a_middle_camera_row() {
    use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    let camera = SpaceWindowCamera { x: 9.0, y: 8.0, zoom: 7.0 };
    for base in [busy_config(), SpaceConfig::default()] {
        for mutation in [
            SpaceConfigMutation::SetActiveNode { node_id: None },
            SpaceConfigMutation::SetFocusedNode { node_id: Some("z".into()) },
            SpaceConfigMutation::SetClipboard { node_ids: vec!["x".into(), "y".into()] },
            SpaceConfigMutation::SetCollapsed { node_ids: Vec::new() },
            SpaceConfigMutation::SetPreviewOff { node_ids: vec!["q".into()] },
            SpaceConfigMutation::SetCamera { window_id: "left".into(), camera },
            SpaceConfigMutation::SetCamera { window_id: "middle".into(), camera },
            SpaceConfigMutation::SetWorkflowEngagementInput { value: "line".into() },
            SpaceConfigMutation::SetCompiledDagEngagementInput { value: String::new() },
            SpaceConfigMutation::SetPendingImport { node_id: None, format: Some("step".into()) },
            SpaceConfigMutation::SetSpaceId { space_id: None },
            SpaceConfigMutation::SetActivePanelTab { tab_id: S_PLAY_CATALOGUE_TAB_ID.into() },
        ] {
            assert_mutation_inverse_sum_law(&mutation, &base).await;
        }
    }
    assert_mutation_inverse_sum_law(&SpaceConfigMutation::RemoveCamera { window_id: "left".into() }, &busy_config()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_diff_names_only_what_changed() {
    let base = busy_config();
    let same = SpaceConfigMutation::SetSpaceId { space_id: base.space_id.clone() }.diff(&base);
    assert!(protocol::DiffAlgebra::is_empty(same.diff()), "an unchanged value is an empty diff");
    let moved = SpaceConfigMutation::SetCamera { window_id: "left".into(), camera: SpaceWindowCamera { x: 0.0, y: 0.0, zoom: 1.0 } }.diff(&base);
    assert_eq!(moved.diff().camera.keys().collect::<Vec<_>>(), vec!["left"]);
}
