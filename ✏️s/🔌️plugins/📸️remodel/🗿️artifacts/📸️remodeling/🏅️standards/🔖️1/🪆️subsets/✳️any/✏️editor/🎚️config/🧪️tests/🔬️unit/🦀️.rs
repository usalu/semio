
use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn remodeling_config_default_matches_the_former_runtime_defaults() {
    let config = RemodelingConfig::default();
    assert_eq!(config.camera, RemodelingWorldCamera { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
    assert!(config.layers.mesh && config.layers.dense && config.layers.sparse && config.layers.cameras && config.layers.gcps);
    assert_eq!(config.frame_cursor, RemodelingFrameCursor::default());
    assert_eq!(config.report_table, "frames");
    assert_eq!(config.active_utility_id, "select");
    assert_eq!(config.locale, "en-US");
}

#[semio_framework_async_macros::async_test]
async fn remodeling_config_operation_diff_is_whole_record_replace() {
    let base = RemodelingConfig::default();
    let mut next = base.clone();
    next.report_table = "gcps".into();
    assert_eq!(protocol::MutationDiff::apply(&next, &base).expect("valid config mutation diff"), next, "apply ignores base entirely, like ShootingConfig");
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_apply_and_backwards_restore_the_pre_edit_snapshot() {
    let base = RemodelingConfig::default();

    let camera = RemodelingWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
    let op = RemodelingConfigMutation::SetCamera(SetCamera { camera: camera.clone() });
    let next = op.diff(&base).into_parts().0;
    assert_eq!(next.camera, camera);
    assert_eq!(op.inverse(&base), vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]);
    assert_eq!(op.inverse(&base)[0].diff(&next).into_parts().0, base, "backwards restores the exact pre-edit config");

    let op = RemodelingConfigMutation::SetLayerVisibility(SetLayerVisibility { layer: "dense".into(), visible: false });
    let next = op.diff(&base).into_parts().0;
    assert!(!next.layers.dense);
    assert!(next.layers.mesh, "only the named layer flips");

    let op = RemodelingConfigMutation::SetFrameCursor(SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 4 });
    let next = op.diff(&base).into_parts().0;
    assert_eq!(next.frame_cursor.stream_id.as_deref(), Some("stream-1"));
    assert_eq!(next.frame_cursor.frame_index, 4);

    let op = RemodelingConfigMutation::SetReportTable(SetReportTable { table: "gcps".into() });
    assert_eq!(op.diff(&base).diff().report_table, "gcps");

    let op = RemodelingConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: "measure".into() });
    assert_eq!(op.diff(&base).diff().active_utility_id, "measure");

    let op = RemodelingConfigMutation::SetLocale(SetLocale { value: "de-DE".into() });
    assert_eq!(op.diff(&base).diff().locale, "de-DE");
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_roundtrip_through_op_text() {
    let config = RemodelingConfig::default();
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config }));
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetCamera(SetCamera { camera: RemodelingWorldCamera::default() }));
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetLayerVisibility(SetLayerVisibility { layer: "gcps".into(), visible: false }));
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetFrameCursor(SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 }));
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetFrameCursor(SetFrameCursor { stream_id: None, frame_index: 0 }));
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetReportTable(SetReportTable { table: "tracks".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: "gcpPlace".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
}
