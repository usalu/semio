#[semio_framework_async_macros::async_test]
async fn primary_asset_is_a_distinct_capture_session_snapshot() {
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, print_dsl};
    let snapshot = parse_dsl(super::PRIMARY_TEXT).expect("session snapshot");
    assert_eq!(snapshot.streams.len(), 2);
    assert_eq!(snapshot.calibration.cameras.len(), 2);
    assert!(snapshot.streams.iter().all(|stream| stream.frames.is_empty()));
    assert!(snapshot.params.motion.enabled);
    assert_eq!(parse_dsl(&print_dsl(&snapshot)).expect("canonical session"), snapshot);
}
