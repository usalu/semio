
use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn dag_config_default_matches_dag_camera_implicit_default() {
    let config = DagConfig::default();
    assert_eq!((config.camera_x, config.camera_y, config.camera_zoom), (0.0, 0.0, 1.0));
    assert_eq!(dag_config_camera(&config), DagCamera { x: 0.0, y: 0.0, zoom: 1.0 });
}

/// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `DagConfig`.
#[semio_framework_async_macros::async_test]
async fn dag_config_dsl_pack_round_trip() {
    let config = DagConfig { camera_x: 12.5, camera_y: -3.0, camera_zoom: 2.25 };
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

#[semio_framework_async_macros::async_test]
async fn dag_config_operation_text_binary_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&DagConfigMutation::ReplaceConfig(ReplaceConfig { config: DagConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0 } }));
    store::os_store::test_support::assert_op_line_round_trip(&DagConfigMutation::ChangeCamera(ChangeCamera { x: 12.5, y: -3.0, zoom: 2.25 }));
}

#[semio_framework_async_macros::async_test]
async fn dag_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = DagConfig { camera_x: 1.0, camera_y: 2.0, camera_zoom: 3.0 };
    let operation = DagConfigMutation::ChangeCamera(ChangeCamera { x: 9.0, y: 8.0, zoom: 7.0 });
    let forward = operation.diff(&base).diff().clone();
    assert_eq!((forward.camera_x, forward.camera_y, forward.camera_zoom), (9.0, 8.0, 7.0));
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![DagConfigMutation::ChangeCamera(ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom })]);
    let restored = backwards[0].diff(&forward).diff().clone();
    assert_eq!(restored, base);
}
